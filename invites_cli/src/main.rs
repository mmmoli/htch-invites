mod args;

use args::InvitesArgs;
use clap::Parser;

fn main() {
    let args = InvitesArgs::parse();
    println!("{:?}", args);
}
