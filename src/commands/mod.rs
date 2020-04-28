//! Strongly typed, pre-built commands.
//!
//! This module contains pre-made definitions of commands and responses, so you don't have to
//! wrangle the stringly-typed raw responses if you don't want to.
//!
//! The fields on the contained structs are mostly undocumented, see the [MPD protocol
//! documentation][0] for details on their specific meaning.
//!
//! [0]: https://www.musicpd.org/doc/html/protocol.html#command-reference

mod definitions;
pub mod responses;

pub use definitions::*;
use responses::Response;

/// Stable identifier of a song in the queue.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SongId(pub u64);

impl From<u64> for SongId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

/// Position of a song in the queue.
///
/// This will change when the queue is modified.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SongPosition(pub usize);

impl From<usize> for SongPosition {
    fn from(pos: usize) -> Self {
        Self(pos)
    }
}

/// Types which can be used as pre-built properly typed commands.
pub trait Command {
    /// The response this command expects.
    type Response: Response;

    /// Create the "raw" command representation for transmission.
    fn to_command(self) -> mpd_protocol::Command;
}
