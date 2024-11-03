//! The `age-plugin-remote` plugin.
//!
//! The plugin implements the age plugin protocol. It is invoked by an age client on the
//! remote machine, and given identities provided by one or more [proxies] running on some
//! local machine. The identities enable the plugin to find and connect to the proxies via
//! Unix sockets they are running on the remote machine. The plugin accepts decryption
//! commands from the age client and issues corresponding decryption requests to each
//! available agent.
//!
//! [proxies]: crate::proxy

use std::{collections::HashMap, convert::Infallible};

use age_core::format::{FileKey, Stanza};
use age_plugin::{
    identity::{Error, IdentityPluginV1},
    PluginHandler,
};

use crate::{identity::Identity, proxy, PLUGIN_NAME};

pub(crate) struct Handler;

impl PluginHandler for Handler {
    type RecipientV1 = Infallible;
    type IdentityV1 = IdentityPlugin;

    fn identity_v1(self) -> std::io::Result<Self::IdentityV1> {
        Ok(IdentityPlugin::default())
    }
}

#[derive(Default)]
pub(crate) struct IdentityPlugin {
    identities: Vec<Identity>,
}

impl IdentityPluginV1 for IdentityPlugin {
    fn add_identity(
        &mut self,
        // TODO: Document index
        index: usize,
        plugin_name: &str,
        bytes: &[u8],
    ) -> Result<(), age_plugin::identity::Error> {
        if let Some(identity) = if plugin_name == PLUGIN_NAME {
            Identity::from_bytes(bytes)
        } else {
            None
        } {
            self.identities.push(identity);
            Ok(())
        } else {
            Err(Error::Identity {
                index,
                message: "Invalid proxy identity".to_owned(),
            })
        }
    }

    fn unwrap_file_keys(
        &mut self,
        files: Vec<Vec<Stanza>>,
        callbacks: impl age_plugin::Callbacks<age_plugin::identity::Error>,
    ) -> std::io::Result<
        std::collections::HashMap<usize, Result<FileKey, Vec<age_plugin::identity::Error>>>,
    > {
        let mut file_keys = HashMap::with_capacity(files.len());

        // Try each identity in order.
        // TODO: Make age-plugin clearer about identity order.
        let rt = tokio::runtime::Runtime::new()?;
        for proxy_identity in &self.identities {
            rt.block_on(proxy::run_remote(proxy_identity))?;
        }

        Ok(file_keys)
    }
}
