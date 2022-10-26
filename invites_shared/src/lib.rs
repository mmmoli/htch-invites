mod invites {
    mod traits {
        use super::models::invite_states::RevokedInvitation;
        use crate::{Recipient, Subdomain};

        trait Invite {
            fn recipient(&self) -> &Recipient;
            fn subdomain(&self) -> &Subdomain;
        }

        trait Revokable {
            fn revoke(self) -> RevokedInvitation;
        }
    }

    pub mod models {
        #[derive(Debug)]
        pub struct Recipient(String);

        #[derive(Debug)]
        pub struct Subdomain(String);

        #[derive(Debug)]
        pub struct Invitation {
            recipient: Recipient,
            subdomain: Subdomain,
        }

        pub mod invite_states {
            use super::{Recipient, Subdomain};

            #[derive(Debug)]
            pub struct RevokedInvitation {
                recipient: Recipient,
                subdomain: Subdomain,
            }
        }
    }
}

pub use invites::models::{Invitation, Recipient, Subdomain};
