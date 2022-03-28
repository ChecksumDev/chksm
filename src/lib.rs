#[cfg(feature = "logging")]
pub mod logging {
    pub mod structs {
        #[path = "level.rs"]
        pub mod level;
    }
    #[path = "logger.rs"]
    pub mod logger;
}

#[cfg(feature = "net")]
pub mod net {
    pub mod tcp {
        #[path = "listener.rs"]
        pub mod listener;
    }

    pub mod http {
        #[path = "server.rs"]
        pub mod server;
    }
}

#[cfg(feature = "crypto")]
pub mod crypto {
    pub mod aes {
        #[path = "cipher.rs"]
        pub mod cipher;
    }
}
