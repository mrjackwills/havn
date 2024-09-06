pub mod host_info;
use crate::{exit, parse_arg::CliArgs, port_descriptions::PortDescriptions};
use std::{collections::HashSet, future::Future, net::IpAddr, pin::Pin};
use tokio::{net::TcpStream, sync::mpsc::Sender};

#[derive(Debug, Clone, Hash, Copy)]
struct PortMessage {
    port: u16,
    open: bool,
}

impl PortMessage {
    /// Check if the next port in the sequence can be spawned
    /// Checks that port + chunk won't overflow, and then if not, check that the output is smaller or equal to the port_max
    fn can_spawn(self, chunk: u16, port_max: u16) -> Option<u16> {
        self.port.checked_add(chunk).filter(|&p| p <= port_max)
    }
}

#[derive(Debug, Clone)]
pub struct AllPortStatus {
    open: HashSet<u16>,
    number_ports: u16,
    port_max: u16,
    pub closed: u16,
}

/// This is used on the second pass, to fill the second pass Struct based on the first
impl From<&Self> for AllPortStatus {
    fn from(value: &Self) -> Self {
        Self {
            open: HashSet::with_capacity(value.open_len().into()),
            closed: value.closed,
            number_ports: value.number_ports,
            port_max: value.port_max,
        }
    }
}

impl AllPortStatus {
    fn new(cli_args: &CliArgs) -> Self {
        Self {
            open: HashSet::with_capacity(64),
            closed: 0,
            number_ports: cli_args.ports.range_size,
            port_max: cli_args.ports.max,
        }
    }

    /// Create the actual output with descriptions
    /// Will only generate the port_descriptions hashset if there are any open ports and all ports have been scanned
    pub fn get_all_open<'a>(&self) -> Option<Vec<(u16, &'a str)>> {
        if !self.open.is_empty() && self.complete() {
            let port_details = PortDescriptions::new();
            let mut output = self
                .open
                .iter()
                .map(|i| (*i, port_details.get(*i)))
                .collect::<Vec<_>>();
            output.sort_by(|a, b| a.0.cmp(&b.0));
            Some(output)
        } else {
            None
        }
    }

    /// Get total number of open ports
    pub fn open_len(&self) -> u16 {
        u16::try_from(self.open.len()).unwrap_or_default()
    }

    /// Return true if all ports have been scanned
    fn complete(&self) -> bool {
        self.open_len() + self.closed == self.number_ports
    }

    /// Insert new port details, if true store port, if false just increase a counter
    fn insert(&mut self, message: PortMessage) {
        if message.open {
            self.open.insert(message.port);
        } else {
            self.closed += 1;
        }
    }

    /// Scan the given port, will be recursive if counter > 1
    /// Should this be changed to an async fn that just uses a while loop, instead of Box::pin recursion?
    fn scan_port(
        port: u16,
        ip: IpAddr,
        timeout: u32,
        sx: Sender<PortMessage>,
        counter: u8,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            if let Ok(Ok(_)) = tokio::time::timeout(
                std::time::Duration::from_millis(u64::from(timeout)),
                TcpStream::connect(format!("{ip}:{port}")),
            )
            .await
            {
                // Should one try to actually write to the port?
                // let open = stream.try_write(&[0]).is_ok();
                if sx.send(PortMessage { port, open: true }).await.is_err() {
                    exit(1);
                };
            } else if counter > 1 {
                Self::scan_port(port, ip, timeout, sx, counter - 1).await;
            } else if sx.send(PortMessage { port, open: false }).await.is_err() {
                exit(1);
            };
        })
    }

    /// Spawn a port scan into its own thread
    fn spawn_scan_port(port: u16, ip: IpAddr, timeout: u32, sx: Sender<PortMessage>, counter: u8) {
        tokio::spawn(Self::scan_port(port, ip, timeout, sx, counter));
    }

    /// Scan the entire range of selected ports by initiating multiple concurrent requests simultaneously
    async fn first_pass(cli_args: &CliArgs, ip: &IpAddr) -> Self {
        let mut first_pass = Self::new(cli_args);
        let retry = cli_args.retry + 1;

        let (sx, mut rx) = tokio::sync::mpsc::channel(usize::from(cli_args.concurrent));

        for port in cli_args.ports.get_range().take(cli_args.concurrent.into()) {
            Self::spawn_scan_port(port, *ip, cli_args.timeout, sx.clone(), retry);
        }

        while let Some(message) = rx.recv().await {
            first_pass.insert(message);

            // Need to manually close the receiver here, as we don't drop sx, and its being continually cloned
            if first_pass.complete() {
                rx.close();
            }

            if let Some(new_port) = message.can_spawn(cli_args.concurrent, first_pass.port_max) {
                Self::spawn_scan_port(new_port, *ip, cli_args.timeout, sx.clone(), retry);
            }
        }
        first_pass
    }

    /// Verify the discovered open ports through sequential scans instead of concurrent spawning.
    async fn second_pass(first_pass: Self, cli_args: &CliArgs, ip: &IpAddr) -> Self {
        let mut validated_result = Self::from(&first_pass);
        let (sx, mut rx) = tokio::sync::mpsc::channel(first_pass.open.len());
        for port in &first_pass.open {
            Self::scan_port(*port, *ip, cli_args.timeout, sx.clone(), cli_args.retry + 1).await;
        }
        drop(sx);
        while let Some(message) = rx.recv().await {
            validated_result.insert(message);
        }
        validated_result
    }

    /// Scan the ports, first pass spawns the request, second pass runs in series
    pub async fn scan_ports(cli_args: &CliArgs, ip: &IpAddr) -> Self {
        let first_pass = Self::first_pass(cli_args, ip).await;
        if first_pass.open_len() > 0 {
            Self::second_pass(first_pass, cli_args, ip).await
        } else {
            first_pass
        }
    }
}

#[cfg(test)]
impl AllPortStatus {
    pub const fn test_new(
        open: HashSet<u16>,
        number_ports: u16,
        port_max: u16,
        closed: u16,
    ) -> Self {
        Self {
            open,
            number_ports,
            port_max,
            closed,
        }
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used)]
mod tests {
    use std::{
        collections::HashSet,
        net::{IpAddr, Ipv4Addr},
    };

    use warp::Filter;

    use crate::{
        parse_arg,
        scanner::{AllPortStatus, PortMessage},
    };

    /// Start a server, on a given port, in own thread
    async fn start_server(port: u16) {
        let routes = warp::any().map(|| "Test Sever");
        tokio::spawn(warp::serve(routes).run(([127, 0, 0, 1], port)));
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }

    #[test]
    /// Check that a port can be spawned off. based on current port, chunk size, and upper limit
    fn test_scanner_can_portmessage_spawn() {
        let message = PortMessage {
            port: 80,
            open: true,
        };

        assert!(message.can_spawn(10, 81).is_none());
        assert!(message.can_spawn(10, 90).is_some());
        assert!(message.can_spawn(10, 91).is_some());
    }

    #[test]
    /// Depending on message open status, port gets inserted into HashSet, or closed total is increased
    fn test_scanner_get_insert() {
        let mut result = AllPortStatus {
            open: HashSet::new(),
            closed: 8,
            number_ports: 10,
            port_max: 10,
        };

        result.insert(PortMessage {
            port: 1,
            open: false,
        });
        assert_eq!(result.closed, 9);

        result.insert(PortMessage {
            port: 80,
            open: true,
        });
        assert_eq!(result.closed, 9);
        assert_eq!(result.open_len(), 1);
        assert!(result.open.contains(&80));
    }

    #[test]
    /// Scan all ports, at least 2 should be open
    fn test_scanner_get_open_none() {
        let mut result = AllPortStatus {
            open: HashSet::new(),
            closed: 8,
            number_ports: 10,
            port_max: 10,
        };

        // 8 of 10 ports scanned, so is_complete is false and get_all_open is none
        assert!(!result.complete());
        assert!(result.get_all_open().is_none());

        result.closed = 9;
        // 9 of 10 ports scanned, so is_complete is false and get_all_open is none
        assert!(!result.complete());
        assert!(result.get_all_open().is_none());

        // All ports now "scanned", so complete() and get_all_open is Some()
        result.closed = 10;
        assert!(result.complete());
        assert!(result.get_all_open().is_none());
    }

    #[test]
    /// Extract port descriptions Vec<(u16, &str)> when all scans complete, retrieve valid descriptions for given ports
    fn test_scanner_get_open_some() {
        let mut result = AllPortStatus {
            open: HashSet::new(),
            closed: 8,
            number_ports: 10,
            port_max: 10,
        };

        // 8 of 10 ports scanned, so is_complete is false
        assert!(!result.complete());
        assert!(result.get_all_open().is_none());

        result.open.insert(6379);
        // 9 of 10 ports scanned, so is_complete is false
        assert!(!result.complete());
        assert!(result.get_all_open().is_none());

        result.open.insert(443);
        assert!(result.complete());
        let result = result.get_all_open();
        assert!(result.is_some());
        let result = result.unwrap();

        assert_eq!(result.len(), 2);
        // First element is 443, as has been pre-sorted
        assert_eq!(result[0], (443, "https"));
        assert_eq!(result[1], (6379, "redis"));
    }

    #[tokio::test]
    /// Zero ports of 1-1000 open
    async fn test_scanner_1000_empty() {
        let cli_args = parse_arg::CliArgs::test_new("1-1000".to_owned(), 2048, None);
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let result = AllPortStatus::first_pass(&cli_args, &ip).await;
        assert_eq!(result.closed, 1000);
        assert_eq!(result.open.len(), 0);
    }

    #[tokio::test]
    /// Scan all ports, due to VSCode some ports might be open
    async fn test_scanner_all() {
        let cli_args = parse_arg::CliArgs::test_new("1-65535".to_owned(), 2048, None);
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let result = AllPortStatus::first_pass(&cli_args, &ip).await;
        println!("{result:#?}");
        assert_eq!(
            usize::from(result.closed) + usize::from(result.open_len()),
            65535
        );
    }

    #[tokio::test]
    /// Port 80 open, but only scan first 10, so zero response
    async fn test_scanner_10_port_80_empty() {
        start_server(80).await;
        let cli_args = parse_arg::CliArgs::test_new("1-10".to_owned(), 2048, None);
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let result = AllPortStatus::first_pass(&cli_args, &ip).await;
        assert_eq!(result.closed, 10);
        assert_eq!(result.open.len(), 0);
    }

    #[tokio::test]
    /// Port 80 open of first 1000
    async fn test_scanner_port_80() {
        start_server(80).await;
        let cli_args = parse_arg::CliArgs::test_new("1-1000".to_owned(), 2048, None);
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let result = AllPortStatus::first_pass(&cli_args, &ip).await;
        assert_eq!(result.closed, 999);
        assert_eq!(result.open.len(), 1);
        assert_eq!(
            usize::from(result.closed) + usize::from(result.open_len()),
            1000
        );
    }

    #[tokio::test]
    /// Port 80 and 443 open of first 1000
    async fn test_scanner_1000_80_443() {
        start_server(80).await;
        start_server(443).await;
        let cli_args = parse_arg::CliArgs::test_new("1-1000".to_owned(), 2048, None);
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let result = AllPortStatus::first_pass(&cli_args, &ip).await;
        assert_eq!(result.closed, 998);
        assert_eq!(result.open.len(), 2);
        assert_eq!(
            usize::from(result.closed) + usize::from(result.open_len()),
            1000
        );
    }

    #[tokio::test]
    /// Scan all ports, at least 2 should be open
    async fn test_scanner_all_80() {
        start_server(80).await;
        let cli_args = parse_arg::CliArgs::test_new("1-65535".to_owned(), 2048, None);
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let result = AllPortStatus::first_pass(&cli_args, &ip).await;
        assert_eq!(
            usize::from(result.closed) + usize::from(result.open_len()),
            65535
        );
        // Random ports can be open when developing in VSCode Dev Container, so just check that open ports is minimum 2
        assert!(result.open.len() >= 2);
        let result = result.get_all_open();
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.contains(&(80, "http")));
    }
}
