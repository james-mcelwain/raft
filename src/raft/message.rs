extern crate byteorder;

use self::byteorder::*;
use std::convert::{TryFrom,TryInto};

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Vote = 0,
    VoteReply = 1,
    Append = 10,
    AppendReply = 11,
    Error = 255,
}

impl From<MessageType> for u8 {
    fn from(original: MessageType) -> u8 {
        match original {
            MessageType::Vote => 0,
            MessageType::VoteReply => 1,
            MessageType::Append => 10,
            MessageType::AppendReply => 11,
            MessageType::Error => 255,
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
            255 => Ok(MessageType::Error),

            n => Err(MessageError::InvalidMessageType(n)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Message {
    pub message_type: MessageType,
    pub flags: u8,
    pub size: u32,
}

impl TryInto<[u8; 8]> for Message {
    type Error = MessageError;

    fn try_into(self) -> Result<[u8; 8], Self::Error> {
        Ok([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
    }
}

impl TryFrom<[u8; 8]> for Message {
    type Error = MessageError;
    fn try_from(buf: [u8; 8]) -> Result<Self, Self::Error> {
        let message_byte = buf[0].to_owned();
        let message_type = match MessageType::try_from(buf.first().unwrap().to_owned()) {
            Ok(message_type) => message_type,
            Err(e) => return Err(e)
        };

        let flags = buf[1].to_owned();

        let size = byteorder::BigEndian::read_u32(&buf[2..6].to_owned()).to_owned();

        Ok(Message {
            message_type,
            flags,
            size
        })
    }
}
