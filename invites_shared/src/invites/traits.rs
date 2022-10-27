use super::models::invite_states::RevokedInvitation;
use crate::{Entity, Recipient};
use chrono::prelude::*;

pub trait Invite {
    fn id(&self) -> String;
    fn recipient(&self) -> &Recipient;
    fn entity(&self) -> &Entity;
    fn created_at(&self) -> DateTime<Utc>;
}

pub trait Revokable {
    fn revoke(self) -> RevokedInvitation;
}
