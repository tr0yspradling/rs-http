use rs_http::http::server::HttpServer;

fn main() {
    let listener = HttpServer::new("127.0.0.1", 2000);
    listener.listen();
}
