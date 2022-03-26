use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    run(listener)?.await
}
