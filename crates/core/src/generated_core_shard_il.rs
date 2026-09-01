//! core shard IL — 100 core stubs EA-sorted, continuation after IK 0x682a28 (EA-sorted ascending, next 100 uncovered).
//!
//! Source: `ida/export.json` filtered where demangled/mangled contains RBX::|boost, excludes Reflection|DataModel|Ogre|RakNet|Lua, EA-sorted, next 100 uncovered after 0x682a28.
//!
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>)")]
// 0x682a60 — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_4ToolENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueINS9_IS8_EEEENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
// type: int __fastcall(int, int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, int, int, int, int, int, int)
// was: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>)
pub fn stub_682a60() -> ! {
    todo!("0x682a60 rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>)")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Tool>,boost::arg<1>>::type> boost::bind<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Tool>,boost::arg<1>>(void (RBX::Tool::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Tool>,boost::arg<1>)")]
// 0x682c1c — __ZN5boost4bindIvN3RBX4ToolENS_10shared_ptrINS1_8InstanceEEENS3_IS2_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISB_T0_T1_EENS9_9list_av_2IT2_T3_E4typeEEEMSE_FSB_SF_ESI_SJ_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Tool>,boost::arg<1>>::type> boost::bind<void,RBX::Tool,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Tool>,boost::arg<1>>(void (RBX::Tool::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Tool>,boost::arg<1>)
pub fn stub_682c1c() -> ! {
    todo!("0x682c1c boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Tool>,boost::arg<1>>::type> boost::bind<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Tool>,boost::arg<1>>(void (RBX::Tool::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Tool>,boost::arg<1>)")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>> const&)")]
// 0x682d38 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_4ToolES6_EENSA_5list2INSA_5valueINS3_ISE_EEEENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>> const&)
pub fn stub_682d38() -> ! {
    todo!("0x682d38 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>> const&)")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>,RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>(void (RBX::Tool::*)(rbx_core::WeakPtr<RBX::Network::Player>),RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)")]
// 0x682e2c — __ZN5boost4bindIvN3RBX4ToolENS_8weak_ptrINS1_7Network6PlayerEEEPS2_S6_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISA_T0_T1_EENS8_9list_av_2IT2_T3_E4typeEEEMSD_FSA_SE_ESH_SI_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,boost::weak_ptr<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>,RBX::Tool*,boost::weak_ptr<RBX::Network::Player>>(void (RBX::Tool::*)(boost::weak_ptr<RBX::Network::Player>),RBX::Tool*,boost::weak_ptr<RBX::Network::Player>)
pub fn stub_682e2c() -> ! {
    todo!("0x682e2c boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>,RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>(void (RBX::Tool::*)(rbx_core::WeakPtr<RBX::Network::Player>),RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)")
}

#[doc(alias = "RBX::Tool::askAddChild(RBX::Instance const*)const")]
// 0x683008 — __ZNK3RBX4Tool11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const RBX::Instance *)
// was: RBX::Tool::askAddChild(RBX::Instance const*)const
pub fn stub_683008() -> ! {
    todo!("0x683008 RBX::Tool::askAddChild(RBX::Instance const*)const")
}

#[doc(alias = "RBX::Tool::askSetParent(RBX::Instance const*)const")]
// 0x68300c — __ZNK3RBX4Tool12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const RBX::Instance *)
// was: RBX::Tool::askSetParent(RBX::Instance const*)const
pub fn stub_68300c() -> ! {
    todo!("0x68300c RBX::Tool::askSetParent(RBX::Instance const*)const")
}

#[doc(alias = "RBX::Tool::canBePickedUpByPlayer(RBX::Network::Player *)")]
// 0x683034 — __ZN3RBX4Tool21canBePickedUpByPlayerEPNS_7Network6PlayerE
// was: RBX::Tool::canBePickedUpByPlayer(RBX::Network::Player *)
pub fn stub_683034() -> ! {
    todo!("0x683034 RBX::Tool::canBePickedUpByPlayer(RBX::Network::Player *)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Tool> RBX::Creatable<RBX::Instance>::create<RBX::Tool>(void)")]
// 0x6832b4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4ToolEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Tool> RBX::Creatable<RBX::Instance>::create<RBX::Tool>(void)
pub fn stub_6832b4() -> ! {
    todo!("0x6832b4 rbx_core::SharedPtr<RBX::Tool> RBX::Creatable<RBX::Instance>::create<RBX::Tool>(void)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Tool>::shared_ptr<RBX::Tool,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x683368 — __ZN5boost10shared_ptrIN3RBX4ToolEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Tool>::shared_ptr<RBX::Tool,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_683368() -> ! {
    todo!("0x683368 rbx_core::SharedPtr<RBX::Tool>::shared_ptr<RBX::Tool,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x68351c — __ZN5boost6detail12shared_countC2IPN3RBX4ToolENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_68351c() -> ! {
    todo!("0x68351c boost::detail::shared_count::shared_count<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x683624 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_683624() -> ! {
    todo!("0x683624 boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x683628 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_683628() -> ! {
    todo!("0x683628 boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x68362c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_68362c() -> ! {
    todo!("0x68362c boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x68364c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_68364c() -> ! {
    todo!("0x68364c boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x683664 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_683664() -> ! {
    todo!("0x683664 boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::Tool *>,boost::_bi::value<RBX::Backpack *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Tool *,RBX::Backpack *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Tool *,RBX::Backpack *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// 0x683e04 — __ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPN3RBX4ToolEEENS4_IPNS5_8BackpackEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES7_SA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::Tool *>,boost::_bi::value<RBX::Backpack *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::Tool *,RBX::Backpack *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Tool *,RBX::Backpack *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_683e04() -> ! {
    todo!("0x683e04 void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::Tool *>,boost::_bi::value<RBX::Backpack *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Tool *,RBX::Backpack *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Tool *,RBX::Backpack *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(rbx_core::WeakPtr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)")]
// 0x683ee0 — __ZN5boost10shared_ptrIN3RBX7Network6PlayerEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(boost::weak_ptr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)
pub fn stub_683ee0() -> ! {
    todo!("0x683ee0 rbx_core::SharedPtr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(rbx_core::WeakPtr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)")
}

#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x683f5c — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_683f5c() -> ! {
    todo!("0x683f5c __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x684044 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
pub fn stub_684044() -> ! {
    todo!("0x684044 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>)")]
// 0x684130 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>)
pub fn stub_684130() -> ! {
    todo!("0x684130 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>)")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x68422c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_68422c() -> ! {
    todo!("0x68422c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x684248 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_684248() -> ! {
    todo!("0x684248 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &)const")]
// 0x684260 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &)const
pub fn stub_684260() -> ! {
    todo!("0x684260 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &)const")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x68434c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_68434c() -> ! {
    todo!("0x68434c bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x684434 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_684434() -> ! {
    todo!("0x684434 void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>> &,boost::_bi::list0 &,int)")]
// 0x68450c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEclINS_4_mfi3mf1IvS4_SA_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>> &,boost::_bi::list0 &,int)
pub fn stub_68450c() -> ! {
    todo!("0x68450c void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>> &,boost::_bi::list0 &,int)")
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>::operator()(RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)const")]
// 0x6845e0 — __ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS2_7Network6PlayerEEEEclEPS3_S7_
// was: boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>::operator()(RBX::Tool*,boost::weak_ptr<RBX::Network::Player>)const
pub fn stub_6845e0() -> ! {
    todo!("0x6845e0 boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>::operator()(RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)const")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x6846c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_6846c8() -> ! {
    todo!("0x6846c8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>)")]
// 0x684824 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEC2ES6_SB_
// was: boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>)
pub fn stub_684824() -> ! {
    todo!("0x684824 boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>)")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*)")]
// 0x68490c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_EC2IPS9_EERKSO_T_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*)
pub fn stub_68490c() -> ! {
    todo!("0x68490c rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*)")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>::~callable_slot()")]
// 0x684a2c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_4ToolES6_EENSA_5list2INSA_5valueINS3_ISE_EEEENS2_3argILi1EEEEEEEED1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>::~callable_slot()
pub fn stub_684a2c() -> ! {
    todo!("0x684a2c rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>::~callable_slot()")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>::~callable_slot()")]
// 0x684b40 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_4ToolES6_EENSA_5list2INSA_5valueINS3_ISE_EEEENS2_3argILi1EEEEEEEED0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>::~callable_slot()
pub fn stub_684b40() -> ! {
    todo!("0x684b40 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>::~callable_slot()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x684c70 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_684c70() -> ! {
    todo!("0x684c70 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x684c8c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_684c8c() -> ! {
    todo!("0x684c8c non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// 0x684ca8 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_NS3_INS4_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_684ca8() -> ! {
    todo!("0x684ca8 void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")
}

#[doc(alias = "void boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>::call<rbx_core::SharedPtr<RBX::Tool>,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Tool> &,void const*,rbx_core::SharedPtr<RBX::Instance> &)const")]
// 0x684d80 — __ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS2_8InstanceEEEE4callINS4_IS3_EES6_EEvRT_PKvRT0_
// type: void __fastcall(char **, _DWORD *, int, const shared_count *)
// was: void boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>::call<boost::shared_ptr<RBX::Tool>,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Tool> &,void const*,boost::shared_ptr<RBX::Instance> &)const
pub fn stub_684d80() -> ! {
    todo!("0x684d80 void boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>::call<rbx_core::SharedPtr<RBX::Tool>,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Tool> &,void const*,rbx_core::SharedPtr<RBX::Instance> &)const")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// 0x684e68 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_684e68() -> ! {
    todo!("0x684e68 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// 0x684f7c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_684f7c() -> ! {
    todo!("0x684f7c rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS8_5list2INS8_5valueINS1_ISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x6850ac — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS8_5list2INS8_5valueINS1_ISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS8_5list2INS8_5valueINS1_ISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_6850ac() -> ! {
    todo!("0x6850ac __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS8_5list2INS8_5valueINS1_ISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS7_5list2INS7_5valueINS1_ISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x685190 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS7_5list2INS7_5valueINS1_ISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS7_5list2INS7_5valueINS1_ISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
pub fn stub_685190() -> ! {
    todo!("0x685190 __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS7_5list2INS7_5valueINS1_ISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>)")]
// 0x685278 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS7_5list2INS7_5valueINS1_ISB_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>)
pub fn stub_685278() -> ! {
    todo!("0x685278 void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>)")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x685370 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINS9_IS8_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_685370() -> ! {
    todo!("0x685370 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x68538c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINS9_IS8_EEEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
pub fn stub_68538c() -> ! {
    todo!("0x68538c boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0x6853a8 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_4ToolES6_EENS9_5list2INS9_5valueINS3_ISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_6853a8() -> ! {
    todo!("0x6853a8 bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x685490 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_4ToolES6_EENS9_5list2INS9_5valueINS3_ISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_685490() -> ! {
    todo!("0x685490 bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x685574 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_4ToolES6_EENS9_5list2INS9_5valueINS3_ISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_685574() -> ! {
    todo!("0x685574 void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x685648 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINS9_IS8_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_685648() -> ! {
    todo!("0x685648 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Mouse>::shared_ptr<RBX::Mouse,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x685950 — __ZN5boost10shared_ptrIN3RBX5MouseEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Mouse>::shared_ptr<RBX::Mouse,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_685950() -> ! {
    todo!("0x685950 rbx_core::SharedPtr<RBX::Mouse>::shared_ptr<RBX::Mouse,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x685b04 — __ZN5boost6detail12shared_countC2IPN3RBX5MouseENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_685b04() -> ! {
    todo!("0x685b04 boost::detail::shared_count::shared_count<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x685c0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_685c0c() -> ! {
    todo!("0x685c0c boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x685c10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_685c10() -> ! {
    todo!("0x685c10 boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x685c14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_685c14() -> ! {
    todo!("0x685c14 boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x685c34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_685c34() -> ! {
    todo!("0x685c34 boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x685c4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_685c4c() -> ! {
    todo!("0x685c4c boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

#[doc(alias = "rbx::signals::connection RBX::Tool::special_equipped_signal::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const &)")]
// 0x68779c — __ZN3RBX4Tool23special_equipped_signal7connectIKN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEEEN3rbx7signals10connectionERT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: rbx::signals::connection RBX::Tool::special_equipped_signal::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const &)
pub fn stub_68779c() -> ! {
    todo!("0x68779c rbx::signals::connection RBX::Tool::special_equipped_signal::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const &)")
}

#[doc(alias = "RBX::HopperBin const* RBX::Instance::findConstFirstChildOfType<RBX::HopperBin>(void)const")]
// 0x688200 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_9HopperBinEEEPKT_v
// was: RBX::HopperBin const* RBX::Instance::findConstFirstChildOfType<RBX::HopperBin>(void)const
pub fn stub_688200() -> ! {
    todo!("0x688200 RBX::HopperBin const* RBX::Instance::findConstFirstChildOfType<RBX::HopperBin>(void)const")
}

#[doc(alias = "RBX::ToolMouseCommand::ToolMouseCommand(RBX::Workspace *,RBX::Tool *)")]
// 0x688b04 — __ZN3RBX16ToolMouseCommandC1EPNS_9WorkspaceEPNS_4ToolE
// type: _DWORD __fastcall(RBX::ToolMouseCommand *__hidden this, RBX::Workspace *, RBX::Tool *)
// was: RBX::ToolMouseCommand::ToolMouseCommand(RBX::Workspace *,RBX::Tool *)
pub fn stub_688b04() -> ! {
    todo!("0x688b04 RBX::ToolMouseCommand::ToolMouseCommand(RBX::Workspace *,RBX::Tool *)")
}

#[doc(alias = "RBX::ToolMouseCommand::ToolMouseCommand(RBX::Workspace *,RBX::Tool *)")]
// 0x688b08 — __ZN3RBX16ToolMouseCommandC2EPNS_9WorkspaceEPNS_4ToolE
// type: _DWORD __fastcall(RBX::ToolMouseCommand *__hidden this, RBX::Workspace *, RBX::Tool *)
// was: RBX::ToolMouseCommand::ToolMouseCommand(RBX::Workspace *,RBX::Tool *)
pub fn stub_688b08() -> ! {
    todo!("0x688b08 RBX::ToolMouseCommand::ToolMouseCommand(RBX::Workspace *,RBX::Tool *)")
}

#[doc(alias = "RBX::ToolMouseCommand::tryClickable(RBX::UIEvent const&,rbx_core::SharedPtr<RBX::PartInstance>)")]
// 0x688f6c — __ZN3RBX16ToolMouseCommand12tryClickableERKNS_7UIEventEN5boost10shared_ptrINS_12PartInstanceEEE
// type: int __fastcall(RBX::Network::Players **, RBX::Instance *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::ToolMouseCommand::tryClickable(RBX::UIEvent const&,boost::shared_ptr<RBX::PartInstance>)
pub fn stub_688f6c() -> ! {
    todo!("0x688f6c RBX::ToolMouseCommand::tryClickable(RBX::UIEvent const&,rbx_core::SharedPtr<RBX::PartInstance>)")
}

#[doc(alias = "RBX::ModelTool::ModelTool(RBX::Workspace *)")]
// 0x689a5c — __ZN3RBX9ModelToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::ModelTool *__hidden this, RBX::Workspace *)
// was: RBX::ModelTool::ModelTool(RBX::Workspace *)
pub fn stub_689a5c() -> ! {
    todo!("0x689a5c RBX::ModelTool::ModelTool(RBX::Workspace *)")
}

#[doc(alias = "RBX::anchorAllChildren(rbx_core::SharedPtr<RBX::Instance>,bool const&)")]
// 0x689e74 — __ZN3RBX17anchorAllChildrenEN5boost10shared_ptrINS_8InstanceEEERKb
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: RBX::anchorAllChildren(boost::shared_ptr<RBX::Instance>,bool const&)
pub fn stub_689e74() -> ! {
    todo!("0x689e74 RBX::anchorAllChildren(rbx_core::SharedPtr<RBX::Instance>,bool const&)")
}

#[doc(alias = "RBX::HasUnAnchoredNode(RBX::Instance *)")]
// 0x68a490 — __ZN3RBXL17HasUnAnchoredNodeEPNS_8InstanceE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Instance *)
// was: RBX::HasUnAnchoredNode(RBX::Instance *)
pub fn stub_68a490() -> ! {
    todo!("0x68a490 RBX::HasUnAnchoredNode(RBX::Instance *)")
}

#[doc(alias = "RBX::AnchorNode::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x68a5c8 — __ZN3RBX10AnchorNodeclEN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::AnchorNode::operator()(boost::shared_ptr<RBX::Instance>)
pub fn stub_68a5c8() -> ! {
    todo!("0x68a5c8 RBX::AnchorNode::operator()(rbx_core::SharedPtr<RBX::Instance>)")
}

#[doc(alias = "void RBX::Instance::visitChildren<RBX::AnchorNode>(RBX::AnchorNode const&)const")]
// 0x68a8bc — __ZNK3RBX8Instance13visitChildrenINS_10AnchorNodeEEEvRKT_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void RBX::Instance::visitChildren<RBX::AnchorNode>(RBX::AnchorNode const&)const
pub fn stub_68a8bc() -> ! {
    todo!("0x68a8bc void RBX::Instance::visitChildren<RBX::AnchorNode>(RBX::AnchorNode const&)const")
}

#[doc(alias = "RBX::PartTool::PartTool(RBX::Workspace *)")]
// 0x68ac9c — __ZN3RBX8PartToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::PartTool *__hidden this, RBX::Workspace *)
// was: RBX::PartTool::PartTool(RBX::Workspace *)
pub fn stub_68ac9c() -> ! {
    todo!("0x68ac9c RBX::PartTool::PartTool(RBX::Workspace *)")
}

#[doc(alias = "RBX::SurfaceTool::SurfaceTool(RBX::Workspace *)")]
// 0x68bb74 — __ZN3RBX11SurfaceToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::SurfaceTool *__hidden this, RBX::Workspace *)
// was: RBX::SurfaceTool::SurfaceTool(RBX::Workspace *)
pub fn stub_68bb74() -> ! {
    todo!("0x68bb74 RBX::SurfaceTool::SurfaceTool(RBX::Workspace *)")
}

#[doc(alias = "RBX::TouchTransmitter::checkTouch(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// 0x68e068 — __ZN3RBX16TouchTransmitter10checkTouchERKN5boost10shared_ptrINS_12PartInstanceEEE
// was: RBX::TouchTransmitter::checkTouch(boost::shared_ptr<RBX::PartInstance> const&)
pub fn stub_68e068() -> ! {
    todo!("0x68e068 RBX::TouchTransmitter::checkTouch(rbx_core::SharedPtr<RBX::PartInstance> const&)")
}

#[doc(alias = "RBX::TouchTransmitter::checkUntouch(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// 0x68e078 — __ZN3RBX16TouchTransmitter12checkUntouchERKN5boost10shared_ptrINS_12PartInstanceEEE
// was: RBX::TouchTransmitter::checkUntouch(boost::shared_ptr<RBX::PartInstance> const&)
pub fn stub_68e078() -> ! {
    todo!("0x68e078 RBX::TouchTransmitter::checkUntouch(rbx_core::SharedPtr<RBX::PartInstance> const&)")
}

#[doc(alias = "RBX::TouchDebouncer::check(rbx_core::SharedPtr<RBX::PartInstance> const&,RBX::TouchPair::Type)")]
// 0x68e088 — __ZN3RBX14TouchDebouncer5checkERKN5boost10shared_ptrINS_12PartInstanceEEENS_9TouchPair4TypeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int)
// was: RBX::TouchDebouncer::check(boost::shared_ptr<RBX::PartInstance> const&,RBX::TouchPair::Type)
pub fn stub_68e088() -> ! {
    todo!("0x68e088 RBX::TouchDebouncer::check(rbx_core::SharedPtr<RBX::PartInstance> const&,RBX::TouchPair::Type)")
}

#[doc(alias = "RBX::ButtonBindingWidget::askAddChild(RBX::Instance const*)const")]
// 0x690de0 — __ZNK3RBX19ButtonBindingWidget11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::ButtonBindingWidget *__hidden this, const RBX::Instance *)
// was: RBX::ButtonBindingWidget::askAddChild(RBX::Instance const*)const
pub fn stub_690de0() -> ! {
    todo!("0x690de0 RBX::ButtonBindingWidget::askAddChild(RBX::Instance const*)const")
}

#[doc(alias = "RBX::ButtonBindingWidget::askSetParent(RBX::Instance const*)const")]
// 0x690de4 — __ZNK3RBX19ButtonBindingWidget12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::ButtonBindingWidget *__hidden this, const RBX::Instance *)
// was: RBX::ButtonBindingWidget::askSetParent(RBX::Instance const*)const
pub fn stub_690de4() -> ! {
    todo!("0x690de4 RBX::ButtonBindingWidget::askSetParent(RBX::Instance const*)const")
}

#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::find<RBX::ControllerService>(RBX::Instance const*)")]
// 0x69343c — __ZN3RBX15ServiceProvider4findINS_17ControllerServiceEEEPT_PKNS_8InstanceE
// was: RBX::ControllerService * RBX::ServiceProvider::find<RBX::ControllerService>(RBX::Instance const*)
pub fn stub_69343c() -> ! {
    todo!("0x69343c RBX::ControllerService * RBX::ServiceProvider::find<RBX::ControllerService>(RBX::Instance const*)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ButtonBindingWidget> RBX::Creatable<RBX::Instance>::create<RBX::ButtonBindingWidget,RBX::Controller::Button,RBX::Controller*>(RBX::Controller::Button,RBX::Controller*)")]
// 0x693838 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19ButtonBindingWidgetENS_10Controller6ButtonEPS5_EEN5boost10shared_ptrIT_EET0_T1_
// type: int __fastcall(std::string *, int, int, int, int)
// was: boost::shared_ptr<RBX::ButtonBindingWidget> RBX::Creatable<RBX::Instance>::create<RBX::ButtonBindingWidget,RBX::Controller::Button,RBX::Controller*>(RBX::Controller::Button,RBX::Controller*)
pub fn stub_693838() -> ! {
    todo!("0x693838 rbx_core::SharedPtr<RBX::ButtonBindingWidget> RBX::Creatable<RBX::Instance>::create<RBX::ButtonBindingWidget,RBX::Controller::Button,RBX::Controller*>(RBX::Controller::Button,RBX::Controller*)")
}

#[doc(alias = "RBX::UserInputService * RBX::ServiceProvider::create<RBX::UserInputService>(RBX::Instance const*)")]
// 0x693da4 — __ZN3RBX15ServiceProvider6createINS_16UserInputServiceEEEPT_PKNS_8InstanceE
// was: RBX::UserInputService * RBX::ServiceProvider::create<RBX::UserInputService>(RBX::Instance const*)
pub fn stub_693da4() -> ! {
    todo!("0x693da4 RBX::UserInputService * RBX::ServiceProvider::create<RBX::UserInputService>(RBX::Instance const*)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HumanoidController> RBX::Creatable<RBX::Instance>::create<RBX::HumanoidController>(void)")]
// 0x693dbc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18HumanoidControllerEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::HumanoidController> RBX::Creatable<RBX::Instance>::create<RBX::HumanoidController>(void)
pub fn stub_693dbc() -> ! {
    todo!("0x693dbc rbx_core::SharedPtr<RBX::HumanoidController> RBX::Creatable<RBX::Instance>::create<RBX::HumanoidController>(void)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VehicleController> RBX::Creatable<RBX::Instance>::create<RBX::VehicleController>(void)")]
// 0x6963a0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17VehicleControllerEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::VehicleController> RBX::Creatable<RBX::Instance>::create<RBX::VehicleController>(void)
pub fn stub_6963a0() -> ! {
    todo!("0x6963a0 rbx_core::SharedPtr<RBX::VehicleController> RBX::Creatable<RBX::Instance>::create<RBX::VehicleController>(void)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VehicleController>::shared_ptr<RBX::VehicleController,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x696450 — __ZN5boost10shared_ptrIN3RBX17VehicleControllerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::VehicleController>::shared_ptr<RBX::VehicleController,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_696450() -> ! {
    todo!("0x696450 rbx_core::SharedPtr<RBX::VehicleController>::shared_ptr<RBX::VehicleController,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x696604 — __ZN5boost6detail12shared_countC2IPN3RBX17VehicleControllerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_696604() -> ! {
    todo!("0x696604 boost::detail::shared_count::shared_count<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x69670c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17VehicleControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_69670c() -> ! {
    todo!("0x69670c boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x696710 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17VehicleControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_696710() -> ! {
    todo!("0x696710 boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x696714 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17VehicleControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_696714() -> ! {
    todo!("0x696714 boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x696738 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17VehicleControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_696738() -> ! {
    todo!("0x696738 boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x696750 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17VehicleControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_696750() -> ! {
    todo!("0x696750 boost::detail::sp_counted_impl_pd<RBX::VehicleController *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HumanoidController>::shared_ptr<RBX::HumanoidController,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x696bd8 — __ZN5boost10shared_ptrIN3RBX18HumanoidControllerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::HumanoidController>::shared_ptr<RBX::HumanoidController,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_696bd8() -> ! {
    todo!("0x696bd8 rbx_core::SharedPtr<RBX::HumanoidController>::shared_ptr<RBX::HumanoidController,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x696d8c — __ZN5boost6detail12shared_countC2IPN3RBX18HumanoidControllerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_696d8c() -> ! {
    todo!("0x696d8c boost::detail::shared_count::shared_count<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x696e94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HumanoidControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_696e94() -> ! {
    todo!("0x696e94 boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x696e98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HumanoidControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_696e98() -> ! {
    todo!("0x696e98 boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x696e9c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HumanoidControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_696e9c() -> ! {
    todo!("0x696e9c boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x696ebc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HumanoidControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_696ebc() -> ! {
    todo!("0x696ebc boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x696ed4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HumanoidControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_696ed4() -> ! {
    todo!("0x696ed4 boost::detail::sp_counted_impl_pd<RBX::HumanoidController *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

#[doc(alias = "RBX::ServiceProvider::findServiceProvider(RBX::Instance const*)")]
// 0x6971f0 — __ZN3RBX15ServiceProvider19findServiceProviderEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::ServiceProvider *__hidden this, const RBX::Instance *)
// was: RBX::ServiceProvider::findServiceProvider(RBX::Instance const*)
pub fn stub_6971f0() -> ! {
    todo!("0x6971f0 RBX::ServiceProvider::findServiceProvider(RBX::Instance const*)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::UserInputService> RBX::Creatable<RBX::Instance>::create<RBX::UserInputService>(void)")]
// 0x697230 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16UserInputServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::UserInputService> RBX::Creatable<RBX::Instance>::create<RBX::UserInputService>(void)
pub fn stub_697230() -> ! {
    todo!("0x697230 rbx_core::SharedPtr<RBX::UserInputService> RBX::Creatable<RBX::Instance>::create<RBX::UserInputService>(void)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GameBasicSettings> RBX::Creatable<RBX::Instance>::create<RBX::GameBasicSettings>(void)")]
// 0x6973e8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17GameBasicSettingsEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::GameBasicSettings> RBX::Creatable<RBX::Instance>::create<RBX::GameBasicSettings>(void)
pub fn stub_6973e8() -> ! {
    todo!("0x6973e8 rbx_core::SharedPtr<RBX::GameBasicSettings> RBX::Creatable<RBX::Instance>::create<RBX::GameBasicSettings>(void)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GameBasicSettings>::shared_ptr<RBX::GameBasicSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x697498 — __ZN5boost10shared_ptrIN3RBX17GameBasicSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::GameBasicSettings>::shared_ptr<RBX::GameBasicSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_697498() -> ! {
    todo!("0x697498 rbx_core::SharedPtr<RBX::GameBasicSettings>::shared_ptr<RBX::GameBasicSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x697650 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_697650() -> ! {
    todo!("0x697650 boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ButtonBindingWidget>::shared_ptr<RBX::ButtonBindingWidget,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x698070 — __ZN5boost10shared_ptrIN3RBX19ButtonBindingWidgetEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ButtonBindingWidget>::shared_ptr<RBX::ButtonBindingWidget,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_698070() -> ! {
    todo!("0x698070 rbx_core::SharedPtr<RBX::ButtonBindingWidget>::shared_ptr<RBX::ButtonBindingWidget,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x698224 — __ZN5boost6detail12shared_countC2IPN3RBX19ButtonBindingWidgetENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_698224() -> ! {
    todo!("0x698224 boost::detail::shared_count::shared_count<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x69832c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ButtonBindingWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_69832c() -> ! {
    todo!("0x69832c boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x698330 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ButtonBindingWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_698330() -> ! {
    todo!("0x698330 boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x698334 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ButtonBindingWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_698334() -> ! {
    todo!("0x698334 boost::detail::sp_counted_impl_pd<RBX::ButtonBindingWidget *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}
