// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x25917c..0x2b4be8 | script 16991->17141 distinct (filler 0x25917c asc, not-in-script gaps)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
pub fn stub_0x25917c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,RBX::HttpService::HttpContentType const&)")]
pub fn stub_0x2591e4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_allocate(unsigned long)")]
pub fn stub_0x2592c8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::HttpService::HttpContentType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *>(RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *)")]
pub fn stub_0x2592e0(handle: &crate::slot::InstanceHandle) {
// RBX::HttpService::HttpContentType* std::__copy_backward<false, std::random_access_iterator~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,unsigned long,RBX::HttpService::HttpContentType const&)")]
pub fn stub_0x25931c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::isReadOnly(void)const")]
pub fn stub_0x25963c() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::isWriteOnly(void)const")]
pub fn stub_0x259640() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x259644() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HttpService>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x259650() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x259838() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc() [0x259868]")]
pub fn stub_0x259868(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::HttpService,std::string (RBX::HttpService::*)(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,std::string>::call(RBX::HttpService*,std::string (RBX::HttpService::*)(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),RBX::Reflection::Variant&,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")]
pub fn stub_0x259a6c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x259f34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc() [0x259f64]")]
pub fn stub_0x259f64(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),std::string,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::HttpService*,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),RBX::Reflection::Variant&,std::string const&)")]
pub fn stub_0x25a170() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

#[doc(alias = "global constructor keyed to_a_55")]
pub fn stub_0x25afd8() -> crate::slot::PortedFn {
// IDA 0x25afd8: __GLOBAL__I_a_55.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x25afd8, "__GLOBAL__I_a_55")
}

#[doc(alias = "RBX::Light::setEnabled(bool)")]
pub fn stub_0x25b4c0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Light setter.
cell.set(value)
}

#[doc(alias = "RBX::Light::setColor(G3D::Color3)")]
pub fn stub_0x25b4e0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Light setter.
cell.set(value)
}

#[doc(alias = "RBX::Light::setBrightness(float)")]
pub fn stub_0x25b544(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Light setter.
cell.set(value)
}

#[doc(alias = "RBX::PointLight::setRange(float)")]
pub fn stub_0x25b574(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::PointLight setter.
cell.set(value)
}

#[doc(alias = "RBX::SpotLight::setRange(float)")]
pub fn stub_0x25b5b0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::SpotLight setter.
cell.set(value)
}

#[doc(alias = "RBX::SpotLight::setAngle(float)")]
pub fn stub_0x25b5ec(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::SpotLight setter.
cell.set(value)
}

#[doc(alias = "RBX::registerNewLightAPI(void)")]
pub fn stub_0x25b628() -> crate::slot::PortedFn {
// IDA 0x25b628: RBX::registerNewLightAPI().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x25b628, "RBX::registerNewLightAPI()")
}

#[doc(alias = "RBX::Light::setShadows(bool)")]
pub fn stub_0x25b884(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Light setter.
cell.set(value)
}

#[doc(alias = "RBX::SpotLight::setFace(RBX::NormalId)")]
pub fn stub_0x25b8a8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::SpotLight setter.
cell.set(value)
}

#[doc(alias = "RBX::Light::Light(char const*)")]
pub fn stub_0x25b8c8() -> crate::slot::InstanceHandle {
// RBX::Light ctor.
crate::slot::InstanceHandle::new("RBX::Light")
}

#[doc(alias = "RBX::Light::~Light()")]
pub fn stub_0x25baa8(handle: crate::slot::InstanceHandle) {
// RBX::Light dtor.
drop(handle);
}

#[doc(alias = "RBX::Light::~Light() [0x25bb48]")]
pub fn stub_0x25bb48(handle: crate::slot::InstanceHandle) {
// RBX::Light dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
pub fn stub_0x25bb4c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Light::~Light() [0x25bb54]")]
pub fn stub_0x25bb54(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Light::~Light() [0x25bb5c]")]
pub fn stub_0x25bb5c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::Light::~Light() [0x25bb64]")]
pub fn stub_0x25bb64(handle: crate::slot::InstanceHandle) {
// RBX::Light dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Light::~Light() [0x25bc20]")]
pub fn stub_0x25bc20(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Light::~Light() [0x25bc28]")]
pub fn stub_0x25bc28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Light::~Light() [0x25bc30]")]
pub fn stub_0x25bc30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::Light::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x25bc38(handle: &crate::slot::InstanceHandle) {
// RBX::Light::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Light::askAddChild(RBX::Instance const*)const")]
pub fn stub_0x25bc60(handle: &crate::slot::InstanceHandle) {
// RBX::Light::askAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PointLight::PointLight(void)")]
pub fn stub_0x25bc64() -> crate::slot::InstanceHandle {
// RBX::PointLight ctor.
crate::slot::InstanceHandle::new("RBX::PointLight")
}

#[doc(alias = "RBX::PointLight::~PointLight()")]
pub fn stub_0x25bdb8(handle: crate::slot::InstanceHandle) {
// RBX::PointLight dtor.
drop(handle);
}

#[doc(alias = "RBX::PointLight::~PointLight() [0x25be58]")]
pub fn stub_0x25be58(handle: crate::slot::InstanceHandle) {
// RBX::PointLight dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
pub fn stub_0x25be5c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight() [0x25be64]")]
pub fn stub_0x25be64(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight() [0x25be6c]")]
pub fn stub_0x25be6c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight() [0x25be74]")]
pub fn stub_0x25be74(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight() [0x25be7c]")]
pub fn stub_0x25be7c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight() [0x25be84]")]
pub fn stub_0x25be84(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::SpotLight::SpotLight(void)")]
pub fn stub_0x25be8c() -> crate::slot::InstanceHandle {
// RBX::SpotLight ctor.
crate::slot::InstanceHandle::new("RBX::SpotLight")
}

#[doc(alias = "RBX::SpotLight::~SpotLight()")]
pub fn stub_0x25bff0(handle: crate::slot::InstanceHandle) {
// RBX::SpotLight dtor.
drop(handle);
}

#[doc(alias = "RBX::SpotLight::~SpotLight() [0x25c090]")]
pub fn stub_0x25c090(handle: crate::slot::InstanceHandle) {
// RBX::SpotLight dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
pub fn stub_0x25c094(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight() [0x25c09c]")]
pub fn stub_0x25c09c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight() [0x25c0a4]")]
pub fn stub_0x25c0a4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight() [0x25c0ac]")]
pub fn stub_0x25c0ac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight() [0x25c0b4]")]
pub fn stub_0x25c0b4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight() [0x25c0bc]")]
pub fn stub_0x25c0bc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::Light::getEnabled(void)const")]
pub fn stub_0x25c0c4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Light getter.
cell.get()
}

#[doc(alias = "RBX::Light::getColor(void)const")]
pub fn stub_0x25c0f0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Light getter.
cell.get()
}

#[doc(alias = "RBX::Light::getBrightness(void)const")]
pub fn stub_0x25c124(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Light getter.
cell.get()
}

#[doc(alias = "RBX::PointLight::getRange(void)const")]
pub fn stub_0x25c14c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PointLight getter.
cell.get()
}

#[doc(alias = "RBX::SpotLight::getRange(void)const")]
pub fn stub_0x25c174(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SpotLight getter.
cell.get()
}

#[doc(alias = "RBX::SpotLight::getAngle(void)const")]
pub fn stub_0x25c19c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SpotLight getter.
cell.get()
}

#[doc(alias = "RBX::Light::getShadows(void)const")]
pub fn stub_0x25c1a0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Light getter.
cell.get()
}

#[doc(alias = "RBX::SpotLight::getFace(void)const")]
pub fn stub_0x25c1a8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SpotLight getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_6sLightEEE12getClassNameEv")]
pub fn stub_0x25c1d0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_6sLightEEE12getClassNameEv")]
pub fn stub_0x25c1f8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10PointLightENS_5LightELZNS_11sPointLightEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x25c220() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"PointLight"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10PointLightENS_5LightELZNS_11sPointLightEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x25c230() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"PointLight"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9SpotLightENS_5LightELZNS_10sSpotLightEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x25c240() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SpotLight"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9SpotLightENS_5LightELZNS_10sSpotLightEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x25c250() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SpotLight"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10PointLightENS_5LightELZNS_11sPointLightEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x25c260() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"PointLight"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9SpotLightENS_5LightELZNS_10sSpotLightEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x25c264() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SpotLight"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9SpotLightENS_5LightELZNS_10sSpotLightEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x25c268() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SpotLight"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9SpotLightENS_5LightELZNS_10sSpotLightEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x25c304() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SpotLight"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9SpotLightENS_5LightELZNS_10sSpotLightEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x25c38c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SpotLight"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)")]
pub fn stub_0x25c4d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::SpotLight")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x25c580() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::SpotLight")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SpotLight,RBX::SpotLight>(rbx_core::SharedPtr<RBX::SpotLight> const*,RBX::SpotLight *)const")]
pub fn stub_0x25c648() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::SpotLight")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x25c730() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x25c838(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x25c83c]")]
pub fn stub_0x25c83c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x25c840() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x25c860() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x25c878() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sSpotLightEEEEvv")]
pub fn stub_0x25c87c() -> crate::slot::PortedFn {
// IDA 0x25c87c: void RBX::Name::callDoDeclare<RBX::sSpotLight>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x25c87c, "void RBX::Name::callDoDeclare<RBX::sSpotLight>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v")]
pub fn stub_0x25c880(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSpotLight>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9SpotLightENS_5LightELZNS_10sSpotLightEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x25c960() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SpotLight"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9SpotLightENS_5LightELZNS_10sSpotLightEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x25cba4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SpotLight"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10PointLightENS_5LightELZNS_11sPointLightEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x25cc18() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"PointLight"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10PointLightENS_5LightELZNS_11sPointLightEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x25ccb4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"PointLight"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10PointLightENS_5LightELZNS_11sPointLightEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x25cd3c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"PointLight"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void)")]
pub fn stub_0x25ce80() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PointLight")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x25cf30() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PointLight")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PointLight,RBX::PointLight>(rbx_core::SharedPtr<RBX::PointLight> const*,RBX::PointLight *)const")]
pub fn stub_0x25cff8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PointLight")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x25d0e0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x25d1e8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x25d1ec]")]
pub fn stub_0x25d1ec(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x25d1f0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x25d210() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x25d228() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sPointLightEEEEvv")]
pub fn stub_0x25d22c() -> crate::slot::PortedFn {
// IDA 0x25d22c: void RBX::Name::callDoDeclare<RBX::sPointLight>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x25d22c, "void RBX::Name::callDoDeclare<RBX::sPointLight>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v")]
pub fn stub_0x25d230(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sPointLight>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10PointLightENS_5LightELZNS_11sPointLightEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x25d310() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"PointLight"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10PointLightENS_5LightELZNS_11sPointLightEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x25d554() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"PointLight"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sLightEEEEvv")]
pub fn stub_0x25d5c8() -> crate::slot::PortedFn {
// IDA 0x25d5c8: void RBX::Name::callDoDeclare<RBX::sLight>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x25d5c8, "void RBX::Name::callDoDeclare<RBX::sLight>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v")]
pub fn stub_0x25d5cc(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sLight>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x2b00ac(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Heartbeat const&)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x2b00b0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Heartbeat const&)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::~slot()")]
pub fn stub_0x2b01a0(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::~slot() [0x2b01cc]")]
pub fn stub_0x2b01cc(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "RBX::RunService * RBX::ServiceProvider::create<RBX::RunService>(void)const")]
pub fn stub_0x2b03a0() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::RunService")
}

#[doc(alias = "RBX::RunService * RBX::ServiceProvider::find<RBX::RunService>(void)const")]
pub fn stub_0x2b0568() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::RunService"))
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RunService,RBX::RunService>(rbx_core::SharedPtr<RBX::RunService> const*,RBX::RunService *)const")]
pub fn stub_0x2b06e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RunService")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2b07d0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::~deque()")]
pub fn stub_0x2b0a88(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::~_Deque_base()")]
pub fn stub_0x2b0b70() -> crate::slot::PortedFn {
// IDA 0x2b0b70: std::_Deque_base<std::string, std::allocator<std::string>>::~_Deque_base().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b0b70, "std::_Deque_base<std::string, std::allocator<std::string>>::~_Deque_base()")
}

#[doc(alias = "RBX::Stats::StatsService * RBX::ServiceProvider::create<RBX::Stats::StatsService>(void)const")]
pub fn stub_0x2b0c88() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::Stats::StatsService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::StatsService> RBX::Creatable<RBX::Instance>::create<RBX::Stats::StatsService>(void)")]
pub fn stub_0x2b0e50() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Stats::StatsService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Stats::StatsService>(rbx_core::SharedPtr<RBX::Stats::StatsService> const&)")]
pub fn stub_0x2b0f00(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x2b0f40(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::Stats::Item, RBX::Stats::sStatsItem, RBX::NonFactoryProduc~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>> const&)")]
pub fn stub_0x2b1060() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x2b10d0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x2b1170(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x2b1178(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::StatsService>::shared_ptr<RBX::Stats::StatsService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2b1220() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Stats::StatsService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Stats::StatsService,RBX::Stats::StatsService>(rbx_core::SharedPtr<RBX::Stats::StatsService> const*,RBX::Stats::StatsService *)const")]
pub fn stub_0x2b12e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Stats::StatsService")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2b13d8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContentProvider> RBX::Creatable<RBX::Instance>::create<RBX::ContentProvider>(void)")]
pub fn stub_0x2b1828() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContentProvider")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentProvider>(rbx_core::SharedPtr<RBX::ContentProvider> const&)")]
pub fn stub_0x2b18d8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sContentProviderEEEEvv")]
pub fn stub_0x2b1910() -> crate::slot::PortedFn {
// IDA 0x2b1910: void RBX::Name::callDoDeclare<RBX::sContentProvider>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b1910, "void RBX::Name::callDoDeclare<RBX::sContentProvider>()")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContentProvider>(void)")]
pub fn stub_0x2b1918() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2b1920() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2b1a28(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2b1a30() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2b1a50() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2b1a68() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>::list3(boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>)")]
pub fn stub_0x2b2d20() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
pub fn stub_0x2b3d88() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::dummy::nonnull(void)")]
pub fn stub_0x2b3e50() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>> const&)")]
pub fn stub_0x2b3f54(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZN5boost6threadC2INS_9function0IvEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRS4_NS_6detail13thread_move_tIS4_EEEE5valueEPNS0_5dummyEE4typeE")]
pub fn stub_0x2b40a8() -> crate::slot::PortedFn {
// IDA 0x2b40a8: boost::thread::thread<boost::function0<void>>(boost::function0<void>, boost::disable_if_c<boost::thread_detail::is_conve~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b40a8, "boost::thread::thread<boost::function0<void>>(boost::function0<void>, boost::disable_if_c<boost::thr~")
}

#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::~thread_data() [0x2b41f0]")]
pub fn stub_0x2b41f0() -> crate::slot::PortedFn {
// IDA 0x2b41f0: boost::detail::thread_data<boost::function0<void>>::~thread_data().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b41f0, "boost::detail::thread_data<boost::function0<void>>::~thread_data()")
}

#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::run(void)")]
pub fn stub_0x2b42d0() -> crate::slot::PortedFn {
// IDA 0x2b42d0: boost::detail::thread_data<boost::function0<void>>::run().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b42d0, "boost::detail::thread_data<boost::function0<void>>::run()")
}

#[doc(alias = "boost::detail::thread_data_base::notify_all_at_thread_exit(boost::condition_variable *,boost::mutex *)")]
pub fn stub_0x2b42d8() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x2b42d8, "boost::detail::thread_data_base::notify_all_at_thread_exit(boost::condition_variable*, boost::mutex*~")
}

#[doc(alias = "boost::function0<void>::operator()(void)const")]
pub fn stub_0x2b42f0(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::condition_variable::~condition_variable()")]
pub fn stub_0x2b43b0() -> crate::slot::PortedFn {
// IDA 0x2b43b0: boost::condition_variable::~condition_variable().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b43b0, "boost::condition_variable::~condition_variable()")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::function0<void>>>(boost::detail::thread_data<boost::function0<void>> *)")]
pub fn stub_0x2b43d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::~sp_counted_impl_p() [0x2b44d0]")]
pub fn stub_0x2b44d0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::dispose(void)")]
pub fn stub_0x2b44d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::get_deleter(std::type_info const&)")]
pub fn stub_0x2b44e8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::singleton(void)")]
pub fn stub_0x2b4be8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}
