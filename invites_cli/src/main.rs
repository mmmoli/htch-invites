mod args;

use args::InvitesArgs;
use clap::{error::Result, Parser};

#[tokio::main]
async fn main() -> Result<()> {
    let args = InvitesArgs::parse();
    println!("{:?}", args);
    Ok(())
}
