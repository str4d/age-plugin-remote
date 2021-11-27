//! The `age-plugin-remote` plugin.
//!
//! The plugin implements the age plugin protocol. It is invoked by an age client on the
//! remote machine, and given identities provided by one or more [agents] running on some
//! local machine. The identities enable the plugin to find and connect to [proxies] via
//! Unix sockets; any running proxies are then used to establish secure connections to the
//! respective agents. The plugin accepts decryption commands from the age client and
//! issues corresponding decryption requests to each available agent.
//!
//! The IPC protocol between the proxy and plugin instances is an internal implementation
//! detail, not covered by any stability guarantees. The `age-plugin-remote` binary that
//! the agent will invoke should be the exact same binary that is available to age clients
//! on their PATH.
//!
//! [agents]: crate::agent
//! [proxies]: crate::proxy
