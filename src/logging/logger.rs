use chrono::Utc;
use std::{
    fmt::Display,
    io::{stdout, Write},
};

use super::structs::level::Level;

pub struct Logger {
    pub level: Level,
}

impl Logger {
    pub fn new(level: Level) -> Logger {
        Logger { level }
    }

    pub async fn log<T>(&self, level: Level, message: T)
    where
        T: Display,
    {
        if level < self.level {
            return;
        }
        stdout()
            .write_all(
                format!(
                    "\x1b[38;5;247m[{}]\x1b[0m {} {}\x1b[0m\n",
                    Utc::now().format("%Y-%m-%d %H:%M:%S"),
                    level,
                    message
                )
                .as_bytes(),
            )
            .unwrap();
    }
}

mod test {
    #[tokio::test]
    async fn log() {
        use super::Level;
        use super::Logger;
        let logger = Logger::new(Level::Debug);
        logger.log(Level::Debug, "This is a debug message.").await;
        logger.log(Level::Info, "This is an info message.").await;
        logger.log(Level::Warn, "This is a warning message.").await;
        logger.log(Level::Error, "This is an error message.").await;
        logger.log(Level::Fatal, "This is a fatal message.").await;
    }

    #[tokio::test]
    async fn log_benchmark() {
        use super::Level;
        use super::Logger;
        use std::time::Instant;
        let logger = Logger::new(Level::Debug);
        let mut times: [u16; 1001] = [0; 1001];
        for i in 0..1000 {
            let start = Instant::now();
            logger.log(Level::Debug, "This is a debug message.").await;
            let end = start.elapsed();
            times[i] = end.as_millis() as u16;
        }

        let mut sum = 0;
        for i in 0..1000 {
            sum += times[i];
        }

        let average = sum / 1000;

        println!("Logger::log() took {} seconds", average);
    }
}
