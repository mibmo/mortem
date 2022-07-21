use tracing::info;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let _mortem = mortem::hard();

    tracing::info!("Hello!");
}
