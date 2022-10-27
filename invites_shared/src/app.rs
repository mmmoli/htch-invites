use crate::{
    invites::models::SentInvitation, notification_services::traits::NotificationSerice, Entity,
    Invitation, Recipient,
};

/// An [App] manages Invitations.
pub struct App {
    pub(crate) name: String,
    pub(crate) notification_services: Vec<Box<dyn NotificationSerice>>,
}

impl App {
    pub fn new(name: String) -> Self {
        Self {
            name,
            notification_services: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_notification_service(&mut self, notification_service: Box<dyn NotificationSerice>) {
        self.notification_services.push(notification_service);
    }

    pub fn send_invitation(
        &self,
        recipient: &str,
        entity_id: &str,
    ) -> anyhow::Result<SentInvitation> {
        let invitation = Invitation::create(
            Recipient(recipient.to_string()),
            Entity(entity_id.to_string()),
        );

        // Create in DB

        let invitation = invitation.send(&self.notification_services)?;

        // Update in DB

        Ok(invitation)
    }
}
