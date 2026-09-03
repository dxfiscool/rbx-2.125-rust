//! `boost::detail::function` vtable machinery → Rust closures (AGENTS.md §4:
//! `boost::bind` / `function` → `Box<dyn Fn>` / closures).
//!
//! - `boost::function<Sig>` is a type-erased slot invoked through the
//!   `invoke*` shims below (`Box<dyn FnMut…>` at the call site).
//! - `functor_manager<F>::manage(in, out, op)` (IDA 0xa368fc): op 4
//!   (`get_functor_tag_type`) publishes `&typeid(F)` into the destination
//!   buffer and returns it; every other op runs the shared manager and
//!   yields its token.
//! - `*_invokerN<F>::invoke(buffer, args…)` (IDA 0xa36920): runs the stored
//!   closure through `listN::operator()` with the bound values plus the
//!   passed args.

#![allow(dead_code)]

/// `boost::detail::function::functor_manager_operation_type`
/// (`function_base.hpp`): clone = 0, move = 1, destroy = 2,
/// `check_functor_alignment` = 3, `get_functor_tag_type` = 4. The `GetTag`
/// numbering is grounded: `manage` writes `&typeid(F)` exactly when
/// `op == 4` (IDA 0xa368fc).
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FunctorOp {
    Clone = 0,
    Move = 1,
    Destroy = 2,
    CheckAlign = 3,
    GetTag = 4,
}

/// Whatever the shared `manager()` yields for non-tag ops (IDA 0xa368fc):
/// an engine-opaque vtable token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ManagerToken;

/// `functor_manager<F>::manage` outcome (IDA 0xa368fc): `op != 4` takes the
/// shared manager, `GetTag` publishes `&typeid(F)` (here, its name).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManageOutcome {
    Tag(&'static str),
    Manager(ManagerToken),
}

/// Small-object `functor_manager<F>::manage` (IDA 0xa368fc): `GetTag`
/// stores `&typeid(F)` at the destination (`dst[0]`, with `dst[1] = 0`)
/// and returns it; clone/move/destroy/align all run the shared manager.
pub fn manage_small(op: FunctorOp, type_name: &'static str) -> ManageOutcome {
    match op {
        FunctorOp::GetTag => ManageOutcome::Tag(type_name),
        _ => ManageOutcome::Manager(ManagerToken),
    }
}

/// `void_function_obj_invoker0<F>::invoke` (IDA 0xa36920): runs the stored
/// closure with its bound values (`listN::operator()` stays engine-side;
/// the caller supplies the composed closure).
pub fn invoke0(slot: &mut dyn FnMut()) {
    slot();
}

/// `void_function_obj_invoker1<F>::invoke` (IDA 0xa2f900).
pub fn invoke1<A>(slot: &mut dyn FnMut(A), a: A) {
    slot(a);
}

/// `void_function_obj_invoker2<F>::invoke` (IDA 0xaa6650).
pub fn invoke2<A, B>(slot: &mut dyn FnMut(A, B), a: A, b: B) {
    slot(a, b);
}

/// `void_function_obj_invoker4<F>::invoke` (IDA 0x9d2900).
pub fn invoke4<A, B, C, D>(slot: &mut dyn FnMut(A, B, C, D), a: A, b: B, c: C, d: D) {
    slot(a, b, c, d);
}

/// `function_obj_invoker0<F>::invoke` (IDA 0xa3bcf0).
pub fn invoke_ret0<R>(slot: &mut dyn FnMut() -> R) -> R {
    slot()
}

/// `function_obj_invoker1<F>::invoke` (IDA 0x9fa080).
pub fn invoke_ret1<R, A>(slot: &mut dyn FnMut(A) -> R, a: A) -> R {
    slot(a)
}

/// `function_obj_invoker2<F>::invoke` (IDA 0x9f26a0).
pub fn invoke_ret2<R, A, B>(slot: &mut dyn FnMut(A, B) -> R, a: A, b: B) -> R {
    slot(a, b)
}

/// `function_obj_invoker3<F>::invoke` (IDA 0x9f6694).
pub fn invoke_ret3<R, A, B, C>(slot: &mut dyn FnMut(A, B, C) -> R, a: A, b: B, c: C) -> R {
    slot(a, b, c)
}

/// `basic_vtableN<Sig>::assign_to<F>` (IDA 0x684260 / 0x9f26bc): installs
/// the functor into the slot — refcount bumps plus the functor-word copy
/// into the buffer stay engine-side — and returns 1. Small-object binds
/// always fit, so the verdict is constantly `true`; the functor itself
/// arrives as an already-composed Rust closure.
pub fn assign_to<T>(slot: &mut Option<Box<T>>, functor: T) -> bool {
    *slot = Some(Box::new(functor));
    true
    }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manage_tag_vs_manager() {
        // IDA 0xa368fc: only op 4 publishes the tag.
        assert_eq!(manage_small(FunctorOp::GetTag, "bind_t"), ManageOutcome::Tag("bind_t"));
        for op in [FunctorOp::Clone, FunctorOp::Move, FunctorOp::Destroy, FunctorOp::CheckAlign] {
            assert_eq!(manage_small(op, "bind_t"), ManageOutcome::Manager(ManagerToken));
        }
    }

    #[test]
    fn invokers_call_through() {

        let mut log = Vec::new();
        invoke0(&mut || log.push(0));
        invoke1(&mut |a: i32| log.push(a), 1);
        invoke2(&mut |a: i32, b: i32| log.push(a + b), 1, 2);
        invoke4(&mut |a: i32, b: i32, c: i32, d: i32| log.push(a + b + c + d), 1, 2, 3, 4);
        assert_eq!(log, vec![0, 1, 3, 10]);
        assert_eq!(invoke_ret0(&mut || 7), 7);
        assert_eq!(invoke_ret1(&mut |a: i32| a * 2, 21), 42);
        assert_eq!(invoke_ret2(&mut |a: i32, b: i32| a - b, 10, 4), 6);
        assert_eq!(invoke_ret3(&mut |a: i32, b: i32, c: i32| a + b + c, 1, 2, 3), 6);
    }

    #[test]
    fn bound_closure_end_to_end() {
        // A `bind(&Players::member, players, "a", "b", packet)`-shaped
        // closure: bound values captured, call-time args passed.
        let players = "players".to_owned();
        let (bound_a, bound_b) = ("a".to_owned(), "b".to_owned());
        let mut slot = {
            let players = players.clone();
            Box::new(move |pkt: i32| format!("{players}:{bound_a}:{bound_b}:{pkt}")) as Box<dyn FnMut(i32) -> String>
        };
        assert_eq!(manage_small(FunctorOp::GetTag, "bind_t"), ManageOutcome::Tag("bind_t"));
        assert_eq!(invoke_ret1(&mut *slot, 9), "players:a:b:9");
    }
    #[test]
    fn assign_installs_and_reports_fit() {
        // IDA 0x684260/0x9f26bc: functor words land in the buffer, `return 1`.
        fn inc(a: i32) -> i32 {
            a + 1
        }
        let mut slot: Option<Box<fn(i32) -> i32>> = None;
        assert!(assign_to(&mut slot, inc as fn(i32) -> i32));
        assert_eq!(slot.as_mut().map(|f| f(41)), Some(42));
    }
}

