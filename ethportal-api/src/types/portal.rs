use crate::types::{content_item::HistoryContentItem, content_key::HistoryContentKey, discv5::Enr};
use serde::{Deserialize, Serialize};

pub type DataRadius = ethereum_types::U256;
pub type Distance = ethereum_types::U256;

pub type BitList = String;

/// Part of a TraceRecursiveFindContent response
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    pub enr: Enr,
    pub distance: Distance,
}

/// Response for Ping endpoint
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PongInfo {
    pub enr_seq: u32,
    pub data_radius: DataRadius,
}

/// Response for FindContent endpoint
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentInfo {
    #[serde(rename_all = "camelCase")]
    ConnectionId { connection_id: u16 },
    #[serde(rename_all = "camelCase")]
    Content { content: HistoryContentItem },
    #[serde(rename_all = "camelCase")]
    Enrs { enrs: Vec<Enr> },
}

/// Response for Offer endpoint
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptInfo {
    pub connection_id: u16,
    pub content_keys: BitList,
}

/// Response for TraceRecursiveFindContent endpoint
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceContentInfo {
    pub content: HistoryContentItem,
    pub route: Vec<NodeInfo>,
}

/// Response for PaginateLocalContentKeys endpoint
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginateLocalContentInfo {
    pub content_keys: Vec<HistoryContentKey>,
    pub total_entries: u64,
}
