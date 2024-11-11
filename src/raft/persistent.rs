use actix::Message;

use super::{RaftDataType, RaftError};

pub struct FullPersistent<T: RaftDataType, E: RaftError> {
    t: std::marker::PhantomData<T>,
    e: std::marker::PhantomData<E>,
}

impl<T: RaftDataType, E: RaftError> Message for FullPersistent<T, E> {
    type Result = Result<(), E>;
}

pub struct PartialPersistent<T: RaftDataType, E: RaftError> {
    t: std::marker::PhantomData<T>,
    e: std::marker::PhantomData<E>,
}

impl<T: RaftDataType, E: RaftError> Message for PartialPersistent<T, E> {
    type Result = Result<(), E>;
}

pub struct LoadPersistent<T: RaftDataType, E: RaftError> {
    t: std::marker::PhantomData<T>,
    e: std::marker::PhantomData<E>,
}

pub struct LoadedPersistent<T: RaftDataType> {
    pub data: Vec<T>,
    pub index: u64,
    pub term: u64,
    pub done: bool,
    t: std::marker::PhantomData<T>,
}

impl<T: RaftDataType, E: RaftError> Message for LoadPersistent<T, E> {
    type Result = Result<LoadedPersistent<T>, E>;
}
