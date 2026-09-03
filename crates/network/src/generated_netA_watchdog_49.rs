//! network generated_netA_watchdog_49 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c583c | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c583c — __ZN3RBX7Network13PhysicsSender8TouchJobD0Ev
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::~TouchJob()")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender8TouchJobD0Ev")]
pub fn stub_9c583c(job: crate::physics::TouchJob) {
    // IDA 0x9c583c..0x9c5892 (D0): runs D2 (IDA 0x9c5e38), then `operator
    // delete`. Taking `job` by value moves ownership here; Rust frees the
    // box on return, covering both steps (`TouchJob` is stateless).
    crate::generated_netA_watchdog_52::stub_9c5e38(job);
}
