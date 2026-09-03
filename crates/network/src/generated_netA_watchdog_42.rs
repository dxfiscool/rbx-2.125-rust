//! network generated_netA_watchdog_42 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c2aa4 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c2aa4 — __ZN3RBX7Network13PhysicsSender13writeVelocityERN6RakNet9BitStreamERKNS_8VelocityE
// type: _DWORD __fastcall(RBX::Network::PhysicsSender *__hidden this, RakNet::BitStream *, const RBX::Velocity *)
#[doc(alias = "RBX::Network::PhysicsSender::writeVelocity(RakNet::BitStream &,RBX::Velocity const&)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender13writeVelocityERN6RakNet9BitStreamERKNS_8VelocityE")]
pub fn stub_9c2aa4(sender: &crate::physics::PhysicsSender, stream: &mut crate::bitstream::BitStream, velocity: &crate::physics::Velocity) {
    sender.write_velocity(stream, velocity);
}
