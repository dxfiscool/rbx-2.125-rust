//! rendering shard 324 — 100 stubs 0x492180..0x496164 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 35320->35420 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 35320 before -> 35420 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x492180 (lowest remaining 0x492180..0x496164, next lowest 0x496188)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x492180 — __ZN5boost6detail12shared_countC2IPN3RBX5DecalENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX5DecalENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_492180() -> ! {
    todo!("0x492180 boost::detail::shared_count::shared_count<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x492288 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_492288() -> ! {
    todo!("0x492288 boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x49228c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_49228c() -> ! {
    todo!("0x49228c boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x492290 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_492290() -> ! {
    todo!("0x492290 boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x4922b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4922b0() -> ! {
    todo!("0x4922b0 boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x4922c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4922c8() -> ! {
    todo!("0x4922c8 boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x4922cc — __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorC2Ev
pub fn stub_4922cc() -> ! {
    todo!("0x4922cc __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorC2Ev")
}

// 0x492510 — __ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_492510() -> ! {
    todo!("0x492510 __ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x492550 — __ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_492550() -> ! {
    todo!("0x492550 __ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x49262c — __ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_49262c() -> ! {
    todo!("0x49262c __ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x492670 — __ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_492670() -> ! {
    todo!("0x492670 __ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x49274c — __ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_49274c() -> ! {
    todo!("0x49274c __ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x492790 — __ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_492790() -> ! {
    todo!("0x492790 __ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x49286c — __ZN3RBX10Reflection14PropDescriptorINS_7TextureEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::PropDescriptor<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>(char const*,char const*,float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7TextureEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_49286c() -> ! {
    todo!("0x49286c RBX::Reflection::PropDescriptor<RBX::Texture,float>::PropDescriptor<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>(char const*,char const*,float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x492980 — __ZN3RBX10Reflection14PropDescriptorINS_7TextureEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7TextureEfED0Ev
pub fn stub_492980() -> ! {
    todo!("0x492980 RBX::Reflection::PropDescriptor<RBX::Texture,float>::~PropDescriptor()")
}

// 0x4929ac — __ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::GetSetImpl<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
pub fn stub_4929ac() -> ! {
    todo!("0x4929ac RBX::Reflection::PropDescriptor<RBX::Texture,float>::GetSetImpl<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>::isReadOnly(void)const")
}

// 0x4929b0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::GetSetImpl<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
pub fn stub_4929b0() -> ! {
    todo!("0x4929b0 RBX::Reflection::PropDescriptor<RBX::Texture,float>::GetSetImpl<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>::isWriteOnly(void)const")
}

// 0x4929b4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::GetSetImpl<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_4929b4() -> ! {
    todo!("0x4929b4 RBX::Reflection::PropDescriptor<RBX::Texture,float>::GetSetImpl<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4929d4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::GetSetImpl<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
pub fn stub_4929d4() -> ! {
    todo!("0x4929d4 RBX::Reflection::PropDescriptor<RBX::Texture,float>::GetSetImpl<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x4929f8 — __ZN3rbx8any_castIN3RBX9TextureIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "RBX::TextureId * rbx::any_cast<RBX::TextureId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: __ZN3rbx8any_castIN3RBX9TextureIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_4929f8() -> ! {
    todo!("0x4929f8 RBX::TextureId * rbx::any_cast<RBX::TextureId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x492a50 — __ZN3rbx8any_castIRN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "RBX::TextureId & rbx::any_cast<RBX::TextureId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_492a50() -> ! {
    todo!("0x492a50 RBX::TextureId & rbx::any_cast<RBX::TextureId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x492b40 — __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEED2Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEED2Ev")]
// was: __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEED2Ev
pub fn stub_492b40() -> ! {
    todo!("0x492b40 __ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEED2Ev")
}

// 0x492bfc — __ZN3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_492bfc() -> ! {
    todo!("0x492bfc __ZN3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x492c00 — __ZN3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_492c00() -> ! {
    todo!("0x492c00 __ZN3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x492ca0 — __ZThn32_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_492ca0() -> ! {
    todo!("0x492ca0 __ZThn32_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x492ca8 — __ZThn32_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_492ca8() -> ! {
    todo!("0x492ca8 __ZThn32_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x492d4c — __ZThn36_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_492d4c() -> ! {
    todo!("0x492d4c __ZThn36_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x492d54 — __ZThn36_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_492d54() -> ! {
    todo!("0x492d54 __ZThn36_N3RBX10Reflection9DescribedINS_5DecalELZNS_6sDecalEENS_14FactoryProductIS2_NS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x492df8 — __ZN3RBX10Reflection14PropDescriptorINS_5DecalEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,float>::PropDescriptor<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>(char const*,char const*,float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5DecalEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_492df8() -> ! {
    todo!("0x492df8 RBX::Reflection::PropDescriptor<RBX::Decal,float>::PropDescriptor<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>(char const*,char const*,float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x492f0c — __ZN3RBX10Reflection14PropDescriptorINS_5DecalEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5DecalEfED0Ev
pub fn stub_492f0c() -> ! {
    todo!("0x492f0c RBX::Reflection::PropDescriptor<RBX::Decal,float>::~PropDescriptor()")
}

// 0x492f38 — __ZNK3RBX10Reflection14PropDescriptorINS_5DecalEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,float>::GetSetImpl<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5DecalEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
pub fn stub_492f38() -> ! {
    todo!("0x492f38 RBX::Reflection::PropDescriptor<RBX::Decal,float>::GetSetImpl<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>::isReadOnly(void)const")
}

// 0x492f3c — __ZNK3RBX10Reflection14PropDescriptorINS_5DecalEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,float>::GetSetImpl<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5DecalEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
pub fn stub_492f3c() -> ! {
    todo!("0x492f3c RBX::Reflection::PropDescriptor<RBX::Decal,float>::GetSetImpl<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>::isWriteOnly(void)const")
}

// 0x492f40 — __ZNK3RBX10Reflection14PropDescriptorINS_5DecalEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,float>::GetSetImpl<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5DecalEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_492f40() -> ! {
    todo!("0x492f40 RBX::Reflection::PropDescriptor<RBX::Decal,float>::GetSetImpl<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x492f60 — __ZNK3RBX10Reflection14PropDescriptorINS_5DecalEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,float>::GetSetImpl<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5DecalEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
pub fn stub_492f60() -> ! {
    todo!("0x492f60 RBX::Reflection::PropDescriptor<RBX::Decal,float>::GetSetImpl<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x492f84 — __ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEEC2IMS2_KFRKS3_vEMS2_FvS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::PropDescriptor<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEEC2IMS2_KFRKS3_vEMS2_FvS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_492f84() -> ! {
    todo!("0x492f84 RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::PropDescriptor<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x493098 — __ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEED0Ev
pub fn stub_493098() -> ! {
    todo!("0x493098 RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::~PropDescriptor()")
}

// 0x4930c4 — __ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_4930c4() -> ! {
    todo!("0x4930c4 RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>::isReadOnly(void)const")
}

// 0x4930c8 — __ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_4930c8() -> ! {
    todo!("0x4930c8 RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>::isWriteOnly(void)const")
}

// 0x4930cc — __ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_4930cc() -> ! {
    todo!("0x4930cc RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x493100 — __ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseES7_
pub fn stub_493100() -> ! {
    todo!("0x493100 RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")
}

// 0x493248 — __GLOBAL__I_a_183
#[doc(alias = "global constructor keyed to_a_183")]
// was: __GLOBAL__I_a_183
pub fn stub_493248() -> ! {
    todo!("0x493248 global constructor keyed to_a_183")
}

// 0x493660 — __ZN3RBX12DialogChoice13setUserDialogESs
#[doc(alias = "RBX::DialogChoice::setUserDialog(std::string)")]
// was: __ZN3RBX12DialogChoice13setUserDialogESs
pub fn stub_493660() -> ! {
    todo!("0x493660 RBX::DialogChoice::setUserDialog(std::string)")
}

// 0x4937d4 — __ZN3RBX12DialogChoice17setResponseDialogESs
#[doc(alias = "RBX::DialogChoice::setResponseDialog(std::string)")]
// was: __ZN3RBX12DialogChoice17setResponseDialogESs
pub fn stub_4937d4() -> ! {
    todo!("0x4937d4 RBX::DialogChoice::setResponseDialog(std::string)")
}

// 0x493810 — __ZN3RBX12DialogChoiceC2Ev
// type: _DWORD __fastcall(RBX::DialogChoice *__hidden this)
#[doc(alias = "RBX::DialogChoice::DialogChoice(void)")]
// was: __ZN3RBX12DialogChoiceC2Ev
pub fn stub_493810() -> ! {
    todo!("0x493810 RBX::DialogChoice::DialogChoice(void)")
}

// 0x493ad4 — __ZNK3RBX12DialogChoice12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::DialogChoice *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::DialogChoice::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX12DialogChoice12askSetParentEPKNS_8InstanceE
pub fn stub_493ad4() -> ! {
    todo!("0x493ad4 RBX::DialogChoice::askSetParent(RBX::Instance const*)const")
}

// 0x493b28 — __ZNK3RBX12DialogChoice13getUserDialogEv
// type: _DWORD __fastcall(RBX::DialogChoice *__hidden this)
#[doc(alias = "RBX::DialogChoice::getUserDialog(void)const")]
// was: __ZNK3RBX12DialogChoice13getUserDialogEv
pub fn stub_493b28() -> ! {
    todo!("0x493b28 RBX::DialogChoice::getUserDialog(void)const")
}

// 0x493b34 — __ZN3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsED1Ev
pub fn stub_493b34() -> ! {
    todo!("0x493b34 RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::~PropDescriptor()")
}

// 0x493b58 — __ZNK3RBX12DialogChoice17getResponseDialogEv
// type: _DWORD __fastcall(RBX::DialogChoice *__hidden this)
#[doc(alias = "RBX::DialogChoice::getResponseDialog(void)const")]
// was: __ZNK3RBX12DialogChoice17getResponseDialogEv
pub fn stub_493b58() -> ! {
    todo!("0x493b58 RBX::DialogChoice::getResponseDialog(void)const")
}

// 0x493b64 — __ZN3RBX12DialogChoiceD1Ev
// type: void __fastcall(RBX::DialogChoice *__hidden this)
#[doc(alias = "RBX::DialogChoice::~DialogChoice()")]
// was: __ZN3RBX12DialogChoiceD1Ev
pub fn stub_493b64() -> ! {
    todo!("0x493b64 RBX::DialogChoice::~DialogChoice()")
}

// 0x493cb8 — __ZN3RBX12DialogChoiceD0Ev
// type: void __fastcall(RBX::DialogChoice *__hidden this)
#[doc(alias = "RBX::DialogChoice::~DialogChoice()")]
// was: __ZN3RBX12DialogChoiceD0Ev
pub fn stub_493cb8() -> ! {
    todo!("0x493cb8 RBX::DialogChoice::~DialogChoice()")
}

// 0x493d58 — __ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E12getClassNameEv
pub fn stub_493d58() -> ! {
    todo!("0x493d58 __ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E12getClassNameEv")
}

// 0x493d68 — __ZThn32_N3RBX12DialogChoiceD1Ev
// type: void __fastcall(RBX::DialogChoice *this, int, int, int)
#[doc(alias = "non-virtual thunk toRBX::DialogChoice::~DialogChoice()")]
// was: __ZThn32_N3RBX12DialogChoiceD1Ev
pub fn stub_493d68() -> ! {
    todo!("0x493d68 non-virtual thunk toRBX::DialogChoice::~DialogChoice()")
}

// 0x493ebc — __ZThn32_N3RBX12DialogChoiceD0Ev
// type: void __fastcall(RBX::DialogChoice *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DialogChoice::~DialogChoice()")]
// was: __ZThn32_N3RBX12DialogChoiceD0Ev
pub fn stub_493ebc() -> ! {
    todo!("0x493ebc non-virtual thunk toRBX::DialogChoice::~DialogChoice()")
}

// 0x494028 — __ZThn32_NK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E12getClassNameEv
pub fn stub_494028() -> ! {
    todo!("0x494028 __ZThn32_NK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E12getClassNameEv")
}

// 0x494038 — __ZThn36_N3RBX12DialogChoiceD1Ev
// type: void __fastcall(RBX::DialogChoice *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DialogChoice::~DialogChoice()")]
// was: __ZThn36_N3RBX12DialogChoiceD1Ev
pub fn stub_494038() -> ! {
    todo!("0x494038 non-virtual thunk toRBX::DialogChoice::~DialogChoice()")
}

// 0x49418c — __ZThn36_N3RBX12DialogChoiceD0Ev
// type: void __fastcall(RBX::DialogChoice *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DialogChoice::~DialogChoice()")]
// was: __ZThn36_N3RBX12DialogChoiceD0Ev
pub fn stub_49418c() -> ! {
    todo!("0x49418c non-virtual thunk toRBX::DialogChoice::~DialogChoice()")
}

// 0x4942f8 — __ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorD1Ev
pub fn stub_4942f8() -> ! {
    todo!("0x4942f8 __ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorD1Ev")
}

// 0x4942fc — __ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorD2Ev
pub fn stub_4942fc() -> ! {
    todo!("0x4942fc __ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorD2Ev")
}

// 0x494398 — __ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7Creator12getClassNameEv
pub fn stub_494398() -> ! {
    todo!("0x494398 __ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7Creator12getClassNameEv")
}

// 0x494420 — __ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7Creator6createEv
pub fn stub_494420() -> ! {
    todo!("0x494420 __ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7Creator6createEv")
}

// 0x494564 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12DialogChoiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::DialogChoice> RBX::Creatable<RBX::Instance>::create<RBX::DialogChoice>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_12DialogChoiceEEEN5boost10shared_ptrIT_EEv
pub fn stub_494564() -> ! {
    todo!("0x494564 boost::shared_ptr<RBX::DialogChoice> RBX::Creatable<RBX::Instance>::create<RBX::DialogChoice>(void)")
}

// 0x494614 — __ZN5boost10shared_ptrIN3RBX12DialogChoiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::DialogChoice>::shared_ptr<RBX::DialogChoice,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX12DialogChoiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_494614() -> ! {
    todo!("0x494614 boost::shared_ptr<RBX::DialogChoice>::shared_ptr<RBX::DialogChoice,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x494804 — __ZN5boost6detail12shared_countC2IPN3RBX12DialogChoiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX12DialogChoiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_494804() -> ! {
    todo!("0x494804 boost::detail::shared_count::shared_count<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x49490c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_49490c() -> ! {
    todo!("0x49490c boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x494910 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_494910() -> ! {
    todo!("0x494910 boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x494914 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_494914() -> ! {
    todo!("0x494914 boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x494934 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_494934() -> ! {
    todo!("0x494934 boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x49494c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_49494c() -> ! {
    todo!("0x49494c boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x494950 — __ZN3RBX4Name13callDoDeclareILZNS_13sDialogChoiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sDialogChoiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sDialogChoiceEEEEvv
pub fn stub_494950() -> ! {
    todo!("0x494950 __ZN3RBX4Name13callDoDeclareILZNS_13sDialogChoiceEEEEvv")
}

// 0x494954 — __ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v
pub fn stub_494954() -> ! {
    todo!("0x494954 __ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v")
}

// 0x494a34 — __ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorC2Ev
pub fn stub_494a34() -> ! {
    todo!("0x494a34 __ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorC2Ev")
}

// 0x494c78 — __ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E17static_getCreatorEv
pub fn stub_494c78() -> ! {
    todo!("0x494c78 __ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E17static_getCreatorEv")
}

// 0x494cec — __ZN3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_494cec() -> ! {
    todo!("0x494cec __ZN3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x494cf0 — __ZN3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_494cf0() -> ! {
    todo!("0x494cf0 __ZN3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x494d90 — __ZThn32_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_494d90() -> ! {
    todo!("0x494d90 __ZThn32_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x494d98 — __ZThn32_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_494d98() -> ! {
    todo!("0x494d98 __ZThn32_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x494e3c — __ZThn36_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_494e3c() -> ! {
    todo!("0x494e3c __ZThn36_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x494e44 — __ZThn36_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_494e44() -> ! {
    todo!("0x494e44 __ZThn36_N3RBX10Reflection9DescribedINS_12DialogChoiceELZNS_13sDialogChoiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sDialogChoiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x494ee8 — __ZN3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::PropDescriptor<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>(char const*,char const*,std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_494ee8() -> ! {
    todo!("0x494ee8 RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::PropDescriptor<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>(char const*,char const*,std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x494ffc — __ZN3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsED0Ev
pub fn stub_494ffc() -> ! {
    todo!("0x494ffc RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::~PropDescriptor()")
}

// 0x495028 — __ZNK3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::GetSetImpl<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
pub fn stub_495028() -> ! {
    todo!("0x495028 RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::GetSetImpl<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>::isReadOnly(void)const")
}

// 0x49502c — __ZNK3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::GetSetImpl<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
pub fn stub_49502c() -> ! {
    todo!("0x49502c RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::GetSetImpl<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>::isWriteOnly(void)const")
}

// 0x495030 — __ZNK3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::GetSetImpl<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_495030() -> ! {
    todo!("0x495030 RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::GetSetImpl<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x495058 — __ZNK3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::GetSetImpl<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
pub fn stub_495058() -> ! {
    todo!("0x495058 RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::GetSetImpl<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x49519c — __GLOBAL__I_a_184
#[doc(alias = "global constructor keyed to_a_184")]
// was: __GLOBAL__I_a_184
pub fn stub_49519c() -> ! {
    todo!("0x49519c global constructor keyed to_a_184")
}

// 0x495428 — __ZN3RBX10DialogRoot16setInitialPromptESs
#[doc(alias = "RBX::DialogRoot::setInitialPrompt(std::string)")]
// was: __ZN3RBX10DialogRoot16setInitialPromptESs
pub fn stub_495428() -> ! {
    todo!("0x495428 RBX::DialogRoot::setInitialPrompt(std::string)")
}

// 0x495464 — __ZN3RBX10DialogRoot16setDialogPurposeENS0_13DialogPurposeE
#[doc(alias = "RBX::DialogRoot::setDialogPurpose(RBX::DialogRoot::DialogPurpose)")]
// was: __ZN3RBX10DialogRoot16setDialogPurposeENS0_13DialogPurposeE
pub fn stub_495464() -> ! {
    todo!("0x495464 RBX::DialogRoot::setDialogPurpose(RBX::DialogRoot::DialogPurpose)")
}

// 0x495480 — __ZN3RBX10DialogRoot13setDialogToneENS0_10DialogToneE
#[doc(alias = "RBX::DialogRoot::setDialogTone(RBX::DialogRoot::DialogTone)")]
// was: __ZN3RBX10DialogRoot13setDialogToneENS0_10DialogToneE
pub fn stub_495480() -> ! {
    todo!("0x495480 RBX::DialogRoot::setDialogTone(RBX::DialogRoot::DialogTone)")
}

// 0x49549c — __ZN3RBX10DialogRoot23setConversationDistanceEf
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this, float)
#[doc(alias = "RBX::DialogRoot::setConversationDistance(float)")]
// was: __ZN3RBX10DialogRoot23setConversationDistanceEf
pub fn stub_49549c() -> ! {
    todo!("0x49549c RBX::DialogRoot::setConversationDistance(float)")
}

// 0x4954c4 — __ZN3RBX10DialogRoot8setInUseEb
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this, bool)
#[doc(alias = "RBX::DialogRoot::setInUse(bool)")]
// was: __ZN3RBX10DialogRoot8setInUseEb
pub fn stub_4954c4() -> ! {
    todo!("0x4954c4 RBX::DialogRoot::setInUse(bool)")
}

// 0x4954e4 — __ZN3RBX10DialogRoot18signalDialogChoiceEN5boost10shared_ptrINS_8InstanceEEES4_
#[doc(alias = "RBX::DialogRoot::signalDialogChoice(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX10DialogRoot18signalDialogChoiceEN5boost10shared_ptrINS_8InstanceEEES4_
pub fn stub_4954e4() -> ! {
    todo!("0x4954e4 RBX::DialogRoot::signalDialogChoice(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)")
}

// 0x4958dc — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEEC1Ev
pub fn stub_4958dc() -> ! {
    todo!("0x4958dc RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::EnumDesc(void)")
}

// 0x4958e0 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEEC2Ev
pub fn stub_4958e0() -> ! {
    todo!("0x4958e0 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::EnumDesc(void)")
}

// 0x495ab8 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEEC1Ev
pub fn stub_495ab8() -> ! {
    todo!("0x495ab8 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::EnumDesc(void)")
}

// 0x495abc — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEEC2Ev
pub fn stub_495abc() -> ! {
    todo!("0x495abc RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::EnumDesc(void)")
}

// 0x495c94 — __ZN3RBX10DialogRootC2Ev
// type: RBX::Instance *__fastcall(RBX::DialogRoot *this)
#[doc(alias = "RBX::DialogRoot::DialogRoot(void)")]
// was: __ZN3RBX10DialogRootC2Ev
pub fn stub_495c94() -> ! {
    todo!("0x495c94 RBX::DialogRoot::DialogRoot(void)")
}

// 0x495f88 — __ZN3RBX10DialogRoot17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::DialogRoot::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX10DialogRoot17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_495f88() -> ! {
    todo!("0x495f88 RBX::DialogRoot::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x49611c — __ZNK3RBX10DialogRoot12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::DialogRoot::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX10DialogRoot12askSetParentEPKNS_8InstanceE
pub fn stub_49611c() -> ! {
    todo!("0x49611c RBX::DialogRoot::askSetParent(RBX::Instance const*)const")
}

// 0x496158 — __ZNK3RBX10DialogRoot16getInitialPromptEv
// type: _DWORD __fastcall(RBX::DialogRoot *__hidden this)
#[doc(alias = "RBX::DialogRoot::getInitialPrompt(void)const")]
// was: __ZNK3RBX10DialogRoot16getInitialPromptEv
pub fn stub_496158() -> ! {
    todo!("0x496158 RBX::DialogRoot::getInitialPrompt(void)const")
}

// 0x496164 — __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsED1Ev
pub fn stub_496164() -> ! {
    todo!("0x496164 RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::~PropDescriptor()")
}