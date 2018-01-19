use std::convert::TryFrom;

#[derive(Debug)]
pub enum MessageType {
    Vote = 0,
    Append = 1
}

impl From<MessageType> for u8 {
    fn from(original: MessageType) -> u8 {
        match original {
            MessageType::Vote => 0,
            MessageType::Append => 1,
        }
    }
}

#[derive(Debug)]
pub enum MessageError {
    InvalidMessageType(u8)
}

impl TryFrom<u8> for MessageType {
    type Error = MessageError;
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(MessageType::Vote),
            1 => Ok(MessageType::Append),
            n => Err(MessageError::InvalidMessageType(n))
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub message_type: MessageType,
    pub size: u32
}

