use std::env::Args;
use std::iter::Peekable;

use crate::commands::{Command, CommandResult};
use crate::commands::{
    add::Add, delete::Delete, export::Export, help::Help, here::Here, import::Import, init::Init,
    parent_entity::ParentEntity, show::Show, upsert::Upsert, version::Version,
};

fn dispatch<C>(mut args: Peekable<Args>) -> Result<CommandResult, String>
where
    C: Command + Default,
{
    if C::consumes_arg() {
        args.next();
    }
    C::default().run(args)
}

pub fn run(mut args: Peekable<Args>) -> Result<CommandResult, String> {
    match args.peek().map(String::as_str) {
        None => dispatch::<Help>(args),
        // General:
        Some("--help") => dispatch::<Help>(args),
        Some("-v") | Some("--version") => dispatch::<Version>(args),
        // Utilities:
        Some("-i") | Some("--init") => dispatch::<Init>(args),
        Some("--import") => dispatch::<Import>(args),
        Some("--export") => dispatch::<Export>(args),
        // Adding, Updating, and Deleting Links:
        Some("-a") | Some("--add") => dispatch::<Add>(args),
        Some("-d") | Some("--delete") => dispatch::<Delete>(args),
        Some("-s") | Some("--show") => dispatch::<Show>(args),
        Some("-u") | Some("--upsert") => dispatch::<Upsert>(args),
        // Opening links:
        Some("here") => dispatch::<Here>(args),
        Some(_parent_entity) => dispatch::<ParentEntity>(args),
    }
}
