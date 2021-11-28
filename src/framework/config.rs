use super::Client;
use super::EventHandler;

pub struct Config {
    pub(crate) name: String,
    pub(crate) host: String,
    pub(crate) password: String,
    pub(crate) flags: u16,
    pub(crate) prefix: u8,
    pub(crate) interval_ms: u16,
    //pub(crate) reconnect: bool,
    //pub(crate) max_reconnect_attempts: u16,
    pub(crate) event_handlers: Vec<Box<dyn EventHandler>>,
}

impl Default for Config {
    fn default() -> Config {
        Config::new()
    }
}

impl Config {
    // Builder functions
    pub fn new() -> Self {
        Self {
            name: "insim.rs".into(),
            host: "127.0.0.1:29999".into(),
            password: "".into(),
            flags: (1 << 5), // TODO make a builder
            prefix: 0,
            interval_ms: 1000,
            // TODO: Readd support for reconnection attempts
            //reconnect: true,
            //max_reconnect_attempts: 1,
            event_handlers: Vec::new(),
        }
    }

    pub fn tcp(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    pub fn relay(mut self) -> Self {
        self.host = "isrelay.lfs.net:47474".into();
        self
    }

    pub fn udp(self, _host: String) -> Self {
        unimplemented!()
    }

    pub fn named(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn set_flags(mut self, flags: u16) -> Self {
        self.flags = flags;
        self
    }

    pub fn clear_flags(mut self) -> Self {
        self.flags = 0;
        self
    }

    pub fn password(mut self, pwd: String) -> Self {
        self.password = pwd;
        self
    }

    pub fn prefix(mut self, prefix: u8) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn interval(mut self, interval: u16) -> Self {
        // TODO take a Duration and automatically convert it
        self.interval_ms = interval;
        self
    }

    pub fn using_event_handler<H: EventHandler + 'static>(mut self, event_handler: H) -> Self {
        self.event_handlers.push(Box::new(event_handler));
        self
    }

    pub fn build(self) -> Client {
        Client::from_config(self)
    }
}