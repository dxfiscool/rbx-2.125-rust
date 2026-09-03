//! network generated_netA_watchdog_60 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c72a8 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c72a8 — __ZN3RBX7Network6Server14getClientCountEv
// type: _DWORD __fastcall(RBX::Network::Server *__hidden this)
#[doc(alias = "RBX::Network::Server::getClientCount(void)")]
#[doc(alias = "__ZN3RBX7Network6Server14getClientCountEv")]
pub fn stub_9c72a8(server: &crate::server::Server) -> usize {
    server.client_count()
}
