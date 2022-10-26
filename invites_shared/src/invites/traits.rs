use super::models::invite_states::RevokedInvitation;
use crate::{Recipient, Subdomain};

pub trait Invite {
    fn id(&self) -> String;
    fn recipient(&self) -> &Recipient;
    fn subdomain(&self) -> &Subdomain;
}

pub trait Revokable {
    fn revoke(self) -> RevokedInvitation;
}
