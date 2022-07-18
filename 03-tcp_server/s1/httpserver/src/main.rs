mod server;
mod router;
mod handler;

use server::Server;

fn main() {
    let server = Server::new("localhost:3000");
    server.run();
    // main 会调用 router 中的功能，router 会调用 handler 中的功能，handler 会调用server中的功能
    println!("Hello World!");
}
