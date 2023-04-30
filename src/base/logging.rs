pub mod Logging {
    use std::io;
    use tracing::{debug, info, instrument, span, Instrument as _, Level};
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    pub fn setup() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
        tracing::debug!("I think it started");
    }
}
