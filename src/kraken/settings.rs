/// Centralizes the settings from the env variables
/// uses the dotenvy_macro to include values in .env at compile time
/// (still debating doing that at runtime, but compile time should be usable
/// with the right CI/CD process)
use dotenvy_macro::dotenv;

pub const KRAKEN_API_KEY: &str = dotenv!("KRAKEN_API_KEY");
pub const KRAKEN_API_SECRET: &str = dotenv!("KRAKEN_API_SECRET");
pub const KRAKEN_API_URL: &str = dotenv!("KRAKEN_API_URL");
