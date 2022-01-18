extern crate config;

use config::MetaConfig;

#[derive(MetaConfig)]
#[derive(Debug)]
pub struct Config {
    #[value(from = "PORT")]
    pub port: u16,

    #[value(from = "HOST", default = "127.0.0.1")]
    pub host: String,
}

// Ensure custom Result can be defined in the current context.
// See: https://github.com/greyblake/envconfig-rs/issues/21
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    std::env::set_var("PORT","1234");
    std::env::set_var("HOST","HO");

    let config =  Config::init_from_env();

    println!("{:#?}", config);
}
