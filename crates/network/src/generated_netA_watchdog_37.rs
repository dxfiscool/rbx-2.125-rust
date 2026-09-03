//! network generated_netA_watchdog_37 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c1f50 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c1f50 — __ZN3RBX7Network13PhysicsSenderD2Ev
// type: void __fastcall(RBX::Network::PhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::~PhysicsSender()")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSenderD2Ev")]
pub fn stub_9c1f50(sender: &mut crate::physics::PhysicsSender) {
    // IDA 0x9c2016..0x9c2248: scheduler removes, job resets, signal
    // disconnect, touch-set and member teardown.
    sender.tear_down();
}
