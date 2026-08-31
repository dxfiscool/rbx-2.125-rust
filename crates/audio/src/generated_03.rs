//! audio generated_03 — next 150 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio|AudioBuffer case-sensitive (2541 distinct) — 2541 distinct already stubbed (2307 in lib.rs + 980 in generated.rs = 3287 stubs total), 0 remaining.
//! Batch: no new distinct EA; file kept for wiring / future export growth. SharedPtr = rbx_core::SharedPtr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
