pub mod users;

use surrealdb::{Result, Surreal};
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::{Ws, Client};

pub static DATABASE: Surreal<Client> = Surreal::init();

pub async fn connect() -> Result<()> {
    let config = crate::config::CONFIG.read().unwrap().database.clone();

    DATABASE.connect::<Ws>(format!("{}:{}", config.address, config.port)).await?;

    DATABASE.signin(Root {
        username: &config.username,
        password: &config.password,
    }).await?;

    Ok(())
}
