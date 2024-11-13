use super::{NodeEndpoint, RaftDataType, RaftError, RaftLog, RaftNetwork, RaftPersistent};
use actix::Addr;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub enum RaftNodeType {
    #[default]
    Follower,
    Candidate,
    Leader,
}

impl Display for RaftNodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RaftNodeType::Follower => write!(f, "Follower"),
            RaftNodeType::Candidate => write!(f, "Candidate"),
            RaftNodeType::Leader => write!(f, "Leader"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct RaftNode {
    pub endpoint: NodeEndpoint,
    pub node_type: RaftNodeType,
    pub current_term: u32,
    pub current_voted_for: Option<NodeEndpoint>,
    pub current_leader: Option<NodeEndpoint>,
    pub commit_index: u32,
    pub last_applied: u32,
    pub next_index: Vec<u32>,
    pub match_index: Vec<u32>,
}

pub struct RaftCluster<T, E, N, L, P>
where
    T: RaftDataType,
    E: RaftError,
    N: RaftNetwork<T>,
    L: RaftLog<T, E>,
    P: RaftPersistent<T, E>,
{
    pub node: RaftNode,
    pub nodes: Vec<RaftNode>,
    pub logs: Vec<T>,
    pub network: Addr<N>,
    pub log: Addr<L>,
    pub persistent: Addr<P>,
    e: std::marker::PhantomData<E>,
}
