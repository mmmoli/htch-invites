pub use self::invite_states::{DraftedInvitation, SentInvitation};
use chrono::prelude::*;

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
            created_at: Utc::now(),
            entity,
            recipient,
        }
    }
}

pub mod invite_states {
    use anyhow::Context;
    use chrono::Utc;

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
        pub(crate) created_at: chrono::DateTime<chrono::Utc>,
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
                created_at: Utc::now(),
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

        fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
            self.created_at
        }
    }

    impl Revokable for DraftedInvitation {
        fn revoke(self) -> RevokedInvitation {
            RevokedInvitation {
                id: self.id,
                recipient: self.recipient,
                entity: self.entity,
                created_at: Utc::now(),
            }
        }
    }

    #[derive(Debug)]
    pub struct SentInvitation {
        pub(crate) id: String,
        pub(crate) recipient: Recipient,
        pub(crate) entity: Entity,
        created_at: chrono::DateTime<chrono::Utc>,
    }

    impl SentInvitation {
        pub fn accept(self) -> AcceptedInvitation {
            AcceptedInvitation {
                id: self.id,
                recipient: self.recipient,
                entity: self.entity,
                created_at: Utc::now(),
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

        fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
            self.created_at
        }
    }

    impl Revokable for SentInvitation {
        fn revoke(self) -> RevokedInvitation {
            RevokedInvitation {
                id: self.id,
                recipient: self.recipient,
                entity: self.entity,
                created_at: Utc::now(),
            }
        }
    }

    #[derive(Debug)]
    pub struct AcceptedInvitation {
        id: String,
        recipient: Recipient,
        entity: Entity,
        created_at: chrono::DateTime<chrono::Utc>,
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

        fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
            self.created_at
        }
    }

    impl Revokable for AcceptedInvitation {
        fn revoke(self) -> RevokedInvitation {
            RevokedInvitation {
                id: self.id,
                recipient: self.recipient,
                entity: self.entity,
                created_at: Utc::now(),
            }
        }
    }

    #[derive(Debug)]
    struct ExpiredInvitation {
        id: InvitationId,
        recipient: Recipient,
        entity: Entity,
        created_at: chrono::DateTime<chrono::Utc>,
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

        fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
            self.created_at
        }
    }

    #[derive(Debug)]
    pub struct RevokedInvitation {
        id: InvitationId,
        recipient: Recipient,
        entity: Entity,
        created_at: chrono::DateTime<chrono::Utc>,
    }

    impl RevokedInvitation {
        pub fn reinstate(self) -> DraftedInvitation {
            DraftedInvitation {
                id: self.id,
                recipient: self.recipient,
                entity: self.entity,
                created_at: Utc::now(),
            }
        }
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

        fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
            self.created_at
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        notification_services::traits::NotificationSerice, Entity, Invitation, Recipient, Revokable,
    };

    struct MockNotificationService;

    impl NotificationSerice for MockNotificationService {
        fn name(&self) -> String {
            "MockNotificationService".to_string()
        }

        fn send(&self) -> anyhow::Result<()> {
            Ok(())
        }
    }

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
        let invitation = Invitation::create(recipient, entity).send(&vec![]).unwrap();
        invitation.revoke();
    }

    #[test]
    fn accept_invitation() {
        let recipient = Recipient(String::from("alice"));
        let entity = Entity(String::from("swimming_pool"));
        let invitation = Invitation::create(recipient, entity)
            .send(&vec![])
            .unwrap()
            .accept();
        invitation.revoke();
    }

    #[test]
    fn reinstate_invitation() {
        let recipient = Recipient(String::from("alice"));
        let entity = Entity(String::from("swimming_pool"));
        let invitation = Invitation::create(recipient, entity).revoke();
        let _invitation = invitation.reinstate();
    }
}
