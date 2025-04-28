use std::{fs::{self, File, OpenOptions}, io::{BufRead, BufReader, BufWriter, Read, Write}, sync::mpsc::channel, thread::{self, sleep}, time::Duration};

use crate::{config::{Config, ProgramPath}, domain::Domain, domain_store::DomainStore, group::Group, module::Module, orig_cache::OrigCache};

pub fn start(config: Config) -> Module<ProcessorSignal> {
    let (sender, receiver) = channel::<ProcessorSignal>();

    let handle = thread::spawn(move || {
        let mut domains = DomainStore::new();
        let mut groups: Vec<Group> = Vec::new();
        let mut stat = OrigCache::new(config.get_path(ProgramPath::Static));
        let mut first_run = true;

        if let Ok(dir) = fs::read_dir(config.get_path(ProgramPath::Directory)) {
            for entry in dir {
                if let Ok(sig) = receiver.try_recv() {
                    match sig {
                        ProcessorSignal::Stop => {return}
                    }
                }

                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if path.to_string_lossy().ends_with(".group") {
                            let name = {
                                let parts: Vec<String> = path.to_str().unwrap().split('/').map(|x|x.to_string()).collect();
                                let parts: Vec<String> = parts[parts.len() - 1].split('.').map(|x|x.to_string()).collect();
                                parts[0].clone()
                            };
                            println!("Loading group: {}", name);
                            if let Some(group) = Group::read(name, path.to_string_lossy().to_string(), &mut domains) {
                                groups.push(group);
                            }
                        }
                        else if path.to_string_lossy().ends_with(".domains") {
                            if let Ok(file) = File::open(&path) {
                                println!("Loading domains from: {}", path.to_string_lossy());
                                
                                let reader = BufReader::new(file);
                                for line in reader.lines() {
                                    match line {
                                        Ok(line) => {
                                            if let Some(domain) = Domain::from_string(&line) {
                                                domains.ingest_domain(domain);
                                            }
                                        }
                                        Err(_) => {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        else {
                            println!("Skipping file {}", path.to_string_lossy());
                        }
                    }
                }
            }

            println!("Initialization finished with {} groups and {} domains", groups.len(), domains.len());

            loop {
                if let Ok(sig) = receiver.try_recv() {
                    match sig {
                        ProcessorSignal::Stop => {break}
                    }
                }
                else if domains.update() > 0 || stat.try_update() || first_run {
                    first_run = false;
                    println!("Starting generation of dynamic content");
                    if let Ok(file) = OpenOptions::new().create(true).truncate(true).write(true).open(config.get_path(ProgramPath::Generated)) {
                        {
                            let mut writer = BufWriter::new(file);
                        
                            stat.write(&mut writer);
                            
                            writeln!(&mut writer, "\n# DYNAMIC CONTENT BEGIN\n\n").unwrap();
                            
                            writer.write_all(domains.render().as_bytes()).unwrap();
                            
                            for group in &groups {
                                writer.write_all(group.render(&domains).as_bytes()).unwrap();
                            
                            }
                            
                            writeln!(&mut writer, "\n# DYNAMIC CONTENT END\n\n").unwrap();
                        }

                        let propagation_result = {
                            let mut buffer = String::new();
                            if let Ok(mut src_file) = File::open(config.get_path(ProgramPath::Generated)) {
                                if src_file.read_to_string(&mut buffer).is_ok() {
                                    if let Ok(mut dst_file) = OpenOptions::new()
                                    .create(true).truncate(true).write(true)
                                    .open(config.get_path(ProgramPath::Original)) {
                                        dst_file.write_all(buffer.as_bytes()).is_ok()
                                    }
                                    else {false}
                                }
                                else {false}
                            }
                            else {false}
                        };

                        if propagation_result {
                            println!("Propagated dynamic content to origin file");
                        }
                        else {
                            println!("Propagation failed");
                        }

                        stat.mark_as_updated();
                    }
                    else {
                        println!("Failed to open file destination file for writing");
                    }

                }
                else {
                    sleep(Duration::from_secs(15));
                }
            }
        }
    });

    Module::new(handle, sender)
}

pub enum ProcessorSignal {
    Stop
}