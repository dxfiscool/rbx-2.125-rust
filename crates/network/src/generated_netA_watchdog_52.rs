//! network generated_netA_watchdog_52 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c5e38 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c5e38 — __ZN3RBX7Network13PhysicsSender8TouchJobD2Ev
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::~TouchJob()")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender8TouchJobD2Ev")]
pub fn stub_9c5e38(_job: crate::physics::TouchJob) {
    // IDA 0x9c5e70..0x9c5f24 (D2): vtable reset, weak-ref release at +123
    // (IDA 0x9c5e7e..0x9c5efa), shared-count dtor at +484 (IDA 0x9c5f18),
    // base `Job` dtor (IDA 0x9c5f24). `TouchJob` is stateless, so dropping
    // `job` is the whole effect.
}
