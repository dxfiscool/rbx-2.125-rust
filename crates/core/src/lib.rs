//! core — rbx:: primitives, zero RBX deps. Builds in <1s.
//! Mirrors Client/Base/rbx/* and boost replacements.

use std::sync::{Arc, Weak};

/// was: `boost::shared_ptr<T>` / `boost::intrusive_ptr<T>`
pub type SharedPtr<T> = Arc<T>;
/// was: `boost::weak_ptr<T>`
pub type WeakPtr<T> = Weak<T>;

pub mod signal;
pub mod task_scheduler;
pub mod intrusive;

/// Re-export for `use rbx_core::SharedPtr`
pub use parking_lot::{Mutex as PLMutex, RwLock};
