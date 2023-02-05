# RustLR: Distributed, language agnostic OpenLR decoder

RustLR is an OpenLR decoder library and framework that attempts to combine the best ideas of existing decoders while addressing their shortcomings.  

# Why another decoder?

- Fast decoder logic: written (probably badly) in Rust
- Fast, reliable transport: uses gRPC via protobufs for map adapter <-> decoder communication
- Realtime: map adapters can directly access live customer data rather than using an imported snapshot
- Language agnostic architecture: clients, decoder, and map adapters are decoupled
- Protocol agnostic architecture: SWIG, Unix domain sockets, Kafka, WebSockets, HTTP/S, etc...
- Simple adapter API: only 2 methods to implement (Java reference decoder requires 34 implementations)
- Distributed: decoder logic, MapDatabase adapters, and clients are logically independent and can be deployed and scaled independently
- Accurate: correctly handles ranges for bearings, DNPs, and offsets resulting from binary OpenLR “buckets”
- Correct: LRP candidate combinations are considered in correct order
- Asynchronous: decoding threads not blocked waiting for I/O from Map adapters
- Flexible tuning parameters: i.e. expected/actual FOW/FRC/bearing tables are fully exposed and customisable
- Improved diagnostics: diagnostic level chosen for each request, and log messages returned with response

# Architecture

RustLR consists of three distributed, decoupled, language and location agnostic components:

## Decoder
The core OpenLR decoder logic which accepts a request from a client containing the following elements:

- OpenLR LocationReference
- set of decoding parameters
- reference to a map server representing the target map
- a logging level

The decoder makes requests to the map server and sends to the client either an OpenLR Location and log messages, or else an explanatory error.  The decoder is completely asynchronous, and can process an aribtrary number of client requests concurrently without blocking on requests to the OS or to the map database server.  

## Map database server
Similar in spirit to the abstract map adapter found in the Openlr Java reference implementation, the map database is a server that responds to geospatial requests from the decoder over an arbitrary transport (currently gRPC is the only transport supported).  There can be an arbitary number of map database servers running at any time, each proxying a map against which OpenLR codes are to be decoded. 
