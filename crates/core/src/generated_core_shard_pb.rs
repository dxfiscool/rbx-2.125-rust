//! core shard pb — 100 core stubs EA-sorted, 0xf28ae4..0xf2d7d4 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered, global-deduped).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::TCritical(unsigned int,RBX::Confidence)")]
#[doc(alias = "j___ZN3RBX9TCriticalEjNS_10ConfidenceE")]
// 0xf28ae4 — j___ZN3RBX9TCriticalEjNS_10ConfidenceE
// type: __int64 __fastcall(_DWORD, _DWORD)
pub fn stub_0xf28ae4() -> ! {
    todo!("0xf28ae4 j___ZN3RBX9TCriticalEjNS_10ConfidenceE")
}

#[doc(alias = "XmlElement::XmlElement(RBX::Name const&)")]
#[doc(alias = "j___ZN10XmlElementC2ERKN3RBX4NameE")]
// 0xf296c4 — j___ZN10XmlElementC2ERKN3RBX4NameE
// type: XmlElement *__fastcall(XmlElement *__hidden this, const RBX::Name *)
pub fn stub_0xf296c4() -> ! {
    todo!("0xf296c4 j___ZN10XmlElementC2ERKN3RBX4NameE")
}

#[doc(alias = "XmlAttribute::XmlAttribute<RBX::Name const*>(RBX::Name const&,RBX::Name const*)")]
#[doc(alias = "j___ZN12XmlAttributeC2IPKN3RBX4NameEEERS3_T_")]
// 0xf296d4 — j___ZN12XmlAttributeC2IPKN3RBX4NameEEERS3_T_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf296d4() -> ! {
    todo!("0xf296d4 j___ZN12XmlAttributeC2IPKN3RBX4NameEEERS3_T_")
}

#[doc(alias = "RBX::Allocator<XmlElement>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorI10XmlElementEC2Ev")]
// 0xf29704 — j___ZN3RBX9AllocatorI10XmlElementEC2Ev
// type: int __fastcall(_DWORD)
pub fn stub_0xf29704() -> ! {
    todo!("0xf29704 j___ZN3RBX9AllocatorI10XmlElementEC2Ev")
}

#[doc(alias = "RBX::Allocator<XmlElement>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorI10XmlElementEnwEm")]
// 0xf29714 — j___ZN3RBX9AllocatorI10XmlElementEnwEm
// type: int __fastcall(_DWORD)
pub fn stub_0xf29714() -> ! {
    todo!("0xf29714 j___ZN3RBX9AllocatorI10XmlElementEnwEm")
}

#[doc(alias = "RBX::Allocator<XmlAttribute>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorI12XmlAttributeEC2Ev")]
// 0xf29724 — j___ZN3RBX9AllocatorI12XmlAttributeEC2Ev
// type: int __fastcall(_DWORD)
pub fn stub_0xf29724() -> ! {
    todo!("0xf29724 j___ZN3RBX9AllocatorI12XmlAttributeEC2Ev")
}

#[doc(alias = "RBX::Allocator<XmlAttribute>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorI12XmlAttributeEnwEm")]
// 0xf29734 — j___ZN3RBX9AllocatorI12XmlAttributeEnwEm
// type: int __fastcall(_DWORD)
pub fn stub_0xf29734() -> ! {
    todo!("0xf29734 j___ZN3RBX9AllocatorI12XmlAttributeEnwEm")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject>(RBX::InputObject const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObjectEEERS3_RKT_")]
// 0xf29d14 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObjectEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf29d14() -> ! {
    todo!("0xf29d14 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObjectEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Region3int16>(RBX::Region3int16 const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12Region3int16EEERS3_RKT_")]
// 0xf29d24 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12Region3int16EEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf29d24() -> ! {
    todo!("0xf29d24 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12Region3int16EEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CellID>(RBX::CellID const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6CellIDEEERS3_RKT_")]
// 0xf29d44 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6CellIDEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf29d44() -> ! {
    todo!("0xf29d44 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6CellIDEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Region3int16>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE9singletonEv")]
// 0xf29d74 — j___ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE9singletonEv
// type: int(void)
pub fn stub_0xf29d74() -> ! {
    todo!("0xf29d74 j___ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CellID>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE9singletonEv")]
// 0xf29d84 — j___ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE9singletonEv
// type: int(void)
pub fn stub_0xf29d84() -> ! {
    todo!("0xf29d84 j___ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::RbxRay>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE9singletonEv")]
// 0xf29d94 — j___ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE9singletonEv
// type: 
pub fn stub_0xf29d94() -> ! {
    todo!("0xf29d94 j___ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE9singletonEv")
}

#[doc(alias = "RBX::Region3int16 const& rbx::any_cast<RBX::Region3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf29dc4 — j___ZN3rbx8any_castIRKN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf29dc4() -> ! {
    todo!("0xf29dc4 j___ZN3rbx8any_castIRKN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Axes const& rbx::any_cast<RBX::Axes const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf29df4 — j___ZN3rbx8any_castIRKN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf29df4() -> ! {
    todo!("0xf29df4 j___ZN3rbx8any_castIRKN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::UDim const& rbx::any_cast<RBX::UDim const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX4UDimENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf29e04 — j___ZN3rbx8any_castIRKN3RBX4UDimENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf29e04() -> ! {
    todo!("0xf29e04 j___ZN3rbx8any_castIRKN3RBX4UDimENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CellID const& rbx::any_cast<RBX::CellID const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX6CellIDENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf29e14 — j___ZN3rbx8any_castIRKN3RBX6CellIDENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf29e14() -> ! {
    todo!("0xf29e14 j___ZN3rbx8any_castIRKN3RBX6CellIDENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Region3 const& rbx::any_cast<RBX::Region3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE")]
// 0xf29e24 — j___ZN3rbx8any_castIRKN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf29e24() -> ! {
    todo!("0xf29e24 j___ZN3rbx8any_castIRKN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::ContentId const& rbx::any_cast<RBX::ContentId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf29e34 — j___ZN3rbx8any_castIRKN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf29e34() -> ! {
    todo!("0xf29e34 j___ZN3rbx8any_castIRKN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "long const& rbx::any_cast<long const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf29e94 — j___ZN3rbx8any_castIRKlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf29e94() -> ! {
    todo!("0xf29e94 j___ZN3rbx8any_castIRKlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::at(unsigned long)const")]
#[doc(alias = "j___ZNKSt6vectorIN3RBX10BrickColorESaIS1_EE2atEm")]
// 0xf2a0a4 — j___ZNKSt6vectorIN3RBX10BrickColorESaIS1_EE2atEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2a0a4() -> ! {
    todo!("0xf2a0a4 j___ZNKSt6vectorIN3RBX10BrickColorESaIS1_EE2atEm")
}

#[doc(alias = "RBX::CellID::operator==(RBX::CellID const&)const")]
#[doc(alias = "j___ZNK3RBX6CellIDeqERKS0_")]
// 0xf2a2e4 — j___ZNK3RBX6CellIDeqERKS0_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2a2e4() -> ! {
    todo!("0xf2a2e4 j___ZNK3RBX6CellIDeqERKS0_")
}

#[doc(alias = "RBX::RbxRay::operator==(RBX::RbxRay const&)const")]
#[doc(alias = "j___ZNK3RBX6RbxRayeqERKS0_")]
// 0xf2a2f4 — j___ZNK3RBX6RbxRayeqERKS0_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2a2f4() -> ! {
    todo!("0xf2a2f4 j___ZNK3RBX6RbxRayeqERKS0_")
}

#[doc(alias = "RBX::LibraryService::~LibraryService()")]
#[doc(alias = "j___ZN3RBX14LibraryServiceD2Ev")]
// 0xf2b0e4 — j___ZN3RBX14LibraryServiceD2Ev
// type: void __fastcall(RBX::LibraryService *__hidden this)
pub fn stub_0xf2b0e4() -> ! {
    todo!("0xf2b0e4 j___ZN3RBX14LibraryServiceD2Ev")
}

#[doc(alias = "RBX::RunningAverage<double,double>::sample(double)")]
#[doc(alias = "j___ZN3RBX14RunningAverageIddE6sampleEd")]
// 0xf2b0f4 — j___ZN3RBX14RunningAverageIddE6sampleEd
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf2b0f4() -> ! {
    todo!("0xf2b0f4 j___ZN3RBX14RunningAverageIddE6sampleEd")
}

#[doc(alias = "RBX::InvocationMeter<2>::updateBuckets(bool)")]
#[doc(alias = "j___ZN3RBX15InvocationMeterILi2EE13updateBucketsEb")]
// 0xf2b104 — j___ZN3RBX15InvocationMeterILi2EE13updateBucketsEb
// type: 
pub fn stub_0xf2b104() -> ! {
    todo!("0xf2b104 j___ZN3RBX15InvocationMeterILi2EE13updateBucketsEb")
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample(void)")]
#[doc(alias = "j___ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv")]
// 0xf2b1b4 — j___ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf2b1b4() -> ! {
    todo!("0xf2b1b4 j___ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv")
}

#[doc(alias = "RBX::Security::Impersonator::Impersonator(RBX::Security::Identities)")]
#[doc(alias = "j___ZN3RBX8Security12ImpersonatorC2ENS0_10IdentitiesE")]
// 0xf2b3c4 — j___ZN3RBX8Security12ImpersonatorC2ENS0_10IdentitiesE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2b3c4() -> ! {
    todo!("0xf2b3c4 j___ZN3RBX8Security12ImpersonatorC2ENS0_10IdentitiesE")
}

#[doc(alias = "RBX::Security::Context::current(void)")]
#[doc(alias = "j___ZN3RBX8Security7Context7currentEv")]
// 0xf2b3d4 — j___ZN3RBX8Security7Context7currentEv
// type: _DWORD __fastcall(RBX::Security::Context *__hidden this)
pub fn stub_0xf2b3d4() -> ! {
    todo!("0xf2b3d4 j___ZN3RBX8Security7Context7currentEv")
}

#[doc(alias = "RBX::ContentId::ContentId(char const*)")]
#[doc(alias = "j___ZN3RBX9ContentIdC2EPKc")]
// 0xf2b3e4 — j___ZN3RBX9ContentIdC2EPKc
// type: _DWORD __fastcall(RBX::ContentId *__hidden this, const char *)
pub fn stub_0xf2b3e4() -> ! {
    todo!("0xf2b3e4 j___ZN3RBX9ContentIdC2EPKc")
}

#[doc(alias = "RBX::ContentId::ContentId(std::string const&)")]
#[doc(alias = "j___ZN3RBX9ContentIdC2ERKSs")]
// 0xf2b3f4 — j___ZN3RBX9ContentIdC2ERKSs
// type: _DWORD __fastcall(RBX::ContentId *__hidden this, const std::string *)
pub fn stub_0xf2b3f4() -> ! {
    todo!("0xf2b3f4 j___ZN3RBX9ContentIdC2ERKSs")
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::Iterator(RobloxExtraSpace*)")]
#[doc(alias = "j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorC2EPS2_")]
// 0xf2b4b4 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorC2EPS2_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2b4b4() -> ! {
    todo!("0xf2b4b4 j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorC2EPS2_")
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator++(void)")]
#[doc(alias = "j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorppEv")]
// 0xf2b4c4 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorppEv
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2b4c4() -> ! {
    todo!("0xf2b4c4 j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorppEv")
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator->(void)")]
#[doc(alias = "j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorptEv")]
// 0xf2b4d4 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorptEv
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2b4d4() -> ! {
    todo!("0xf2b4d4 j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorptEv")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<std::string>(std::string const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSISsEERS3_RKT_")]
// 0xf2b544 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSISsEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2b544() -> ! {
    todo!("0xf2b544 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSISsEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<bool>(bool const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIbEERS3_RKT_")]
// 0xf2b554 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIbEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2b554() -> ! {
    todo!("0xf2b554 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIbEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<double>(double const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIdEERS3_RKT_")]
// 0xf2b564 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIdEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2b564() -> ! {
    todo!("0xf2b564 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIdEERS3_RKT_")
}

#[doc(alias = "rbx::signals::signal_with_args<0,void ()(void)>::operator()(void)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv")]
// 0xf2b594 — j___ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv
// type: int __fastcall(int)
pub fn stub_0xf2b594() -> ! {
    todo!("0xf2b594 j___ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv")]
// 0xf2b5f4 — j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf2b5f4() -> ! {
    todo!("0xf2b5f4 j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv")]
// 0xf2b6f4 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv
// type: int(void)
pub fn stub_0xf2b6f4() -> ! {
    todo!("0xf2b6f4 j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv")]
// 0xf2b704 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv
// type: 
pub fn stub_0xf2b704() -> ! {
    todo!("0xf2b704 j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::insert(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6insertEPNS7_4slotE")]
// 0xf2b714 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6insertEPNS7_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf2b714() -> ! {
    todo!("0xf2b714 j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6insertEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::remove(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6removeEPNS7_4slotE")]
// 0xf2b724 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6removeEPNS7_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf2b724() -> ! {
    todo!("0xf2b724 j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6removeEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFviEE24safe_static_do_get_mutexEv")]
// 0xf2b7c4 — j___ZN3rbx7signals6signalIFviEE24safe_static_do_get_mutexEv
// type: int(void)
pub fn stub_0xf2b7c4() -> ! {
    todo!("0xf2b7c4 j___ZN3rbx7signals6signalIFviEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvvEE13disconnectAllEv")]
// 0xf2b7d4 — j___ZN3rbx7signals6signalIFvvEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf2b7d4() -> ! {
    todo!("0xf2b7d4 j___ZN3rbx7signals6signalIFvvEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvvEE24safe_static_do_get_mutexEv")]
// 0xf2b7e4 — j___ZN3rbx7signals6signalIFvvEE24safe_static_do_get_mutexEv
// type: int(void)
pub fn stub_0xf2b7e4() -> ! {
    todo!("0xf2b7e4 j___ZN3rbx7signals6signalIFvvEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "std::string const& rbx::any_cast<std::string const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKSsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf2b7f4 — j___ZN3rbx8any_castIRKSsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf2b7f4() -> ! {
    todo!("0xf2b7f4 j___ZN3rbx8any_castIRKSsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "bool const& rbx::any_cast<bool const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf2b804 — j___ZN3rbx8any_castIRKbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf2b804() -> ! {
    todo!("0xf2b804 j___ZN3rbx8any_castIRKbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "int const& rbx::any_cast<int const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf2b814 — j___ZN3rbx8any_castIRKiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf2b814() -> ! {
    todo!("0xf2b814 j___ZN3rbx8any_castIRKiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "boost::scoped_ptr<RBX::LibraryService>::~scoped_ptr()")]
#[doc(alias = "j___ZN5boost10scoped_ptrIN3RBX14LibraryServiceEED2Ev")]
// 0xf2b874 — j___ZN5boost10scoped_ptrIN3RBX14LibraryServiceEED2Ev
// type: 
pub fn stub_0xf2b874() -> ! {
    todo!("0xf2b874 j___ZN5boost10scoped_ptrIN3RBX14LibraryServiceEED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::operator=(rbx_core::SharedPtr<RBX::RunService> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10RunServiceEEaSERKS3_")]
// 0xf2b904 — j___ZN5boost10shared_ptrIN3RBX10RunServiceEEaSERKS3_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2b904() -> ! {
    todo!("0xf2b904 j___ZN5boost10shared_ptrIN3RBX10RunServiceEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::GcJob>(RBX::GcJob *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_5GcJobEEEPT_")]
// 0xf2b9c4 — j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_5GcJobEEEPT_
// type: int __fastcall(int, void *, int, int, int, int)
pub fn stub_0xf2b9c4() -> ! {
    todo!("0xf2b9c4 j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_5GcJobEEEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::operator=(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEaSERKS4_")]
// 0xf2b9d4 — j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEaSERKS4_
// type: 
pub fn stub_0xf2b9d4() -> ! {
    todo!("0xf2b9d4 j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEaSERKS4_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot>::operator=(rbx::signals::signal<void ()(RBX::RunTransition)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotEEaSEPS8_")]
// 0xf2bac4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotEEaSEPS8_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2bac4() -> ! {
    todo!("0xf2bac4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotEEaSEPS8_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSEPSA_")]
// 0xf2bb14 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSEPSA_
// type: 
pub fn stub_0xf2bb14() -> ! {
    todo!("0xf2bb14 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSEPSA_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSERKSB_")]
// 0xf2bb24 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSERKSB_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2bb24() -> ! {
    todo!("0xf2bb24 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSERKSB_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSERKS7_")]
// 0xf2bb54 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSERKS7_
// type: int(void)
pub fn stub_0xf2bb54() -> ! {
    todo!("0xf2bb54 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSERKS7_")]
// 0xf2bb64 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSERKS7_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2bb64() -> ! {
    todo!("0xf2bb64 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSERKS7_")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::reset(RBX::Security::Context*)")]
#[doc(alias = "j___ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE5resetEPS3_")]
// 0xf2bb94 — j___ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE5resetEPS3_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2bb94() -> ! {
    todo!("0xf2bb94 j___ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE5resetEPS3_")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::~thread_specific_ptr()")]
#[doc(alias = "j___ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEED2Ev")]
// 0xf2bba4 — j___ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEED2Ev
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf2bba4() -> ! {
    todo!("0xf2bba4 j___ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEED2Ev")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GcJob>(RBX::GcJob *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX5GcJobEEEPT_")]
// 0xf2bdc4 — j___ZN5boost6detail12shared_countC2IN3RBX5GcJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2bdc4() -> ! {
    todo!("0xf2bdc4 j___ZN5boost6detail12shared_countC2IN3RBX5GcJobEEEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>(boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS8_EEEET_T0_")]
// 0xf2be84 — j___ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS8_EEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2be84() -> ! {
    todo!("0xf2be84 j___ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS8_EEEET_T0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSB_RKSD_RKSaINS1_8ptr_nodeIS8_EEE")]
// 0xf2c2a4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSB_RKSD_RKSaINS1_8ptr_nodeIS8_EEE
// type: 
pub fn stub_0xf2c2a4() -> ! {
    todo!("0xf2c2a4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSB_RKSD_RKSaINS1_8ptr_nodeIS8_EEE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::GcJob>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const*,RBX::GcJob *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_5GcJobEEEvPKNS_10shared_ptrIT_EEPT0_")]
// 0xf2c434 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_5GcJobEEEvPKNS_10shared_ptrIT_EEPT0_
// type: 
pub fn stub_0xf2c434() -> ! {
    todo!("0xf2c434 j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_5GcJobEEEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::_M_clear(void)")]
#[doc(alias = "j___ZNSt10_List_baseIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE8_M_clearEv")]
// 0xf2c684 — j___ZNSt10_List_baseIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE8_M_clearEv
// type: 
pub fn stub_0xf2c684() -> ! {
    todo!("0xf2c684 j___ZNSt10_List_baseIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE8_M_clearEv")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")]
// 0xf2ca74 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
// type: 
pub fn stub_0xf2ca74() -> ! {
    todo!("0xf2ca74 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf2ca84 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
pub fn stub_0xf2ca84() -> ! {
    todo!("0xf2ca84 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<bool>(char const*,bool const&)")]
#[doc(alias = "j___ZN3RBX5Stats4Item20createBoundChildItemIbEEPS1_PKcRKT_")]
// 0xf2cc44 — j___ZN3RBX5Stats4Item20createBoundChildItemIbEEPS1_PKcRKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf2cc44() -> ! {
    todo!("0xf2cc44 j___ZN3RBX5Stats4Item20createBoundChildItemIbEEPS1_PKcRKT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ActivityMeter<2>>::shared_ptr<RBX::ActivityMeter<2>>(RBX::ActivityMeter<2> *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX13ActivityMeterILi2EEEEC2IS3_EEPT_")]
// 0xf2cc64 — j___ZN5boost10shared_ptrIN3RBX13ActivityMeterILi2EEEEC2IS3_EEPT_
// type: 
pub fn stub_0xf2cc64() -> ! {
    todo!("0xf2cc64 j___ZN5boost10shared_ptrIN3RBX13ActivityMeterILi2EEEEC2IS3_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ActivityMeter<2>>::operator=(rbx_core::SharedPtr<RBX::ActivityMeter<2>> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX13ActivityMeterILi2EEEEaSERKS4_")]
// 0xf2cc74 — j___ZN5boost10shared_ptrIN3RBX13ActivityMeterILi2EEEEaSERKS4_
// type: 
pub fn stub_0xf2cc74() -> ! {
    todo!("0xf2cc74 j___ZN5boost10shared_ptrIN3RBX13ActivityMeterILi2EEEEaSERKS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::InvocationMeter<2>>::shared_ptr<RBX::InvocationMeter<2>>(RBX::InvocationMeter<2> *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEC2IS3_EEPT_")]
// 0xf2cc84 — j___ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEC2IS3_EEPT_
// type: 
pub fn stub_0xf2cc84() -> ! {
    todo!("0xf2cc84 j___ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEC2IS3_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::InvocationMeter<2>>::operator=(rbx_core::SharedPtr<RBX::InvocationMeter<2>> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEaSERKS4_")]
// 0xf2cc94 — j___ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEaSERKS4_
// type: 
pub fn stub_0xf2cc94() -> ! {
    todo!("0xf2cc94 j___ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEaSERKS4_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ActivityMeter<2>>(RBX::ActivityMeter<2> *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX13ActivityMeterILi2EEEEEPT_")]
// 0xf2ccc4 — j___ZN5boost6detail12shared_countC2IN3RBX13ActivityMeterILi2EEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2ccc4() -> ! {
    todo!("0xf2ccc4 j___ZN5boost6detail12shared_countC2IN3RBX13ActivityMeterILi2EEEEEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::InvocationMeter<2>>(RBX::InvocationMeter<2> *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX15InvocationMeterILi2EEEEEPT_")]
// 0xf2ccd4 — j___ZN5boost6detail12shared_countC2IN3RBX15InvocationMeterILi2EEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2ccd4() -> ! {
    todo!("0xf2ccd4 j___ZN5boost6detail12shared_countC2IN3RBX15InvocationMeterILi2EEEEEPT_")
}

#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::reset(RBX::AdvRunDragger*)")]
#[doc(alias = "j___ZNSt8auto_ptrIN3RBX13AdvRunDraggerEE5resetEPS1_")]
// 0xf2d0c4 — j___ZNSt8auto_ptrIN3RBX13AdvRunDraggerEE5resetEPS1_
// type: 
pub fn stub_0xf2d0c4() -> ! {
    todo!("0xf2d0c4 j___ZNSt8auto_ptrIN3RBX13AdvRunDraggerEE5resetEPS1_")
}

#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::~auto_ptr()")]
#[doc(alias = "j___ZNSt8auto_ptrIN3RBX13AdvRunDraggerEED2Ev")]
// 0xf2d0d4 — j___ZNSt8auto_ptrIN3RBX13AdvRunDraggerEED2Ev
// type: 
pub fn stub_0xf2d0d4() -> ! {
    todo!("0xf2d0d4 j___ZNSt8auto_ptrIN3RBX13AdvRunDraggerEED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvMoveToolBase> RBX::shared_from<RBX::AdvMoveToolBase>(RBX::AdvMoveToolBase*)")]
#[doc(alias = "j___ZN3RBX11shared_fromINS_15AdvMoveToolBaseEEEN5boost10shared_ptrIT_EEPS4_")]
// 0xf2d114 — j___ZN3RBX11shared_fromINS_15AdvMoveToolBaseEEEN5boost10shared_ptrIT_EEPS4_
// type: 
pub fn stub_0xf2d114() -> ! {
    todo!("0xf2d114 j___ZN3RBX11shared_fromINS_15AdvMoveToolBaseEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::DrawAdorn::resizeColor(void)")]
#[doc(alias = "j___ZN3RBX9DrawAdorn11resizeColorEv")]
// 0xf2d124 — j___ZN3RBX9DrawAdorn11resizeColorEv
// type: _DWORD __fastcall(RBX::DrawAdorn *__hidden this)
pub fn stub_0xf2d124() -> ! {
    todo!("0xf2d124 j___ZN3RBX9DrawAdorn11resizeColorEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AxisToolBase> RBX::shared_from<RBX::AxisToolBase>(RBX::AxisToolBase*)")]
#[doc(alias = "j___ZN3RBX11shared_fromINS_12AxisToolBaseEEEN5boost10shared_ptrIT_EEPS4_")]
// 0xf2d184 — j___ZN3RBX11shared_fromINS_12AxisToolBaseEEEN5boost10shared_ptrIT_EEPS4_
// type: 
pub fn stub_0xf2d184() -> ! {
    todo!("0xf2d184 j___ZN3RBX11shared_fromINS_12AxisToolBaseEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "std::auto_ptr<RBX::MegaDragger>::reset(RBX::MegaDragger*)")]
#[doc(alias = "j___ZNSt8auto_ptrIN3RBX11MegaDraggerEE5resetEPS1_")]
// 0xf2d194 — j___ZNSt8auto_ptrIN3RBX11MegaDraggerEE5resetEPS1_
// type: 
pub fn stub_0xf2d194() -> ! {
    todo!("0xf2d194 j___ZNSt8auto_ptrIN3RBX11MegaDraggerEE5resetEPS1_")
}

#[doc(alias = "RBX::Extents::negativeMaxExtents(void)")]
#[doc(alias = "j___ZN3RBX7Extents18negativeMaxExtentsEv")]
// 0xf2d214 — j___ZN3RBX7Extents18negativeMaxExtentsEv
// type: _DWORD __fastcall(RBX::Extents *__hidden this)
pub fn stub_0xf2d214() -> ! {
    todo!("0xf2d214 j___ZN3RBX7Extents18negativeMaxExtentsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::table(unsigned long,boost::hash<RBX::Primitive const*> const&,std::equal_to<RBX::Primitive const*> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSA_RKSC_RKSaINS1_8ptr_nodeIS7_EEE")]
// 0xf2d224 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSA_RKSC_RKSaINS1_8ptr_nodeIS7_EEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf2d224() -> ! {
    todo!("0xf2d224 j___ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSA_RKSC_RKSaINS1_8ptr_nodeIS7_EEE")
}

#[doc(alias = "RBX::POLY::Edge::getVertex(RBX::POLY::Face const*,unsigned long)const")]
#[doc(alias = "j___ZNK3RBX4POLY4Edge9getVertexEPKNS0_4FaceEm")]
// 0xf2d234 — j___ZNK3RBX4POLY4Edge9getVertexEPKNS0_4FaceEm
// type: _DWORD __fastcall(RBX::POLY::Edge *__hidden this, const RBX::POLY::Face *, unsigned int)
pub fn stub_0xf2d234() -> ! {
    todo!("0xf2d234 j___ZNK3RBX4POLY4Edge9getVertexEPKNS0_4FaceEm")
}

#[doc(alias = "std::_Vector_base<RBX::Extents,std::allocator<RBX::Extents>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX7ExtentsESaIS1_EE11_M_allocateEm")]
// 0xf2d244 — j___ZNSt12_Vector_baseIN3RBX7ExtentsESaIS1_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2d244() -> ! {
    todo!("0xf2d244 j___ZNSt12_Vector_baseIN3RBX7ExtentsESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Extents * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Extents *,RBX::Extents *>(RBX::Extents *,RBX::Extents *,RBX::Extents *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7ExtentsES5_EET0_T_S7_S6_")]
// 0xf2d254 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7ExtentsES5_EET0_T_S7_S6_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2d254() -> ! {
    todo!("0xf2d254 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7ExtentsES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Extents*,std::vector<RBX::Extents,std::allocator<RBX::Extents>>>,RBX::Extents const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// 0xf2d264 — j___ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: 
pub fn stub_0xf2d264() -> ! {
    todo!("0xf2d264 j___ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::reserve(unsigned long)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX7ExtentsESaIS1_EE7reserveEm")]
// 0xf2d274 — j___ZNSt6vectorIN3RBX7ExtentsESaIS1_EE7reserveEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2d274() -> ! {
    todo!("0xf2d274 j___ZNSt6vectorIN3RBX7ExtentsESaIS1_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::push_back(RBX::Extents const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX7ExtentsESaIS1_EE9push_backERKS1_")]
// 0xf2d284 — j___ZNSt6vectorIN3RBX7ExtentsESaIS1_EE9push_backERKS1_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2d284() -> ! {
    todo!("0xf2d284 j___ZNSt6vectorIN3RBX7ExtentsESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Joint>,std::allocator<rbx_core::SharedPtr<RBX::Joint>>>::~vector()")]
#[doc(alias = "j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5JointEEESaIS4_EED2Ev")]
// 0xf2d4b4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5JointEEESaIS4_EED2Ev
// type: 
pub fn stub_0xf2d4b4() -> ! {
    todo!("0xf2d4b4 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX5JointEEESaIS4_EED2Ev")
}

#[doc(alias = "std::auto_ptr<RBX::RunDragger>::reset(RBX::RunDragger*)")]
#[doc(alias = "j___ZNSt8auto_ptrIN3RBX10RunDraggerEE5resetEPS1_")]
// 0xf2d4e4 — j___ZNSt8auto_ptrIN3RBX10RunDraggerEE5resetEPS1_
// type: 
pub fn stub_0xf2d4e4() -> ! {
    todo!("0xf2d4e4 j___ZNSt8auto_ptrIN3RBX10RunDraggerEE5resetEPS1_")
}

#[doc(alias = "std::auto_ptr<RBX::RunDragger>::~auto_ptr()")]
#[doc(alias = "j___ZNSt8auto_ptrIN3RBX10RunDraggerEED2Ev")]
// 0xf2d4f4 — j___ZNSt8auto_ptrIN3RBX10RunDraggerEED2Ev
// type: 
pub fn stub_0xf2d4f4() -> ! {
    todo!("0xf2d4f4 j___ZNSt8auto_ptrIN3RBX10RunDraggerEED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MoveResizeJoinTool> RBX::shared_from<RBX::MoveResizeJoinTool>(RBX::MoveResizeJoinTool*)")]
#[doc(alias = "j___ZN3RBX11shared_fromINS_18MoveResizeJoinToolEEEN5boost10shared_ptrIT_EEPS4_")]
// 0xf2d554 — j___ZN3RBX11shared_fromINS_18MoveResizeJoinToolEEEN5boost10shared_ptrIT_EEPS4_
// type: 
pub fn stub_0xf2d554() -> ! {
    todo!("0xf2d554 j___ZN3RBX11shared_fromINS_18MoveResizeJoinToolEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NewNullTool> RBX::shared_from<RBX::NewNullTool>(RBX::NewNullTool*)")]
#[doc(alias = "j___ZN3RBX11shared_fromINS_11NewNullToolEEEN5boost10shared_ptrIT_EEPS4_")]
// 0xf2d564 — j___ZN3RBX11shared_fromINS_11NewNullToolEEEN5boost10shared_ptrIT_EEPS4_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2d564() -> ! {
    todo!("0xf2d564 j___ZN3RBX11shared_fromINS_11NewNullToolEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NullTool> RBX::shared_from<RBX::NullTool>(RBX::NullTool*)")]
#[doc(alias = "j___ZN3RBX11shared_fromINS_8NullToolEEEN5boost10shared_ptrIT_EEPS4_")]
// 0xf2d574 — j___ZN3RBX11shared_fromINS_8NullToolEEEN5boost10shared_ptrIT_EEPS4_
// type: 
pub fn stub_0xf2d574() -> ! {
    todo!("0xf2d574 j___ZN3RBX11shared_fromINS_8NullToolEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand>::shared_ptr<RBX::BoxSelectCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX16BoxSelectCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")]
// 0xf2d664 — j___ZN5boost10shared_ptrIN3RBX16BoxSelectCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: 
pub fn stub_0xf2d664() -> ! {
    todo!("0xf2d664 j___ZN5boost10shared_ptrIN3RBX16BoxSelectCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX16BoxSelectCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")]
// 0xf2d684 — j___ZN5boost6detail12shared_countC2IPN3RBX16BoxSelectCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2d684() -> ! {
    todo!("0xf2d684 j___ZN5boost6detail12shared_countC2IPN3RBX16BoxSelectCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::BoxSelectCommand,RBX::BoxSelectCommand>(rbx_core::SharedPtr<RBX::BoxSelectCommand> const*,RBX::BoxSelectCommand *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16BoxSelectCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0xf2d6b4 — j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16BoxSelectCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: 
pub fn stub_0xf2d6b4() -> ! {
    todo!("0xf2d6b4 j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16BoxSelectCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "std::_Vector_base<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX6Action10ActionTypeESaIS2_EE11_M_allocateEm")]
// 0xf2d7b4 — j___ZNSt12_Vector_baseIN3RBX6Action10ActionTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2d7b4() -> ! {
    todo!("0xf2d7b4 j___ZNSt12_Vector_baseIN3RBX6Action10ActionTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Action::ActionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Action::ActionType *,RBX::Action::ActionType *>(RBX::Action::ActionType *,RBX::Action::ActionType *,RBX::Action::ActionType *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Action10ActionTypeES6_EET0_T_S8_S7_")]
// 0xf2d7c4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Action10ActionTypeES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2d7c4() -> ! {
    todo!("0xf2d7c4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Action10ActionTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Action::ActionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_6Action10ActionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf2d7d4 — j___ZNSt3mapIPKN3RBX4NameENS0_6Action10ActionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2d7d4() -> ! {
    todo!("0xf2d7d4 j___ZNSt3mapIPKN3RBX4NameENS0_6Action10ActionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}
