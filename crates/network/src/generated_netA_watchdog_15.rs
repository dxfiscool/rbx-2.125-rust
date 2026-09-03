//! network generated_netA_watchdog_15 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9a2648 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9a2648 — __ZN3RBX7Network31SharedStringProtectedDictionary17deserializeStringERSsRN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::SharedStringProtectedDictionary *this, std::string *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::SharedStringProtectedDictionary::deserializeString(std::string &,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network31SharedStringProtectedDictionary17deserializeStringERSsRN6RakNet9BitStreamE")]
pub fn stub_9a2648(
    dict: &mut crate::string_dictionary::SharedStringProtectedDictionary,
    out: &mut String,
    stream: &mut crate::bitstream::BitStream,
) -> bool {
    // IDA 0x9a265a: tail-calls `ReceiverStringDictionary::receive` on +540.
    dict.deserialize_string(out, stream)
}
