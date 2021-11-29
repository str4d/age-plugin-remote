use std::io;

use gumdrop::Options;

mod plugin;
mod proxy;

const BINARY_NAME: &str = "age-plugin-remote";

#[derive(Debug, Options)]
struct PluginOptions {
    #[options(help = "Print this help message and exit.")]
    help: bool,

    #[options(help = "Print version info and exit.", short = "V")]
    version: bool,

    #[options(
        help = "Run the given age plugin state machine. Internal use only.",
        meta = "STATE-MACHINE",
        no_short
    )]
    age_plugin: Option<String>,

    #[options(help = "Expose the identity file at IDENTITY. May be repeated.")]
    identity: Vec<String>,

    #[options(
        help = "SSH destination to proxy identity files to. May be repeated.",
        free
    )]
    destination: Vec<String>,
}

fn main() -> io::Result<()> {
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Off)
        .parse_default_env()
        .init();

    let opts = PluginOptions::parse_args_default_or_exit();

    if let Some(state_machine) = opts.age_plugin {
        // TODO: Run plugin state machine
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(proxy::run_remote())?;
    } else if opts.version {
        println!("{} {}", BINARY_NAME, env!("CARGO_PKG_VERSION"));
    } else if opts.identity.is_empty() {
        eprintln!("At least one age identity must be specified to expose.");
    } else if opts.destination.is_empty() {
        eprintln!("At least one SSH destination must be specified to proxy identities to.");
    } else {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(proxy::run_local(opts.destination))?;
    }

    Ok(())
}
