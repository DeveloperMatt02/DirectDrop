use crate::{
    TransferState,
    hdl::{TransportInfo, info::TransferMetadata},
};

#[derive(Debug, Clone, PartialEq)]
pub enum TransferAction {
    ConsentAccept,
    ConsentDecline,
    TransferCancel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransferKind {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone)]
pub struct MessageClient {
    pub kind: TransferKind,
    pub state: Option<TransferState>,
    pub metadata: Option<TransferMetadata>,
    /// The link the transfer runs on; re-sent whenever it changes (the latest one counts).
    pub transport: Option<TransportInfo>,
}

// TODO: This should be separate structs
#[derive(Debug, Clone)]
pub enum Message {
    Lib { action: TransferAction },
    Client(MessageClient),
}

impl Message {
    pub fn as_client(&self) -> Option<&MessageClient> {
        match self {
            Message::Client(message_client) => Some(message_client),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChannelMessage {
    pub id: String,
    pub msg: Message,
}
