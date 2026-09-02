#![allow(unused_attributes, dead_code, unused_variables, non_snake_case, non_camel_case_types, clippy::all)]
//! rbx_core — core crate second shard.
//! Provides SharedPtr and generated stubs. Excluded from workspace; see Cargo.toml exclude.

use std::sync::{Arc, Weak};

/// was: `boost::shared_ptr<T>` / `boost::intrusive_ptr<T>`
pub type SharedPtr<T> = Arc<T>;
/// was: `boost::weak_ptr<T>`
pub type WeakPtr<T> = Weak<T>;

pub use parking_lot::{Mutex as PLMutex, RwLock};

pub mod generated_core_watchdog_b;
pub mod generated_core_watchdog_c;
pub mod generated_core_watchdog_d;
pub mod generated_core_watchdog_e;
pub mod generated_core_watchdog_f;
pub mod generated_core_watchdog_g;
pub mod generated_core_watchdog_h;
pub mod generated_core_watchdog_i;
pub mod generated_core_watchdog_j;
pub mod generated_core_watchdog_k;
pub mod generated_core_watchdog_l;
pub mod generated_rbxcore_wdog_B;
pub mod generated_rbxcore_wdog_C;
pub mod generated_core_wdog7H;
pub mod generated_core_bg4;
pub mod generated_core_bg5;
pub mod generated_gap_wd;
pub mod generated_gap_wd25;
pub mod generated_core_bg6;
pub mod generated_core_bg7;
pub mod generated_watchdog_coreB;
pub mod generated_watchdog_coreA;
pub mod generated_core_bg8;
