# BOOST → Rust (from scratch)

No `boost` crate. Every `boost::` replaced. Enforced by `cargo xtask boost-audit`.

| C++ | Rust | Notes |
|-----|------|-------|
| `boost::shared_ptr<T>` / `intrusive_ptr<T>` | `rbx_core::SharedPtr<T>` = `Arc<T>` | `Arc<Mutex<T>>` if mutable |
| `boost::weak_ptr<T>` | `Weak<T>` | |
| `scoped_ptr<T>` | `Box<T>` | |
| `signals::signal<Sig>` | `rbx_core::signal::Signal<Sig>` | `Mutex<Vec<Weak<dyn Fn>>>` |
| `function<Sig>` | `Box<dyn Fn(..) + Send + Sync>` | |
| `bind` / `_bi::bind_t` / `mfi::mf*` | closures `move \|\| {}` | `bind(&X::foo, this, _1)` → `let t=Arc::clone(&this); move \|a\| t.foo(a)` |
| `thread` / `thread_data` | `std::thread::spawn` | |
| `mutex` / `recursive_mutex` | `parking_lot::Mutex` | |
| `unordered_map/set` | `HashMap/Set` | `std::map` → `BTreeMap` |
| `asio` | `std::net` / `tokio` | prefer std |
| `exception` / `system_error` | `thiserror` / `anyhow` | |
| `noncopyable` | `!Clone` (move) | |
| `optional<T>` | `Option<T>` | |
| `any` / `variant` | `Box<dyn Any>` / `enum` | |

Mangled `N5boost` → Rust sig already uses `SharedPtr` with `// was: boost::shared_ptr<RBX::Game>`.

Audit: `cargo xtask boost-audit` → 22,806 hits, per-crate counts.
