//! network generated_netA_watchdog_32 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c0908 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c0908 — __ZN3RBX7Network13PhysicsSenderC2ERNS0_10ReplicatorE
// type: RBX::Network::PhysicsSender *__fastcall(RBX::Network::PhysicsSender *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::PhysicsSender::PhysicsSender(RBX::Network::Replicator &)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSenderC2ERNS0_10ReplicatorE")]
pub fn stub_9c0908() -> crate::physics::PhysicsSender {
    // IDA 0x9c0940..0x9c0a18: vtable, touch set, job slots, 0.05 interval,
    // and the +108/+112 flags.
    crate::physics::PhysicsSender::new()
}
