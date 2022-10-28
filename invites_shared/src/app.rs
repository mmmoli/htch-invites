use anyhow::Ok;

use crate::{
    db::Db, invites::models::SentInvitation, notification_services::traits::NotificationSerice,
    Entity, Invitation, Recipient,
};

#[allow(dead_code)]
pub struct CallbackConfig {
    url: String,
}

impl CallbackConfig {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}

/// An [App] manages [Invitation].
///
/// It maintains a vec of [NotificationSerice]
/// which is used to send notifications.
#[allow(dead_code)]
pub struct App {
    pub(crate) name: String,
    pub(crate) notification_services: Vec<Box<dyn NotificationSerice>>,
    pub(crate) db: Db,
    pub(crate) callback_config: CallbackConfig,
}

impl App {
    pub async fn new(name: &str, callback_config: CallbackConfig) -> anyhow::Result<Self> {
        let db = Db::new().await?;
        Ok(Self {
            name: name.to_string(),
            notification_services: Vec::new(),
            db,
            callback_config,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_notification_service(&mut self, notification_service: Box<dyn NotificationSerice>) {
        self.notification_services.push(notification_service);
    }

    pub async fn accept_invitation(&self) -> anyhow::Result<()> {
        // Find Invitation in Db
        let recipient = Recipient(String::from("alice"));
        let entity = Entity(String::from("swimming_pool"));
        let _invitation = Invitation::create(recipient, entity, &self.db)
            .await?
            .send(&vec![], &self.db)
            .unwrap()
            .accept(&self.db)?;
        Ok(())
    }

    pub async fn send_invitation(
        &self,
        recipient: &str,
        entity_id: &str,
    ) -> anyhow::Result<SentInvitation> {
        let invitation = Invitation::create(
            Recipient(recipient.to_string()),
            Entity(entity_id.to_string()),
            &self.db,
        )
        .await?;

        // Create in DB

        let invitation = invitation.send(&self.notification_services, &self.db)?;

        // Update in DB

        Ok(invitation)
    }
}

#[cfg(test)]
mod tests {
    use crate::App;

    use super::CallbackConfig;

    #[tokio::test]
    async fn accept_notifications() {
        let callback_config = CallbackConfig::new("http://localhost:8080");
        let app = App::new("HTCH", callback_config).await.unwrap();
        app.send_invitation("alice", "swimming_pool").await.unwrap();
        let _ = app.accept_invitation().await.unwrap();
    }
}
