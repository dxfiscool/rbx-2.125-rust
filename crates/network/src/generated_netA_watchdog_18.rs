//! network generated_netA_watchdog_18 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9a29f4 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9a29f4 — __ZN3RBX7Network24ReceiverStringDictionary7receiveISsEEbRN6RakNet9BitStreamERT_
// type: int __fastcall(RBX::Network::ReceiverStringDictionary *, RakNet::BitStream *this, std::string *)
#[doc(alias = "bool RBX::Network::ReceiverStringDictionary::receive<std::string>(RakNet::BitStream &,std::string &)")]
#[doc(alias = "__ZN3RBX7Network24ReceiverStringDictionary7receiveISsEEbRN6RakNet9BitStreamERT_")]
pub fn stub_9a29f4(
    dict: &mut crate::string_dictionary::ReceiverStringDictionary,
    stream: &mut crate::bitstream::BitStream,
    out: &mut String,
) -> bool {
    // IDA 0x9a2a08..0x9a2a4e: clear on 0, `get` below 0x80, `learn` above.
    dict.receive(stream, out)
}
