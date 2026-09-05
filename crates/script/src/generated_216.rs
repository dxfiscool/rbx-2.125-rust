// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x38e9e0..0x391798 | script 21952->22052 distinct (filler 0x38e9e0 asc, not-in-script 63793->63693)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement() [0x38e9e0]")]
pub fn stub_0x38e9e0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "RBX::Accoutrement::~Accoutrement() [0x38e9e8]")]
pub fn stub_0x38e9e8(handle: crate::slot::InstanceHandle) {
// RBX::Accoutrement dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement() [0x38ef1c]")]
pub fn stub_0x38ef1c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement() [0x38ef2c]")]
pub fn stub_0x38ef2c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement() [0x38ef3c]")]
pub fn stub_0x38ef3c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement() [0x38ef4c]")]
pub fn stub_0x38ef4c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "RBX::Accoutrement::onCameraNear(float)")]
pub fn stub_0x38ef5c(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::onCameraNear(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::onCameraNear(float)")]
pub fn stub_0x38ef98(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "RBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
pub fn stub_0x38efa0(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::render3dSelect(RBX::Adorn*, RBX::SelectState) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
pub fn stub_0x38f014(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 104, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 104);
}

#[doc(alias = "RBX::Accoutrement::dropAll(RBX::ModelInstance *)")]
pub fn stub_0x38f01c(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::dropAll(RBX::ModelInstance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::dropAllOthers(RBX::ModelInstance *,RBX::Accoutrement*)")]
pub fn stub_0x38f024(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::dropAllOthers(RBX::ModelInstance*, RBX::Accoutrement*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::getHandleConst(void)const")]
pub fn stub_0x38f054(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Accoutrement getter.
cell.get()
}

#[doc(alias = "RBX::Accoutrement::getLocation(void)")]
pub fn stub_0x38f1c4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Accoutrement getter.
cell.get()
}

#[doc(alias = "virtual thunk toRBX::Accoutrement::getLocation(void)")]
pub fn stub_0x38f1f8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

#[doc(alias = "RBX::Accoutrement::connectTouchEvent(void)")]
pub fn stub_0x38f20c(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::connectTouchEvent() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::onEvent_HandleTouched(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x38f3ec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Accoutrement::rebuildBackendState(void)")]
pub fn stub_0x38f47c(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::rebuildBackendState() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::computeDesiredState(void)")]
pub fn stub_0x38f4f4(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::computeDesiredState() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::setDesiredState(RBX::Accoutrement::AccoutrementState,RBX::ServiceProvider const*)")]
pub fn stub_0x38f578(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Accoutrement setter.
cell.set(value)
}

#[doc(alias = "RBX::Accoutrement::computeDesiredState(RBX::Instance *)")]
pub fn stub_0x38f6f0(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::computeDesiredState(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::upTo_Equipped(void)")]
pub fn stub_0x38f714(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::upTo_Equipped() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::upTo_InCharacter(void)")]
pub fn stub_0x38f92c(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::upTo_InCharacter() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::upTo_InWorkspace(void)")]
pub fn stub_0x38fb1c(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::upTo_InWorkspace() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::downFrom_Equipped(void)")]
pub fn stub_0x38fbcc(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::downFrom_Equipped() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::downFrom_HasHandle(void)")]
pub fn stub_0x38fd24(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::downFrom_HasHandle() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::onEvent_AddedBackend(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x38fd60() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Accoutrement::onEvent_RemovedBackend(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x38fe18() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Accoutrement::onChildAdded(RBX::Instance *)")]
pub fn stub_0x38ff34(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::onChildAdded(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::onChildRemoved(RBX::Instance *)")]
pub fn stub_0x38ff5c(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::onChildRemoved(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::onAncestorChanged(RBX::AncestorChanged const&)")]
pub fn stub_0x38ff84(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::onAncestorChanged(RBX::AncestorChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Hat::Hat(void)")]
pub fn stub_0x38fff0() -> crate::slot::InstanceHandle {
// RBX::Hat ctor.
crate::slot::InstanceHandle::new("RBX::Hat")
}

#[doc(alias = "RBX::Accoutrement::getAttachmentPoint(void)const")]
pub fn stub_0x3901bc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Accoutrement getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::CoordinateFrame>::~PropDescriptor()")]
pub fn stub_0x3901c0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,G3D::Vector3>::~PropDescriptor()")]
pub fn stub_0x3901e4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Accoutrement::getBackendAccoutrementState(void)const")]
pub fn stub_0x390208(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Accoutrement getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::~PropDescriptor()")]
pub fn stub_0x390210(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::CameraSubject * RBX::Instance::queryTypedChild<RBX::CameraSubject>(int)")]
pub fn stub_0x390234(handle: &crate::slot::InstanceHandle) {
// RBX::CameraSubject* RBX::Instance::queryTypedChild<RBX::CameraSubject>(int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)")]
pub fn stub_0x390270() -> crate::slot::SlotConnection {
// IDA 0x390270: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>> const&)")]
pub fn stub_0x3903f0() -> crate::slot::SlotConnection {
// IDA 0x3903f0: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")]
pub fn stub_0x390464() -> crate::slot::InstanceHandle {
// RBX::DescribedCreatable ctor.
crate::slot::InstanceHandle::new("RBX::DescribedCreatable")
}

#[doc(alias = "RBX::Accoutrement::askAddChild(RBX::Instance const*)const")]
pub fn stub_0x390654(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::askAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Accoutrement::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x390658(handle: &crate::slot::InstanceHandle) {
// RBX::Accoutrement::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E12getClassNameEv")]
pub fn stub_0x39065c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Accoutrement"
}

#[doc(alias = "RBX::Accoutrement::getRenderLocation(void)")]
pub fn stub_0x39066c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Accoutrement getter.
cell.get()
}

#[doc(alias = "RBX::Accoutrement::getRenderSize(void)")]
pub fn stub_0x39067c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Accoutrement getter.
cell.get()
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E12getClassNameEv")]
pub fn stub_0x3906a4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Accoutrement"
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::getRenderLocation(void)")]
pub fn stub_0x3906b4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::getRenderSize(void)")]
pub fn stub_0x3906c4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorD1Ev")]
pub fn stub_0x3906d0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Accoutrement"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x3906d4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hat"
}

#[doc(alias = "RBX::Hat::~Hat()")]
pub fn stub_0x3906d8(handle: crate::slot::InstanceHandle) {
// RBX::Hat dtor.
drop(handle);
}

#[doc(alias = "RBX::Hat::~Hat() [0x3906ec]")]
pub fn stub_0x3906ec(handle: crate::slot::InstanceHandle) {
// RBX::Hat dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x39079c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hat"
}

#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat()")]
pub fn stub_0x3907ac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat() [0x3907c0]")]
pub fn stub_0x3907c0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x390874() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hat"
}

#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat() [0x390884]")]
pub fn stub_0x390884(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat() [0x390898]")]
pub fn stub_0x390898(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat() [0x39094c]")]
pub fn stub_0x39094c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat() [0x390960]")]
pub fn stub_0x390960(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat() [0x390a14]")]
pub fn stub_0x390a14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat() [0x390a28]")]
pub fn stub_0x390a28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x390adc(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x390af0(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZThn128_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x390ba0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "__ZThn128_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x390bb4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x390c68(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x390c7c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn128_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x390d2c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "__ZThn128_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x390d40(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
pub fn stub_0x390df4(handle: crate::slot::InstanceHandle) {
// RBX::FactoryProduct dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
pub fn stub_0x390e08(handle: crate::slot::InstanceHandle) {
// RBX::FactoryProduct dtor.
drop(handle);
}

#[doc(alias = "__ZThn128_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
pub fn stub_0x390eb8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "__ZThn128_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
pub fn stub_0x390ecc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 128, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 128);
}

#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
pub fn stub_0x390ed4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
pub fn stub_0x390edc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
pub fn stub_0x390ee4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
pub fn stub_0x390eec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
pub fn stub_0x390f00(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
pub fn stub_0x390f14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x390f28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x390fdc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x391090(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x391144(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x391158(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39116c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x391180(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x391234(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3912e8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39139c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3913b0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3913c4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x3913d8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hat"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x39144c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hat"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_4sHatEEEEvv")]
pub fn stub_0x3914d4() -> crate::slot::PortedFn {
// IDA 0x3914d4: void RBX::Name::callDoDeclare<RBX::sHat>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3914d4, "void RBX::Name::callDoDeclare<RBX::sHat>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_4sHatEEEERKS0_v")]
pub fn stub_0x3914d8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sHat>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x3915b8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hat"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x391654() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hat"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Hat> RBX::Creatable<RBX::Instance>::create<RBX::Hat>(void)")]
pub fn stub_0x391798() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Hat")
}
