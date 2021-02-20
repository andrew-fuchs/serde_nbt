// Serde library for reading/writing to Minecraft NBT format as
// documented at https://minecraft.gamepedia.com/NBT_format

mod de;
mod error;
mod ser;

pub use de::{from_reader, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_writer, Serializer};
