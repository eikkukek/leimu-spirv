use core::{
    slice,
    fmt::{self, Display},
};

/// A trait for types, which can be trivially created from a [`u32`].
pub trait Word: Sized {
    
    fn from_word(word: u32) -> Self;
}

impl Word for u32 { 

    #[inline]
    fn from_word(word: u32) -> Self {
        word
    }
}

#[inline]
pub(crate) fn slice_as_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe {
        slice::from_raw_parts(slice.as_ptr() as *const u8, size_of_val(slice))
    }
}

/// Represents a string inside SPIR-V.
///
/// Doesn't do any extra allocations and is trivially copyable.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct LiteralStr<'a> {
    bytes: &'a [u8],
}

impl<'a> LiteralStr<'a> {

    /// Constructs a new string from words.
    pub fn new(words: &'a [u32]) -> crate::ParseResult<Self> {
        let bytes = slice_as_bytes(words);
        let len = bytes
            .iter()
            .copied()
            .take_while(|&byte| byte != 0)
            .count();
        let str = str::from_utf8(&bytes[0..len+1])?;
        Ok(Self {
            bytes: str.as_bytes(),
        })
    }

    /// Returns the length of this SPIR-V string.
    ///
    /// This includes the nul-byte.
    #[allow(clippy::len_without_is_empty)]
    pub fn len_with_nul(&self) -> usize {
        self.bytes.len()
    }

    /// Converts self to a [`str`].
    #[inline]
    pub fn as_str(&self) -> &'a str {
        unsafe {
            str::from_utf8_unchecked(&self.bytes[0..self.bytes.len() - 1])
        }
    }
}

impl Display for LiteralStr<'_> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Specifies the value of a literal constant.
#[derive(Clone, Copy, Debug)]
pub enum Literal {
    /// 16-bit floating point literal.
    ///
    /// Not representable in Rust without nightly.
    F16(u16),
    /// 32-bit floating point literal.
    F32(f32),
    /// 64-bit floating point literal.
    F64(f64),
    /// 8-bit signed integer literal.
    I8(i8),
    /// 16-bit signed integer literal.
    I16(i16),
    /// 32-bit signed integer literal.
    I32(i32),
    /// 64-bit signed integer literal.
    I64(i64),
    /// 8-bit unsigned integer literal.
    U8(u8),
    /// 16-bit unsigned integer literal.
    U16(u16),
    /// 32-bit unsigned integer literal.
    U32(u32),
    /// 64-bit unsigned integer literal.
    U64(u64),
}

impl Literal {

    /// Converts self to an usize, if its an integer literal.
    #[inline]
    pub fn as_usize(self) -> Option<usize> {
        match self {
            Self::I8(x) => Some(x as usize),
            Self::I16(x) => Some(x as usize),
            Self::I32(x) => Some(x as usize),
            Self::I64(x) => Some(x as usize),
            Self::U8(x) => Some(x as usize),
            Self::U16(x) => Some(x as usize),
            Self::U32(x) => Some(x as usize),
            Self::U64(x) => Some(x as usize),
            _ => None,
        }
    }
}

impl Display for Literal {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::F16(x) => write!(f, "{x:#x}"),
            Self::F32(x) => write!(f, "{x}"),
            Self::F64(x) => write!(f, "{x}"),
            Self::I8(x) => write!(f, "{x}"),
            Self::I16(x) => write!(f, "{x}"),
            Self::I32(x) => write!(f, "{x}"),
            Self::I64(x) => write!(f, "{x}"),
            Self::U8(x) => write!(f, "{x}"),
            Self::U16(x) => write!(f, "{x}"),
            Self::U32(x) => write!(f, "{x}"),
            Self::U64(x) => write!(f, "{x}"),
        }
    }
}
