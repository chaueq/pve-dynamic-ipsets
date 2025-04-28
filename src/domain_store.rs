use std::collections::HashMap;

use crate::domain::Domain;

pub struct DomainStore {
    domains: HashMap<String, Domain>
}

impl DomainStore {
    pub fn new() -> Self {
        Self {
            domains: HashMap::new()
        }
    }

    pub fn ingest_domain(&mut self, domain: Domain) {
        if let Some(stored) = self.domains.get_mut(&domain.get_fqdn()) {
            if stored.get_interval() > domain.get_interval() {
                stored.set_interval(domain.get_interval());
            }
        }
        else {
            self.domains.insert(domain.get_fqdn(), domain);
        }
    }

    pub fn update(&mut self) -> usize {
        let mut changed: usize = 0;
        for domain in self.domains.values_mut() {
            if let Some(change_applied) = domain.try_update() {
                if change_applied {
                    changed += 1;
                }
            }
        }
        if changed > 0 {
            println!("Updated {} domains", changed);
        }
        changed
    }

    pub fn render(&self) -> String {
        let mut buf: String = String::new();
        for domain in self.domains.values() {
            if let Some(ipset) = domain.try_render() {
                buf += ipset.as_str();
            }
        }
        buf
    }

    pub fn get(&self, fqdn: &String) -> Option<&Domain> {
        self.domains.get(fqdn)
    }

    pub fn len(&self) -> usize {
        self.domains.len()
    }
}