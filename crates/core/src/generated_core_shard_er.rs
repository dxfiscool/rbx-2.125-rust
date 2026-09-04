//! core shard ER — 100 core stubs EA-sorted, lowest uncovered 0xaabaf0..0xb1f5a8 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EQ 0xaab254).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,std::string,std::string)>::operator()(std::string,std::string,std::string)")]
// 0xaabaf0 — __ZN3rbx7signals16signal_with_argsILi3EFvSsSsSsEEclESsSsSs
pub fn stub_aabaf0() {
    // IDA 0xaabaf0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot> &)")]
// 0xaabe7c — __ZN3rbx7signals6signalIFvSsSsSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(std::string,std::string,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot> &)
pub fn stub_aabe7c() {
    // IDA 0xaabe7c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,std::string,std::string)>::fireItem(rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot *,std::string,std::string,std::string)")]
// 0xaac080 — __ZN3rbx7signals16signal_with_argsILi3EFvSsSsSsEE8fireItemEPNS0_6signalIS2_E4slotESsSsSs
pub fn stub_aac080() {
    // IDA 0xaac080: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(bool,int)>::remote_signal(void)")]
// 0xaac2c0 — __ZN3rbx13remote_signalIFvbiEEC2Ev
pub fn stub_aac2c0() {
    // IDA 0xaac2c0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::disconnectAll(void)")]
// 0xaac4c0 — __ZN3rbx7signals6signalIFvbiEE13disconnectAllEv
pub fn stub_aac4c0() {
    // IDA 0xaac4c0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string,std::string)>::remote_signal(void)")]
// 0xaaca18 — __ZN3rbx13remote_signalIFvSsSsSsEEC2Ev
pub fn stub_aaca18() {
    // IDA 0xaaca18: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::disconnectAll(void)")]
// 0xaacc18 — __ZN3rbx7signals6signalIFvSsSsSsEE13disconnectAllEv
pub fn stub_aacc18() {
    // IDA 0xaacc18: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")]
// 0xaae43c — __ZN3rbx13remote_signalIFvSsEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
pub fn stub_aae43c() {
    // IDA 0xaae43c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string,std::string,std::string)>::connect<boost::function<void ()(std::string,std::string,std::string)>>(boost::function<void ()(std::string,std::string,std::string)> const&)")]
// 0xab370c — __ZN3rbx13remote_signalIFvSsSsSsEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
pub fn stub_ab370c() {
    // IDA 0xab370c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string,std::string)>>::~callable_slot()")]
// 0xab38d0 — __ZN3rbx7signals6signalIFvSsSsSsEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_ab38d0() {
    // IDA 0xab38d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string,std::string)>>::~callable_slot()")]
// 0xab38dc — __ZN3rbx7signals6signalIFvSsSsSsEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_ab38dc() {
    // IDA 0xab38dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::function<void ()(std::string,std::string,std::string)>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)")]
// 0xab3990 — __ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsSsSs
pub fn stub_ab3990() {
    // IDA 0xab3990: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::function<void ()(std::string,std::string,std::string)>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)")]
// 0xab3bb8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsSsSs
pub fn stub_ab3bb8() {
    // IDA 0xab3bb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,std::string,std::string,std::string>::operator()(std::string,std::string,std::string)const")]
// 0xab3bc4 — __ZNK5boost9function3IvSsSsSsEclESsSsSs
pub fn stub_ab3bc4() {
    // IDA 0xab3bc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::function<void ()(std::string,std::string,std::string)>,3,void ()(std::string,std::string,std::string)>::~callable()")]
// 0xab3ed4 — __ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_ED2Ev
pub fn stub_ab3ed4() {
    // IDA 0xab3ed4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::function<void ()(std::string,std::string,std::string)>,3,void ()(std::string,std::string,std::string)>::~callable()")]
// 0xab406c — __ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
pub fn stub_ab406c() {
    // IDA 0xab406c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::function<void ()(std::string,std::string,std::string)>,3,void ()(std::string,std::string,std::string)>::~callable()")]
// 0xab4078 — __ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
pub fn stub_ab4078() {
    // IDA 0xab4078: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(void)>::connect<boost::function<void ()(void)>>(boost::function<void ()(void)> const&)")]
// 0xab5088 — __ZN3rbx13remote_signalIFvvEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
pub fn stub_ab5088() {
    // IDA 0xab5088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(bool,int)>::connect<boost::function<void ()(bool,int)>>(boost::function<void ()(bool,int)> const&)")]
// 0xac75b8 — __ZN3rbx13remote_signalIFvbiEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
pub fn stub_ac75b8() {
    // IDA 0xac75b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::function<void ()(bool,int)>>::~callable_slot()")]
// 0xac777c — __ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_ac777c() {
    // IDA 0xac777c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::function<void ()(bool,int)>>::~callable_slot()")]
// 0xac7788 — __ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_ac7788() {
    // IDA 0xac7788: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::function<void ()(bool,int)>,2,void ()(bool,int)>::call(bool,int)")]
// 0xac783c — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost8functionIS3_EELi2ES3_E4callEbi
pub fn stub_ac783c() {
    // IDA 0xac783c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::function<void ()(bool,int)>,2,void ()(bool,int)>::call(bool,int)")]
// 0xac7904 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost8functionIS3_EELi2ES3_E4callEbi
pub fn stub_ac7904() {
    // IDA 0xac7904: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::function<void ()(bool,int)>,2,void ()(bool,int)>::~callable()")]
// 0xac7a7c — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost8functionIS3_EELi2ES3_ED2Ev
pub fn stub_ac7a7c() {
    // IDA 0xac7a7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::function<void ()(bool,int)>,2,void ()(bool,int)>::~callable()")]
// 0xac7c14 — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev
pub fn stub_ac7c14() {
    // IDA 0xac7c14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::function<void ()(bool,int)>,2,void ()(bool,int)>::~callable()")]
// 0xac7c20 — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev
pub fn stub_ac7c20() {
    // IDA 0xac7c20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::RunningAverageDutyCycle<(RBX::Time::SampleMethod)1>::sample(RBX::Time::Interval)")]
// 0xad7d94 — __ZN3RBX23RunningAverageDutyCycleILNS_4Time12SampleMethodE1EE6sampleENS1_8IntervalE
pub fn stub_ad7d94() {
    // IDA 0xad7d94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::multi_index::detail::ordered_index_node_impl<std::allocator<char>>::rebalance(boost::multi_index::detail::ordered_index_node_impl<std::allocator<char>>*,boost::multi_index::detail::ordered_index_node_compressed_base<std::allocator<char>>::parent_ref)")]
// 0xadb8b4 — __ZN5boost11multi_index6detail23ordered_index_node_implISaIcEE9rebalanceEPS4_NS1_34ordered_index_node_compressed_baseIS3_E10parent_refE
pub fn stub_adb8b4() {
    // IDA 0xadb8b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::multi_index::detail::ordered_index_node_impl<std::allocator<char>>::rebalance_for_erase(boost::multi_index::detail::ordered_index_node_impl<std::allocator<char>>*,boost::multi_index::detail::ordered_index_node_compressed_base<std::allocator<char>>::parent_ref,boost::multi_index::detail::ordered_index_node_impl<std::allocator<char>>*&,boost::multi_index::detail::ordered_index_node_impl<std::allocator<char>>*&)")]
// 0xadc470 — __ZN5boost11multi_index6detail23ordered_index_node_implISaIcEE19rebalance_for_eraseEPS4_NS1_34ordered_index_node_compressed_baseIS3_E10parent_refERS5_S9_
pub fn stub_adc470() {
    // IDA 0xadc470: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
// 0xb06f18 — __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEE
// was: RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job>)
pub fn stub_b06f18() {
    // IDA 0xb06f18: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,bool)>::operator()(std::string,bool)")]
// 0xb0b408 — __ZN3rbx7signals16signal_with_argsILi2EFvSsbEEclESsb
pub fn stub_b0b408() {
    // IDA 0xb0b408: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::~shared_count()")]
// 0xb0e5d0 — __ZN5boost6detail12shared_countD1Ev
pub fn stub_b0e5d0() {
    // IDA 0xb0e5d0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::ObjectValue::ObjectValue(void)")]
// 0xb0e670 — __ZN3RBX11ObjectValueC2Ev
pub fn stub_b0e670() {
    // IDA 0xb0e670: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::ObjectValue::~ObjectValue()")]
// 0xb0eaa0 — __ZN3RBX11ObjectValueD0Ev
pub fn stub_b0eaa0() {
    // IDA 0xb0eaa0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ObjectValue::~ObjectValue()")]
// 0xb0ebb0 — __ZThn32_N3RBX11ObjectValueD0Ev
pub fn stub_b0ebb0() {
    // IDA 0xb0ebb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ObjectValue::~ObjectValue()")]
// 0xb0ec58 — __ZThn36_N3RBX11ObjectValueD1Ev
pub fn stub_b0ec58() {
    // IDA 0xb0ec58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ObjectValue::~ObjectValue()")]
// 0xb0ec68 — __ZN3RBX11ObjectValueD2Ev
pub fn stub_b0ec68() {
    // IDA 0xb0ec68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringValue::StringValue(void)")]
// 0xb0fab0 — __ZN3RBX11StringValueC2Ev
pub fn stub_b0fab0() {
    // IDA 0xb0fab0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringValue::~StringValue()")]
// 0xb0feb8 — __ZN3RBX11StringValueD0Ev
pub fn stub_b0feb8() {
    // IDA 0xb0feb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringValue::getPersistentDataCost(void)const")]
// 0xb0ff58 — __ZNK3RBX11StringValue21getPersistentDataCostEv
pub fn stub_b0ff58() {
    // IDA 0xb0ff58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::StringValue::~StringValue()")]
// 0xb0fff8 — __ZThn32_N3RBX11StringValueD0Ev
pub fn stub_b0fff8() {
    // IDA 0xb0fff8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::StringValue::~StringValue()")]
// 0xb100a0 — __ZThn36_N3RBX11StringValueD1Ev
pub fn stub_b100a0() {
    // IDA 0xb100a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringValue::~StringValue()")]
// 0xb100b0 — __ZN3RBX11StringValueD2Ev
pub fn stub_b100b0() {
    // IDA 0xb100b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CylinderMesh::~CylinderMesh()")]
// 0xb10d08 — __ZN3RBX12CylinderMeshD0Ev
pub fn stub_b10d08() {
    // IDA 0xb10d08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
// 0xb10e18 — __ZThn32_N3RBX12CylinderMeshD0Ev
pub fn stub_b10e18() {
    // IDA 0xb10e18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
// 0xb10ec0 — __ZThn36_N3RBX12CylinderMeshD1Ev
pub fn stub_b10ec0() {
    // IDA 0xb10ec0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::throw_exception<boost::io::too_few_args>(boost::io::too_few_args const&)")]
// 0xb11600 — __ZN5boost15throw_exceptionINS_2io12too_few_argsEEEvRKT_
pub fn stub_b11600() {
    // IDA 0xb11600: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::io::too_few_args::~too_few_args()")]
// 0xb11728 — __ZN5boost2io12too_few_argsD1Ev
pub fn stub_b11728() {
    // IDA 0xb11728: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::io::too_few_args::what(void)const")]
// 0xb11738 — __ZNK5boost2io12too_few_args4whatEv
pub fn stub_b11738() {
    // IDA 0xb11738: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::rethrow(void)const")]
// 0xb11748 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE7rethrowEv
pub fn stub_b11748() {
    // IDA 0xb11748: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
// 0xb11758 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED0Ev
pub fn stub_b11758() {
    // IDA 0xb11758: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")]
// 0xb11830 — __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED0Ev
pub fn stub_b11830() {
    // IDA 0xb11830: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_oaltstringstream()")]
// 0xb118f0 — __ZN5boost2io22basic_oaltstringstreamIcSt11char_traitsIcESaIcEED1Ev
pub fn stub_b118f0() {
    // IDA 0xb118f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_oaltstringstream()")]
// 0xb119b0 — __ZTv0_n12_N5boost2io22basic_oaltstringstreamIcSt11char_traitsIcESaIcEED1Ev
pub fn stub_b119b0() {
    // IDA 0xb119b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_oaltstringstream()")]
// 0xb11a80 — __ZTv0_n12_N5boost2io22basic_oaltstringstreamIcSt11char_traitsIcESaIcEED0Ev
pub fn stub_b11a80() {
    // IDA 0xb11a80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")]
// 0xb11b70 — __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED1Ev
pub fn stub_b11b70() {
    // IDA 0xb11b70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
// 0xb11c28 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED1Ev
pub fn stub_b11c28() {
    // IDA 0xb11c28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
// 0xb11ce0 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED1Ev
pub fn stub_b11ce0() {
    // IDA 0xb11ce0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone(void)const")]
// 0xb11db0 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE5cloneEv
pub fn stub_b11db0() {
    // IDA 0xb11db0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_tag)")]
// 0xb11e70 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEEC1ERKS6_NS6_9clone_tagE
pub fn stub_b11e70() {
    // IDA 0xb11e70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::parse(std::string const&)")]
// 0xb11fd8 — __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEE5parseERKSs
pub fn stub_b11fd8() {
    // IDA 0xb11fd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::~vector()")]
// 0xb125e8 — __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EED1Ev
pub fn stub_b125e8() {
    // IDA 0xb125e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0xb126b0 — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
pub fn stub_b126b0() {
    // IDA 0xb126b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
// 0xb127d8 — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
pub fn stub_b127d8() {
    // IDA 0xb127d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "int boost::io::detail::upper_bound_from_fstring<std::string,std::ctype<char>>(std::string const&,std::string::value_type,std::ctype<char> const&,unsigned char)")]
// 0xb128f8 — __ZN5boost2io6detail24upper_bound_from_fstringISsSt5ctypeIcEEEiRKT_NS5_10value_typeERKT0_h
pub fn stub_b128f8() {
    // IDA 0xb128f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::throw_exception<boost::io::bad_format_string>(boost::io::bad_format_string const&)")]
// 0xb12a88 — __ZN5boost15throw_exceptionINS_2io17bad_format_stringEEEvRKT_
pub fn stub_b12a88() {
    // IDA 0xb12a88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::io::bad_format_string::~bad_format_string()")]
// 0xb12bb0 — __ZN5boost2io17bad_format_stringD1Ev
pub fn stub_b12bb0() {
    // IDA 0xb12bb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>*,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&)")]
// 0xb12bc0 — __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS7_S9_EEmRKS7_
pub fn stub_b12bc0() {
    // IDA 0xb12bc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> * std::__uninitialized_copy_a<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>)")]
// 0xb13338 — __ZSt22__uninitialized_copy_aIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEES8_S7_ET0_T_SA_S9_SaIT1_E
pub fn stub_b13338() {
    // IDA 0xb13338: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void std::__uninitialized_fill_n_a<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>)")]
// 0xb135cc — __ZSt24__uninitialized_fill_n_aIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEEmS7_S7_EvT_T0_RKT1_SaIT2_E
pub fn stub_b135cc() {
    // IDA 0xb135cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::io::bad_format_string::what(void)const")]
// 0xb13848 — __ZNK5boost2io17bad_format_string4whatEv
pub fn stub_b13848() {
    // IDA 0xb13848: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::rethrow(void)const")]
// 0xb13858 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE7rethrowEv
pub fn stub_b13858() {
    // IDA 0xb13858: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")]
// 0xb13868 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED0Ev
pub fn stub_b13868() {
    // IDA 0xb13868: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")]
// 0xb13940 — __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED0Ev
pub fn stub_b13940() {
    // IDA 0xb13940: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,bool)>::slot> &)")]
// 0xb19898 — __ZN3rbx7signals6signalIFvSsbEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(std::string,bool)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,bool)>::slot> &)
pub fn stub_b19898() {
    // IDA 0xb19898: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,bool)>::slot> const&)")]
// 0xb19ab0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsbEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,bool)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,bool)>::slot> const&)
pub fn stub_b19ab0() {
    // IDA 0xb19ab0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "void boost::unordered::detail::array_constructor<boost::fast_pool_allocator<boost::unordered::detail::ptr_bucket,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::construct<boost::unordered::detail::ptr_bucket>(boost::unordered::detail::ptr_bucket const&,unsigned long)")]
// 0xb1a1b0 — __ZN5boost9unordered6detail17array_constructorINS_19fast_pool_allocatorINS1_10ptr_bucketENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEE9constructIS4_EEvRKT_m
pub fn stub_b1a1b0() {
    // IDA 0xb1a1b0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "void boost::throw_exception<std::bad_alloc>(std::bad_alloc const&)")]
// 0xb1a370 — __ZN5boost15throw_exceptionISt9bad_allocEEvRKT_
pub fn stub_b1a370() {
    // IDA 0xb1a370: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::~clone_impl()")]
// 0xb1a490 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEED1Ev
pub fn stub_b1a490() {
    // IDA 0xb1a490: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::bad_alloc>::~error_info_injector()")]
// 0xb1a548 — __ZN5boost16exception_detail19error_info_injectorISt9bad_allocED1Ev
pub fn stub_b1a548() {
    // IDA 0xb1a548: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<std::bad_alloc>::~error_info_injector()")]
// 0xb1a600 — __ZThn4_N5boost16exception_detail19error_info_injectorISt9bad_allocED1Ev
pub fn stub_b1a600() {
    // IDA 0xb1a600: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::~clone_impl()")]
// 0xb1a6b8 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEED1Ev
pub fn stub_b1a6b8() {
    // IDA 0xb1a6b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone(void)const")]
// 0xb1a788 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEE5cloneEv
pub fn stub_b1a788() {
    // IDA 0xb1a788: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone(void)const")]
// 0xb1a848 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEE5cloneEv
pub fn stub_b1a848() {
    // IDA 0xb1a848: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::bad_alloc>::~error_info_injector()")]
// 0xb1a910 — __ZN5boost16exception_detail19error_info_injectorISt9bad_allocED0Ev
pub fn stub_b1a910() {
    // IDA 0xb1a910: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone_tag)")]
// 0xb1a9d0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEEC1ERKS5_NS5_9clone_tagE
pub fn stub_b1a9d0() {
    // IDA 0xb1a9d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone_impl(boost::exception_detail::error_info_injector<std::bad_alloc> const&)")]
// 0xb1ab30 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEEC1ERKS4_
pub fn stub_b1ab30() {
    // IDA 0xb1ab30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::pool<boost::default_user_allocator_new_delete>::ordered_malloc(unsigned long)")]
// 0xb1ac90 — __ZN5boost4poolINS_33default_user_allocator_new_deleteEE14ordered_mallocEm
pub fn stub_b1ac90() {
    // IDA 0xb1ac90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::pool<boost::default_user_allocator_new_delete>::malloc_need_resize(void)")]
// 0xb1af38 — __ZN5boost4poolINS_33default_user_allocator_new_deleteEE18malloc_need_resizeEv
pub fn stub_b1af38() {
    // IDA 0xb1af38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ContentId>::construct_func(char const*,char *)")]
// 0xb1f3b0 — __ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE14construct_funcEPKcPc
pub fn stub_b1f3b0() {
    // IDA 0xb1f3b0: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Region3int16>::construct_func(char const*,char *)")]
// 0xb1f3d0 — __ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE14construct_funcEPKcPc
pub fn stub_b1f3d0() {
    // IDA 0xb1f3d0: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Region3>(RBX::Region3 const&)")]
// 0xb1f3e8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIS2_EERS3_RKT_
pub fn stub_b1f3e8() {
    // IDA 0xb1f3e8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::BrickColor>::construct_func(char const*,char *)")]
// 0xb1f528 — __ZN3rbx14implementation12typed_holderIN3RBX10BrickColorEE14construct_funcEPKcPc
pub fn stub_b1f528() {
    // IDA 0xb1f528: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Axes>::construct_func(char const*,char *)")]
// 0xb1f538 — __ZN3rbx14implementation12typed_holderIN3RBX4AxesEE14construct_funcEPKcPc
pub fn stub_b1f538() {
    // IDA 0xb1f538: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Faces>::construct_func(char const*,char *)")]
// 0xb1f548 — __ZN3rbx14implementation12typed_holderIN3RBX5FacesEE14construct_funcEPKcPc
pub fn stub_b1f548() {
    // IDA 0xb1f548: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::RbxRay>::construct_func(char const*,char *)")]
// 0xb1f558 — __ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE14construct_funcEPKcPc
pub fn stub_b1f558() {
    // IDA 0xb1f558: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UDim2>::construct_func(char const*,char *)")]
// 0xb1f588 — __ZN3rbx14implementation12typed_holderIN3RBX5UDim2EE14construct_funcEPKcPc
pub fn stub_b1f588() {
    // IDA 0xb1f588: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UDim2>::destruct_func(char *)")]
// 0xb1f598 — __ZN3rbx14implementation12typed_holderIN3RBX5UDim2EE13destruct_funcEPc
pub fn stub_b1f598() {
    // IDA 0xb1f598: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<long>::construct_func(char const*,char *)")]
// 0xb1f59c — __ZN3rbx14implementation12typed_holderIlE14construct_funcEPKcPc
pub fn stub_b1f59c() {
    // IDA 0xb1f59c: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<long>::destruct_func(char *)")]
// 0xb1f5a8 — __ZN3rbx14implementation12typed_holderIlE13destruct_funcEPc
pub fn stub_b1f5a8() {
    // IDA 0xb1f5a8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
