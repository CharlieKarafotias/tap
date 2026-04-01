use super::commands::{
    Command, CommandResult, add::Add, delete::Delete, export::Export, help::Help, here::Here,
    import::Import, init::Init, parent_entity::ParentEntity, show::Show, upsert::Upsert,
    version::Version,
};
use crate::utils::{
    datastore::{Datastore, Truncate},
    os_implementations::RealLinkOpener,
};
use std::{
    env::Args,
    fs::File,
    io::{Read, Seek, Write, stderr, stdout},
    iter::Peekable,
};

struct Context<WOut: Write, WErr: Write> {
    writer_out: WOut,
    writer_err: WErr,
}

impl<WOut: Write, WErr: Write> Context<WOut, WErr> {
    fn new(w_out: WOut, w_err: WErr) -> Self {
        Context {
            writer_out: w_out,
            writer_err: w_err,
        }
    }
}

fn dispatch<C, RW>(mut args: Peekable<Args>, ds: Datastore<RW>) -> Result<CommandResult, String>
where
    RW: Read + Write + Seek + Truncate,
    C: Command<RW> + Default,
{
    if C::consumes_arg() {
        args.next();
    }
    C::default().run(args, ds)
}

fn run(mut args: Peekable<Args>) -> Result<CommandResult, String> {
    // Dispatch called in production so use file datastore
    let ds: Datastore<File> = Datastore::new().map_err(|e| e.to_string())?;

    match args.peek().map(String::as_str) {
        None => dispatch::<Help, File>(args, ds),
        // General:
        Some("--help") => dispatch::<Help, File>(args, ds),
        Some("-v") | Some("--version") => dispatch::<Version, File>(args, ds),
        // Utilities:
        Some("-i") | Some("--init") => dispatch::<Init, File>(args, ds),
        Some("--import") => dispatch::<Import, File>(args, ds),
        Some("--export") => dispatch::<Export, File>(args, ds),
        // Adding, Updating, and Deleting Links:
        Some("-a") | Some("--add") => dispatch::<Add, File>(args, ds),
        Some("-d") | Some("--delete") => dispatch::<Delete, File>(args, ds),
        Some("-s") | Some("--show") => dispatch::<Show, File>(args, ds),
        Some("-u") | Some("--upsert") => dispatch::<Upsert, File>(args, ds),
        // Opening links:
        Some("here") => dispatch::<Here<RealLinkOpener>, File>(args, ds),
        Some(_parent_entity) => dispatch::<ParentEntity<RealLinkOpener>, File>(args, ds),
    }
}

/// Wrapper around CLI to setup production experience
/// - Uses stdout and stderr
/// - Setup arguments for run function call by removing first arg (executable path)
pub(super) fn run_with_stdio() -> i32 {
    let mut ctx = Context::new(stdout(), stderr());
    let mut args = std::env::args().peekable();
    // NOTE: consume the executable path
    args.next();

    match run(args) {
        Ok(res) => {
            let _ = writeln!(ctx.writer_out, "{}", res);
            0
        }
        Err(e) => {
            let _ = writeln!(ctx.writer_err, "ERROR: {}", e);
            1
        }
    }
}
