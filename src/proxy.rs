//! The `age-plugin-remote` proxy.
//!
//! The proxy runs on the local machine. It opens a connection to the remote machine to
//! create a Unix socket, and produces an identity file that can be used by age clients on
//! the remote machine. It then accepts connections via the Unix socket from [plugin]
//! instances started by those age clients. For each plugin instance, the proxy itself
//! acts like an age client and age plugin combined: it receives decryption commands
//! forwarded from the plugin instance, and then acts on them using pre-configured local
//! identities (which may include plugin identities).
//!
//! TODO: Should proxy and plugin communicate via age stanzas? May as well, I guess, but
//! then it would be nice to have a plugin variant that allows for direct forwarding? Or
//! is that unnecessary / doesn't work for this protocol?
//!
//! [plugin]: crate::plugin

use std::env;
use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::{rngs::OsRng, RngCore};
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::{
    io::AsyncWriteExt,
    net::{UnixListener, UnixStream},
    process::Command,
};

use crate::identity::Identity;

pub(crate) fn local_path() -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!(
        ".age-plugin-remote.local.{}-{}.sock",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        OsRng.next_u64()
    ));
    path
}

pub(crate) async fn run_local(
    identities: Vec<Box<dyn age::Identity>>,
    destinations: Vec<String>,
) -> io::Result<()> {
    let identity = Identity::new();
    println!(
        "Starting age-plugin-remote proxy with identity {}",
        identity
    );

    let local_path = local_path();
    let remote_path = identity.remote_path();

    // Open the local listener.
    let listener = UnixListener::bind(&local_path)?;

    // Start the SSH tunnels.
    let forward_arg = {
        let mut arg = remote_path.as_os_str().to_os_string();
        arg.push(":");
        arg.push(local_path);
        arg
    };
    let _ssh = destinations
        .into_iter()
        .map(|remote| {
            Command::new("ssh")
                .arg("-NR")
                .arg(&forward_arg)
                .arg(remote)
                .spawn()
        })
        .collect::<io::Result<Vec<_>>>()?;

    loop {
        // Accept a new plugin connection.
        let (conn, _) = listener.accept().await?;

        tokio::spawn(async move {
            // TODO: Implement the real protocol.
            let mut conn = BufReader::new(conn);

            let mut line = String::new();
            conn.read_line(&mut line).await?;
            println!("Server line received: {}", line);

            io::Result::Ok(())
        });
    }
}

pub(crate) async fn run_remote(proxy_identity: &Identity) -> io::Result<()> {
    let remote_path = proxy_identity.remote_path();
    let mut conn = UnixStream::connect(remote_path).await?;

    // TODO: Implement the real protocol.
    conn.write_all(b"Hello proxy!\n").await?;

    Ok(())
}
