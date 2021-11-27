//! The `age-plugin-remote` proxy.
//!
//! The proxy runs on the remote machine. It is invoked by the [agent] and persists until
//! the SSH connection from the agent is closed. The proxy maintains a Unix socket that
//! [plugin] instances can connect to; the proxy forwards each connection to the agent,
//! and passes data between them.
//!
//! The IPC protocol between the proxy and plugin instances is an internal implementation
//! detail, not covered by any stability guarantees. The `age-plugin-remote` binary that
//! the agent will invoke should be the exact same binary that is available to age clients
//! on their PATH.
//!
//! [agent]: crate::agent
//! [plugin]: crate::plugin
