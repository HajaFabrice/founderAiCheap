use crate::config::InternetCheckConfig;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn internet_is_available(config: &InternetCheckConfig) -> bool {
    if !config.enabled {
        return true;
    }

    for probe in &config.hosts {
        if let Ok(addrs) = (probe.host.as_str(), probe.port).to_socket_addrs() {
            for addr in addrs {
                if TcpStream::connect_timeout(&addr, Duration::from_secs_f64(config.timeout_seconds)).is_ok() {
                    return true;
                }
            }
        }
    }

    false
}
