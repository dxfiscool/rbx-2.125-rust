//! Auto-generated skeletons for rbx-network — RakNet|Network|Replicat|Socket|Upnp|HTTP EA-sorted asc (filtered 6232 ci / 5273 cs, 799 remaining after batch)
//! Filter: RakNet|Network|Replicat|Socket|Upnp|HTTP -> 5273 funcs (cs), 6232 (ci), 919 remaining before batch; batch EA-sorted asc next 120 filtered
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0x5a6a8..0x2feab0 | existing 18249 -> 18369 total (filtered EA-sorted asc, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x5a6a8 — -[LoginManager processSuccessfulLoginResponse:httpResponse:userLoginInfo:]
// type: id __cdecl(LoginManager *self, SEL, id, id, id)
#[doc(alias = "-[LoginManager processSuccessfulLoginResponse:httpResponse:userLoginInfo:]")]
pub fn stub_5a6a8() -> ! {
    todo!("0x5a6a8 -[LoginManager processSuccessfulLoginResponse:httpResponse:userLoginInfo:]")
}

// 0x5ac78 — -[LoginManager processSuccessfulLogoutResponse:httpResponse:]
// type: id __cdecl(LoginManager *self, SEL, id, id)
#[doc(alias = "-[LoginManager processSuccessfulLogoutResponse:httpResponse:]")]
pub fn stub_5ac78() -> ! {
    todo!("0x5ac78 -[LoginManager processSuccessfulLogoutResponse:httpResponse:]")
}

// 0xca58c — __ZN4FMOD7NetFile10openAsHTTPEPKcPcS3_S3_tPj
// demangled: FMOD::NetFile::openAsHTTP(char const*,char *,char *,char *,unsigned short,unsigned int *)
// type: _DWORD __fastcall(FMOD::NetFile *__hidden this, const char *, char *, char *, char *, unsigned __int16, unsigned int *)
#[doc(alias = "FMOD::NetFile::openAsHTTP(char const*,char *,char *,char *,unsigned short,unsigned int *)")]
pub fn stub_ca58c() -> ! {
    todo!("0xca58c FMOD::NetFile::openAsHTTP(char const*,char *,char *,char *,unsigned short,unsigned int *)")
}

// 0xcf1a8 — __Z24FMOD_Net_ParseHTTPStatusPciPiS0_
// demangled: FMOD_Net_ParseHTTPStatus(char *,int,int *,int *)
// type: _DWORD __fastcall(char *, int, int *, int *)
#[doc(alias = "FMOD_Net_ParseHTTPStatus(char *,int,int *,int *)")]
pub fn stub_cf1a8() -> ! {
    todo!("0xcf1a8 FMOD_Net_ParseHTTPStatus(char *,int,int *,int *)")
}

// 0x255d98 — __ZN3RBX11HttpService16userHttpGetAsyncESsN5boost8functionIFvSsEEES4_
// demangled: RBX::HttpService::userHttpGetAsync(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
// type: void __fastcall(int, int, int, int)
#[doc(alias = "RBX::HttpService::userHttpGetAsync(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
pub fn stub_255d98() -> ! {
    todo!("0x255d98 RBX::HttpService::userHttpGetAsync(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x25620c — __ZN3RBX11HttpService17userHttpPostAsyncESsSsNS0_15HttpContentTypeEN5boost8functionIFvSsEEES5_
// demangled: RBX::HttpService::userHttpPostAsync(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
// type: void __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::HttpService::userHttpPostAsync(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
pub fn stub_25620c() -> ! {
    todo!("0x25620c RBX::HttpService::userHttpPostAsync(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x256a6c — __ZN3RBX11HttpService10decodeJSONESs
// demangled: RBX::HttpService::decodeJSON(std::string)
// type: void __fastcall(_DWORD *, int, _DWORD *)
#[doc(alias = "RBX::HttpService::decodeJSON(std::string)")]
pub fn stub_256a6c() -> ! {
    todo!("0x256a6c RBX::HttpService::decodeJSON(std::string)")
}

// 0x256d10 — __ZN3RBX11HttpService10encodeJSONEN5boost10shared_ptrIKNS1_9unordered13unordered_mapISsNS_10Reflection7VariantENS1_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEEE
// demangled: RBX::HttpService::encodeJSON(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::HttpService::encodeJSON(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_256d10() -> ! {
    todo!("0x256d10 RBX::HttpService::encodeJSON(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")
}

// 0x256ef8 — __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::EnumDesc(void)
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::EnumDesc(void)")]
pub fn stub_256ef8() -> ! {
    todo!("0x256ef8 RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::EnumDesc(void)")
}

// 0x256efc — __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::EnumDesc(void)
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::EnumDesc(void)")]
pub fn stub_256efc() -> ! {
    todo!("0x256efc RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::EnumDesc(void)")
}

// 0x2570c0 — __ZN3RBX15StringConverterINS_11HttpService15HttpContentTypeEE14convertToValueERKSsRS2_
// demangled: RBX::StringConverter<RBX::HttpService::HttpContentType>::convertToValue(std::string const&,RBX::HttpService::HttpContentType&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::HttpService::HttpContentType>::convertToValue(std::string const&,RBX::HttpService::HttpContentType&)")]
pub fn stub_2570c0() -> ! {
    todo!("0x2570c0 RBX::StringConverter<RBX::HttpService::HttpContentType>::convertToValue(std::string const&,RBX::HttpService::HttpContentType&)")
}

// 0x257110 — __ZN3RBX11HttpServiceC2Ev
// demangled: RBX::HttpService::HttpService(void)
// type: RBX::Instance *__fastcall(RBX::HttpService *this)
#[doc(alias = "RBX::HttpService::HttpService(void)")]
pub fn stub_257110() -> ! {
    todo!("0x257110 RBX::HttpService::HttpService(void)")
}

// 0x257338 — __ZN3RBX11HttpService15checkEverythingERSsN5boost8functionIFvSsEEE
// demangled: RBX::HttpService::checkEverything(std::string &,boost::function<void ()(std::string)>)
// type: int __fastcall(RBX::HttpService *, _DWORD *, int, int, int, int, int, int, int, int, int, int, char, int, int, char, char, int, int, char, char, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::HttpService::checkEverything(std::string &,boost::function<void ()(std::string)>)")]
pub fn stub_257338() -> ! {
    todo!("0x257338 RBX::HttpService::checkEverything(std::string &,boost::function<void ()(std::string)>)")
}

// 0x257758 — __ZN3RBX11HttpService18checkUserHasAccessEv
// demangled: RBX::HttpService::checkUserHasAccess(void)
// type: bool __fastcall(RBX::HttpService *this, RBX::Instance *)
#[doc(alias = "RBX::HttpService::checkUserHasAccess(void)")]
pub fn stub_257758() -> ! {
    todo!("0x257758 RBX::HttpService::checkUserHasAccess(void)")
}

// 0x2577c0 — __ZN3RBX11HttpService10checkLimitEv
// demangled: RBX::HttpService::checkLimit(void)
// type: bool __fastcall(RBX::HttpService *this)
#[doc(alias = "RBX::HttpService::checkLimit(void)")]
pub fn stub_2577c0() -> ! {
    todo!("0x2577c0 RBX::HttpService::checkLimit(void)")
}

// 0x257980 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EED1Ev
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()")]
pub fn stub_257980() -> ! {
    todo!("0x257980 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()")
}

// 0x2579c0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EED1Ev
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::~BoundYieldFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::~BoundYieldFuncDesc()")]
pub fn stub_2579c0() -> ! {
    todo!("0x2579c0 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::~BoundYieldFuncDesc()")
}

// 0x257a10 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_257a10() -> ! {
    todo!("0x257a10 RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc()")
}

// 0x257a50 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc()")]
pub fn stub_257a50() -> ! {
    todo!("0x257a50 RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc()")
}

// 0x257b5c — __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE7addPairES3_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::addPair(RBX::HttpService::HttpContentType,char const*)
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::addPair(RBX::HttpService::HttpContentType,char const*)")]
pub fn stub_257b5c() -> ! {
    todo!("0x257b5c RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::addPair(RBX::HttpService::HttpContentType,char const*)")
}

// 0x257ebc — __ZN3RBX10Reflection7Variant14genericConvertINS_11HttpService15HttpContentTypeEEERT_v
// demangled: RBX::HttpService::HttpContentType & RBX::Reflection::Variant::genericConvert<RBX::HttpService::HttpContentType>(void)
// type: int __fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::HttpService::HttpContentType & RBX::Reflection::Variant::genericConvert<RBX::HttpService::HttpContentType>(void)")]
pub fn stub_257ebc() -> ! {
    todo!("0x257ebc RBX::HttpService::HttpContentType & RBX::Reflection::Variant::genericConvert<RBX::HttpService::HttpContentType>(void)")
}

// 0x2580a8 — __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorD1Ev")]
pub fn stub_2580a8() -> ! {
    todo!("0x2580a8 __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorD1Ev")
}

// 0x2580ac — __ZN3RBX11HttpServiceD1Ev
// demangled: RBX::HttpService::~HttpService()
// type: void __fastcall(RBX::HttpService *__hidden this)
#[doc(alias = "RBX::HttpService::~HttpService()")]
pub fn stub_2580ac() -> ! {
    todo!("0x2580ac RBX::HttpService::~HttpService()")
}

// 0x2580b0 — __ZN3RBX11HttpServiceD0Ev
// demangled: RBX::HttpService::~HttpService()
// type: void __fastcall(RBX::HttpService *__hidden this)
#[doc(alias = "RBX::HttpService::~HttpService()")]
pub fn stub_2580b0() -> ! {
    todo!("0x2580b0 RBX::HttpService::~HttpService()")
}

// 0x258150 — __ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E12getClassNameEv")]
pub fn stub_258150() -> ! {
    todo!("0x258150 __ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E12getClassNameEv")
}

// 0x258160 — __ZThn32_N3RBX11HttpServiceD1Ev
// demangled: `non-virtual thunk to'RBX::HttpService::~HttpService()
// type: void __fastcall(RBX::HttpService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HttpService::~HttpService()")]
pub fn stub_258160() -> ! {
    todo!("0x258160 `non-virtual thunk to'RBX::HttpService::~HttpService()")
}

// 0x258168 — __ZThn32_N3RBX11HttpServiceD0Ev
// demangled: `non-virtual thunk to'RBX::HttpService::~HttpService()
// type: void __fastcall(RBX::HttpService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HttpService::~HttpService()")]
pub fn stub_258168() -> ! {
    todo!("0x258168 `non-virtual thunk to'RBX::HttpService::~HttpService()")
}

// 0x25820c — __ZThn32_NK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E12getClassNameEv")]
pub fn stub_25820c() -> ! {
    todo!("0x25820c __ZThn32_NK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E12getClassNameEv")
}

// 0x25821c — __ZThn36_N3RBX11HttpServiceD1Ev
// demangled: `non-virtual thunk to'RBX::HttpService::~HttpService()
// type: void __fastcall(RBX::HttpService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HttpService::~HttpService()")]
pub fn stub_25821c() -> ! {
    todo!("0x25821c `non-virtual thunk to'RBX::HttpService::~HttpService()")
}

// 0x258224 — __ZThn36_N3RBX11HttpServiceD0Ev
// demangled: `non-virtual thunk to'RBX::HttpService::~HttpService()
// type: void __fastcall(RBX::HttpService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HttpService::~HttpService()")]
pub fn stub_258224() -> ! {
    todo!("0x258224 `non-virtual thunk to'RBX::HttpService::~HttpService()")
}

// 0x2582c8 — __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E17static_getCreatorEv")]
pub fn stub_2582c8() -> ! {
    todo!("0x2582c8 __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E17static_getCreatorEv")
}

// 0x25833c — __ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7Creator12getClassNameEv")]
pub fn stub_25833c() -> ! {
    todo!("0x25833c __ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7Creator12getClassNameEv")
}

// 0x2583c4 — __ZN3RBX4Name13callDoDeclareILZNS_12sHttpServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sHttpServiceEEEEvv")]
pub fn stub_2583c4() -> ! {
    todo!("0x2583c4 __ZN3RBX4Name13callDoDeclareILZNS_12sHttpServiceEEEEvv")
}

// 0x2583c8 — __ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v")]
pub fn stub_2583c8() -> ! {
    todo!("0x2583c8 __ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v")
}

// 0x2584a8 — __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorD2Ev")]
pub fn stub_2584a8() -> ! {
    todo!("0x2584a8 __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorD2Ev")
}

// 0x258544 — __ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7Creator6createEv
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7Creator6createEv")]
pub fn stub_258544() -> ! {
    todo!("0x258544 __ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7Creator6createEv")
}

// 0x258688 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11HttpServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)")]
pub fn stub_258688() -> ! {
    todo!("0x258688 boost::shared_ptr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)")
}

// 0x258738 — __ZN5boost10shared_ptrIN3RBX11HttpServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_258738() -> ! {
    todo!("0x258738 boost::shared_ptr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x258800 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11HttpServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpService,RBX::HttpService>(boost::shared_ptr<RBX::HttpService> const*,RBX::HttpService *)const
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpService,RBX::HttpService>(rbx_core::SharedPtr<RBX::HttpService> const*,RBX::HttpService *)const")]
pub fn stub_258800() -> ! {
    todo!("0x258800 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpService,RBX::HttpService>(boost::shared_ptr<RBX::HttpService> const*,RBX::HttpService *)const")
}

// 0x2588e8 — __ZN5boost6detail12shared_countC2IPN3RBX11HttpServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_2588e8() -> ! {
    todo!("0x2588e8 boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2589f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2589f0() -> ! {
    todo!("0x2589f0 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2589f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2589f4() -> ! {
    todo!("0x2589f4 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2589f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_2589f8() -> ! {
    todo!("0x2589f8 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x258a18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_258a18() -> ! {
    todo!("0x258a18 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x258a30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_258a30() -> ! {
    todo!("0x258a30 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x258a34 — __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorC2Ev")]
pub fn stub_258a34() -> ! {
    todo!("0x258a34 __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorC2Ev")
}

// 0x258c78 — __ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_258c78() -> ! {
    todo!("0x258c78 __ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x258c7c — __ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_258c7c() -> ! {
    todo!("0x258c7c __ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x258d1c — __ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_258d1c() -> ! {
    todo!("0x258d1c __ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x258d24 — __ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_258d24() -> ! {
    todo!("0x258d24 __ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x258dc8 — __ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_258dc8() -> ! {
    todo!("0x258dc8 __ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x258dd0 — __ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_258dd0() -> ! {
    todo!("0x258dd0 __ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x258e74 — __ZN3rbx8any_castIN3RBX11HttpService15HttpContentTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// demangled: RBX::HttpService::HttpContentType * rbx::any_cast<RBX::HttpService::HttpContentType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::HttpService::HttpContentType * rbx::any_cast<RBX::HttpService::HttpContentType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_258e74() -> ! {
    todo!("0x258e74 RBX::HttpService::HttpContentType * rbx::any_cast<RBX::HttpService::HttpContentType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x258ecc — __ZN3rbx8any_castIRN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: RBX::HttpService::HttpContentType & rbx::any_cast<RBX::HttpService::HttpContentType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::HttpService::HttpContentType & rbx::any_cast<RBX::HttpService::HttpContentType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_258ecc() -> ! {
    todo!("0x258ecc RBX::HttpService::HttpContentType & rbx::any_cast<RBX::HttpService::HttpContentType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x258fbc — __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE6resizeEmS2_
// demangled: std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::resize(unsigned long,RBX::HttpService::HttpContentType)
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::resize(unsigned long,RBX::HttpService::HttpContentType)")]
pub fn stub_258fbc() -> ! {
    todo!("0x258fbc std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::resize(unsigned long,RBX::HttpService::HttpContentType)")
}

// 0x258ff0 — __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::push_back(RBX::HttpService::HttpContentType const&)
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::push_back(RBX::HttpService::HttpContentType const&)")]
pub fn stub_258ff0() -> ! {
    todo!("0x258ff0 std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::push_back(RBX::HttpService::HttpContentType const&)")
}

// 0x259018 — __ZNSt3mapIPKN3RBX4NameENS0_11HttpService15HttpContentTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// demangled: std::map<RBX::Name const*,RBX::HttpService::HttpContentType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::operator[](RBX::Name const* const&)
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::HttpService::HttpContentType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_259018() -> ! {
    todo!("0x259018 std::map<RBX::Name const*,RBX::HttpService::HttpContentType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::operator[](RBX::Name const* const&)")
}

// 0x259070 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
pub fn stub_259070() -> ! {
    todo!("0x259070 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")
}

// 0x259124 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
pub fn stub_259124() -> ! {
    todo!("0x259124 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")
}

// 0x25917c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
pub fn stub_25917c() -> ! {
    todo!("0x25917c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")
}

// 0x2591e4 — __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,RBX::HttpService::HttpContentType const&)
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,RBX::HttpService::HttpContentType const&)")]
pub fn stub_2591e4() -> ! {
    todo!("0x2591e4 std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,RBX::HttpService::HttpContentType const&)")
}

// 0x2592c8 — __ZNSt12_Vector_baseIN3RBX11HttpService15HttpContentTypeESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_allocate(unsigned long)
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_allocate(unsigned long)")]
pub fn stub_2592c8() -> ! {
    todo!("0x2592c8 std::_Vector_base<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_allocate(unsigned long)")
}

// 0x2592e0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11HttpService15HttpContentTypeES6_EET0_T_S8_S7_
// demangled: RBX::HttpService::HttpContentType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *>(RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::HttpService::HttpContentType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *>(RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *)")]
pub fn stub_2592e0() -> ! {
    todo!("0x2592e0 RBX::HttpService::HttpContentType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *>(RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *)")
}

// 0x25931c — __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// demangled: std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,unsigned long,RBX::HttpService::HttpContentType const&)
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,unsigned long,RBX::HttpService::HttpContentType const&)")]
pub fn stub_25931c() -> ! {
    todo!("0x25931c std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,unsigned long,RBX::HttpService::HttpContentType const&)")
}

// 0x2594ac — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_11HttpServiceEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HttpService>(char const*,char const*,bool RBX::HttpService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HttpService>(char const*,char const*,bool RBX::HttpService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_2594ac() -> ! {
    todo!("0x2594ac RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HttpService>(char const*,char const*,bool RBX::HttpService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x25963c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11HttpServiceEE10isReadOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::isReadOnly(void)const")]
pub fn stub_25963c() -> ! {
    todo!("0x25963c RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::isReadOnly(void)const")
}

// 0x259640 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11HttpServiceEE11isWriteOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::isWriteOnly(void)const")]
pub fn stub_259640() -> ! {
    todo!("0x259640 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::isWriteOnly(void)const")
}

// 0x259644 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11HttpServiceEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_259644() -> ! {
    todo!("0x259644 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x259650 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11HttpServiceEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_259650() -> ! {
    todo!("0x259650 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x2596a0 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EEC2EMS2_FSsSI_EPKcSO_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::BoundFuncDesc(std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::BoundFuncDesc(std::string (RBX::HttpService::*)(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_2596a0() -> ! {
    todo!("0x2596a0 RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::BoundFuncDesc(std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x259838 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EE16declareSignatureEPKcS7_
// demangled: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_259838() -> ! {
    todo!("0x259838 RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x259868 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EED0Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc()
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc()")]
pub fn stub_259868() -> ! {
    todo!("0x259868 RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc()")
}

// 0x259984 — __ZNK3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_259984() -> ! {
    todo!("0x259984 RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x259a6c — __ZN3RBX10Reflection11Call1HelperINS_11HttpServiceEMS2_FSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEESI_SsE4callEPS2_SK_RS7_RKSI_
// demangled: RBX::Reflection::Call1Helper<RBX::HttpService,std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,std::string>::call(RBX::HttpService*,std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),RBX::Reflection::Variant&,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)
// type: void __fastcall(int, char *, int, _DWORD *, const shared_count *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::HttpService,std::string (RBX::HttpService::*)(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,std::string>::call(RBX::HttpService*,std::string (RBX::HttpService::*)(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),RBX::Reflection::Variant&,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")]
pub fn stub_259a6c() -> ! {
    todo!("0x259a6c RBX::Reflection::Call1Helper<RBX::HttpService,std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,std::string>::call(RBX::HttpService*,std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),RBX::Reflection::Variant&,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")
}

// 0x259dbc — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EEC2EMS2_FSI_SsEPKcSO_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_259dbc() -> ! {
    todo!("0x259dbc RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x259f34 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EE16declareSignatureEPKcS7_
// demangled: RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_259f34() -> ! {
    todo!("0x259f34 RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x259f64 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EED0Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc()
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_259f64() -> ! {
    todo!("0x259f64 RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc()")
}

// 0x25a030 — __ZNK3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_25a030() -> ! {
    todo!("0x25a030 RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x25a170 — __ZN3RBX10Reflection11Call1HelperINS_11HttpServiceEMS2_FN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsESsSI_E4callEPS2_SK_RS7_RSD_
// demangled: RBX::Reflection::Call1Helper<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::HttpService*,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),RBX::Reflection::Variant&,std::string const&)
// type: void __fastcall(int, char *, int, _DWORD *, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),std::string,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::HttpService*,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),RBX::Reflection::Variant&,std::string const&)")]
pub fn stub_25a170() -> ! {
    todo!("0x25a170 RBX::Reflection::Call1Helper<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::HttpService*,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),RBX::Reflection::Variant&,std::string const&)")
}

// 0x25a2f0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EEC2EMS2_FvSsSsS3_N5boost8functionIFvSsEEES9_EPKcSD_SD_SD_S3_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,char const*,RBX::HttpService::HttpContentType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,char const*,RBX::HttpService::HttpContentType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_25a2f0() -> ! {
    todo!("0x25a2f0 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,char const*,RBX::HttpService::HttpContentType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x25a540 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EE16declareSignatureEPKcNS0_7VariantES7_S8_S7_S8_
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_25a540() -> ! {
    todo!("0x25a540 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x25a5a8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EED0Ev
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::~BoundYieldFuncDesc()
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::~BoundYieldFuncDesc()")]
pub fn stub_25a5a8() -> ! {
    todo!("0x25a5a8 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::~BoundYieldFuncDesc()")
}

// 0x25a68c — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSC_IFvSsEEE
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
// type: void __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_25a68c() -> ! {
    todo!("0x25a68c RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x25a958 — __ZN3RBX10Reflection9ArgHelper6getArgINS_11HttpService15HttpContentTypeELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// demangled: RBX::HttpService::HttpContentType RBX::Reflection::ArgHelper::getArg<RBX::HttpService::HttpContentType,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::HttpService::HttpContentType> const&,boost::disable_if<boost::is_same<RBX::HttpService::HttpContentType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
// type: int (__fastcall ***__fastcall(int (__fastcall ***)(_DWORD), int))(_DWORD)
#[doc(alias = "RBX::HttpService::HttpContentType RBX::Reflection::ArgHelper::getArg<RBX::HttpService::HttpContentType,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::HttpService::HttpContentType> const&,boost::disable_if<boost::is_same<RBX::HttpService::HttpContentType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_25a958() -> ! {
    todo!("0x25a958 RBX::HttpService::HttpContentType RBX::Reflection::ArgHelper::getArg<RBX::HttpService::HttpContentType,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::HttpService::HttpContentType> const&,boost::disable_if<boost::is_same<RBX::HttpService::HttpContentType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x25aaec — __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_11HttpService15HttpContentTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// demangled: bool RBX::Reflection::ArgHelper::try_enum<3,RBX::HttpService::HttpContentType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::HttpService::HttpContentType &,boost::enable_if<boost::is_enum<RBX::HttpService::HttpContentType>,void>::type *)
// type: int __fastcall(int, _DWORD *, int, int)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::HttpService::HttpContentType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::HttpService::HttpContentType &,boost::enable_if<boost::is_enum<RBX::HttpService::HttpContentType>,void>::type *)")]
pub fn stub_25aaec() -> ! {
    todo!("0x25aaec bool RBX::Reflection::ArgHelper::try_enum<3,RBX::HttpService::HttpContentType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::HttpService::HttpContentType &,boost::enable_if<boost::is_enum<RBX::HttpService::HttpContentType>,void>::type *)")
}

// 0x25ab40 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EEC2EMS2_FvSsN5boost8functionIFvSsEEES8_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_25ab40() -> ! {
    todo!("0x25ab40 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x25acb8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EE16declareSignatureEPKcNS0_7VariantE
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::declareSignature(char const*,RBX::Reflection::Variant)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_25acb8() -> ! {
    todo!("0x25acb8 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x25ace8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EED0Ev
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()")]
pub fn stub_25ace8() -> ! {
    todo!("0x25ace8 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()")
}

// 0x25adb4 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
// type: void __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_25adb4() -> ! {
    todo!("0x25adb4 RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x2faa84 — __ZN3RBX14AsyncHttpQueueC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEi
// demangled: RBX::AsyncHttpQueue::AsyncHttpQueue(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int)
// type: RBX::AsyncHttpQueue *__fastcall(RBX::AsyncHttpQueue *, int, void *, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, int, pthread_mutex_t *, int, int, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpQueue::AsyncHttpQueue(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int)")]
pub fn stub_2faa84() -> ! {
    todo!("0x2faa84 RBX::AsyncHttpQueue::AsyncHttpQueue(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int)")
}

// 0x2fad24 — __ZN3RBX14AsyncHttpQueue13setThreadPoolEi
// demangled: RBX::AsyncHttpQueue::setThreadPool(int)
// type: _DWORD __fastcall(RBX::AsyncHttpQueue *__hidden this, int)
#[doc(alias = "RBX::AsyncHttpQueue::setThreadPool(int)")]
pub fn stub_2fad24() -> ! {
    todo!("0x2fad24 RBX::AsyncHttpQueue::setThreadPool(int)")
}

// 0x2fae00 — __ZN3RBX14AsyncHttpQueue14resetStatsItemEPNS_15ServiceProviderE
// demangled: RBX::AsyncHttpQueue::resetStatsItem(RBX::ServiceProvider *)
// type: _DWORD __fastcall(RBX::AsyncHttpQueue *__hidden this, RBX::ServiceProvider *)
#[doc(alias = "RBX::AsyncHttpQueue::resetStatsItem(RBX::ServiceProvider *)")]
pub fn stub_2fae00() -> ! {
    todo!("0x2fae00 RBX::AsyncHttpQueue::resetStatsItem(RBX::ServiceProvider *)")
}

// 0x2faf2c — __ZNK3RBX14AsyncHttpQueue19getRequestQueueSizeEv
// demangled: RBX::AsyncHttpQueue::getRequestQueueSize(void)const
// type: _DWORD __fastcall(RBX::AsyncHttpQueue *__hidden this)
#[doc(alias = "RBX::AsyncHttpQueue::getRequestQueueSize(void)const")]
pub fn stub_2faf2c() -> ! {
    todo!("0x2faf2c RBX::AsyncHttpQueue::getRequestQueueSize(void)const")
}

// 0x2faf68 — __ZN3RBX14AsyncHttpQueueD0Ev
// demangled: RBX::AsyncHttpQueue::~AsyncHttpQueue()
// type: void __fastcall(RBX::AsyncHttpQueue *__hidden this)
#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
pub fn stub_2faf68() -> ! {
    todo!("0x2faf68 RBX::AsyncHttpQueue::~AsyncHttpQueue()")
}

// 0x2fb008 — __ZN3RBX14AsyncHttpQueueD1Ev
// demangled: RBX::AsyncHttpQueue::~AsyncHttpQueue()
// type: void __fastcall(RBX::AsyncHttpQueue *__hidden this)
#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
pub fn stub_2fb008() -> ! {
    todo!("0x2fb008 RBX::AsyncHttpQueue::~AsyncHttpQueue()")
}

// 0x2fb00c — __ZN3RBX14AsyncHttpQueueD2Ev
// demangled: RBX::AsyncHttpQueue::~AsyncHttpQueue()
// type: void __fastcall(RBX::AsyncHttpQueue *__hidden this)
#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
pub fn stub_2fb00c() -> ! {
    todo!("0x2fb00c RBX::AsyncHttpQueue::~AsyncHttpQueue()")
}

// 0x2fb2ac — __ZN3RBX14AsyncHttpQueue11onHeartbeatERKNS_9HeartbeatE
// demangled: RBX::AsyncHttpQueue::onHeartbeat(RBX::Heartbeat const&)
// type: int __fastcall(int, int, int, int, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, pthread_mutex_t *, char, pthread_mutex_t *, char, void *, int, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpQueue::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_2fb2ac() -> ! {
    todo!("0x2fb2ac RBX::AsyncHttpQueue::onHeartbeat(RBX::Heartbeat const&)")
}

// 0x2fb548 — __ZN3RBX14AsyncHttpQueue15processRequestsEN5boost8weak_ptrIS0_EESt14_List_iteratorINS0_7RequestEENS1_10shared_ptrINS_5mutexEEE
// demangled: RBX::AsyncHttpQueue::processRequests(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>)
#[doc(alias = "RBX::AsyncHttpQueue::processRequests(Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>)")]
pub fn stub_2fb548() -> ! {
    todo!("0x2fb548 RBX::AsyncHttpQueue::processRequests(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>)")
}

// 0x2fc440 — __ZN3RBX14AsyncHttpQueue23dispatchGenericCallbackEN5boost8functionIFvPNS_9DataModelEEEEPNS_8InstanceENS0_9ResultJobE
// demangled: RBX::AsyncHttpQueue::dispatchGenericCallback(boost::function<void ()(RBX::DataModel *)>,RBX::Instance *,RBX::AsyncHttpQueue::ResultJob)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::AsyncHttpQueue::dispatchGenericCallback(boost::function<void ()(RBX::DataModel *)>,RBX::Instance *,RBX::AsyncHttpQueue::ResultJob)")]
pub fn stub_2fc440() -> ! {
    todo!("0x2fc440 RBX::AsyncHttpQueue::dispatchGenericCallback(boost::function<void ()(RBX::DataModel *)>,RBX::Instance *,RBX::AsyncHttpQueue::ResultJob)")
}

// 0x2fc6a8 — __ZN3RBX14AsyncHttpQueue16dispatchCallbackEN5boost8functionIFvNS0_13RequestResultEPSiNS1_10shared_ptrIKSsEEEEEPNS_8InstanceES3_S7_NS0_9ResultJobE
// demangled: RBX::AsyncHttpQueue::dispatchCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>,RBX::AsyncHttpQueue::ResultJob)
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpQueue::dispatchCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>,RBX::AsyncHttpQueue::ResultJob)")]
pub fn stub_2fc6a8() -> ! {
    todo!("0x2fc6a8 RBX::AsyncHttpQueue::dispatchCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>,RBX::AsyncHttpQueue::ResultJob)")
}

// 0x2fc874 — __ZN3RBXL19InvokeAsyncCallbackEN5boost8functionIFvNS_14AsyncHttpQueue13RequestResultEPSiNS0_10shared_ptrIKSsEEEEES3_S7_
// demangled: RBX::InvokeAsyncCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>)
// type: void __fastcall(int, int, const shared_count *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::InvokeAsyncCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_2fc874() -> ! {
    todo!("0x2fc874 RBX::InvokeAsyncCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>)")
}

// 0x2fca04 — __ZN3RBX14AsyncHttpQueue19isRequestQueueEmptyEv
// demangled: RBX::AsyncHttpQueue::isRequestQueueEmpty(void)
// type: _DWORD __fastcall(RBX::AsyncHttpQueue *__hidden this)
#[doc(alias = "RBX::AsyncHttpQueue::isRequestQueueEmpty(void)")]
pub fn stub_2fca04() -> ! {
    todo!("0x2fca04 RBX::AsyncHttpQueue::isRequestQueueEmpty(void)")
}

// 0x2fcfb4 — __ZN3RBXL8callbackENS_14AsyncHttpQueue15CallbackWrapperEPNS_8InstanceENS0_13RequestResultEN5boost10shared_ptrISsEE
// demangled: RBX::callback(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>)
// type: int __fastcall(char, int, int, boost::detail::sp_counted_base *, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::callback(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>)")]
pub fn stub_2fcfb4() -> ! {
    todo!("0x2fcfb4 RBX::callback(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>)")
}

// 0x2fd150 — __ZN3RBX14AsyncHttpQueue9FailedUrlC2EPKc
// demangled: RBX::AsyncHttpQueue::FailedUrl::FailedUrl(char const*)
// type: _DWORD __fastcall(RBX::AsyncHttpQueue::FailedUrl *__hidden this, const char *)
#[doc(alias = "RBX::AsyncHttpQueue::FailedUrl::FailedUrl(char const*)")]
pub fn stub_2fd150() -> ! {
    todo!("0x2fd150 RBX::AsyncHttpQueue::FailedUrl::FailedUrl(char const*)")
}

// 0x2fd220 — __ZN3RBX14AsyncHttpQueue8isUrlBadERKSs
// demangled: RBX::AsyncHttpQueue::isUrlBad(std::string const&)
// type: _DWORD __fastcall(RBX::AsyncHttpQueue *__hidden this, const std::string *)
#[doc(alias = "RBX::AsyncHttpQueue::isUrlBad(std::string const&)")]
pub fn stub_2fd220() -> ! {
    todo!("0x2fd220 RBX::AsyncHttpQueue::isUrlBad(std::string const&)")
}

// 0x2fd37c — __ZN3RBX14AsyncHttpQueue12asyncRequestERKSsfPN5boost8functionIFvNS0_13RequestResultEPSiNS3_10shared_ptrIS1_EEEEENS0_9ResultJobEb
// demangled: RBX::AsyncHttpQueue::asyncRequest(std::string const&,float,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)> *,RBX::AsyncHttpQueue::ResultJob,bool)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::AsyncHttpQueue::asyncRequest(std::string const&,float,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)> *,RBX::AsyncHttpQueue::ResultJob,bool)")]
pub fn stub_2fd37c() -> ! {
    todo!("0x2fd37c RBX::AsyncHttpQueue::asyncRequest(std::string const&,float,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)> *,RBX::AsyncHttpQueue::ResultJob,bool)")
}

// 0x2fd910 — __ZN3RBX14AsyncHttpQueue11syncRequestERKSs
// demangled: RBX::AsyncHttpQueue::syncRequest(std::string const&)
// type: _DWORD __fastcall(RBX::AsyncHttpQueue *__hidden this, const std::string *)
#[doc(alias = "RBX::AsyncHttpQueue::syncRequest(std::string const&)")]
pub fn stub_2fd910() -> ! {
    todo!("0x2fd910 RBX::AsyncHttpQueue::syncRequest(std::string const&)")
}

// 0x2fded0 — __ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEaSERKS3_
// demangled: boost::shared_ptr<RBX::HttpQueueStatsItem>::operator=(boost::shared_ptr<RBX::HttpQueueStatsItem> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem>::operator=(rbx_core::SharedPtr<RBX::HttpQueueStatsItem> const&)")]
pub fn stub_2fded0() -> ! {
    todo!("0x2fded0 boost::shared_ptr<RBX::HttpQueueStatsItem>::operator=(boost::shared_ptr<RBX::HttpQueueStatsItem> const&)")
}

// 0x2fdf08 — __ZN3RBX18HttpQueueStatsItem6createEPNS_14AsyncHttpQueueEPNS_8InstanceE
// demangled: RBX::HttpQueueStatsItem::create(RBX::AsyncHttpQueue *,RBX::Instance *)
// type: _DWORD __fastcall(RBX::HttpQueueStatsItem *__hidden this, RBX::AsyncHttpQueue *, RBX::Instance *)
#[doc(alias = "RBX::HttpQueueStatsItem::create(RBX::AsyncHttpQueue *,RBX::Instance *)")]
pub fn stub_2fdf08() -> ! {
    todo!("0x2fdf08 RBX::HttpQueueStatsItem::create(RBX::AsyncHttpQueue *,RBX::Instance *)")
}

// 0x2fdfbc — __ZN5boost4bindIvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS3_7RequestEENS_10shared_ptrINS2_5mutexEEES4_S7_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_
// demangled: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list_av_3<boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>::type> boost::bind<void,boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>,boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>(void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list_av_3<Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>::type> boost::bind<void,Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>,Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>(void (*)(Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>)")]
pub fn stub_2fdfbc() -> ! {
    todo!("0x2fdfbc boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list_av_3<boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>::type> boost::bind<void,boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>,boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>(void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>)")
}

// 0x2fe168 — __ZN3RBX9weak_fromINS_14AsyncHttpQueueEEEN5boost8weak_ptrIT_EEPS4_
// demangled: boost::weak_ptr<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)
#[doc(alias = "Weak<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)")]
pub fn stub_2fe168() -> ! {
    todo!("0x2fe168 boost::weak_ptr<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)")
}

// 0x2fe358 — __ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES4_S8_SA_S4_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_
// demangled: boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>)
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_2fe358() -> ! {
    todo!("0x2fe358 boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>)")
}

// 0x2fe5f0 — __ZN5boost10shared_ptrIN3RBX4HttpEEaSERKS3_
// demangled: boost::shared_ptr<RBX::Http>::operator=(boost::shared_ptr<RBX::Http> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Http>::operator=(rbx_core::SharedPtr<RBX::Http> const&)")]
pub fn stub_2fe5f0() -> ! {
    todo!("0x2fe5f0 boost::shared_ptr<RBX::Http>::operator=(boost::shared_ptr<RBX::Http> const&)")
}

// 0x2fe654 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEaSERKS4_
// demangled: std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
pub fn stub_2fe654() -> ! {
    todo!("0x2fe654 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")
}

// 0x2fe884 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN3RBX14AsyncHttpQueue15CallbackWrapperESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvPFvS4_PNS2_8InstanceENS3_13RequestResultENSA_10shared_ptrISsEEENSB_5list4INSA_3argILi1EEENSB_5valueISE_EENSN_ISF_EENSN_ISH_EEEEEEET0_T_SU_ST_
// demangled: boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>>>(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>>)
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>>>(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>>)")]
pub fn stub_2fe884() -> ! {
    todo!("0x2fe884 boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>>>(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>>)")
}

// 0x2fe8f4 — __ZN5boost4bindIvN3RBX14AsyncHttpQueue15CallbackWrapperEPNS1_8InstanceENS2_13RequestResultENS_10shared_ptrISsEENS_3argILi1EEES5_S6_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_T3_ENSB_9list_av_4IT4_T5_T6_T7_E4typeEEESJ_SL_SM_SN_SO_
// demangled: boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list_av_4<boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>>::type> boost::bind<void,RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>,boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>>(void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list_av_4<boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>>::type> boost::bind<void,RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>,boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>>(void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>)")]
pub fn stub_2fe8f4() -> ! {
    todo!("0x2fe8f4 boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list_av_4<boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>>::type> boost::bind<void,RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>,boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>>(void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>)")
}

// 0x2fea20 — __ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_
// demangled: std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)
// type: int __fastcall(int, std::_List_node_base *this)
#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")]
pub fn stub_2fea20() -> ! {
    todo!("0x2fea20 std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")
}

// 0x2fea58 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")]
pub fn stub_2fea58() -> ! {
    todo!("0x2fea58 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")
}

// 0x2feaa8 — __ZN3RBX14AsyncHttpQueue15registerContentERKSsN5boost10shared_ptrIS1_EES5_
// demangled: RBX::AsyncHttpQueue::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)
#[doc(alias = "RBX::AsyncHttpQueue::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_2feaa8() -> ! {
    todo!("0x2feaa8 RBX::AsyncHttpQueue::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)")
}

// 0x2feab0 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&)")]
pub fn stub_2feab0() -> ! {
    todo!("0x2feab0 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&)")
}