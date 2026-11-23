//! Compact SPIR-V reflection library written in pure-Rust with minimal allocations.
//!
//! # Usage 
//! ``` rust
//! use leimu_spirv::op;
//! use leimu_spirv::Module;
//! use leimu_spirv::reflect::{Reflector, ResourceType};
//! use leimu_spirv::include_spirv;
//!
//! 
//! let spirv = include_spirv!("../mesh.spv"); 
//! let module = Module::new(spirv).unwrap();
//! let mut reflector = Reflector::new(module).unwrap();
//! reflector.set_entry_point("main", op::ExecutionModel::MESH_EXT).unwrap();
//! for ubo in reflector.resources_for_type(ResourceType::UniformBuffer).unwrap() {
//!     match ubo {
//!         Ok(ubo) => {
//!             let mut set = None;
//!             let mut binding = None;
//!             for dec in reflector.decorations(ubo.variable_id) {
//!                 if let op::DecorationOperands::DescriptorSet { descriptor_set } = dec.decoration {
//!                     set = Some(descriptor_set);
//!                 } else if let op::DecorationOperands::Binding { binding_point } = dec.decoration {
//!                     binding = Some(binding_point);
//!                 }
//!             }
//!             println!("Uniform buffer (set {}, binding {}): {}",
//!                 set.unwrap(), binding.unwrap(), ubo.name.unwrap_or_default(),
//!             );
//!         },
//!         Err(err) => eprintln!("parse error: {err}")
//!     }
//! }
//! for pc in reflector.resources_for_type(ResourceType::PushConstant).unwrap() {
//!     match pc {
//!         Ok(pc) => {
//!             let offset = pc.offset.unwrap();
//!             let size = reflector
//!                 .type_description(pc.base_type_id)
//!                 .unwrap().size_hint.declared();
//!             println!("Push constant (offset {offset}, size {size}): {}", pc.name.unwrap_or_default());
//!         },
//!         Err(err) => eprintln!("parse error: {err}"),
//!     }
//! }
//! ```

#![warn(rustdoc::all)]

mod core;
pub mod stream;
pub mod op;
mod module;
pub mod reflect;
pub mod include_bytes;

#[cfg(test)]
mod tests;

pub use {
    core::*,
    module::*,
    include_bytes::*,
};

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadMeDoctests;
