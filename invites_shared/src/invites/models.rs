use crate::db::HtchDb;

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
    pub async fn create<T>(
        recipient: Recipient,
        entity: Entity,
        db: &T,
    ) -> anyhow::Result<DraftedInvitation>
    where
        T: HtchDb,
    {
        let _responses = db.execute("").await?;
        Ok(DraftedInvitation {
            id: format!("{}-{}", recipient.0, entity.0),
            created_at: Utc::now(),
            entity,
            recipient,
        })
    }
}

pub mod invite_states {
    use anyhow::Context;
    use chrono::Utc;

    use super::{Entity, InvitationId, Recipient};
    use crate::{
        db::HtchDb,
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
        #[allow(unreachable_code)]
        pub fn send<T>(
            self,
            notification_services: &Vec<Box<dyn NotificationSerice>>,
            _db: &T,
        ) -> anyhow::Result<SentInvitation>
        where
            T: HtchDb,
        {
            for service in notification_services {
                service
                    .send()
                    .with_context(|| format!("{} failed to send notification", service.name()))?;
            }

            todo!("Save to DB");

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
        #[allow(unreachable_code)]
        fn revoke<T>(self, _db: &T) -> RevokedInvitation
        where
            T: HtchDb,
        {
            todo!("Save to DB");
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
        #[allow(unreachable_code)]
        pub fn accept<T>(self, _db: &T) -> anyhow::Result<AcceptedInvitation>
        where
            T: HtchDb,
        {
            todo!("Update in DB");
            Ok(AcceptedInvitation {
                id: self.id,
                recipient: self.recipient,
                entity: self.entity,
                created_at: Utc::now(),
            })
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
        #[allow(unreachable_code)]
        fn revoke<T>(self, _db: &T) -> RevokedInvitation
        where
            T: HtchDb,
        {
            todo!("Update in DB");
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
        #[allow(unreachable_code)]
        fn revoke<T>(self, _db: &T) -> RevokedInvitation
        where
            T: HtchDb,
        {
            todo!("Update in DB");
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
        #[allow(unreachable_code)]
        pub fn reinstate<T>(self, _db: &T) -> DraftedInvitation
        where
            T: HtchDb,
        {
            todo!("Update in DB");
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

    use async_trait::async_trait;
    use surrealdb::Response;

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

    struct MockDb;

    #[async_trait]
    impl crate::db::HtchDb for MockDb {
        async fn execute(&self, _query: &str) -> anyhow::Result<Vec<Response>> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn create_invitation() {
        let mock_db = MockDb;
        let recipient = Recipient(String::from("alice"));
        let entity = Entity(String::from("swimming_pool"));
        let invitation = Invitation::create(recipient, entity, &mock_db)
            .await
            .unwrap();
        invitation.revoke(&mock_db);
    }

    #[tokio::test]
    async fn revoke_sent_invitation() {
        let mock_db = MockDb;
        let recipient = Recipient(String::from("alice"));
        let entity = Entity(String::from("swimming_pool"));
        let invitation = Invitation::create(recipient, entity, &mock_db)
            .await
            .unwrap()
            .send(&vec![], &mock_db)
            .unwrap();
        invitation.revoke(&mock_db);
    }

    #[tokio::test]
    async fn accept_invitation() {
        let mock_db = MockDb;
        let recipient = Recipient(String::from("alice"));
        let entity = Entity(String::from("swimming_pool"));
        let invitation = Invitation::create(recipient, entity, &mock_db)
            .await
            .unwrap()
            .send(&vec![], &mock_db)
            .unwrap()
            .accept(&mock_db)
            .unwrap();
        invitation.revoke(&mock_db);
    }

    #[tokio::test]
    async fn reinstate_invitation() {
        let mock_db = MockDb;
        let recipient = Recipient(String::from("alice"));
        let entity = Entity(String::from("swimming_pool"));
        let invitation = Invitation::create(recipient, entity, &mock_db)
            .await
            .unwrap()
            .revoke(&mock_db);
        let _invitation = invitation.reinstate(&mock_db);
    }
}
