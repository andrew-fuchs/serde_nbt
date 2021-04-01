// Serde library for reading/writing to Minecraft NBT format as
// documented at https://minecraft.gamepedia.com/NBT_format

mod de;
mod error;
// FIXME: this module should be made private
pub mod nbt;
// mod ser;

pub use crate::error::{Error, Result};
pub use crate::de::{from_reader, Deserializer};
// pub use ser::{to_writer, Serializer};
