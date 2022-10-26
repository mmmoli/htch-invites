use self::invite_states::{DraftedInvitation, SentInvitation};

#[derive(Debug)]
pub struct Recipient(String);

#[derive(Debug)]
pub struct Subdomain(String);

#[derive(Debug)]
pub struct Invitation {
    recipient: Recipient,
    subdomain: Subdomain,
}

impl Invitation {
    pub fn create(recipient: Recipient, subdomain: Subdomain) -> DraftedInvitation {
        DraftedInvitation {
            recipient,
            subdomain,
        }
    }

    pub fn send(self) -> SentInvitation {
        SentInvitation {
            recipient: self.recipient,
            subdomain: self.subdomain,
        }
    }
}

pub mod invite_states {
    use super::{Recipient, Subdomain};
    use crate::invites::traits::{Invite, Revokable};

    #[derive(Debug)]
    pub struct DraftedInvitation {
        pub(crate) recipient: Recipient,
        pub(crate) subdomain: Subdomain,
    }

    impl Invite for DraftedInvitation {
        fn recipient(&self) -> &Recipient {
            &self.recipient
        }

        fn subdomain(&self) -> &Subdomain {
            &self.subdomain
        }
    }

    impl Revokable for DraftedInvitation {
        fn revoke(self) -> RevokedInvitation {
            RevokedInvitation {
                recipient: self.recipient,
                subdomain: self.subdomain,
            }
        }
    }

    #[derive(Debug)]
    pub struct SentInvitation {
        pub(crate) recipient: Recipient,
        pub(crate) subdomain: Subdomain,
    }

    impl Invite for SentInvitation {
        fn recipient(&self) -> &Recipient {
            &self.recipient
        }

        fn subdomain(&self) -> &Subdomain {
            &self.subdomain
        }
    }

    impl Revokable for SentInvitation {
        fn revoke(self) -> RevokedInvitation {
            RevokedInvitation {
                recipient: self.recipient,
                subdomain: self.subdomain,
            }
        }
    }

    #[derive(Debug)]
    struct AcceptedInvitation {
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
    }

    #[derive(Debug)]
    struct ExpiredInvitation {
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
    }

    #[derive(Debug)]
    pub struct RevokedInvitation {
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
    }
}
