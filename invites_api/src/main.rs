#[macro_use]
extern crate rocket;

mod domains;
mod invitations;
use domains::{create_domain, delete_domain, get_domain};
use invitations::{delete_invitation, get_invitation, list_invitations};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount(
            "/domains",
            routes![create_domain, get_domain, delete_domain],
        )
        .mount(
            "/invitations",
            routes![get_invitation, list_invitations, delete_invitation],
        )
        .launch()
        .await?;

    Ok(())
}
