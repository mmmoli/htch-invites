pub use self::invite_states::{DraftedInvitation, SentInvitation};

#[derive(Debug)]
pub struct Recipient(pub String);

#[derive(Debug)]
pub struct Entity(pub String);

impl Entity {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }

    pub fn delete(_id: &str) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Invitation;

type InvitationId = String;

impl Invitation {
    pub fn create(recipient: Recipient, entity: Entity) -> DraftedInvitation {
        DraftedInvitation {
            id: format!("{}-{}", recipient.0, entity.0),
            recipient,
            entity,
        }
    }
}

pub mod invite_states {
    use anyhow::Context;

    use super::{Entity, InvitationId, Recipient};
    use crate::{
        invites::traits::{Invite, Revokable},
        notification_services::traits::NotificationSerice,
    };

    #[derive(Debug)]
    pub struct DraftedInvitation {
        pub(crate) id: InvitationId,
        pub(crate) recipient: Recipient,
        pub(crate) entity: Entity,
    }

    impl DraftedInvitation {
        pub fn send(
            self,
            notification_services: &Vec<Box<dyn NotificationSerice>>,
        ) -> anyhow::Result<SentInvitation> {
            for service in notification_services {
                service
                    .send()
                    .with_context(|| format!("{} failed to send notification", service.name()))?;
            }

            Ok(SentInvitation {
                id: self.id,
                recipient: self.recipient,
                entity: self.entity,
            })
        }
    }

    impl Invite for DraftedInvitation {
        fn recipient(&self) -> &Recipient {
            &self.recipient
        }

        fn entity(&self) -> &Entity {
            &self.entity
        }

        fn id(&self) -> String {
            self.id.clone()
        }
    }

    impl Revokable for DraftedInvitation {
        fn revoke(self) -> RevokedInvitation {
            RevokedInvitation {
                id: self.id,
                recipient: self.recipient,
                entity: self.entity,
            }
        }
    }

    #[derive(Debug)]
    pub struct SentInvitation {
        pub(crate) id: String,
        pub(crate) recipient: Recipient,
        pub(crate) entity: Entity,
    }

    impl SentInvitation {
        pub fn accept(self) -> AcceptedInvitation {
            AcceptedInvitation {
                id: self.id,
                recipient: self.recipient,
                entity: self.entity,
            }
        }
    }

    impl Invite for SentInvitation {
        fn recipient(&self) -> &Recipient {
            &self.recipient
        }

        fn entity(&self) -> &Entity {
            &self.entity
        }

        fn id(&self) -> String {
            self.id.clone()
        }
    }

    impl Revokable for SentInvitation {
        fn revoke(self) -> RevokedInvitation {
            RevokedInvitation {
                id: self.id,
                recipient: self.recipient,
                entity: self.entity,
            }
        }
    }

    #[derive(Debug)]
    pub struct AcceptedInvitation {
        id: String,
        recipient: Recipient,
        entity: Entity,
    }

    impl Invite for AcceptedInvitation {
        fn recipient(&self) -> &Recipient {
            &self.recipient
        }

        fn entity(&self) -> &Entity {
            &self.entity
        }

        fn id(&self) -> String {
            self.id.clone()
        }
    }

    impl Revokable for AcceptedInvitation {
        fn revoke(self) -> RevokedInvitation {
            RevokedInvitation {
                id: self.id,
                recipient: self.recipient,
                entity: self.entity,
            }
        }
    }

    #[derive(Debug)]
    struct ExpiredInvitation {
        id: InvitationId,
        recipient: Recipient,
        entity: Entity,
    }

    impl Invite for ExpiredInvitation {
        fn recipient(&self) -> &Recipient {
            &self.recipient
        }

        fn entity(&self) -> &Entity {
            &self.entity
        }

        fn id(&self) -> String {
            self.id.clone()
        }
    }

    #[derive(Debug)]
    pub struct RevokedInvitation {
        id: InvitationId,
        recipient: Recipient,
        entity: Entity,
    }

    impl Invite for RevokedInvitation {
        fn recipient(&self) -> &Recipient {
            &self.recipient
        }

        fn entity(&self) -> &Entity {
            &self.entity
        }

        fn id(&self) -> String {
            self.id.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Entity, Invitation, Recipient, Revokable};

    #[test]
    fn create_invitation() {
        let recipient = Recipient(String::from("alice"));
        let entity = Entity(String::from("swimming_pool"));
        let invitation = Invitation::create(recipient, entity);
        invitation.revoke();
    }

    #[test]
    fn revoke_sent_invitation() {
        let recipient = Recipient(String::from("alice"));
        let entity = Entity(String::from("swimming_pool"));
        let invitation = Invitation::create(recipient, entity).send();
        invitation.revoke();
    }

    #[test]
    fn accept_invitation() {
        let recipient = Recipient(String::from("alice"));
        let entity = Entity(String::from("swimming_pool"));
        let invitation = Invitation::create(recipient, entity).send().accept();
        invitation.revoke();
    }
}
