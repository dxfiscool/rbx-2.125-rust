//! rendering shard 290 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 31540->31640 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 31540 before -> 31640 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x3fff3c

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;
// 0x40008c — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv
pub fn stub_40008c() -> ! {
    todo!("0x40008c __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv")
}
// 0x400090 — __ZN3RBX17FilteredSelectionINS_8InstanceEE18onSelectionChangedERKNS_16SelectionChangedE
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::FilteredSelection<RBX::Instance>::onSelectionChanged(RBX::SelectionChanged const&)")]
// was: __ZN3RBX17FilteredSelectionINS_8InstanceEE18onSelectionChangedERKNS_16SelectionChangedE
pub fn stub_400090() -> ! {
    todo!("0x400090 RBX::FilteredSelection<RBX::Instance>::onSelectionChanged(RBX::SelectionChanged const&)")
}
// 0x4000e4 — __ZThn32_N3RBX17FilteredSelectionINS_8InstanceEED1Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::Instance>::~FilteredSelection()")]
// was: __ZThn32_N3RBX17FilteredSelectionINS_8InstanceEED1Ev
pub fn stub_4000e4() -> ! {
    todo!("0x4000e4 `non-virtual thunk to'RBX::FilteredSelection<RBX::Instance>::~FilteredSelection()")
}
// 0x4000ec — __ZThn32_N3RBX17FilteredSelectionINS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::Instance>::~FilteredSelection()")]
// was: __ZThn32_N3RBX17FilteredSelectionINS_8InstanceEED0Ev
pub fn stub_4000ec() -> ! {
    todo!("0x4000ec `non-virtual thunk to'RBX::FilteredSelection<RBX::Instance>::~FilteredSelection()")
}
// 0x4000f4 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv
pub fn stub_4000f4() -> ! {
    todo!("0x4000f4 __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv")
}
// 0x4000f8 — __ZThn36_N3RBX17FilteredSelectionINS_8InstanceEED1Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::Instance>::~FilteredSelection()")]
// was: __ZThn36_N3RBX17FilteredSelectionINS_8InstanceEED1Ev
pub fn stub_4000f8() -> ! {
    todo!("0x4000f8 `non-virtual thunk to'RBX::FilteredSelection<RBX::Instance>::~FilteredSelection()")
}
// 0x400100 — __ZThn36_N3RBX17FilteredSelectionINS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::Instance>::~FilteredSelection()")]
// was: __ZThn36_N3RBX17FilteredSelectionINS_8InstanceEED0Ev
pub fn stub_400100() -> ! {
    todo!("0x400100 `non-virtual thunk to'RBX::FilteredSelection<RBX::Instance>::~FilteredSelection()")
}
// 0x400108 — __ZThn96_N3RBX17FilteredSelectionINS_8InstanceEE18onSelectionChangedERKNS_16SelectionChangedE
// type: int()
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::Instance>::onSelectionChanged(RBX::SelectionChanged const&)")]
// was: __ZThn96_N3RBX17FilteredSelectionINS_8InstanceEE18onSelectionChangedERKNS_16SelectionChangedE
pub fn stub_400108() -> ! {
    todo!("0x400108 `non-virtual thunk to'RBX::FilteredSelection<RBX::Instance>::onSelectionChanged(RBX::SelectionChanged const&)")
}
// 0x400110 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX8InstanceESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
// type: _DWORD *__fastcall(_DWORD *, int, int *)
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,RBX::Instance *>(__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,RBX::Instance * const&,std::random_access_iterator_tag)")]
// was: __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX8InstanceESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
pub fn stub_400110() -> ! {
    todo!("0x400110 __gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,RBX::Instance *>(__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,RBX::Instance * const&,std::random_access_iterator_tag)")
}
// 0x4001a0 — __ZN3RBX17FilteredSelectionINS_8InstanceEED2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "RBX::FilteredSelection<RBX::Instance>::~FilteredSelection()")]
// was: __ZN3RBX17FilteredSelectionINS_8InstanceEED2Ev
pub fn stub_4001a0() -> ! {
    todo!("0x4001a0 RBX::FilteredSelection<RBX::Instance>::~FilteredSelection()")
}
// 0x4002d4 — __ZN5boost10shared_ptrIN3RBX9SelectionEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Selection>::operator=(boost::shared_ptr<RBX::Selection> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX9SelectionEEaSERKS3_
pub fn stub_4002d4() -> ! {
    todo!("0x4002d4 boost::shared_ptr<RBX::Selection>::operator=(boost::shared_ptr<RBX::Selection> const&)")
}
// 0x40030c — __ZN3RBX11shared_fromINS_9SelectionEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "boost::shared_ptr<RBX::Selection> RBX::shared_from<RBX::Selection>(RBX::Selection*)")]
// was: __ZN3RBX11shared_fromINS_9SelectionEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_40030c() -> ! {
    todo!("0x40030c boost::shared_ptr<RBX::Selection> RBX::shared_from<RBX::Selection>(RBX::Selection*)")
}
// 0x4003f4 — __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_8InstanceEEEEC2IS4_NS1_9CreatableIS3_E7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::FilteredSelection<RBX::Instance>>::shared_ptr<RBX::FilteredSelection<RBX::Instance>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_8InstanceEEEEC2IS4_NS1_9CreatableIS3_E7DeleterEEEPT_T0_
pub fn stub_4003f4() -> ! {
    todo!("0x4003f4 boost::shared_ptr<RBX::FilteredSelection<RBX::Instance>>::shared_ptr<RBX::FilteredSelection<RBX::Instance>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter)")
}
// 0x4004bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_8InstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::Instance>,RBX::FilteredSelection<RBX::Instance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::Instance>> const*,RBX::FilteredSelection<RBX::Instance> *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_8InstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_4004bc() -> ! {
    todo!("0x4004bc void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::Instance>,RBX::FilteredSelection<RBX::Instance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::Instance>> const*,RBX::FilteredSelection<RBX::Instance> *)const")
}
// 0x4005a4 — __ZN5boost6detail12shared_countC2IPN3RBX17FilteredSelectionINS3_8InstanceEEENS3_9CreatableIS5_E7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX17FilteredSelectionINS3_8InstanceEEENS3_9CreatableIS5_E7DeleterEEET_T0_
pub fn stub_4005a4() -> ! {
    todo!("0x4005a4 boost::detail::shared_count::shared_count<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter)")
}
// 0x4006ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_8InstanceEEENS2_9CreatableIS4_E7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_8InstanceEEENS2_9CreatableIS4_E7DeleterEED1Ev
pub fn stub_4006ac() -> ! {
    todo!("0x4006ac boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}
// 0x4006b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_8InstanceEEENS2_9CreatableIS4_E7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_8InstanceEEENS2_9CreatableIS4_E7DeleterEED0Ev
pub fn stub_4006b0() -> ! {
    todo!("0x4006b0 boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}
// 0x4006b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_8InstanceEEENS2_9CreatableIS4_E7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_8InstanceEEENS2_9CreatableIS4_E7DeleterEE7disposeEv
pub fn stub_4006b4() -> ! {
    todo!("0x4006b4 boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}
// 0x4006d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_8InstanceEEENS2_9CreatableIS4_E7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_8InstanceEEENS2_9CreatableIS4_E7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4006d4() -> ! {
    todo!("0x4006d4 boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}
// 0x4006ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_8InstanceEEENS2_9CreatableIS4_E7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_8InstanceEEENS2_9CreatableIS4_E7DeleterEE19get_untyped_deleterEv
pub fn stub_4006ec() -> ! {
    todo!("0x4006ec boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::Instance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}
// 0x4006f0 — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEE12getSelectionEv
// type: int __fastcall(int)
#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::getSelection(void)")]
// was: __ZN3RBX17FilteredSelectionINS_13ModelInstanceEE12getSelectionEv
pub fn stub_4006f0() -> ! {
    todo!("0x4006f0 RBX::FilteredSelection<RBX::ModelInstance>::getSelection(void)")
}
// 0x400748 — __ZN3RBX7UngroupclERKN5boost10shared_ptrINS_8InstanceEEE
// type: _BYTE *__fastcall(int *, RBX::Instance **)
#[doc(alias = "RBX::Ungroup::operator()(boost::shared_ptr<RBX::Instance> const&)")]
// was: __ZN3RBX7UngroupclERKN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_400748() -> ! {
    todo!("0x400748 RBX::Ungroup::operator()(boost::shared_ptr<RBX::Instance> const&)")
}
// 0x4007b4 — __ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_13ModelInstanceEEEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance> * RBX::ServiceProvider::find<RBX::FilteredSelection<RBX::ModelInstance>>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_13ModelInstanceEEEEEPT_v
pub fn stub_4007b4() -> ! {
    todo!("0x4007b4 RBX::FilteredSelection<RBX::ModelInstance> * RBX::ServiceProvider::find<RBX::FilteredSelection<RBX::ModelInstance>>(void)const")
}
// 0x400928 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_13ModelInstanceEEEEEvv
// type: 
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FilteredSelection<RBX::ModelInstance>>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_13ModelInstanceEEEEEvv
pub fn stub_400928() -> ! {
    todo!("0x400928 void RBX::ServiceProvider::callDoGetClassIndex<RBX::FilteredSelection<RBX::ModelInstance>>(void)")
}
// 0x40092c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_13ModelInstanceEEEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FilteredSelection<RBX::ModelInstance>>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_13ModelInstanceEEEEEmv
pub fn stub_40092c() -> ! {
    todo!("0x40092c unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FilteredSelection<RBX::ModelInstance>>(void)")
}
// 0x400a04 — __ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_13ModelInstanceEEEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::ModelInstance>>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_13ModelInstanceEEEEEPT_v
pub fn stub_400a04() -> ! {
    todo!("0x400a04 RBX::FilteredSelection<RBX::ModelInstance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::ModelInstance>>(void)const")
}
// 0x400bcc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17FilteredSelectionINS_13ModelInstanceEEEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "boost::shared_ptr<RBX::FilteredSelection<RBX::ModelInstance>> RBX::Creatable<RBX::Instance>::create<RBX::FilteredSelection<RBX::ModelInstance>>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_17FilteredSelectionINS_13ModelInstanceEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_400bcc() -> ! {
    todo!("0x400bcc boost::shared_ptr<RBX::FilteredSelection<RBX::ModelInstance>> RBX::Creatable<RBX::Instance>::create<RBX::FilteredSelection<RBX::ModelInstance>>(void)")
}
// 0x400c7c — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17FilteredSelectionINS1_13ModelInstanceEEEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FilteredSelection<RBX::ModelInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::ModelInstance>> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17FilteredSelectionINS1_13ModelInstanceEEEEERS3_RKNS0_IT_EE
pub fn stub_400c7c() -> ! {
    todo!("0x400c7c boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FilteredSelection<RBX::ModelInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::ModelInstance>> const&)")
}
// 0x400cb0 — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::FilteredSelection(void)")]
// was: __ZN3RBX17FilteredSelectionINS_13ModelInstanceEEC2Ev
pub fn stub_400cb0() -> ! {
    todo!("0x400cb0 RBX::FilteredSelection<RBX::ModelInstance>::FilteredSelection(void)")
}
// 0x400e6c — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev
// type: int()
#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")]
// was: __ZN3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev
pub fn stub_400e6c() -> ! {
    todo!("0x400e6c RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")
}
// 0x400e70 — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")]
// was: __ZN3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev
pub fn stub_400e70() -> ! {
    todo!("0x400e70 RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")
}
// 0x400f10 — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEE17onAncestorChangedERKNS_15AncestorChangedE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::onAncestorChanged(RBX::AncestorChanged const&)")]
// was: __ZN3RBX17FilteredSelectionINS_13ModelInstanceEE17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_400f10() -> ! {
    todo!("0x400f10 RBX::FilteredSelection<RBX::ModelInstance>::onAncestorChanged(RBX::AncestorChanged const&)")
}
// 0x401088 — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::onSelectionChanged(RBX::SelectionChanged const&)")]
// was: __ZN3RBX17FilteredSelectionINS_13ModelInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
pub fn stub_401088() -> ! {
    todo!("0x401088 RBX::FilteredSelection<RBX::ModelInstance>::onSelectionChanged(RBX::SelectionChanged const&)")
}
// 0x401104 — __ZThn32_N3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")]
// was: __ZThn32_N3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev
pub fn stub_401104() -> ! {
    todo!("0x401104 `non-virtual thunk to'RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")
}
// 0x40110c — __ZThn32_N3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")]
// was: __ZThn32_N3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev
pub fn stub_40110c() -> ! {
    todo!("0x40110c `non-virtual thunk to'RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")
}
// 0x401114 — __ZThn36_N3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")]
// was: __ZThn36_N3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev
pub fn stub_401114() -> ! {
    todo!("0x401114 `non-virtual thunk to'RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")
}
// 0x40111c — __ZThn36_N3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")]
// was: __ZThn36_N3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev
pub fn stub_40111c() -> ! {
    todo!("0x40111c `non-virtual thunk to'RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")
}
// 0x401124 — __ZThn96_N3RBX17FilteredSelectionINS_13ModelInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::ModelInstance>::onSelectionChanged(RBX::SelectionChanged const&)")]
// was: __ZThn96_N3RBX17FilteredSelectionINS_13ModelInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
pub fn stub_401124() -> ! {
    todo!("0x401124 `non-virtual thunk to'RBX::FilteredSelection<RBX::ModelInstance>::onSelectionChanged(RBX::SelectionChanged const&)")
}
// 0x40112c — __ZNSt6vectorIPN3RBX13ModelInstanceESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>::push_back(RBX::ModelInstance * const&)")]
// was: __ZNSt6vectorIPN3RBX13ModelInstanceESaIS2_EE9push_backERKS2_
pub fn stub_40112c() -> ! {
    todo!("0x40112c std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>::push_back(RBX::ModelInstance * const&)")
}
// 0x401158 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13ModelInstanceESt6vectorIS4_SaIS4_EEEEPNS2_8InstanceEET_SC_SC_RKT0_St26random_access_iterator_tag
// type: _DWORD *__fastcall(_DWORD *, int, int *)
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>>,RBX::Instance *>(__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>>,__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>>,RBX::Instance * const&,std::random_access_iterator_tag)")]
// was: __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13ModelInstanceESt6vectorIS4_SaIS4_EEEEPNS2_8InstanceEET_SC_SC_RKT0_St26random_access_iterator_tag
pub fn stub_401158() -> ! {
    todo!("0x401158 __gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>>,RBX::Instance *>(__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>>,__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>>,RBX::Instance * const&,std::random_access_iterator_tag)")
}
// 0x4011e8 — __ZNSt6vectorIPN3RBX13ModelInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>>,RBX::ModelInstance * const&)")]
// was: __ZNSt6vectorIPN3RBX13ModelInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_4011e8() -> ! {
    todo!("0x4011e8 std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ModelInstance **,std::vector<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>>,RBX::ModelInstance * const&)")
}
// 0x4012c8 — __ZNSt12_Vector_baseIPN3RBX13ModelInstanceESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIPN3RBX13ModelInstanceESaIS2_EE11_M_allocateEm
pub fn stub_4012c8() -> ! {
    todo!("0x4012c8 std::_Vector_base<RBX::ModelInstance *,std::allocator<RBX::ModelInstance *>>::_M_allocate(unsigned long)")
}
// 0x4012e0 — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEED2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")]
// was: __ZN3RBX17FilteredSelectionINS_13ModelInstanceEED2Ev
pub fn stub_4012e0() -> ! {
    todo!("0x4012e0 RBX::FilteredSelection<RBX::ModelInstance>::~FilteredSelection()")
}
// 0x401414 — __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_13ModelInstanceEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::FilteredSelection<RBX::ModelInstance>>::shared_ptr<RBX::FilteredSelection<RBX::ModelInstance>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_13ModelInstanceEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_401414() -> ! {
    todo!("0x401414 boost::shared_ptr<RBX::FilteredSelection<RBX::ModelInstance>>::shared_ptr<RBX::FilteredSelection<RBX::ModelInstance>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")
}
// 0x4014dc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_13ModelInstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::ModelInstance>,RBX::FilteredSelection<RBX::ModelInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::ModelInstance>> const*,RBX::FilteredSelection<RBX::ModelInstance> *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_13ModelInstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_4014dc() -> ! {
    todo!("0x4014dc void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::ModelInstance>,RBX::FilteredSelection<RBX::ModelInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::ModelInstance>> const*,RBX::FilteredSelection<RBX::ModelInstance> *)const")
}
// 0x4015c4 — __ZN5boost6detail12shared_countC2IPN3RBX17FilteredSelectionINS3_13ModelInstanceEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX17FilteredSelectionINS3_13ModelInstanceEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4015c4() -> ! {
    todo!("0x4015c4 boost::detail::shared_count::shared_count<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")
}
// 0x4016cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_13ModelInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_13ModelInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4016cc() -> ! {
    todo!("0x4016cc boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}
// 0x4016d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_13ModelInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_13ModelInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_4016d0() -> ! {
    todo!("0x4016d0 boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}
// 0x4016d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_13ModelInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_13ModelInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4016d4() -> ! {
    todo!("0x4016d4 boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}
// 0x4016f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_13ModelInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_13ModelInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4016f4() -> ! {
    todo!("0x4016f4 boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}
// 0x40170c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_13ModelInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_13ModelInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_40170c() -> ! {
    todo!("0x40170c boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::ModelInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}
// 0x401710 — __ZN5boost3_bi5list1INS_3argILi1EEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEEENS1_IRKS9_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int, void (__fastcall **)(sp_counted_base **), const shared_count **)
#[doc(alias = "void boost::_bi::list1<boost::arg<1>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")]
// was: __ZN5boost3_bi5list1INS_3argILi1EEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEEENS1_IRKS9_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_401710() -> ! {
    todo!("0x401710 void boost::_bi::list1<boost::arg<1>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")
}
// 0x4017dc — __ZN3RBX8Instance14findCommonNodeEPS0_S1_
// type: RBX::Instance *__fastcall(RBX::Instance *this, RBX::Instance *, RBX::Instance *)
#[doc(alias = "RBX::Instance::findCommonNode(RBX::Instance*,RBX::Instance*)")]
// was: __ZN3RBX8Instance14findCommonNodeEPS0_S1_
pub fn stub_4017dc() -> ! {
    todo!("0x4017dc RBX::Instance::findCommonNode(RBX::Instance*,RBX::Instance*)")
}
// 0x40181c — __ZNK3RBX8Instance11canAddChildEPKS0_
// type: int __fastcall(RBX::Instance *this, const RBX::Instance *)
#[doc(alias = "RBX::Instance::canAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX8Instance11canAddChildEPKS0_
pub fn stub_40181c() -> ! {
    todo!("0x40181c RBX::Instance::canAddChild(RBX::Instance const*)const")
}
// 0x40187c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ModelInstanceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "boost::shared_ptr<RBX::ModelInstance> RBX::Creatable<RBX::Instance>::create<RBX::ModelInstance>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ModelInstanceEEEN5boost10shared_ptrIT_EEv
pub fn stub_40187c() -> ! {
    todo!("0x40187c boost::shared_ptr<RBX::ModelInstance> RBX::Creatable<RBX::Instance>::create<RBX::ModelInstance>(void)")
}
// 0x401930 — __ZN5boost10shared_ptrIN3RBX13ModelInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::ModelInstance>::shared_ptr<RBX::ModelInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13ModelInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_401930() -> ! {
    todo!("0x401930 boost::shared_ptr<RBX::ModelInstance>::shared_ptr<RBX::ModelInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter)")
}
// 0x4019f8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ModelInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ModelInstance,RBX::ModelInstance>(boost::shared_ptr<RBX::ModelInstance> const*,RBX::ModelInstance *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ModelInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_4019f8() -> ! {
    todo!("0x4019f8 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ModelInstance,RBX::ModelInstance>(boost::shared_ptr<RBX::ModelInstance> const*,RBX::ModelInstance *)const")
}
// 0x401ae0 — __ZN5boost6detail12shared_countC2IPN3RBX13ModelInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13ModelInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_401ae0() -> ! {
    todo!("0x401ae0 boost::detail::shared_count::shared_count<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter)")
}
// 0x401be8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_401be8() -> ! {
    todo!("0x401be8 boost::detail::sp_counted_impl_pd<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}
// 0x401bec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_401bec() -> ! {
    todo!("0x401bec boost::detail::sp_counted_impl_pd<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}
// 0x401c0c — __ZNK3RBX13ServiceClientINS_10RunServiceEE13createServiceEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::ServiceClient<RBX::RunService>::createService(void)const")]
// was: __ZNK3RBX13ServiceClientINS_10RunServiceEE13createServiceEv
pub fn stub_401c0c() -> ! {
    todo!("0x401c0c RBX::ServiceClient<RBX::RunService>::createService(void)const")
}
// 0x401cec — __ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_401cec() -> ! {
    todo!("0x401cec __ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator12getClassNameEv")
}
// 0x401d58 — __ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v
// type: 
#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v
pub fn stub_401d58() -> ! {
    todo!("0x401d58 __ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v")
}
// 0x401d9c — __ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v
pub fn stub_401d9c() -> ! {
    todo!("0x401d9c __ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v")
}
// 0x401e80 — __ZN3RBX17FilteredSelectionINS_10PVInstanceEE12getSelectionEv
// type: int __fastcall(int)
#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::getSelection(void)")]
// was: __ZN3RBX17FilteredSelectionINS_10PVInstanceEE12getSelectionEv
pub fn stub_401e80() -> ! {
    todo!("0x401e80 RBX::FilteredSelection<RBX::PVInstance>::getSelection(void)")
}
// 0x401ed8 — __ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::PVInstance>>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_v
pub fn stub_401ed8() -> ! {
    todo!("0x401ed8 RBX::FilteredSelection<RBX::PVInstance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::PVInstance>>(void)const")
}
// 0x4020a0 — __ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance> * RBX::ServiceProvider::find<RBX::FilteredSelection<RBX::PVInstance>>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_v
pub fn stub_4020a0() -> ! {
    todo!("0x4020a0 RBX::FilteredSelection<RBX::PVInstance> * RBX::ServiceProvider::find<RBX::FilteredSelection<RBX::PVInstance>>(void)const")
}
// 0x402214 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17FilteredSelectionINS_10PVInstanceEEEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> RBX::Creatable<RBX::Instance>::create<RBX::FilteredSelection<RBX::PVInstance>>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_17FilteredSelectionINS_10PVInstanceEEEEEN5boost10shared_ptrIT_EEv
pub fn stub_402214() -> ! {
    todo!("0x402214 boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> RBX::Creatable<RBX::Instance>::create<RBX::FilteredSelection<RBX::PVInstance>>(void)")
}
// 0x4022c4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17FilteredSelectionINS1_10PVInstanceEEEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FilteredSelection<RBX::PVInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17FilteredSelectionINS1_10PVInstanceEEEEERS3_RKNS0_IT_EE
pub fn stub_4022c4() -> ! {
    todo!("0x4022c4 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FilteredSelection<RBX::PVInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> const&)")
}
// 0x4022f8 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_10PVInstanceEEEEEvv
// type: 
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FilteredSelection<RBX::PVInstance>>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_10PVInstanceEEEEEvv
pub fn stub_4022f8() -> ! {
    todo!("0x4022f8 void RBX::ServiceProvider::callDoGetClassIndex<RBX::FilteredSelection<RBX::PVInstance>>(void)")
}
// 0x4022fc — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_10PVInstanceEEEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FilteredSelection<RBX::PVInstance>>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_10PVInstanceEEEEEmv
pub fn stub_4022fc() -> ! {
    todo!("0x4022fc unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FilteredSelection<RBX::PVInstance>>(void)")
}
// 0x4023d4 — __ZN3RBX17FilteredSelectionINS_10PVInstanceEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::FilteredSelection(void)")]
// was: __ZN3RBX17FilteredSelectionINS_10PVInstanceEEC2Ev
pub fn stub_4023d4() -> ! {
    todo!("0x4023d4 RBX::FilteredSelection<RBX::PVInstance>::FilteredSelection(void)")
}
// 0x402590 — __ZN3RBX17FilteredSelectionINS_10PVInstanceEED1Ev
// type: int()
#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")]
// was: __ZN3RBX17FilteredSelectionINS_10PVInstanceEED1Ev
pub fn stub_402590() -> ! {
    todo!("0x402590 RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")
}
// 0x402594 — __ZN3RBX17FilteredSelectionINS_10PVInstanceEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")]
// was: __ZN3RBX17FilteredSelectionINS_10PVInstanceEED0Ev
pub fn stub_402594() -> ! {
    todo!("0x402594 RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")
}
// 0x402634 — __ZN3RBX17FilteredSelectionINS_10PVInstanceEE17onAncestorChangedERKNS_15AncestorChangedE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::onAncestorChanged(RBX::AncestorChanged const&)")]
// was: __ZN3RBX17FilteredSelectionINS_10PVInstanceEE17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_402634() -> ! {
    todo!("0x402634 RBX::FilteredSelection<RBX::PVInstance>::onAncestorChanged(RBX::AncestorChanged const&)")
}
// 0x4027ac — __ZN3RBX17FilteredSelectionINS_10PVInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::onSelectionChanged(RBX::SelectionChanged const&)")]
// was: __ZN3RBX17FilteredSelectionINS_10PVInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
pub fn stub_4027ac() -> ! {
    todo!("0x4027ac RBX::FilteredSelection<RBX::PVInstance>::onSelectionChanged(RBX::SelectionChanged const&)")
}
// 0x402828 — __ZThn32_N3RBX17FilteredSelectionINS_10PVInstanceEED1Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")]
// was: __ZThn32_N3RBX17FilteredSelectionINS_10PVInstanceEED1Ev
pub fn stub_402828() -> ! {
    todo!("0x402828 `non-virtual thunk to'RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")
}
// 0x402830 — __ZThn32_N3RBX17FilteredSelectionINS_10PVInstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")]
// was: __ZThn32_N3RBX17FilteredSelectionINS_10PVInstanceEED0Ev
pub fn stub_402830() -> ! {
    todo!("0x402830 `non-virtual thunk to'RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")
}
// 0x402838 — __ZThn36_N3RBX17FilteredSelectionINS_10PVInstanceEED1Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")]
// was: __ZThn36_N3RBX17FilteredSelectionINS_10PVInstanceEED1Ev
pub fn stub_402838() -> ! {
    todo!("0x402838 `non-virtual thunk to'RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")
}
// 0x402840 — __ZThn36_N3RBX17FilteredSelectionINS_10PVInstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")]
// was: __ZThn36_N3RBX17FilteredSelectionINS_10PVInstanceEED0Ev
pub fn stub_402840() -> ! {
    todo!("0x402840 `non-virtual thunk to'RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")
}
// 0x402848 — __ZThn96_N3RBX17FilteredSelectionINS_10PVInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PVInstance>::onSelectionChanged(RBX::SelectionChanged const&)")]
// was: __ZThn96_N3RBX17FilteredSelectionINS_10PVInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
pub fn stub_402848() -> ! {
    todo!("0x402848 `non-virtual thunk to'RBX::FilteredSelection<RBX::PVInstance>::onSelectionChanged(RBX::SelectionChanged const&)")
}
// 0x402850 — __ZNSt6vectorIPN3RBX10PVInstanceESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>::push_back(RBX::PVInstance * const&)")]
// was: __ZNSt6vectorIPN3RBX10PVInstanceESaIS2_EE9push_backERKS2_
pub fn stub_402850() -> ! {
    todo!("0x402850 std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>::push_back(RBX::PVInstance * const&)")
}
// 0x40287c — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX10PVInstanceESt6vectorIS4_SaIS4_EEEEPNS2_8InstanceEET_SC_SC_RKT0_St26random_access_iterator_tag
// type: _DWORD *__fastcall(_DWORD *, int, int *)
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>>,RBX::Instance *>(__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>>,__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>>,RBX::Instance * const&,std::random_access_iterator_tag)")]
// was: __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX10PVInstanceESt6vectorIS4_SaIS4_EEEEPNS2_8InstanceEET_SC_SC_RKT0_St26random_access_iterator_tag
pub fn stub_40287c() -> ! {
    todo!("0x40287c __gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>>,RBX::Instance *>(__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>>,__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>>,RBX::Instance * const&,std::random_access_iterator_tag)")
}
// 0x40290c — __ZNSt6vectorIPN3RBX10PVInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>>,RBX::PVInstance * const&)")]
// was: __ZNSt6vectorIPN3RBX10PVInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_40290c() -> ! {
    todo!("0x40290c std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PVInstance **,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>>,RBX::PVInstance * const&)")
}
// 0x4029ec — __ZNSt12_Vector_baseIPN3RBX10PVInstanceESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIPN3RBX10PVInstanceESaIS2_EE11_M_allocateEm
pub fn stub_4029ec() -> ! {
    todo!("0x4029ec std::_Vector_base<RBX::PVInstance *,std::allocator<RBX::PVInstance *>>::_M_allocate(unsigned long)")
}
// 0x402a04 — __ZN3RBX17FilteredSelectionINS_10PVInstanceEED2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")]
// was: __ZN3RBX17FilteredSelectionINS_10PVInstanceEED2Ev
pub fn stub_402a04() -> ! {
    todo!("0x402a04 RBX::FilteredSelection<RBX::PVInstance>::~FilteredSelection()")
}
// 0x402b38 — __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_10PVInstanceEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>>::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_10PVInstanceEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_402b38() -> ! {
    todo!("0x402b38 boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>>::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")
}
// 0x402c00 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_10PVInstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::PVInstance>,RBX::FilteredSelection<RBX::PVInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> const*,RBX::FilteredSelection<RBX::PVInstance> *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_10PVInstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_402c00() -> ! {
    todo!("0x402c00 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::PVInstance>,RBX::FilteredSelection<RBX::PVInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> const*,RBX::FilteredSelection<RBX::PVInstance> *)const")
}
// 0x402ce8 — __ZN5boost6detail12shared_countC2IPN3RBX17FilteredSelectionINS3_10PVInstanceEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX17FilteredSelectionINS3_10PVInstanceEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_402ce8() -> ! {
    todo!("0x402ce8 boost::detail::shared_count::shared_count<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")
}
// 0x402df0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_10PVInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_10PVInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_402df0() -> ! {
    todo!("0x402df0 boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}
// 0x402df4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_10PVInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_10PVInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_402df4() -> ! {
    todo!("0x402df4 boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}
// 0x402df8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_10PVInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_10PVInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_402df8() -> ! {
    todo!("0x402df8 boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}
// 0x402e18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_10PVInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_10PVInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_402e18() -> ! {
    todo!("0x402e18 boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}
// 0x402e30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_10PVInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_10PVInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_402e30() -> ! {
    todo!("0x402e30 boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PVInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}
// 0x402e34 — __ZNK3RBX13ServiceClientINS_17FilteredSelectionINS_10PVInstanceEEEE13createServiceEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::ServiceClient<RBX::FilteredSelection<RBX::PVInstance>>::createService(void)const")]
// was: __ZNK3RBX13ServiceClientINS_17FilteredSelectionINS_10PVInstanceEEEE13createServiceEv
pub fn stub_402e34() -> ! {
    todo!("0x402e34 RBX::ServiceClient<RBX::FilteredSelection<RBX::PVInstance>>::createService(void)const")
}
// 0x402f14 — __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_10PVInstanceEEEEaSERKS5_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>>::operator=(boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_10PVInstanceEEEEaSERKS5_
pub fn stub_402f14() -> ! {
    todo!("0x402f14 boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>>::operator=(boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> const&)")
}
// 0x402f4c — __ZN3RBX11shared_fromINS_17FilteredSelectionINS_10PVInstanceEEEEEN5boost10shared_ptrIT_EEPS6_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> RBX::shared_from<RBX::FilteredSelection<RBX::PVInstance>>(RBX::FilteredSelection<RBX::PVInstance>*)")]
// was: __ZN3RBX11shared_fromINS_17FilteredSelectionINS_10PVInstanceEEEEEN5boost10shared_ptrIT_EEPS6_
pub fn stub_402f4c() -> ! {
    todo!("0x402f4c boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> RBX::shared_from<RBX::FilteredSelection<RBX::PVInstance>>(RBX::FilteredSelection<RBX::PVInstance>*)")
}
// 0x403034 — __ZN3RBX21BoolPropertyVerbSetItclEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(unsigned __int8 *, int *)
#[doc(alias = "RBX::BoolPropertyVerbSetIt::operator()(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX21BoolPropertyVerbSetItclEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_403034() -> ! {
    todo!("0x403034 RBX::BoolPropertyVerbSetIt::operator()(boost::shared_ptr<RBX::Instance>)")
}
// 0x403198 — __ZNK3RBX13ServiceClientINS_9SelectionEE13createServiceEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::ServiceClient<RBX::Selection>::createService(void)const")]
// was: __ZNK3RBX13ServiceClientINS_9SelectionEE13createServiceEv
pub fn stub_403198() -> ! {
    todo!("0x403198 RBX::ServiceClient<RBX::Selection>::createService(void)const")
}
// 0x403278 — __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIbPFbPKcS6_ENSD_5list2INSD_5valueISG_EENS2_3argILi1EEEEEEEET_SQ_SQ_T0_St26random_access_iterator_tag
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<bool,bool (*)(char const*,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<char const*>,boost::arg<1>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<bool,bool (*)(char const*,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<char const*>,boost::arg<1>>>,std::random_access_iterator_tag)")]
// was: __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIbPFbPKcS6_ENSD_5list2INSD_5valueISG_EENS2_3argILi1EEEEEEEET_SQ_SQ_T0_St26random_access_iterator_tag
pub fn stub_403278() -> ! {
    todo!("0x403278 __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<bool,bool (*)(char const*,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<char const*>,boost::arg<1>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<bool,bool (*)(char const*,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<char const*>,boost::arg<1>>>,std::random_access_iterator_tag)")
}