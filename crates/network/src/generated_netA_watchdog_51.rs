//! network generated_netA_watchdog_51 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c58fc | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c58fc — __ZN3RBX7Network13PhysicsSender8TouchJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender8TouchJob5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_9c58fc(
    gate: &crate::physics::SendGate,
    replicator_present: bool,
    job_pending: bool,
    error: f64,
    rate_hz: f32,
) -> crate::physics::StandardError {
    // IDA 0x9c5958..0x9c59ca: `canSendPacket` + replicator/job gates select
    // the zero shape or `computeStandardError` (IDA 0x9c59e4).
    crate::physics::TouchJob::error(gate, replicator_present, job_pending, error, rate_hz)
}
