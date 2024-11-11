use actix::Message;

use super::{RaftDataType, RaftError};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]

pub struct GetLog<T: RaftDataType, E: RaftError> {
    pub term: u64,
    pub index: u64,
    pub data: T,
    e: std::marker::PhantomData<E>,
}

impl<T: RaftDataType, E: RaftError> Message for GetLog<T, E> {
    type Result = Result<(), E>;
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SaveLog<T: RaftDataType, E: RaftError> {
    pub term: u64,
    pub index: u64,
    pub data: T,
    e: std::marker::PhantomData<E>,
}

impl<T: RaftDataType, E: RaftError> Message for SaveLog<T, E> {
    type Result = Result<(), E>;
}
