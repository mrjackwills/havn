use std::ops::RangeInclusive;

use clap::Parser;

pub const PORT_UPPER_DEFAULT: u16 = 1000;

#[derive(Parser, Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
#[command(version, about)]
pub struct Cli {
    /// The target to scan, accepts IP address or domain name
    #[clap(default_value = "127.0.0.1")]
    address: String,

    /// Scan all ports, conflicts with "-p".
    #[clap(short = 'a', default_value_t = false, conflicts_with = "ports")]
    all_ports: bool,

    /// Maximum number of concurrent requests.
    #[clap(short = 'c', value_name = "concurrent", default_value_t = 1000)]
    concurrent: u16,

    /// Ports to scan, accepts a range, or single port, conflicts with "-p".
    #[clap(
        short = 'p',
        value_name = "ports",
        default_value = "-1000",
        conflicts_with = "all_ports",
        allow_hyphen_values = true
    )]
    ports: String,

    /// Maximum number of connection attempts per port.
    #[clap(short = 'r', value_name = "retries", default_value_t = 1)]
    retry: u8,

    /// Timeout for port scanning in milliseconds.
    #[clap(short = 't', value_name = "ms", default_value_t = 2000)]
    timeout: u32,

    /// Enable IPv6 scanning. Defaults to IPv4.
    #[clap(short = '6', default_value_t = false)]
    ip_v6: bool,
}

#[derive(Debug, Clone)]
pub struct PortRange {
    pub min: u16,
    pub max: u16,
    pub range_size: u16,
}

impl PortRange {
    /// Generate an inclusive range, given the min and max port
    pub const fn get_range(&self) -> RangeInclusive<u16> {
        self.min..=self.max
    }
}

/// Parse port ranges, any parsing errors will return default values
impl From<&Cli> for PortRange {
    fn from(cli: &Cli) -> Self {
        if cli.all_ports {
            Self {
                min: 1,
                max: u16::MAX,
                range_size: u16::MAX,
            }
        } else if cli.ports.contains('-') {
            let (start, end) = cli.ports.split_once('-').unwrap_or_default();
            let start = start.parse::<u16>().unwrap_or(1);
            let end = end.parse::<u16>().unwrap_or(PORT_UPPER_DEFAULT);
            let range_size = start.abs_diff(end) + 1;

            if start <= end {
                Self {
                    min: start,
                    max: end,
                    range_size,
                }
            } else {
                Self {
                    min: end,
                    max: start,
                    range_size,
                }
            }
        } else {
            cli.ports.parse::<u16>().map_or(
                Self {
                    min: 1,
                    max: PORT_UPPER_DEFAULT,
                    range_size: PORT_UPPER_DEFAULT,
                },
                |single_port| Self {
                    min: single_port,
                    max: single_port,
                    range_size: 1,
                },
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct CliArgs {
    pub address: String,
    pub concurrent: u16,
    pub ip6: bool,
    pub ports: PortRange,
    pub retry: u8,
    pub timeout: u32,
}

impl CliArgs {
    pub fn new() -> Self {
        let cli = Cli::parse();

        let port_range = PortRange::from(&cli);

        Self {
            address: cli.address,
            concurrent: cli.concurrent,
            ip6: cli.ip_v6,
            ports: port_range,
            retry: cli.retry,
            timeout: cli.timeout,
        }
    }
}

#[cfg(test)]
impl CliArgs {
    pub fn test_new(ports: String, concurrent: u16, address: Option<&str>) -> Self {
        let cli = Cli {
            address: address.unwrap_or("127.0.0.1").to_owned(),
            all_ports: false,
            concurrent,
            ports,
            retry: 1,
            timeout: 1250,
            ip_v6: false,
        };
        let port_range = PortRange::from(&cli);

        Self {
            address: cli.address,
            concurrent: cli.concurrent,
            ip6: cli.ip_v6,
            ports: port_range,
            retry: cli.retry,
            timeout: cli.timeout,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
mod tests {
    use super::{Cli, PortRange};

	/// Re-useable test to make sure ports get parsed correctly
    fn test(ports: &str, min: u16, max: u16, range: u16) {
        let result = PortRange::from(&Cli {
            address: "127.0.0.1".to_owned(),
            all_ports: false,
            concurrent: 1024,
            ports: ports.to_owned(),
            retry: 1,
            timeout: 1000,
            ip_v6: false,
        });
        assert_eq!(result.min, min);
        assert_eq!(result.max, max);
        assert_eq!(result.range_size, range);
		assert_eq!(result.get_range(), (min..=max))
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
            address: "127.0.0.1".to_owned(),
            all_ports: true,
            concurrent: 1024,
            ports: "".to_owned(),
            retry: 100,
            timeout: 1000,
            ip_v6: false,
        });
        assert_eq!(result.min, 1);
        assert_eq!(result.max, 65535);
        assert_eq!(result.range_size, 65535);
    }
}
