//! network generated_netA_watchdog_46 — auto-generated, do not edit manually
//! Source: ida/export.json (85545 funcs, base 0x4000) EA 0x9c3664 | rbx_core::SharedPtr (not boost)
//! Batch: watchdog network+audio 120 stubs — 60 network EA-sorted gap-fill NOT in global

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x9c3664 — __ZN3RBX9Mechanism19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf3IvNS_7Network13PhysicsSenderEPNS_9PrimitiveEPN6RakNet9BitStreamEPNS7_10ReplicatorEEENS3_5list4INS3_5valueIPS8_EENS2_3argILi1EEENSI_ISD_EENSI_ISF_EEEEEEEEvT_PNS_8AssemblyE
// type: int __fastcall(int, int, int, int, int, int, RBX::Assembly *)
#[doc(alias = "void RBX::Mechanism::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::PhysicsSender,RBX::Primitive *,RakNet::BitStream *,RBX::Network::Replicator *>,boost::_bi::list4<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>,boost::_bi::value<RakNet::BitStream *>,boost::_bi::value<RBX::Network::Replicator *>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::PhysicsSender,RBX::Primitive *,RakNet::BitStream *,RBX::Network::Replicator *>,boost::_bi::list4<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>,boost::_bi::value<RakNet::BitStream *>,boost::_bi::value<RBX::Network::Replicator *>>>,RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX9Mechanism19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf3IvNS_7Network13PhysicsSenderEPNS_9PrimitiveEPN6RakNet9BitStreamEPNS7_10ReplicatorEEENS3_5list4INS3_5valueIPS8_EENS2_3argILi1EEENSI_ISD_EENSI_ISF_EEEEEEEEvT_PNS_8AssemblyE")]
pub fn stub_9c3664(tree: &crate::physics::MechanismTree, visit: &mut dyn FnMut(u32)) {
    // IDA 0x9c367c..0x9c3774: assert the assembly primitive (`p`,
    // `Assembly.h:203`), visit it through the assembly step, then recurse
    // into each child mechanism with the `indexOf` invariant.
    tree.visit_primitives(visit);
}
