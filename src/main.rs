#[macro_use]
extern crate rustful;
use std::error::Error;

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

// Simple structure to handle requests
// This is useful when adding endoint handlers,
// so we can set different functions to each endpoint.
struct HandlerRequest(fn(Context, Response));
impl Handler for HandlerRequest {
    fn handle_request(&self, context: Context, response: Response) {
        self.0(context, response);
    }
}

fn main() {
    let server_result = Server {
        host: 8080.into(),
        handlers: insert_routes!{
            TreeRouter::new() => {
                "/" => Get: HandlerRequest(print_packet),
                // Just an example.
                "/:endpoint/:id" => Get: HandlerRequest(print_packet)
            } 
        },
        .. Server::default()
    }.run();

    match server_result {
        Ok(_server) => {},
        Err(e) => println!("could not start server: {}", e.description())
    }
}