//! network generated_netA_watchdog_56 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c6234 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c6234 — __ZN3RBX7Network13PhysicsSender3Job5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::PhysicsSender::Job *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::PhysicsSender::Job::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender3Job5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_9c6234(
    gate: &crate::physics::SendGate,
    error: f64,
    rate_hz: f32,
) -> crate::physics::StandardError {
    // IDA 0x9c6248..0x9c627e: `canSendPacket` selects the zero shape or
    // `computeStandardError`.
    crate::physics::SendJob::error(gate, error, rate_hz)
}
