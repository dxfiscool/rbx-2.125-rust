# rbx-2.125-rust — Roblox 2.125.39335 iOS Rust Decomp (from scratch)

> IPA `ROBLOX 2.125.39335` (2013-10-17) → `robloxmobile` Mach-O armv7, PIE. IDA ground truth at `127.0.0.1:13337`. Toolchain: Rust 1.98, `omp` (oh-my-pi) v18.

**This is a decompilation, written in Rust, 1:1 fidelity.** Every function, every output, same code paths. `boost::*` is not vendored — replaced with `Arc`/`Signal`/`std::thread` etc.

## New architecture (DAG, not monolith)

```
core (0 deps) -> reflection -> datamodel -> script -> platform
                \-> rendering -------> platform
                \-> network ---------> platform
                \-> audio -----------> platform
```

- `core` — `rbx::` primitives (`SharedPtr=Arc`, `Signal`, `TaskScheduler`) — builds <1s
- `reflection` — `RBX::Reflection` (EnumDesc, Descriptor)
- `datamodel` — `RBX::Instance`, `DataModel`, `Workspace` (largest, but isolated)
- `script` — Lua VM
- `rendering` — Ogre + G3D
- `network` — RakNet
- `audio` — FMOD
- `platform` — iOS (`RobloxView`, controllers) — ObjC only here, depends all

Split old `app` monolith (57k funcs) into 3 crates for parallel `omp` work. See `docs/ARCHITECTURE.md`.

## Quick start

```bash
# IDA must be running with robloxmobile.i64
cargo xtask ida-export          # writes ida/export.json (85,545 funcs) + ida/metadata.json
cargo xtask boost-audit         # reports 22k boost sites per crate
cargo xtask verify              # cargo check + clippy -D warnings

cargo check --workspace
cargo clippy --workspace -- -D warnings

# omp (reads AGENT.md)
omp --cwd .                     # interactive
omp --print "xtask gen-skeleton --crate reflection"
```

## Layout 1:1

```
Client/Base/rbx/TaskScheduler.cpp        -> crates/core/src/task_scheduler.rs
Client/App/include/reflection/Descriptor.h -> crates/reflection/src/descriptor.rs
Client/App/v8datamodel/DataModel.cpp     -> crates/datamodel/src/data_model.rs
Ogre/OgreEntity.cpp                       -> crates/rendering/src/ogre/mod.rs
Client/iOS/RobloxView.mm                  -> crates/platform/src/roblox_view.rs
```

Each fn keeps `// 0xADDR — __ZN...` + `#[doc(alias = "RBX::...")]`.

## Boost → Rust

`boost::shared_ptr<T>` → `rbx_core::SharedPtr<T>` (`Arc<T>`), `boost::signals` → `rbx_core::signal::Signal`, `boost::bind/function` → closures, `boost::thread` → `std::thread`. No `boost` crate. See `docs/BOOST.md` + `cargo xtask boost-audit`.

## Fidelity

Every IDA func appears as `todo!("0xADDR")` or impl with `// IDA 0xADDR`. `cargo xtask verify` enforces 1:1. See `docs/FIDELITY.md`.

## GitHub

Repo `dxfiscool/rbx-2.125-rust`, issues per crate (`area/*`, `phase/*`), CI `cargo check + clippy`.

## IDA

MCP `POST http://127.0.0.1:13337/mcp` — `tools/call` only, see `AGENT.md` for `py` helper.

