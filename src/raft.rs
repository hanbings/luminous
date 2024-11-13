use actix::{dev::ToEnvelope, Actor, Context, Handler};
use network::{AppendEntriesRequest, InstallSnapshotRequest, VoteRequest};
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt::Debug};

pub mod action;
pub mod log;
pub mod network;
pub mod node;
pub mod persistent;

/// We need to define some types globally so that we can pass them accurately throughout the implementation.
///
/// Generally speaking, using `NodeId` should be able to normally index the endpoint part of a `Node`
/// in a data structure containing all `Node`.
/// However, in order to better simplify some unnecessary operations
/// (such as requesting `Node` information back and forth in the first few RPCs when a new node joins the cluster),
/// we also define a `NodeEndpoint` for emergencies.
pub type NodeId = u64;
pub type NodeEndpoint = String;
/// `RaftDataType` is used to define the data stored
/// in the internal data structure (the state inside the state machine).
///
/// Generics are abbreviated as `T`.
pub trait RaftDataType:
    Clone + Debug + Send + Sync + Serialize + DeserializeOwned + 'static
{
}
/// `RaftError` will be used throughout the system to return possible errors.
/// Therefore, users should define their own error types to convey accurate error information.
/// Generics are abbreviated as `E``.
pub trait RaftError: Error + Debug + Send + Sync + Serialize + DeserializeOwned + 'static {}

pub trait RaftNetwork<T>
where
    T: RaftDataType,
    Self: Actor<Context = Context<Self>>,

    Self: Handler<AppendEntriesRequest<T>>,
    Self::Context: ToEnvelope<Self, AppendEntriesRequest<T>>,

    Self: Handler<VoteRequest>,
    Self::Context: ToEnvelope<Self, VoteRequest>,

    Self: Handler<InstallSnapshotRequest>,
    Self::Context: ToEnvelope<Self, InstallSnapshotRequest>,
{
}

pub trait RaftLog<T, E>
where
    T: RaftDataType,
    E: RaftError,
    Self: Actor<Context = Context<Self>>,

    Self: Handler<log::GetLog<T, E>>,
    Self::Context: ToEnvelope<Self, log::GetLog<T, E>>,

    Self: Handler<log::SaveLog<T, E>>,
    Self::Context: ToEnvelope<Self, log::SaveLog<T, E>>,

    Self: Handler<log::GetLogRange<T, E>>,
    Self::Context: ToEnvelope<Self, log::GetLogRange<T, E>>,

    Self: Handler<log::GetLastLog<T, E>>,
    Self::Context: ToEnvelope<Self, log::GetLastLog<T, E>>,
{
}

pub trait RaftPersistent<T, E>
where
    T: RaftDataType,
    E: RaftError,
    Self: Actor<Context = Context<Self>>,

    Self: Handler<persistent::FullPersistent<T, E>>,
    Self::Context: ToEnvelope<Self, persistent::FullPersistent<T, E>>,

    Self: Handler<persistent::PartialPersistent<T, E>>,
    Self::Context: ToEnvelope<Self, persistent::PartialPersistent<T, E>>,

    Self: Handler<persistent::LoadPersistent<T, E>>,
    Self::Context: ToEnvelope<Self, persistent::LoadPersistent<T, E>>,
{
}
