use std::env;
use std::fmt;
use std::path::PathBuf;

use bech32::{ToBase32, Variant};
use rand::{rngs::OsRng, RngCore};

use crate::IDENTITY_PREFIX;

/// An identity that can be used to establish communication with an agent and use its
/// identities for decryption.
// TODO: Add PAKE passphrase.
pub(crate) struct Identity {
    tag: u32,
}

impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            bech32::encode(
                IDENTITY_PREFIX,
                self.to_bytes().to_base32(),
                Variant::Bech32,
            )
            .expect("HRP is valid")
            .to_uppercase()
            .as_str(),
        )
    }
}

impl Identity {
    pub(crate) fn new() -> Self {
        Identity {
            tag: OsRng.next_u32(),
        }
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.try_into().ok().map(|tag| Identity {
            tag: u32::from_le_bytes(tag),
        })
    }

    fn to_bytes(&self) -> [u8; 4] {
        self.tag.to_le_bytes()
    }

    /// Returns the predictable path that a proxy for the given tag will listen on.
    ///
    /// TODO: Make this actually predictable? Run a quick command on the remote to figure out
    /// the tmpdir, and then use that here?
    pub(crate) fn remote_path(&self) -> PathBuf {
        let mut path = env::temp_dir();
        path.push(format!(".age-plugin-remote.{:08x}.sock", self.tag));
        path
    }
}
