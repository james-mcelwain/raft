extern crate byteorder;

use self::byteorder::*;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum MessageType {
    Vote = 0,
    Append = 1,
    Error = 255,
}

impl From<MessageType> for u8 {
    fn from(original: MessageType) -> u8 {
        match original {
            MessageType::Vote => 0,
            MessageType::Append => 1,
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

#[derive(Debug)]
pub struct Message {
    pub message_type: MessageType,
    pub flags: u8,
    pub size: u32,
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

