extern crate rustful;
use std::error::Error;
use rustful::{Server, Context, Response, DefaultRouter};
use std::io::{BufReader, BufRead};

fn print_packet(context: Context, response: Response) {
    println!("Method: {}", context.method);
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

    println!("Content: ");
    let mut lines = BufReader::new(context.body).lines().enumerate();
    while let Some((lineno, Ok(line))) = lines.next() {
        println!("{}: {}", lineno + 1, line);
    }
    response.send("ok");
}

fn main() {

    let mut router = DefaultRouter::<fn(Context, Response)>::new();
    router.build().many(|mut node| {
        node.then().on_get(print_packet);
        node.path(":endpoint/:id").then().on_get(print_packet);
        node.path(":endpoint/:id").then().on_post(print_packet);
        node.path(":endpoint/:id").then().on_put(print_packet);
    });
    let server_result = Server {
        host: 8081.into(),
        handlers: router,
        ..Server::default()
    }
    .run();

    match server_result {
        Ok(_server) => {}
        Err(e) => println!("could not start server: {}", e.description()),
    }
}
