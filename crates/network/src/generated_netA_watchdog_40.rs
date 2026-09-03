//! network generated_netA_watchdog_40 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c2950 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c2950 — __ZN3RBX7Network13PhysicsSender13writeAssemblyERN6RakNet9BitStreamEPKNS_8AssemblyE
// type: unsigned int __fastcall(RBX::Network::PhysicsSender *this, RakNet::BitStream *, const RBX::Assembly *)
#[doc(alias = "RBX::Network::PhysicsSender::writeAssembly(RakNet::BitStream &,RBX::Assembly const*)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender13writeAssemblyERN6RakNet9BitStreamEPKNS_8AssemblyE")]
pub fn stub_9c2950(
    sender: &mut crate::physics::PhysicsSender,
    stream: &mut crate::bitstream::BitStream,
    packet: &crate::physics::AssemblyPacket<'_>,
) {
    // IDA 0x9c2962..0x9c29bc: the original resolves the assembly primitive
    // (`getConstAssemblyPrimitive`), its PV (`getPV`), and the +124 flags
    // nibble; the caller passes them as `packet`.
    sender.write_assembly(stream, packet);
}
