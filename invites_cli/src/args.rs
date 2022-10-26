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
    Subdomain(SubdomainArgs),
    Invitation(InvitationArgs),
}

#[derive(Debug, Args)]
pub struct InvitationArgs {
    #[clap(subcommand)]
    pub operation: InvitationOperation,
}

#[derive(Debug, Args)]
pub struct SubdomainArgs {
    #[clap(subcommand)]
    pub operation: SubdomainOperation,
}

#[derive(Debug, Subcommand)]
pub enum SubdomainOperation {
    /// Create a new Invitation List.
    Create(CreateSubdomainArgs),
    /// Delete Invitation List
    Delete(DeleteSubdomainArgs),
}

#[derive(Debug, Args)]
pub struct CreateSubdomainArgs {
    /// The id for the new Subdomain.
    pub id: String,
}

#[derive(Debug, Args)]
pub struct DeleteSubdomainArgs {
    /// The id for the Subdomain to delete.
    pub id: String,
}

#[derive(Debug, Args)]
pub struct ListArgs;

#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// the ID of the Invitation List to delete
    pub id: String,
}

#[derive(Debug, Subcommand)]
pub enum InvitationOperation {
    Create(CreateInvitationArgs),
    Send(SendInvitationArgs),
    Revoke(RevokeInvitationArgs),
}

#[derive(Debug, Args)]
pub struct CreateInvitationArgs {
    /// The email address of the Recipient.
    pub recipient: String,

    /// The Subdomain the Recipient will be invited to.
    pub subdomain: String,
}

#[derive(Debug, Args)]
pub struct SendInvitationArgs {
    /// the ID of the Invitation to send.
    pub id: String,
}

#[derive(Debug, Args)]
pub struct RevokeInvitationArgs {
    /// the ID of the Invitation to revoke.
    pub id: String,
}
