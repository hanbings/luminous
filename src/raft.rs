use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt::Debug};

pub mod action;
pub mod node;
pub mod request;

pub type NodeEndpoint = String;
pub trait RaftDataType:
    Clone + Debug + Send + Sync + Serialize + DeserializeOwned + 'static
{
}
pub trait RaftError: Error + Debug + Send + Sync + Serialize + DeserializeOwned + 'static {}
