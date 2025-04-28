use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    directory: String,
    file: String,
    filename: String
}

impl Config {
    pub fn from_args() -> Self {
        let argv: Vec<String> = env::args().collect();
        let args: usize = argv.len();

        let directory = {
            if args > 1 {
                let d = argv[1].clone();
                if d.as_bytes()[d.len()-1] != b'/' {
                    d + "/"
                }
                else {
                    d
                }
            }
            else {
                "/opt/pve-dynamic-ipsets/".to_string()
            }
        };
        let file = {
            if args > 2 {
                argv[2].clone()
            }
            else {
                "/etc/pve/firewall/cluster.fw".to_string()
            }
        };

        let filename = {
            let parts: Vec<String> = file.split('/').map(|x|x.to_string()).collect();
            parts[parts.len() - 1].clone()
        };

        Self {
            directory,
            file,
            filename
        }
    }

    pub fn get_path(&self, path: ProgramPath) -> String {
        match path {
            ProgramPath::Original => {self.file.clone()},
            ProgramPath::Generated => {
                self.directory.clone() + "generated/" + self.filename.as_str()
            },
            ProgramPath::Directory => {self.directory.clone()}
            ProgramPath::Static => {
                self.directory.clone() + "static/" + self.filename.as_str()
            }
        }
    }
}

pub enum ProgramPath {
    Original,
    Generated,
    Directory,
    Static
}