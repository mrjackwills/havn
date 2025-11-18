pub mod host_info;
use crate::{exit, parse_arg::CliArgs, port_descriptions::PortDescriptions};
use async_channel::Sender;
use std::{collections::HashSet, future::Future, net::IpAddr, pin::Pin};
use tokio::net::TcpStream;

#[derive(Debug, Clone, Hash, Copy)]
struct PortMessage {
    open: bool,
    port: u16,
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
            closed: value.closed,
            number_ports: value.number_ports,
            open: HashSet::with_capacity(value.open_len().into()),
            port_max: value.port_max,
        }
    }
}

impl AllPortStatus {
    fn new(cli_args: &CliArgs) -> Self {
        Self {
            closed: 0,
            number_ports: cli_args.ports_len(),
            open: HashSet::with_capacity(64),
            port_max: cli_args.port_range.end,
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
        counter: u8,
        ip: IpAddr,
        port: u16,
        sx: Sender<PortMessage>,
        timeout: u32,
        verbose: Option<u8>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let now = std::time::Instant::now();

        // Print some information when in verbose mode
        let verbose_print_open = move |status: bool| {
            if let Some(verbose) = verbose {
                println!(
                    "port: {port:>5}, attempt: #{attempt:>2}, status: {status:>6}, time: {ms:>4} ms",
                    attempt = verbose - counter + 1,
                    ms = now.elapsed().as_millis(),
                    status = if status { "open" } else { "closed" }
                );
            }
        };

        Box::pin(async move {
            if let Ok(Ok(_)) = tokio::time::timeout(
                std::time::Duration::from_millis(u64::from(timeout)),
                TcpStream::connect(format!("{ip}:{port}")),
            )
            .await
            {
                verbose_print_open(true);
                // Should one try to actually write to the port?
                // let open = stream.try_write(&[0]).is_ok();
                if sx.send(PortMessage { port, open: true }).await.is_err() {
                    exit(1);
                }
            } else {
                verbose_print_open(false);
                if counter > 0 {
                    Self::scan_port(counter - 1, ip, port, sx, timeout, verbose).await;
                } else if sx.send(PortMessage { port, open: false }).await.is_err() {
                    exit(1);
                }
            }
        })
    }

    /// Spawn a port scan into its own thread
    fn spawn_scan_port(
        counter: u8,
        ip: IpAddr,
        port: u16,
        sx: Sender<PortMessage>,
        timeout: u32,
        verbose: Option<u8>,
    ) {
        tokio::spawn(Self::scan_port(counter, ip, port, sx, timeout, verbose));
    }

    /// Scan the entire range of selected ports by initiating multiple concurrent requests simultaneously
    async fn first_pass(cli_args: &mut CliArgs, ip: &IpAddr) -> Self {
        let mut first_pass = Self::new(cli_args);
        let counter = cli_args.retry;

        let (sx, rx) = async_channel::bounded(usize::from(cli_args.concurrent));

        let mut to_spawn = cli_args.ports_split();

        let verbose = if cli_args.verbose.is_some() {
            Some(counter)
        } else {
            None
        };

        while let Some(port) = cli_args.ports_pop() {
            Self::spawn_scan_port(counter, *ip, port, sx.clone(), cli_args.timeout, verbose);
        }

        while let Ok(message) = rx.recv().await {
            first_pass.insert(message);

            // Need to manually close the receiver here, as we don't drop sx, and its being continually cloned
            if first_pass.complete() {
                rx.close();
            }

            if let Some(port) = to_spawn.pop() {
                Self::spawn_scan_port(counter, *ip, port, sx.clone(), cli_args.timeout, verbose);
            }
        }
        first_pass
    }

    /// Verify the discovered open ports through sequential scans instead of concurrent spawning.
    async fn second_pass(first_pass: Self, cli_args: CliArgs, ip: &IpAddr) -> Self {
        let mut validated_result = Self::from(&first_pass);
        let (sx, rx) = async_channel::bounded(first_pass.open.len());
        let verbose = if cli_args.verbose.is_some() {
            Some(cli_args.retry)
        } else {
            None
        };

        for port in &first_pass.open {
            Self::scan_port(
                cli_args.retry,
                *ip,
                *port,
                sx.clone(),
                cli_args.timeout,
                verbose,
            )
            .await;
        }
        drop(sx);
        while let Ok(message) = rx.recv().await {
            validated_result.insert(message);
        }
        validated_result
    }

    /// Scan the ports, first pass spawns the request, second pass runs in series
    pub async fn scan_ports(mut cli_args: CliArgs, ip: &IpAddr) -> Self {
        let first_pass = Self::first_pass(&mut cli_args, ip).await;
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
        net::{IpAddr, Ipv4Addr, Ipv6Addr},
    };

    use warp::Filter;

    use crate::{
        parse_arg,
        scanner::{AllPortStatus, PortMessage},
    };

    const V4_LOCAL: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
    const V6_LOCAL: IpAddr = IpAddr::V6(Ipv6Addr::LOCALHOST);

    /// Start a server, on a given port, in own thread, on both IPv4 and IPv6 interfaces
    async fn start_server(port: u16) {
        let routes = warp::any().map(|| "Test Sever");

        tokio::spawn(warp::serve(routes).run((V4_LOCAL, port)));
        tokio::spawn(warp::serve(routes).run((V6_LOCAL, port)));
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
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
        let mut cli_args = parse_arg::CliArgs::test_new("1-1000".to_owned(), 2048, None, false);
        let result = AllPortStatus::first_pass(&mut cli_args, &V4_LOCAL).await;
        assert_eq!(result.closed, 1000);
        assert_eq!(result.open.len(), 0);
    }

    #[tokio::test]
    /// Scan all ports, due to VSCode some ports might be open
    async fn test_scanner_all() {
        let mut cli_args = parse_arg::CliArgs::test_new("1-65535".to_owned(), 2048, None, false);
        let result = AllPortStatus::first_pass(&mut cli_args, &V4_LOCAL).await;
        println!("{result:#?}");
        assert_eq!(
            usize::from(result.closed) + usize::from(result.open_len()),
            65535
        );
    }

    #[tokio::test]
    /// Port 80 open, but only scan first 10, so zero response, on both IPv4 and IPv6 interfaces
    async fn test_scanner_10_port_80_empty() {
        start_server(80).await;
        let mut cli_args = parse_arg::CliArgs::test_new("1-10".to_owned(), 2048, None, false);
        let result = AllPortStatus::first_pass(&mut cli_args, &V4_LOCAL).await;
        assert_eq!(result.closed, 10);
        assert_eq!(result.open.len(), 0);

        let mut cli_args = parse_arg::CliArgs::test_new("1-10".to_owned(), 2048, Some("::1"), true);
        let result = AllPortStatus::first_pass(&mut cli_args, &V6_LOCAL).await;
        assert_eq!(result.closed, 10);
        assert_eq!(result.open.len(), 0);
    }

    #[tokio::test]
    /// Port 80 open of first 1000, on both IPv4 and IPv6 interfaces
    async fn test_scanner_port_80() {
        start_server(80).await;
        let mut cli_args = parse_arg::CliArgs::test_new("1-1000".to_owned(), 2048, None, false);
        let result = AllPortStatus::first_pass(&mut cli_args, &V4_LOCAL).await;
        assert_eq!(result.closed, 999);
        assert_eq!(result.open.len(), 1);
        assert_eq!(
            usize::from(result.closed) + usize::from(result.open_len()),
            1000
        );

        let mut cli_args =
            parse_arg::CliArgs::test_new("1-1000".to_owned(), 2048, Some("::1"), true);
        let result = AllPortStatus::first_pass(&mut cli_args, &V6_LOCAL).await;
        assert_eq!(result.closed, 999);
        assert_eq!(result.open.len(), 1);
        assert_eq!(
            usize::from(result.closed) + usize::from(result.open_len()),
            1000
        );
    }

    #[tokio::test]
    /// Port 80 and 443 open of first 1000, on both IPv4 and IPv6 interfaces
    async fn test_scanner_1000_80_443() {
        start_server(80).await;
        start_server(443).await;
        let mut cli_args = parse_arg::CliArgs::test_new("1-1000".to_owned(), 2048, None, false);
        let result = AllPortStatus::first_pass(&mut cli_args, &V4_LOCAL).await;
        assert_eq!(result.closed, 998);
        assert_eq!(result.open.len(), 2);
        assert_eq!(
            usize::from(result.closed) + usize::from(result.open_len()),
            1000
        );

        let mut cli_args =
            parse_arg::CliArgs::test_new("1-1000".to_owned(), 2048, Some("::1"), false);
        let result = AllPortStatus::first_pass(&mut cli_args, &V6_LOCAL).await;
        assert_eq!(result.closed, 998);
        assert_eq!(result.open.len(), 2);
        assert_eq!(
            usize::from(result.closed) + usize::from(result.open_len()),
            1000
        );
    }

    #[tokio::test]
    /// Scan all ports, at least 2 should be open, on both IPv4 and IPv6 interfaces
    async fn test_scanner_all_80() {
        start_server(80).await;
        let mut cli_args = parse_arg::CliArgs::test_new("1-65535".to_owned(), 2048, None, false);
        let result = AllPortStatus::first_pass(&mut cli_args, &V4_LOCAL).await;
        assert_eq!(
            usize::from(result.closed) + usize::from(result.open_len()),
            65535
        );
        assert!(!result.open.is_empty());
        let result = result.get_all_open();
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.contains(&(80, "http")));

        let mut cli_args =
            parse_arg::CliArgs::test_new("1-65535".to_owned(), 2048, Some("::1"), false);
        let result = AllPortStatus::first_pass(&mut cli_args, &V6_LOCAL).await;
        assert_eq!(
            usize::from(result.closed) + usize::from(result.open_len()),
            65535
        );
        assert!(!result.open.is_empty());
        let result = result.get_all_open();
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.contains(&(80, "http")));
    }
}

// TODO test ports_split, is order, and split correctly, use a 100 spit 10, 100 spit 1, and 1000 split 10
