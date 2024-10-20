mod color;
pub mod spinner;

pub use color::text_color;

pub mod print {
    use std::net::IpAddr;

    use crate::{
        exit,
        parse_arg::CliArgs,
        scanner::{host_info::HostInfo, AllPortStatus},
        terminal::color::Color,
    };

    /// Generate a string from a Duration, in format "x.xxxs"
    fn ms_to_string(dur: std::time::Duration) -> String {
        let as_ms = dur.as_millis();
        let as_sec = dur.as_secs();
        format!(
            "{}.{:<03}s",
            as_sec,
            as_ms.saturating_sub(u128::from(as_sec * 1000))
        )
    }

    /// Generate an Optional<String> of any additional IP addresses
    fn get_extra_ips(host_info: &HostInfo, ip: &IpAddr) -> Option<String> {
        let ip_len = host_info.ip_len();
        if ip_len > 1 {
            let mut output = "Other IPs: ".to_owned();
            host_info
                .iter_ip()
                .filter(|z| z != &ip)
                .enumerate()
                .for_each(|(index, i)| {
                    let prefix = if index > 0 { ", " } else { "" };
                    output.push_str(&format!("{prefix}{i}"));
                });
            Some(output)
        } else {
            None
        }
    }

    /// Generate information about the host/address/IP that will be scanned, to be shown on two lines along with the application name
    fn get_host_ip(cli_args: &CliArgs, ip: &IpAddr) -> (String, String, String) {
        let ports = if cli_args.ports.min == cli_args.ports.max {
            format!("{}", cli_args.ports.min)
        } else {
            format!("{}-{}", cli_args.ports.min, cli_args.ports.max)
        };
        if cli_args.address == ip.to_string() {
            (format!("{}:", cli_args.address), ports, String::new())
        } else {
            (format!("{ip}:"), ports, cli_args.address.to_string())
        }
    }

    /// Generate the string for scan_time function
    fn get_scan_time(result: &AllPortStatus, done: std::time::Duration) -> String {
        format!(
            "{g}{t} open{r}, {re}{cl} closed{r}, took {ms}",
            g = Color::Green,
            t = result.open_len(),
            r = Color::Reset,
            re = Color::Red,
            cl = result.closed,
            ms = ms_to_string(done)
        )
    }

    /// Generate the results table, assuming there are open ports
    fn get_table(result: &AllPortStatus) -> Option<String> {
        result.get_all_open().map(|ports| {
            let mut output = format!(
                "{u}PORT{r}  {u}DESCRIPTION{r}",
                u = Color::Underline,
                r = Color::Reset
            );

            for (index, (port, desc)) in ports.iter().enumerate() {
                let color = if index % 2 == 0 {
                    Color::Yellow
                } else {
                    Color::Reset
                };
                output.push_str(&format!("\n{}{port:<5} {desc}{}", color, Color::Reset));
            }
            output
        })
    }

    /// Print invalid address to screen, then quit with error code 1
    pub fn address_error(cli_args: &CliArgs) {
        println!(
            "{c}Error with address: {r}{a}",
            a = cli_args.address,
            c = Color::Red,
            r = Color::Reset,
        );
        exit(1);
    }

    /// Print any additional IP's, ignoring the IP that will be scanned
    pub fn extra_ips(host_info: &HostInfo, ip: &IpAddr) {
        if let Some(extra_ips) = get_extra_ips(host_info, ip) {
            println!("{extra_ips}");
        }
    }

    /// Print the name of the application, using a small figlet font
    pub fn name_and_target(cli_args: &CliArgs, ip: &IpAddr) {
        let (ip, ports, address) = get_host_ip(cli_args, ip);
        let bar = (0..=address
            .chars()
            .count()
            .max(ip.chars().count() + ports.chars().count())
            + 19)
            .map(|_| '═')
            .collect::<String>();
        println!("{m}{bar}\n|__|  /\\  \\  / |\\ |{r} {ip}{y}{ports}{r}\n{m}|  | /--\\  \\/  | \\|{r} {address}\n{m}{bar}{r}",
        m = Color::Magenta,
        y = Color::Yellow,
        r = Color::Reset
);
    }

    /// If any open ports found, print the results into a table
    pub fn result_table(result: &AllPortStatus) {
        if let Some(result) = get_table(result) {
            println!("{result}");
        }
    }

    /// Print totals of open & closed ports, and the time it took to scan them
    pub fn scan_time(result: &AllPortStatus, done: std::time::Duration) {
        println!("{}", get_scan_time(result, done));
    }

    #[cfg(test)]
    #[expect(clippy::unwrap_used)]
    mod tests {
        use std::{
            collections::HashSet,
            net::{Ipv4Addr, Ipv6Addr},
        };

        use crate::{parse_arg, terminal::color::MONOCHROME};

        use super::*;

        #[test]
        /// A duration is formatted into a "X.YYYs" string
        fn test_terminal_ms_to_string() {
            assert_eq!(
                ms_to_string(std::time::Duration::from_millis(250)),
                "0.250s"
            );
            assert_eq!(
                ms_to_string(std::time::Duration::from_millis(1000)),
                "1.000s"
            );
            assert_eq!(
                ms_to_string(std::time::Duration::from_millis(1250)),
                "1.250s"
            );
            assert_eq!(
                ms_to_string(std::time::Duration::from_millis(12_8250)),
                "128.250s"
            );
        }

        #[test]
        /// Generate extra ip function will either return None, or a String in format "Other IPs: xx"
        fn test_terminal_generate_extra_ips() {
            let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            let ips = Vec::from([ip]);
            let host_info = HostInfo::test_get(ips);
            assert!(get_extra_ips(&host_info, &ip).is_none());

            let ip_2 = IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255));
            let ips = Vec::from([ip, ip_2]);
            let host_info = HostInfo::test_get(ips);
            let result = get_extra_ips(&host_info, &ip);
            assert!(result.is_some());
            assert_eq!(result.unwrap(), "Other IPs: 255.255.255.255");

            let ip_3 = IpAddr::V6(Ipv6Addr::new(127, 126, 125, 124, 123, 122, 121, 120));
            let ips = Vec::from([ip, ip_2, ip_3]);
            let host_info = HostInfo::test_get(ips);
            let result = get_extra_ips(&host_info, &ip);
            assert!(result.is_some());
            assert_eq!(
                get_extra_ips(&host_info, &ip).unwrap(),
                "Other IPs: 255.255.255.255, 7f:7e:7d:7c:7b:7a:79:78"
            );
        }

        #[test]
        /// Get the IP and Address or just IP and a blank string, as well as the colorised port range
        fn test_terminal_host_ip() {
            let cli_args = parse_arg::CliArgs::test_new("1".to_owned(), 512, None, false);
            let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            assert_eq!(
                get_host_ip(&cli_args, &ip),
                ("127.0.0.1:".into(), "1".into(), String::new())
            );

            let cli_args = parse_arg::CliArgs::test_new("1-10000".to_owned(), 512, None, false);
            let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            assert_eq!(
                get_host_ip(&cli_args, &ip),
                ("127.0.0.1:".into(), "1-10000".into(), String::new())
            );

            let cli_args = parse_arg::CliArgs::test_new("1-65535".to_owned(), 512, None, false);
            let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            assert_eq!(
                get_host_ip(&cli_args, &ip),
                ("127.0.0.1:".into(), "1-65535".into(), String::new())
            );

            let cli_args =
                parse_arg::CliArgs::test_new("1".to_owned(), 512, Some("www.google.com"), false);
            let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            assert_eq!(
                get_host_ip(&cli_args, &ip),
                (
                    "127.0.0.1:".into(),
                    "1".into(),
                    String::from("www.google.com")
                )
            );

            let cli_args = parse_arg::CliArgs::test_new(
                "1-10000".to_owned(),
                512,
                Some("www.google.com"),
                false,
            );
            let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            assert_eq!(
                get_host_ip(&cli_args, &ip),
                (
                    "127.0.0.1:".into(),
                    "1-10000".into(),
                    String::from("www.google.com")
                )
            );

            let cli_args = parse_arg::CliArgs::test_new(
                "1-65535".to_owned(),
                512,
                Some("www.google.com"),
                false,
            );
            let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            assert_eq!(
                get_host_ip(&cli_args, &ip),
                (
                    "127.0.0.1:".into(),
                    "1-65535".into(),
                    String::from("www.google.com")
                )
            );
        }

        #[test]
        /// Generate extra ip function will either return None, or a String in format "Other IPs: xx"
        fn test_terminal_table() {
            let input = AllPortStatus::test_new(HashSet::new(), 10, 1, 0);
            assert!(get_table(&input).is_none());

            let input = AllPortStatus::test_new(HashSet::from([80]), 1, 80, 0);
            let result = get_table(&input);
            assert!(result.is_some());
            let result = result.unwrap();
            assert!(result.contains("PORT"));
            assert!(result.contains("DESCRIPTION"));
            assert!(result.contains("80 "));
            assert!(result.contains("http"));

            let input = AllPortStatus::test_new(HashSet::from([443, 6379, 32825]), 3, 32825, 0);
            let result = get_table(&input);
            assert!(result.is_some());
            let result = result.unwrap();
            assert!(result.contains("PORT"));
            assert!(result.contains("DESCRIPTION"));
            assert!(result.contains("443 "));
            assert!(result.contains("https"));
            assert!(result.contains("6379 "));
            assert!(result.contains("redis"));
            assert!(result.contains("32825 "));
            assert!(result.contains("unknown"));
        }

        #[test]
        /// Generate extra ip function will either return None, or a String in format "Other IPs: xx"
        fn test_terminal_scan_time() {
            let input = (
                AllPortStatus::test_new(HashSet::new(), 10, 1, 10),
                std::time::Duration::from_millis(100),
            );
            let result = get_scan_time(&input.0, input.1);
            assert!(result.contains("0 open"));
            assert!(result.contains("10 closed"));
            assert!(result.contains(", took 0.100s"));

            let input = (
                AllPortStatus::test_new(HashSet::from([1, 2, 3]), 10, 1, 6),
                std::time::Duration::from_millis(2500),
            );
            let result = get_scan_time(&input.0, input.1);
            assert!(result.contains("3 open"));
            assert!(result.contains("6 closed"));
            assert!(result.contains(", took 2.500s"));
        }

        #[test]
        /// Test that escape codes are printed to output
        fn test_terminal_monochrome_false() {
            MONOCHROME.store(false, std::sync::atomic::Ordering::SeqCst);
            let input = (
                AllPortStatus::test_new(HashSet::new(), 10, 1, 10),
                std::time::Duration::from_millis(100),
            );
            let result = get_scan_time(&input.0, input.1);

            assert!(result.contains("\x1b[32m0 open\x1b[0m"));
            assert!(result.contains("\x1b[31m10 closed\x1b[0m"));

            let input = AllPortStatus::test_new(HashSet::from([443, 6379, 32825]), 3, 32825, 0);
            let result = get_table(&input);
            assert!(result.is_some());
            let result = result.unwrap();

            assert!(result.contains("\x1b[33m443"));
            assert!(result.contains("https\x1b[0m"));

            assert!(result.contains("6379"));
            assert!(result.contains("redis"));

            assert!(result.contains("\x1b[33m32825"));
            assert!(result.contains("unknown\x1b[0m"));
        }

        #[test]
        /// Test that escape codes are not printed to output
        fn test_terminal_monochrome_true() {
            MONOCHROME.store(true, std::sync::atomic::Ordering::SeqCst);
            let input = (
                AllPortStatus::test_new(HashSet::new(), 10, 1, 10),
                std::time::Duration::from_millis(100),
            );
            let result = get_scan_time(&input.0, input.1);

            assert!(!result.contains("\x1b[32m0 open\x1b[0m"));
            assert!(!result.contains("\x1b[31m10 closed\x1b[0m"));

            let input = AllPortStatus::test_new(HashSet::from([443, 6379, 32825]), 3, 32825, 0);
            let result = get_table(&input);
            assert!(result.is_some());
            let result = result.unwrap();

            assert!(!result.contains("\x1b[33m443"));
            assert!(!result.contains("https\x1b[0m"));

            assert!(result.contains("6379"));
            assert!(result.contains("redis"));

            assert!(!result.contains("\x1b[33m32825"));
            assert!(!result.contains("unknown\x1b[0m"));
        }
    }
}
