use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct InvitesArgs {
    /// First argument
    #[clap(subcommand)]
    pub entity: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    InvitationList(InvitationListArgs),
    Invitation(InvitationArgs),
}

#[derive(Debug, Args)]
pub struct InvitationArgs {
    #[clap(subcommand)]
    pub operation: InvitationListOperation,
}


#[derive(Debug, Args)]
pub struct InvitationListArgs {
    #[clap(subcommand)]
    pub operation: InvitationListOperation,
};


#[derive(Debug, Subcommand)]
pub enum InvitationListOperation {
    /// Create a new Invitation List
    Create(CreateArgs),
    /// List all Invitations Lists
    List(ListArgs),
    /// Delete Invitation List
    Delete(DeleteArgs),
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

#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// the ID of the Invitation List to delete
    pub id: String,
}