mod invites;

pub use invites::models::{Invitation, Recipient, Subdomain};
pub use invites::traits::{Invite, Revokable};

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
