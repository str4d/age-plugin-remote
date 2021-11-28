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
