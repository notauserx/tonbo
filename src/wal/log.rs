use std::mem::size_of;

use fusio::{SeqRead, Write};

use crate::serdes::{Decode, Encode};

#[derive(Debug)]
pub struct Log<Re> {
    pub log_type: LogType,
    pub record: Re,
}

impl<Re> Log<Re> {
    pub fn new(log_type: LogType, record: Re) -> Self {
        Self { log_type, record }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum LogType {
    Full,
    First,
    Middle,
    Last,
}

impl From<u8> for LogType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Full,
            1 => Self::First,
            2 => Self::Middle,
            3 => Self::Last,
            _ => unreachable!(),
        }
    }
}

impl<Re> Encode for Log<Re>
where
    Re: Encode + Sync,
{
    type Error = Re::Error;

    async fn encode<W>(&self, writer: &mut W) -> Result<(), Self::Error>
    where
        W: Write,
    {
        (self.log_type as u8).encode(writer).await?;
        self.record.encode(writer).await
    }

    fn size(&self) -> usize {
        size_of::<u8>() + self.record.size()
    }
}

impl<Re> Decode for Log<Re>
where
    Re: Decode,
{
    type Error = Re::Error;

    async fn decode<R>(reader: &mut R) -> Result<Self, Self::Error>
    where
        R: SeqRead,
    {
        let log_type = LogType::from(u8::decode(reader).await?);
        let log = Re::decode(reader).await?;

        Ok(Self {
            log_type,
            record: log,
        })
    }
}
