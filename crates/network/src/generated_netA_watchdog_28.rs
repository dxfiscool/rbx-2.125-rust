//! network generated_netA_watchdog_28 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9be2ec | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9be2ec — __ZN3RBX7Network15PhysicsReceiver17readCompactCFrameERN6RakNet9BitStreamERNS_13CompactCFrameE
// type: int __fastcall(RBX::Network::PhysicsReceiver *this, RakNet::BitStream *, RBX::CompactCFrame *)
#[doc(alias = "RBX::Network::PhysicsReceiver::readCompactCFrame(RakNet::BitStream &,RBX::CompactCFrame &)")]
#[doc(alias = "__ZN3RBX7Network15PhysicsReceiver17readCompactCFrameERN6RakNet9BitStreamERNS_13CompactCFrameE")]
pub fn stub_9be2ec(receiver: &crate::physics::PhysicsReceiver, stream: &mut crate::bitstream::BitStream, frame: &mut crate::physics::CompactCFrame) {
    // IDA 0x9be620: the original returns the asserts-flag echo; no caller uses it.
    receiver.read_compact_cframe(stream, frame);
}
