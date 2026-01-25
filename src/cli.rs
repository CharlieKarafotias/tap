use std::{
    env::Args,
    io::{Write, stderr, stdout},
    iter::Peekable,
};

use super::commands::{
    Command, CommandResult, add::Add, delete::Delete, export::Export, help::Help, here::Here,
    import::Import, init::Init, parent_entity::ParentEntity, show::Show, upsert::Upsert,
    version::Version,
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

fn dispatch<C>(mut args: Peekable<Args>) -> Result<CommandResult, String>
where
    C: Command + Default,
{
    if C::consumes_arg() {
        args.next();
    }
    C::default().run(args)
}

fn run(mut args: Peekable<Args>) -> Result<CommandResult, String> {
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
