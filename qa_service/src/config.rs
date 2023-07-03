use clap::Parser;
use config::Config as cfg;
use dotenv::dotenv;
use std::env;

/// Q&A web service API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Which errors we want to log (info, warn or error)
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,
    /// Which PORT the server is listening to
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
    /// Database user
    #[clap(long, default_value = "user")]
    pub db_user: String,
    /// Database user
    #[clap(long, default_value = "")]
    pub db_password: String,
    /// URL for the postgres database
    #[clap(long, default_value = "localhost")]
    pub db_host: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "5432")]
    pub db_port: u16,
    /// Database name
    #[clap(long, default_value = "rustwebdev")]
    pub db_name: String,
}

#[derive(Parser, Debug, Default, serde::Deserialize, PartialEq)]
struct Args {
    log_level: String,
    db_host: String,
    db_port: u16,
    db_name: String,
    port: u16,
}

impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        // Config file: config.toml
        // config.port
        let args_config = cfg::builder()
            .add_source(config::File::with_name("config"))
            .build()
            .unwrap();
        let args = args_config.try_deserialize::<Args>().unwrap();

        // CLI argument
        // let config = Config::parse();
        let log_level = Config::parse().log_level;

        // .env file
        // use env::var
        dotenv().ok();
        if let Err(_) = env::var("BAD_WORDS_API_KEY") {
            panic!("BadWords API key not set");
        }
        if let Err(_) = env::var("PASETO_KEY") {
            panic!("PASETO_KEY not set");
        }
        // let port = env::var("PORT")
        //     .ok()
        //     .map(|val| val.parse::<u16>())
        //     .unwrap_or(Ok(config.port))
        //     .map_err(|e| handle_errors::Error::ParseError(e))?;
        let db_user = env::var("POSTGRES_USER").unwrap();
        let db_password = env::var("POSTGRES_PASSWORD").unwrap();

        // let db_host = env::var("POSTGRES_HOST").unwrap_or(config.db_host.to_owned());
        // let db_port = env::var("POSTGRES_PORT").unwrap_or(config.db_port.to_string());
        // let db_name = env::var("POSTGRES_DB").unwrap_or(config.db_name.to_owned());

        Ok(Config {
            log_level,
            port: args.port,
            db_user,
            db_password,
            db_host: args.db_host,
            db_port: args.db_port,
            // .parse::<u16>()
            // .map_err(|e| handle_errors::Error::ParseError(e))?,
            db_name: args.db_name,
        })
    }
}
