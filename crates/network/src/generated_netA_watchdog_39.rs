//! network generated_netA_watchdog_39 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c28a8 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c28a8 — __ZN3RBX7Network13PhysicsSender17sendChildAssemblyEPN6RakNet9BitStreamEPKNS_8AssemblyE
// type: int __fastcall(RBX::Network::IdSerializer **this, RakNet::BitStream *, const RBX::Assembly *)
#[doc(alias = "RBX::Network::PhysicsSender::sendChildAssembly(RakNet::BitStream *,RBX::Assembly const*)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender17sendChildAssemblyEPN6RakNet9BitStreamEPKNS_8AssemblyE")]
pub fn stub_9c28a8(
    part_present: bool,
    use_try_serialize_id: bool,
    can_serialize_id: bool,
    stream: &mut crate::bitstream::BitStream,
    serialize_id: &mut dyn FnMut(&mut crate::bitstream::BitStream),
    sender: &mut crate::physics::PhysicsSender,
    packet: &crate::physics::AssemblyPacket<'_>,
) -> bool {
    // IDA 0x9c28bc..0x9c290c: `ReleaseAssert(part)` (PhysicsSender.cpp:328)
    // when `FLog::Asserts`; the null part never reaches the write below.
    debug_assert!(part_present, "part Client/Network/PhysicsSender.cpp line: 328");
    // IDA 0x9c2928..0x9c294c: without `DFFlag::PhysicsSenderUseTrySerializeId`
    // the id gate is skipped; otherwise a failed `canSerializeId` returns.
    if use_try_serialize_id && !can_serialize_id {
        return false;
    }
    // IDA 0x9c292e: presence bit for the child assembly.
    stream.write_bit(false);
    // IDA 0x9c2938: `IdSerializer::serializeId(*(this + 17), ...)`.
    serialize_id(stream);
    // IDA 0x9c2946: virtual body write (the `ErrorCompPhysicsSender`
    // override is selected engine-side).
    sender.write_assembly(stream, packet);
    true
}
