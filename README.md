![Luminous](https://picture.hanbings.io/2024/10/15/d94d5f39b7fff8fd3ffd7aa1215b7422.png)

<h1 align="center">🌟 Luminous</h1>

## What's this?

In another repository [Iris](https://github.com/hanbings/icarus/tree/main/iris), a very simple Raft tool library is written based on the [original Raft paper](https://pdos.csail.mit.edu/6.824/papers/raft-extended.pdf) and using `actix-web`.

The tool library includes the following parts:

- Follower, Leader and Candidate node state machines
- Term, log index
- `AppendEntries` RPC, `VoteRequest` RPC
- Heartbeat packet mechanism, timeout election, random election timeout and election timeout

Unimplemented parts:

- Log consistency check process
- High consistency (to be precise, the log replication process before the node joins the cluster)
- Two-stage method required to solve the cluster split problem
- Log compression

In this tool library, only keys and values ​​of type `String` can be stored with fixed logic, because the internal data storage uses `Map<String, String>`. Obviously, this is a huge flaw, which means redundant serialization and deserialization steps. Considering Rust's ownership mechanism, there will be additional clones. And because the logic is fixed, it is also difficult to implement the extended functions of ordinary kv storage (such as automatic expiration, automatic disk backup, shard storage, etc.) in every application using the tool library.

## Usage

According to the original paper, it is divided into the following parts:

- Timer (heartbeat packet mechanism, timeout mechanism, random election timeout and election timeout mechanism)
- Network (including RPC, serialization and deserialization, and data packet processing according to the state machine)
- Internal storage
- Disk persistence

In order to achieve the best possible performance and concurrency safety, the `Actor` mode is first considered to encapsulate the operations.

Taking storing a `Log Entry` as an example, the specific method is as follows:

1. Define basic data types

   ```rust
   pub trait RaftDataType: Clone + Debug + Send + Sync + Serialize + DeserializeOwned + 'static {}
   pub trait RaftError: Error + Debug + Send + Sync + Serialize + DeserializeOwned + 'static {}
   ```

   The basic data type here refers to the data that the `Log Entry` will carry in the entire implementation. For example, the basic data type in the Iris implementation is `String`.

   In addition, for the sake of implementation completeness, we also need to define an `Error` type to express as accurately as possible the errors that may occur during data processing.

2. Implement a `SaveLog` structure and implement `actix::Message`

   ```rust
   #[derive(Clone, Eq, PartialEq, Hash, Debug)]
   pub struct SaveLog<T: RaftDataType, E: RaftError> {
       pub term: u64,
       pub index: u64,
       pub data: T,
       e: std::marker::PhantomData<E>,
   }
   
   impl<T: RaftDataType, E: RaftError> actix::Message for SaveLog<T, E> {
       type Result = Result<(), E>;
   }
   ```

   Does this look a bit strange? Actually, this `SaveLog` acts as a function, declaring the parameters that will be passed to `Actor` (`term: u64, index: u64, data: T`) and the return value (`type Result = Result<(), E>;`).

   As for `std::marker::PhantomData<E>`, since `E` is not used as a field in the structure, it will generate [unbounded lifetime](https://doc.rust-lang.org/nomicon/unbounded-lifetimes.html). So we need to use a placeholder tool so that it can be declared in the structure but cannot generate real memory consumption, [PhantomData](https://doc.rust-lang.org/nomicon/phantom-data.html) is very suitable.

3. Define an "aggregate" feature, provide it to users to implement and pass it into the implementation through an interface.

   ```rust
   pub trait RaftLog<T, E>
   where
       T: RaftDataType,
       E: RaftError,
       Self: Actor<Context = Context<Self>>,
       Self: Handler<log::SaveLog<T, E>>,
       Self::Context: ToEnvelope<Self, log::SaveLog<T, E>>,
   {
   }
   ```

   At this point, we have provided a "function" for users to process and save `Log Entry`. The usage is as follows:
   
   ```rust
   impl RaftLog<String, XXError> for RaftCluster<String, XXError> { }
   
   impl Actor for RaftCluster<String, XXError> {
       type Context = XXContext;
   }
   
   impl Handler<log::SaveLog<String, XXError>> for RaftCluster<String, XXError> {
       fn handle(&mut self, msg: SaveLog<String, XXError>) {
           // Perform the actual storage step here
       }
   }
   ```

## References and Acknowledgements

[In Search of an Understandable Consensus Algorithm](https://pdos.csail.mit.edu/6.824/papers/raft-extended.pdf) Original Raft paper

A rigorous and scalable implementation of [actix-raft](https://github.com/bjornmolin/actix-raft)

[raft-rs](https://github.com/tikv/raft-rs) is a high-performance and easy-to-understand implementation

Thanks!
