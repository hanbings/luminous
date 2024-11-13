use actix::Message;

use super::{RaftDataType, RaftError};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]

pub struct GetLog<T: RaftDataType, E: RaftError> {
    pub term: u64,
    pub index: u64,
    t: std::marker::PhantomData<T>,
    e: std::marker::PhantomData<E>,
}

impl<T: RaftDataType, E: RaftError> Message for GetLog<T, E> {
    type Result = Result<T, E>;
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

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GetLogRange<T: RaftDataType, E: RaftError> {
    pub start: (u64, u64),
    pub end: (u64, u64),
    t: std::marker::PhantomData<T>,
    e: std::marker::PhantomData<E>,
}

impl<T: RaftDataType, E: RaftError> Message for GetLogRange<T, E> {
    type Result = Result<Vec<T>, E>;
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GetLastLog<T: RaftDataType, E: RaftError> {
    t: std::marker::PhantomData<T>,
    e: std::marker::PhantomData<E>,
}

impl<T: RaftDataType, E: RaftError> Message for GetLastLog<T, E> {
    type Result = Result<T, E>;
}
