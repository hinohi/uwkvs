use std::{fmt, io::Cursor};

use bytes::Buf;
use std::fmt::Debug;

pub type Key = u64;
pub type Value = (i8, u8);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Cmd {
    Get(Key),
    Set(Key, Value),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Error {
    Incomplete,
    UnknownCommand(u8),
}

impl Cmd {
    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Cmd, Error> {
        match get_u8(src)? {
            b'g' => Ok(Cmd::Get(get_u64(src)?)),
            b's' => Ok(Cmd::Set(get_u64(src)?, get_value(src)?)),
            b => Err(Error::UnknownCommand(b)),
        }
    }
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if src.has_remaining() {
        Ok(src.get_u8())
    } else {
        Err(Error::Incomplete)
    }
}

fn get_value(src: &mut Cursor<&[u8]>) -> Result<Value, Error> {
    if 2 <= src.remaining() {
        Ok((src.get_i8(), src.get_u8()))
    } else {
        Err(Error::Incomplete)
    }
}

fn get_u64(src: &mut Cursor<&[u8]>) -> Result<u64, Error> {
    if 8 <= src.remaining() {
        Ok(src.get_u64())
    } else {
        Err(Error::Incomplete)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Incomplete => fmt::Display::fmt("stream ended early", fmt),
            Error::UnknownCommand(b) => {
                fmt::Display::fmt(&format!("unknown command: {:?}", b), fmt)
            }
        }
    }
}

impl std::error::Error for Error {}
