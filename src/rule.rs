use crate::domain::Domain;

#[derive(PartialEq)]
pub struct DynRule {
    direction: Direction,
    action: Action,
    protocol: String,
    logging: LogLevel
}

impl DynRule {
    pub fn from_string(s: String) -> Option<Self> {
        let parts: Vec<String> = s.split(' ').map(|x|x.to_string()).collect();
        if parts.len() == 4 {
            if let Some(direction) = Direction::from_string(&parts[0]) {
                if let Some(action) = Action::from_string(&parts[1]) {
                    if let Some(logging) = LogLevel::from_string(&parts[3]) {
                        return Some(Self {
                            direction,
                            action,
                            protocol: parts[2].clone(),
                            logging
                        });
                    }
                }
            }
        }
        None
    }

    pub fn render(&self, domain: &Domain) -> String {
        format!("{} {}({}) -{} +dc/domain_{} -log {}\n",
            self.direction.to_string(),
            self.protocol,
            self.action.to_string(),
            self.direction.get_flag(),
            domain.get_name(),
            self.logging.to_string()
        )
    }
}

#[derive(PartialEq)]
pub enum Direction {
    In,
    Out
}

impl Direction {
    pub fn from_string(s: &String) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "in" => Some(Self::In),
            "out" => Some(Self::Out),
            _ => None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::In => "IN",
            Self::Out => "OUT",
        }.to_string()
    }

    pub fn get_flag(&self) -> String {
        match self {
            Self::In => "source",
            Self::Out => "dest",
        }.to_string()
    }
}

#[derive(PartialEq)]
pub enum Action {
    Accept,
    Drop,
    Reject
}

impl Action {
    pub fn from_string(s: &String) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "accept" => Some(Self::Accept),
            "drop" => Some(Self::Drop),
            "reject" => Some(Self::Reject),
            _ => None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Accept => "ACCEPT",
            Self::Drop => "DROP",
            Self::Reject => "REJECT"
        }.to_string()
    }
}

#[derive(PartialEq)]
pub enum LogLevel {
    NoLog,
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Info,
    Debug
}

impl LogLevel {
    pub fn from_string(s: &String) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "nolog" | "none" => Some(Self::NoLog),
            "emergency" | "emerg" => Some(Self::Emergency),
            "alert" => Some(Self::Alert),
            "critical" | "crit" => Some(Self::Critical),
            "error" | "err" => Some(Self::Error),
            "warning" | "warn" => Some(Self::Warning),
            "notice" | "ntc" => Some(Self::Notice),
            "info" | "inf" => Some(Self::Info),
            "debug" | "dbg" => Some(Self::Debug),
            _ => None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            LogLevel::NoLog => "nolog",
            LogLevel::Emergency => "emerg",
            LogLevel::Alert => "alert",
            LogLevel::Critical => "crit",
            LogLevel::Error => "err",
            LogLevel::Warning => "warning",
            LogLevel::Notice => "notice",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug"
        }.to_string()
    }
}