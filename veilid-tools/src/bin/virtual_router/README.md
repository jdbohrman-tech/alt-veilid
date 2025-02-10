# Veilid VirtualRouter

VirtualRouter is a virtual networking router (`RouterServer`) standalone application built specifically for Veilid applications.

## RouterServer Concepts

`RouterServer` is a is a deterministic network simulator with an 'infrastructure as code' language for defining whole 'Internets', in terms of a few primitive components:

* `Allocation` - sets of IPv4 and IPv6 addresses that are used for a common function. For example, '192.168.0.0/16' is an allocation for IPv4 private addresses.
* `Machine` - an instance storing a single Veilid node's state, including its connection/socket tables, IP addresses and interfaces.
* `Network` - an instance storing a single `Network`'s allocations, to which one or more machines may belong. `Network`s also specify how they are connected together, including to the 'Internet', and how translation and gateway routing is performed.
* `Template` - instructions for creating `Machine`s, along with limits on how many `Machine`s per `Network` can be created, and which `Network`s or `Blueprint`s they are connected to.
* `Blueprint` - instructions for creating `Network`s, along with limits on how many `Network`s can be created.
* `Profiles` - a set of `Machine`s and `Template`s to use when attaching a Veilid application to the RouterServer.

Applications can connect to VirtualRouter over TCP or WebSockets, see the `--help` for more details.

Applications can also host a `RouterServer` inside their own process for fully encapsulated simulation and testing, connected via a `flume` channel.

## Example

To run VirtualRouter:

```
cargo run --bin virtual_router --features=virtual-router-bin
```

