pub mod parser;

// https://minecraft.gamepedia.com/NBT_format#TAG_definition
pub const TAG_END: u8 = 0;
pub const TAG_I8: u8 = 1;
pub const TAG_I16: u8 = 2;
pub const TAG_I32: u8 = 3;
pub const TAG_I64: u8 = 4;
pub const TAG_F32: u8 = 5;
pub const TAG_F64: u8 = 6;
pub const TAG_I8_ARRAY: u8 = 7;
pub const TAG_STRING: u8 = 8;
pub const TAG_LIST: u8 = 9;
pub const TAG_COMPOUND: u8 = 10;
pub const TAG_I32_ARRAY: u8 = 11;
pub const TAG_I64_ARRAY: u8 = 12;

// maximum number of elements in a list
// https://minecraft.gamepedia.com/NBT_format#TAG_definition
pub const LIST_MAX_LEN: usize = 2_147_483_639;
