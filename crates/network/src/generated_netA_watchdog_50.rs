//! network generated_netA_watchdog_50 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c58dc | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c58dc — __ZN3RBX7Network13PhysicsSender8TouchJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender8TouchJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_9c58dc(
    elapsed: f64,
    rate_hz: f32,
    ctx: &crate::physics::SleepContext,
) -> f64 {
    // IDA 0x9c58ea..0x9c58f2: stats rate at +496 into `computeStandardSleepTime`.
    crate::physics::TouchJob::sleep_time(elapsed, rate_hz, ctx)
}
