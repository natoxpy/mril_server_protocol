use std::{
    net::{TcpListener, ToSocketAddrs},
    sync::{Arc, Mutex},
};

use mril_transfer_protocol::{
    bufferable::Bufferable,
    package::{packages::Packages, Package},
    stream::Stream,
};

/// Allows to connect and receive server packages
pub trait Route {
    fn route(client: Arc<Mutex<Stream>>);
}

/// Routers receive all packages and handle them independently   
pub trait Router {
    fn route(&self, package: Package, client: Arc<Mutex<Stream>>);
}

pub struct Server<T: Router> {
    tcp_listener: TcpListener,
    routers: Vec<T>,
}

impl<T: Router> Server<T> {
    pub fn bind<B: ToSocketAddrs>(addr: B) -> Self {
        let tcp_listener = TcpListener::bind(addr).unwrap();

        Self {
            tcp_listener,
            routers: vec![],
        }
    }

    pub fn add_router(&mut self, router: T) {
        self.routers.push(router);
    }

    pub fn listen(&mut self) {
        loop {
            let (tcp_stream, _) = self.tcp_listener.accept().unwrap();
            let mut stream = Stream::connect_stream(tcp_stream);

            let package = Package::from_stream(&mut stream.tcp_stream);

            let arc = Arc::new(Mutex::new(stream));

            self.routers.iter().for_each(|router| {
                router.route(package.clone(), arc.clone());
            });
        }
    }
}
