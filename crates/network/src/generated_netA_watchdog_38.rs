//! network generated_netA_watchdog_38 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c2504 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c2504 — __ZN3RBX7Network13PhysicsSender33sendChildPrimitiveCoordinateFrameEPNS_9PrimitiveEPN6RakNet9BitStreamEPNS0_10ReplicatorE
// type: unsigned int __fastcall(RBX::Network::PhysicsSender *this, RBX::Primitive *, RakNet::BitStream *, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::PhysicsSender::sendChildPrimitiveCoordinateFrame(RBX::Primitive *,RakNet::BitStream *,RBX::Network::Replicator *)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender33sendChildPrimitiveCoordinateFrameEPNS_9PrimitiveEPN6RakNet9BitStreamEPNS0_10ReplicatorE")]
pub fn stub_9c2504(
    sender: &crate::physics::PhysicsSender,
    stream: &mut crate::bitstream::BitStream,
    translation: [f32; 3],
    rotation: [f32; 4],
    streaming_enabled: bool,
    part_present: bool,
    replication_container: bool,
    try_serialize_id: &mut dyn FnMut(&mut crate::bitstream::BitStream) -> bool,
) -> bool {
    // IDA 0x9c2520..0x9c2562:
    // `ReleaseAssert(replicator->isStreamingEnabled())` (PhysicsSender.cpp:254).
    debug_assert!(
        streaming_enabled,
        "replicator->isStreamingEnabled() Client/Network/PhysicsSender.cpp line: 254"
    );
    // IDA 0x9c2566..0x9c256e: `PartInstance::fromPrimitive` null check.
    if !part_present {
        return false;
    }
    // IDA 0x9c2574..0x9c257a: `Replicator::isReplicationContainer`.
    if !replication_container {
        return false;
    }
    // IDA 0x9c2582..0x9c2588: `IdSerializer::trySerializeId` writes the id
    // and gates the body.
    if !try_serialize_id(stream) {
        return false;
    }
    // IDA 0x9c2594..0x9c259c: `writeTranslation` of the `getCoordinateFrame`
    // position (CF + 36) with the sender compression member.
    crate::physics::write_translation(stream, translation, sender.translation_compression);
    // IDA 0x9c25a8: tail `writeRotation`; its result is the return value,
    // surfaced here as sent.
    crate::physics::write_rotation(stream, rotation, sender.translation_compression);
    true
}
