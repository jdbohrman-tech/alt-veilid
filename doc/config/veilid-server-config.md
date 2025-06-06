---
title: Veilid Server Configuration
keywords:
- config
- veilid-server
status: Draft
---
# Veilid Server Configuration

## Configuration File

`veilid-server` may be run using configuration from both command-line arguments
and the `veilid-server.conf` file.

## Global Directives

| Directive                    | Description                           |
| ---------------------------- | ------------------------------------- |
| [daemon](#daemon)            | Run `veilid-server` in the background |
| [client\_api](#client_api)   |                                       |
| [auto\_attach](#auto_attach) |                                       |
| [logging](#logging)          |                                       |
| [testing](#testing)          |                                       |
| [core](#core)                |                                       |


### daemon

```yaml
daemon:
    enabled: false
```

### client_api

```yaml
client_api:
    enabled: true
    listen_address: 'localhost:5959'
```

| Parameter                                    | Description |
| -------------------------------------------- | ----------- |
| [enabled](#client_apienabled)                |             |
| [listen\_address](#client_apilisten_address) |             |

#### client\_api:enabled

**TODO**

#### client\_api:listen\_address

**TODO**

### auto\_attach

```yaml
auto_attach: true
```

### logging

```yaml
logging:
    system:
        enabled: false
        level: 'info'
    terminal:
        enabled: true
        level: 'info'
    file:
        enabled: false
        path: ''
        append: true
        level: 'info'
    api:
        enabled: true
        level: 'info'
    otlp:
        enabled: false
        level: 'trace'
        grpc_endpoint: 'localhost:4317'
```

| Parameter                    | Description |
| ---------------------------- | ----------- |
| [system](#loggingsystem)     |             |
| [terminal](#loggingterminal) |             |
| [file](#loggingfile)         |             |
| [api](#loggingapi)           |             |
| [otlp](#loggingotlp)         |             |

#### logging:system

```yaml
system:
    enabled: false
    level: 'info'
```

#### logging:terminal

```yaml
terminal:
    enabled: true
    level: 'info'
```

#### logging:file

```yaml
file:
    enabled: false
    path: ''
    append: true
    level: 'info'
```

#### logging:api

```yaml
api:
    enabled: true
    level: 'info'
```

#### logging:otlp

```yaml
otlp:
    enabled: false
    level: 'trace'
    grpc_endpoint: 'localhost:4317'
```

### testing

```yaml
testing:
    subnode_index: 0
    subnode_count: 1
```

### core

| Parameter                                | Description |
| ---------------------------------------- | ----------- |
| [protected\_store](#coreprotected_store) |             |
| [table\_store](#coretable_store)         |             |
| [block\_store](#block_store)             |             |
| [network](#corenetwork)                  |             |

#### core:protected\_store

```yaml
protected_store:
    allow_insecure_fallback: true
    always_use_insecure_storage: true
    directory: '%DIRECTORY%'
    delete: false
```

#### core:table\_store

```yaml
table_store:
    directory: '%TABLE_STORE_DIRECTORY%'
    delete: false
```

#### core:block\_store

```yaml
block_store:
    directory: '%BLOCK_STORE_DIRECTORY%'
    delete: false
```

#### core:network

```yaml
network:
    connection_initial_timeout_ms: 2000
    connection_inactivity_timeout_ms: 60000
    max_connections_per_ip4: 32
    max_connections_per_ip6_prefix: 32
    max_connections_per_ip6_prefix_size: 56
    max_connection_frequency_per_min: 128
    client_allowlist_timeout_ms: 300000
    reverse_connection_receipt_time_ms: 5000
    hole_punch_receipt_time_ms: 5000
    network_key_password: null
    disable_capabilites: []
    node_id: null
    node_id_secret: null
    upnp: true
    detect_address_changes: true
    enable_local_peer_scope: false
    restricted_nat_retries: 0
```

| Parameter                                   | Description |
| ------------------------------------------- | ----------- |
| [routing\_table](#corenetworkrouting_table) |             |
| [rpc](#corenetworkrpc)                      |             |
| [dht](#corenetworkdht)                      |             |
| [tls](#corenetworktls)                      |             |
| [application](#corenetworkapplication)      |             |
| [protocol](#corenetworkprotocol)            |             |

#### core:network:routing\_table

```yaml
routing_table:
    node_id: null
    node_id_secret: null
    bootstrap: ['bootstrap-v1.veilid.net']
    bootstrap_keys: ['VLD0:Vj0lKDdUQXmQ5Ol1SZdlvXkBHUccBcQvGLN9vbLSI7k','VLD0:QeQJorqbXtC7v3OlynCZ_W3m76wGNeB5NTF81ypqHAo','VLD0:QNdcl-0OiFfYVj9331XVR6IqZ49NG-E18d5P7lwi4TA']
    limit_over_attached: 64
    limit_fully_attached: 32
    limit_attached_strong: 16
    limit_attached_good: 8
    limit_attached_weak: 4
```

#### core:network:rpc

```yaml
rpc:
    concurrency: 0
    queue_size: 1024
    max_timestamp_behind_ms: 10000
    max_timestamp_ahead_ms: 10000
    timeout_ms: 5000
    max_route_hop_count: 4
    default_route_hop_count: 1
```

#### core:network:dht

```yaml
dht:
    max_find_node_count: 20
    resolve_node_timeout_ms: 10000
    resolve_node_count: 1
    resolve_node_fanout: 4
    get_value_timeout_ms: 10000
    get_value_count: 3
    get_value_fanout: 4
    set_value_timeout_ms: 10000
    set_value_count: 5
    set_value_fanout: 4
    min_peer_count: 20
    min_peer_refresh_time_ms: 60000
    validate_dial_info_receipt_time_ms: 2000
    local_subkey_cache_size: 128
    local_max_subkey_cache_memory_mb: 256
    remote_subkey_cache_size: 1024
    remote_max_records: 65536
    remote_max_subkey_cache_memory_mb: %REMOTE_MAX_SUBKEY_CACHE_MEMORY_MB%
    remote_max_storage_space_mb: 0
    public_watch_limit: 32
    member_watch_limit: 8
    max_watch_expiration_ms: 600000
```

#### core:network:tls

```yaml
tls:
    certificate_path: '%CERTIFICATE_PATH%'
    private_key_path: '%PRIVATE_KEY_PATH%'
    connection_initial_timeout_ms: 2000
```

#### core:network:application

```yaml
application:
    https:
        enabled: false
        listen_address: ':5150'
        path: 'app'
        # url: 'https://localhost:5150'
    http:
        enabled: false
        listen_address: ':5150'
        path: 'app'
        # url: 'http://localhost:5150'
```

#### core:network:protocol

```yaml
protocol:
    udp:
        enabled: true
        socket_pool_size: 0
        listen_address: ':5150'
        # public_address: ''
    tcp:
        connect: true
        listen: true
        max_connections: 32
        listen_address: ':5150'
        #'public_address: ''
    ws:
        connect: true
        listen: true
        max_connections: 16
        listen_address: ':5150'
        path: 'ws'
        # url: 'ws://localhost:5150/ws'
    wss:
        connect: true
        listen: false
        max_connections: 16
        listen_address: ':5150'
        path: 'ws'
        # url: ''
```
