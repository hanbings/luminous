use actix::Message;
use serde::{Deserialize, Serialize};

use super::{NodeEndpoint, RaftDataType};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct AppendEntriesRequest<T: RaftDataType> {
    pub leader_endpoint: NodeEndpoint,
    pub term: u64,
    pub prev_log_index: u64,
    pub prev_log_term: u64,
    #[serde(bound = "T: RaftDataType")]
    pub entries: Vec<T>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct AppendEntriesResponse {
    pub term: u64,
    pub success: bool,
    pub conflict_index: u64,
    pub conflict_term: u64,
}

impl<T: RaftDataType> Message for AppendEntriesRequest<T> {
    type Result = AppendEntriesResponse;
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct VoteRequest {
    pub candidate_endpoint: NodeEndpoint,
    pub term: u64,
    pub last_log_index: u64,
    pub last_log_term: u64,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct VoteResponse {
    pub last_log_index: u64,
    pub last_log_term: u64,
    pub vote_granted: bool,
}

impl Message for VoteRequest {
    type Result = VoteResponse;
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct InstallSnapshotRequest {
    pub leader_endpoint: NodeEndpoint,
    pub term: u64,
    pub start_log: (u64, u64),
    pub end_log: (u64, u64),
    pub total: u64,
    pub sent: u64,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct InstallSnapshotResponse {
    pub success: bool,
    pub received: u64,
}

impl Message for InstallSnapshotRequest {
    type Result = InstallSnapshotResponse;
}
