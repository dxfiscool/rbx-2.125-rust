# New Architecture — rbx-2.125-rust (from scratch)

## Why new arch (vs roblox2013/robloxmobile-2.125-rust-decomp old)

Old arch failed:
- `crates/app` was 57k funcs monolith — too large for parallel omp, slow cargo check, huge merge conflicts
- `base/app/ios` split was namespace-only, not dependency DAG — ios depended on app but app also pulled ios types via ObjC, circular mental model
- No `xtask` — IDA export, boost audit, skeleton gen were loose python scripts, not `cargo xtask` reproducible
- No fidelity harness — no `cargo xtask verify` to assert 1:1 before push
- Boost table was doc-only, not enforced by `clippy`/`allow` + type aliases

New arch fixes:

### 1. Dependency DAG, not namespace

```
core (0 deps) -> reflection -> datamodel -> script -> platform
                \-> rendering -> platform
                \-> network -> platform
                \-> audio -> platform
```

- `core` = rbx:: primitives: Arc vs shared_ptr, Signal, atomic, task_scheduler, intrusive — zero RBX deps, builds in <1s
- `reflection` = RBX::Reflection only (EnumDesc, Descriptor) — depends core
- `datamodel` = RBX:: Instance, DataModel, Workspace, Part — depends core+reflection (largest crate, but isolated)
- `script` = RBX::Lua, Script, bridging
- `rendering` = Ogre + G3D + RenderSettings
- `network` = RakNet + RBX net
- `audio` = FMOD
- `platform` = iOS bridge (RobloxView, controllers) — depends all, built last, ObjC interop only here

This splits old `app` into 3 (reflection, datamodel, script) for parallel omp by crate, and isolates heavy native deps (Ogre) so `core` stays fast.

### 2. xtask

Single `cargo xtask <cmd>` binary in `xtask/`:

- `cargo xtask ida-export` — calls MCP POST /mcp, writes `ida/export.json` + `ida/metadata.json`, asserts 85545 funcs
- `cargo xtask boost-audit` — parses export.json, reports 22k boost sites per crate, fails if any `boost::` left without `// was:`
- `cargo xtask gen-skeleton --crate datamodel` — generates `todo!("0xADDR")` stubs per IDA entry, ensures `cargo check` green
- `cargo xtask verify` — cargo check + clippy + `rg 'boost::'` + size_of asserts

No loose `scripts/*.py` — everything is `cargo` runnable, omp can invoke it.

### 3. Module mapping 1:1 still, but file-per-class

```
Client/App/v8datamodel/DataModel.cpp + .h -> crates/datamodel/src/data_model.rs
Client/App/include/reflection/Descriptor.h -> crates/reflection/src/descriptor.rs
Client/Base/rbx/TaskScheduler.cpp -> crates/core/src/task_scheduler.rs
Client/iOS/RobloxView.mm -> crates/platform/src/roblox_view.rs
Ogre/OgreEntity.cpp -> crates/rendering/src/ogre/entity.rs
```

Each Rust file keeps `// 0xADDR — mangled` + `#[doc(alias = "RBX::...")]`, but file is created only when class first appears in IDA — no pre-created empty `mod.rs` placeholders.

### 4. Boost enforcement

In `core/src/lib.rs`:

```rust
pub type SharedPtr<T> = Arc<T>; // was: boost::shared_ptr<T>
pub type WeakPtr<T> = Weak<T>;
```

Every crate `use core::SharedPtr;` — no direct `boost::` string in Rust except in `// was:` comments. `cargo xtask boost-audit` + `clippy` enforces.

### 5. Fidelity harness

`docs/FIDELITY.md` lists rules, but `xtask verify` enforces:
- every IDA ea appears as `todo!("0xADDR")` or impl with `// IDA 0xADDR`
- `rg -c 'todo!' crates/` count matches `ida/export.json` length minus impl count
- `cargo check` must pass, `clippy -D warnings` must pass

### 6. omp workflow

omp reads `AGENT.md` (new, not copied) + `DESIGN.md`. Per-crate subagents:

```
omp --cwd . --print "xtask gen-skeleton --crate reflection"
omp --cwd . "implement RBX::Reflection::EnumDesc per IDA 0x850c"
```

Sessions in `~/.omp/`, no manual IDA python — omp calls `cargo xtask ida-export` itself.

