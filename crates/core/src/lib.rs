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
pub mod boost_skeletons;
pub mod boost_skeletons2;
pub mod boost_core_a;
pub mod boost_core_b;
pub mod boost_core_c;
pub mod boost_core_d;
pub mod boost_core_e;
pub mod boost_core_f;
pub mod boost_core_g;
pub mod boost_core_h;
pub mod boost_core_i;
pub mod boost_core_j;
pub mod generated_core_d;
pub mod generated_core_e;
pub mod generated_core_f;
/// Re-export for `use rbx_core::SharedPtr`
pub use parking_lot::{Mutex as PLMutex, RwLock};
