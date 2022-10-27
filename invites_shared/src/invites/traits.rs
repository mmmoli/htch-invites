use super::models::invite_states::RevokedInvitation;
use crate::{Entity, Recipient};

pub trait Invite {
    fn id(&self) -> String;
    fn recipient(&self) -> &Recipient;
    fn entity(&self) -> &Entity;
}

pub trait Revokable {
    fn revoke(self) -> RevokedInvitation;
}
