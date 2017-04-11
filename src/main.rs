#[macro_use]
extern crate rustful;
use std::error::Error;
use std::sync::Mutex;
use rustful::{Server, Context, Response, TreeRouter, Handler};

fn print_packet(context: Context, response: Response) {
    // Print headers
    println!("Headers: ");
    for value in context.headers.iter() {
        print!(">> {}", value);
    }

    println!("Variables: ");
    for (key, value) in context.variables.iter() {
        println!("{} -> {}", key.as_utf8().unwrap(), value.as_utf8().unwrap());
    }

    println!("Query: ");
    for (key, value) in context.query.iter() {
        println!("{} -> {}", key.as_utf8().unwrap(), value.as_utf8().unwrap());
    }

    println!("Fragment: ");
    for value in context.fragment.iter() {
        println!(">> {}", value.as_utf8().unwrap());
    }

    response.send("ok");
}

struct RequestHandler {
    mutex: Mutex<u32>,
    callback: fn(Context, Response),
}

impl RequestHandler {
    fn new(cbk: fn(Context, Response)) -> RequestHandler {
        RequestHandler { 
            mutex: Mutex::new(0),
            callback: cbk 
        }
    }
}

impl Handler for RequestHandler {
    fn handle_request(&self, context: Context, response: Response) {
        let _g = self.mutex.lock().unwrap();
        (self.callback)(context, response);
    }
}

fn main() {
    let server_result = Server {
            host: 8080.into(),
            handlers: insert_routes!{
            TreeRouter::new() => {
                "/" => Get: RequestHandler
            ::new(print_packet),
                // Just an example.
                "/:endpoint/:id" => Get: RequestHandler
            ::new(print_packet)
            } 
        },
            ..Server::default()
        }
        .run();

    match server_result {
        Ok(_server) => {}
        Err(e) => println!("could not start server: {}", e.description()),
    }
}