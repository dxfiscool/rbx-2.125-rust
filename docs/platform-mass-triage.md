# Platform mass-file triage (473 files, 54,645 todos @ f8d62ee0)

Source: `#[doc(alias)]` namespace census of `crates/platform/src/*.rs`.
`other` = mostly `j___` import thunks + bare functions (G3D::Array,
rbx::signals internals, std). Counts are alias occurrences per file.

## Crate-wide namespace totals (alias occurrences)

| ns | count | rightful owner (hypothesis) |
|----|-------|------------------------------|
| other (j___ thunks, G3D::Array, bare fns) | 39146 | rendering (G3D), core (signals) |
| RBX (non-platform: Reflection::EnumDesc, Instance, DataModel…) | 13001 | reflection/datamodel/script |
| boost | 4816 | core (boost_skeletons pattern exists) |
| std | 3643 | core |
| ObjC (`-[`/`+[`) | 2952 | **platform (this crate)** |
| Ogre | 2950 | rendering |
| FMOD | 1909 | audio |
| rbx (signals/callable) | 1755 | core |
| RakNet | 611 | network |
| Platform (RBX::PlatformImpl etc.) | 200 | **platform (this crate)** |
| DataStructures/iOSSettingsService/RobloxView/CRenderSettingsItem | ~200 | platform/rendering edge |

Takeaway: only ~3.2k ObjC + ~0.2k Platform + a slice of RBX/Ogre-adjacent
aliases are truly platform work. The bulk duplicates other crates' EAs
(same pattern as generated.rs duplicating all 50 view_controllers EAs and
all 130 roblox_view EAs — delegate, don't re-implement).

## File ranking (todos | top namespaces)

- `generated.rs` 3709 | ObjC:2763 — platform's main file; 130 already
  delegated to roblox_view; remaining 2763 ObjC are PlaceLauncher/
  HomeViewController/MainViewController/Teleporter methods (do EA by EA,
  methods onto view_controllers.rs types, file last per brace history).
- `generated_platform_watchdog_m.rs` 200 | boost/spirit-heavy — low value,
  likely core-owned EAs; check core overlap before touching.
- `generated_plat_ba.rs` 151, `generated_next_ab.rs` 150,
  `generated_next_j.rs` 150 | FMOD/ASfxDsp — **audio-owned**; reassign.
- `generated_next_m/n/o/p/t…` 150 each | RBX::Reflection::EnumDesc —
  **reflection-owned** (ReflectionImpl already lands EnumDesc converters);
  delegate or reassign.
- `generated_next_d/e/f/h/i/l/q/r/s…` | RBX + rbx-signals + boost::function —
  split; rbx::signals → core, RBX::Instance/DataModel → datamodel.
- `generated_34/35*.rs`, `generated_plat_aw…bf.rs` (~150 each) |
  `j___G3D::Array`, signals thunks — **rendering/core-owned**; reassign.
- RakNet 611 dispersed — **network-owned**.

## Ownership hypotheses for Main

- boost::spirit::classic/property_tree/multi_index in platform files →
  script (parsers) or core, not platform.
- FMOD::*/ASfxDsp → audio.
- Ogre::*/G3D::* → rendering.
- RBX::Reflection::* → reflection; RBX::Instance/DataModel/Workspace →
  datamodel; RBX::Script/Lua → script.
- rbx::signals/boost::function/boost::_bi → core.
- Keep in platform: ObjC `-[`/`+[`, `RBX::Platform*`, `RobloxView`,
  `PlaceLauncher`, `*ViewController*`, `iOSSettingsService`,
  `Client/iOS/*` paths.

## Protocol note (from this session)

Per-EA regex codemod (`pub fn stub_X() -> ! { todo!("0xX…") }` →
grounded body) beat line-anchored edits for scattered duplicates;
standalone `rustc --crate-type lib` with `--extern rbx_core,parking_lot`
plus a `#[path]` wrapper verifies one file without the workspace gate;
never fight siblings' red deps — notify with file:line and keep working.
