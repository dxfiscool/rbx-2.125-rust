//! core shard pa — 100 core stubs EA-sorted, 0xc334e4..0xf28914 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered, global-deduped).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::resizeShadowProjection(boost::scoped_array<unsigned char> &,unsigned int,unsigned int)")]
#[doc(alias = "__ZN3RBXL22resizeShadowProjectionERN5boost12scoped_arrayIhEEjj")]
// 0xc334e4 — __ZN3RBXL22resizeShadowProjectionERN5boost12scoped_arrayIhEEjj
// type: 
pub fn stub_0xc334e4() {
    // IDA 0xc334e4: boost template instantiation (mangled-only context). Per Boost map (AGENTS.md section 4) — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LightObject,bool>,boost::_bi::list2<boost::_bi::value<RBX::LightObject*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX11LightObjectEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")]
// 0xc336a0 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX11LightObjectEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
// type: 
pub fn stub_0xc336a0() {
    // IDA 0xc336a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LightObject,bool>,boost::_bi::list2<boost::_bi::value<RBX::LightObject*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX11LightObjectEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")]
// 0xc336fc — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX11LightObjectEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
// type: 
pub fn stub_0xc336fc() {
    // IDA 0xc336fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LightObject,bool>,boost::_bi::list2<boost::_bi::value<RBX::LightObject*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX11LightObjectEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")]
// 0xc33804 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX11LightObjectEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
// type: 
pub fn stub_0xc33804() {
    // IDA 0xc33804: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LightObject,bool>,boost::_bi::list2<boost::_bi::value<RBX::LightObject*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX11LightObjectEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")]
// 0xc3381c — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX11LightObjectEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
// type: 
pub fn stub_0xc3381c() {
    // IDA 0xc3381c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::LightObject>,boost::_bi::list1<boost::_bi::value<RBX::LightObject*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11LightObjectEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")]
// 0xc339d0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11LightObjectEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
// type: 
pub fn stub_0xc339d0() {
    // IDA 0xc339d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::LightObject>,boost::_bi::list1<boost::_bi::value<RBX::LightObject*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11LightObjectEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")]
// 0xc33a2c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11LightObjectEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
// type: 
pub fn stub_0xc33a2c() {
    // IDA 0xc33a2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::LightObject>,boost::_bi::list1<boost::_bi::value<RBX::LightObject*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11LightObjectEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
// 0xc33b34 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11LightObjectEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// type: 
pub fn stub_0xc33b34() {
    // IDA 0xc33b34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::LightObject>,boost::_bi::list1<boost::_bi::value<RBX::LightObject*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11LightObjectEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
// 0xc33b4c — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11LightObjectEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// type: 
pub fn stub_0xc33b4c() {
    // IDA 0xc33b4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::scoped_ptr<RBX::LightShadowMap>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX14LightShadowMapEED2Ev")]
// 0xc3491c — __ZN5boost10scoped_ptrIN3RBX14LightShadowMapEED2Ev
// type: 
pub fn stub_0xc3491c() {
    // IDA 0xc3491c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AdornBillboarder::setTexture(int,rbx_core::SharedPtr<RBX::TextureProxyBase> const&)")]
#[doc(alias = "__ZN3RBX16AdornBillboarder10setTextureEiRKN5boost10shared_ptrINS_16TextureProxyBaseEEE")]
// 0xeab30c — __ZN3RBX16AdornBillboarder10setTextureEiRKN5boost10shared_ptrINS_16TextureProxyBaseEEE
// type: 
pub fn stub_0xeab30c() {
    // IDA 0xeab30c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AdornBillboarder::getTextureSize(rbx_core::SharedPtr<RBX::TextureProxyBase> const&)const")]
#[doc(alias = "__ZNK3RBX16AdornBillboarder14getTextureSizeERKN5boost10shared_ptrINS_16TextureProxyBaseEEE")]
// 0xeab31c — __ZNK3RBX16AdornBillboarder14getTextureSizeERKN5boost10shared_ptrINS_16TextureProxyBaseEEE
// type: 
pub fn stub_0xeab31c() {
    // IDA 0xeab31c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_$shim")]
// 0xf1f42c — __ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1f42c() {
    // IDA 0xf1f42c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf1fbb8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: 
pub fn stub_0xf1fbb8() {
    // IDA 0xf1fbb8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf1fbc4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: 
pub fn stub_0xf1fbc4() {
    // IDA 0xf1fbc4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf1fc18 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: 
pub fn stub_0xf1fc18() {
    // IDA 0xf1fc18: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf1fc24 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: 
pub fn stub_0xf1fc24() {
    // IDA 0xf1fc24: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsbENS3_5list3INS3_5valueIS8_EENSC_ISsEENSC_IbEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf1fc30 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsbENS3_5list3INS3_5valueIS8_EENSC_ISsEENSC_IbEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: 
pub fn stub_0xf1fc30() {
    // IDA 0xf1fc30: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsSsbbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i$shim")]
// 0xf1fc6c — __ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsSsbbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i$shim
// type: int __fastcall(std::string *)
pub fn stub_0xf1fc6c() {
    // IDA 0xf1fc6c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i$shim")]
// 0xf1fc84 — __ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i$shim
// type: 
pub fn stub_0xf1fc84() {
    // IDA 0xf1fc84: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm$shim")]
// 0xf1fcb4 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm$shim
// type: 
pub fn stub_0xf1fcb4() {
    // IDA 0xf1fcb4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
// 0xf1fdd4 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim
// type: 
pub fn stub_0xf1fdd4() {
    // IDA 0xf1fdd4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
// 0xf1fde0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim
// type: 
pub fn stub_0xf1fde0() {
    // IDA 0xf1fde0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_$shim")]
// 0xf1fdec — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_$shim
// type: 
pub fn stub_0xf1fdec() {
    // IDA 0xf1fdec: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8ISteppedERKNS4_7SteppedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_$shim")]
// 0xf20080 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8ISteppedERKNS4_7SteppedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20080() {
    // IDA 0xf20080: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_$shim")]
// 0xf200bc — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_$shim
// type: int(void)
pub fn stub_0xf200bc() {
    // IDA 0xf200bc: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE15_M_erase_at_endEPS4_$shim")]
// 0xf200c8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE15_M_erase_at_endEPS4_$shim
// type: int(void)
pub fn stub_0xf200c8() {
    // IDA 0xf200c8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE16_M_push_back_auxERKS7_$shim")]
// 0xf200e0 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE16_M_push_back_auxERKS7_$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf200e0() {
    // IDA 0xf200e0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_reallocate_mapEmb$shim")]
// 0xf200ec — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_reallocate_mapEmb$shim
// type: int(void)
pub fn stub_0xf200ec() {
    // IDA 0xf200ec: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf20434 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int(void)
pub fn stub_0xf20434() {
    // IDA 0xf20434: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf20440 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int(void)
pub fn stub_0xf20440() {
    // IDA 0xf20440: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEaSERKS7_$shim")]
// 0xf20494 — __ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEaSERKS7_$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20494() {
    // IDA 0xf20494: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS0_5list1INS0_5valueIPS6_EEEEEclEv$shim")]
// 0xf20674 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS0_5list1INS0_5valueIPS6_EEEEEclEv$shim
// type: int(void)
pub fn stub_0xf20674() {
    // IDA 0xf20674: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim")]
// 0xf2068c — __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim
// type: int(void)
pub fn stub_0xf2068c() {
    // IDA 0xf2068c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf23614 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf23614() {
    // IDA 0xf23614: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_7Network11ChatMessageEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRKT_$shim")]
// 0xf24574 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_7Network11ChatMessageEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRKT_$shim
// type: int()
pub fn stub_0xf24574() {
    // IDA 0xf24574: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX16SecurePlayerGameEEEPT_")]
// 0xf26a14 — j___ZN5boost6detail12shared_countC2IN3RBX16SecurePlayerGameEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf26a14() {
    // IDA 0xf26a14: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::UnsecuredStudioGame>(RBX::UnsecuredStudioGame *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX19UnsecuredStudioGameEEEPT_")]
// 0xf26a24 — j___ZN5boost6detail12shared_countC2IN3RBX19UnsecuredStudioGameEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf26a24() {
    // IDA 0xf26a24: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "RBX::TaskScheduler::removeBlocking(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,boost::function<void ()(void)>)")]
#[doc(alias = "j___ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEENS1_8functionIFvvEEE")]
// 0xf26dd4 — j___ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEENS1_8functionIFvvEEE
// type: void __fastcall(int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf26dd4() {
    // IDA 0xf26dd4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::Tasks::Sequence>::reset<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX5Tasks8SequenceEE5resetIS3_EEvPT_")]
// 0xf26fe4 — j___ZN5boost10shared_ptrIN3RBX5Tasks8SequenceEE5resetIS3_EEvPT_
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf26fe4() {
    // IDA 0xf26fe4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ViewBase>::reset(void)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX8ViewBaseEE5resetEv")]
// 0xf27014 — j___ZN5boost10shared_ptrIN3RBX8ViewBaseEE5resetEv
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf27014() {
    // IDA 0xf27014: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// 0xf27074 — j___ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
pub fn stub_0xf27074() {
    // IDA 0xf27074: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>::operator()(void)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")]
// 0xf27184 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// type: int()
pub fn stub_0xf27184() {
    // IDA 0xf27184: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tasks::ExclusiveSequence>(RBX::Tasks::ExclusiveSequence *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX5Tasks17ExclusiveSequenceEEEPT_")]
// 0xf271d4 — j___ZN5boost6detail12shared_countC2IN3RBX5Tasks17ExclusiveSequenceEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf271d4() {
    // IDA 0xf271d4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX5Tasks8SequenceEEEPT_")]
// 0xf271e4 — j___ZN5boost6detail12shared_countC2IN3RBX5Tasks8SequenceEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf271e4() {
    // IDA 0xf271e4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ViewBase>(RBX::ViewBase *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_")]
// 0xf271f4 — j___ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf271f4() {
    // IDA 0xf271f4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")]
// 0xf274d4 — j___ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
pub fn stub_0xf274d4() {
    // IDA 0xf274d4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)")]
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_")]
// 0xf274e4 — j___ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf274e4() {
    // IDA 0xf274e4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_")]
// 0xf274f4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf274f4() {
    // IDA 0xf274f4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_")]
// 0xf27504 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf27504() {
    // IDA 0xf27504: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::assign_to_own(boost::function3<void,bool,void *,RBX::UIEvent> const&)")]
#[doc(alias = "j___ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_")]
// 0xf27514 — j___ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_
// type: int()
pub fn stub_0xf27514() {
    // IDA 0xf27514: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::clear(void)")]
#[doc(alias = "j___ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv")]
// 0xf27524 — j___ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv
// type: int()
pub fn stub_0xf27524() {
    // IDA 0xf27524: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::operator()(bool,void *,RBX::UIEvent)const")]
#[doc(alias = "j___ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_")]
// 0xf27534 — j___ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
pub fn stub_0xf27534() {
    // IDA 0xf27534: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv")]
// 0xf27554 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf27554() {
    // IDA 0xf27554: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6insertEPNS8_4slotE")]
// 0xf27564 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf27564() {
    // IDA 0xf27564: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6removeEPNS8_4slotE")]
// 0xf27574 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf27574() {
    // IDA 0xf27574: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_")]
// 0xf27584 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
pub fn stub_0xf27584() {
    // IDA 0xf27584: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_")]
// 0xf275e4 — j___ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf275e4() {
    // IDA 0xf275e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*)")]
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_")]
// 0xf275f4 — j___ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf275f4() {
    // IDA 0xf275f4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_")]
// 0xf27624 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf27624() {
    // IDA 0xf27624: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX7TextBoxEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0xf27654 — j___ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX7TextBoxEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int, int, sp_counted_base **), const shared_count **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf27654() {
    // IDA 0xf27654: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>> const&)")]
#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_")]
// 0xf27664 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_
// type: int()
pub fn stub_0xf27664() {
    // IDA 0xf27664: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::clear(void)")]
#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv")]
// 0xf27674 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv
// type: 
pub fn stub_0xf27674() {
    // IDA 0xf27674: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::operator()(rbx_core::SharedPtr<RBX::TextBox>)const")]
#[doc(alias = "j___ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_")]
// 0xf276a4 — j___ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_
// type: int()
pub fn stub_0xf276a4() {
    // IDA 0xf276a4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox>&&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_")]
// 0xf276d4 — j___ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_
// type: _DWORD *__fastcall(_DWORD *, __int64 *)
pub fn stub_0xf276d4() {
    // IDA 0xf276d4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_")]
// 0xf276e4 — j___ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_
// type: int __fastcall(int, const shared_count *)
pub fn stub_0xf276e4() {
    // IDA 0xf276e4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::connect<boost::function<void ()(RBX::StandardOutMessage const&)>>(boost::function<void ()(RBX::StandardOutMessage const&)> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE7connectIN5boost8functionIS6_EEEENS0_10connectionERKT_")]
// 0xf27784 — j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE7connectIN5boost8functionIS6_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
pub fn stub_0xf27784() {
    // IDA 0xf27784: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*>(boost::function<void ()(RBX::StandardOutMessage const&)> const&,rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*)")]
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_EC2IPS8_EERKSC_T_")]
// 0xf27794 — j___ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_EC2IPS8_EERKSC_T_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf27794() {
    // IDA 0xf27794: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSEPSA_")]
// 0xf277a4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSEPSA_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf277a4() {
    // IDA 0xf277a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSERKSB_")]
// 0xf277b4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSERKSB_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf277b4() {
    // IDA 0xf277b4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::assign_to_own(boost::function1<void,RBX::StandardOutMessage const&> const&)")]
#[doc(alias = "j___ZN5boost9function1IvRKN3RBX18StandardOutMessageEE13assign_to_ownERKS5_")]
// 0xf277c4 — j___ZN5boost9function1IvRKN3RBX18StandardOutMessageEE13assign_to_ownERKS5_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf277c4() {
    // IDA 0xf277c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::clear(void)")]
#[doc(alias = "j___ZN5boost9function1IvRKN3RBX18StandardOutMessageEE5clearEv")]
// 0xf277d4 — j___ZN5boost9function1IvRKN3RBX18StandardOutMessageEE5clearEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf277d4() {
    // IDA 0xf277d4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::operator()(RBX::StandardOutMessage const&)const")]
#[doc(alias = "j___ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_")]
// 0xf277e4 — j___ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_
// type: 
pub fn stub_0xf277e4() {
    // IDA 0xf277e4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_")]
// 0xf284e4 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf284e4() {
    // IDA 0xf284e4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) &,boost::_bi::list0 &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEclIPFvS7_RKSB_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
// 0xf284f4 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEclIPFvS7_RKSB_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf284f4() {
    // IDA 0xf284f4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>::bind_t(void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>> const&)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS4_11work_resultEEEENS0_5list2INS0_5valueIS6_EENSF_IS9_EEEEEC2ESD_RKSI_")]
// 0xf28514 — j___ZN5boost3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS4_11work_resultEEEENS0_5list2INS0_5valueIS6_EENSF_IS9_EEEEEC2ESD_RKSI_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf28514() {
    // IDA 0xf28514: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
#[doc(alias = "j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_")]
// 0xf28524 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf28524() {
    // IDA 0xf28524: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&,rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>(void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>)")]
#[doc(alias = "j___ZN5boost4bindIvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS3_11work_resultEEES5_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_")]
// 0xf28544 — j___ZN5boost4bindIvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS3_11work_resultEEES5_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
pub fn stub_0xf28544() {
    // IDA 0xf28544: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::worker_thread::data,RBX::worker_thread::data>(rbx_core::SharedPtr<RBX::worker_thread::data> *,RBX::worker_thread::data *,boost::detail::shared_count &)")]
#[doc(alias = "j___ZN5boost6detail20sp_pointer_constructIN3RBX13worker_thread4dataES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xf28564 — j___ZN5boost6detail20sp_pointer_constructIN3RBX13worker_thread4dataES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf28564() {
    // IDA 0xf28564: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0xf28574 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf28574() {
    // IDA 0xf28574: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>)")]
#[doc(alias = "j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS0_INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSH_ISB_EEEEEEEEvT_")]
// 0xf28594 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS0_INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSH_ISB_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
pub fn stub_0xf28594() {
    // IDA 0xf28594: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS9_11work_resultEEEENS5_5list2INS5_5valueISB_EENSK_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0xf285b4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS9_11work_resultEEEENS5_5list2INS5_5valueISB_EENSK_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf285b4() {
    // IDA 0xf285b4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::reset(rbx_core::SharedPtr<RBX::Limits::Counter>*)")]
#[doc(alias = "j___ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE5resetEPS5_")]
// 0xf285f4 — j___ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE5resetEPS5_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf285f4() {
    // IDA 0xf285f4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::~thread_specific_ptr()")]
#[doc(alias = "j___ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEED2Ev")]
// 0xf28604 — j___ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEED2Ev
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf28604() {
    // IDA 0xf28604: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(rbx_core::WeakPtr<RBX::TaskScheduler::Job> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE")]
// 0xf28644 — j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf28644() {
    // IDA 0xf28644: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::~thread_specific_ptr()")]
#[doc(alias = "j___ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev")]
// 0xf28674 — j___ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf28674() {
    // IDA 0xf28674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CEvent>(RBX::CEvent *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_")]
// 0xf28684 — j___ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf28684() {
    // IDA 0xf28684: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_create_node(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_")]
// 0xf286b4 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
pub fn stub_0xf286b4() {
    // IDA 0xf286b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_")]
// 0xf286c4 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
pub fn stub_0xf286c4() {
    // IDA 0xf286c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_")]
// 0xf286d4 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf286d4() {
    // IDA 0xf286d4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::TaskScheduler::Job>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf286e4 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf286e4() {
    // IDA 0xf286e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_")]
// 0xf28774 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf28774() {
    // IDA 0xf28774: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *)")]
#[doc(alias = "j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_")]
// 0xf28784 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf28784() {
    // IDA 0xf28784: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>*,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&)")]
#[doc(alias = "j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")]
// 0xf28794 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf28794() {
    // IDA 0xf28794: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::push_back(rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&)")]
#[doc(alias = "j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE9push_backERKS5_")]
// 0xf287a4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int, int)
pub fn stub_0xf287a4() {
    // IDA 0xf287a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>> std::__find_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>,std::random_access_iterator_tag)")]
#[doc(alias = "j___ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESt6vectorIS7_SaIS7_EEEENS2_3_bi6bind_tIbNS2_4_mfi3mf1IbS6_PNS4_13TaskScheduler3JobEEENSD_5list2INS2_3argILi1EEENSD_5valueISJ_EEEEEEET_SS_SS_T0_St26random_access_iterator_tag")]
// 0xf287b4 — j___ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESt6vectorIS7_SaIS7_EEEENS2_3_bi6bind_tIbNS2_4_mfi3mf1IbS6_PNS4_13TaskScheduler3JobEEENSD_5list2INS2_3argILi1EEENSD_5valueISJ_EEEEEEET_SS_SS_T0_St26random_access_iterator_tag
// type: 
pub fn stub_0xf287b4() {
    // IDA 0xf287b4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::sample(void)")]
#[doc(alias = "j___ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE6sampleEv")]
// 0xf287f4 — j___ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE6sampleEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf287f4() {
    // IDA 0xf287f4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Thread>::shared_ptr<RBX::TaskScheduler::Thread>(rbx_core::WeakPtr<RBX::TaskScheduler::Thread> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEC2IS3_EERKNS_8weak_ptrIT_EE")]
// 0xf28804 — j___ZN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEC2IS3_EERKNS_8weak_ptrIT_EE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf28804() {
    // IDA 0xf28804: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::reset(RBX::TaskScheduler::Job **)")]
#[doc(alias = "j___ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE5resetEPS4_")]
// 0xf288d4 — j___ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE5resetEPS4_
// type: int __fastcall(_DWORD)
pub fn stub_0xf288d4() {
    // IDA 0xf288d4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>)")]
#[doc(alias = "j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13TaskScheduler6ThreadEEEEEEC2ES8_")]
// 0xf28914 — j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13TaskScheduler6ThreadEEEEEEC2ES8_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf28914() {
    // IDA 0xf28914: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
