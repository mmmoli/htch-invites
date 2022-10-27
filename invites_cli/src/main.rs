mod args;

use args::{Cli, EntityType};
use clap::Parser;
use invites_shared::Entity;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    match &cli.entity {
        EntityType::Subdomain(args) => match &args.operation {
            args::SubdomainOperation::Create(args) => Entity::new(&args.id),
            args::SubdomainOperation::Delete(_) => todo!(),
        },
        EntityType::Invitation(args) => match &args.operation {
            args::InvitationOperation::Create(_) => todo!(),
            args::InvitationOperation::Revoke(_) => todo!(),
            args::InvitationOperation::Send(_) => todo!(),
        },
    };

    Ok(())
}
