use regex::Regex;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

pub enum StatusCode {
    Ok,
    Error,
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            StatusCode::Ok => write!(f, "200 OK"),
            StatusCode::Error => write!(f, "500 Internal Server Error"),
        }
    }
}

impl StatusCode {
    /// Drops stream; wrties status code to stream; returns status code
    pub fn construct(&self, mut stream: TcpStream) -> Self {
        match *self {
            StatusCode::Ok => {
                write!(stream, "{}", StatusCode::Ok).unwrap();
                StatusCode::Ok
            }
            StatusCode::Error => {
                write!(stream, "{}", StatusCode::Error).unwrap();
                StatusCode::Error
            }
        }
    }
}

pub struct Route {
    match_pattern: Regex,
    route: Arc<dyn Fn(TcpStream, [u8; 128]) -> StatusCode>,
}

pub struct Router {
    built: bool,
    pub routes: Vec<Route>,
}

/// poll_rate in milliseconds
pub struct Server {
    pub bind_address: String,
    pub port: u16,
    pub router: Router,
    pub poll_rate: u64,
}

impl Server {
    pub fn new<S: AsRef<str>>(
        bind_address: S,
        port: u16,
        router: Router,
        poll_rate: u64,
    ) -> Server {
        Server {
            bind_address: bind_address.as_ref().to_string(),
            port,
            router,
            poll_rate,
        }
    }

    pub fn construct(&mut self) {
        self.router.built = true;
    }

    ///! # unoptimized
    /// binds to port; listens for connections; accepts connection; filters connection via regex; passes stream to appropriate route; loops
    pub fn run(self) -> ! {
        if !self.router.built {
            panic!("Attempted to run server without first calling construct()");
        }

        let listner = TcpListener::bind(format!("{}:{}", self.bind_address, self.port)).unwrap();
        loop {
            for stream in listner.incoming() {
                match stream {
                    Ok(stream) => {
                        let mut stream = stream;
                        let mut _buffer = [0; 128];
                        stream.read(&mut _buffer).unwrap();
                        let buffer = String::from_utf8_lossy(&_buffer);

                        for route in self.router.routes.iter() {
                            if route.match_pattern.is_match(&buffer) {

                                stream.flush().unwrap();
                                route.route.clone()(stream, _buffer).to_string();
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }
    }
}

impl Router {
    pub fn new() -> Router {
        Router {
            built: false,
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, route: Route) {
        if self.built {
            panic!("Attempted to add route to a built router");
        }
        self.routes.push(route);
    }
}

impl Route {
    pub fn new(match_pattern: Regex, route: Arc<dyn Fn(TcpStream, [u8; 128]) -> StatusCode>) -> Route {
        Route {
            match_pattern,
            route,
        }
    }
}

#[macro_export]
macro_rules! router{
    { $($match_pattern:expr => $route:expr),+ } => {
        {
            let mut router = Router::new();
            $(
                router.add_route(Route::new(Regex::new($match_pattern).unwrap(), Arc::new($route)));
            )+
            router
        }
    };
}

// #[test]
// fn test_router() {
//     fn d_route(mut stream: TcpStream, _data: [u8; 128]) -> StatusCode {
//         println!("called d_route");
//         write!(stream, "d_route").unwrap();

//         stream.flush().unwrap();

//         //let mut buf = String::new();

//         stream.read(&mut [0; 128]).unwrap();

//         //println!("{}", buf);

//         StatusCode::Ok.construct(stream)
//     }
//     let router = router! {
//         ".*" => d_route
//     };

//     let mut server = Server::new("127.0.0.1", 8080, router, 10);
//     server.construct();
//     server.run();
// }
