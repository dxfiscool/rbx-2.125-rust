//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xf4b304..0xf4df74 (100 stubs, EA-sorted asc, 11660->11760 covered, 1573 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xf4b304 — j___ZN3RBX10Reflection13BoundFuncDescINS_11VirtualUserEFvN3G3D7Vector2ENS3_15CoordinateFrameEELi2EE16declareSignatureEPKcNS0_7VariantES9_SA_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
// IDA 0xf4b304: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4b304() {
}

// 0xf4b314 — j___ZN3RBX10Reflection13BoundFuncDescINS_11VirtualUserEFvN3G3D7Vector2ENS3_15CoordinateFrameEELi2EEC2EMS2_FvS4_S5_EPKcSB_SB_S5_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::BoundFuncDesc(void (RBX::VirtualUser::*)(G3D::Vector2,G3D::CoordinateFrame),char const*,char const*,char const*,G3D::CoordinateFrame,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::BoundFuncDesc(void (RBX::VirtualUser::*)(G3D::Vector2,G3D::CoordinateFrame),char const*,char const*,char const*,G3D::CoordinateFrame,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0xf4b314: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4b314() {
}

// 0xf4b354 — j___ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector2ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "G3D::Vector2 RBX::Reflection::ArgHelper::getArg<G3D::Vector2,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector2> const&,boost::disable_if<boost::is_same<G3D::Vector2,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: G3D::Vector2 RBX::Reflection::ArgHelper::getArg<G3D::Vector2,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector2> const&,boost::disable_if<boost::is_same<G3D::Vector2,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
// IDA 0xf4b354: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4b354() {
}

// 0xf4b5c4 — j___ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::World::TouchInfo,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::World::TouchInfo,10,32ul>::resize(int,bool)
// IDA 0xf4b5c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4b5c4() {
}

// 0xf4b5d4 — j___ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::World::TouchInfo,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::World::TouchInfo,10,32ul>::realloc(int)
// IDA 0xf4b5d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4b5d4() {
}

// 0xf4b5e4 — j___ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::PartInstance *,10,32ul>::Array(void)
// IDA 0xf4b5e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4b5e4() {
}

// 0xf4b5f4 — j___ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::~Array()")]
// was: G3D::Array<RBX::PartInstance *,10,32ul>::~Array()
// IDA 0xf4b5f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4b5f4() {
}

// 0xf4bc24 — j___ZN3rbx14implementation12typed_holderIN3G3D7Vector3EE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3>::singleton(void)")]
// was: rbx::implementation::typed_holder<G3D::Vector3>::singleton(void)
// IDA 0xf4bc24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4bc24() {
}

// 0xf4c2c4 — j___ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::append(RBX::Body * const&)")]
// was: G3D::Array<RBX::Body *,10,32ul>::append(RBX::Body * const&)
// IDA 0xf4c2c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c2c4() {
}

// 0xf4c2d4 — j___ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::Body *,10,32ul>::resize(int,bool)
// IDA 0xf4c2d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c2d4() {
}

// 0xf4c2e4 — j___ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::Body *,10,32ul>::realloc(int)
// IDA 0xf4c2e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c2e4() {
}

// 0xf4c2f4 — j___ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::Body *,10,32ul>::Array(void)
// IDA 0xf4c2f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c2f4() {
}

// 0xf4c304 — j___ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::~Array()")]
// was: G3D::Array<RBX::Body *,10,32ul>::~Array()
// IDA 0xf4c304: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4c304() {
}

// 0xf4c314 — j___ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::append(RBX::Point * const&)")]
// was: G3D::Array<RBX::Point *,10,32ul>::append(RBX::Point * const&)
// IDA 0xf4c314: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c314() {
}

// 0xf4c324 — j___ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::Point *,10,32ul>::resize(int,bool)
// IDA 0xf4c324: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c324() {
}

// 0xf4c334 — j___ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::Point *,10,32ul>::realloc(int)
// IDA 0xf4c334: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c334() {
}

// 0xf4c344 — j___ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::Point *,10,32ul>::Array(void)
// IDA 0xf4c344: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c344() {
}

// 0xf4c354 — j___ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::~Array()")]
// was: G3D::Array<RBX::Point *,10,32ul>::~Array()
// IDA 0xf4c354: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4c354() {
}

// 0xf4c364 — j___ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::append(RBX::SimBody * const&)")]
// was: G3D::Array<RBX::SimBody *,10,32ul>::append(RBX::SimBody * const&)
// IDA 0xf4c364: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c364() {
}

// 0xf4c374 — j___ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::SimBody *,10,32ul>::resize(int,bool)
// IDA 0xf4c374: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c374() {
}

// 0xf4c384 — j___ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::SimBody *,10,32ul>::realloc(int)
// IDA 0xf4c384: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c384() {
}

// 0xf4c394 — j___ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::SimBody *,10,32ul>::Array(void)
// IDA 0xf4c394: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c394() {
}

// 0xf4c3a4 — j___ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::~Array()")]
// was: G3D::Array<RBX::SimBody *,10,32ul>::~Array()
// IDA 0xf4c3a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4c3a4() {
}

// 0xf4c3b4 — j___ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::append(RBX::Connector * const&)")]
// was: G3D::Array<RBX::Connector *,10,32ul>::append(RBX::Connector * const&)
// IDA 0xf4c3b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c3b4() {
}

// 0xf4c3c4 — j___ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::Connector *,10,32ul>::resize(int,bool)
// IDA 0xf4c3c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c3c4() {
}

// 0xf4c3d4 — j___ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::Connector *,10,32ul>::realloc(int)
// IDA 0xf4c3d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c3d4() {
}

// 0xf4c3e4 — j___ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::Connector *,10,32ul>::Array(void)
// IDA 0xf4c3e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c3e4() {
}

// 0xf4c3f4 — j___ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::~Array()")]
// was: G3D::Array<RBX::Connector *,10,32ul>::~Array()
// IDA 0xf4c3f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4c3f4() {
}

// 0xf4c574 — j___ZN3G3D4Line13closestPointsERKS0_S2_RNS_7Vector3ES4_
#[doc(alias = "G3D::Line::closestPoints(G3D::Line const&,G3D::Line const&,G3D::Vector3 &,G3D::Vector3 &)")]
// was: G3D::Line::closestPoints(G3D::Line const&,G3D::Line const&,G3D::Vector3 &,G3D::Vector3 &)
// IDA 0xf4c574: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c574() {
}

// 0xf4c624 — j___ZN3RBX10Reflection4TypeC2IN3G3D12Vector2int16EEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<G3D::Vector2int16>(char const*,G3D::Vector2int16 *)")]
// was: RBX::Reflection::Type::Type<G3D::Vector2int16>(char const*,G3D::Vector2int16 *)
// IDA 0xf4c624: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c624() {
}

// 0xf4c634 — j___ZN3RBX10Reflection4TypeC2IN3G3D12Vector3int16EEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<G3D::Vector3int16>(char const*,G3D::Vector3int16 *)")]
// was: RBX::Reflection::Type::Type<G3D::Vector3int16>(char const*,G3D::Vector3int16 *)
// IDA 0xf4c634: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c634() {
}

// 0xf4c644 — j___ZN3RBX10Reflection4TypeC2IN3G3D6Color3EEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<G3D::Color3>(char const*,G3D::Color3 *)")]
// was: RBX::Reflection::Type::Type<G3D::Color3>(char const*,G3D::Color3 *)
// IDA 0xf4c644: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c644() {
}

// 0xf4c654 — j___ZN3RBX10Reflection4TypeC2IN3G3D7Vector2EEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<G3D::Vector2>(char const*,G3D::Vector2 *)")]
// was: RBX::Reflection::Type::Type<G3D::Vector2>(char const*,G3D::Vector2 *)
// IDA 0xf4c654: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c654() {
}

// 0xf4c664 — j___ZN3RBX10Reflection4TypeC2IN3G3D7Vector3EEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<G3D::Vector3>(char const*,G3D::Vector3 *)")]
// was: RBX::Reflection::Type::Type<G3D::Vector3>(char const*,G3D::Vector3 *)
// IDA 0xf4c664: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c664() {
}

// 0xf4c7a4 — j___ZN3RBX10Reflection7Variant14genericConvertIN3G3D12Vector2int16EEERT_v
#[doc(alias = "G3D::Vector2int16 & RBX::Reflection::Variant::genericConvert<G3D::Vector2int16>(void)")]
// was: G3D::Vector2int16 & RBX::Reflection::Variant::genericConvert<G3D::Vector2int16>(void)
// IDA 0xf4c7a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c7a4() {
}

// 0xf4c7b4 — j___ZN3RBX10Reflection7Variant14genericConvertIN3G3D15CoordinateFrameEEERT_v
#[doc(alias = "G3D::CoordinateFrame & RBX::Reflection::Variant::genericConvert<G3D::CoordinateFrame>(void)")]
// was: G3D::CoordinateFrame & RBX::Reflection::Variant::genericConvert<G3D::CoordinateFrame>(void)
// IDA 0xf4c7b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c7b4() {
}

// 0xf4c7c4 — j___ZN3RBX10Reflection7Variant14genericConvertIN3G3D6Color3EEERT_v
#[doc(alias = "G3D::Color3 & RBX::Reflection::Variant::genericConvert<G3D::Color3>(void)")]
// was: G3D::Color3 & RBX::Reflection::Variant::genericConvert<G3D::Color3>(void)
// IDA 0xf4c7c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c7c4() {
}

// 0xf4c7d4 — j___ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector2EEERT_v
#[doc(alias = "G3D::Vector2 & RBX::Reflection::Variant::genericConvert<G3D::Vector2>(void)")]
// was: G3D::Vector2 & RBX::Reflection::Variant::genericConvert<G3D::Vector2>(void)
// IDA 0xf4c7d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c7d4() {
}

// 0xf4c7e4 — j___ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector3EEERT_v
#[doc(alias = "G3D::Vector3 & RBX::Reflection::Variant::genericConvert<G3D::Vector3>(void)")]
// was: G3D::Vector3 & RBX::Reflection::Variant::genericConvert<G3D::Vector3>(void)
// IDA 0xf4c7e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c7e4() {
}

// 0xf4c904 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D12Vector2int16EEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2int16>(G3D::Vector2int16 const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2int16>(G3D::Vector2int16 const&)
// IDA 0xf4c904: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c904() {
}

// 0xf4c954 — j___ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2int16>::singleton(void)")]
// was: rbx::implementation::typed_holder<G3D::Vector2int16>::singleton(void)
// IDA 0xf4c954: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c954() {
}

// 0xf4c964 — j___ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<G3D::CoordinateFrame>::singleton(void)")]
// was: rbx::implementation::typed_holder<G3D::CoordinateFrame>::singleton(void)
// IDA 0xf4c964: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c964() {
}

// 0xf4c9a4 — j___ZN3rbx8any_castIN3G3D12Vector2int16EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector2int16 * rbx::any_cast<G3D::Vector2int16,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: G3D::Vector2int16 * rbx::any_cast<G3D::Vector2int16,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// IDA 0xf4c9a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c9a4() {
}

// 0xf4c9b4 — j___ZN3rbx8any_castIN3G3D15CoordinateFrameEN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "G3D::CoordinateFrame * rbx::any_cast<G3D::CoordinateFrame,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: G3D::CoordinateFrame * rbx::any_cast<G3D::CoordinateFrame,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// IDA 0xf4c9b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c9b4() {
}

// 0xf4c9c4 — j___ZN3rbx8any_castIN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::CoordinateFrame rbx::any_cast<G3D::CoordinateFrame,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::CoordinateFrame rbx::any_cast<G3D::CoordinateFrame,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xf4c9c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c9c4() {
}

// 0xf4c9d4 — j___ZN3rbx8any_castIN3G3D6Color3EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Color3 * rbx::any_cast<G3D::Color3,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: G3D::Color3 * rbx::any_cast<G3D::Color3,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// IDA 0xf4c9d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c9d4() {
}

// 0xf4c9e4 — j___ZN3rbx8any_castIN3G3D7Vector2EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector2 * rbx::any_cast<G3D::Vector2,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: G3D::Vector2 * rbx::any_cast<G3D::Vector2,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// IDA 0xf4c9e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c9e4() {
}

// 0xf4c9f4 — j___ZN3rbx8any_castIN3G3D7Vector3EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector3 * rbx::any_cast<G3D::Vector3,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: G3D::Vector3 * rbx::any_cast<G3D::Vector3,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// IDA 0xf4c9f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4c9f4() {
}

// 0xf4ca04 — j___ZN3rbx8any_castIN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector3 rbx::any_cast<G3D::Vector3,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector3 rbx::any_cast<G3D::Vector3,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xf4ca04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ca04() {
}

// 0xf4cb04 — j___ZN3rbx8any_castIRN3G3D12Vector2int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector2int16 & rbx::any_cast<G3D::Vector2int16 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector2int16 & rbx::any_cast<G3D::Vector2int16 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xf4cb04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4cb04() {
}

// 0xf4cb14 — j___ZN3rbx8any_castIRN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::CoordinateFrame & rbx::any_cast<G3D::CoordinateFrame &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::CoordinateFrame & rbx::any_cast<G3D::CoordinateFrame &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xf4cb14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4cb14() {
}

// 0xf4cb24 — j___ZN3rbx8any_castIRN3G3D6Color3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Color3 & rbx::any_cast<G3D::Color3 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Color3 & rbx::any_cast<G3D::Color3 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xf4cb24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4cb24() {
}

// 0xf4cb34 — j___ZN3rbx8any_castIRN3G3D7Vector2EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector2 & rbx::any_cast<G3D::Vector2 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector2 & rbx::any_cast<G3D::Vector2 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xf4cb34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4cb34() {
}

// 0xf4cb44 — j___ZN3rbx8any_castIRN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector3 & rbx::any_cast<G3D::Vector3 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector3 & rbx::any_cast<G3D::Vector3 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xf4cb44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4cb44() {
}

// 0xf4d9e4 — j___ZN3G3D5ArrayIPKN3RBX5JointELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::Joint const*,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::Joint const*,10,32ul>::Array(void)
// IDA 0xf4d9e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4d9e4() {
}

// 0xf4d9f4 — j___ZN3G3D5ArrayIPKN3RBX5JointELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::Joint const*,10,32ul>::~Array()")]
// was: G3D::Array<RBX::Joint const*,10,32ul>::~Array()
// IDA 0xf4d9f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4d9f4() {
}

// 0xf4da04 — j___ZN3G3D5ArrayIPN3RBX4EdgeELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::Edge *,10,32ul>::append(RBX::Edge * const&)")]
// was: G3D::Array<RBX::Edge *,10,32ul>::append(RBX::Edge * const&)
// IDA 0xf4da04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4da04() {
}

// 0xf4da14 — j___ZN3G3D5ArrayIPN3RBX4EdgeELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::Edge *,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::Edge *,10,32ul>::resize(int,bool)
// IDA 0xf4da14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4da14() {
}

// 0xf4da24 — j___ZN3G3D5ArrayIPN3RBX4EdgeELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::Edge *,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::Edge *,10,32ul>::realloc(int)
// IDA 0xf4da24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4da24() {
}

// 0xf4da34 — j___ZN3G3D5ArrayIPN3RBX4EdgeELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::Edge *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::Edge *,10,32ul>::Array(void)
// IDA 0xf4da34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4da34() {
}

// 0xf4da44 — j___ZN3G3D5ArrayIPN3RBX4EdgeELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::Edge *,10,32ul>::~Array()")]
// was: G3D::Array<RBX::Edge *,10,32ul>::~Array()
// IDA 0xf4da44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4da44() {
}

// 0xf4da54 — j___ZN3G3D5ArrayIPN3RBX5JointELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::Joint *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::Joint *,10,32ul>::Array(void)
// IDA 0xf4da54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4da54() {
}

// 0xf4da64 — j___ZN3G3D5ArrayIPN3RBX5JointELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::Joint *,10,32ul>::~Array()")]
// was: G3D::Array<RBX::Joint *,10,32ul>::~Array()
// IDA 0xf4da64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4da64() {
}

// 0xf4da84 — j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERKN3G3D7Vector3ERfENS3_5list3INS2_3argILi1EEENS2_17reference_wrapperIS9_EENSH_IfEEEEEEEEvT_S6_
#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,G3D::Vector3 const&,float &),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<G3D::Vector3 const>,boost::reference_wrapper<float>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,G3D::Vector3 const&,float &),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<G3D::Vector3 const>,boost::reference_wrapper<float>>>,RBX::Primitive *)")]
// was: void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,G3D::Vector3 const&,float &),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<G3D::Vector3 const>,boost::reference_wrapper<float>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,G3D::Vector3 const&,float &),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<G3D::Vector3 const>,boost::reference_wrapper<float>>>,RBX::Primitive *)
// IDA 0xf4da84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4da84() {
}

// 0xf4dc74 — j___ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY12BlockCornersENS_15Vector3ComparerEE10ValueCountC2ERKS2_
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount::ValueCount(G3D::Vector3 const&)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount::ValueCount(G3D::Vector3 const&)
// IDA 0xf4dc74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dc74() {
}

// 0xf4dc84 — j___ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY12BlockCornersENS_15Vector3ComparerEE10ValueCountD2Ev
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount::~ValueCount()")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount::~ValueCount()
// IDA 0xf4dc84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4dc84() {
}

// 0xf4dc94 — j___ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY12BlockCornersENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *)
// IDA 0xf4dc94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dc94() {
}

// 0xf4dca4 — j___ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY12BlockCornersENS_15Vector3ComparerEE29safe_static_do_get_staticDataEv
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)
// IDA 0xf4dca4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dca4() {
}

// 0xf4dcb4 — j___ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY12BlockCornersENS_15Vector3ComparerEE8getTokenERKS2_
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)
// IDA 0xf4dcb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dcb4() {
}

// 0xf4dcc4 — j___ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE10ValueCountC2ERKS2_
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount::ValueCount(G3D::Vector3 const&)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount::ValueCount(G3D::Vector3 const&)
// IDA 0xf4dcc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dcc4() {
}

// 0xf4dcd4 — j___ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE10ValueCountD2Ev
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount::~ValueCount()")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount::~ValueCount()
// IDA 0xf4dcd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4dcd4() {
}

// 0xf4dce4 — j___ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *)
// IDA 0xf4dce4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dce4() {
}

// 0xf4dcf4 — j___ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE8getTokenERKS2_
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)
// IDA 0xf4dcf4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dcf4() {
}

// 0xf4dd04 — j___ZN3RBX4POLY12BlockCornersC2ERKN3G3D7Vector3E
#[doc(alias = "RBX::POLY::BlockCorners::BlockCorners(G3D::Vector3 const&)")]
// was: RBX::POLY::BlockCorners::BlockCorners(G3D::Vector3 const&)
// IDA 0xf4dd04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dd04() {
}

// 0xf4dd14 — j___ZN3RBX4POLY9BlockMeshC2ERKN3G3D7Vector3E
#[doc(alias = "RBX::POLY::BlockMesh::BlockMesh(G3D::Vector3 const&)")]
// was: RBX::POLY::BlockMesh::BlockMesh(G3D::Vector3 const&)
// IDA 0xf4dd14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dd14() {
}

// 0xf4dd94 — j___ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY12BlockCornersENS1_15Vector3ComparerEE5TokenEEC2IS9_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token *)")]
// was: boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token *)
// IDA 0xf4dd94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dd94() {
}

// 0xf4dda4 — j___ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY12BlockCornersENS1_15Vector3ComparerEE5TokenEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token> const&)")]
// was: boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::operator=(boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token> const&)
// IDA 0xf4dda4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dda4() {
}

// 0xf4ddb4 — j___ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY9BlockMeshENS1_15Vector3ComparerEE5TokenEEC2IS9_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token *)")]
// was: boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token *)
// IDA 0xf4ddb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ddb4() {
}

// 0xf4ddc4 — j___ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY9BlockMeshENS1_15Vector3ComparerEE5TokenEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token> const&)")]
// was: boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::operator=(boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token> const&)
// IDA 0xf4ddc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ddc4() {
}

// 0xf4de14 — j___ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolIN3G3D7Vector3ENS3_4POLY12BlockCornersENS3_15Vector3ComparerEE5TokenEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token *)")]
// was: boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token *)
// IDA 0xf4de14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4de14() {
}

// 0xf4de24 — j___ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolIN3G3D7Vector3ENS3_4POLY9BlockMeshENS3_15Vector3ComparerEE5TokenEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token *)")]
// was: boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token *)
// IDA 0xf4de24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4de24() {
}

// 0xf4de34 — j___ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY12BlockCornersENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_
#[doc(alias = "std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)")]
// was: std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)
// IDA 0xf4de34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4de34() {
}

// 0xf4de44 — j___ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY9BlockMeshENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_
#[doc(alias = "std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)")]
// was: std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)
// IDA 0xf4de44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4de44() {
}

// 0xf4de74 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11lower_boundERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)
// IDA 0xf4de74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4de74() {
}

// 0xf4de84 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11upper_boundERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)
// IDA 0xf4de84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4de84() {
}

// 0xf4de94 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueERKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *> const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *> const&)
// IDA 0xf4de94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4de94() {
}

// 0xf4dea4 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *> const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *> const&)
// IDA 0xf4dea4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dea4() {
}

// 0xf4deb4 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE4findERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)
// IDA 0xf4deb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4deb4() {
}

// 0xf4dec4 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)
// IDA 0xf4dec4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dec4() {
}

// 0xf4ded4 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>)
// IDA 0xf4ded4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ded4() {
}

// 0xf4dee4 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>> *)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>> *)
// IDA 0xf4dee4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4dee4() {
}

// 0xf4def4 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *> const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *> const&)
// IDA 0xf4def4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4def4() {
}

// 0xf4df04 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11lower_boundERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)
// IDA 0xf4df04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4df04() {
}

// 0xf4df14 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11upper_boundERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)
// IDA 0xf4df14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4df14() {
}

// 0xf4df24 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueERKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *> const&)
// IDA 0xf4df24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4df24() {
}

// 0xf4df34 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *> const&)
// IDA 0xf4df34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4df34() {
}

// 0xf4df44 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE4findERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)
// IDA 0xf4df44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4df44() {
}

// 0xf4df54 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)
// IDA 0xf4df54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4df54() {
}

// 0xf4df64 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>)
// IDA 0xf4df64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4df64() {
}

// 0xf4df74 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *> const&)
// IDA 0xf4df74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4df74() {
}
