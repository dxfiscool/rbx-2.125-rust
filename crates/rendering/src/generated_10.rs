//! rendering shard B continuation — next 120 Gfx|G3D stubs EA-sorted
//! Filter: Gfx|G3D low-EA window 0xB740..0x2A4338 already fully stubbed (104/104 in generated_05 + generated + next_batch)
//! This shard: next 120 unstubbed Gfx|G3D EA-sorted continuation 0x3cbaf0..0x52d8e0 (4084 total, 2324 prior stubbed, 120 this batch, 3348 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x3cbaf0 — __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
#[doc(alias = "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>*,std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>>,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)")]
// was: std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>*,std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>>,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)
// IDA 0x3cbaf0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_3cbaf0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x3cbe40 — __ZNSt12_Vector_baseISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::_M_allocate(unsigned long)
// IDA 0x3cbe40: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_3cbe40() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x3cbe64 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairIN3G3D15CoordinateFrameES5_ES7_EET0_T_S9_S8_
#[doc(alias = "std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *>(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *)")]
// was: std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *>(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *)
// IDA 0x3cbe64: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_3cbe64() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x3ce4dc — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EEC2EMS2_FvS4_S4_fEPKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::BoundFuncDesc(void (RBX::Camera::*)(G3D::CoordinateFrame,G3D::CoordinateFrame,float),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::BoundFuncDesc(void (RBX::Camera::*)(G3D::CoordinateFrame,G3D::CoordinateFrame,float),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0x3ce4dc: 213 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ce4dc() {
}

// 0x3ce6f4 — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EE16declareSignatureEPKcNS0_7VariantES8_S9_S8_S9_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
// IDA 0x3ce6f4: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ce6f4() {
}

// 0x3ce75c — __ZN3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::~BoundFuncDesc()
// IDA 0x3ce75c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ce75c() {
}

// 0x3ce848 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvN3G3D15CoordinateFrameES4_fELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(G3D::CoordinateFrame,G3D::CoordinateFrame,float),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// IDA 0x3ce848: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ce848() {
}

// 0x3ce94c — __ZN3RBX10Reflection11Call3HelperINS_6CameraEMS2_FvN3G3D15CoordinateFrameES4_fES4_S4_fvE4callEPS2_S6_RNS0_7VariantERKS4_SC_RKf
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Camera,void (RBX::Camera::*)(G3D::CoordinateFrame,G3D::CoordinateFrame,float),G3D::CoordinateFrame,G3D::CoordinateFrame,float,void>::call(RBX::Camera*,void (RBX::Camera::*)(G3D::CoordinateFrame,G3D::CoordinateFrame,float),RBX::Reflection::Variant &,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float const&)")]
// was: RBX::Reflection::Call3Helper<RBX::Camera,void (RBX::Camera::*)(G3D::CoordinateFrame,G3D::CoordinateFrame,float),G3D::CoordinateFrame,G3D::CoordinateFrame,float,void>::call(RBX::Camera*,void (RBX::Camera::*)(G3D::CoordinateFrame,G3D::CoordinateFrame,float),RBX::Reflection::Variant &,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float const&)
// IDA 0x3ce94c: 86 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ce94c() {
}

// 0x3cea40 — __ZN3RBX10Reflection9ArgHelper6getArgIN3G3D15CoordinateFrameELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "G3D::CoordinateFrame RBX::Reflection::ArgHelper::getArg<G3D::CoordinateFrame,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::CoordinateFrame> const&,boost::disable_if<boost::is_same<G3D::CoordinateFrame,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: G3D::CoordinateFrame RBX::Reflection::ArgHelper::getArg<G3D::CoordinateFrame,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::CoordinateFrame> const&,boost::disable_if<boost::is_same<G3D::CoordinateFrame,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
// IDA 0x3cea40: 176 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cea40() {
}

// 0x3cec10 — __ZN3RBX10Reflection9ArgHelper6getArgIN3G3D15CoordinateFrameELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "G3D::CoordinateFrame RBX::Reflection::ArgHelper::getArg<G3D::CoordinateFrame,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::CoordinateFrame> const&,boost::disable_if<boost::is_same<G3D::CoordinateFrame,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: G3D::CoordinateFrame RBX::Reflection::ArgHelper::getArg<G3D::CoordinateFrame,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::CoordinateFrame> const&,boost::disable_if<boost::is_same<G3D::CoordinateFrame,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
// IDA 0x3cec10: 177 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3cec10() {
}

// 0x3d0bc4 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x3d0bc4: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0bc4() {
}

// 0x3d0cd8 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::~PropDescriptor()
// IDA 0x3d0cd8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d0cd8() {
}

// 0x3d0d04 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::isReadOnly(void)const
// IDA 0x3d0d04: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0d04() {
}

// 0x3d0d08 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::isWriteOnly(void)const
// IDA 0x3d0d08: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0d08() {
}

// 0x3d0d0c — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x3d0d0c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0d0c() {
}

// 0x3d0d48 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const
// IDA 0x3d0d48: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d0d48() {
}

// 0x3d362c — __ZN3RBX20ChangeHistoryService7setCellERKN3G3D12Vector3int16ES4_NS_5Voxel4CellENS5_12CellMaterialE
#[doc(alias = "RBX::ChangeHistoryService::setCell(G3D::Vector3int16 const&,G3D::Vector3int16 const&,RBX::Voxel::Cell,RBX::Voxel::CellMaterial)")]
// was: RBX::ChangeHistoryService::setCell(G3D::Vector3int16 const&,G3D::Vector3int16 const&,RBX::Voxel::Cell,RBX::Voxel::CellMaterial)
// IDA 0x3d362c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d362c() {
}

// 0x3faac8 — __ZN3RBX17RotateAxisCommand15rotateAboutAxisERKN3G3D7Matrix3ERKSt6vectorIPNS_10PVInstanceESaIS7_EE
#[doc(alias = "RBX::RotateAxisCommand::rotateAboutAxis(G3D::Matrix3 const&,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&)")]
// was: RBX::RotateAxisCommand::rotateAboutAxis(G3D::Matrix3 const&,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&)
// IDA 0x3faac8: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3faac8() {
}

// 0x418e28 — __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEC2IN3G3D7Vector3EEET_
#[doc(alias = "__ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEC2IN3G3D7Vector3EEET_")]
// was: __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEC2IN3G3D7Vector3EEET_
// IDA 0x418e28: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_418e28() {
}

// 0x46f710 — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::~PropDescriptor()
// IDA 0x46f710: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_46f710() {
}

// 0x46f768 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::~TypedPropertyDescriptor()
// IDA 0x46f768: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_46f768() {
}

// 0x46f798 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x46f798: 134 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46f798() {
}

// 0x46f8fc — __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EEC2IMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x46f8fc: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46f8fc() {
}

// 0x46fa10 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x46fa10: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46fa10() {
}

// 0x46fb60 — __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::isReadOnly(void)const
// IDA 0x46fb60: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46fb60() {
}

// 0x46fb64 — __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::isWriteOnly(void)const
// IDA 0x46fb64: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46fb64() {
}

// 0x46fb68 — __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x46fb68: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46fb68() {
}

// 0x46fba0 — __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const
// IDA 0x46fba0: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46fba0() {
}

// 0x46fbd4 — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EEC2IMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x46fbd4: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46fbd4() {
}

// 0x46fce8 — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::~PropDescriptor()
// IDA 0x46fce8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_46fce8() {
}

// 0x46fd14 — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::isReadOnly(void)const
// IDA 0x46fd14: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46fd14() {
}

// 0x46fd18 — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::isWriteOnly(void)const
// IDA 0x46fd18: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46fd18() {
}

// 0x46fd1c — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x46fd1c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46fd1c() {
}

// 0x46fd54 — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EE10GetSetImplIMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const
// IDA 0x46fd54: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_46fd54() {
}

// 0x475108 — __ZN3RBX13DataModelMesh8setScaleERKN3G3D7Vector3E
#[doc(alias = "RBX::DataModelMesh::setScale(G3D::Vector3 const&)")]
// was: RBX::DataModelMesh::setScale(G3D::Vector3 const&)
// IDA 0x475108: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475108() {
}

// 0x4751a8 — __ZN3RBX13DataModelMesh12setVertColorERKN3G3D7Vector3E
#[doc(alias = "RBX::DataModelMesh::setVertColor(G3D::Vector3 const&)")]
// was: RBX::DataModelMesh::setVertColor(G3D::Vector3 const&)
// IDA 0x4751a8: 31 insns (VLDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4751a8() {
}

// 0x475210 — __ZN3RBX13DataModelMesh9setOffsetERKN3G3D7Vector3E
#[doc(alias = "RBX::DataModelMesh::setOffset(G3D::Vector3 const&)")]
// was: RBX::DataModelMesh::setOffset(G3D::Vector3 const&)
// IDA 0x475210: 31 insns (VLDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475210() {
}

// 0x475878 — __ZN3RBX10Reflection14PropDescriptorINS_13DataModelMeshEN3G3D7Vector3EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::~PropDescriptor()
// IDA 0x475878: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_475878() {
}

// 0x475dd0 — __ZN3RBX10Reflection14PropDescriptorINS_13DataModelMeshEN3G3D7Vector3EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::PropDescriptor<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::PropDescriptor<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x475dd0: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475dd0() {
}

// 0x475ee4 — __ZN3RBX10Reflection14PropDescriptorINS_13DataModelMeshEN3G3D7Vector3EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::~PropDescriptor()
// IDA 0x475ee4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_475ee4() {
}

// 0x475f10 — __ZNK3RBX10Reflection14PropDescriptorINS_13DataModelMeshEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>::isReadOnly(void)const
// IDA 0x475f10: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475f10() {
}

// 0x475f14 — __ZNK3RBX10Reflection14PropDescriptorINS_13DataModelMeshEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>::isWriteOnly(void)const
// IDA 0x475f14: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475f14() {
}

// 0x475f18 — __ZNK3RBX10Reflection14PropDescriptorINS_13DataModelMeshEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x475f18: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475f18() {
}

// 0x475f4c — __ZNK3RBX10Reflection14PropDescriptorINS_13DataModelMeshEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const
// IDA 0x475f4c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475f4c() {
}

// 0x47b6d8 — __ZNK3RBX13DebugSettings7gfxcardEv
#[doc(alias = "RBX::DebugSettings::gfxcard(void)const")]
// was: RBX::DebugSettings::gfxcard(void)const
// IDA 0x47b6d8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b6d8() {
}

// 0x47e5f8 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE7getSizeEv
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::getSize(void)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::getSize(void)
// IDA 0x47e5f8: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47e5f8() {
}

// 0x483eb0 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE27safe_static_init_staticDataEv
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::safe_static_init_staticData(void)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::safe_static_init_staticData(void)
// IDA 0x483eb0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_483eb0() {
}

// 0x483eb4 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE29safe_static_do_get_staticDataEv
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)
// IDA 0x483eb4: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483eb4() {
}

// 0x483fc4 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE10StaticDataD1Ev
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::StaticData::~StaticData()")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::StaticData::~StaticData()
// IDA 0x483fc4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_483fc4() {
}

// 0x484074 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>> *)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>> *)
// IDA 0x484074: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484074() {
}

// 0x4a0468 — __ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EED1Ev
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// was: RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::~BoundProp()
// IDA 0x4a0468: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a0468() {
}

// 0x4a60bc — __ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS9_MT_S3_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,G3D::Vector3 RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,G3D::Vector3 RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x4a60bc: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a60bc() {
}

// 0x4a6250 — __ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// was: RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::~BoundProp()
// IDA 0x4a6250: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a6250() {
}

// 0x4a6280 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x4a6280: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6280() {
}

// 0x4a62b0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x4a62b0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a62b0() {
}

// 0x4a62d8 — __ZN3rbx8any_castIRKN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector3 const& rbx::any_cast<G3D::Vector3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector3 const& rbx::any_cast<G3D::Vector3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4a62d8: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a62d8() {
}

// 0x4a63c8 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::~TypedPropertyDescriptor()
// IDA 0x4a63c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a63c8() {
}

// 0x4a63f4 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isReadOnly(void)const")]
// was: RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isReadOnly(void)const
// IDA 0x4a63f4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a63f4() {
}

// 0x4a63f8 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isWriteOnly(void)const")]
// was: RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isWriteOnly(void)const
// IDA 0x4a63f8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a63f8() {
}

// 0x4a63fc — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x4a63fc: 11 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a63fc() {
}

// 0x4a6418 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
// was: RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const
// IDA 0x4a6418: 52 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a6418() {
}

// 0x4a72a8 — __ZN3RBX20ExtrudedPartInstance14setPartSizeXmlERKN3G3D7Vector3E
#[doc(alias = "RBX::ExtrudedPartInstance::setPartSizeXml(G3D::Vector3 const&)")]
// was: RBX::ExtrudedPartInstance::setPartSizeXml(G3D::Vector3 const&)
// IDA 0x4a72a8: 193 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a72a8() {
}

// 0x4a86f4 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEC2IN3G3D7Vector3EEET_
#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEC2IN3G3D7Vector3EEET_")]
// was: __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEC2IN3G3D7Vector3EEET_
// IDA 0x4a86f4: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a86f4() {
}

// 0x4aaf60 — __ZN3RBX10Reflection4Type12getSingletonIN3G3D7Vector34AxisEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<G3D::Vector3::Axis>(void)")]
// was: RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<G3D::Vector3::Axis>(void)
// IDA 0x4aaf60: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aaf60() {
}

// 0x4caf94 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescIN3G3D7Vector34AxisEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<G3D::Vector3::Axis> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<G3D::Vector3::Axis> const>::initSingleton(void)
// IDA 0x4caf94: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4caf94() {
}

// 0x4caf98 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescIN3G3D7Vector34AxisEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<G3D::Vector3::Axis> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<G3D::Vector3::Axis> const>::doGetSingleton(void)
// IDA 0x4caf98: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4caf98() {
}

// 0x4cb088 — __ZN3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::~EnumDesc()
// IDA 0x4cb088: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4cb088() {
}

// 0x4cb08c — __ZN3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::~EnumDesc()
// IDA 0x4cb08c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4cb08c() {
}

// 0x4cb260 — __ZN3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::~EnumDesc()
// IDA 0x4cb260: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4cb260() {
}

// 0x4cb300 — __ZNK3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::lookup(char const*)const
// IDA 0x4cb300: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb300() {
}

// 0x4cb330 — __ZNK3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4cb330: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb330() {
}

// 0x4cb350 — __ZNK3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4cb350: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb350() {
}

// 0x4cb3ac — __ZNK3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::convertToString(unsigned long,std::string &)const
// IDA 0x4cb3ac: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb3ac() {
}

// 0x4cb4f0 — __ZNK3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEE15convertToStringERKS4_
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::convertToString(G3D::Vector3::Axis const&)const")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::convertToString(G3D::Vector3::Axis const&)const
// IDA 0x4cb4f0: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb4f0() {
}

// 0x4cb690 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D7Vector34AxisEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3::Axis>(G3D::Vector3::Axis const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3::Axis>(G3D::Vector3::Axis const&)
// IDA 0x4cb690: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb690() {
}

// 0x4cb6e0 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector34AxisEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3::Axis>::singleton(void)")]
// was: rbx::implementation::typed_holder<G3D::Vector3::Axis>::singleton(void)
// IDA 0x4cb6e0: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb6e0() {
}

// 0x4cb74c — __ZN3rbx14implementation12typed_holderIN3G3D7Vector34AxisEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3::Axis>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<G3D::Vector3::Axis>::construct_func(char const*,char *)
// IDA 0x4cb74c: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb74c() {
}

// 0x4cb758 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector34AxisEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3::Axis>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<G3D::Vector3::Axis>::destruct_func(char *)
// IDA 0x4cb758: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4cb758() {
}

// 0x4cb75c — __ZNK3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEE13convertToItemERKS4_
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::convertToItem(G3D::Vector3::Axis const&)const")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::convertToItem(G3D::Vector3::Axis const&)const
// IDA 0x4cb75c: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb75c() {
}

// 0x4cb828 — __ZN3rbx8any_castIRKN3G3D7Vector34AxisEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector3::Axis const& rbx::any_cast<G3D::Vector3::Axis const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector3::Axis const& rbx::any_cast<G3D::Vector3::Axis const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4cb828: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb828() {
}

// 0x4cb918 — __ZNK3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEE14convertToValueERKNS_4NameERS4_
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::convertToValue(RBX::Name const&,G3D::Vector3::Axis&)const")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::convertToValue(RBX::Name const&,G3D::Vector3::Axis&)const
// IDA 0x4cb918: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb918() {
}

// 0x4cb994 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,G3D::Vector3::Axis>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,G3D::Vector3::Axis>> *)
// IDA 0x4cb994: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb994() {
}

// 0x4ddcb8 — __ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEELZNS_12sColor3ValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_12sColor3ValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEELZNS_12sColor3ValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_12sColor3ValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEELZNS_12sColor3ValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_12sColor3ValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x4ddcb8: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ddcb8() {
}

// 0x4dddd8 — __ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEELZNS_12sCFrameValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_12sCFrameValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEELZNS_12sCFrameValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_12sCFrameValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEELZNS_12sCFrameValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_12sCFrameValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x4dddd8: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4dddd8() {
}

// 0x4de018 — __ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEELZNS_13sVector3ValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_13sVector3ValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEELZNS_13sVector3ValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_13sVector3ValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEELZNS_13sVector3ValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_13sVector3ValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x4de018: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4de018() {
}

// 0x4e52b8 — __ZNK3RBX7Feature14getRenderCoordERN3G3D15CoordinateFrameE
#[doc(alias = "RBX::Feature::getRenderCoord(G3D::CoordinateFrame &)const")]
// was: RBX::Feature::getRenderCoord(G3D::CoordinateFrame &)const
// IDA 0x4e52b8: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4e52b8() {
}

// 0x4ef6f0 — __ZN3RBX4Fire8setColorEN3G3D6Color3E
#[doc(alias = "RBX::Fire::setColor(G3D::Color3)")]
// was: RBX::Fire::setColor(G3D::Color3)
// IDA 0x4ef6f0: 31 insns (VLDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef6f0() {
}

// 0x4ef758 — __ZN3RBX4Fire17setSecondaryColorEN3G3D6Color3E
#[doc(alias = "RBX::Fire::setSecondaryColor(G3D::Color3)")]
// was: RBX::Fire::setSecondaryColor(G3D::Color3)
// IDA 0x4ef758: 31 insns (VLDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ef758() {
}

// 0x4efcd4 — __ZN3RBX10Reflection14PropDescriptorINS_4FireEN3G3D6Color3EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::~PropDescriptor()
// IDA 0x4efcd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4efcd4() {
}

// 0x4f0cd0 — __ZN3RBX10Reflection14PropDescriptorINS_4FireEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Fire::*)(void)const,void (RBX::Fire::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Fire::*)(void)const,void (RBX::Fire::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Fire::*)(void)const,void (RBX::Fire::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Fire::*)(void)const,void (RBX::Fire::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x4f0cd0: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f0cd0() {
}

// 0x4f0de4 — __ZN3RBX10Reflection14PropDescriptorINS_4FireEN3G3D6Color3EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::~PropDescriptor()
// IDA 0x4f0de4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f0de4() {
}

// 0x4f0e10 — __ZNK3RBX10Reflection14PropDescriptorINS_4FireEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Fire::*)(void)const,void (RBX::Fire::*)(G3D::Color3)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Fire::*)(void)const,void (RBX::Fire::*)(G3D::Color3)>::isReadOnly(void)const
// IDA 0x4f0e10: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f0e10() {
}

// 0x4f0e14 — __ZNK3RBX10Reflection14PropDescriptorINS_4FireEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Fire::*)(void)const,void (RBX::Fire::*)(G3D::Color3)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Fire::*)(void)const,void (RBX::Fire::*)(G3D::Color3)>::isWriteOnly(void)const
// IDA 0x4f0e14: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f0e14() {
}

// 0x4f0e18 — __ZNK3RBX10Reflection14PropDescriptorINS_4FireEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Fire::*)(void)const,void (RBX::Fire::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Fire::*)(void)const,void (RBX::Fire::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x4f0e18: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f0e18() {
}

// 0x4f0e40 — __ZNK3RBX10Reflection14PropDescriptorINS_4FireEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Fire::*)(void)const,void (RBX::Fire::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Fire,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Fire::*)(void)const,void (RBX::Fire::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const
// IDA 0x4f0e40: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f0e40() {
}

// 0x505188 — __ZN3RBX15GeometryService33getPartsTouchingExtentsWithIgnoreERKNS_7ExtentsEPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS8_EEiRN3G3D5ArrayIPNS_12PartInstanceELi10ELm32EEE
#[doc(alias = "RBX::GeometryService::getPartsTouchingExtentsWithIgnore(RBX::Extents const&,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const*,int,G3D::Array<RBX::PartInstance *,10,32ul> &)")]
// was: RBX::GeometryService::getPartsTouchingExtentsWithIgnore(RBX::Extents const&,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const*,int,G3D::Array<RBX::PartInstance *,10,32ul> &)
// IDA 0x505188: 257 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_505188() {
}

// 0x50546c — __ZN3RBX15GeometryService23getPartsTouchingExtentsERKNS_7ExtentsEPKNS_9PrimitiveEiRN3G3D5ArrayIPNS_12PartInstanceELi10ELm32EEE
#[doc(alias = "RBX::GeometryService::getPartsTouchingExtents(RBX::Extents const&,RBX::Primitive const*,int,G3D::Array<RBX::PartInstance *,10,32ul> &)")]
// was: RBX::GeometryService::getPartsTouchingExtents(RBX::Extents const&,RBX::Primitive const*,int,G3D::Array<RBX::PartInstance *,10,32ul> &)
// IDA 0x50546c: 72 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50546c() {
}

// 0x505b70 — __ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::append(RBX::PartInstance * const&)")]
// was: G3D::Array<RBX::PartInstance *,10,32ul>::append(RBX::PartInstance * const&)
// IDA 0x505b70: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_505b70() {
}

// 0x505bcc — __ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsINS_8InstanceEEEN3G3D7Vector3EPT_NS_6RbxRayERN5boost10shared_ptrINS_12PartInstanceEEERNS_6CellIDEb
#[doc(alias = "G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<RBX::Instance>(RBX::Instance *,RBX::RbxRay,rbx_core::SharedPtr<RBX::PartInstance> &,RBX::CellID &,bool)")]
// was: G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<RBX::Instance>(RBX::Instance *,RBX::RbxRay,boost::shared_ptr<RBX::PartInstance> &,RBX::CellID &,bool)
// IDA 0x505bcc: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_505bcc() {
}

// 0x505d08 — __ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsIKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS6_EEEEN3G3D7Vector3EPT_NS_6RbxRayERNS4_INS_12PartInstanceEEERNS_6CellIDEb
#[doc(alias = "G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const *,RBX::RbxRay,rbx_core::SharedPtr<RBX::PartInstance> &,RBX::CellID &,bool)")]
// was: G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const *,RBX::RbxRay,boost::shared_ptr<RBX::PartInstance> &,RBX::CellID &,bool)
// IDA 0x505d08: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_505d08() {
}

// 0x506524 — __ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::PartInstance *,10,32ul>::resize(int,bool)
// IDA 0x506524: 59 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_506524() {
}

// 0x5065dc — __ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::PartInstance *,10,32ul>::realloc(int)
// IDA 0x5065dc: 147 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5065dc() {
}

// 0x506fa8 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::Primitive *,10,32ul>::Array(void)
// IDA 0x506fa8: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_506fa8() {
}

// 0x526444 — __ZN3RBX9GuiObject15setBorderColor3EN3G3D6Color3E
#[doc(alias = "RBX::GuiObject::setBorderColor3(G3D::Color3)")]
// was: RBX::GuiObject::setBorderColor3(G3D::Color3)
// IDA 0x526444: 39 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_526444() {
}

// 0x5264e4 — __ZN3RBX9GuiObject19setBackgroundColor3EN3G3D6Color3E
#[doc(alias = "RBX::GuiObject::setBackgroundColor3(G3D::Color3)")]
// was: RBX::GuiObject::setBackgroundColor3(G3D::Color3)
// IDA 0x5264e4: 39 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5264e4() {
}

// 0x5293fc — __ZN3RBX9GuiObject12handleResizeERKN3G3D6Rect2DEb
#[doc(alias = "RBX::GuiObject::handleResize(G3D::Rect2D const&,bool)")]
// was: RBX::GuiObject::handleResize(G3D::Rect2D const&,bool)
// IDA 0x5293fc: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5293fc() {
}

// 0x529438 — __ZN3RBX9GuiObject10handleDragEN3G3D7Vector2E
#[doc(alias = "RBX::GuiObject::handleDrag(G3D::Vector2)")]
// was: RBX::GuiObject::handleDrag(G3D::Vector2)
// IDA 0x529438: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_529438() {
}

// 0x5294c8 — __ZN3RBX9GuiObject34recalculateAbsoluteSizeAndPositionERKN3G3D6Rect2DE
#[doc(alias = "RBX::GuiObject::recalculateAbsoluteSizeAndPosition(G3D::Rect2D const&)")]
// was: RBX::GuiObject::recalculateAbsoluteSizeAndPosition(G3D::Rect2D const&)
// IDA 0x5294c8: 117 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5294c8() {
}

// 0x5298b0 — __ZN3RBX9GuiObject14legacyRender2dEPNS_5AdornERKN3G3D6Rect2DE
#[doc(alias = "RBX::GuiObject::legacyRender2d(RBX::Adorn *,G3D::Rect2D const&)")]
// was: RBX::GuiObject::legacyRender2d(RBX::Adorn *,G3D::Rect2D const&)
// IDA 0x5298b0: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5298b0() {
}

// 0x529948 — __ZN3RBX9GuiObject12render2dImplEPNS_5AdornERKN3G3D6Color4E
#[doc(alias = "RBX::GuiObject::render2dImpl(RBX::Adorn *,G3D::Color4 const&)")]
// was: RBX::GuiObject::render2dImpl(RBX::Adorn *,G3D::Color4 const&)
// IDA 0x529948: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_529948() {
}

// 0x529968 — __ZN3RBX9GuiObject12render2dImplEPNS_5AdornERKN3G3D6Color4ERNS3_6Rect2DE
#[doc(alias = "RBX::GuiObject::render2dImpl(RBX::Adorn *,G3D::Color4 const&,G3D::Rect2D &)")]
// was: RBX::GuiObject::render2dImpl(RBX::Adorn *,G3D::Color4 const&,G3D::Rect2D &)
// IDA 0x529968: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_529968() {
}

// 0x529b14 — __ZN3RBX9GuiObject12Scale9Rect2DERKN3G3D6Rect2DEff
#[doc(alias = "RBX::GuiObject::Scale9Rect2D(G3D::Rect2D const&,float,float)")]
// was: RBX::GuiObject::Scale9Rect2D(G3D::Rect2D const&,float,float)
// IDA 0x529b14: 53 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_529b14() {
}

// 0x529bd4 — __ZN3RBX9GuiObject18render2dScale9ImplEPNS_5AdornERKNS_9TextureIdERKN3G3D12Vector2int16ERKNS6_7Vector2ERNS_12GuiDrawImageERNS6_6Rect2DEPS0_
#[doc(alias = "RBX::GuiObject::render2dScale9Impl(RBX::Adorn *,RBX::TextureId const&,G3D::Vector2int16 const&,G3D::Vector2 const&,RBX::GuiDrawImage &,G3D::Rect2D &,RBX::GuiObject*)")]
// was: RBX::GuiObject::render2dScale9Impl(RBX::Adorn *,RBX::TextureId const&,G3D::Vector2int16 const&,G3D::Vector2 const&,RBX::GuiDrawImage &,G3D::Rect2D &,RBX::GuiObject*)
// IDA 0x529bd4: 219 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_529bd4() {
}

// 0x529eac — __ZN3RBX9GuiObject16render2dTextImplEPNS_5AdornERKN3G3D6Color4ERKSsNS_11TextService4FontENS9_8FontSizeES6_S6_bbNS9_10XAlignmentENS9_10YAlignmentE
#[doc(alias = "RBX::GuiObject::render2dTextImpl(RBX::Adorn *,G3D::Color4 const&,std::string const&,RBX::TextService::Font,RBX::TextService::FontSize,G3D::Color4 const&,G3D::Color4 const&,bool,bool,RBX::TextService::XAlignment,RBX::TextService::YAlignment)")]
// was: RBX::GuiObject::render2dTextImpl(RBX::Adorn *,G3D::Color4 const&,std::string const&,RBX::TextService::Font,RBX::TextService::FontSize,G3D::Color4 const&,G3D::Color4 const&,bool,bool,RBX::TextService::XAlignment,RBX::TextService::YAlignment)
// IDA 0x529eac: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_529eac() {
}

// 0x529f10 — __ZN3RBX9GuiObject16render2dTextImplEPNS_5AdornERKN3G3D6Rect2DERKSsNS_11TextService4FontENS9_8FontSizeERKNS3_6Color4ESE_bbNS9_10XAlignmentENS9_10YAlignmentE
#[doc(alias = "RBX::GuiObject::render2dTextImpl(RBX::Adorn *,G3D::Rect2D const&,std::string const&,RBX::TextService::Font,RBX::TextService::FontSize,G3D::Color4 const&,G3D::Color4 const&,bool,bool,RBX::TextService::XAlignment,RBX::TextService::YAlignment)")]
// was: RBX::GuiObject::render2dTextImpl(RBX::Adorn *,G3D::Rect2D const&,std::string const&,RBX::TextService::Font,RBX::TextService::FontSize,G3D::Color4 const&,G3D::Color4 const&,bool,bool,RBX::TextService::XAlignment,RBX::TextService::YAlignment)
// IDA 0x529f10: 204 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_529f10() {
}

// 0x52a12c — __ZN3RBX9GuiObject17getScaledFontSizeERKN3G3D6Rect2DERKSsNS_11TextService4FontEbf
#[doc(alias = "RBX::GuiObject::getScaledFontSize(G3D::Rect2D const&,std::string const&,RBX::TextService::Font,bool,float)")]
// was: RBX::GuiObject::getScaledFontSize(G3D::Rect2D const&,std::string const&,RBX::TextService::Font,bool,float)
// IDA 0x52a12c: 153 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52a12c() {
}

// 0x52a334 — __ZN3RBX9GuiObject11mouseIsOverERKN3G3D7Vector2E
#[doc(alias = "RBX::GuiObject::mouseIsOver(G3D::Vector2 const&)")]
// was: RBX::GuiObject::mouseIsOver(G3D::Vector2 const&)
// IDA 0x52a334: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52a334() {
}

// 0x52b31c — __ZN3RBX9GuiButton18render2dButtonImplEPNS_5AdornERN3G3D6Rect2DE
#[doc(alias = "RBX::GuiButton::render2dButtonImpl(RBX::Adorn *,G3D::Rect2D &)")]
// was: RBX::GuiButton::render2dButtonImpl(RBX::Adorn *,G3D::Rect2D &)
// IDA 0x52b31c: 601 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52b31c() {
}

// 0x52bed0 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiObjectEN3G3D6Color3EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,G3D::Color3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiObject,G3D::Color3>::~PropDescriptor()
// IDA 0x52bed0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_52bed0() {
}

// 0x52d8e0 — __ZNK3G3D6Rect2D9intersectERKS0_
#[doc(alias = "G3D::Rect2D::intersect(G3D::Rect2D const&)const")]
// was: G3D::Rect2D::intersect(G3D::Rect2D const&)const
// IDA 0x52d8e0: 65 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_52d8e0() {
}
