/// An [Invitation] is authorisation for a [Recipient] to access a [Subdomain].
///
/// It's single-use, and can be revoked at any time.

#[derive(Debug)]
pub struct Invitation {
    recipient: Recipient,
    subdomain: Subdomain,
}

// impl Invitation {
//     /// Create a new [Invitation] for a [Recipient] to access a [Subdomain].
//     pub fn new(recipient: Recipient, subdomain: Subdomain) -> PendingInvitation {
//         PendingInvitation {
//             recipient,
//             subdomain,
//         }
//     }
// }

#[derive(Debug)]
pub struct RevokedInvitation {
    recipient: Recipient,
    subdomain: Subdomain,
}

// pub struct PendingInvitation {
//     recipient: Recipient,
//     subdomain: Subdomain,
// }

// impl PendingInvitation {
//     pub fn send(self) -> SentInvitation {
//         SentInvitation {
//             recipient: self.recipient,
//             subdomain: self.subdomain,
//         }
//     }
// }

// impl Invite for PendingInvitation {
//     /// Returns the [Recipient] of this [Invitation].
//     fn recipient(&self) -> &Recipient {
//         &self.recipient
//     }

//     /// Returns the [Subdomain] of this [Invitation].
//     fn subdomain(&self) -> &Subdomain {
//         &self.subdomain
//     }
// }

// impl Revokable for PendingInvitation {
//     /// Revoke this [PendingInvitation].
//     fn revoke(self) -> RevokedInvitation {
//         RevokedInvitation {
//             recipient: self.recipient,
//             subdomain: self.subdomain,
//         }
//     }
// }

// pub struct AcceptedInvitation {
//     recipient: Recipient,
//     subdomain: Subdomain,
// }

// impl Invite for AcceptedInvitation {
//     /// Returns the [Recipient] of this [Invitation].
//     fn recipient(&self) -> &Recipient {
//         &self.recipient
//     }

//     /// Returns the [Subdomain] of this [Invitation].
//     fn subdomain(&self) -> &Subdomain {
//         &self.subdomain
//     }
// }

// pub struct SentInvitation {
//     recipient: Recipient,
//     subdomain: Subdomain,
// }

// impl SentInvitation {
//     pub fn accept(self) -> Invitation {
//         Invitation {
//             recipient: self.recipient,
//             subdomain: self.subdomain,
//         }
//     }
// }

// impl Invite for SentInvitation {
//     /// Returns the [Recipient] of this [Invitation].
//     fn recipient(&self) -> &Recipient {
//         &self.recipient
//     }

//     /// Returns the [Subdomain] of this [Invitation].
//     fn subdomain(&self) -> &Subdomain {
//         &self.subdomain
//     }
// }
