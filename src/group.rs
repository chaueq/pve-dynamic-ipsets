use std::{fs::File, io::{BufRead, BufReader}};

use crate::{domain::Domain, domain_store::DomainStore, rule::DynRule};

pub struct Group {
    name: String,
    domains: Vec<String>,
    static_rules: Vec<String>,
    dynamic_rules: Vec<DynRule>
}

impl Group {
    pub fn read(name: String, path: String, store: &mut DomainStore) -> Option<Self> {
        if let Ok(file) = File::open(path) {
            let source = BufReader::new(file);
            let mut state = ReadState::None;
            let mut domains: Vec<String> = Vec::new();
            let mut static_rules: Vec<String> = Vec::new();
            let mut dynamic_rules: Vec<DynRule> = Vec::new();

            for line in source.lines() {
                match line {
                    Ok(line) => {
                        if line.len() == 0 {
                            continue;
                        }

                        if let Some(s) = ReadState::from_string(&line) {
                            state = s;
                            continue;
                        }

                        match state {
                            ReadState::None => {
                                return None;
                            },
                            ReadState::Domains => {
                                if let Some(domain) = Domain::from_string(&line) {
                                    if !domains.contains(&domain.get_fqdn()) {
                                        domains.push(domain.get_fqdn());
                                    }
                                    store.ingest_domain(domain);
                                }
                                else {
                                    println!("Failed to load domain from: {line}");
                                }
                            },
                            ReadState::StaticRules => {
                                if !static_rules.contains(&line) {
                                    static_rules.push(line);
                                }
                            },
                            ReadState::DynamicRules => {
                                if let Some(rule) = DynRule::from_string(line) {
                                    if !dynamic_rules.contains(&rule) {
                                        dynamic_rules.push(rule);
                                    }
                                }
                            },
                        }
                    },
                    Err(_) => {
                        return None;
                    }
                }
            }

            return Some(Self {
                name,
                domains,
                static_rules,
                dynamic_rules
            })
        }
        None
    }

    pub fn render(&self, store: &DomainStore) -> String {
        let mut buf: String = format!("[group {}]\n\n", self.name);
        for rule in &self.static_rules {
            buf += format!("{rule}\n").as_str();
        }
        for rule in &self.dynamic_rules {
            for fqdn in &self.domains {
                if let Some(domain) = store.get(&fqdn) {
                    if domain.verify() {
                        buf += rule.render(domain).as_str();
                    }
                }
            }
        }
        buf += "\n";
        buf
    }
}

#[derive(PartialEq)]
enum ReadState {
    None,
    Domains,
    StaticRules,
    DynamicRules
}

impl ReadState {
    pub fn from_string(s: &String) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "[domains]" => Some(Self::Domains),
            "[static rules]" => Some(Self::StaticRules),
            "[dynamic rules]" => Some(Self::DynamicRules),
            _ => None
        }
    }
}