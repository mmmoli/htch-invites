use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct InvitesArgs {
    /// First argument
    #[clap(subcommand)]
    pub command: Instruction,
}

#[derive(Debug, Subcommand)]
pub enum Instruction {
    /// Create a new Invitation
    Create(CreateArgs),
    /// List all Invitations
    List(ListArgs),
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// The email address of the Recipient.
    pub email: String,

    /// The Subdomain the Recipient will be invited to.
    pub subdomain: String,
}

#[derive(Debug, Args)]
pub struct ListArgs;
