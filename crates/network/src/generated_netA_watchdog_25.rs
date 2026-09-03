//! network generated_netA_watchdog_25 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9bcba8 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9bcba8 — __ZN3RBX7Network15PhysicsReceiver15readMotorAnglesERN6RakNet9BitStreamERNS_12AssemblyItemE
// type: void __fastcall(RBX::Network::PhysicsReceiver *this, RakNet::BitStream *, RBX::Network::PhysicsReceiver **)
#[doc(alias = "RBX::Network::PhysicsReceiver::readMotorAngles(RakNet::BitStream &,RBX::AssemblyItem &)")]
#[doc(alias = "__ZN3RBX7Network15PhysicsReceiver15readMotorAnglesERN6RakNet9BitStreamERNS_12AssemblyItemE")]
pub fn stub_9bcba8(receiver: &crate::physics::PhysicsReceiver, stream: &mut crate::bitstream::BitStream, out: &mut Vec<crate::physics::CompactCFrame>) {
    receiver.read_motor_angles(stream, out);
}
