# `age-plugin-remote`

TBD

```mermaid
sequenceDiagram
    actor Authority
    participant Agent
    participant Proxy
    participant Plugin
    participant Client
    actor User
    Note over Proxy,User: Remote Machine
    Note over Authority,Agent: Local Machine

    %% Opening delegation proxy
    Authority->>Agent: -i real.identity
    activate Agent
    Authority->>Agent: -s login@remote.address
    Note over Agent,Proxy: SSH connection
    Agent->>Proxy: Start proxy
    activate Proxy
    Proxy-->>Agent: proxy tag
    Agent-->>Authority: proxy.identity

    %% User wants to decrypt a file
    User->>Client: -i proxy.identity
    activate Client
    User->>Client: encrypted file
    Note over Client,Plugin: plugin protocol
    Client->>Plugin: proxy.identity
    activate Plugin
    Client->>Plugin: file header
    Note over Plugin,Proxy: unix socket
    Plugin->>Proxy: proxy tag
    Note over Agent,Plugin: encrypted session
    Plugin->>Agent: proxy.identity, file header
    Agent->>Plugin: file key
    Plugin-->>Client: file key
    deactivate Plugin
    Client-->>User: decrypted file
    deactivate Client

    %% Closing delegation proxy
    Authority->>Agent: Ctrl+C
    Agent-->Proxy: Connection closed
    deactivate Agent
    deactivate Proxy

    %% When the proxy is off, the client gets an error
    User->>Client: -i proxy.identity
    activate Client
    User->>Client: encrypted file
    Client->>Plugin: proxy.identity
    activate Plugin
    Client->>Plugin: file header
    Plugin-XProxy: proxy tag
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
