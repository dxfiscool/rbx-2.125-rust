# FIDELITY — 1:1 (Rust but still a decompilation)

- Every IDA func (85,545) appears in Rust as `todo!("0xADDR")` or impl with `// IDA 0xADDR`. No merging.
- Outputs identical for same inputs. Preserve original bugs with `// BUG: original at 0xADDR` + `#[cfg(feature = "fix-bugs")]`.
- Control flow preserved, then `clippy`.
- `#[repr(C)]` + `size_of` asserts for binary-layout structs.
- Before push: `cargo xtask verify` (check + clippy). CI enforces.
