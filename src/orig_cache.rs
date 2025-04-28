use std::{io::Write, fs::{self, File}, io::{BufRead, BufReader}, time::Instant};

pub struct OrigCache {
    content: String,
    last_updated: Instant,
    path: String
}

impl OrigCache {
    pub fn new(path: String) -> Self {
        let mut oc = Self {
            content: String::new(),
            last_updated: Instant::now(),
            path
        };
        oc.update();
        oc
    }

    fn update(&mut self) -> bool  {
        if let Ok(file) = File::open(&self.path) {
            let mut state = ReadState::Orig;
            let reader: BufReader<File> = BufReader::new(file);
            let mut buf = String::new();
            let mut last_was_empty = false;
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        if line == "# DYNAMIC CONTENT BEGIN" {
                            state = ReadState::Dynamic;
                            continue;
                        }
                        else if line == "# DYNAMIC CONTENT END" {
                            state = ReadState::Orig;
                            continue;
                        }
                        
                        if state == ReadState::Dynamic {
                            continue;
                        }

                        if line == "" {
                            if last_was_empty {
                                continue;
                            }
                            last_was_empty = true;
                        }
                        else {
                            last_was_empty = false;
                        }

                        buf += format!("{line}\n").as_str();
                    }
                    Err(_) => {return false;}
                }
            }
            self.content = buf;
            println!("Updated origin file");
            self.mark_as_updated();
            return true;
        }
        false
    }

    pub fn try_update(&mut self) -> bool{
        if let Ok(metadata) = fs::metadata(&self.path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(since_modif) = modified.elapsed() {
                    if since_modif < self.last_updated.elapsed() {
                        return self.update();
                    }
                }
            }
        }
        false
    }

    pub fn write<T>(&self, mut writer: T) -> bool
    where T: Write {
        writer.write_all(self.content.as_bytes()).is_ok()
    }

    pub fn mark_as_updated(&mut self) {
        self.last_updated = Instant::now();
    }
}

#[derive(PartialEq)]
enum ReadState {
    Orig,
    Dynamic
}