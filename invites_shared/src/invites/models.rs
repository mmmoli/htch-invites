pub use self::invite_states::{DraftedInvitation, SentInvitation};

#[derive(Debug)]
pub struct Recipient(pub String);

#[derive(Debug)]
pub struct Subdomain(pub String);

impl Subdomain {
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
    pub fn create(recipient: Recipient, subdomain: Subdomain) -> DraftedInvitation {
        DraftedInvitation {
            id: "invitation_id".to_string(),
            recipient,
            subdomain,
        }
    }
}

pub mod invite_states {
    use super::{InvitationId, Recipient, Subdomain};
    use crate::invites::traits::{Invite, Revokable};

    #[derive(Debug)]
    pub struct DraftedInvitation {
        pub(crate) id: InvitationId,
        pub(crate) recipient: Recipient,
        pub(crate) subdomain: Subdomain,
    }

    impl DraftedInvitation {
        pub fn send(self) -> SentInvitation {
            SentInvitation {
                id: self.id,
                recipient: self.recipient,
                subdomain: self.subdomain,
            }
        }
    }

    impl Invite for DraftedInvitation {
        fn recipient(&self) -> &Recipient {
            &self.recipient
        }

        fn subdomain(&self) -> &Subdomain {
            &self.subdomain
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
                subdomain: self.subdomain,
            }
        }
    }

    #[derive(Debug)]
    pub struct SentInvitation {
        pub(crate) id: String,
        pub(crate) recipient: Recipient,
        pub(crate) subdomain: Subdomain,
    }

    impl SentInvitation {
        pub fn accept(self) -> AcceptedInvitation {
            AcceptedInvitation {
                id: self.id,
                recipient: self.recipient,
                subdomain: self.subdomain,
            }
        }
    }

    impl Invite for SentInvitation {
        fn recipient(&self) -> &Recipient {
            &self.recipient
        }

        fn subdomain(&self) -> &Subdomain {
            &self.subdomain
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
                subdomain: self.subdomain,
            }
        }
    }

    #[derive(Debug)]
    pub struct AcceptedInvitation {
        id: String,
        recipient: Recipient,
        subdomain: Subdomain,
    }

    impl Invite for AcceptedInvitation {
        fn recipient(&self) -> &Recipient {
            &self.recipient
        }

        fn subdomain(&self) -> &Subdomain {
            &self.subdomain
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
                subdomain: self.subdomain,
            }
        }
    }

    #[derive(Debug)]
    struct ExpiredInvitation {
        id: InvitationId,
        recipient: Recipient,
        subdomain: Subdomain,
    }

    impl Invite for ExpiredInvitation {
        fn recipient(&self) -> &Recipient {
            &self.recipient
        }

        fn subdomain(&self) -> &Subdomain {
            &self.subdomain
        }

        fn id(&self) -> String {
            self.id.clone()
        }
    }

    #[derive(Debug)]
    pub struct RevokedInvitation {
        id: InvitationId,
        recipient: Recipient,
        subdomain: Subdomain,
    }

    impl Invite for RevokedInvitation {
        fn recipient(&self) -> &Recipient {
            &self.recipient
        }

        fn subdomain(&self) -> &Subdomain {
            &self.subdomain
        }

        fn id(&self) -> String {
            self.id.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Invitation, Recipient, Revokable, Subdomain};

    #[test]
    fn create_invitation() {
        let recipient = Recipient(String::from("alice"));
        let subdomain = Subdomain(String::from("swimming_pool"));
        let invitation = Invitation::create(recipient, subdomain);
        invitation.revoke();
    }

    #[test]
    fn revoke_sent_invitation() {
        let recipient = Recipient(String::from("alice"));
        let subdomain = Subdomain(String::from("swimming_pool"));
        let invitation = Invitation::create(recipient, subdomain).send();
        invitation.revoke();
    }

    #[test]
    fn accept_invitation() {
        let recipient = Recipient(String::from("alice"));
        let subdomain = Subdomain(String::from("swimming_pool"));
        let invitation = Invitation::create(recipient, subdomain).send().accept();
        invitation.revoke();
    }
}
