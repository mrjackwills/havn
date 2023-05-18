use std::collections::HashMap;

// https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml

/// Descriptions of a large range of ports, currently TCP only
pub struct PortDescriptions {
    tcp: HashMap<u16, &'static str>,
}

impl PortDescriptions {
    /// Generate the HashMap based on the "tcp.txt" file
    /// Only gets generated if open ports > 0
    pub fn new() -> Self {
        let txt = include_str!("tcp.txt");
        let txt_len = txt.lines().count();
        let mut port_details = HashMap::with_capacity(txt_len);

        for line in txt.lines() {
            let (name, port) = line.split_once(',').unwrap_or_default();
            if let Ok(port) = port.parse::<u16>() {
                port_details.insert(port, name);
            }
        }
        Self { tcp: port_details }
    }

    /// Get the description of a given port, with always return a &str, "unknown" for items with no descriptions
    /// At the moment it just gets TCP ports, need to implement UDP functionality later
    pub fn get<'a>(&self, port: u16) -> &'a str {
        self.tcp.get(&port).unwrap_or(&"unknown")
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
pub mod tests {

    use crate::port_descriptions::PortDescriptions;

    #[test]
    /// Check that known and unknown ports return the correct &str
    fn test_port_descriptions() {
        let values = PortDescriptions::new();

        let response = values.get(43);
        assert_eq!(response, "whois");

        let response = values.get(80);
        assert_eq!(response, "http");

        let response = values.get(443);
        assert_eq!(response, "https");

        let response = values.get(6379);
        assert_eq!(response, "redis");

        let response = values.get(5432);
        assert_eq!(response, "postgresql");

        let response = values.get(50001);
        assert_eq!(response, "unknown");
    }
}
