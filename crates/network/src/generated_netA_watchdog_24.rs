//! network generated_netA_watchdog_24 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9bb4ec | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9bb4ec — __ZN3RBX7Network15PhysicsReceiver23receiveMechanismCFramesERN6RakNet9BitStreamEyRKNS_10RemoteTimeE
// type: void __fastcall(int, RBX::Network::Compressor *, unsigned int, unsigned int, int)
#[doc(alias = "RBX::Network::PhysicsReceiver::receiveMechanismCFrames(RakNet::BitStream &,unsigned long long,RBX::RemoteTime const&)")]
#[doc(alias = "__ZN3RBX7Network15PhysicsReceiver23receiveMechanismCFramesERN6RakNet9BitStreamEyRKNS_10RemoteTimeE")]
pub fn stub_9bb4ec(
    receiver: &crate::physics::PhysicsReceiver,
    stream: &mut crate::bitstream::BitStream,
    stamp_lo: u32,
    stamp_hi: u32,
    now_lo: u32,
    now_hi: u32,
) -> Option<crate::physics::CFrameSample> {
    // IDA 0x9bb568: the caller loops `receivePart` until it fails; each
    // iteration arrives here with the part's 64-bit stamp (part + 164).
    // IDA 0x9bb594..0x9bb682: stale parts log "Physics-in old packet"
    // (gated on the +94 verbose flag) and reset with no sample following.
    if !crate::physics::PhysicsReceiver::cframe_is_fresh(stamp_lo, stamp_hi, now_lo, now_hi) {
        return None;
    }
    // IDA 0x9bb68c..0x9bb6b4: `readTranslation` then `readRotation`.
    // Applying via `PartInstance::setPhysics` + `addInterpolationSample`
    // (IDA 0x9bb6c4..0x9bb6d4) stays engine-side.
    Some(receiver.read_cframe_sample(stream))
}
