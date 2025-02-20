use backend_service::configuration::get_configuration;
use backend_service::startup::run;
use backend_service::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    let subscriber = get_subscriber(
        env!("CARGO_CRATE_NAME").into(),
        configuration.log_level,
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let address = format!("{}:{}", configuration.host, configuration.port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await?;
    Ok(())
}
