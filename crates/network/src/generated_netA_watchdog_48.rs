//! network generated_netA_watchdog_48 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c5830 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c5830 — __ZN3RBX7Network13PhysicsSender8TouchJobD1Ev
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::~TouchJob()")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender8TouchJobD1Ev")]
pub fn stub_9c5830(_job: crate::physics::TouchJob) {
    // IDA 0x9c5830..0x9c5834 (D1): tail-calls D2 (IDA 0x9c5e38). `TouchJob`
    // is stateless, so dropping `job` covers the vtable reset, the weak-ref
    // release at +123, the shared-count dtor at +484, and the base `Job`
    // dtor with no fields to release.
}
