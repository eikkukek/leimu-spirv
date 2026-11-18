//! SPIR-V Op-types and enumerations.

mod common;
mod enums;
mod definitions;
mod inst_info;

pub use common::*;
pub use enums::*;
pub use definitions::*;
pub(crate) use inst_info::INST_INFOS;
impl Code {
    #[inline]
    pub fn get_inst_info(self) -> &'static InstInfo {
        &INST_INFOS[self.0 as usize]
    }
}
