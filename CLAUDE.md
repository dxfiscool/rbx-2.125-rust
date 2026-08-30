# AGENT.md — rbx-2.125-rust (from scratch)

> Read this first. Every session, every subagent. Ground truth. No copy from older projects — this is fresh.

## 0. Artifacts

| What | Where |
|------|-------|
| IPA | `/home/zzzzzzz/Downloads/ROBLOX 2.125.39335.ipa` (63M, md5 d4a442d7eb8d7e5063c6ca62f92e6667) |
| Mach-O | `/home/zzzzzzz/Downloads/ROBLOX 2.125.39335/Payload/robloxmobile.app/robloxmobile` (54M, armv7, PIE) |
| IDA DB | `.../robloxmobile.i64` (496M, base 0x4000, size 0x13a8efc, sha256 00e764c7...) — not checked in |
| IDA MCP | `ida` on `127.0.0.1:13337` — `POST http://127.0.0.1:13337/mcp` (tools/call, resources/read) |
| Workspace | `/home/zzzzzzz/rbx-2.125-rust/` — Cargo workspace, 8 crates + xtask |

## 1. New architecture (DAG)

```
core (0) -> reflection -> datamodel -> script -> platform
            \-> rendering -----> platform
            \-> network ------> platform
            \-> audio --------> platform
```

- `core` = `rbx_core` (SharedPtr=Arc, Signal, atomic, TaskScheduler) — fast, no RBX deps
- `reflection` = `RBX::Reflection`
- `datamodel` = `RBX::Instance`/`DataModel`/`Workspace`
- `script` = Lua
- `rendering` = Ogre/G3D
- `network` = RakNet
- `audio` = FMOD
- `platform` = iOS (RobloxView, controllers) — ObjC only here, depends all

Old monolith `app` (57k funcs) split into `reflection`+`datamodel`+`script` for parallel omp.

## 2. Workflow — skeleton first (NON-NEGOTIABLE)

1. `cargo xtask ida-export` → `ida/export.json` (85,545 funcs) — ground truth, not .i64
2. `cargo xtask gen-skeleton --crate <name>` — generate `todo!("0xADDR")` stubs per IDA ea, keep `#[doc(alias = "RBX::...")]` + `// 0xADDR — mangled`
3. `cargo check --workspace` must be green before any impl
4. Impl: one fn at a time from `decompile(ea)`+`disasm(ea)` → Rust, preserve control flow, `// IDA 0xADDR`
5. `cargo xtask verify` (check + clippy -D warnings) before push — CI enforces

Parallelize by crate, not by guessing.

## 3. Naming — keep RBX::

- `RBX::Reflection::EnumDesc<T>` → `rbx_reflection::EnumDesc<T>` with `#[doc(alias = "RBX::Reflection::EnumDesc")]` + `// 0x850c — __ZN...`
- Rust fn `snake_case`, original preserved as alias. `rg` must find either form.
- `Ogre::Entity` → `rbx_rendering::ogre::Entity`
- ObjC `-[GameViewController viewDidLoad]` → `rbx_platform::GameViewController::view_did_load` with `#[doc = "-[GameViewController viewDidLoad]"]`

## 4. Boost — do not shim

No `boost` crate. Every `boost::` maps to Rust:

- `boost::shared_ptr<T>` / `intrusive_ptr<T>` → `rbx_core::SharedPtr<T>` (`Arc<T>`)
- `boost::weak_ptr<T>` → `Weak<T>`
- `boost::signals` / `rbx::signals::signal` → `rbx_core::signal::Signal<T>`
- `boost::thread` / `thread_data` → `std::thread`
- `boost::bind` / `function` / `_bi::bind_t` / `mfi::mf*` → `Box<dyn Fn>` / closures
- `boost::unordered_map` → `HashMap`, `boost::asio` → `tokio` or `std::net`
- `boost::exception` → `thiserror`/`anyhow`

Use `cargo xtask boost-audit` to report 22,806 boost funcs; skeleton `// was: boost::shared_ptr<RBX::Game>`.

## 5. Fidelity

Rust but still a decompilation. 1:1 outputs, code paths, preserve bugs initially with `// BUG: original at 0xADDR`. See `docs/FIDELITY.md`. `cargo xtask verify` enforces.

## 6. IDA MCP

```python
import urllib.request, json
def mcp_call(tool, args):
    data=json.dumps({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":tool,"arguments":args}}).encode()
    req=urllib.request.Request("http://127.0.0.1:13337/mcp", data=data, headers={"Content-Type":"application/json","Accept":"application/json"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.loads(r.read())["result"]["structuredContent"]
def py(code): return mcp_call("py_eval", {"code": code})
```

Snippets:

```python
py("import idautils, ida_funcs, idc, json; funcs=list(idautils.Functions()); open('/tmp/ida_export.json','w').write(json.dumps([{'ea':hex(ea),'mangled':ida_funcs.get_func_name(ea),'demangled':idc.demangle_name(ida_funcs.get_func_name(ea),0),'type':idc.get_type(ea)} for ea in funcs]))")
```

Export via `cargo xtask ida-export`, not manual.

## 7. Verification

- Skeleton: `cargo check` green, `todo!("0xADDR name")` not `{}`
- Impl: per-fn test vs IDA pseudo, `cargo miri` for `unsafe`, `clippy` clean
- Do not edit IDA DB. Do not check in `.i64`.

---
*From scratch 2026-08-30. 85,545 funcs, 22,806 boost, armv7 Mach-O. omp + GitHub Issues enabled.*
