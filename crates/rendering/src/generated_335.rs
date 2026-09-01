//! rendering shard 335 — 100 stubs 0x5b9cb0..0x5bfde8 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 36460->36560 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 36460 before -> 36560 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x5b9838 (lowest remaining 0x5b9cb0..0x5bfde8, next lowest 0x5bfe18 if exists)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x5b9cb0 — __ZN3RBX24KeyframeSequenceProvider30registerActiveKeyframeSequenceEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequenceProvider::registerActiveKeyframeSequence(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX24KeyframeSequenceProvider30registerActiveKeyframeSequenceEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5b9cb0() -> ! {
    todo!("0x5b9cb0 RBX::KeyframeSequenceProvider::registerActiveKeyframeSequence(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x5b9fb0 — __ZN3RBX24KeyframeSequenceProvider24registerKeyframeSequenceEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequenceProvider::registerKeyframeSequence(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX24KeyframeSequenceProvider24registerKeyframeSequenceEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5b9fb0() -> ! {
    todo!("0x5b9fb0 RBX::KeyframeSequenceProvider::registerKeyframeSequence(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x5ba2b4 — __ZN3RBX24KeyframeSequenceProvider22getKeyframeSequenceLuaENS_9ContentIdE
#[doc(alias = "RBX::KeyframeSequenceProvider::getKeyframeSequenceLua(RBX::ContentId)")]
// was: __ZN3RBX24KeyframeSequenceProvider22getKeyframeSequenceLuaENS_9ContentIdE
pub fn stub_5ba2b4() -> ! {
    todo!("0x5ba2b4 RBX::KeyframeSequenceProvider::getKeyframeSequenceLua(RBX::ContentId)")
}

// 0x5ba44c — __ZN3RBX24KeyframeSequenceProviderC1Ev
// type: _DWORD __fastcall(RBX::KeyframeSequenceProvider *__hidden this)
#[doc(alias = "RBX::KeyframeSequenceProvider::KeyframeSequenceProvider(void)")]
// was: __ZN3RBX24KeyframeSequenceProviderC1Ev
pub fn stub_5ba44c() -> ! {
    todo!("0x5ba44c RBX::KeyframeSequenceProvider::KeyframeSequenceProvider(void)")
}

// 0x5ba450 — __ZN3RBX24KeyframeSequenceProviderC2Ev
// type: _DWORD __fastcall(RBX::KeyframeSequenceProvider *__hidden this)
#[doc(alias = "RBX::KeyframeSequenceProvider::KeyframeSequenceProvider(void)")]
// was: __ZN3RBX24KeyframeSequenceProviderC2Ev
pub fn stub_5ba450() -> ! {
    todo!("0x5ba450 RBX::KeyframeSequenceProvider::KeyframeSequenceProvider(void)")
}

// 0x5ba624 — __ZN3RBXL11itIsInScopeEPNS_8InstanceE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::itIsInScope(RBX::Instance *)")]
// was: __ZN3RBXL11itIsInScopeEPNS_8InstanceE
pub fn stub_5ba624() -> ! {
    todo!("0x5ba624 RBX::itIsInScope(RBX::Instance *)")
}

// 0x5ba628 — __ZN3RBX24KeyframeSequenceProvider26privateGetKeyframeSequenceENS_9ContentIdEb
#[doc(alias = "RBX::KeyframeSequenceProvider::privateGetKeyframeSequence(RBX::ContentId,bool)")]
// was: __ZN3RBX24KeyframeSequenceProvider26privateGetKeyframeSequenceENS_9ContentIdEb
pub fn stub_5ba628() -> ! {
    todo!("0x5ba628 RBX::KeyframeSequenceProvider::privateGetKeyframeSequence(RBX::ContentId,bool)")
}

// 0x5bb084 — __ZN3RBX24KeyframeSequenceProvider19getKeyframeSequenceENS_9ContentIdE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::KeyframeSequenceProvider::getKeyframeSequence(RBX::ContentId)")]
// was: __ZN3RBX24KeyframeSequenceProvider19getKeyframeSequenceENS_9ContentIdE
pub fn stub_5bb084() -> ! {
    todo!("0x5bb084 RBX::KeyframeSequenceProvider::getKeyframeSequence(RBX::ContentId)")
}

// 0x5bb1ec — __ZN3RBXL25AsyncKeyframeLoaderHelperENS_14AsyncHttpQueue13RequestResultEPSiN5boost8weak_ptrINS_24KeyframeSequenceProviderEEENS4_INS_16KeyframeSequenceEEE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::AsyncKeyframeLoaderHelper(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>)")]
// was: __ZN3RBXL25AsyncKeyframeLoaderHelperENS_14AsyncHttpQueue13RequestResultEPSiN5boost8weak_ptrINS_24KeyframeSequenceProviderEEENS4_INS_16KeyframeSequenceEEE
pub fn stub_5bb1ec() -> ! {
    todo!("0x5bb1ec RBX::AsyncKeyframeLoaderHelper(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>)")
}

// 0x5bb30c — __ZN3RBXL20KeyframeLoaderHelperENS_14AsyncHttpQueue13RequestResultEPSiN5boost8weak_ptrINS_24KeyframeSequenceProviderEEENS4_INS_16KeyframeSequenceEEEb
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, int)
#[doc(alias = "RBX::KeyframeLoaderHelper(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>,bool)")]
// was: __ZN3RBXL20KeyframeLoaderHelperENS_14AsyncHttpQueue13RequestResultEPSiN5boost8weak_ptrINS_24KeyframeSequenceProviderEEENS4_INS_16KeyframeSequenceEEEb
pub fn stub_5bb30c() -> ! {
    todo!("0x5bb30c RBX::KeyframeLoaderHelper(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>,bool)")
}

// 0x5bb8cc — __ZN3RBXL24CopyKeyframeSequenceDataEN5boost8weak_ptrINS_16KeyframeSequenceEEENS0_10shared_ptrIS2_EE
#[doc(alias = "RBX::CopyKeyframeSequenceData(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>)")]
// was: __ZN3RBXL24CopyKeyframeSequenceDataEN5boost8weak_ptrINS_16KeyframeSequenceEEENS0_10shared_ptrIS2_EE
pub fn stub_5bb8cc() -> ! {
    todo!("0x5bb8cc RBX::CopyKeyframeSequenceData(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>)")
}

// 0x5bb990 — __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
pub fn stub_5bb990() -> ! {
    todo!("0x5bb990 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0x5bba9c — __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EED1Ev
pub fn stub_5bba9c() -> ! {
    todo!("0x5bba9c RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::~BoundFuncDesc()")
}

// 0x5bbadc — __ZNSt3mapISsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_
// type: int(void)
#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::operator[](std::string const&)")]
// was: __ZNSt3mapISsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_
pub fn stub_5bbadc() -> ! {
    todo!("0x5bbadc std::map<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::operator[](std::string const&)")
}

// 0x5bbcf8 — __ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEaSERKS3_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence>::operator=(rbx_core::SharedPtr<RBX::KeyframeSequence> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEaSERKS3_
pub fn stub_5bbcf8() -> ! {
    todo!("0x5bbcf8 rbx_core::SharedPtr<RBX::KeyframeSequence>::operator=(rbx_core::SharedPtr<RBX::KeyframeSequence> const&)")
}

// 0x5bbd34 — __ZNK3RBX11AnimationId8isActiveEv
// type: _DWORD __fastcall(RBX::AnimationId *__hidden this)
#[doc(alias = "RBX::AnimationId::isActive(void)const")]
// was: __ZNK3RBX11AnimationId8isActiveEv
pub fn stub_5bbd34() -> ! {
    todo!("0x5bbd34 RBX::AnimationId::isActive(void)const")
}

// 0x5bbe64 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16KeyframeSequenceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequence>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_16KeyframeSequenceEEEN5boost10shared_ptrIT_EEv
pub fn stub_5bbe64() -> ! {
    todo!("0x5bbe64 rbx_core::SharedPtr<RBX::KeyframeSequence> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequence>(void)")
}

// 0x5bbf14 — __ZN3RBX20SizeEnforcedLRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6insertERKSsRKS4_m
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::insert(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&,unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6insertERKSsRKS4_m
pub fn stub_5bbf14() -> ! {
    todo!("0x5bbf14 RBX::SizeEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::insert(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&,unsigned long)")
}

// 0x5bbf48 — __ZN3RBX9weak_fromINS_24KeyframeSequenceProviderEEEN5boost8weak_ptrIT_EEPS4_
// type: int(void)
#[doc(alias = "rbx_core::WeakPtr<RBX::KeyframeSequenceProvider> RBX::weak_from<RBX::KeyframeSequenceProvider>(RBX::KeyframeSequenceProvider*)")]
// was: __ZN3RBX9weak_fromINS_24KeyframeSequenceProviderEEEN5boost8weak_ptrIT_EEPS4_
pub fn stub_5bbf48() -> ! {
    todo!("0x5bbf48 rbx_core::WeakPtr<RBX::KeyframeSequenceProvider> RBX::weak_from<RBX::KeyframeSequenceProvider>(RBX::KeyframeSequenceProvider*)")
}

// 0x5bc150 — __ZN5boost4bindIvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS1_24KeyframeSequenceProviderEEENS5_INS1_16KeyframeSequenceEEENS_3argILi1EEENSA_ILi2EEES7_S9_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_T3_ENSD_9list_av_4IT4_T5_T6_T7_E4typeEEESL_SN_SO_SP_SQ_
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>>::type> boost::bind<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>,boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>>(void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>)")]
// was: __ZN5boost4bindIvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS1_24KeyframeSequenceProviderEEENS5_INS1_16KeyframeSequenceEEENS_3argILi1EEENSA_ILi2EEES7_S9_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_T3_ENSD_9list_av_4IT4_T5_T6_T7_E4typeEEESL_SN_SO_SP_SQ_
pub fn stub_5bc150() -> ! {
    todo!("0x5bc150 boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>>::type> boost::bind<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>,boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>>(void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>)")
}

// 0x5bc33c — __ZN3RBX24KeyframeSequenceProviderD1Ev
// type: void __fastcall(RBX::KeyframeSequenceProvider *__hidden this)
#[doc(alias = "RBX::KeyframeSequenceProvider::~KeyframeSequenceProvider()")]
// was: __ZN3RBX24KeyframeSequenceProviderD1Ev
pub fn stub_5bc33c() -> ! {
    todo!("0x5bc33c RBX::KeyframeSequenceProvider::~KeyframeSequenceProvider()")
}

// 0x5bc480 — __ZN3RBX24KeyframeSequenceProviderD0Ev
// type: void __fastcall(RBX::KeyframeSequenceProvider *__hidden this)
#[doc(alias = "RBX::KeyframeSequenceProvider::~KeyframeSequenceProvider()")]
// was: __ZN3RBX24KeyframeSequenceProviderD0Ev
pub fn stub_5bc480() -> ! {
    todo!("0x5bc480 RBX::KeyframeSequenceProvider::~KeyframeSequenceProvider()")
}

// 0x5bc5d8 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE12getClassNameEv
pub fn stub_5bc5d8() -> ! {
    todo!("0x5bc5d8 __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE12getClassNameEv")
}

// 0x5bc604 — __ZThn32_N3RBX24KeyframeSequenceProviderD1Ev
// type: void __fastcall(RBX::KeyframeSequenceProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::KeyframeSequenceProvider::~KeyframeSequenceProvider()")]
// was: __ZThn32_N3RBX24KeyframeSequenceProviderD1Ev
pub fn stub_5bc604() -> ! {
    todo!("0x5bc604 non-virtual thunk toRBX::KeyframeSequenceProvider::~KeyframeSequenceProvider()")
}

// 0x5bc748 — __ZThn32_N3RBX24KeyframeSequenceProviderD0Ev
// type: void __fastcall(RBX::KeyframeSequenceProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::KeyframeSequenceProvider::~KeyframeSequenceProvider()")]
// was: __ZThn32_N3RBX24KeyframeSequenceProviderD0Ev
pub fn stub_5bc748() -> ! {
    todo!("0x5bc748 non-virtual thunk toRBX::KeyframeSequenceProvider::~KeyframeSequenceProvider()")
}

// 0x5bc8a0 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE12getClassNameEv
pub fn stub_5bc8a0() -> ! {
    todo!("0x5bc8a0 __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE12getClassNameEv")
}

// 0x5bc8c8 — __ZThn36_N3RBX24KeyframeSequenceProviderD1Ev
// type: void __fastcall(RBX::KeyframeSequenceProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::KeyframeSequenceProvider::~KeyframeSequenceProvider()")]
// was: __ZThn36_N3RBX24KeyframeSequenceProviderD1Ev
pub fn stub_5bc8c8() -> ! {
    todo!("0x5bc8c8 non-virtual thunk toRBX::KeyframeSequenceProvider::~KeyframeSequenceProvider()")
}

// 0x5bca0c — __ZThn36_N3RBX24KeyframeSequenceProviderD0Ev
// type: void __fastcall(RBX::KeyframeSequenceProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::KeyframeSequenceProvider::~KeyframeSequenceProvider()")]
// was: __ZThn36_N3RBX24KeyframeSequenceProviderD0Ev
pub fn stub_5bca0c() -> ! {
    todo!("0x5bca0c non-virtual thunk toRBX::KeyframeSequenceProvider::~KeyframeSequenceProvider()")
}

// 0x5bcb68 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEED2Ev
#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::~LRUCache()")]
// was: __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEED2Ev
pub fn stub_5bcb68() -> ! {
    todo!("0x5bcb68 RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::~LRUCache()")
}

// 0x5bcc68 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6resizeEm
#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::resize(unsigned long)")]
// was: __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6resizeEm
pub fn stub_5bcc68() -> ! {
    todo!("0x5bcc68 RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::resize(unsigned long)")
}

// 0x5bcca0 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6insertERKSsRKS4_m
// type: int(void)
#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::insert(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&,unsigned long)")]
// was: __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6insertERKSsRKS4_m
pub fn stub_5bcca0() -> ! {
    todo!("0x5bcca0 RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::insert(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&,unsigned long)")
}

// 0x5bd1c8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERS5_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERS5_RKT_
pub fn stub_5bd1c8() -> ! {
    todo!("0x5bd1c8 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> const&)")
}

// 0x5bd368 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_
// type: int(void)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> const&)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_
pub fn stub_5bd368() -> ! {
    todo!("0x5bd368 void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> const&)")
}

// 0x5bd38c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: int(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_5bd38c() -> ! {
    todo!("0x5bd38c boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")
}

// 0x5bd3dc — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEED2Ev
// type: int(void)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::~node_constructor()")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEED2Ev
pub fn stub_5bd3dc() -> ! {
    todo!("0x5bd3dc boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::~node_constructor()")
}

// 0x5bd3f8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_5bd3f8() -> ! {
    todo!("0x5bd3f8 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")
}

// 0x5bd520 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// type: int(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
pub fn stub_5bd520() -> ! {
    todo!("0x5bd520 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")
}

// 0x5bd5b0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// type: int(void)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
pub fn stub_5bd5b0() -> ! {
    todo!("0x5bd5b0 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")
}

// 0x5bd5dc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISK_EEPNS1_10ptr_bucketE
// type: int(void)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISK_EEPNS1_10ptr_bucketE
pub fn stub_5bd5dc() -> ! {
    todo!("0x5bd5dc boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0x5bd634 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE9constructEv
// type: int(void)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE9constructEv
pub fn stub_5bd634() -> ! {
    todo!("0x5bd634 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::construct(void)")
}

// 0x5bd670 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSJ_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEmRKT_RKT0_
// type: int(void)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSJ_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEmRKT_RKT0_
pub fn stub_5bd670() -> ! {
    todo!("0x5bd670 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")
}

// 0x5bd6dc — __ZNSt4pairISsS_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEC2ERKSsRKS5_
// type: _DWORD *__fastcall(_DWORD *, const std::string *, const shared_count *)
#[doc(alias = "std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>::pair(std::string const&,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
// was: __ZNSt4pairISsS_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEC2ERKSsRKS5_
pub fn stub_5bd6dc() -> ! {
    todo!("0x5bd6dc std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>::pair(std::string const&,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")
}

// 0x5bd7a4 — __ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE14_M_create_nodeERKS7_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>> const&)")]
// was: __ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE14_M_create_nodeERKS7_
pub fn stub_5bd7a4() -> ! {
    todo!("0x5bd7a4 std::list<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>> const&)")
}

// 0x5bd8b8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_
// type: int(void)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_
pub fn stub_5bd8b8() -> ! {
    todo!("0x5bd8b8 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> *)")
}

// 0x5bd914 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// type: int(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_5bd914() -> ! {
    todo!("0x5bd914 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}

// 0x5bd940 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// type: int(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
pub fn stub_5bd940() -> ! {
    todo!("0x5bd940 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")
}

// 0x5bd980 — __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEE7destroyEPS8_
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::destroy(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>*)")]
// was: __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEE7destroyEPS8_
pub fn stub_5bd980() -> ! {
    todo!("0x5bd980 __gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::destroy(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>*)")
}

// 0x5bda24 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// type: int(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
pub fn stub_5bda24() -> ! {
    todo!("0x5bda24 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")
}

// 0x5bda64 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6removeERKSs
// type: int(void)
#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::remove(std::string const&)")]
// was: __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6removeERKSs
pub fn stub_5bda64() -> ! {
    todo!("0x5bda64 RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::remove(std::string const&)")
}

// 0x5bdab8 — __ZNSt10_List_baseISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE8_M_clearEv
// type: int(void)
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::_M_clear(void)")]
// was: __ZNSt10_List_baseISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE8_M_clearEv
pub fn stub_5bdab8() -> ! {
    todo!("0x5bdab8 std::_List_base<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::_M_clear(void)")
}

// 0x5bdae0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// type: int(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_5bdae0() -> ! {
    todo!("0x5bdae0 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")
}

// 0x5bdb18 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// type: int(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE5clearEv
pub fn stub_5bdb18() -> ! {
    todo!("0x5bdb18 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")
}

// 0x5bdb50 — __ZN3RBX4Name9doDeclareILZNS_25sKeyframeSequenceProviderEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_25sKeyframeSequenceProviderEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_25sKeyframeSequenceProviderEEEERKS0_v
pub fn stub_5bdb50() -> ! {
    todo!("0x5bdb50 __ZN3RBX4Name9doDeclareILZNS_25sKeyframeSequenceProviderEEEERKS0_v")
}

// 0x5bdc30 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSD_INS1_16KeyframeSequenceEEEENSB_5list4INS_3argILi1EEENSL_ILi2EEENSB_5valueISF_EENSO_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSD_INS1_16KeyframeSequenceEEEENSB_5list4INS_3argILi1EEENSL_ILi2EEENSB_5valueISF_EENSO_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSD_INS1_16KeyframeSequenceEEEENSB_5list4INS_3argILi1EEENSL_ILi2EEENSB_5valueISF_EENSO_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
pub fn stub_5bdc30() -> ! {
    todo!("0x5bdc30 __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSD_INS1_16KeyframeSequenceEEEENSB_5list4INS_3argILi1EEENSL_ILi2EEENSB_5valueISF_EENSO_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

// 0x5bdd90 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
pub fn stub_5bdd90() -> ! {
    todo!("0x5bdd90 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

// 0x5bdef8 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>)")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEEvT_
pub fn stub_5bdef8() -> ! {
    todo!("0x5bdef8 void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>)")
}

// 0x5be070 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
pub fn stub_5be070() -> ! {
    todo!("0x5be070 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x5be08c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEvS7_S8_NS_10shared_ptrIKSsEEE6invokeERNS1_15function_bufferES7_S8_SR_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEvS7_S8_NS_10shared_ptrIKSsEEE6invokeERNS1_15function_bufferES7_S8_SR_
pub fn stub_5be08c() -> ! {
    todo!("0x5be08c boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")
}

// 0x5be0b0 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS5_S6_NS_8weak_ptrINS3_24KeyframeSequenceProviderEEENSE_INS3_16KeyframeSequenceEEEENSC_5list4INS_3argILi1EEENSM_ILi2EEENSC_5valueISG_EENSP_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS5_S6_NS_8weak_ptrINS3_24KeyframeSequenceProviderEEENSE_INS3_16KeyframeSequenceEEEENSC_5list4INS_3argILi1EEENSM_ILi2EEENSC_5valueISG_EENSP_ISI_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_5be0b0() -> ! {
    todo!("0x5be0b0 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const")
}

// 0x5be214 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS5_S6_NS_8weak_ptrINS3_24KeyframeSequenceProviderEEENSE_INS3_16KeyframeSequenceEEEENSC_5list4INS_3argILi1EEENSM_ILi2EEENSC_5valueISG_EENSP_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, void *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS5_S6_NS_8weak_ptrINS3_24KeyframeSequenceProviderEEENSE_INS3_16KeyframeSequenceEEEENSC_5list4INS_3argILi1EEENSM_ILi2EEENSC_5valueISG_EENSP_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_5be214() -> ! {
    todo!("0x5be214 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x5be374 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvS5_S6_NS_8weak_ptrINS3_24KeyframeSequenceProviderEEENSE_INS3_16KeyframeSequenceEEEENSC_5list4INS_3argILi1EEENSM_ILi2EEENSC_5valueISG_EENSP_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvS5_S6_NS_8weak_ptrINS3_24KeyframeSequenceProviderEEENSE_INS3_16KeyframeSequenceEEEENSC_5list4INS_3argILi1EEENSM_ILi2EEENSC_5valueISG_EENSP_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_5be374() -> ! {
    todo!("0x5be374 void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x5be488 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEclIPFvNS7_14AsyncHttpQueue13RequestResultEPSiS9_SC_ENS0_5list3IRSH_RSI_RNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>::operator()<void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
// was: __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEclIPFvNS7_14AsyncHttpQueue13RequestResultEPSiS9_SC_ENS0_5list3IRSH_RSI_RNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_5be488() -> ! {
    todo!("0x5be488 void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>::operator()<void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")
}

// 0x5be5ac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_5be5ac() -> ! {
    todo!("0x5be5ac boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x5be76c — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEC2ES3_S4_SA_SD_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>)")]
// was: __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEC2ES3_S4_SA_SD_
pub fn stub_5be76c() -> ! {
    todo!("0x5be76c boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>)")
}

// 0x5be884 — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEC2ES3_S4_SA_SD_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>)")]
// was: __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEC2ES3_S4_SA_SD_
pub fn stub_5be884() -> ! {
    todo!("0x5be884 boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>)")
}

// 0x5be9a0 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE23removeLeastRecentlyUsedEv
// type: int(void)
#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::removeLeastRecentlyUsed(void)")]
// was: __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE23removeLeastRecentlyUsedEv
pub fn stub_5be9a0() -> ! {
    todo!("0x5be9a0 RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::removeLeastRecentlyUsed(void)")
}

// 0x5be9f8 — __ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_5be9f8() -> ! {
    todo!("0x5be9f8 rbx_core::SharedPtr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5beac0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16KeyframeSequenceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequence,RBX::KeyframeSequence>(rbx_core::SharedPtr<RBX::KeyframeSequence> const*,RBX::KeyframeSequence *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16KeyframeSequenceES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_5beac0() -> ! {
    todo!("0x5beac0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequence,RBX::KeyframeSequence>(rbx_core::SharedPtr<RBX::KeyframeSequence> const*,RBX::KeyframeSequence *)const")
}

// 0x5beba8 — __ZN5boost6detail12shared_countC2IPN3RBX16KeyframeSequenceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX16KeyframeSequenceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_5beba8() -> ! {
    todo!("0x5beba8 boost::detail::shared_count::shared_count<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5becb0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16KeyframeSequenceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16KeyframeSequenceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5becb0() -> ! {
    todo!("0x5becb0 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5becb4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16KeyframeSequenceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16KeyframeSequenceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5becb4() -> ! {
    todo!("0x5becb4 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5becb8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16KeyframeSequenceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16KeyframeSequenceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5becb8() -> ! {
    todo!("0x5becb8 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x5becd8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16KeyframeSequenceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16KeyframeSequenceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5becd8() -> ! {
    todo!("0x5becd8 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x5becf0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16KeyframeSequenceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16KeyframeSequenceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5becf0() -> ! {
    todo!("0x5becf0 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x5becf4 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_
pub fn stub_5becf4() -> ! {
    todo!("0x5becf4 std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::find(std::string const&)")
}

// 0x5bed48 — __ZN5boost6detail8function17function_invoker1IPFbPN3RBX8InstanceEEbS5_E6invokeERNS1_15function_bufferES5_
#[doc(alias = "boost::detail::function::function_invoker1<bool (*)(RBX::Instance *),bool,RBX::Instance *>::invoke(boost::detail::function::function_buffer &,RBX::Instance *)")]
// was: __ZN5boost6detail8function17function_invoker1IPFbPN3RBX8InstanceEEbS5_E6invokeERNS1_15function_bufferES5_
pub fn stub_5bed48() -> ! {
    todo!("0x5bed48 boost::detail::function::function_invoker1<bool (*)(RBX::Instance *),bool,RBX::Instance *>::invoke(boost::detail::function::function_buffer &,RBX::Instance *)")
}

// 0x5bed54 — __ZNSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEC2ERS0_RKS5_
#[doc(alias = "std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>::pair(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&)")]
// was: __ZNSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEC2ERS0_RKS5_
pub fn stub_5bed54() -> ! {
    todo!("0x5bed54 std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>::pair(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&)")
}

// 0x5bee10 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_5bee10() -> ! {
    todo!("0x5bee10 std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")
}

// 0x5beefc — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
pub fn stub_5beefc() -> ! {
    todo!("0x5beefc std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")
}

// 0x5bef4c — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_5bef4c() -> ! {
    todo!("0x5bef4c std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")
}

// 0x5befd0 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_
pub fn stub_5befd0() -> ! {
    todo!("0x5befd0 std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")
}

// 0x5bf0d8 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_
// type: _DWORD *__fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::lower_bound(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_
pub fn stub_5bf0d8() -> ! {
    todo!("0x5bf0d8 std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::lower_bound(std::string const&)")
}

// 0x5bf108 — __ZN3RBX10Reflection9DescribedINS_16KeyframeSequenceELZNS_17sKeyframeSequenceEENS_14FactoryProductIS2_NS_8InstanceELZNS_17sKeyframeSequenceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_16KeyframeSequenceELZNS_17sKeyframeSequenceEENS_14FactoryProductIS2_NS_8InstanceELZNS_17sKeyframeSequenceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_16KeyframeSequenceELZNS_17sKeyframeSequenceEENS_14FactoryProductIS2_NS_8InstanceELZNS_17sKeyframeSequenceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_5bf108() -> ! {
    todo!("0x5bf108 __ZN3RBX10Reflection9DescribedINS_16KeyframeSequenceELZNS_17sKeyframeSequenceEENS_14FactoryProductIS2_NS_8InstanceELZNS_17sKeyframeSequenceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x5bf228 — __ZN3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_5bf228() -> ! {
    todo!("0x5bf228 __ZN3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x5bf344 — __ZN3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5bf344() -> ! {
    todo!("0x5bf344 __ZN3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5bf348 — __ZN3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5bf348() -> ! {
    todo!("0x5bf348 __ZN3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5bf3e8 — __ZThn32_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5bf3e8() -> ! {
    todo!("0x5bf3e8 __ZThn32_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5bf3f0 — __ZThn32_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5bf3f0() -> ! {
    todo!("0x5bf3f0 __ZThn32_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5bf494 — __ZThn36_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5bf494() -> ! {
    todo!("0x5bf494 __ZThn36_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5bf49c — __ZThn36_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5bf49c() -> ! {
    todo!("0x5bf49c __ZThn36_N3RBX10Reflection9DescribedINS_24KeyframeSequenceProviderELZNS_25sKeyframeSequenceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5bf540 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEEC2Ev
#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::LRUCache(void)")]
// was: __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEEC2Ev
pub fn stub_5bf540() -> ! {
    todo!("0x5bf540 RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::LRUCache(void)")
}

// 0x5bf620 — __ZN3RBX20SizeEnforcedLRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6resizeEm
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::resize(unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6resizeEm
pub fn stub_5bf620() -> ! {
    todo!("0x5bf620 RBX::SizeEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::resize(unsigned long)")
}

// 0x5bf6a4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSH_RKSJ_RKSaINS1_8ptr_nodeISE_EEE
// type: int(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSH_RKSJ_RKSaINS1_8ptr_nodeISE_EEE
pub fn stub_5bf6a4() -> ! {
    todo!("0x5bf6a4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>> const&)")
}

// 0x5bf710 — __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EEC2EMS2_FS6_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EEC2EMS2_FS6_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_5bf710() -> ! {
    todo!("0x5bf710 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5bf888 — __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_5bf888() -> ! {
    todo!("0x5bf888 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x5bf8b8 — __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EED0Ev
pub fn stub_5bf8b8() -> ! {
    todo!("0x5bf8b8 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::~BoundFuncDesc()")
}

// 0x5bf984 — __ZNK3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_5bf984() -> ! {
    todo!("0x5bf984 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x5bfac4 — __ZN3RBX10Reflection11Call1HelperINS_24KeyframeSequenceProviderEMS2_FN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEES7_S6_E4callEPS2_S9_RNS0_7VariantERKS7_
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::ContentId,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::KeyframeSequenceProvider*,rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_24KeyframeSequenceProviderEMS2_FN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEES7_S6_E4callEPS2_S9_RNS0_7VariantERKS7_
pub fn stub_5bfac4() -> ! {
    todo!("0x5bfac4 RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::ContentId,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::KeyframeSequenceProvider*,rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")
}

// 0x5bfc50 — __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FS3_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FS3_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_5bfc50() -> ! {
    todo!("0x5bfc50 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5bfde8 — __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_5bfde8() -> ! {
    todo!("0x5bfde8 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}