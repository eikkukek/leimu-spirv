# Cangelog
All changes to this project will be documented in this file.
## [0.5.6] - 2026-11-21
### Fixed
- `Module::parse_next_capture_inst` returning only instructions with a result id.
### Changed
- `Module::parse_next_capture_inst` to take in a mutable reference of the module in the closure.
## [0.5.5] - 2026-11-19
### Added
- `parse_next_capture_inst` and `parse_full_capture_inst` for `Module`.
## [0.5.4] - 2026-11-19
### Changed
- Internal result storage.
### Removed
- `fmt::Display` implementations for "c-enums" and Rust enums.
- manual bitmask implementations in favor using the `bitflags` macro.
## [0.5.3] - 2026-11-18
### Added
- Exposed `InstInfo` with `op::Code::get_inst_info`.
## [0.5.2] - 2026-11-18
### Changed
- Internals to take endianness into account (untested for little endian).
### Fixed
- Docs
## [0.5.1] - 2026-11-18
### Fixed
- Errors in upper snake case names

## [0.5.0] - 2026-11-18
### Changed
- SPIR-V autogeneration (`Hash` for enums)
## [0.4.2] - 2026-11-02
### Fixed
- license format
## [0.4.1] - 2026-11-02
### Fixed
- Docs
## [0.4.0] - 2026-11-02
### Changed
- Using UTF-8 string slices over just `CStr` for correctness.
### Added
- `Module::from_bytes`

## [0.3.1] - 2026-04-25
### Added
- License

## [0.3.0] - 2026-04-11

### Changed
- Name change to leimu-spirv
## [0.2.0] - 2026-04-10

### Added

- Version parsing for modules.
- Validation of the first words in SPIR-V.
- Required entry point specification.

### Removed
- Rust definitions of SPIR-V structs

## [0.1.1] - 2026-04-9

### Added
- More information for resources

### Fixed
- Resources not showing up in some versions of SPIR-V
##  [0.1.0] - 2026-04-09

### Added
- Initial crate release
- Basic reflection

[0.5.4]: https://crates.io/crates/leimu-spirv/0.5.4
[0.5.3]: https://crates.io/crates/leimu-spirv/0.5.3
[0.5.2]: https://crates.io/crates/leimu-spirv/0.5.2
[0.5.1]: https://crates.io/crates/leimu-spirv/0.5.1
[0.5.0]: https://crates.io/crates/leimu-spirv/0.5.0
[0.4.2]: https://crates.io/crates/leimu-spirv/0.4.2
[0.4.1]: https://crates.io/crates/leimu-spirv/0.4.1
[0.4.0]: https://crates.io/crates/leimu-spirv/0.4.0
[0.3.1]: https://crates.io/crates/leimu-spirv/0.3.1
[0.3.0]: https://crates.io/crates/leimu-spirv/0.3.0
[0.2.0]: https://crates.io/crates/nox-spirv/0.2.0
[0.1.1]: https://crates.io/crates/nox-spirv/0.1.1
[0.1.0]: https://crates.io/crates/nox-spirv/0.1.0
