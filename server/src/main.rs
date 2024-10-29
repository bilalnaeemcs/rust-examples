use std::{
    error::Error,
    io::Write,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    thread::{self, JoinHandle},
};

use ctrlc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

struct Server {
    port: String,
    ip: String,
    thread_handles: Vec<thread::JoinHandle<()>>,
}

pub fn handle_client(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    println!("handling the client");
    let _ = std::io::stdout().flush();

    let _ = stream.write("hello".as_bytes());
    Ok(())
}

impl Server {
    pub fn new(ip: String, port: String) -> Server {
        Server {
            port,
            ip,
            thread_handles: Vec::new(),
        }
    }

    pub fn<'a> get_thread_handles(&self) -> &Vec<JoinHandle<()>> {
        &self.thread_handles
    }

    pub fn serve(&mut self) -> Result<(), Result<(), Box<dyn Error>>> {
        let mut addr = String::new();

        addr.push_str(&self.ip);
        addr.push_str(":");
        addr.push_str(&self.port);

        let listener = TcpListener::bind(addr).unwrap();

        // accept connections and process them serially
        for stream in listener.incoming() {
            let thread_join_handle = thread::spawn(move || {
                // some work here
                //println!("handling the client");
                //
                let id = thread::current().id();

                dbg!(id);
                let _ = handle_client(&mut stream.unwrap());
            });

            self.thread_handles.push(thread_join_handle);

            //println!("will be tryng to handle a stream");
        }

        Ok(())
    }
}

// some work here
fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    println!("Hello, world!");

    let server = Server::new("127.0.0.1".to_string(), "8080".to_string());
    let thread_handles = server.get_thread_handles();
    // set up  the handler to just exit everything
    //
    // TODO: Add some sort of msging that help a more graceful exit  -- dont like it right now
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("ctrlc handler called");
        //let thread_handles = server.
        for handle in thread_handles {
            let _ = handle.join();
        }
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    let _ = server.serve();
    // set up the call back for ctrl-c
    // run join for the code
}
