//! network generated_netA_watchdog_21 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9add90 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9add90 — __ZN3RBX7Network4Item13writeItemTypeERN6RakNet9BitStreamENS1_8ItemTypeE
// type: unsigned int __fastcall(RakNet::BitStream *this, int)
#[doc(alias = "RBX::Network::Item::writeItemType(RakNet::BitStream &,RBX::Network::Item::ItemType)")]
#[doc(alias = "__ZN3RBX7Network4Item13writeItemTypeERN6RakNet9BitStreamENS1_8ItemTypeE")]
pub fn stub_9add90(stream: &mut crate::bitstream::BitStream, item_type: u8) {
    crate::item::write_item_type(stream, item_type);
}
