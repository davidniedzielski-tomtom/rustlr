# RustLR: distributed, asynchronous, language agnostic, opinionated OpenLR decoder

RustLR is an OpenLR decoder library and framework that attempts to combine the
best ideas of existing decoders while addressing some of their shortcomings.  

# Why yet another OpenLR implementation?

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

RustLR consists of three distributed, decoupled, language and location agnostic
components:

## OpenLR Server
The core OpenLR decoder / encoder logic (currently only decoding is supported).
The server accepts requests from clients containing the following elements:

- request id (used to associate asynchronous responses with requests)
- OpenLR LocationReference or binary string (decoding) or OpenLR Location (encoding)
- named set of decoding/encoding parameters
- reference to a map server representing the target map
- credentials authenticating client to selected map server
- logging verbosity level

The server makes requests to the map server over the map server specific
protocol (currently REST and gRPC are implemented) and sends to the client
either an OpenLR Location and log messages, or else an explanatory error.  The
decoder is completely asynchronous, and can process an aribtrary number of
client requests concurrently without blocking on requests to the OS or to the
map database server.  

## Map database server
Similar in spirit to the abstract map adapter found in the Openlr Java reference
implementation, the map database is a server that responds to geospatial
requests from the decoder over an arbitrary transport (currently REST and gRPC
are the only transports implemented).  It at least is able to perform a simple
radius search for edges(raods) in it's map within a certain radius of a point,
and also determine which edges (raods) are reachable from a given source edge.
It can opetionally return metadata about the map it is serving, including the
bounding box, source, and version.  There can be an arbitary number of map
database servers running at any time, each proxying a map against which OpenLR
codes are to be decoded. 

## Client
A client submits OpenLR requests to the server and accepts the response.  The
client can (currently) communicate with the OpenLR server via REST (either with
JSON-encoded messages or protobufs), or else via gRPC.