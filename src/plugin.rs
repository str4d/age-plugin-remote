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
