# ARCHITECTURE — from scratch

See AGENT.md §1 for DAG.

## Crate DAG

```
rbx-core (0 deps)
  ↑   |  -> rbx-reflection -> rbx-datamodel -> rbx-script -> rbx-platform
  |      \-> rbx-rendering ---------------------> rbx-platform
  |      \-> rbx-network ----------------------> rbx-platform
  |      \-> rbx-audio ------------------------> rbx-platform
```

- `rbx-core` builds <1s, no RBX types — holds `SharedPtr`, `Signal`, `TaskScheduler`
- `rbx-reflection` only `RBX::Reflection`
- `rbx-datamodel` is largest, isolated for parallel work
- `rbx-platform` built last, only crate with ObjC (`objc2` optional)

## File mapping

One Rust file per C++ class/header+cpp pair, merged:

```
Client/App/v8datamodel/DataModel.h + .cpp -> crates/datamodel/src/data_model.rs
Client/Base/rbx/TaskScheduler.cpp -> crates/core/src/task_scheduler.rs
Ogre/OgreEntity.h -> crates/rendering/src/ogre/mod.rs (or ogre/entity.rs when split)
```

File created only when class first seen in IDA demangled names.

## xtask

`cargo xtask <cmd>` — `xtask/src/main.rs` implements ida-export, boost-audit, verify, gen-skeleton.

## omp

Reads AGENT.md + this file. One subagent per crate.

