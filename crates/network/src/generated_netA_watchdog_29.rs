//! network generated_netA_watchdog_29 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9be624 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9be624 — __ZN3RBX7Network15PhysicsReceiver10setPhysicsERKNS_13MechanismItemERKNS_10RemoteTimeEj
// type: void __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::PhysicsReceiver::setPhysics(RBX::MechanismItem const&,RBX::RemoteTime const&,unsigned int)")]
#[doc(alias = "__ZN3RBX7Network15PhysicsReceiver10setPhysicsERKNS_13MechanismItemERKNS_10RemoteTimeEj")]
pub fn stub_9be624(
    receiver: &crate::physics::PhysicsReceiver,
    items: &[crate::physics::MechanismItemSample<'_>],
    first_flag_28: bool,
) -> Vec<crate::physics::AppliedItem> {
    // IDA 0x9be686..0x9be910: per-item filter/root/grounded gates (verbose
    // logging only) with `Assembly::setPhysics` application for ungrounded
    // roots; the world writes (`setPhysics`, `addInterpolationSample`,
    // IDA 0x9be8c8..0x9be908) stay engine-side.
    receiver.set_physics_batch(items, first_flag_28)
}
