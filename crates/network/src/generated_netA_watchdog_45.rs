//! network generated_netA_watchdog_45 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c35a0 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c35a0 — __ZN3RBX11IndexedTree23visitConstMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvNS3_4_mfi3mf2IvNS_7Network13PhysicsSenderEPN6RakNet9BitStreamEPKS2_EENS4_5list3INS4_5valueIPS9_EENSH_ISC_EENS3_3argILi1EEEEEEEEEvT0_
// type: int __fastcall(int, void (*)(void), int, int, int)
#[doc(alias = "void RBX::IndexedTree::visitConstMeAndChildren<RBX::Assembly,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::PhysicsSender,RakNet::BitStream *,RBX::Assembly const*>,boost::_bi::list3<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::_bi::value<RakNet::BitStream *>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::PhysicsSender,RakNet::BitStream *,RBX::Assembly const*>,boost::_bi::list3<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::_bi::value<RakNet::BitStream *>,boost::arg<1>>>)")]
#[doc(alias = "__ZN3RBX11IndexedTree23visitConstMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvNS3_4_mfi3mf2IvNS_7Network13PhysicsSenderEPN6RakNet9BitStreamEPKS2_EENS4_5list3INS4_5valueIPS9_EENSH_ISC_EENS3_3argILi1EEEEEEEEEvT0_")]
pub fn stub_9c35a0(
    tree: &crate::physics::AssemblyTree,
    visit: &mut dyn FnMut(&crate::physics::AssemblyTree),
) {
    // IDA 0x9c35c6..0x9c3662: invoke the `PhysicsSender::writeAssembly`
    // bind on self, then recurse into each child assembly. The `a3 & 1`
    // thunk select is a `boost::bind` dispatch detail; the
    // `indexOf(array[n]) == n` asserts (`IndexArray.h:103`) live in the
    // model.
    tree.visit_const_me_and_children(visit);
}
