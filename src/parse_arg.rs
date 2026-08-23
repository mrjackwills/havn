use std::collections::VecDeque;

use clap::Parser;
pub const PORT_UPPER_DEFAULT: u16 = 1000;

#[derive(Parser, Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
#[command(version, about)]
pub struct Cli {
    /// The target to scan, accepts IP address or domain name
    #[clap(default_value = "127.0.0.1", default_value_if("ip_v6", "true", "::1"))]
    address: String,

    /// Scan all ports, conflicts with "-p".
    #[clap(short = 'a', default_value_t = false, conflicts_with = "ports")]
    all_ports: bool,

    /// Maximum number of concurrent requests.
    #[clap(short = 'c', value_name = "concurrent", default_value_t = 1000)]
    concurrent: u16,

    /// Ports to scan, accepts a range, or single port, conflicts with "-a".
    #[clap(
        short = 'p',
        value_name = "ports",
        default_value = "-1000",
        conflicts_with = "all_ports",
        allow_hyphen_values = true
    )]
    ports: String,

    /// Monochrome mode - remove text colouring
    #[clap(short = 'm', default_value_t = false)]
    monochrome: bool,

    /// Maximum number of retry attempts per port.
    #[clap(short = 'r', value_name = "retries", default_value_t = 1)]
    retry: u8,

    /// Timeout for port scanning in milliseconds.
    #[clap(short = 't', value_name = "ms", default_value_t = 2000)]
    timeout: u32,

    /// Enable IPv6 scanning. Defaults to IPv4.
    #[clap(short = '6', default_value_t = false)]
    ip_v6: bool,

    /// Enable verbose debugging.
    #[clap(long = "verbose", default_value_t = false)]
    verbose: bool,
}

/// Create a VecDeque<u16> (aka ports) from the CliArgs.
/// Pop from front, from smallest to largest, no need to reverse
/// Can maybe introduce a "shuffle" mode, so scan ports in a random order, with buffered_unordered partially random?
/// although not sure if that would have any benefit, and would require using another dependency
impl From<&Cli> for PortRange {
    fn from(cli: &Cli) -> Self {
        let (start, end) = if cli.all_ports {
            (1, u16::MAX)
        } else if cli.ports.contains('-') {
            let (start, end) = cli.ports.split_once('-').unwrap_or_default();
            let start = start.parse::<u16>().unwrap_or(1);
            let end = end.parse::<u16>().unwrap_or(PORT_UPPER_DEFAULT);

            if start <= end {
                (start, end)
            } else {
                (end, start)
            }
        } else {
            cli.ports
                .parse::<u16>()
                .map_or((1, PORT_UPPER_DEFAULT), |i| (i, i))
        };

        Self {
            start,
            end,
            ports: (start..=end).collect::<VecDeque<_>>(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
    ports: VecDeque<u16>,
}

#[derive(Debug, Clone)]
pub struct CliArgs {
    pub address: String,
    pub concurrent: u16,
    pub ip6: bool,
    pub monochrome: bool,
    pub port_range: PortRange,
    pub retry: u8,
    pub timeout: u32,
    pub verbose: Option<()>,
}

impl CliArgs {
    /// Check if a given arg is 0, and if so, return as 1, else return value
    fn check_if_zero<T: PartialEq + From<u8>>(x: T) -> T {
        if x == 0.into() { T::from(1) } else { x }
    }

    pub fn new() -> Self {
        Self::from(Cli::parse())
    }

    /// Get the total number of ports to scan
    pub fn ports_len(&self) -> u16 {
        u16::try_from(self.port_range.ports.len()).unwrap_or_default()
    }

    /// Remove the first entry from the ports deque
    pub fn ports_pop(&mut self) -> Option<u16> {
        self.port_range.ports.pop_front()
    }
}

/// Build CliArgs from the Cli, converting zero-value concurrent/timeout args to 1
impl From<Cli> for CliArgs {
    fn from(cli: Cli) -> Self {
        let port_range = PortRange::from(&cli);
        Self {
            address: cli.address,
            concurrent: Self::check_if_zero(cli.concurrent),
            ip6: cli.ip_v6,
            monochrome: cli.monochrome,
            port_range,
            retry: cli.retry,
            timeout: Self::check_if_zero(cli.timeout),
            verbose: if cli.verbose { Some(()) } else { None },
        }
    }
}

#[cfg(test)]
impl CliArgs {
    pub fn test_new(ports: String, concurrent: u16, address: Option<&str>, ip_v6: bool) -> Self {
        let adr = if ip_v6 && address.is_none() {
            "::1"
        } else {
            "127.0.0.1"
        };

        let cli = Cli {
            address: address.unwrap_or(adr).to_owned(),
            all_ports: false,
            concurrent,
            ip_v6,
            monochrome: false,
            ports,
            retry: 1,
            timeout: 1250,
            verbose: false,
        };
        let ports = PortRange::from(&cli);

        Self {
            address: cli.address,
            concurrent: cli.concurrent,
            ip6: cli.ip_v6,
            monochrome: cli.monochrome,
            port_range: ports,
            retry: cli.retry,
            timeout: cli.timeout,
            verbose: None,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use clap::Parser;

    use crate::parse_arg::{CliArgs, PortRange};

    use super::Cli;

    /// Re-useable test to make sure ports get parsed correctly
    fn test(ports: &str, min: u16, max: u16, range: u16) {
        let result = PortRange::from(&Cli {
            address: "127.0.0.1".to_owned(),
            all_ports: false,
            concurrent: 1024,
            ip_v6: false,
            monochrome: false,
            ports: ports.to_owned(),
            retry: 1,
            timeout: 1000,
            verbose: false,
        });
        // We can't assume this is in shuffle mode!
        assert_eq!(result.start, min);
        assert_eq!(result.end, max);
        assert_eq!(result.ports.len(), usize::from(range));
    }

    #[test]
    /// Test that ports are parsed correctly, everything else should be handled by Clap directly
    fn test_cli_port_range() {
        test("1-1000", 1, 1000, 1000);
        test("1000-1", 1, 1000, 1000);
        test("100-200", 100, 200, 101);
        test("100-", 100, 1000, 901);
        test("80", 80, 80, 1);
        test("1-1000000", 1, 1000, 1000);
        test("65536-1000000", 1, 1000, 1000);
        test("random", 1, 1000, 1000);

        let result = PortRange::from(&Cli {
            monochrome: false,
            address: "127.0.0.1".to_owned(),
            all_ports: true,
            concurrent: 1024,
            ip_v6: false,
            ports: String::new(),
            retry: 100,
            timeout: 1000,
            verbose: false,
        });
        assert_eq!(result.start, 1);
        assert_eq!(result.end, 65535);
        assert_eq!(result.ports.len(), 65535);
    }

    #[test]
    /// Timeout and concurrent values are changed from 0 to 1
    fn test_cli_zero_values() {
        let cli = Cli::try_parse_from(["havn", "-c", "0", "-t", "0"]).unwrap();
        let args = CliArgs::from(cli);
        assert_eq!(args.concurrent, 1);
        assert_eq!(args.timeout, 1);

        let cli = Cli::try_parse_from(["havn", "-c", "10", "-t", "0"]).unwrap();
        let args = CliArgs::from(cli);
        assert_eq!(args.concurrent, 10);
        assert_eq!(args.timeout, 1);

        let cli = Cli::try_parse_from(["havn", "-c", "0", "-t", "10"]).unwrap();
        let args = CliArgs::from(cli);
        assert_eq!(args.concurrent, 1);
        assert_eq!(args.timeout, 10);

        let cli = Cli::try_parse_from(["havn", "-c", "10", "-t", "10"]).unwrap();
        let args = CliArgs::from(cli);
        assert_eq!(args.concurrent, 10);
        assert_eq!(args.timeout, 10);
    }
}
