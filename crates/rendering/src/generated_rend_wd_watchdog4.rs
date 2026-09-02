//! rendering shard rend_wd_watchdog4 — 120 stubs 0x7d75cc..0x7dd388 EA-sorted asc gap filler not yet in crates/rendering/src (Ogre/G3D/Render 17124 total filtered, 376 uncovered -> global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA asc Ogre/G3D/Render-filtered then global gap filler not yet in crates/rendering/src — next 120 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7d75cc — __ZN3RBX5HUMAN17StrafingNoPhysicsD0Ev
// type: void __fastcall(RBX::HUMAN::StrafingNoPhysics *__hidden this)
#[doc(alias = "RBX::HUMAN::StrafingNoPhysics::~StrafingNoPhysics()")]
#[doc(alias = "__ZN3RBX5HUMAN17StrafingNoPhysicsD0Ev")]
pub fn stub_7d75cc() -> ! {
    todo!("0x7d75cc RBX::HUMAN::StrafingNoPhysics::~StrafingNoPhysics()")
}


// 0x7d766c — __ZNK3RBX5HUMAN17StrafingNoPhysics12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::StrafingNoPhysics *__hidden this)
#[doc(alias = "RBX::HUMAN::StrafingNoPhysics::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN17StrafingNoPhysics12getStateTypeEv")]
pub fn stub_7d766c() -> ! {
    todo!("0x7d766c RBX::HUMAN::StrafingNoPhysics::getStateType(void)const")
}


// 0x7d7670 — __ZThn4_N3RBX5HUMAN17StrafingNoPhysicsD1Ev
// type: void __fastcall(RBX::HUMAN::StrafingNoPhysics *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::StrafingNoPhysics::~StrafingNoPhysics()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN17StrafingNoPhysicsD1Ev")]
pub fn stub_7d7670() -> ! {
    todo!("0x7d7670 non-virtual thunk toRBX::HUMAN::StrafingNoPhysics::~StrafingNoPhysics()")
}


// 0x7d7678 — __ZThn4_N3RBX5HUMAN17StrafingNoPhysicsD0Ev
// type: void __fastcall(RBX::HUMAN::StrafingNoPhysics *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::StrafingNoPhysics::~StrafingNoPhysics()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN17StrafingNoPhysicsD0Ev")]
pub fn stub_7d7678() -> ! {
    todo!("0x7d7678 non-virtual thunk toRBX::HUMAN::StrafingNoPhysics::~StrafingNoPhysics()")
}


// 0x7d771c — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEEvv")]
pub fn stub_7d771c() -> ! {
    todo!("0x7d771c __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEEvv")
}


// 0x7d7720 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEERKS0_v")]
pub fn stub_7d7720() -> ! {
    todo!("0x7d7720 __ZN3RBX4Name9doDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEERKS0_v")
}


// 0x7d7800 — __GLOBAL__I_a_384
#[doc(alias = "global constructor keyed to_a_384")]
#[doc(alias = "__GLOBAL__I_a_384")]
pub fn stub_7d7800() -> ! {
    todo!("0x7d7800 global constructor keyed to_a_384")
}


// 0x7d7a3c — __ZN3RBX10Reflection18CallbackDescriptorC2ERNS0_15ClassDescriptorEPKcNS0_10Descriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::CallbackDescriptor::CallbackDescriptor(RBX::Reflection::ClassDescriptor &,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18CallbackDescriptorC2ERNS0_15ClassDescriptorEPKcNS0_10Descriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_7d7a3c() -> ! {
    todo!("0x7d7a3c __ZN3RBX10Reflection18CallbackDescriptorC2ERNS0_15ClassDescriptorEPKcNS0_10Descriptor10AttributesENS_8Security11PermissionsE")
}


// 0x7d7b58 — __ZNK3RBX10Reflection18CallbackDescriptor24setGenericCallbackHelperEPNS0_13DescribedBaseERKN5boost8functionIFNS4_10shared_ptrINS0_5TupleEEENS6_IKS7_EEEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::CallbackDescriptor::setGenericCallbackHelper(RBX::Reflection::DescribedBase *,boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18CallbackDescriptor24setGenericCallbackHelperEPNS0_13DescribedBaseERKN5boost8functionIFNS4_10shared_ptrINS0_5TupleEEENS6_IKS7_EEEEE")]
// was: RBX::Reflection::CallbackDescriptor::setGenericCallbackHelper(RBX::Reflection::DescribedBase *,boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> const&)const — uses rbx_core::SharedPtr not boost
pub fn stub_7d7b58() -> ! {
    todo!("0x7d7b58 __ZNK3RBX10Reflection18CallbackDescriptor24setGenericCallbackHelperEPNS0_13DescribedBaseERKN5boost8functionIFNS4_10shared_ptrINS0_5TupleEEENS6_IKS7_EEEEE")
}


// 0x7d7c98 — __ZN5boost10shared_ptrINS_8functionIFNS0_IN3RBX10Reflection5TupleEEENS0_IKS4_EEEEEEC2IS9_EEPT_
#[doc(alias = "boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>(boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> *)")]
#[doc(alias = "__ZN5boost10shared_ptrINS_8functionIFNS0_IN3RBX10Reflection5TupleEEENS0_IKS4_EEEEEEC2IS9_EEPT_")]
// was: boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>(boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> *) — uses rbx_core::SharedPtr not boost
pub fn stub_7d7c98() -> ! {
    todo!("0x7d7c98 __ZN5boost10shared_ptrINS_8functionIFNS0_IN3RBX10Reflection5TupleEEENS0_IKS4_EEEEEEC2IS9_EEPT_")
}


// 0x7d7d70 — __ZN5boost6detail12shared_countC2INS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS4_IKS7_EEEEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>(boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2INS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS4_IKS7_EEEEEEEPT_")]
// was: boost::detail::shared_count::shared_count<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>(boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> *) — uses rbx_core::SharedPtr not boost
pub fn stub_7d7d70() -> ! {
    todo!("0x7d7d70 __ZN5boost6detail12shared_countC2INS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS4_IKS7_EEEEEEEPT_")
}


// 0x7d7e7c — __ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEE5clearEv
#[doc(alias = "boost::function1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::clear(void)")]
#[doc(alias = "__ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEE5clearEv")]
// was: boost::function1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::clear(void) — uses rbx_core::SharedPtr not boost
pub fn stub_7d7e7c() -> ! {
    todo!("0x7d7e7c __ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEE5clearEv")
}


// 0x7d7ea8 — __ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEED1Ev")]
// was: boost::detail::sp_counted_impl_p<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::~sp_counted_impl_p() — uses rbx_core::SharedPtr not boost
pub fn stub_7d7ea8() -> ! {
    todo!("0x7d7ea8 __ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEED1Ev")
}


// 0x7d7eac — __ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEED0Ev")]
// was: boost::detail::sp_counted_impl_p<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::~sp_counted_impl_p() — uses rbx_core::SharedPtr not boost
pub fn stub_7d7eac() -> ! {
    todo!("0x7d7eac __ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEED0Ev")
}


// 0x7d7eb0 — __ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEE7disposeEv")]
// was: boost::detail::sp_counted_impl_p<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::dispose(void) — uses rbx_core::SharedPtr not boost
pub fn stub_7d7eb0() -> ! {
    todo!("0x7d7eb0 __ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEE7disposeEv")
}


// 0x7d7f58 — __ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEE11get_deleterERKSt9type_info")]
// was: boost::detail::sp_counted_impl_p<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::get_deleter(std::type_info const&) — uses rbx_core::SharedPtr not boost
pub fn stub_7d7f58() -> ! {
    todo!("0x7d7f58 __ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEE11get_deleterERKSt9type_info")
}


// 0x7d7f5c — __ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEE19get_untyped_deleterEv")]
// was: boost::detail::sp_counted_impl_p<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::get_untyped_deleter(void) — uses rbx_core::SharedPtr not boost
pub fn stub_7d7f5c() -> ! {
    todo!("0x7d7f5c __ZN5boost6detail17sp_counted_impl_pINS_8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEEEEE19get_untyped_deleterEv")
}


// 0x7d7f60 — __ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEE13assign_to_ownERKS8_
#[doc(alias = "boost::function1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to_own(boost::function1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>> const&)")]
#[doc(alias = "__ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEE13assign_to_ownERKS8_")]
// was: boost::function1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to_own(boost::function1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>> const&) — uses rbx_core::SharedPtr not boost
pub fn stub_7d7f60() -> ! {
    todo!("0x7d7f60 __ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEE13assign_to_ownERKS8_")
}


// 0x7d7f90 — __GLOBAL__I_a_385
#[doc(alias = "global constructor keyed to_a_385")]
#[doc(alias = "__GLOBAL__I_a_385")]
pub fn stub_7d7f90() -> ! {
    todo!("0x7d7f90 global constructor keyed to_a_385")
}


// 0x7d80c0 — __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEEC1Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEEC1Ev")]
pub fn stub_7d80c0() -> ! {
    todo!("0x7d80c0 RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::EnumDesc(void)")
}


// 0x7d80c4 — __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEEC2Ev")]
pub fn stub_7d80c4() -> ! {
    todo!("0x7d80c4 RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::EnumDesc(void)")
}


// 0x7d833c — __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEEC1Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService4FontEEC1Ev")]
pub fn stub_7d833c() -> ! {
    todo!("0x7d833c RBX::Reflection::EnumDesc<RBX::TextService::Font>::EnumDesc(void)")
}


// 0x7d8340 — __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService4FontEEC2Ev")]
pub fn stub_7d8340() -> ! {
    todo!("0x7d8340 RBX::Reflection::EnumDesc<RBX::TextService::Font>::EnumDesc(void)")
}


// 0x7d8544 — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEEC1Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEEC1Ev")]
pub fn stub_7d8544() -> ! {
    todo!("0x7d8544 RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::EnumDesc(void)")
}


// 0x7d8548 — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEEC2Ev")]
pub fn stub_7d8548() -> ! {
    todo!("0x7d8548 RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::EnumDesc(void)")
}


// 0x7d8720 — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEEC1Ev")]
pub fn stub_7d8720() -> ! {
    todo!("0x7d8720 RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::EnumDesc(void)")
}


// 0x7d8724 — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEEC2Ev")]
pub fn stub_7d8724() -> ! {
    todo!("0x7d8724 RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::EnumDesc(void)")
}


// 0x7d88fc — __ZN3RBX11TextService12FromTextFontENS_4Text4FontE
// type: unsigned int __fastcall(unsigned int, int, int)
#[doc(alias = "RBX::TextService::FromTextFont(RBX::Text::Font)")]
#[doc(alias = "__ZN3RBX11TextService12FromTextFontENS_4Text4FontE")]
pub fn stub_7d88fc() -> ! {
    todo!("0x7d88fc RBX::TextService::FromTextFont(RBX::Text::Font)")
}


// 0x7d895c — __ZN3RBX11TextService10ToTextFontENS0_4FontE
#[doc(alias = "RBX::TextService::ToTextFont(RBX::TextService::Font)")]
#[doc(alias = "__ZN3RBX11TextService10ToTextFontENS0_4FontE")]
pub fn stub_7d895c() -> ! {
    todo!("0x7d895c RBX::TextService::ToTextFont(RBX::TextService::Font)")
}


// 0x7d89bc — __ZN3RBX11TextService12ToTextXAlignENS0_10XAlignmentE
#[doc(alias = "RBX::TextService::ToTextXAlign(RBX::TextService::XAlignment)")]
#[doc(alias = "__ZN3RBX11TextService12ToTextXAlignENS0_10XAlignmentE")]
pub fn stub_7d89bc() -> ! {
    todo!("0x7d89bc RBX::TextService::ToTextXAlign(RBX::TextService::XAlignment)")
}


// 0x7d8a30 — __ZN3RBX11TextService12ToTextYAlignENS0_10YAlignmentE
#[doc(alias = "RBX::TextService::ToTextYAlign(RBX::TextService::YAlignment)")]
#[doc(alias = "__ZN3RBX11TextService12ToTextYAlignENS0_10YAlignmentE")]
pub fn stub_7d8a30() -> ! {
    todo!("0x7d8a30 RBX::TextService::ToTextYAlign(RBX::TextService::YAlignment)")
}


// 0x7d8a90 — __ZN3RBX11TextServiceC1Ev
// type: _DWORD __fastcall(RBX::TextService *__hidden this)
#[doc(alias = "RBX::TextService::TextService(void)")]
#[doc(alias = "__ZN3RBX11TextServiceC1Ev")]
pub fn stub_7d8a90() -> ! {
    todo!("0x7d8a90 RBX::TextService::TextService(void)")
}


// 0x7d8a94 — __ZN3RBX11TextServiceC2Ev
// type: _DWORD __fastcall(RBX::TextService *__hidden this)
#[doc(alias = "RBX::TextService::TextService(void)")]
#[doc(alias = "__ZN3RBX11TextServiceC2Ev")]
pub fn stub_7d8a94() -> ! {
    todo!("0x7d8a94 RBX::TextService::TextService(void)")
}


// 0x7d8d24 — __ZN3RBX11TextService16clearTypesettersEv
// type: _DWORD __fastcall(RBX::TextService *__hidden this)
#[doc(alias = "RBX::TextService::clearTypesetters(void)")]
#[doc(alias = "__ZN3RBX11TextService16clearTypesettersEv")]
pub fn stub_7d8d24() -> ! {
    todo!("0x7d8d24 RBX::TextService::clearTypesetters(void)")
}


// 0x7d8e48 — __ZN3RBX11TextService18registerTypesetterENS0_4FontEN5boost10shared_ptrINS_10TypesetterEEE
#[doc(alias = "RBX::TextService::registerTypesetter(RBX::TextService::Font,boost::shared_ptr<RBX::Typesetter>)")]
#[doc(alias = "__ZN3RBX11TextService18registerTypesetterENS0_4FontEN5boost10shared_ptrINS_10TypesetterEEE")]
// was: RBX::TextService::registerTypesetter(RBX::TextService::Font,boost::shared_ptr<RBX::Typesetter>) — uses rbx_core::SharedPtr not boost
pub fn stub_7d8e48() -> ! {
    todo!("0x7d8e48 RBX::TextService::registerTypesetter(RBX::TextService::Font,boost::shared_ptr<RBX::Typesetter>)")
}


// 0x7d8eb8 — __ZN3RBX11TextService13getTypesetterENS0_4FontE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::TextService::getTypesetter(RBX::TextService::Font)")]
#[doc(alias = "__ZN3RBX11TextService13getTypesetterENS0_4FontE")]
pub fn stub_7d8eb8() -> ! {
    todo!("0x7d8eb8 RBX::TextService::getTypesetter(RBX::TextService::Font)")
}


// 0x7d8f38 — __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::addPair(RBX::TextService::FontSize,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE7addPairES3_PKc")]
pub fn stub_7d8f38() -> ! {
    todo!("0x7d8f38 RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::addPair(RBX::TextService::FontSize,char const*)")
}


// 0x7d9298 — __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::addPair(RBX::TextService::Font,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService4FontEE7addPairES3_PKc")]
pub fn stub_7d9298() -> ! {
    todo!("0x7d9298 RBX::Reflection::EnumDesc<RBX::TextService::Font>::addPair(RBX::TextService::Font,char const*)")
}


// 0x7d95f8 — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::addPair(RBX::TextService::XAlignment,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE7addPairES3_PKc")]
pub fn stub_7d95f8() -> ! {
    todo!("0x7d95f8 RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::addPair(RBX::TextService::XAlignment,char const*)")
}


// 0x7d9958 — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::addPair(RBX::TextService::YAlignment,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE7addPairES3_PKc")]
pub fn stub_7d9958() -> ! {
    todo!("0x7d9958 RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::addPair(RBX::TextService::YAlignment,char const*)")
}


// 0x7d9cb8 — __ZN5boost10shared_ptrIN3RBX10TypesetterEEaSERKS3_
#[doc(alias = "boost::shared_ptr<RBX::Typesetter>::operator=(boost::shared_ptr<RBX::Typesetter> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10TypesetterEEaSERKS3_")]
// was: boost::shared_ptr<RBX::Typesetter>::operator=(boost::shared_ptr<RBX::Typesetter> const&) — uses rbx_core::SharedPtr not boost
pub fn stub_7d9cb8() -> ! {
    todo!("0x7d9cb8 boost::shared_ptr<RBX::Typesetter>::operator=(boost::shared_ptr<RBX::Typesetter> const&)")
}


// 0x7d9cf0 — __ZN3RBX11TextServiceD1Ev
// type: void __fastcall(RBX::TextService *__hidden this)
#[doc(alias = "RBX::TextService::~TextService()")]
#[doc(alias = "__ZN3RBX11TextServiceD1Ev")]
pub fn stub_7d9cf0() -> ! {
    todo!("0x7d9cf0 RBX::TextService::~TextService()")
}


// 0x7d9e34 — __ZN3RBX11TextServiceD0Ev
// type: void __fastcall(RBX::TextService *__hidden this)
#[doc(alias = "RBX::TextService::~TextService()")]
#[doc(alias = "__ZN3RBX11TextServiceD0Ev")]
pub fn stub_7d9e34() -> ! {
    todo!("0x7d9e34 RBX::TextService::~TextService()")
}


// 0x7d9ed4 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEE12getClassNameEv")]
pub fn stub_7d9ed4() -> ! {
    todo!("0x7d9ed4 __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEE12getClassNameEv")
}


// 0x7d9efc — __ZThn32_N3RBX11TextServiceD1Ev
// type: void __fastcall(RBX::TextService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TextService::~TextService()")]
#[doc(alias = "__ZThn32_N3RBX11TextServiceD1Ev")]
pub fn stub_7d9efc() -> ! {
    todo!("0x7d9efc non-virtual thunk toRBX::TextService::~TextService()")
}


// 0x7d9f04 — __ZThn32_N3RBX11TextServiceD0Ev
// type: void __fastcall(RBX::TextService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TextService::~TextService()")]
#[doc(alias = "__ZThn32_N3RBX11TextServiceD0Ev")]
pub fn stub_7d9f04() -> ! {
    todo!("0x7d9f04 non-virtual thunk toRBX::TextService::~TextService()")
}


// 0x7d9fa8 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEE12getClassNameEv")]
pub fn stub_7d9fa8() -> ! {
    todo!("0x7d9fa8 __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEE12getClassNameEv")
}


// 0x7d9fd0 — __ZThn36_N3RBX11TextServiceD1Ev
// type: void __fastcall(RBX::TextService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TextService::~TextService()")]
#[doc(alias = "__ZThn36_N3RBX11TextServiceD1Ev")]
pub fn stub_7d9fd0() -> ! {
    todo!("0x7d9fd0 non-virtual thunk toRBX::TextService::~TextService()")
}


// 0x7d9fd8 — __ZThn36_N3RBX11TextServiceD0Ev
// type: void __fastcall(RBX::TextService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TextService::~TextService()")]
#[doc(alias = "__ZThn36_N3RBX11TextServiceD0Ev")]
pub fn stub_7d9fd8() -> ! {
    todo!("0x7d9fd8 non-virtual thunk toRBX::TextService::~TextService()")
}


// 0x7da07c — __ZN3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_7da07c() -> ! {
    todo!("0x7da07c __ZN3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x7da080 — __ZN3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_7da080() -> ! {
    todo!("0x7da080 __ZN3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x7da120 — __ZThn32_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_7da120() -> ! {
    todo!("0x7da120 __ZThn32_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x7da128 — __ZThn32_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_7da128() -> ! {
    todo!("0x7da128 __ZThn32_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x7da1cc — __ZThn36_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_7da1cc() -> ! {
    todo!("0x7da1cc __ZThn36_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x7da1d4 — __ZThn36_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_7da1d4() -> ! {
    todo!("0x7da1d4 __ZThn36_N3RBX10Reflection9DescribedINS_11TextServiceELZNS_12sTextServiceEENS_17NonFactoryProductINS_8InstanceELZNS_12sTextServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x7da278 — __ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::TextService::YAlignment,std::allocator<RBX::TextService::YAlignment>>::resize(unsigned long,RBX::TextService::YAlignment)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE6resizeEmS2_")]
pub fn stub_7da278() -> ! {
    todo!("0x7da278 __ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE6resizeEmS2_")
}


// 0x7da2ac — __ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::TextService::YAlignment,std::allocator<RBX::TextService::YAlignment>>::push_back(RBX::TextService::YAlignment const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE9push_backERKS2_")]
pub fn stub_7da2ac() -> ! {
    todo!("0x7da2ac __ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE9push_backERKS2_")
}


// 0x7da2d4 — __ZNSt3mapIPKN3RBX4NameENS0_11TextService10YAlignmentESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::TextService::YAlignment,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_11TextService10YAlignmentESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_7da2d4() -> ! {
    todo!("0x7da2d4 __ZNSt3mapIPKN3RBX4NameENS0_11TextService10YAlignmentESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}


// 0x7da32c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::YAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>,std::pair<RBX::Name const* const,RBX::TextService::YAlignment> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_7da32c() -> ! {
    todo!("0x7da32c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}


// 0x7da3e0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::YAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TextService::YAlignment> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_7da3e0() -> ! {
    todo!("0x7da3e0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}


// 0x7da438 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::YAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TextService::YAlignment> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_7da438() -> ! {
    todo!("0x7da438 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}


// 0x7da4a0 — __ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::TextService::YAlignment,std::allocator<RBX::TextService::YAlignment>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextService::YAlignment*,std::vector<RBX::TextService::YAlignment,std::allocator<RBX::TextService::YAlignment>>>,RBX::TextService::YAlignment const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_7da4a0() -> ! {
    todo!("0x7da4a0 __ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}


// 0x7da584 — __ZNSt12_Vector_baseIN3RBX11TextService10YAlignmentESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::TextService::YAlignment,std::allocator<RBX::TextService::YAlignment>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX11TextService10YAlignmentESaIS2_EE11_M_allocateEm")]
pub fn stub_7da584() -> ! {
    todo!("0x7da584 __ZNSt12_Vector_baseIN3RBX11TextService10YAlignmentESaIS2_EE11_M_allocateEm")
}


// 0x7da59c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService10YAlignmentES6_EET0_T_S8_S7_
#[doc(alias = "RBX::TextService::YAlignment * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TextService::YAlignment *,RBX::TextService::YAlignment *>(RBX::TextService::YAlignment *,RBX::TextService::YAlignment *,RBX::TextService::YAlignment *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService10YAlignmentES6_EET0_T_S8_S7_")]
pub fn stub_7da59c() -> ! {
    todo!("0x7da59c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService10YAlignmentES6_EET0_T_S8_S7_")
}


// 0x7da5d8 — __ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::TextService::YAlignment,std::allocator<RBX::TextService::YAlignment>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TextService::YAlignment*,std::vector<RBX::TextService::YAlignment,std::allocator<RBX::TextService::YAlignment>>>,unsigned long,RBX::TextService::YAlignment const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_7da5d8() -> ! {
    todo!("0x7da5d8 __ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}


// 0x7da768 — __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>::resize(unsigned long,RBX::TextService::XAlignment)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE6resizeEmS2_")]
pub fn stub_7da768() -> ! {
    todo!("0x7da768 __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE6resizeEmS2_")
}


// 0x7da79c — __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>::push_back(RBX::TextService::XAlignment const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE9push_backERKS2_")]
pub fn stub_7da79c() -> ! {
    todo!("0x7da79c __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE9push_backERKS2_")
}


// 0x7da7c4 — __ZNSt3mapIPKN3RBX4NameENS0_11TextService10XAlignmentESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::TextService::XAlignment,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_11TextService10XAlignmentESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_7da7c4() -> ! {
    todo!("0x7da7c4 __ZNSt3mapIPKN3RBX4NameENS0_11TextService10XAlignmentESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}


// 0x7da81c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::XAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>,std::pair<RBX::Name const* const,RBX::TextService::XAlignment> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_7da81c() -> ! {
    todo!("0x7da81c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}


// 0x7da8d0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::XAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TextService::XAlignment> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_7da8d0() -> ! {
    todo!("0x7da8d0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}


// 0x7da928 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::XAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TextService::XAlignment> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_7da928() -> ! {
    todo!("0x7da928 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}


// 0x7da990 — __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextService::XAlignment*,std::vector<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>>,RBX::TextService::XAlignment const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_7da990() -> ! {
    todo!("0x7da990 __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}


// 0x7daa74 — __ZNSt12_Vector_baseIN3RBX11TextService10XAlignmentESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX11TextService10XAlignmentESaIS2_EE11_M_allocateEm")]
pub fn stub_7daa74() -> ! {
    todo!("0x7daa74 __ZNSt12_Vector_baseIN3RBX11TextService10XAlignmentESaIS2_EE11_M_allocateEm")
}


// 0x7daa8c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService10XAlignmentES6_EET0_T_S8_S7_
#[doc(alias = "RBX::TextService::XAlignment * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TextService::XAlignment *,RBX::TextService::XAlignment *>(RBX::TextService::XAlignment *,RBX::TextService::XAlignment *,RBX::TextService::XAlignment *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService10XAlignmentES6_EET0_T_S8_S7_")]
pub fn stub_7daa8c() -> ! {
    todo!("0x7daa8c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService10XAlignmentES6_EET0_T_S8_S7_")
}


// 0x7daac8 — __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TextService::XAlignment*,std::vector<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>>,unsigned long,RBX::TextService::XAlignment const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_7daac8() -> ! {
    todo!("0x7daac8 __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}


// 0x7dac58 — __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::resize(unsigned long,RBX::TextService::Font)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE6resizeEmS2_")]
pub fn stub_7dac58() -> ! {
    todo!("0x7dac58 __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE6resizeEmS2_")
}


// 0x7dac8c — __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::push_back(RBX::TextService::Font const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE9push_backERKS2_")]
pub fn stub_7dac8c() -> ! {
    todo!("0x7dac8c std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::push_back(RBX::TextService::Font const&)")
}


// 0x7dacb4 — __ZNSt3mapIPKN3RBX4NameENS0_11TextService4FontESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::TextService::Font,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_11TextService4FontESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_7dacb4() -> ! {
    todo!("0x7dacb4 __ZNSt3mapIPKN3RBX4NameENS0_11TextService4FontESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}


// 0x7dad0c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::Font>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::pair<RBX::Name const* const,RBX::TextService::Font> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_7dad0c() -> ! {
    todo!("0x7dad0c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}


// 0x7dadc0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::Font>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TextService::Font> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_7dadc0() -> ! {
    todo!("0x7dadc0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}


// 0x7dae18 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::Font>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TextService::Font> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_7dae18() -> ! {
    todo!("0x7dae18 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}


// 0x7dae80 — __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextService::Font*,std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>>,RBX::TextService::Font const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_7dae80() -> ! {
    todo!("0x7dae80 __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}


// 0x7daf64 — __ZNSt12_Vector_baseIN3RBX11TextService4FontESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX11TextService4FontESaIS2_EE11_M_allocateEm")]
pub fn stub_7daf64() -> ! {
    todo!("0x7daf64 std::_Vector_base<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::_M_allocate(unsigned long)")
}


// 0x7daf7c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService4FontES6_EET0_T_S8_S7_
#[doc(alias = "RBX::TextService::Font * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TextService::Font *,RBX::TextService::Font *>(RBX::TextService::Font *,RBX::TextService::Font *,RBX::TextService::Font *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService4FontES6_EET0_T_S8_S7_")]
pub fn stub_7daf7c() -> ! {
    todo!("0x7daf7c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService4FontES6_EET0_T_S8_S7_")
}


// 0x7dafb8 — __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TextService::Font*,std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>>,unsigned long,RBX::TextService::Font const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_7dafb8() -> ! {
    todo!("0x7dafb8 __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}


// 0x7db148 — __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>::resize(unsigned long,RBX::TextService::FontSize)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE6resizeEmS2_")]
pub fn stub_7db148() -> ! {
    todo!("0x7db148 __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE6resizeEmS2_")
}


// 0x7db17c — __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>::push_back(RBX::TextService::FontSize const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE9push_backERKS2_")]
pub fn stub_7db17c() -> ! {
    todo!("0x7db17c __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE9push_backERKS2_")
}


// 0x7db1a4 — __ZNSt3mapIPKN3RBX4NameENS0_11TextService8FontSizeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::TextService::FontSize,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_11TextService8FontSizeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_7db1a4() -> ! {
    todo!("0x7db1a4 __ZNSt3mapIPKN3RBX4NameENS0_11TextService8FontSizeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}


// 0x7db1fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::FontSize>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::pair<RBX::Name const* const,RBX::TextService::FontSize> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_7db1fc() -> ! {
    todo!("0x7db1fc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}


// 0x7db2b0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::FontSize>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TextService::FontSize> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_7db2b0() -> ! {
    todo!("0x7db2b0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}


// 0x7db308 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::FontSize>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TextService::FontSize> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_7db308() -> ! {
    todo!("0x7db308 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}


// 0x7db370 — __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextService::FontSize*,std::vector<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>>,RBX::TextService::FontSize const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_7db370() -> ! {
    todo!("0x7db370 __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}


// 0x7db454 — __ZNSt12_Vector_baseIN3RBX11TextService8FontSizeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX11TextService8FontSizeESaIS2_EE11_M_allocateEm")]
pub fn stub_7db454() -> ! {
    todo!("0x7db454 std::_Vector_base<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>::_M_allocate(unsigned long)")
}


// 0x7db46c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService8FontSizeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::TextService::FontSize * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TextService::FontSize *,RBX::TextService::FontSize *>(RBX::TextService::FontSize *,RBX::TextService::FontSize *,RBX::TextService::FontSize *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService8FontSizeES6_EET0_T_S8_S7_")]
pub fn stub_7db46c() -> ! {
    todo!("0x7db46c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService8FontSizeES6_EET0_T_S8_S7_")
}


// 0x7db4a8 — __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TextService::FontSize*,std::vector<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>>,unsigned long,RBX::TextService::FontSize const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_7db4a8() -> ! {
    todo!("0x7db4a8 __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}


// 0x7db638 — __GLOBAL__I_a_386
#[doc(alias = "global constructor keyed to_a_386")]
#[doc(alias = "__GLOBAL__I_a_386")]
pub fn stub_7db638() -> ! {
    todo!("0x7db638 global constructor keyed to_a_386")
}


// 0x7db7d0 — __ZN3RBX13WebSerializer10writeTableERKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS3_EEE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::WebSerializer::writeTable(std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const&)")]
#[doc(alias = "__ZN3RBX13WebSerializer10writeTableERKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS3_EEE")]
pub fn stub_7db7d0() -> ! {
    todo!("0x7db7d0 __ZN3RBX13WebSerializer10writeTableERKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS3_EEE")
}


// 0x7db8d0 — __ZN3RBX13WebSerializer10writeEntryERKSsRKNS_10Reflection7VariantE
#[doc(alias = "RBX::WebSerializer::writeEntry(std::string const&,RBX::Reflection::Variant const&)")]
#[doc(alias = "__ZN3RBX13WebSerializer10writeEntryERKSsRKNS_10Reflection7VariantE")]
pub fn stub_7db8d0() -> ! {
    todo!("0x7db8d0 RBX::WebSerializer::writeEntry(std::string const&,RBX::Reflection::Variant const&)")
}


// 0x7dbac0 — __ZN3RBX13WebSerializer9writeListERKSt6vectorINS_10Reflection7VariantESaIS3_EE
#[doc(alias = "RBX::WebSerializer::writeList(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)")]
#[doc(alias = "__ZN3RBX13WebSerializer9writeListERKSt6vectorINS_10Reflection7VariantESaIS3_EE")]
pub fn stub_7dbac0() -> ! {
    todo!("0x7dbac0 RBX::WebSerializer::writeList(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)")
}


// 0x7dbbb4 — __ZN3RBX13WebSerializer10writeValueERKNS_10Reflection7VariantE
#[doc(alias = "RBX::WebSerializer::writeValue(RBX::Reflection::Variant const&)")]
#[doc(alias = "__ZN3RBX13WebSerializer10writeValueERKNS_10Reflection7VariantE")]
pub fn stub_7dbbb4() -> ! {
    todo!("0x7dbbb4 RBX::WebSerializer::writeValue(RBX::Reflection::Variant const&)")
}


// 0x7dc6cc — __ZN16XmlNameValuePair8setValueEPKc
// type: _DWORD __fastcall(XmlNameValuePair *__hidden this, const char *)
#[doc(alias = "XmlNameValuePair::setValue(char const*)")]
#[doc(alias = "__ZN16XmlNameValuePair8setValueEPKc")]
pub fn stub_7dc6cc() -> ! {
    todo!("0x7dc6cc XmlNameValuePair::setValue(char const*)")
}


// 0x7dc784 — __GLOBAL__I_a_387
#[doc(alias = "global constructor keyed to_a_387")]
#[doc(alias = "__GLOBAL__I_a_387")]
pub fn stub_7dc784() -> ! {
    todo!("0x7dc784 global constructor keyed to_a_387")
}


// 0x7dc98c — -[MacHttpController receivedData]
// type: id __cdecl(MacHttpController *self, SEL)
#[doc(alias = "-[MacHttpController receivedData]")]
#[doc(alias = "-[MacHttpController receivedData]")]
pub fn stub_7dc98c() -> ! {
    todo!("0x7dc98c -[MacHttpController receivedData]")
}


// 0x7dc99c — -[MacHttpController initWithUrl:additionalHeaders:]
// type: MacHttpController *__cdecl(MacHttpController *self, SEL, const basic_string<char, std::char_traits<char>, std::allocator<char> > *, const map<std::string, std::string, std::less<std::string >, std::allocator<std::pair<const std::string, std::string > > > *)
#[doc(alias = "-[MacHttpController initWithUrl:additionalHeaders:]")]
#[doc(alias = "-[MacHttpController initWithUrl:additionalHeaders:]")]
pub fn stub_7dc99c() -> ! {
    todo!("0x7dc99c -[MacHttpController initWithUrl:additionalHeaders:]")
}


// 0x7dcbc8 — __ZL18createSanitizedURLSs
#[doc(alias = "createSanitizedURL(std::string)")]
#[doc(alias = "__ZL18createSanitizedURLSs")]
pub fn stub_7dcbc8() -> ! {
    todo!("0x7dcbc8 createSanitizedURL(std::string)")
}


// 0x7dcc6c — -[MacHttpController setPostDataFromStream:]
// type: void __cdecl(MacHttpController *self, SEL, basic_istream<char, std::char_traits<char> > *)
#[doc(alias = "-[MacHttpController setPostDataFromStream:]")]
#[doc(alias = "-[MacHttpController setPostDataFromStream:]")]
pub fn stub_7dcc6c() -> ! {
    todo!("0x7dcc6c -[MacHttpController setPostDataFromStream:]")
}


// 0x7dcd0c — -[MacHttpController setPostCompressedDataFromString:]
// type: void __cdecl(MacHttpController *self, SEL, basic_string<char, std::char_traits<char>, std::allocator<char> > *)
#[doc(alias = "-[MacHttpController setPostCompressedDataFromString:]")]
#[doc(alias = "-[MacHttpController setPostCompressedDataFromString:]")]
pub fn stub_7dcd0c() -> ! {
    todo!("0x7dcd0c -[MacHttpController setPostCompressedDataFromString:]")
}


// 0x7dcdbc — -[MacHttpController dealloc]
// type: void __cdecl(MacHttpController *self, SEL)
#[doc(alias = "-[MacHttpController dealloc]")]
#[doc(alias = "-[MacHttpController dealloc]")]
pub fn stub_7dcdbc() -> ! {
    todo!("0x7dcdbc -[MacHttpController dealloc]")
}


// 0x7dce08 — -[MacHttpController setAuthDomain:withr:]
// type: void __cdecl(MacHttpController *self, SEL, const basic_string<char, std::char_traits<char>, std::allocator<char> > *, id)
#[doc(alias = "-[MacHttpController setAuthDomain:withr:]")]
#[doc(alias = "-[MacHttpController setAuthDomain:withr:]")]
pub fn stub_7dce08() -> ! {
    todo!("0x7dce08 -[MacHttpController setAuthDomain:withr:]")
}


// 0x7dce68 — -[MacHttpController configureRequest:]
// type: void __cdecl(MacHttpController *self, SEL, id)
#[doc(alias = "-[MacHttpController configureRequest:]")]
#[doc(alias = "-[MacHttpController configureRequest:]")]
pub fn stub_7dce68() -> ! {
    todo!("0x7dce68 -[MacHttpController configureRequest:]")
}


// 0x7dcfa4 — -[MacHttpController startConnectionWithRequest:]
// type: void __cdecl(MacHttpController *self, SEL, id)
#[doc(alias = "-[MacHttpController startConnectionWithRequest:]")]
#[doc(alias = "-[MacHttpController startConnectionWithRequest:]")]
pub fn stub_7dcfa4() -> ! {
    todo!("0x7dcfa4 -[MacHttpController startConnectionWithRequest:]")
}


// 0x7dd034 — -[MacHttpController doGetPost:]
// type: int __cdecl(MacHttpController *self, SEL, const basic_string<char, std::char_traits<char>, std::allocator<char> > *)
#[doc(alias = "-[MacHttpController doGetPost:]")]
#[doc(alias = "-[MacHttpController doGetPost:]")]
pub fn stub_7dd034() -> ! {
    todo!("0x7dd034 -[MacHttpController doGetPost:]")
}


// 0x7dd18c — -[MacHttpController connection:didFailWithError:]
// type: void __cdecl(MacHttpController *self, SEL, id, id)
#[doc(alias = "-[MacHttpController connection:didFailWithError:]")]
#[doc(alias = "-[MacHttpController connection:didFailWithError:]")]
pub fn stub_7dd18c() -> ! {
    todo!("0x7dd18c -[MacHttpController connection:didFailWithError:]")
}


// 0x7dd1c4 — -[MacHttpController connection:didReceiveData:]
// type: void __cdecl(MacHttpController *self, SEL, id, id)
#[doc(alias = "-[MacHttpController connection:didReceiveData:]")]
#[doc(alias = "-[MacHttpController connection:didReceiveData:]")]
pub fn stub_7dd1c4() -> ! {
    todo!("0x7dd1c4 -[MacHttpController connection:didReceiveData:]")
}


// 0x7dd1e4 — -[MacHttpController connection:didReceiveResponse:]
// type: void __cdecl(MacHttpController *self, SEL, id, id)
#[doc(alias = "-[MacHttpController connection:didReceiveResponse:]")]
#[doc(alias = "-[MacHttpController connection:didReceiveResponse:]")]
pub fn stub_7dd1e4() -> ! {
    todo!("0x7dd1e4 -[MacHttpController connection:didReceiveResponse:]")
}


// 0x7dd24c — -[MacHttpController connectionDidFinishLoading:]
// type: void __cdecl(MacHttpController *self, SEL, id)
#[doc(alias = "-[MacHttpController connectionDidFinishLoading:]")]
#[doc(alias = "-[MacHttpController connectionDidFinishLoading:]")]
pub fn stub_7dd24c() -> ! {
    todo!("0x7dd24c -[MacHttpController connectionDidFinishLoading:]")
}


// 0x7dd260 — -[MacHttpController connection:willSendRequest:redirectResponse:]
// type: id __cdecl(MacHttpController *self, SEL, id, id, id)
#[doc(alias = "-[MacHttpController connection:willSendRequest:redirectResponse:]")]
#[doc(alias = "-[MacHttpController connection:willSendRequest:redirectResponse:]")]
pub fn stub_7dd260() -> ! {
    todo!("0x7dd260 -[MacHttpController connection:willSendRequest:redirectResponse:]")
}


// 0x7dd35c — -[MacHttpController url]
// type: NSURL *__cdecl(MacHttpController *self, SEL)
#[doc(alias = "-[MacHttpController url]")]
#[doc(alias = "-[MacHttpController url]")]
pub fn stub_7dd35c() -> ! {
    todo!("0x7dd35c -[MacHttpController url]")
}


// 0x7dd370 — -[MacHttpController setUrl:]
// type: void __cdecl(MacHttpController *self, SEL, id)
#[doc(alias = "-[MacHttpController setUrl:]")]
#[doc(alias = "-[MacHttpController setUrl:]")]
pub fn stub_7dd370() -> ! {
    todo!("0x7dd370 -[MacHttpController setUrl:]")
}


// 0x7dd388 — -[MacHttpController .cxx_destruct]
// type: void __cdecl(MacHttpController *self, SEL)
#[doc(alias = "-[MacHttpController .cxx_destruct]")]
#[doc(alias = "-[MacHttpController .cxx_destruct]")]
pub fn stub_7dd388() -> ! {
    todo!("0x7dd388 -[MacHttpController .cxx_destruct]")
}
