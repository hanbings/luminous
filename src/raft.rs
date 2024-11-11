use actix::{dev::ToEnvelope, Actor, Context, Handler};
use network::{AppendEntriesRequest, InstallSnapshotRequest, VoteRequest};
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt::Debug};

pub mod action;
pub mod log;
pub mod network;
pub mod node;
pub mod persistent;

pub type NodeEndpoint = String;

pub trait RaftDataType:
    Clone + Debug + Send + Sync + Serialize + DeserializeOwned + 'static
{
}

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
