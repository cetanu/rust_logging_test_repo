use tracing::{warn, debug};
use tracing_subscriber::{EnvFilter, FmtSubscriber};


fn main() {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(tracing::Level::DEBUG)
        .event_format(
            tracing_subscriber::fmt::format()
        )
        .compact()
        .init();
        
    warn!("This will show up by default");
    debug!("This will not");
}
