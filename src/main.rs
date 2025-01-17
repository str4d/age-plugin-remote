use std::fmt;
use std::io;

use age::cli_common::StdinGuard;
use age_plugin::run_state_machine;
use clap::Parser;

mod identity;
mod plugin;
mod proxy;

const PLUGIN_NAME: &str = "remote";
const BINARY_NAME: &str = "age-plugin-remote";
const IDENTITY_PREFIX: &str = "age-plugin-remote-";

enum Error {
    IdentityRead(age::cli_common::ReadError),
    Io(io::Error),
}

impl From<age::cli_common::ReadError> for Error {
    fn from(e: age::cli_common::ReadError) -> Self {
        Error::IdentityRead(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

// Rust only supports `fn main() -> Result<(), E: Debug>`, so we implement `Debug`
// manually to provide the error output we want.
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IdentityRead(e) => write!(f, "{}", e),
            Error::Io(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, Parser)]
#[command(version)]
struct PluginOptions {
    /// Run the given age plugin state machine. Internal use only.
    #[arg(long, hide = true)]
    age_plugin: Option<String>,

    /// Expose the identity file at IDENTITY. May be repeated.
    #[arg(short, long)]
    identity: Vec<String>,

    /// SSH destination to proxy identity files to. May be repeated.
    destination: Vec<String>,
}

fn main() -> Result<(), Error> {
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Off)
        .parse_default_env()
        .init();

    let opts = PluginOptions::parse();

    if let Some(state_machine) = opts.age_plugin {
        run_state_machine(&state_machine, plugin::Handler)?;
    } else if opts.identity.is_empty() {
        eprintln!("At least one age identity must be specified to expose.");
    } else if opts.destination.is_empty() {
        eprintln!("At least one SSH destination must be specified to proxy identities to.");
    } else {
        let mut stdin_guard = StdinGuard::new(false);

        let identities = age::cli_common::read_identities(opts.identity, None, &mut stdin_guard)?;

        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(proxy::run_local(identities, opts.destination))?;
    }

    Ok(())
}
