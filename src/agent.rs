//! The `age-plugin-remote` agent.
//!
//! The agent runs on the local machine. It opens a connection to the remote machine,
//! starts the [proxy], and produces an identity file that can be used by age clients on
//! the remote machine. It then accepts connections via the proxy from [plugin] instances
//! started by those age clients. For each plugin instance, the agent acts itself like an
//! age client and age plugin combined: it receives decryption commands forwarded from the
//! plugin instance, and then acts on them using pre-configured local identities (which
//! may include plugin identities).
//!
//! [plugin]: crate::plugin
//! [proxy]: crate::proxy
