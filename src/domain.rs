use std::{net::IpAddr, time::{Duration, Instant}};

use dns_lookup::lookup_host;

pub struct Domain {
    fqdn: String,
    interval: Duration,
    last_refresh: Option<Instant>,
    ips: Vec<IpAddr>
}

impl Domain {
    pub fn from_string(s: &String) -> Option<Self> {
        let parts: Vec<String> = s.split(' ').map(|x|x.to_string()).collect();
        if parts.len() == 2 {
            if let Ok(minutes) = u64::from_str_radix(&parts[1], 10) {
                let mut domain = Self {
                    fqdn: parts[0].clone(),
                    interval: Duration::from_secs(minutes * 60),
                    last_refresh: None,
                    ips: Vec::new()
                };

                domain.update();

                return Some(domain);
            }
        }
        None
    }

    pub fn get_name(&self) -> String {
        self.fqdn.replace('.', "_")
    }

    pub fn get_fqdn(&self) -> String {
        self.fqdn.clone()
    }

    fn update(&mut self) -> Option<bool> {
        println!("Updating domain: {}", self.fqdn);
        match lookup_host(&self.fqdn) {
            Ok(ips) => {
                if ips.len() == self.ips.len() {
                    for ip in &ips {
                        if !self.ips.contains(ip) {
                            self.ips = ips;
                            self.last_refresh = Some(Instant::now());
                            return Some(true);
                        }
                    }
                    self.last_refresh = Some(Instant::now());
                    return Some(false);
                }

                self.ips = ips;
                self.last_refresh = Some(Instant::now());
                Some(true)
            }
            Err(e) => {
                println!("Name resolve for {} failed. Keeping old config for this host. Error: {e}", &self.fqdn);
                None
            }
        }
    }

    pub fn try_update(&mut self) -> Option<bool> {
        if self.last_refresh.is_none() {
            return self.update();
        }
        if self.last_refresh.unwrap().elapsed() >= self.interval {
            return self.update();
        }
        Some(false)
    }

    pub fn try_render(&self) -> Option<String> {
        if self.last_refresh.is_some() && self.ips.len() > 0 {
            let mut result = format!("[IPSET domain_{}]\n\n", self.get_name());
            for ip in &self.ips {
                let suffix = {
                    if ip.is_ipv4() {"/32"}
                    else {"/128"}
                };
                result += format!("{}{suffix}\n", ip.to_string()).as_str();
            }
            result += "\n";
            return Some(result);
        }
        None
    }

    pub fn get_interval(&self) -> Duration {
        self.interval.clone()
    }

    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
    }

    pub fn verify(&self) -> bool {
        self.last_refresh.is_some()
        &&
        self.ips.len() > 0
    }
}