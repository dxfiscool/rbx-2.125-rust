//! network generated_netA_watchdog_55 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c6214 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c6214 — __ZN3RBX7Network13PhysicsSender3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::PhysicsSender::Job *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::PhysicsSender::Job::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_9c6214(
    elapsed: f64,
    rate_hz: f32,
    ctx: &crate::physics::SleepContext,
) -> f64 {
    // IDA 0x9c6222..0x9c622a: stats rate at +496 into `computeStandardSleepTime`.
    crate::physics::SendJob::sleep_time(elapsed, rate_hz, ctx)
}
