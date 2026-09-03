//! network generated_netA_watchdog_54 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c6174 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c6174 — __ZN3RBX7Network13PhysicsSender3JobD0Ev
// type: void __fastcall(RBX::Network::PhysicsSender::Job *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::Job::~Job()")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender3JobD0Ev")]
pub fn stub_9c6174(job: crate::physics::SendJob) {
    // IDA 0x9c6174..0x9c61ca (D0): runs D2 (IDA 0x9c6568), then `operator
    // delete`. Taking `job` by value moves ownership here; Rust frees the
    // box on return, covering both steps (`SendJob` is stateless).
    crate::generated_netA_watchdog_57::stub_9c6568(job);
}
