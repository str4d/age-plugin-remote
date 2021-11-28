# `age-plugin-remote`

TBD

## Design Notes

Proxy identities act as bearer tokens. A proxy identity is only "active" if the authority
provides it when starting the agent. The proxy identity may access all real identities
that the authority provides when starting the agent.

`TAG` exists because `PORT` is too small to be collision-resistant across multiple
restarts. A 32-bit `TAG` can be persisted in the proxy identity file, and then reused with
a small-enough chance of collision (since we only care about collisions on a per-user
level, and similar age tags in other identities are also 32 bits). The proxy run by the
agent provides the live mapping from `TAG` to `PORT`.

To reuse proxy identities, you might need a one-time remote server setup:

```
sudo sh -c 'echo "StreamLocalBindUnlink yes" >> /etc/ssh/sshd_config'
```

Or if we end up querying the environment before we start, we could also take the time to
clear out the old socket file, assuming we can verify that it isn't actually being used?

```mermaid
sequenceDiagram
    actor Authority
    participant Proxy
    participant RSock as Remote Machine
    participant Plugin
    participant Client
    actor User
    Note over RSock,User: Remote Machine
    Note over Authority,Proxy: Local Machine

    %% Opening delegation proxy
    Authority->>Proxy: -i real.identity
    activate Proxy
    Authority->>Proxy: -s login@remote.address
    Note over Proxy,RSock: SSH connection
    Proxy->>RSock: Query environment
    activate RSock
    RSock-->>Proxy: valid TAG
    deactivate RSock
    Proxy->>RSock: -R TAG.sock:local
    activate RSock
    Note over RSock: Unix socket TAG.sock
    Note over Proxy: Prepares identity containing PAKE passphrase and TAG
    Proxy-->>Authority: proxy.identity

    %% User wants to decrypt a file
    User->>Client: -i proxy.identity
    activate Client
    User->>Client: encrypted file
    Note over Client,Plugin: plugin protocol
    Client->>Plugin: proxy.identity
    activate Plugin
    Client->>Plugin: file header
    Plugin->>RSock: TAG.sock
    Note over Proxy,Plugin: Encrypted session using PAKE
    Plugin->>Proxy: proxy.identity, file header
    Note over Proxy: Queries identities, local plugins
    Proxy-->>Plugin: file key
    Plugin-->>Client: file key
    deactivate Plugin
    Client-->>User: decrypted file
    deactivate Client

    %% Closing delegation proxy
    Authority->>Proxy: Ctrl+C
    Proxy-->RSock: Connection closed
    deactivate Proxy
    deactivate RSock

    %% When the proxy is off, the client gets an error
    User->>Client: -i proxy.identity
    activate Client
    User->>Client: encrypted file
    Client->>Plugin: proxy.identity
    activate Plugin
    Client->>Plugin: file header
    Plugin-XRSock: TAG.sock
    Plugin-->>Client: error
    deactivate Plugin
    Client-->>User: error
    deactivate Client
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
