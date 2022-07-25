use crappy_user::{configuration, telemetry};
use crappy_user::actix::CrappyUserApp;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    // just to remove later:
    log::info!("Using configuration: {:?}", configuration);

    let crappy = CrappyUserApp::new(configuration);

    let application_ready = crappy.build_actix_server().await?;
    application_ready.run_actix_server_until_stopped().await?;
    Ok(())
}