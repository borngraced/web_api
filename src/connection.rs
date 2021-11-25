use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::{Client, Database};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use std::env;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct DB {
    pub db: Database,
    pub connected: bool,
}

#[derive(Debug)]
pub enum MyError {
    NotConnected,
}

impl DB {
    pub async fn init() -> Result<Self, Box<dyn Error>> {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true).unwrap();
        log::info!("linking db!");

        let client_uri =
            env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

        let options: ClientOptions =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await?;

        let client: Client = Client::with_options(options)?;
        let client_db = client.database("progigs");
        log::info!("db linking successful");
        Ok(Self {
            db: client_db,
            connected: true,
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DB {
    type Error = MyError;
    async fn from_request(_request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let client_uri =
            env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

        let options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await;
        let cl = options.unwrap();

        let client = Client::with_options(cl);
        match client {
            Ok(t) => {
                let client_db = t.database("progigs");
                Outcome::Success(Self {
                    db: client_db,
                    connected: true,
                })
            }
            Err(_) => Outcome::Failure((Status::Forbidden, MyError::NotConnected)),
        }
    }
}
