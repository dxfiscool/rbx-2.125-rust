//! network generated_netA_watchdog_44 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c30ac | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c30ac — __ZN3RBX7Network16CustomSerializer11writeVectorEbRKfS3_S3_RN6RakNet9BitStreamE
// type: unsigned int __fastcall(RBX::Network::CustomSerializer *this, __int32 *, float *, float *, RakNet::BitStream *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::CustomSerializer::writeVector(bool,float const&,float const&,float const&,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network16CustomSerializer11writeVectorEbRKfS3_S3_RN6RakNet9BitStreamE")]
pub fn stub_9c30ac(heavy: bool, x: f32, y: f32, z: f32, stream: &mut crate::bitstream::BitStream) {
    // IDA 0x9c30ac: `heavy` arrives in the `this` pointer slot (fastcall bool).
    crate::custom_serializer::write_vector(heavy, x, y, z, stream);
}
