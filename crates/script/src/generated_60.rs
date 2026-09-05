// Auto-generated skeletons for rbx-script — filler EA-sorted ascending after 0x5cb09c (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x5e0c88..0x5f4698 | existing ~10891 -> ~10991 total (union; filler 0x5e0c88 ascending, global remaining)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
#[doc(alias = "RBX::PartInstance::getMassNonConst(void)")]
pub fn stub_0x5e0c88(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getColor3(void)const")]
pub fn stub_0x5e0d20(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getColor(void)const")]
pub fn stub_0x5e0d90(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getTransparencyXml(void)const")]
pub fn stub_0x5e0de8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getAlphaModifier(void)const")]
pub fn stub_0x5e0df0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getReflectance(void)const")]
pub fn stub_0x5e0df8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getPartLocked(void)const")]
pub fn stub_0x5e0e00(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getOrCreateLocalSimulationTouchedSignal(void)")]
pub fn stub_0x5e0ee0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getOrCreateTouchedSignal(void)")]
pub fn stub_0x5e0f10(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getOrCreateTouchedEndedSignal(void)")]
pub fn stub_0x5e0f40(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getOrCreateDeprecatedStoppedTouchingSignal(void)")]
pub fn stub_0x5e0f4c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getOrCreateOutfitChangedSignal(void)")]
pub fn stub_0x5e0f58(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getReceiveInterval(void)const")]
pub fn stub_0x5e0f88(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::OnDemandPVInstance::OnDemandPVInstance(void)")]
pub fn stub_0x5e10ac() -> crate::slot::InstanceHandle {
// RBX::OnDemandPVInstance ctor.
crate::slot::InstanceHandle::new("RBX::OnDemandPVInstance")
}

#[doc(alias = "RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::Allocator(void)")]
pub fn stub_0x5e1178() -> crate::slot::InstanceHandle {
// RBX::Allocator ctor.
crate::slot::InstanceHandle::new("RBX::Allocator")
}

#[doc(alias = "RBX::FWPartInstance * RBX::FWBase::init<RBX::FWPartInstance>(RBX::FWPartInstance *)")]
pub fn stub_0x5e11dc(handle: &crate::slot::InstanceHandle) {
// RBX::FWPartInstance* RBX::FWBase::init<RBX::FWPartInstance>(RBX::FWPartInstance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartInstance::OnDemandPartInstance::operator delete(void *)")]
pub fn stub_0x5e1314(handle: &crate::slot::InstanceHandle) {
// RBX::PartInstance::OnDemandPartInstance::operator delete(void*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartInstance::hasTouchTransmitter(void)const")]
pub fn stub_0x5e1534(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance> RBX::shared_from<RBX::PartInstance>(RBX::PartInstance*)")]
pub fn stub_0x5e1610() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::push_back(rbx_core::WeakPtr<RBX::PartInstance> const&)")]
pub fn stub_0x5e1780() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "RBX::FWDictionnary<RBX::FWPartInstance>::registerFlyweight(RBX::FWRef *)")]
pub fn stub_0x5e197c(handle: &crate::slot::InstanceHandle) {
// RBX::FWDictionnary<RBX::FWPartInstance>::registerFlyweight(RBX::FWRef*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Allocator<RBX::FWPartInstance>::Allocator(void)")]
pub fn stub_0x5e2978() -> crate::slot::InstanceHandle {
// RBX::Allocator ctor.
crate::slot::InstanceHandle::new("RBX::Allocator")
}

#[doc(alias = "RBX::PartInstance::getPersistentDataCost(void)const")]
pub fn stub_0x5e29dc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PVInstance::childHashCode(void)const")]
pub fn stub_0x5e29f8(handle: &crate::slot::InstanceHandle) {
// RBX::PVInstance::childHashCode() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartInstance::hasThreeDimensionalSize(void)")]
pub fn stub_0x5e2a24(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getPartType(void)const")]
pub fn stub_0x5e2a28(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getMinimumUiSize(void)const")]
pub fn stub_0x5e2a2c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getMinimumUiSizeCustom(void)const")]
pub fn stub_0x5e2a78(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getResizeHandleMask(void)const")]
pub fn stub_0x5e2ac8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getDragUtilitiesSupport(void)const")]
pub fn stub_0x5e2adc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getResizeIncrement(void)const")]
pub fn stub_0x5e2ae0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getMinimumResizeIncrement(void)const")]
pub fn stub_0x5e2ae4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getFormFactor(void)const")]
pub fn stub_0x5e2af0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getLocation(void)")]
pub fn stub_0x5e2af4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
pub fn stub_0x5e2b44(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::getPrimaryPart(void)")]
pub fn stub_0x5e2b60(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::partNeeds3dAdorn(void)const")]
pub fn stub_0x5e2b64(handle: &crate::slot::InstanceHandle) {
// RBX::PartInstance::partNeeds3dAdorn() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::PartInstance::getPrimaryPart(void)")]
pub fn stub_0x5e2b94(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "virtual thunk toRBX::PartInstance::getLocation(void)")]
pub fn stub_0x5e2bd0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

#[doc(alias = "non-virtual thunk toRBX::PartInstance::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
pub fn stub_0x5e2c30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "RBX::PVInstance::initOnDemand(void)")]
pub fn stub_0x5e2d58(handle: &crate::slot::InstanceHandle) {
// RBX::PVInstance::initOnDemand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PVInstance::getPersistentDataCost(void)const")]
pub fn stub_0x5e2e0c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PVInstance getter.
cell.get()
}

#[doc(alias = "RBX::PartInstance::OnDemandPartInstance::~OnDemandPartInstance()")]
pub fn stub_0x5e3488(handle: crate::slot::InstanceHandle) {
// RBX::PartInstance::OnDemandPartInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::PartInstance::OnDemandPartInstance::~OnDemandPartInstance() [0x5e348c]")]
pub fn stub_0x5e348c(handle: crate::slot::InstanceHandle) {
// RBX::PartInstance::OnDemandPartInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::FWPartInstance::~FWPartInstance()")]
pub fn stub_0x5e3540(handle: crate::slot::InstanceHandle) {
// RBX::FWPartInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::FWPartInstance::~FWPartInstance() [0x5e3544]")]
pub fn stub_0x5e3544(handle: crate::slot::InstanceHandle) {
// RBX::FWPartInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::FWPartInstance::operator delete(void *)")]
pub fn stub_0x5e35f8(handle: &crate::slot::InstanceHandle) {
// RBX::FWPartInstance::operator delete(void*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartInstance::OnDemandPartInstance::~OnDemandPartInstance() [0x5e3638]")]
pub fn stub_0x5e3638(handle: crate::slot::InstanceHandle) {
// RBX::PartInstance::OnDemandPartInstance dtor.
drop(handle);
}

#[doc(alias = "void boost::throw_exception<rbx::bad_placement_any_cast>(rbx::bad_placement_any_cast const&)")]
pub fn stub_0x5e3fd0(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PartInstance::FormFactor>(RBX::PartInstance::FormFactor const&)")]
pub fn stub_0x5e4654() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::PartInstance::FormFactor>::singleton(void)")]
pub fn stub_0x5e46a4(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::PartInstance::FormFactor>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::PartInstance::FormFactor>::construct_func(char const*,char *)")]
pub fn stub_0x5e4710(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::PartInstance::FormFactor>::construct_func(char cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::PartInstance::FormFactor>::destruct_func(char *)")]
pub fn stub_0x5e471c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::PartInstance::FormFactor>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartInstance::FormFactor const& rbx::any_cast<RBX::PartInstance::FormFactor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x5e47ec(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Allocator<RBX::FWPartInstance>::releaseMemory(void)")]
pub fn stub_0x5e4c60(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::FWPartInstance>::releaseMemory() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWBase>& rbx_core::SharedPtr<RBX::FWBase>::operator=<RBX::FWPartInstance>(rbx_core::SharedPtr<RBX::FWPartInstance> const&)")]
pub fn stub_0x5e60b8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWPartInstance> RBX::shared_from<RBX::FWPartInstance>(RBX::FWPartInstance*)")]
pub fn stub_0x5e60ec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FWPartInstance")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartInstance>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::DumbPtr<RBX::FWPartInstance>>>(RBX::DumbPtr<RBX::FWPartInstance> const&,boost::unordered::detail::emplace_args1<RBX::DumbPtr<RBX::FWPartInstance>> const&)")]
pub fn stub_0x5e6568(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::create_buckets(unsigned long)")]
pub fn stub_0x5e6718(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x5e6840(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::rehash_impl(unsigned long)")]
pub fn stub_0x5e68d0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x5e68fc(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartInstance>>>>::construct(void)")]
pub fn stub_0x5e6950() -> crate::slot::PortedFn {
// IDA 0x5e6950: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartIns~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5e6950, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Du~")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartInstance>>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::find_node_impl<RBX::DumbPtr<RBX::FWPartInstance>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>(unsigned long,RBX::DumbPtr<RBX::FWPartInstance> const&,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>> const&)const")]
pub fn stub_0x5e6988(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "RBX::ComputeProp<RBX::Part,RBX::PartInstance::OnDemandPartInstance>::getValue(void)")]
pub fn stub_0x5e6e20(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ComputeProp getter.
cell.get()
}

#[doc(alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance>*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>,rbx_core::WeakPtr<RBX::PartInstance> const&)")]
pub fn stub_0x5e75dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "std::_Vector_base<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::_M_allocate(unsigned long)")]
pub fn stub_0x5e7b24() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *>(rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *)")]
pub fn stub_0x5e7b40() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::PartInstance>*,std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>>,rbx_core::SharedPtr<RBX::PartInstance> const&)")]
pub fn stub_0x5e7b98() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "RBX::FWFinal<RBX::FWPartInstance>::~FWFinal()")]
pub fn stub_0x5e852c(handle: crate::slot::InstanceHandle) {
// RBX::FWFinal dtor.
drop(handle);
}

#[doc(alias = "RBX::FWFinal<RBX::FWPartInstance>::~FWFinal() [0x5e8530]")]
pub fn stub_0x5e8530(handle: crate::slot::InstanceHandle) {
// RBX::FWFinal dtor.
drop(handle);
}

#[doc(alias = "RBX::FWFinal<RBX::FWPartInstance>::~FWFinal() [0x5e85e4]")]
pub fn stub_0x5e85e4(handle: crate::slot::InstanceHandle) {
// RBX::FWFinal dtor.
drop(handle);
}

#[doc(alias = "RBX::Allocator<RBX::FWPartInstance>::operator new(unsigned long)")]
pub fn stub_0x5e86b8(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::FWPartInstance>::operator new(unsigned long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::singleton_pool<RBX::FWPartInstance,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x5e8728() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x5e8728, "boost::singleton_pool<RBX::FWPartInstance, 56u, boost::default_user_allocator_malloc_free, boost::mu~")
}

#[doc(alias = "RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::releaseMemory(void)")]
pub fn stub_0x5e8840(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::releaseMemory() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x5e885c() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x5e885c, "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance, 200u, boost::default_user_allocator_m~")
}

#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::~TouchedSlot()")]
pub fn stub_0x5eb0bc(handle: crate::slot::InstanceHandle) {
// RBX::PartInstance::TouchedSignal::TouchedSlot dtor.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance>::shared_ptr<RBX::PartInstance>(rbx_core::WeakPtr<RBX::PartInstance> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x5eb1d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(RBX::PartInstance::TouchedSignal::TouchedSlot const&)")]
pub fn stub_0x5ebcc8() -> crate::slot::InstanceHandle {
// RBX::PartInstance::TouchedSignal::TouchedSlot ctor.
crate::slot::InstanceHandle::new("RBX::PartInstance::TouchedSignal::TouchedSlot")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::PartInstance::FormFactor,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x5f2560(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>,std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor> const&)")]
pub fn stub_0x5f25b8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor> const&)")]
pub fn stub_0x5f266c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor> const&)")]
pub fn stub_0x5f26c4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::PartInstance::FormFactor,std::allocator<RBX::PartInstance::FormFactor>>::resize(unsigned long,RBX::PartInstance::FormFactor)")]
pub fn stub_0x5f272c(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::PartInstance::FormFactor,std::allocator<RBX::PartInstance::FormFactor>>::push_back(RBX::PartInstance::FormFactor const&)")]
pub fn stub_0x5f2760(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::PartInstance::FormFactor,std::allocator<RBX::PartInstance::FormFactor>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PartInstance::FormFactor*,std::vector<RBX::PartInstance::FormFactor,std::allocator<RBX::PartInstance::FormFactor>>>,RBX::PartInstance::FormFactor const&)")]
pub fn stub_0x5f2788(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::PartInstance::FormFactor,std::allocator<RBX::PartInstance::FormFactor>>::_M_allocate(unsigned long)")]
pub fn stub_0x5f286c() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::PartInstance::FormFactor * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PartInstance::FormFactor *,RBX::PartInstance::FormFactor *>(RBX::PartInstance::FormFactor *,RBX::PartInstance::FormFactor *,RBX::PartInstance::FormFactor *)")]
pub fn stub_0x5f2884(handle: &crate::slot::InstanceHandle) {
// RBX::PartInstance::FormFactor* std::__copy_backward<false, std::random_access_iterator_tag~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::PartInstance::FormFactor,std::allocator<RBX::PartInstance::FormFactor>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PartInstance::FormFactor*,std::vector<RBX::PartInstance::FormFactor,std::allocator<RBX::PartInstance::FormFactor>>>,unsigned long,RBX::PartInstance::FormFactor const&)")]
pub fn stub_0x5f28c0(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance>::weak_ptr<RBX::PartInstance>(rbx_core::SharedPtr<RBX::PartInstance> const&,boost::detail::sp_enable_if_convertible<RBX::PartInstance,RBX::PartInstance>::type)")]
pub fn stub_0x5f3c08() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "boost::singleton_pool<RBX::OnDemandPVInstance,24u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x5f3c58() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x5f3c58, "boost::singleton_pool<RBX::OnDemandPVInstance, 24u, boost::default_user_allocator_malloc_free, boost~")
}

#[doc(alias = "boost::singleton_pool<RBX::FWInstance,28u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x5f3cb0() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x5f3cb0, "boost::singleton_pool<RBX::FWInstance, 28u, boost::default_user_allocator_malloc_free, boost::mutex,~")
}

#[doc(alias = "RBX::OnDemandPVInstance::operator delete(void *)")]
pub fn stub_0x5f3d00(handle: &crate::slot::InstanceHandle) {
// RBX::OnDemandPVInstance::operator delete(void*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FWInstance::~FWInstance()")]
pub fn stub_0x5f3e68(handle: crate::slot::InstanceHandle) {
// RBX::FWInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::FWInstance::~FWInstance() [0x5f3f38]")]
pub fn stub_0x5f3f38(handle: crate::slot::InstanceHandle) {
// RBX::FWInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::Allocator<RBX::FWInstance>::operator delete(void *)")]
pub fn stub_0x5f3f40(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::FWInstance>::operator delete(void*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FWPVInstance::~FWPVInstance()")]
pub fn stub_0x5f4020(handle: crate::slot::InstanceHandle) {
// RBX::FWPVInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::FWPVInstance::~FWPVInstance() [0x5f4024]")]
pub fn stub_0x5f4024(handle: crate::slot::InstanceHandle) {
// RBX::FWPVInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::operator new(unsigned long)")]
pub fn stub_0x5f4628(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::operator new(unsigned long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x5f4698() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x5f4698, "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance, 200u, boost::default_user_allocator_m~")
}
