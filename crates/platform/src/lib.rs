//! platform — iOS bridge, RobloxView, view controllers
//! Mirrors Client/iOS/* — ObjC interop only in this crate.
//! `#[doc = "-[GameViewController viewDidLoad]"]` + snake_case fns

pub mod roblox_view;
pub mod view_controllers;
pub mod app_delegate;
pub mod generated;
