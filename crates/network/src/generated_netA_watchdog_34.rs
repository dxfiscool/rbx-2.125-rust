//! network generated_netA_watchdog_34 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c0ab8 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c0ab8 — __ZN3RBX7Network13PhysicsSender14connectTouchesEv
// type: void __fastcall(RBX::Network::PhysicsSender *this)
#[doc(alias = "RBX::Network::PhysicsSender::connectTouches(void)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender14connectTouchesEv")]
pub fn stub_9c0ab8(sender: &mut crate::physics::PhysicsSender) {
    // IDA 0x9c0bd4..0x9c0c5e: insert the `onTouchStep` slot into the
    // workspace touch signal, keeping the `scoped_connection` at +44.
    sender.connect_touches();
}
