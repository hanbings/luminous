use super::NodeEndpoint;
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
