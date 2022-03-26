use std::net::TcpListener;
use zero2prod::run;

struct Settings {
    host: String,
    port: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings {
        host: String::from("127.0.0.1"),
        port: 8080,
    };

    let listener = TcpListener::bind(format!("{}:{}", settings.host, settings.port))
        .expect("Failed to bind random port");

    run(listener)?.await
}
