use std::{
    boxed::Box,
    error::Error as StdError,
    result::Result as StdResult,
};

mod client;

pub type Result<T> = StdResult<T, Box<dyn StdError + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    kankyo::init()?;
    
    client::start().await?;

    Ok(())
}
