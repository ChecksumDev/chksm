use std::{
    cmp::{self, Ordering},
    fmt::{Display, Formatter, Result},
};

pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl Level {
    fn to_str(&self) -> &'static str {
        match *self {
            Level::Debug => "\x1b[90mDEBUG:",
            Level::Info => "\x1b[32mINFO:",
            Level::Warn => "\x1b[33mWARN:",
            Level::Error => "\x1b[31mERROR:",
            Level::Fatal => "\x1b[1;31mFATAL:",
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_str())
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Level) -> bool {
        self.to_str() == other.to_str()
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Level) -> cmp::Ordering {
        match (self, other) {
            (Level::Debug, Level::Debug) => Ordering::Equal,
            (Level::Debug, _) => Ordering::Less,
            (_, Level::Debug) => Ordering::Greater,
            (Level::Info, Level::Info) => Ordering::Equal,
            (Level::Info, _) => Ordering::Less,
            (_, Level::Info) => Ordering::Greater,
            (Level::Warn, Level::Warn) => Ordering::Equal,
            (Level::Warn, _) => Ordering::Less,
            (_, Level::Warn) => Ordering::Greater,
            (Level::Error, Level::Error) => Ordering::Equal,
            (Level::Error, _) => Ordering::Less,
            (_, Level::Error) => Ordering::Greater,
            (Level::Fatal, Level::Fatal) => Ordering::Equal,
        }
    }
}

impl Eq for Level {}
