# early α docs

# please don't share publicly

# Veilid Architecture Guide

-   [From Orbit](#from-orbit)
-   [Bird's Eye View](#birds-eye-view)
    -   [Peer Network for Data Storage](#peer-network-for-data-storage)
    -   [Block Store](#block-store)
    -   [Key-Value Store](#key-value-store)
    -   [Structuring Data](#structuring-data)
    -   [Peer and User Identity](#peer-and-user-identity)
-   [On The Ground](#on-the-ground)
    -   [Peer Network, Revisited](#peer-network-revisited)
    -   [User Privacy](#user-privacy)
    -   [Block Store, Revisited](#block-store-revisited)
    -   [Key-Value Store, Revisited](#key-value-store-revisited)

## From Orbit

The first matter to address is the question "What is Veilid?" The highest-level description is that Veilid is a peer-to-peer network for easily sharing various kinds of data.

Veilid is designed with a social dimension in mind, so that each user can have their personal content stored on the network, but also can share that content with other people of their choosing, or with the entire world if they want.

The primary purpose of the Veilid network is to provide the infrastructure for a specific kind of shared data: social media in various forms. That includes light-weight content such as Twitter's tweets or Mastodon's toots, medium-weight content like images and songs, and heavy-weight content like videos. Meta-content such as personal feeds, replies, private messages, and so forth are also intended to run atop Veilid.

* * *

## Bird's Eye View

Now that we know what Veilid is and what we intend to put on it, the second order of business is to address the parts of the question of how Veilid achieves that. Not at a very detailed level, of course, that will come later, but rather at a middle level of detail such that all of it can fit in your head at the same time.

### Peer Network for Data Storage

The bottom-most level of Veilid is a network of peers communicating to one another over the internet. Peers send each other messages (remote procedure calls) about the data being stored on the network, and also messages about the network itself. For instance, one peer might ask another for some file, or it might ask for info about what other peers exist in the network.

The data stored in the network is segmented into two kinds of data: file-like data, which typically is large, and textual data, which typically is small. Each kind of data is stored in its own subsystem specifically chosen to optimize for that kind of data.

### Block Store

File-like content is stored in a content-addressable block store. Each block is just some arbitrary blob of data (for instance, a JPEG or an MP4) of whatever size. The hash of that block acts as the unique identifier for the block, and can be used by peers to request particular blocks. Technically, textual data can be stored as a block as well, and this is expected to be done when the textual data is thought of as a document or file of some sort.

### Key-Value Store

Smaller, more ephemeral textual content generally, however, is stored in a key-value-store (KV store). Things like status updates, blog posts, user bios, etc. are all thought of as being suited for storage in this part of the data store. KV store data is not simply "on the Veilid network", but also owned/controlled by users, and identified by an arbitrary name chosen by the owner the data. Any group of users can add data, but can only change the data they've added.

For instance, we might talk about Boone's bio vs. Boone's blogpost titled "Hi, I'm Boone!", which are two things owned by the same user but with different identifiers, or on Boone's bio vs. Marquette's bio, which are two things owned by distinct users but with the same identifier.

KV store data is also stateful, so that updates to it can be made. Boone's bio, for instance, would not be fixed in time, but rather is likely to vary over time as he changes jobs, picks up new hobbies, etc. Statefulness, together with arbitrary user-chosen identifiers instead of content hashes, means that we can talk about "Boone's Bio" as an abstract thing, and subscribe to updates to it.

### Structuring Data

The combination of block storage and key-value storage together makes it possible to have higher-level concepts as well. A song, for instance, might be represented in two places in Veilid: the block store would hold the raw data, while the KV store would store a representation of the idea of the song. Maybe that would consist of a JSON object with metadata about the song, like the title, composer, date, encoding information, etc. as well as the ID of the block store data. We can then also store different _versions_ of that JSON data, as the piece is updated, upsampled, remastered, or whatever, each one pointing to a different block in the block store. It's still "the same song", at a conceptual level, so it has the same identifier in the KV store, but the raw bits associated with each version differ.

Another example of this, but with even more tenuous connection between the block store data, is the notion of a profile picture. "Marquette's Profile Picture" is a really abstracted notion, and precisely which bits it corresponds to can vary wildly over time, not just being different versions of the picture but completely different pictures entirely. Maybe one day it's a photo of Marquette and the next day it's a photo of a flower.

Social media offers many examples of these concepts. Friends lists, block lists, post indexes, favorites. These are all stateful notions, in a sense: a stable reference to a thing, but the precise content of the thing changes over time. These are exactly what we would put in the KV store, as opposed to in the block store, even if this data makes reference to content in the block store.

### Peer and User Identity

Two notions of identity are at play in the above network: peer identity and user identity. Peer identity is simple enough: each peer has a cryptographic key pair that it uses to communicate securely with other peers, both through traditional encrypted communication, and also through the various encrypted routes. Peer identity is just the identity of the particular instance of the Veilid software running on a computer.

User identity is a slightly richer notion. Users, that is to say, _people_, will want to access the Veilid network in a way that has a consistent identity across devices and apps. But since Veilid doesn't have servers in any traditional sense, we can't have a normal notion of "account". Doing so would also introduce points of centralization, which federated systems have shown to be a source of trouble. Many Mastodon users have found themselves in a tricky situation when their instance sysadmins burned out and suddenly shut down the instance without enough warning.

To avoid this re-centralization of identity, we use cryptographic identity for users as well. The user's key pair is used to sign and encrypt their content as needed for publication to the data store. A user is said to be "logged in" to a client app whenever that app has a copy of their private key. When logged in a client app act like any other of the user's client apps, able to decrypt and encrypt content, sign messages, and so forth. Keys can be added to new apps to sign in on them, allowing the user to have any number of clients they want, on any number of devices they want.

* * *

## On The Ground

The bird's eye view of things makes it possible to hold it all in mind at once, but leaves out lots of information about implementation choice. It's now time to come down to earth and get our hands dirty. In principl, this should be enough information to implement a system very much like Veilid, with the exception perhaps of the specific details of the APIs and data formats. This section won't have code, it's not documentation of the codebase, but rather is intended to form the meat of a whitepaper.

### Peer Network, Revisited

First, let's look at the peer network, since its structure forms the basis for the remainder of the data storage approach. Veilid's peer network is similar to other peer-to-peer systems in that it's overlaid on top of other protocols. Veilid tries to be somewhat protocol-agnostic, however, and currently is designed to use TCP, UDP, WebSockets, and WebRTC, as well as various methods of traversing NATs so that Veilid peers can be smartphones, personal computers on hostile ISPs, etc. To facilitate this, peers are identified not by some network identity like an IP address, but instead by peer-chosen cryptographic key-pairs. Each peer also advertises a variety of options for how to communicate with it, called dial info, and when one peer wants to talk to another, it gets the dial info for that peer from the network and then uses it to communicate.

When a peer first connects to Veilid, it does so by contacting bootstrap peers, which have simple IP address dial info that is guaranteed to be stable by the maintainers of the network. These bootstrap peers are the first entries in the peer's routing table -- an address book of sorts, which it uses to figure out how to talk to a peer. The routing table consists of a mapping from peer public keys to prioritized choices for dial info. To populate the routing table, the peer asks other peers what its neighbors are in the network. The notion of neighbor here is defined by a similarity metric on peer IDs, in particular an XOR metric like many DHTs use. Over the course of interacting with the network, the peer will keep dial info up to date when it detects changes. It may also add dial info for peers it discovers along the way, depending on the peer ID.

To talk to a specific peer, its dial info is looked up in the routing table. If there is dial info present, then the options are attempted in order of the priority specified in the routing table. Otherwise, the peer has to request the dial info from the network, so it looks through its routing table to find the peer who's ID is nearest the target peer according to the XOR metric, and sends it an RPC call with a procedure named `find_node`. Given any particular peer ID, the receiver of a `find_node` call returns dial info for the peers in its routing table that are nearest the given ID. This gets the peer closer to its destination, at least in the direction of the other peer it asked. If the desired peer's information was in the result of the call, then it's done, otherwise it calls `find_node` again to get closer. It iterates in this way, possibly trying alternate peers, as necessary, in a nearest-first fashion until it either finds the desire'd peer's dial info, has exhausted the entire network, or gives up.

### User Privacy

In order to ensure that users can participate in Veilid with some amount of privacy, we need to address the fact that being connected to Veilid entails communicating with other peers, and therefore sharing IP addresses. A user's peer will therefore be frequently issuing RPCs in a way that directly associates the user's identifying information with their peer's ID. Veilid provides privacy by allowing the use of an RPC forwarding mechanism that uses cryptography to similar to onion routing in order to hide the path that a message takes between its actual originating peer and its actual destination peer, by hopping between additional intermediate peers.

The specific approach that Veilid takes to privacy is two sided: privacy of the sender of a message, and privacy of the receiver of a message. Either or both sides can want privacy or opt out of privacy. To achieve sender privacy, Veilid use something called a Safety Route: a sequence of any number of peers, chosen by the sender, who will forward messages. The sequence of addresses is put into a nesting doll of encryption, so that each hop can see the previous and next hops, while no hop can see the whole route. This is similar to a Tor route, except only the addresses are encrypted for each hop. The route can be chosen at random for each message being sent.

Receiver privacy is similar, in that we have a nesting doll of encrypted peer addresses, except because it's for incoming messages, the various addresses have to be shared ahead of time. We call such things Private Routes, and they are published to the key-value store as part of a user's public data. For full privacy on both ends, a Private Route will be used as the final destination of a Safety Route, and the total route is the composition of the two, so that neither the sender nor receiver knows the IP address of the other.

Each peer in the hop, including the initial peer, sends a `route` RPC to the next peer in the hop, with the remainder of the full route (safety + private), forwarding the data along. The final peer decrypts the remainder of the route, which is now empty, and then can inspect the forwarded RPC to act on it. The RPC itself doesn't need to be encrypted, but it's good practice to encrypt it for the final receiving peer so that the intermediate peers can't de-anonymize the sending user from traffic analysis.

Note that the routes are _user_ oriented. They should be understood as a way to talk to a particular _user's_ peer, wherever that may be. Each peer of course has to know about the actual IP addresses of the peers, otherwise it couldn't communicate, but safety and private routes make it hard to associate the _user's_ identity with their _peer's_ identity. You know that the user is somewhere on the network, but you don't know which IP address is their's, even if you do in fact have their peer's dial info stored in the routing table.

### Block Store Revisited

As mentioned in the Bird's Eye View, the block store is intended to store content-addressed blocks of data. Like many other peer-to-peer systems for storing data, Veilid uses a distributed hash table as the core of the block store. The block store DHT has as keys BLAKE3 hashes of block content. For each key the DHT associates a list of peer IDs for peers that have declared to the network that they can supply the block.

If a peer wishes to supply the block, it makes a `supply_block` RPC call to the network with the id of the block. The receiver of the call can then store the information that the peer supplies the designated block if it wants, and also can return other peers nearer to the block's ID that should also store the information. Peers determine whether or not to store this information based on how close it is to the block's ID. It may also choose to cache the block, possibly also declaring itself to be a supplier as well.

Supplier records are potentially brittle because peers leave the network, making their information unavailable. Because of this, any peer that wishes to supply a block will periodically send `supply_block` messages to refresh the records. Peers that are caching blocks determine when to stop caching based on how popular a block is, how much space or bandwidth it can spare, etc.

To retrieve a block that has been stored in the blockstore, a peer makes a `find_block` RPC. The receiver will then either return the block, or possibly return a list of suppliers for the block that it knows about, or return a list of peers that are closer to the block.

Unlike BitTorrent, blocks are not inherently part of a larger file. A block can be just a single file, and often that will be the case for small files. Large files can be broken up into smaller blocks, however, and then an additional block with a list of those component blocks can be stored in the block store. Veilid itself, however, would treat this like any other block, and there are no built-in mechanisms for determining which blocks to download first, which to share first, etc. like there are in BitTorrent. These features would be dependent on the peer software's implementation and could vary. Different clients will also be able to decide how they want to download such "compound" blocks -- automatically, via a prompt to the user, or something else.

The mechanism of having blocks that refer to other blocks also enables IPFS-style DAGs of hierarchical data as one mode of use of the block store, allowing entire directory structures to be stored, not just files. However, as with sub-file blocks, this is not a built-in part of Veilid but rather a mode of use, and how they're downloaded and presented to the user is up to the client program.

### Key-Value Store, Revisited

The key-value store is a DHT similar to the block store. However, rather than using content hashes as keys, the KV store uses user IDs as keys (note: _not_ peer IDs). At a given key, the KV store has a hierarchical key-value map that associates in-principle arbitrary strings with values, which themselves can be numbers, strings, datetimes, or other key-value maps. The specific value stored in at a user's ID is versioned, so that particular schemas of subkeys and values can be defined and handled appropriately by different versions of clients.

When a user wishes to store data under their key, they send a `set_value` RPC to the peer's whose IDs are closest by the XOR metric to their own user ID. The value provided to the RPC is a signed value, so that the network can ensure only the designated user is storing data at their key. The peers that receive the RPC may return other peer IDs closer to the key, and so on, similar to how the block store handles `supply_block` calls. Eventually, some peers will store the data. The user's own peer should periodically refresh the stored data, to ensure that it persists. It's also good practice for the user's own peer to cache the data, so that client programs can use the user's own peer as a canonical source of the most-up-to-date value, but doing so would require a route to be published that lets other peers send the user's own peer messages. A private route suffices for this.

Retrieval is similar to block store retrieval. The desired key is provided to a `get_value` call, which may return th value, or a list of other peers that are closer to the key. Eventually the signed data is returned, and the recipient can verify that it does indeed belong to the specified user by checking the signature.

When storing and retrieving, the key provided to the RPCs is not required to be only the user's ID. It can include a list of strings which act as a path into the data stored at the user's key, targetting it specifically for update or retrieval. This lets the network minimize data transfer, because only the relevant information has to move around.

The specific content of the user's keys is determined partially by the protocol and partially by the client software. Early versions of the protocol use a DHT schema version that defines a fairly simple social network oriented schema. Later versions will enable a more generic schema so that client plugins can store and display richer information.

The stateful nature of the key-value store means that values will change over time, and actions may need to be taken in response to those changes. A polling mechanism could be used to periodically check for new values, but this will lead to lots of unnecessary traffic in the network, so to avoid this, Veilid allows peers to send `watch_value` RPCs, with a DHT key (with subkeys) as its argument. The receiver would then store a record that the sender of the RPC wants to be alerted when the receiver gets subsequent `set_value` calls, at which time the receiver sends the sending peer a `value_changed` RPC to push the new value. As with other RPC calls, `watch_value` needs to be periodically re-sent to refresh the subscription to the value. Additionally, also as with other calls, `watch_value` may not succeed on the receiver, which instead might return other peers closer to the value, or might return other peers that have successfully subscribed to the value and thus might act as a source for it.

TODO How to avoid replay updates?? maybe via a sequence number in the signed patch?

## Appendix 1: Dial Info and Signaling

## Appendix 2: RPC Listing
