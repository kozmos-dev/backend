pub mod users;

use surrealdb::{Result, Surreal};
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::{Ws, Client};

pub static DATABASE: Surreal<Client> = Surreal::init();

pub async fn connect() -> Result<()> {
    let config = crate::config::CONFIG.read().unwrap();

    DATABASE.connect::<Ws>(format!("{}:{}", config.database.address, config.database.port)).await?;

    DATABASE.signin(Root {
        username: &config.database.username,
        password: &config.database.password,
    }).await?;

    Ok(())
}
