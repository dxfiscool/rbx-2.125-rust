//! core shard IK — 100 core stubs EA-sorted, continuation after IJ 0x5dbcfc (EA-sorted ascending, next 100 uncovered).
//!
//! Source: `ida/export.json` filtered where demangled/mangled contains RBX::|boost, excludes Reflection|DataModel|Ogre|RakNet|Lua, EA-sorted, next 100 uncovered after 0x5dbcfc.
//!
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::getComponent(XmlElement const*,RBX::Name const&)")]
// 0x5dfc80 — __ZN3RBXL12getComponentEPK10XmlElementRKNS_4NameE
// type: _DWORD __fastcall(RBX *__hidden this, const XmlElement *, const RBX::Name *)
// was: RBX::getComponent(XmlElement const*,RBX::Name const&)
pub fn stub_5dfc80() -> ! {
    todo!("0x5dfc80 RBX::getComponent(XmlElement const*,RBX::Name const&)")
}

#[doc(alias = "RBX::Primitive::getExtentsWorld(void)const")]
// 0x5e0f90 — __ZNK3RBX9Primitive15getExtentsWorldEv
// type: _DWORD __fastcall(RBX::Primitive *__hidden this)
// was: RBX::Primitive::getExtentsWorld(void)const
pub fn stub_5e0f90() -> ! {
    todo!("0x5e0f90 RBX::Primitive::getExtentsWorld(void)const")
}

#[doc(alias = "RBX::IMoving::~IMoving()")]
// 0x5e12a8 — __ZN3RBX7IMovingD2Ev
// type: void __fastcall(RBX::IMoving *__hidden this)
// was: RBX::IMoving::~IMoving()
pub fn stub_5e12a8() -> ! {
    todo!("0x5e12a8 RBX::IMoving::~IMoving()")
}

#[doc(alias = "RBX::Dragger::dragSnap(void)")]
// 0x5e1468 — __ZN3RBX7Dragger8dragSnapEv
// type: _DWORD __fastcall(RBX::Dragger *__hidden this)
// was: RBX::Dragger::dragSnap(void)
pub fn stub_5e1468() -> ! {
    todo!("0x5e1468 RBX::Dragger::dragSnap(void)")
}

#[doc(alias = "RBX::Joint::getNormalId(int)const")]
// 0x5e14bc — __ZNK3RBX5Joint11getNormalIdEi
// type: _DWORD __fastcall(RBX::Joint *__hidden this, int)
// was: RBX::Joint::getNormalId(int)const
pub fn stub_5e14bc() -> ! {
    todo!("0x5e14bc RBX::Joint::getNormalId(int)const")
}

#[doc(alias = "RBX::FWValue<RBX::World *>::set(RBX::World * const&,RBX::FWRef *)")]
// 0x5e1bc0 — __ZN3RBX7FWValueIPNS_5WorldEE3setERKS2_PNS_5FWRefE
// type: int(void)
// was: RBX::FWValue<RBX::World *>::set(RBX::World * const&,RBX::FWRef *)
pub fn stub_5e1bc0() -> ! {
    todo!("0x5e1bc0 RBX::FWValue<RBX::World *>::set(RBX::World * const&,RBX::FWRef *)")
}

#[doc(alias = "RBX::FWValue<RBX::Material>::set(RBX::Material const&,RBX::FWRef *)")]
// 0x5e21fc — __ZN3RBX7FWValueINS_8MaterialEE3setERKS1_PNS_5FWRefE
// type: int(void)
// was: RBX::FWValue<RBX::Material>::set(RBX::Material const&,RBX::FWRef *)
pub fn stub_5e21fc() -> ! {
    todo!("0x5e21fc RBX::FWValue<RBX::Material>::set(RBX::Material const&,RBX::FWRef *)")
}

#[doc(alias = "RBX::FWValue<float>::set(float const&,RBX::FWRef *)")]
// 0x5e2424 — __ZN3RBX7FWValueIfE3setERKfPNS_5FWRefE
// type: int(void)
// was: RBX::FWValue<float>::set(float const&,RBX::FWRef *)
pub fn stub_5e2424() -> ! {
    todo!("0x5e2424 RBX::FWValue<float>::set(float const&,RBX::FWRef *)")
}

#[doc(alias = "RBX::FWValue<RBX::BrickColor>::set(RBX::BrickColor const&,RBX::FWRef *)")]
// 0x5e2658 — __ZN3RBX7FWValueINS_10BrickColorEE3setERKS1_PNS_5FWRefE
// type: int(void)
// was: RBX::FWValue<RBX::BrickColor>::set(RBX::BrickColor const&,RBX::FWRef *)
pub fn stub_5e2658() -> ! {
    todo!("0x5e2658 RBX::FWValue<RBX::BrickColor>::set(RBX::BrickColor const&,RBX::FWRef *)")
}

#[doc(alias = "RBX::CameraSubject::onCameraWrapMouse(void)")]
// 0x5e2c00 — __ZN3RBX13CameraSubject17onCameraWrapMouseEv
// type: _DWORD __fastcall(RBX::CameraSubject *__hidden this)
// was: RBX::CameraSubject::onCameraWrapMouse(void)
pub fn stub_5e2c00() -> ! {
    todo!("0x5e2c00 RBX::CameraSubject::onCameraWrapMouse(void)")
}

#[doc(alias = "RBX::CameraSubject::getSelectionIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
// 0x5e2c48 — __ZN3RBX13CameraSubject28getSelectionIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
// type: 
// was: RBX::CameraSubject::getSelectionIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)
pub fn stub_5e2c48() -> ! {
    todo!("0x5e2c48 RBX::CameraSubject::getSelectionIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")
}

#[doc(alias = "RBX::Selectable::isSelectable3d(void)")]
// 0x5e2c90 — __ZN3RBX10Selectable14isSelectable3dEv
// type: _DWORD __fastcall(RBX::Selectable *__hidden this)
// was: RBX::Selectable::isSelectable3d(void)
pub fn stub_5e2c90() -> ! {
    todo!("0x5e2c90 RBX::Selectable::isSelectable3d(void)")
}

#[doc(alias = "RBX::CameraSubject::~CameraSubject()")]
// 0x5e2fa0 — __ZN3RBX13CameraSubjectD1Ev
// type: void __fastcall(RBX::CameraSubject *__hidden this)
// was: RBX::CameraSubject::~CameraSubject()
pub fn stub_5e2fa0() -> ! {
    todo!("0x5e2fa0 RBX::CameraSubject::~CameraSubject()")
}

#[doc(alias = "RBX::CameraSubject::~CameraSubject()")]
// 0x5e2fa4 — __ZN3RBX13CameraSubjectD0Ev
// type: void __fastcall(RBX::CameraSubject *__hidden this)
// was: RBX::CameraSubject::~CameraSubject()
pub fn stub_5e2fa4() -> ! {
    todo!("0x5e2fa4 RBX::CameraSubject::~CameraSubject()")
}

#[doc(alias = "RBX::CameraSubject::onCameraNear(float)")]
// 0x5e2fa8 — __ZN3RBX13CameraSubject12onCameraNearEf
// type: _DWORD __fastcall(RBX::CameraSubject *__hidden this, float)
// was: RBX::CameraSubject::onCameraNear(float)
pub fn stub_5e2fa8() -> ! {
    todo!("0x5e2fa8 RBX::CameraSubject::onCameraNear(float)")
}

#[doc(alias = "RBX::CameraSubject::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
// 0x5e2fac — __ZN3RBX13CameraSubject25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
// type: 
// was: RBX::CameraSubject::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)
pub fn stub_5e2fac() -> ! {
    todo!("0x5e2fac RBX::CameraSubject::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Material>(RBX::Material const&)")]
// 0x5e3d48 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8MaterialEEERS3_RKT_
// type: int(void)
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Material>(RBX::Material const&)
pub fn stub_5e3d48() -> ! {
    todo!("0x5e3d48 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Material>(RBX::Material const&)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Material>::singleton(void)")]
// 0x5e3d98 — __ZN3rbx14implementation12typed_holderIN3RBX8MaterialEE9singletonEv
// type: int(void)
// was: rbx::implementation::typed_holder<RBX::Material>::singleton(void)
pub fn stub_5e3d98() -> ! {
    todo!("0x5e3d98 rbx::implementation::typed_holder<RBX::Material>::singleton(void)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Material>::construct_func(char const*,char *)")]
// 0x5e3e04 — __ZN3rbx14implementation12typed_holderIN3RBX8MaterialEE14construct_funcEPKcPc
// type: 
// was: rbx::implementation::typed_holder<RBX::Material>::construct_func(char const*,char *)
pub fn stub_5e3e04() -> ! {
    todo!("0x5e3e04 rbx::implementation::typed_holder<RBX::Material>::construct_func(char const*,char *)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Material>::destruct_func(char *)")]
// 0x5e3e10 — __ZN3rbx14implementation12typed_holderIN3RBX8MaterialEE13destruct_funcEPc
// type: 
// was: rbx::implementation::typed_holder<RBX::Material>::destruct_func(char *)
pub fn stub_5e3e10() -> ! {
    todo!("0x5e3e10 rbx::implementation::typed_holder<RBX::Material>::destruct_func(char *)")
}

#[doc(alias = "RBX::Material const& rbx::any_cast<RBX::Material const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5e3ee0 — __ZN3rbx8any_castIRKN3RBX8MaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
// was: RBX::Material const& rbx::any_cast<RBX::Material const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_5e3ee0() -> ! {
    todo!("0x5e3ee0 RBX::Material const& rbx::any_cast<RBX::Material const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

#[doc(alias = "XmlElement::XmlElement<float>(RBX::Name const&,float)")]
// 0x5e4cb4 — __ZN10XmlElementC2IfEERKN3RBX4NameET_
// type: 
// was: XmlElement::XmlElement<float>(RBX::Name const&,float)
pub fn stub_5e4cb4() -> ! {
    todo!("0x5e4cb4 XmlElement::XmlElement<float>(RBX::Name const&,float)")
}

#[doc(alias = "RBX::FWService * RBX::ServiceProvider::find<RBX::FWService>(void)const")]
// 0x5e6a00 — __ZNK3RBX15ServiceProvider4findINS_9FWServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::FWService * RBX::ServiceProvider::find<RBX::FWService>(void)const
pub fn stub_5e6a00() -> ! {
    todo!("0x5e6a00 RBX::FWService * RBX::ServiceProvider::find<RBX::FWService>(void)const")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FWService>(void)")]
// 0x5e6d44 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_9FWServiceEEEvv
// type: 
// was: void RBX::ServiceProvider::callDoGetClassIndex<RBX::FWService>(void)
pub fn stub_5e6d44() -> ! {
    todo!("0x5e6d44 void RBX::ServiceProvider::callDoGetClassIndex<RBX::FWService>(void)")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FWService>(void)")]
// 0x5e6d48 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_9FWServiceEEEmv
// type: 
// was: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FWService>(void)
pub fn stub_5e6d48() -> ! {
    todo!("0x5e6d48 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FWService>(void)")
}

#[doc(alias = "RBX::SystemAddress const& rbx::any_cast<RBX::SystemAddress const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5ed068 — __ZN3rbx8any_castIRKN3RBX13SystemAddressENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
// was: RBX::SystemAddress const& rbx::any_cast<RBX::SystemAddress const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_5ed068() -> ! {
    todo!("0x5ed068 RBX::SystemAddress const& rbx::any_cast<RBX::SystemAddress const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SystemAddress>(RBX::SystemAddress const&)")]
// 0x5ed158 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SystemAddressEEERS3_RKT_
// type: int(void)
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SystemAddress>(RBX::SystemAddress const&)
pub fn stub_5ed158() -> ! {
    todo!("0x5ed158 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SystemAddress>(RBX::SystemAddress const&)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SystemAddress>::singleton(void)")]
// 0x5ed1b0 — __ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE9singletonEv
// type: int(void)
// was: rbx::implementation::typed_holder<RBX::SystemAddress>::singleton(void)
pub fn stub_5ed1b0() -> ! {
    todo!("0x5ed1b0 rbx::implementation::typed_holder<RBX::SystemAddress>::singleton(void)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SystemAddress>::construct_func(char const*,char *)")]
// 0x5ed21c — __ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE14construct_funcEPKcPc
// type: 
// was: rbx::implementation::typed_holder<RBX::SystemAddress>::construct_func(char const*,char *)
pub fn stub_5ed21c() -> ! {
    todo!("0x5ed21c rbx::implementation::typed_holder<RBX::SystemAddress>::construct_func(char const*,char *)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SystemAddress>::destruct_func(char *)")]
// 0x5ed22c — __ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE13destruct_funcEPc
// type: 
// was: rbx::implementation::typed_holder<RBX::SystemAddress>::destruct_func(char *)
pub fn stub_5ed22c() -> ! {
    todo!("0x5ed22c rbx::implementation::typed_holder<RBX::SystemAddress>::destruct_func(char *)")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::NormalId>(RBX::NormalId const&)")]
// 0x5eddb0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8NormalIdEEERS3_RKT_
// type: int(void)
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::NormalId>(RBX::NormalId const&)
pub fn stub_5eddb0() -> ! {
    todo!("0x5eddb0 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::NormalId>(RBX::NormalId const&)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::NormalId>::singleton(void)")]
// 0x5ede00 — __ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE9singletonEv
// type: int(void)
// was: rbx::implementation::typed_holder<RBX::NormalId>::singleton(void)
pub fn stub_5ede00() -> ! {
    todo!("0x5ede00 rbx::implementation::typed_holder<RBX::NormalId>::singleton(void)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::NormalId>::destruct_func(char *)")]
// 0x5ede70 — __ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE13destruct_funcEPc
// type: 
// was: rbx::implementation::typed_holder<RBX::NormalId>::destruct_func(char *)
pub fn stub_5ede70() -> ! {
    todo!("0x5ede70 rbx::implementation::typed_holder<RBX::NormalId>::destruct_func(char *)")
}

#[doc(alias = "RBX::NormalId const& rbx::any_cast<RBX::NormalId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5ede78 — __ZN3rbx8any_castIRKN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
// was: RBX::NormalId const& rbx::any_cast<RBX::NormalId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_5ede78() -> ! {
    todo!("0x5ede78 RBX::NormalId const& rbx::any_cast<RBX::NormalId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

#[doc(alias = "RBX::Faces const& rbx::any_cast<RBX::Faces const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5eeb20 — __ZN3rbx8any_castIRKN3RBX5FacesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
// was: RBX::Faces const& rbx::any_cast<RBX::Faces const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_5eeb20() -> ! {
    todo!("0x5eeb20 RBX::Faces const& rbx::any_cast<RBX::Faces const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Faces>(RBX::Faces const&)")]
// 0x5eec10 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5FacesEEERS3_RKT_
// type: int **__fastcall(int **, int **)
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Faces>(RBX::Faces const&)
pub fn stub_5eec10() -> ! {
    todo!("0x5eec10 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Faces>(RBX::Faces const&)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Faces>::singleton(void)")]
// 0x5eec60 — __ZN3rbx14implementation12typed_holderIN3RBX5FacesEE9singletonEv
// type: int(void)
// was: rbx::implementation::typed_holder<RBX::Faces>::singleton(void)
pub fn stub_5eec60() -> ! {
    todo!("0x5eec60 rbx::implementation::typed_holder<RBX::Faces>::singleton(void)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Faces>::destruct_func(char *)")]
// 0x5eecd0 — __ZN3rbx14implementation12typed_holderIN3RBX5FacesEE13destruct_funcEPc
// type: 
// was: rbx::implementation::typed_holder<RBX::Faces>::destruct_func(char *)
pub fn stub_5eecd0() -> ! {
    todo!("0x5eecd0 rbx::implementation::typed_holder<RBX::Faces>::destruct_func(char *)")
}

#[doc(alias = "RBX::BrickColor const& rbx::any_cast<RBX::BrickColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5f0010 — __ZN3rbx8any_castIRKN3RBX10BrickColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
// was: RBX::BrickColor const& rbx::any_cast<RBX::BrickColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_5f0010() -> ! {
    todo!("0x5f0010 RBX::BrickColor const& rbx::any_cast<RBX::BrickColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::BrickColor>(RBX::BrickColor const&)")]
// 0x5f0100 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10BrickColorEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::BrickColor>(RBX::BrickColor const&)
pub fn stub_5f0100() -> ! {
    todo!("0x5f0100 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::BrickColor>(RBX::BrickColor const&)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::BrickColor>::singleton(void)")]
// 0x5f0150 — __ZN3rbx14implementation12typed_holderIN3RBX10BrickColorEE9singletonEv
// type: int(void)
// was: rbx::implementation::typed_holder<RBX::BrickColor>::singleton(void)
pub fn stub_5f0150() -> ! {
    todo!("0x5f0150 rbx::implementation::typed_holder<RBX::BrickColor>::singleton(void)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::BrickColor>::destruct_func(char *)")]
// 0x5f01c0 — __ZN3rbx14implementation12typed_holderIN3RBX10BrickColorEE13destruct_funcEPc
// type: 
// was: rbx::implementation::typed_holder<RBX::BrickColor>::destruct_func(char *)
pub fn stub_5f01c0() -> ! {
    todo!("0x5f01c0 rbx::implementation::typed_holder<RBX::BrickColor>::destruct_func(char *)")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Material,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::operator[](RBX::Name const* const&)")]
// 0x5f1fb4 — __ZNSt3mapIPKN3RBX4NameENS0_8MaterialESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
// type: _Rb_tree_node_base **__fastcall(int, int *)
// was: std::map<RBX::Name const*,RBX::Material,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::operator[](RBX::Name const* const&)
pub fn stub_5f1fb4() -> ! {
    todo!("0x5f1fb4 std::map<RBX::Name const*,RBX::Material,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::operator[](RBX::Name const* const&)")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Material>>,std::pair<RBX::Name const* const,RBX::Material> const&)")]
// 0x5f200c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Material>>,std::pair<RBX::Name const* const,RBX::Material> const&)
pub fn stub_5f200c() -> ! {
    todo!("0x5f200c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Material>>,std::pair<RBX::Name const* const,RBX::Material> const&)")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Material> const&)")]
// 0x5f20c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Material> const&)
pub fn stub_5f20c0() -> ! {
    todo!("0x5f20c0 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Material> const&)")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Material> const&)")]
// 0x5f2118 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int *)
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Material> const&)
pub fn stub_5f2118() -> ! {
    todo!("0x5f2118 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Material> const&)")
}

#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::resize(unsigned long,RBX::Material)")]
// 0x5f21ac — __ZNSt6vectorIN3RBX8MaterialESaIS1_EE6resizeEmS1_
// type: int(void)
// was: std::vector<RBX::Material,std::allocator<RBX::Material>>::resize(unsigned long,RBX::Material)
pub fn stub_5f21ac() -> ! {
    todo!("0x5f21ac std::vector<RBX::Material,std::allocator<RBX::Material>>::resize(unsigned long,RBX::Material)")
}

#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::push_back(RBX::Material const&)")]
// 0x5f2214 — __ZNSt6vectorIN3RBX8MaterialESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
// was: std::vector<RBX::Material,std::allocator<RBX::Material>>::push_back(RBX::Material const&)
pub fn stub_5f2214() -> ! {
    todo!("0x5f2214 std::vector<RBX::Material,std::allocator<RBX::Material>>::push_back(RBX::Material const&)")
}

#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,RBX::Material const&)")]
// 0x5f228c — __ZNSt6vectorIN3RBX8MaterialESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
// was: std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,RBX::Material const&)
pub fn stub_5f228c() -> ! {
    todo!("0x5f228c std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,RBX::Material const&)")
}

#[doc(alias = "std::_Vector_base<RBX::Material,std::allocator<RBX::Material>>::_M_allocate(unsigned long)")]
// 0x5f2370 — __ZNSt12_Vector_baseIN3RBX8MaterialESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
// was: std::_Vector_base<RBX::Material,std::allocator<RBX::Material>>::_M_allocate(unsigned long)
pub fn stub_5f2370() -> ! {
    todo!("0x5f2370 std::_Vector_base<RBX::Material,std::allocator<RBX::Material>>::_M_allocate(unsigned long)")
}

#[doc(alias = "RBX::Material * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Material *,RBX::Material *>(RBX::Material *,RBX::Material *,RBX::Material *)")]
// 0x5f2388 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8MaterialES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
// was: RBX::Material * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Material *,RBX::Material *>(RBX::Material *,RBX::Material *,RBX::Material *)
pub fn stub_5f2388() -> ! {
    todo!("0x5f2388 RBX::Material * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Material *,RBX::Material *>(RBX::Material *,RBX::Material *,RBX::Material *)")
}

#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,unsigned long,RBX::Material const&)")]
// 0x5f23c8 — __ZNSt6vectorIN3RBX8MaterialESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(int result, char *, unsigned int, int *)
// was: std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,unsigned long,RBX::Material const&)
pub fn stub_5f23c8() -> ! {
    todo!("0x5f23c8 std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,unsigned long,RBX::Material const&)")
}

#[doc(alias = "std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::push_back(RBX::Primitive const* const&)")]
// 0x5f3d3c — __ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE9push_backERKS3_
// type: int(void)
// was: std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::push_back(RBX::Primitive const* const&)
pub fn stub_5f3d3c() -> ! {
    todo!("0x5f3d3c std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::push_back(RBX::Primitive const* const&)")
}

#[doc(alias = "std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Primitive const**,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>,RBX::Primitive const* const&)")]
// 0x5f3d68 — __ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int, void *__src)
// was: std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Primitive const**,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>,RBX::Primitive const* const&)
pub fn stub_5f3d68() -> ! {
    todo!("0x5f3d68 std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Primitive const**,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>,RBX::Primitive const* const&)")
}

#[doc(alias = "std::_Vector_base<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_allocate(unsigned long)")]
// 0x5f3e48 — __ZNSt12_Vector_baseIPKN3RBX9PrimitiveESaIS3_EE11_M_allocateEm
// type: int(void)
// was: std::_Vector_base<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_allocate(unsigned long)
pub fn stub_5f3e48() -> ! {
    todo!("0x5f3e48 std::_Vector_base<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_allocate(unsigned long)")
}

#[doc(alias = "RBX::FWBase::~FWBase()")]
// 0x5f3f80 — __ZN3RBX6FWBaseD0Ev
// type: void __fastcall(RBX::FWBase *__hidden this)
// was: RBX::FWBase::~FWBase()
pub fn stub_5f3f80() -> ! {
    todo!("0x5f3f80 RBX::FWBase::~FWBase()")
}

#[doc(alias = "RBX::SurfaceData::empty(void)")]
// 0x5f40dc — __ZN3RBX11SurfaceData5emptyEv
// type: _DWORD __fastcall(RBX::SurfaceData *__hidden this)
// was: RBX::SurfaceData::empty(void)
pub fn stub_5f40dc() -> ! {
    todo!("0x5f40dc RBX::SurfaceData::empty(void)")
}

#[doc(alias = "RBX::EdgeList::getOther(int)const")]
// 0x5f412c — __ZNK3RBX8EdgeList8getOtherEi
// type: _DWORD __fastcall(RBX::EdgeList *__hidden this, int)
// was: RBX::EdgeList::getOther(int)const
pub fn stub_5f412c() -> ! {
    todo!("0x5f412c RBX::EdgeList::getOther(int)const")
}

#[doc(alias = "RBX::Body::updatePV(void)")]
// 0x5f41f0 — __ZN3RBX4Body8updatePVEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
// was: RBX::Body::updatePV(void)
pub fn stub_5f41f0() -> ! {
    todo!("0x5f41f0 RBX::Body::updatePV(void)")
}

#[doc(alias = "RBX::Body::getMeInParent(void)")]
// 0x5f441c — __ZN3RBX4Body13getMeInParentEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
// was: RBX::Body::getMeInParent(void)
pub fn stub_5f441c() -> ! {
    todo!("0x5f441c RBX::Body::getMeInParent(void)")
}

#[doc(alias = "RBX::Security::Context::ptr(void)")]
// 0x5f4488 — __ZN3RBX8Security7Context3ptrEv
// type: _DWORD __fastcall(RBX::Security::Context *__hidden this)
// was: RBX::Security::Context::ptr(void)
pub fn stub_5f4488() -> ! {
    todo!("0x5f4488 RBX::Security::Context::ptr(void)")
}

#[doc(alias = "RBX::RbxRay::~RbxRay()")]
// 0x5f45c8 — __ZN3RBX6RbxRayD0Ev
// type: void __fastcall(RBX::RbxRay *__hidden this)
// was: RBX::RbxRay::~RbxRay()
pub fn stub_5f45c8() -> ! {
    todo!("0x5f45c8 RBX::RbxRay::~RbxRay()")
}

#[doc(alias = "RBX::InterpolatedCFrame::InterpolatedCFrame(void)")]
// 0x5f46f4 — __ZN3RBX18InterpolatedCFrameC2Ev
// type: _DWORD __fastcall(RBX::InterpolatedCFrame *__hidden this)
// was: RBX::InterpolatedCFrame::InterpolatedCFrame(void)
pub fn stub_5f46f4() -> ! {
    todo!("0x5f46f4 RBX::InterpolatedCFrame::InterpolatedCFrame(void)")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Material>> *)")]
// 0x5f5634 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int(void)
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Material>> *)
pub fn stub_5f5634() -> ! {
    todo!("0x5f5634 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Material>> *)")
}

#[doc(alias = "RBX::PhysicsInstructions::PhysicsInstructions(void)")]
// 0x5f6800 — __ZN3RBX19PhysicsInstructionsC1Ev
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
// was: RBX::PhysicsInstructions::PhysicsInstructions(void)
pub fn stub_5f6800() -> ! {
    todo!("0x5f6800 RBX::PhysicsInstructions::PhysicsInstructions(void)")
}

#[doc(alias = "RBX::PhysicsInstructions::PhysicsInstructions(void)")]
// 0x5f6804 — __ZN3RBX19PhysicsInstructionsC2Ev
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
// was: RBX::PhysicsInstructions::PhysicsInstructions(void)
pub fn stub_5f6804() -> ! {
    todo!("0x5f6804 RBX::PhysicsInstructions::PhysicsInstructions(void)")
}

#[doc(alias = "RBX::PhysicsInstructions::dPhysicsServerDutyPercent(void)")]
// 0x5f6948 — __ZN3RBX19PhysicsInstructions25dPhysicsServerDutyPercentEv
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
// was: RBX::PhysicsInstructions::dPhysicsServerDutyPercent(void)
pub fn stub_5f6948() -> ! {
    todo!("0x5f6948 RBX::PhysicsInstructions::dPhysicsServerDutyPercent(void)")
}

#[doc(alias = "RBX::PhysicsInstructions::dPhysicsClientDutyPercent(void)")]
// 0x5f6a60 — __ZN3RBX19PhysicsInstructions25dPhysicsClientDutyPercentEv
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
// was: RBX::PhysicsInstructions::dPhysicsClientDutyPercent(void)
pub fn stub_5f6a60() -> ! {
    todo!("0x5f6a60 RBX::PhysicsInstructions::dPhysicsClientDutyPercent(void)")
}

#[doc(alias = "RBX::SocialService::getRankInGroup(int,int,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0x6396c4 — __ZN3RBX13SocialService14getRankInGroupEiiN5boost8functionIFviEEENS2_IFvSsEEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
// was: RBX::SocialService::getRankInGroup(int,int,boost::function<void ()(int)>,boost::function<void ()(std::string)>)
pub fn stub_6396c4() -> ! {
    todo!("0x6396c4 RBX::SocialService::getRankInGroup(int,int,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")
}

#[doc(alias = "RBX::SocialService::getRoleInGroup(int,int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0x639910 — __ZN3RBX13SocialService14getRoleInGroupEiiN5boost8functionIFvSsEEES4_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
// was: RBX::SocialService::getRoleInGroup(int,int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
pub fn stub_639910() -> ! {
    todo!("0x639910 RBX::SocialService::getRoleInGroup(int,int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

#[doc(alias = "RBX::SocialService::isFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x639b5c — __ZN3RBX13SocialService13isFriendsWithEiiN5boost8functionIFvbEEENS2_IFvSsEEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
// was: RBX::SocialService::isFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
pub fn stub_639b5c() -> ! {
    todo!("0x639b5c RBX::SocialService::isFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

#[doc(alias = "RBX::SocialService::isBestFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x639da8 — __ZN3RBX13SocialService17isBestFriendsWithEiiN5boost8functionIFvbEEENS2_IFvSsEEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
// was: RBX::SocialService::isBestFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
pub fn stub_639da8() -> ! {
    todo!("0x639da8 RBX::SocialService::isBestFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

#[doc(alias = "RBX::SocialService::isInGroup(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x639ff4 — __ZN3RBX13SocialService9isInGroupEiiN5boost8functionIFvbEEENS2_IFvSsEEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
// was: RBX::SocialService::isInGroup(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
pub fn stub_639ff4() -> ! {
    todo!("0x639ff4 RBX::SocialService::isInGroup(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

#[doc(alias = "void RBX::SocialService::dispatchRequest<int>(std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0x63a5e0 — __ZN3RBX13SocialService15dispatchRequestIiEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// type: int(void)
// was: void RBX::SocialService::dispatchRequest<int>(std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)
pub fn stub_63a5e0() -> ! {
    todo!("0x63a5e0 void RBX::SocialService::dispatchRequest<int>(std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")
}

#[doc(alias = "void RBX::SocialService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0x63a888 — __ZN3RBX13SocialService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// type: int(void)
// was: void RBX::SocialService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
pub fn stub_63a888() -> ! {
    todo!("0x63a888 void RBX::SocialService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

#[doc(alias = "void RBX::SocialService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x63ab30 — __ZN3RBX13SocialService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// type: int(void)
// was: void RBX::SocialService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
pub fn stub_63ab30() -> ! {
    todo!("0x63ab30 void RBX::SocialService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)")]
// 0x665064 — __ZN3RBX11shared_fromINS_4TeamEEEN5boost10shared_ptrIT_EEPS4_
// type: int(void)
// was: boost::shared_ptr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)
pub fn stub_665064() -> ! {
    todo!("0x665064 boost::shared_ptr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(char const*,bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0x668884 — __ZN3rbx7signals6signalIFvPKcbEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int(void)
// was: rbx::signals::connection rbx::signals::signal<void ()(char const*,bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>> const&)
pub fn stub_668884() -> ! {
    todo!("0x668884 rbx::signals::connection rbx::signals::signal<void ()(char const*,bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>> const&)")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::operator()(rbx_core::SharedPtr<RBX::TextBox>)")]
// 0x6688f8 — __ZN3rbx7signals16signal_with_argsILi1EFvN5boost10shared_ptrIN3RBX7TextBoxEEEEEclES6_
// type: int(void)
// was: rbx::signals::signal_with_args<1,void ()(boost::shared_ptr<RBX::TextBox>)>::operator()(boost::shared_ptr<RBX::TextBox>)
pub fn stub_6688f8() -> ! {
    todo!("0x6688f8 rbx::signals::signal_with_args<1,void ()(boost::shared_ptr<RBX::TextBox>)>::operator()(boost::shared_ptr<RBX::TextBox>)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox> RBX::shared_from<RBX::TextBox>(RBX::TextBox*)")]
// 0x668adc — __ZN3RBX11shared_fromINS_7TextBoxEEEN5boost10shared_ptrIT_EEPS4_
// type: int(void)
// was: boost::shared_ptr<RBX::TextBox> RBX::shared_from<RBX::TextBox>(RBX::TextBox*)
pub fn stub_668adc() -> ! {
    todo!("0x668adc boost::shared_ptr<RBX::TextBox> RBX::shared_from<RBX::TextBox>(RBX::TextBox*)")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot> &)")]
// 0x66996c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot> &)
pub fn stub_66996c() -> ! {
    todo!("0x66996c rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot> &)")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::on_error(std::exception &)")]
// 0x669acc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE8on_errorERSt9exception
// type: int(void)
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::on_error(std::exception &)
pub fn stub_669acc() -> ! {
    todo!("0x669acc rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::on_error(std::exception &)")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(rbx::signals::signal<void ()(char const*,bool)>::slot*)")]
// 0x669d00 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKcbEE4slotEEaSEPS8_
// type: int(void)
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(rbx::signals::signal<void ()(char const*,bool)>::slot*)
pub fn stub_669d00() -> ! {
    todo!("0x669d00 boost::intrusive_ptr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(rbx::signals::signal<void ()(char const*,bool)>::slot*)")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x669d24 — __ZN3rbx7signals6signalIFvPKcbEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
// type: 
// was: rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
pub fn stub_669d24() -> ! {
    todo!("0x669d24 rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x669d50 — __ZN3rbx7signals6signalIFvPKcbEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
// type: 
// was: rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
pub fn stub_669d50() -> ! {
    todo!("0x669d50 rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)")]
// 0x669f40 — __ZN3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_E4callES4_b
// type: 
// was: rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)
pub fn stub_669f40() -> ! {
    todo!("0x669f40 rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)")]
// 0x669f68 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_E4callES4_b
// type: 
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)
pub fn stub_669f68() -> ! {
    todo!("0x669f68 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::TextBox *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list2<char const*&,bool &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool> &,boost::_bi::list2<char const*&,bool &> &,int)")]
// 0x669f90 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX7TextBoxEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_PKcbEENS0_5list2IRSF_RbEEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
// was: void boost::_bi::list3<boost::_bi::value<RBX::TextBox *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list2<char const*&,bool &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool> &,boost::_bi::list2<char const*&,bool &> &,int)
pub fn stub_669f90() -> ! {
    todo!("0x669f90 void boost::_bi::list3<boost::_bi::value<RBX::TextBox *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list2<char const*&,bool &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool> &,boost::_bi::list2<char const*&,bool &> &,int)")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()")]
// 0x66a2a0 — __ZN3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_ED1Ev
// type: 
// was: rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()
pub fn stub_66a2a0() -> ! {
    todo!("0x66a2a0 rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()")]
// 0x66a2cc — __ZN3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_ED0Ev
// type: 
// was: rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()
pub fn stub_66a2cc() -> ! {
    todo!("0x66a2cc rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()")
}

#[doc(alias = "boost::scoped_ptr<RBX::GuiObject::Tween>::~scoped_ptr()")]
// 0x66afbc — __ZN5boost10scoped_ptrIN3RBX9GuiObject5TweenEED2Ev
// type: 
// was: boost::scoped_ptr<RBX::GuiObject::Tween>::~scoped_ptr()
pub fn stub_66afbc() -> ! {
    todo!("0x66afbc boost::scoped_ptr<RBX::GuiObject::Tween>::~scoped_ptr()")
}

#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::clear(void)")]
// 0x66b068 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5clearEv
// type: int __fastcall(_DWORD)
// was: boost::function1<void,RBX::GuiObject::TweenStatus>::clear(void)
pub fn stub_66b068() -> ! {
    todo!("0x66b068 boost::function1<void,RBX::GuiObject::TweenStatus>::clear(void)")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot> const&)")]
// 0x66b358 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiEE4slotEEaSERKS7_
// type: int(void)
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot> const&)
pub fn stub_66b358() -> ! {
    todo!("0x66b358 boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot> const&)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::disconnectAll(void)")]
// 0x66b5c4 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(RBX::UDim2)>::disconnectAll(void)
pub fn stub_66b5c4() -> ! {
    todo!("0x66b5c4 rbx::signals::signal<void ()(RBX::UDim2)>::disconnectAll(void)")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> const&)")]
// 0x66b73c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSERKS9_
// type: int(void)
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> const&)
pub fn stub_66b73c() -> ! {
    todo!("0x66b73c boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> const&)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::safe_static_init_mutex(void)")]
// 0x66b760 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE22safe_static_init_mutexEv
// type: 
// was: rbx::signals::signal<void ()(RBX::UDim2)>::safe_static_init_mutex(void)
pub fn stub_66b760() -> ! {
    todo!("0x66b760 rbx::signals::signal<void ()(RBX::UDim2)>::safe_static_init_mutex(void)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::safe_static_do_get_mutex(void)")]
// 0x66b764 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE24safe_static_do_get_mutexEv
// type: int()
// was: rbx::signals::signal<void ()(RBX::UDim2)>::safe_static_do_get_mutex(void)
pub fn stub_66b764() -> ! {
    todo!("0x66b764 rbx::signals::signal<void ()(RBX::UDim2)>::safe_static_do_get_mutex(void)")
}

#[doc(alias = "RBX::TimerService::delay(boost::function0<void>,double)")]
// 0x67d650 — __ZN3RBX12TimerService5delayEN5boost9function0IvEEd
// type: void __fastcall(int, int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, char, int, int, int, int, int, int, int, int, int)
// was: RBX::TimerService::delay(boost::function0<void>,double)
pub fn stub_67d650() -> ! {
    todo!("0x67d650 RBX::TimerService::delay(boost::function0<void>,double)")
}

#[doc(alias = "boost::function0<void>::operator=(boost::function0<void> const&)")]
// 0x67d8fc — __ZN5boost9function0IvEaSERKS1_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::function0<void>::operator=(boost::function0<void> const&)
pub fn stub_67d8fc() -> ! {
    todo!("0x67d8fc boost::function0<void>::operator=(boost::function0<void> const&)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Mouse>::operator=(rbx_core::SharedPtr<RBX::Mouse> const&)")]
// 0x682a28 — __ZN5boost10shared_ptrIN3RBX5MouseEEaSERKS3_
// type: 
// was: boost::shared_ptr<RBX::Mouse>::operator=(boost::shared_ptr<RBX::Mouse> const&)
pub fn stub_682a28() -> ! {
    todo!("0x682a28 boost::shared_ptr<RBX::Mouse>::operator=(boost::shared_ptr<RBX::Mouse> const&)")
}
