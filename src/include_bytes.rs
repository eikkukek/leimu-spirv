use core::ops::Deref;

/// A pile of word returned by [`include_words`].
pub struct Words<const N: usize> {
    len: usize,
    bytes: [u8; N],
}

impl<const N: usize> Words<N> {

    #[doc(hidden)]
    pub const fn new(bytes: [u8; N]) -> Self {
        let len = bytes.len() & !(4 - 1);
        Self {
            len,
            bytes,
        }
    }
}

impl<const N: usize> Deref for Words<N> {

    type Target = [u32];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            core::slice::from_raw_parts(
                self.bytes.as_ptr().cast(),
                self.len / 4
            )
        }
    }
}

/// Includes a file as a slice of [`u32`] values.
///
/// If the number of bytes in the file is not a multiple of four, this will truncate the byte array
/// to the last multiple of four.
#[macro_export]
macro_rules! include_spirv {
    ($file:expr $(,)?) => {
        &$crate::Words::new(*include_bytes!($file))
    };
}
