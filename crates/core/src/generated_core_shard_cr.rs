//! core shard CR — 100 core stubs EA-sorted, next uncovered after CQ 0x6e73fc (strict RBX|boost|std|rbx earliest gap).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::SimBody::SimBody(RBX::Body *)")]
// 0x6f0bb8 — __ZN3RBX7SimBodyC1EPNS_4BodyE
pub fn stub_6f0bb8() -> ! {
    todo!("0x6f0bb8 __ZN3RBX7SimBodyC1EPNS_4BodyE")
}

#[doc(alias = "RBX::SimBody::SimBody(RBX::Body *)")]
// 0x6f0bbc — __ZN3RBX7SimBodyC2EPNS_4BodyE
pub fn stub_6f0bbc() -> ! {
    todo!("0x6f0bbc __ZN3RBX7SimBodyC2EPNS_4BodyE")
}

#[doc(alias = "RBX::SimBody::~SimBody()")]
// 0x6f0d18 — __ZN3RBX7SimBodyD1Ev
pub fn stub_6f0d18() -> ! {
    todo!("0x6f0d18 __ZN3RBX7SimBodyD1Ev")
}

#[doc(alias = "RBX::SimBody::~SimBody()")]
// 0x6f0d1c — __ZN3RBX7SimBodyD2Ev
pub fn stub_6f0d1c() -> ! {
    todo!("0x6f0d1c __ZN3RBX7SimBodyD2Ev")
}

#[doc(alias = "RBX::SimBody::getOwnerPV(void)")]
// 0x6f0eac — __ZN3RBX7SimBody10getOwnerPVEv
pub fn stub_6f0eac() -> ! {
    todo!("0x6f0eac __ZN3RBX7SimBody10getOwnerPVEv")
}

#[doc(alias = "RBX::SimBody::update(void)")]
// 0x6f10a8 — __ZN3RBX7SimBody6updateEv
pub fn stub_6f10a8() -> ! {
    todo!("0x6f10a8 __ZN3RBX7SimBody6updateEv")
}

#[doc(alias = "RBX::SimBody::updateAngMomentum(void)")]
// 0x6f145c — __ZN3RBX7SimBody17updateAngMomentumEv
pub fn stub_6f145c() -> ! {
    todo!("0x6f145c __ZN3RBX7SimBody17updateAngMomentumEv")
}

#[doc(alias = "RBX::SimBody::clearVelocity(void)")]
// 0x6f14ec — __ZN3RBX7SimBody13clearVelocityEv
pub fn stub_6f14ec() -> ! {
    todo!("0x6f14ec __ZN3RBX7SimBody13clearVelocityEv")
}

#[doc(alias = "RBX::SimBody::step(void)")]
// 0x6f1548 — __ZN3RBX7SimBody4stepEv
pub fn stub_6f1548() -> ! {
    todo!("0x6f1548 __ZN3RBX7SimBody4stepEv")
}

#[doc(alias = "RBX::SimBody::stepVelocity(void)")]
// 0x6f1984 — __ZN3RBX7SimBody12stepVelocityEv
pub fn stub_6f1984() -> ! {
    todo!("0x6f1984 __ZN3RBX7SimBody12stepVelocityEv")
}

#[doc(alias = "RBX::SimBody::stepPosition(void)")]
// 0x6f1ea0 — __ZN3RBX7SimBody12stepPositionEv
pub fn stub_6f1ea0() -> ! {
    todo!("0x6f1ea0 __ZN3RBX7SimBody12stepPositionEv")
}

#[doc(alias = "RBX::SimBody::stepFreeFall(void)")]
// 0x6f2224 — __ZN3RBX7SimBody12stepFreeFallEv
pub fn stub_6f2224() -> ! {
    todo!("0x6f2224 __ZN3RBX7SimBody12stepFreeFallEv")
}

#[doc(alias = "RBX::Allocator<RBX::SimBody>::Allocator(void)")]
// 0x6f2618 — __ZN3RBX9AllocatorINS_7SimBodyEEC2Ev
pub fn stub_6f2618() -> ! {
    todo!("0x6f2618 __ZN3RBX9AllocatorINS_7SimBodyEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::SimBody>::releaseMemory(void)")]
// 0x6f267c — __ZN3RBX9AllocatorINS_7SimBodyEE13releaseMemoryEv
pub fn stub_6f267c() -> ! {
    todo!("0x6f267c __ZN3RBX9AllocatorINS_7SimBodyEE13releaseMemoryEv")
}

#[doc(alias = "RBX::StringConverter<RBX::NormalId>::convertToValue(std::string const&,RBX::NormalId&)")]
// 0x6f2804 — __ZN3RBX15StringConverterINS_8NormalIdEE14convertToValueERKSsRS1_
pub fn stub_6f2804() -> ! {
    todo!("0x6f2804 __ZN3RBX15StringConverterINS_8NormalIdEE14convertToValueERKSsRS1_")
}

#[doc(alias = "XmlNameValuePair::setValue(std::string)")]
// 0x6f7b7c — __ZN16XmlNameValuePair8setValueESs
pub fn stub_6f7b7c() -> ! {
    todo!("0x6f7b7c __ZN16XmlNameValuePair8setValueESs")
}

#[doc(alias = "double rbx::any_cast<double,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6f7c44 — __ZN3rbx8any_castIdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6f7c44() -> ! {
    todo!("0x6f7c44 __ZN3rbx8any_castIdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "float rbx::any_cast<float,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6f7d30 — __ZN3rbx8any_castIfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6f7d30() -> ! {
    todo!("0x6f7d30 __ZN3rbx8any_castIfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "bool rbx::any_cast<bool,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6f7e18 — __ZN3rbx8any_castIbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6f7e18() -> ! {
    todo!("0x6f7e18 __ZN3rbx8any_castIbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "int rbx::any_cast<int,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6f8144 — __ZN3rbx8any_castIiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6f8144() -> ! {
    todo!("0x6f8144 __ZN3rbx8any_castIiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "long rbx::any_cast<long,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6f95f8 — __ZN3rbx8any_castIlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6f95f8() -> ! {
    todo!("0x6f95f8 __ZN3rbx8any_castIlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::string * rbx::any_cast<std::string,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x6f9a30 — __ZN3rbx8any_castISsN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_6f9a30() -> ! {
    todo!("0x6f9a30 __ZN3rbx8any_castISsN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "XmlNameValuePair::setValue(RBX::ContentId)")]
// 0x6fae8c — __ZN16XmlNameValuePair8setValueEN3RBX9ContentIdE
pub fn stub_6fae8c() -> ! {
    todo!("0x6fae8c __ZN16XmlNameValuePair8setValueEN3RBX9ContentIdE")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::NormalId>::construct_func(char const*,char *)")]
// 0x6fb090 — __ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE14construct_funcEPKcPc
pub fn stub_6fb090() -> ! {
    todo!("0x6fb090 __ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE14construct_funcEPKcPc")
}

#[doc(alias = "RBX::SystemAddress * rbx::any_cast<RBX::SystemAddress,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x6fb3ec — __ZN3rbx8any_castIN3RBX13SystemAddressENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_6fb3ec() -> ! {
    todo!("0x6fb3ec __ZN3rbx8any_castIN3RBX13SystemAddressENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::BrickColor * rbx::any_cast<RBX::BrickColor,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x6fb4f4 — __ZN3rbx8any_castIN3RBX10BrickColorENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_6fb4f4() -> ! {
    todo!("0x6fb4f4 __ZN3rbx8any_castIN3RBX10BrickColorENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::BrickColor & rbx::any_cast<RBX::BrickColor &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fb54c — __ZN3rbx8any_castIRN3RBX10BrickColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6fb54c() -> ! {
    todo!("0x6fb54c __ZN3rbx8any_castIRN3RBX10BrickColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::RbxRay * rbx::any_cast<RBX::RbxRay,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x6fb6ec — __ZN3rbx8any_castIN3RBX6RbxRayENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_6fb6ec() -> ! {
    todo!("0x6fb6ec __ZN3rbx8any_castIN3RBX6RbxRayENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::RbxRay>(RBX::RbxRay const&)")]
// 0x6fb744 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6RbxRayEEERS3_RKT_
pub fn stub_6fb744() -> ! {
    todo!("0x6fb744 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6RbxRayEEERS3_RKT_")
}

#[doc(alias = "RBX::RbxRay & rbx::any_cast<RBX::RbxRay &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fb7c8 — __ZN3rbx8any_castIRN3RBX6RbxRayENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6fb7c8() -> ! {
    todo!("0x6fb7c8 __ZN3rbx8any_castIRN3RBX6RbxRayENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Axes * rbx::any_cast<RBX::Axes,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x6fbb60 — __ZN3rbx8any_castIN3RBX4AxesENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_6fbb60() -> ! {
    todo!("0x6fbb60 __ZN3rbx8any_castIN3RBX4AxesENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Axes>(RBX::Axes const&)")]
// 0x6fbbb8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4AxesEEERS3_RKT_
pub fn stub_6fbbb8() -> ! {
    todo!("0x6fbbb8 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4AxesEEERS3_RKT_")
}

#[doc(alias = "RBX::Axes & rbx::any_cast<RBX::Axes &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fbc08 — __ZN3rbx8any_castIRN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6fbc08() -> ! {
    todo!("0x6fbc08 __ZN3rbx8any_castIRN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Axes>::singleton(void)")]
// 0x6fbcf8 — __ZN3rbx14implementation12typed_holderIN3RBX4AxesEE9singletonEv
pub fn stub_6fbcf8() -> ! {
    todo!("0x6fbcf8 __ZN3rbx14implementation12typed_holderIN3RBX4AxesEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Axes>::destruct_func(char *)")]
// 0x6fbd64 — __ZN3rbx14implementation12typed_holderIN3RBX4AxesEE13destruct_funcEPc
pub fn stub_6fbd64() -> ! {
    todo!("0x6fbd64 __ZN3rbx14implementation12typed_holderIN3RBX4AxesEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Faces * rbx::any_cast<RBX::Faces,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x6fbe18 — __ZN3rbx8any_castIN3RBX5FacesENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_6fbe18() -> ! {
    todo!("0x6fbe18 __ZN3rbx8any_castIN3RBX5FacesENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Faces & rbx::any_cast<RBX::Faces &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fbe70 — __ZN3rbx8any_castIRN3RBX5FacesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6fbe70() -> ! {
    todo!("0x6fbe70 __ZN3rbx8any_castIRN3RBX5FacesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::UDim2 * rbx::any_cast<RBX::UDim2,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x6fc010 — __ZN3rbx8any_castIN3RBX5UDim2ENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_6fc010() -> ! {
    todo!("0x6fc010 __ZN3rbx8any_castIN3RBX5UDim2ENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::UDim2>(RBX::UDim2 const&)")]
// 0x6fc068 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5UDim2EEERS3_RKT_
pub fn stub_6fc068() -> ! {
    todo!("0x6fc068 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5UDim2EEERS3_RKT_")
}

#[doc(alias = "RBX::UDim2 & rbx::any_cast<RBX::UDim2 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fc0c8 — __ZN3rbx8any_castIRN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6fc0c8() -> ! {
    todo!("0x6fc0c8 __ZN3rbx8any_castIRN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UDim2>::singleton(void)")]
// 0x6fc1b8 — __ZN3rbx14implementation12typed_holderIN3RBX5UDim2EE9singletonEv
pub fn stub_6fc1b8() -> ! {
    todo!("0x6fc1b8 __ZN3rbx14implementation12typed_holderIN3RBX5UDim2EE9singletonEv")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::UDim>(RBX::UDim const&)")]
// 0x6fc384 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4UDimEEERS3_RKT_
pub fn stub_6fc384() -> ! {
    todo!("0x6fc384 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4UDimEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UDim>::singleton(void)")]
// 0x6fc3dc — __ZN3rbx14implementation12typed_holderIN3RBX4UDimEE9singletonEv
pub fn stub_6fc3dc() -> ! {
    todo!("0x6fc3dc __ZN3rbx14implementation12typed_holderIN3RBX4UDimEE9singletonEv")
}

#[doc(alias = "RBX::ContentId * rbx::any_cast<RBX::ContentId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x6fcec4 — __ZN3rbx8any_castIN3RBX9ContentIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_6fcec4() -> ! {
    todo!("0x6fcec4 __ZN3rbx8any_castIN3RBX9ContentIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::ContentId & rbx::any_cast<RBX::ContentId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fcf1c — __ZN3rbx8any_castIRN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6fcf1c() -> ! {
    todo!("0x6fcf1c __ZN3rbx8any_castIRN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Region3int16 * rbx::any_cast<RBX::Region3int16,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x6fd504 — __ZN3rbx8any_castIN3RBX12Region3int16ENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_6fd504() -> ! {
    todo!("0x6fd504 __ZN3rbx8any_castIN3RBX12Region3int16ENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Region3int16 & rbx::any_cast<RBX::Region3int16 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fd55c — __ZN3rbx8any_castIRN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6fd55c() -> ! {
    todo!("0x6fd55c __ZN3rbx8any_castIRN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Region3 * rbx::any_cast<RBX::Region3,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x6fd64c — __ZN3rbx8any_castIN3RBX7Region3ES2_EEPT_PNS_13placement_anyIT0_EE
pub fn stub_6fd64c() -> ! {
    todo!("0x6fd64c __ZN3rbx8any_castIN3RBX7Region3ES2_EEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Region3 & rbx::any_cast<RBX::Region3 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fd6a4 — __ZN3rbx8any_castIRN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE
pub fn stub_6fd6a4() -> ! {
    todo!("0x6fd6a4 __ZN3rbx8any_castIRN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Region3>::construct_func(char const*,char *)")]
// 0x6fd794 — __ZN3rbx14implementation12typed_holderIN3RBX7Region3EE14construct_funcEPKcPc
pub fn stub_6fd794() -> ! {
    todo!("0x6fd794 __ZN3rbx14implementation12typed_holderIN3RBX7Region3EE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Region3>::destruct_func(char *)")]
// 0x6fd7c4 — __ZN3rbx14implementation12typed_holderIN3RBX7Region3EE13destruct_funcEPc
pub fn stub_6fd7c4() -> ! {
    todo!("0x6fd7c4 __ZN3rbx14implementation12typed_holderIN3RBX7Region3EE13destruct_funcEPc")
}

#[doc(alias = "double & rbx::any_cast<double &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fd7c8 — __ZN3rbx8any_castIRdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6fd7c8() -> ! {
    todo!("0x6fd7c8 __ZN3rbx8any_castIRdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "float & rbx::any_cast<float &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fd954 — __ZN3rbx8any_castIRfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6fd954() -> ! {
    todo!("0x6fd954 __ZN3rbx8any_castIRfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "bool & rbx::any_cast<bool &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fdae0 — __ZN3rbx8any_castIRbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6fdae0() -> ! {
    todo!("0x6fdae0 __ZN3rbx8any_castIRbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "int & rbx::any_cast<int &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fdc6c — __ZN3rbx8any_castIRiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6fdc6c() -> ! {
    todo!("0x6fdc6c __ZN3rbx8any_castIRiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::NormalId * rbx::any_cast<RBX::NormalId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x6fe0ac — __ZN3rbx8any_castIN3RBX8NormalIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_6fe0ac() -> ! {
    todo!("0x6fe0ac __ZN3rbx8any_castIN3RBX8NormalIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::NormalId & rbx::any_cast<RBX::NormalId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x6fe104 — __ZN3rbx8any_castIRN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_6fe104() -> ! {
    todo!("0x6fe104 __ZN3rbx8any_castIRN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::NormalId,std::allocator<RBX::NormalId>>::resize(unsigned long,RBX::NormalId)")]
// 0x6fe1f4 — __ZNSt6vectorIN3RBX8NormalIdESaIS1_EE6resizeEmS1_
pub fn stub_6fe1f4() -> ! {
    todo!("0x6fe1f4 __ZNSt6vectorIN3RBX8NormalIdESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<RBX::NormalId,std::allocator<RBX::NormalId>>::push_back(RBX::NormalId const&)")]
// 0x6fe228 — __ZNSt6vectorIN3RBX8NormalIdESaIS1_EE9push_backERKS1_
pub fn stub_6fe228() -> ! {
    todo!("0x6fe228 __ZNSt6vectorIN3RBX8NormalIdESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::NormalId,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NormalId>>>::operator[](RBX::Name const* const&)")]
// 0x6fe250 — __ZNSt3mapIPKN3RBX4NameENS0_8NormalIdESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
pub fn stub_6fe250() -> ! {
    todo!("0x6fe250 __ZNSt3mapIPKN3RBX4NameENS0_8NormalIdESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NormalId>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NormalId>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NormalId>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::NormalId>>,std::pair<RBX::Name const* const,RBX::NormalId> const&)")]
// 0x6fe2a8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_6fe2a8() -> ! {
    todo!("0x6fe2a8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NormalId>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NormalId>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NormalId>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::NormalId> const&)")]
// 0x6fe35c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
pub fn stub_6fe35c() -> ! {
    todo!("0x6fe35c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NormalId>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NormalId>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NormalId>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::NormalId> const&)")]
// 0x6fe3b4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_6fe3b4() -> ! {
    todo!("0x6fe3b4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::vector<RBX::NormalId,std::allocator<RBX::NormalId>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::NormalId*,std::vector<RBX::NormalId,std::allocator<RBX::NormalId>>>,RBX::NormalId const&)")]
// 0x6fe41c — __ZNSt6vectorIN3RBX8NormalIdESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_6fe41c() -> ! {
    todo!("0x6fe41c __ZNSt6vectorIN3RBX8NormalIdESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::_Vector_base<RBX::NormalId,std::allocator<RBX::NormalId>>::_M_allocate(unsigned long)")]
// 0x6fe500 — __ZNSt12_Vector_baseIN3RBX8NormalIdESaIS1_EE11_M_allocateEm
pub fn stub_6fe500() -> ! {
    todo!("0x6fe500 __ZNSt12_Vector_baseIN3RBX8NormalIdESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::NormalId * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::NormalId *,RBX::NormalId *>(RBX::NormalId *,RBX::NormalId *,RBX::NormalId *)")]
// 0x6fe518 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NormalIdES5_EET0_T_S7_S6_
pub fn stub_6fe518() -> ! {
    todo!("0x6fe518 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NormalIdES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::vector<RBX::NormalId,std::allocator<RBX::NormalId>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::NormalId*,std::vector<RBX::NormalId,std::allocator<RBX::NormalId>>>,unsigned long,RBX::NormalId const&)")]
// 0x6fe554 — __ZNSt6vectorIN3RBX8NormalIdESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_6fe554() -> ! {
    todo!("0x6fe554 __ZNSt6vectorIN3RBX8NormalIdESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NormalId>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NormalId>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NormalId>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::NormalId>> *)")]
// 0x6fe6e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_6fe6e4() -> ! {
    todo!("0x6fe6e4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8NormalIdEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "XmlElement::addChild(RBX::Name const&)")]
// 0x7038f8 — __ZN10XmlElement8addChildERKN3RBX4NameE
pub fn stub_7038f8() -> ! {
    todo!("0x7038f8 __ZN10XmlElement8addChildERKN3RBX4NameE")
}

#[doc(alias = "RBX::Security::Context::requirePermission(RBX::Security::Permissions,char const*)const")]
// 0x703be0 — __ZNK3RBX8Security7Context17requirePermissionENS0_11PermissionsEPKc
pub fn stub_703be0() -> ! {
    todo!("0x703be0 __ZNK3RBX8Security7Context17requirePermissionENS0_11PermissionsEPKc")
}

#[doc(alias = "RBX::FWStringValue::FWStringValue(char const*)")]
// 0x704688 — __ZN3RBX13FWStringValueC1EPKc
pub fn stub_704688() -> ! {
    todo!("0x704688 __ZN3RBX13FWStringValueC1EPKc")
}

#[doc(alias = "RBX::FWValue<std::string>::set(std::string const&,RBX::FWRef *)")]
// 0x7047ec — __ZN3RBX7FWValueISsE3setERKSsPNS_5FWRefE
pub fn stub_7047ec() -> ! {
    todo!("0x7047ec __ZN3RBX7FWValueISsE3setERKSsPNS_5FWRefE")
}

#[doc(alias = "RBX::FWBase::FWBase(void)")]
// 0x704b6c — __ZN3RBX6FWBaseC2Ev
pub fn stub_704b6c() -> ! {
    todo!("0x704b6c __ZN3RBX6FWBaseC2Ev")
}

#[doc(alias = "RBX::FWBase::operator==(RBX::FWBase const&)const")]
// 0x704cd8 — __ZNK3RBX6FWBaseeqERKS0_
pub fn stub_704cd8() -> ! {
    todo!("0x704cd8 __ZNK3RBX6FWBaseeqERKS0_")
}

#[doc(alias = "RBX::Allocator<XmlAttribute>::operator delete(void *)")]
// 0x705740 — __ZN3RBX9AllocatorI12XmlAttributeEdlEPv
pub fn stub_705740() -> ! {
    todo!("0x705740 __ZN3RBX9AllocatorI12XmlAttributeEdlEPv")
}

#[doc(alias = "RBX::StringConverter<RBX::Region3>::convertToString(RBX::Region3 const&)")]
// 0x71281c — __ZN3RBX15StringConverterINS_7Region3EE15convertToStringERKS1_
pub fn stub_71281c() -> ! {
    todo!("0x71281c __ZN3RBX15StringConverterINS_7Region3EE15convertToStringERKS1_")
}

#[doc(alias = "RBX::StringConverter<RBX::Region3>::convertToValue(std::string const&,RBX::Region3&)")]
// 0x712980 — __ZN3RBX15StringConverterINS_7Region3EE14convertToValueERKSsRS1_
pub fn stub_712980() -> ! {
    todo!("0x712980 __ZN3RBX15StringConverterINS_7Region3EE14convertToValueERKSsRS1_")
}

#[doc(alias = "RBX::StringConverter<RBX::Region3int16>::convertToString(RBX::Region3int16 const&)")]
// 0x712984 — __ZN3RBX15StringConverterINS_12Region3int16EE15convertToStringERKS1_
pub fn stub_712984() -> ! {
    todo!("0x712984 __ZN3RBX15StringConverterINS_12Region3int16EE15convertToStringERKS1_")
}

#[doc(alias = "RBX::StringConverter<RBX::Region3int16>::convertToValue(std::string const&,RBX::Region3int16&)")]
// 0x712afc — __ZN3RBX15StringConverterINS_12Region3int16EE14convertToValueERKSsRS1_
pub fn stub_712afc() -> ! {
    todo!("0x712afc __ZN3RBX15StringConverterINS_12Region3int16EE14convertToValueERKSsRS1_")
}

#[doc(alias = "RBX::StringConverter<RBX::RbxRay>::convertToString(RBX::RbxRay const&)")]
// 0x712d58 — __ZN3RBX15StringConverterINS_6RbxRayEE15convertToStringERKS1_
pub fn stub_712d58() -> ! {
    todo!("0x712d58 __ZN3RBX15StringConverterINS_6RbxRayEE15convertToStringERKS1_")
}

#[doc(alias = "RBX::StringConverter<RBX::RbxRay>::convertToValue(std::string const&,RBX::RbxRay&)")]
// 0x713188 — __ZN3RBX15StringConverterINS_6RbxRayEE14convertToValueERKSsRS1_
pub fn stub_713188() -> ! {
    todo!("0x713188 __ZN3RBX15StringConverterINS_6RbxRayEE14convertToValueERKSsRS1_")
}

#[doc(alias = "RBX::StringConverter<RBX::BrickColor>::convertToString(RBX::BrickColor const&)")]
// 0x7133e8 — __ZN3RBX15StringConverterINS_10BrickColorEE15convertToStringERKS1_
pub fn stub_7133e8() -> ! {
    todo!("0x7133e8 __ZN3RBX15StringConverterINS_10BrickColorEE15convertToStringERKS1_")
}

#[doc(alias = "RBX::StringConverter<RBX::CellID>::convertToString(RBX::CellID const&)")]
// 0x713400 — __ZN3RBX15StringConverterINS_6CellIDEE15convertToStringERKS1_
pub fn stub_713400() -> ! {
    todo!("0x713400 __ZN3RBX15StringConverterINS_6CellIDEE15convertToStringERKS1_")
}

#[doc(alias = "RBX::StringConverter<RBX::ContentId>::convertToValue(std::string const&,RBX::ContentId&)")]
// 0x713de0 — __ZN3RBX15StringConverterINS_9ContentIdEE14convertToValueERKSsRS1_
pub fn stub_713de0() -> ! {
    todo!("0x713de0 __ZN3RBX15StringConverterINS_9ContentIdEE14convertToValueERKSsRS1_")
}

#[doc(alias = "RBX::StringConverter<RBX::BrickColor>::convertToValue(std::string const&,RBX::BrickColor&)")]
// 0x713f04 — __ZN3RBX15StringConverterINS_10BrickColorEE14convertToValueERKSsRS1_
pub fn stub_713f04() -> ! {
    todo!("0x713f04 __ZN3RBX15StringConverterINS_10BrickColorEE14convertToValueERKSsRS1_")
}

#[doc(alias = "RBX::StringConverter<RBX::ContentId>::convertToString(RBX::ContentId const&)")]
// 0x713f20 — __ZN3RBX15StringConverterINS_9ContentIdEE15convertToStringERKS1_
pub fn stub_713f20() -> ! {
    todo!("0x713f20 __ZN3RBX15StringConverterINS_9ContentIdEE15convertToStringERKS1_")
}

#[doc(alias = "RBX::ServiceProvider::findPublicServiceByClassNameString(std::string)")]
// 0x7140c4 — __ZN3RBX15ServiceProvider34findPublicServiceByClassNameStringESs
pub fn stub_7140c4() -> ! {
    todo!("0x7140c4 __ZN3RBX15ServiceProvider34findPublicServiceByClassNameStringESs")
}

#[doc(alias = "RBX::ServiceProvider::getPublicServiceByClassNameString(std::string)")]
// 0x714248 — __ZN3RBX15ServiceProvider33getPublicServiceByClassNameStringESs
pub fn stub_714248() -> ! {
    todo!("0x714248 __ZN3RBX15ServiceProvider33getPublicServiceByClassNameStringESs")
}

#[doc(alias = "RBX::ServiceProvider::newIndex(void)")]
// 0x7143cc — __ZN3RBX15ServiceProvider8newIndexEv
pub fn stub_7143cc() -> ! {
    todo!("0x7143cc __ZN3RBX15ServiceProvider8newIndexEv")
}

#[doc(alias = "RBX::ServiceProvider::findServiceByClassName(RBX::Name const&)const")]
// 0x714540 — __ZNK3RBX15ServiceProvider22findServiceByClassNameERKNS_4NameE
pub fn stub_714540() -> ! {
    todo!("0x714540 __ZNK3RBX15ServiceProvider22findServiceByClassNameERKNS_4NameE")
}

#[doc(alias = "RBX::ServiceProvider::clearServices(void)")]
// 0x7146e8 — __ZN3RBX15ServiceProvider13clearServicesEv
pub fn stub_7146e8() -> ! {
    todo!("0x7146e8 __ZN3RBX15ServiceProvider13clearServicesEv")
}

#[doc(alias = "RBX::ServiceProvider::createChild(RBX::Name const&,RBX::CreatorRole)")]
// 0x714978 — __ZN3RBX15ServiceProvider11createChildERKNS_4NameENS_11CreatorRoleE
pub fn stub_714978() -> ! {
    todo!("0x714978 __ZN3RBX15ServiceProvider11createChildERKNS_4NameENS_11CreatorRoleE")
}

#[doc(alias = "RBX::Verb::Verb(RBX::VerbContainer *,std::string const&)")]
// 0x715ea0 — __ZN3RBX4VerbC2EPNS_13VerbContainerERKSs
pub fn stub_715ea0() -> ! {
    todo!("0x715ea0 __ZN3RBX4VerbC2EPNS_13VerbContainerERKSs")
}

#[doc(alias = "RBX::VerbContainer::addVerb(RBX::Verb *)")]
// 0x715ed8 — __ZN3RBX13VerbContainer7addVerbEPNS_4VerbE
pub fn stub_715ed8() -> ! {
    todo!("0x715ed8 __ZN3RBX13VerbContainer7addVerbEPNS_4VerbE")
}

#[doc(alias = "RBX::Verb::~Verb()")]
// 0x715f8c — __ZN3RBX4VerbD0Ev
pub fn stub_715f8c() -> ! {
    todo!("0x715f8c __ZN3RBX4VerbD0Ev")
}

#[doc(alias = "RBX::Verb::~Verb()")]
// 0x716044 — __ZN3RBX4VerbD1Ev
pub fn stub_716044() -> ! {
    todo!("0x716044 __ZN3RBX4VerbD1Ev")
}

#[doc(alias = "RBX::Verb::~Verb()")]
// 0x716068 — __ZN3RBX4VerbD2Ev
pub fn stub_716068() -> ! {
    todo!("0x716068 __ZN3RBX4VerbD2Ev")
}

#[doc(alias = "RBX::VerbContainer::removeVerb(RBX::Verb *)")]
// 0x71608c — __ZN3RBX13VerbContainer10removeVerbEPNS_4VerbE
pub fn stub_71608c() -> ! {
    todo!("0x71608c __ZN3RBX13VerbContainer10removeVerbEPNS_4VerbE")
}

#[doc(alias = "RBX::VerbContainer::VerbContainer(RBX::VerbContainer*)")]
// 0x7160f4 — __ZN3RBX13VerbContainerC2EPS0_
pub fn stub_7160f4() -> ! {
    todo!("0x7160f4 __ZN3RBX13VerbContainerC2EPS0_")
}

#[doc(alias = "RBX::VerbContainer::setVerbParent(RBX::VerbContainer*)")]
// 0x71611c — __ZN3RBX13VerbContainer13setVerbParentEPS0_
pub fn stub_71611c() -> ! {
    todo!("0x71611c __ZN3RBX13VerbContainer13setVerbParentEPS0_")
}
