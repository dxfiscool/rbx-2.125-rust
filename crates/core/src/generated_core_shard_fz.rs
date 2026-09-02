//! core shard FZ — 100 core stubs EA-sorted, 0xf43304..0xf442b4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf432f4).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf432f4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::ManualWeldJoint::~ManualWeldJoint()")]
// 0xf43304 — j___ZN3RBX15ManualWeldJointD0Ev
pub fn stub_f43304() -> ! {
    todo!("0xf43304 j___ZN3RBX15ManualWeldJointD0Ev")
}

#[doc(alias = "RBX::SnapJoint::~SnapJoint()")]
// 0xf433d4 — j___ZN3RBX9SnapJointD0Ev
pub fn stub_f433d4() -> ! {
    todo!("0xf433d4 j___ZN3RBX9SnapJointD0Ev")
}

#[doc(alias = "RBX::WeldJoint::~WeldJoint()")]
// 0xf433e4 — j___ZN3RBX9WeldJointD0Ev
pub fn stub_f433e4() -> ! {
    todo!("0xf433e4 j___ZN3RBX9WeldJointD0Ev")
}

#[doc(alias = "RBX::JointsService::~JointsService()")]
// 0xf43574 — j___ZN3RBX13JointsServiceD2Ev
pub fn stub_f43574() -> ! {
    todo!("0xf43574 j___ZN3RBX13JointsServiceD2Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::safe_static_do_get_mutex(void)")]
// 0xf43774 — j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv
pub fn stub_f43774() -> ! {
    todo!("0xf43774 j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::safe_static_do_get_mutex(void)")]
// 0xf43784 — j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f43784() -> ! {
    todo!("0xf43784 j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::insert(rbx::signals::signal<void ()(RBX::Joint *)>::slot *)")]
// 0xf43794 — j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE6insertEPNS6_4slotE
pub fn stub_f43794() -> ! {
    todo!("0xf43794 j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::remove(rbx::signals::signal<void ()(RBX::Joint *)>::slot *)")]
// 0xf437a4 — j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE6removeEPNS6_4slotE
pub fn stub_f437a4() -> ! {
    todo!("0xf437a4 j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Joint *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>> const&)")]
// 0xf437b4 — j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_f437b4() -> ! {
    todo!("0xf437b4 j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::IAdornableCollector>::operator=(rbx_core::SharedPtr<RBX::IAdornableCollector> const&)")]
// 0xf43834 — j___ZN5boost10shared_ptrIN3RBX19IAdornableCollectorEEaSERKS3_
// was: boost::shared_ptr<RBX::IAdornableCollector>::operator=(boost::shared_ptr<RBX::IAdornableCollector> const&)
pub fn stub_f43834() -> ! {
    todo!("0xf43834 j___ZN5boost10shared_ptrIN3RBX19IAdornableCollectorEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Joint *)>::slot*)")]
// 0xf43894 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Joint *)>::slot*)
pub fn stub_f43894() -> ! {
    todo!("0xf43894 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot> const&)")]
// 0xf438a4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot> const&)
pub fn stub_f438a4() -> ! {
    todo!("0xf438a4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSERKSA_")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>::operator()<RBX::Joint *>(RBX::Joint * &)")]
// 0xf438c4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13JointsServiceEPNS4_5JointEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
pub fn stub_f438c4() -> ! {
    todo!("0xf438c4 j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13JointsServiceEPNS4_5JointEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_")
}

#[doc(alias = "RBX::KeyframeSequence::~KeyframeSequence()")]
// 0xf43b64 — j___ZN3RBX16KeyframeSequenceD1Ev
pub fn stub_f43b64() -> ! {
    todo!("0xf43b64 j___ZN3RBX16KeyframeSequenceD1Ev")
}

#[doc(alias = "unsigned long RBX::findOrAdd<std::string>(std::vector<std::string,std::allocator<std::string>> &,std::string const&)")]
// 0xf43b84 — j___ZN3RBX9findOrAddISsEEmRSt6vectorIT_SaIS2_EERKS2_
pub fn stub_f43b84() -> ! {
    todo!("0xf43b84 j___ZN3RBX9findOrAddISsEEmRSt6vectorIT_SaIS2_EERKS2_")
}

#[doc(alias = "unsigned long RBX::findOrAdd<std::pair<unsigned long,unsigned long>>(std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>> &,std::pair<unsigned long,unsigned long> const&)")]
// 0xf43b94 — j___ZN3RBX9findOrAddISt4pairImmEEEmRSt6vectorIT_SaIS4_EERKS4_
pub fn stub_f43b94() -> ! {
    todo!("0xf43b94 j___ZN3RBX9findOrAddISt4pairImmEEEmRSt6vectorIT_SaIS4_EERKS4_")
}

#[doc(alias = "std::_Vector_base<RBX::CachedPose,std::allocator<RBX::CachedPose>>::_M_allocate(unsigned long)")]
// 0xf43c24 — j___ZNSt12_Vector_baseIN3RBX10CachedPoseESaIS1_EE11_M_allocateEm
pub fn stub_f43c24() -> ! {
    todo!("0xf43c24 j___ZNSt12_Vector_baseIN3RBX10CachedPoseESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::_M_allocate(unsigned long)")]
// 0xf43c34 — j___ZNSt12_Vector_baseIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE11_M_allocateEm
pub fn stub_f43c34() -> ! {
    todo!("0xf43c34 j___ZNSt12_Vector_baseIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::_M_allocate(unsigned long)")]
// 0xf43c44 — j___ZNSt12_Vector_baseIN3RBX16KeyframeSequence8PriorityESaIS2_EE11_M_allocateEm
pub fn stub_f43c44() -> ! {
    todo!("0xf43c44 j___ZNSt12_Vector_baseIN3RBX16KeyframeSequence8PriorityESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::_M_allocate(unsigned long)")]
// 0xf43c54 — j___ZNSt12_Vector_baseIPN3RBX10CachedPoseESaIS2_EE11_M_allocateEm
pub fn stub_f43c54() -> ! {
    todo!("0xf43c54 j___ZNSt12_Vector_baseIPN3RBX10CachedPoseESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::_Vector_base(unsigned long,std::allocator<RBX::CachedPose *> const&)")]
// 0xf43c64 — j___ZNSt12_Vector_baseIPN3RBX10CachedPoseESaIS2_EEC2EmRKS3_
pub fn stub_f43c64() -> ! {
    todo!("0xf43c64 j___ZNSt12_Vector_baseIPN3RBX10CachedPoseESaIS2_EEC2EmRKS3_")
}

#[doc(alias = "std::_Vector_base<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>>::_M_allocate(unsigned long)")]
// 0xf43c74 — j___ZNSt12_Vector_baseISt4pairImmESaIS1_EE11_M_allocateEm
pub fn stub_f43c74() -> ! {
    todo!("0xf43c74 j___ZNSt12_Vector_baseISt4pairImmESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::CachedPose * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CachedPose *,RBX::CachedPose *>(RBX::CachedPose *,RBX::CachedPose *,RBX::CachedPose *)")]
// 0xf43c84 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10CachedPoseES5_EET0_T_S7_S6_
pub fn stub_f43c84() -> ! {
    todo!("0xf43c84 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10CachedPoseES5_EET0_T_S7_S6_")
}

#[doc(alias = "RBX::KeyframeSequence::CachedKeyframe * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::KeyframeSequence::CachedKeyframe *,RBX::KeyframeSequence::CachedKeyframe *>(RBX::KeyframeSequence::CachedKeyframe *,RBX::KeyframeSequence::CachedKeyframe *,RBX::KeyframeSequence::CachedKeyframe *)")]
// 0xf43c94 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16KeyframeSequence14CachedKeyframeES6_EET0_T_S8_S7_
pub fn stub_f43c94() -> ! {
    todo!("0xf43c94 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16KeyframeSequence14CachedKeyframeES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::KeyframeSequence::Priority * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::KeyframeSequence::Priority *,RBX::KeyframeSequence::Priority *>(RBX::KeyframeSequence::Priority *,RBX::KeyframeSequence::Priority *,RBX::KeyframeSequence::Priority *)")]
// 0xf43ca4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16KeyframeSequence8PriorityES6_EET0_T_S8_S7_
pub fn stub_f43ca4() -> ! {
    todo!("0xf43ca4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16KeyframeSequence8PriorityES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::pair<unsigned long,unsigned long> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<unsigned long,unsigned long> *,std::pair<unsigned long,unsigned long> *>(std::pair<unsigned long,unsigned long> *,std::pair<unsigned long,unsigned long> *,std::pair<unsigned long,unsigned long> *)")]
// 0xf43cb4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairImmES5_EET0_T_S7_S6_
pub fn stub_f43cb4() -> ! {
    todo!("0xf43cb4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairImmES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::KeyframeSequence::Priority,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::operator[](RBX::Name const* const&)")]
// 0xf43cc4 — j___ZNSt3mapIPKN3RBX4NameENS0_16KeyframeSequence8PriorityESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f43cc4() -> ! {
    todo!("0xf43cc4 j___ZNSt3mapIPKN3RBX4NameENS0_16KeyframeSequence8PriorityESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CachedPose*,std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>>,RBX::CachedPose const&)")]
// 0xf43cd4 — j___ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_f43cd4() -> ! {
    todo!("0xf43cd4 j___ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::reserve(unsigned long)")]
// 0xf43ce4 — j___ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE7reserveEm
pub fn stub_f43ce4() -> ! {
    todo!("0xf43ce4 j___ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::push_back(RBX::CachedPose const&)")]
// 0xf43cf4 — j___ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE9push_backERKS1_
pub fn stub_f43cf4() -> ! {
    todo!("0xf43cf4 j___ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe const&)")]
// 0xf43d04 — j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f43d04() -> ! {
    todo!("0xf43d04 j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::_M_erase_at_end(RBX::KeyframeSequence::CachedKeyframe*)")]
// 0xf43d14 — j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE15_M_erase_at_endEPS2_
pub fn stub_f43d14() -> ! {
    todo!("0xf43d14 j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE15_M_erase_at_endEPS2_")
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::push_back(RBX::KeyframeSequence::CachedKeyframe const&)")]
// 0xf43d24 — j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE9push_backERKS2_
pub fn stub_f43d24() -> ! {
    todo!("0xf43d24 j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::~vector()")]
// 0xf43d34 — j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EED2Ev
pub fn stub_f43d34() -> ! {
    todo!("0xf43d34 j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EED2Ev")
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::Priority*,std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>>,RBX::KeyframeSequence::Priority const&)")]
// 0xf43d44 — j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f43d44() -> ! {
    todo!("0xf43d44 j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::Priority*,std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>>,unsigned long,RBX::KeyframeSequence::Priority const&)")]
// 0xf43d54 — j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f43d54() -> ! {
    todo!("0xf43d54 j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::resize(unsigned long,RBX::KeyframeSequence::Priority)")]
// 0xf43d64 — j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE6resizeEmS2_
pub fn stub_f43d64() -> ! {
    todo!("0xf43d64 j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::push_back(RBX::KeyframeSequence::Priority const&)")]
// 0xf43d74 — j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE9push_backERKS2_
pub fn stub_f43d74() -> ! {
    todo!("0xf43d74 j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CachedPose **,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>>,unsigned long,RBX::CachedPose * const&)")]
// 0xf43d84 — j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f43d84() -> ! {
    todo!("0xf43d84 j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::resize(unsigned long,RBX::CachedPose *)")]
// 0xf43d94 — j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE6resizeEmS2_
pub fn stub_f43d94() -> ! {
    todo!("0xf43d94 j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::vector(std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> const&)")]
// 0xf43da4 — j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEC2ERKS4_
pub fn stub_f43da4() -> ! {
    todo!("0xf43da4 j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEC2ERKS4_")
}

#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::operator=(std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> const&)")]
// 0xf43db4 — j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEaSERKS4_
pub fn stub_f43db4() -> ! {
    todo!("0xf43db4 j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEaSERKS4_")
}

#[doc(alias = "std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<unsigned long,unsigned long>*,std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>>>,std::pair<unsigned long,unsigned long> const&)")]
// 0xf43dc4 — j___ZNSt6vectorISt4pairImmESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_f43dc4() -> ! {
    todo!("0xf43dc4 j___ZNSt6vectorISt4pairImmESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>>::push_back(std::pair<unsigned long,unsigned long> const&)")]
// 0xf43dd4 — j___ZNSt6vectorISt4pairImmESaIS1_EE9push_backERKS1_
pub fn stub_f43dd4() -> ! {
    todo!("0xf43dd4 j___ZNSt6vectorISt4pairImmESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority> const&)")]
// 0xf43de4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f43de4() -> ! {
    todo!("0xf43de4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority> const&)")]
// 0xf43df4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f43df4() -> ! {
    todo!("0xf43df4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority> const&)")]
// 0xf43e04 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f43e04() -> ! {
    todo!("0xf43e04 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int,RBX::KeyframeSequence::CachedKeyframe>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int,int,RBX::KeyframeSequence::CachedKeyframe)")]
// 0xf43e14 — j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
pub fn stub_f43e14() -> ! {
    todo!("0xf43e14 j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_")
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int,RBX::KeyframeSequence::CachedKeyframe>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int,int,RBX::KeyframeSequence::CachedKeyframe)")]
// 0xf43e24 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
pub fn stub_f43e24() -> ! {
    todo!("0xf43e24 j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_")
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
// 0xf43e34 — j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_SA_
pub fn stub_f43e34() -> ! {
    todo!("0xf43e34 j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_SA_")
}

#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
// 0xf43e44 — j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
pub fn stub_f43e44() -> ! {
    todo!("0xf43e44 j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int)")]
// 0xf43e54 — j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiEvT_SA_T0_
pub fn stub_f43e54() -> ! {
    todo!("0xf43e54 j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiEvT_SA_T0_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>> std::__unguarded_partition<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe)")]
// 0xf43e64 — j___ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_T0_
pub fn stub_f43e64() -> ! {
    todo!("0xf43e64 j___ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_T0_")
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
// 0xf43e74 — j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
pub fn stub_f43e74() -> ! {
    todo!("0xf43e74 j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")
}

#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe)")]
// 0xf43e84 — j___ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEES4_EvT_T0_
pub fn stub_f43e84() -> ! {
    todo!("0xf43e84 j___ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEES4_EvT_T0_")
}

#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
// 0xf43e94 — j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
pub fn stub_f43e94() -> ! {
    todo!("0xf43e94 j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")
}

#[doc(alias = "void std::make_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
// 0xf43ea4 — j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
pub fn stub_f43ea4() -> ! {
    todo!("0xf43ea4 j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")
}

#[doc(alias = "void std::sort_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
// 0xf43eb4 — j___ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
pub fn stub_f43eb4() -> ! {
    todo!("0xf43eb4 j___ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::removeLeastRecentlyUsed(void)")]
// 0xf43f54 — j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE23removeLeastRecentlyUsedEv
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::removeLeastRecentlyUsed(void)
pub fn stub_f43f54() -> ! {
    todo!("0xf43f54 j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE23removeLeastRecentlyUsedEv")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::insert(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&,unsigned long)")]
// 0xf43f64 — j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6insertERKSsRKS4_m
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::insert(std::string const&,boost::shared_ptr<RBX::KeyframeSequence> const&,unsigned long)
pub fn stub_f43f64() -> ! {
    todo!("0xf43f64 j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6insertERKSsRKS4_m")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::remove(std::string const&)")]
// 0xf43f74 — j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6removeERKSs
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::remove(std::string const&)
pub fn stub_f43f74() -> ! {
    todo!("0xf43f74 j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6removeERKSs")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::LRUCache(void)")]
// 0xf43f84 — j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEEC2Ev
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::LRUCache(void)
pub fn stub_f43f84() -> ! {
    todo!("0xf43f84 j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEEC2Ev")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::~LRUCache()")]
// 0xf43f94 — j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEED2Ev
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::~LRUCache()
pub fn stub_f43f94() -> ! {
    todo!("0xf43f94 j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEED2Ev")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::KeyframeSequenceProvider> RBX::weak_from<RBX::KeyframeSequenceProvider>(RBX::KeyframeSequenceProvider*)")]
// 0xf43fb4 — j___ZN3RBX9weak_fromINS_24KeyframeSequenceProviderEEEN5boost8weak_ptrIT_EEPS4_
// was: boost::weak_ptr<RBX::KeyframeSequenceProvider> RBX::weak_from<RBX::KeyframeSequenceProvider>(RBX::KeyframeSequenceProvider*)
pub fn stub_f43fb4() -> ! {
    todo!("0xf43fb4 j___ZN3RBX9weak_fromINS_24KeyframeSequenceProviderEEEN5boost8weak_ptrIT_EEPS4_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ContentId>(RBX::ContentId const&)")]
// 0xf43fc4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9ContentIdEEERS3_RKT_
pub fn stub_f43fc4() -> ! {
    todo!("0xf43fc4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9ContentIdEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ContentId>::singleton(void)")]
// 0xf43fd4 — j___ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE9singletonEv
pub fn stub_f43fd4() -> ! {
    todo!("0xf43fd4 j___ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE9singletonEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence>(rbx_core::WeakPtr<RBX::KeyframeSequence> const&,boost::detail::sp_nothrow_tag)")]
// 0xf43fe4 — j___ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence>(boost::weak_ptr<RBX::KeyframeSequence> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f43fe4() -> ! {
    todo!("0xf43fe4 j___ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence>::operator=(rbx_core::SharedPtr<RBX::KeyframeSequence> const&)")]
// 0xf44004 — j___ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEaSERKS3_
// was: boost::shared_ptr<RBX::KeyframeSequence>::operator=(boost::shared_ptr<RBX::KeyframeSequence> const&)
pub fn stub_f44004() -> ! {
    todo!("0xf44004 j___ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider>(rbx_core::WeakPtr<RBX::KeyframeSequenceProvider> const&,boost::detail::sp_nothrow_tag)")]
// 0xf44014 — j___ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider>(boost::weak_ptr<RBX::KeyframeSequenceProvider> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f44014() -> ! {
    todo!("0xf44014 j___ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>)")]
// 0xf44024 — j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// was: boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>::list2(boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>)
pub fn stub_f44024() -> ! {
    todo!("0xf44024 j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_")
}

#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>)")]
// 0xf44044 — j___ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEC2ES3_S4_SA_SD_
// was: boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>)
pub fn stub_f44044() -> ! {
    todo!("0xf44044 j___ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEC2ES3_S4_SA_SD_")
}

#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>::operator()<void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
// 0xf44054 — j___ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEclIPFvNS7_14AsyncHttpQueue13RequestResultEPSiS9_SC_ENS0_5list3IRSH_RSI_RNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>::operator()<void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const> &>>(boost::_bi::type<void>,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const> &> &,int)
pub fn stub_f44054() -> ! {
    todo!("0xf44054 j___ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEclIPFvNS7_14AsyncHttpQueue13RequestResultEPSiS9_SC_ENS0_5list3IRSH_RSI_RNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>)")]
// 0xf44064 — j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>)
pub fn stub_f44064() -> ! {
    todo!("0xf44064 j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_")
}

#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>)")]
// 0xf44074 — j___ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEC2ES3_S4_SA_SD_
// was: boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>)
pub fn stub_f44074() -> ! {
    todo!("0xf44074 j___ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEC2ES3_S4_SA_SD_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>>::type> boost::bind<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>,boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>>(void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>)")]
// 0xf44084 — j___ZN5boost4bindIvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS1_24KeyframeSequenceProviderEEENS5_INS1_16KeyframeSequenceEEENS_3argILi1EEENSA_ILi2EEES7_S9_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_T3_ENSD_9list_av_4IT4_T5_T6_T7_E4typeEEESL_SN_SO_SP_SQ_
// was: boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>>::type> boost::bind<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>,boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>>(void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>)
pub fn stub_f44084() -> ! {
    todo!("0xf44084 j___ZN5boost4bindIvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS1_24KeyframeSequenceProviderEEENS5_INS1_16KeyframeSequenceEEENS_3argILi1EEENSA_ILi2EEES7_S9_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_T3_ENSD_9list_av_4IT4_T5_T6_T7_E4typeEEESL_SN_SO_SP_SQ_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>,rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>(void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>)")]
// 0xf44094 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS3_EES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list_av_2<boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>>::type> boost::bind<void,boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>,boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>>(void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>)
pub fn stub_f44094() -> ! {
    todo!("0xf44094 j___ZN5boost4bindIvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS3_EES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf440b4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_f440b4() -> ! {
    todo!("0xf440b4 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf440c4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_f440c4() -> ! {
    todo!("0xf440c4 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>)")]
// 0xf44114 — j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEEvT_
// was: void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>>)
pub fn stub_f44114() -> ! {
    todo!("0xf44114 j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEEvT_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> *)")]
// 0xf44134 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>> *)
pub fn stub_f44134() -> ! {
    todo!("0xf44134 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// 0xf44144 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)
pub fn stub_f44144() -> ! {
    todo!("0xf44144 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> const&)")]
// 0xf44154 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERS5_RKT_
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>> const&)
pub fn stub_f44154() -> ! {
    todo!("0xf44154 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERS5_RKT_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0xf44164 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISK_EEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)
pub fn stub_f44164() -> ! {
    todo!("0xf44164 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISK_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> const&)")]
// 0xf44174 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_
// was: void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>> const&)
pub fn stub_f44174() -> ! {
    todo!("0xf44174 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::construct(void)")]
// 0xf44184 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE9constructEv
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>>::construct(void)
pub fn stub_f44184() -> ! {
    todo!("0xf44184 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::~node_constructor()")]
// 0xf44194 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEED2Ev
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>>::~node_constructor()
pub fn stub_f44194() -> ! {
    todo!("0xf44194 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEED2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// 0xf441a4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)
pub fn stub_f441a4() -> ! {
    todo!("0xf441a4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0xf441b4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)
pub fn stub_f441b4() -> ! {
    todo!("0xf441b4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xf441c4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)
pub fn stub_f441c4() -> ! {
    todo!("0xf441c4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0xf441d4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)
pub fn stub_f441d4() -> ! {
    todo!("0xf441d4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xf441e4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)
pub fn stub_f441e4() -> ! {
    todo!("0xf441e4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// 0xf441f4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)
pub fn stub_f441f4() -> ! {
    todo!("0xf441f4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE5clearEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>> const&)")]
// 0xf44204 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSH_RKSJ_RKSaINS1_8ptr_nodeISE_EEE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>> const&)
pub fn stub_f44204() -> ! {
    todo!("0xf44204 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSH_RKSJ_RKSaINS1_8ptr_nodeISE_EEE")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>*)")]
// 0xf44214 — j___ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEE7destroyEPS8_
// was: __gnu_cxx::new_allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>::destroy(std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>*)
pub fn stub_f44214() -> ! {
    todo!("0xf44214 j___ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEE7destroyEPS8_")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::destroy(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>*)")]
// 0xf44224 — j___ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEE7destroyEPS8_
// was: __gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>::destroy(std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>*)
pub fn stub_f44224() -> ! {
    todo!("0xf44224 j___ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEE7destroyEPS8_")
}

#[doc(alias = "RBX::AnimationId::isActive(void)const")]
// 0xf44234 — j___ZNK3RBX11AnimationId8isActiveEv
pub fn stub_f44234() -> ! {
    todo!("0xf44234 j___ZNK3RBX11AnimationId8isActiveEv")
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf44284 — j___ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvS5_S6_NS_8weak_ptrINS3_24KeyframeSequenceProviderEEENSE_INS3_16KeyframeSequenceEEEENSC_5list4INS_3argILi1EEENSM_ILi2EEENSC_5valueISG_EENSP_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_f44284() -> ! {
    todo!("0xf44284 j___ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvS5_S6_NS_8weak_ptrINS3_24KeyframeSequenceProviderEEENSE_INS3_16KeyframeSequenceEEEENSC_5list4INS_3argILi1EEENSM_ILi2EEENSC_5valueISG_EENSP_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const")]
// 0xf44294 — j___ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS5_S6_NS_8weak_ptrINS3_24KeyframeSequenceProviderEEENSE_INS3_16KeyframeSequenceEEEENSC_5list4INS_3argILi1EEENSM_ILi2EEENSC_5valueISG_EENSP_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const
pub fn stub_f44294() -> ! {
    todo!("0xf44294 j___ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS5_S6_NS_8weak_ptrINS3_24KeyframeSequenceProviderEEENSE_INS3_16KeyframeSequenceEEEENSC_5list4INS_3argILi1EEENSM_ILi2EEENSC_5valueISG_EENSP_ISI_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf442a4 — j___ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS5_S6_NS_8weak_ptrINS3_24KeyframeSequenceProviderEEENSE_INS3_16KeyframeSequenceEEEENSC_5list4INS_3argILi1EEENSM_ILi2EEENSC_5valueISG_EENSP_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_f442a4() -> ! {
    todo!("0xf442a4 j___ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS5_S6_NS_8weak_ptrINS3_24KeyframeSequenceProviderEEENSE_INS3_16KeyframeSequenceEEEENSC_5list4INS_3argILi1EEENSM_ILi2EEENSC_5valueISG_EENSP_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xf442b4 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSJ_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEmRKT_RKT0_
// was: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const
pub fn stub_f442b4() -> ! {
    todo!("0xf442b4 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSJ_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEmRKT_RKT0_")
}

