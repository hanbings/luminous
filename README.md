![Luminous](https://picture.hanbings.io/2024/10/15/d94d5f39b7fff8fd3ffd7aa1215b7422.png)

<h1 align="center">🌟 Luminous</h1>

## 为什么有这个库

简单来说，在另一个存储库 [Iris](https://github.com/hanbings/icarus/tree/main/iris) 中，基于 [Raft 原始论文](https://pdos.csail.mit.edu/6.824/papers/raft-extended.pdf) 和使用 `actix-web` 来实现了一个非常简单的 Raft 共识算法的分布式算法。

这个实现包括以下部分：

- Follower、Leader 和 Candidate 节点状态机
- 任期、日志索引
- `AppendEntries` RPC、`VoteRequest` RPC
- heartbeat 包机制、超时选举、随机选举超时和选举超时

未实现部分：

- 日志一致性检查流程
- 高度一致性（准确来说，是节点加入集群前的日志复制流程）
- 解决集群分裂问题所需的两阶段方式
- 日志压缩

在这个实现中，只能够以固定逻辑存储 `String` 类型的键和值，因为内部数据存储使用 `Map<String, String>`，显而易见这是一个巨大的缺陷，这意味着多余的序列化和反序列化步骤，考虑到 Rust 的所有权机制等，还会有额外的 clone，且因为逻辑固定，也很难在每一个使用本系统的应用中实现普通 kv 存储器的拓展功能（如自动过期、自动落盘备份、分片存储等）。

## 主要内容

考虑按照原始论文的划分分为以下部分：

- 计时器（心跳包机制、超时机制、随机选举超时和选举超时机制）
- 网络（包括 RPC 、序列化反序列化和根据状态机处理数据包）
- 内部存储
- 落盘持久化

为了尽可能好的性能和并发安全，首先考虑了 `Actor` 模式进行封装。

以存储一个 Log Entry 为例子，具体方式如下：

1. 定义基本数据类型

   ```rust
   pub trait RaftDataType: Clone + Debug + Send + Sync + Serialize + DeserializeOwned + 'static {}
   pub trait RaftError: Error + Debug + Send + Sync + Serialize + DeserializeOwned + 'static {}
   ```

   这里指的数据类型是在整个实现中，Log Entry 将携带的数据，例如 Iris 实现中的基本数据类型是 `String`。

   此外，从实现的完整性来说，我们还需要定义一个 Error 类型，以尽可能准确表达处理数据过程中可能出现的错误。

2. 实现一个 SaveLog 结构体并实现 `actix::Message`

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

   看起来会有些怪？实际上这个 `SaveLog` 充当的是一个函数的作用，声明了将会传入 `Actor` 的参数（`term: u64, index: u64, data: T`）以及返回值（`type Result = Result<(), E>;`）。

   至于 `std::marker::PhantomData<E>`，是因为在结构体中并没有使用 E 作为一个字段，它将产生 [无界生命周期](https://doc.rust-lang.org/nomicon/unbounded-lifetimes.html)。于是我们需要使用一个用于占位的工具，使得它在结构体内被声明但不能产生真正的内存消耗，[PhantomData](https://doc.rust-lang.org/nomicon/phantom-data.html) 就很适合。

3. 定义一个 “聚合” 特性，提供给用户实现它并通过某个接口传入实现中。

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

   到这一步来说，我们就为用户提供了一个 “函数”，用于处理保存 Log Entry。使用方式大致如下：
   
   ```rust
   impl RaftLog<String, XXError> for RaftCluster<String, XXError> { }
   
   impl Actor for RaftCluster<String, XXError> {
       type Context = XXContext;
   }
   
   impl Handler<log::SaveLog<String, XXError>> for RaftCluster<String, XXError> {
       fn handle(&mut self, msg: SaveLog<String, XXError>) {
           // 在这里进行实际的存储步骤
       }
   }
   ```

## 参考与致谢

[In Search of an Understandable Consensus Algorithm](https://pdos.csail.mit.edu/6.824/papers/raft-extended.pdf) Raft 的原始论文

[actix-raft](https://github.com/bjornmolin/actix-raft) 的严谨且具有拓展性实现

[raft-rs](https://github.com/tikv/raft-rs) 是一个高性能和易于理解的实现

感谢！
