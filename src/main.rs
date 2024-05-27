use choki::*;
use reqwest::{Request, Response};

fn main() {
    let mut server = Server::new(Some(1024));
    server
        .get(
            "/".to_string(),
            |mut req: choki::structs::Request, mut res: choki::structs::Response| {
                res.send_string("Hello");
            },
        )
        .unwrap();
    server.listen(3000, None).unwrap();
    Server::lock();
}
