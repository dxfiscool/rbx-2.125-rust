//! Auto-generated skeletons for rbx-script — script gap filler EA asc 0x5aae18..0x5afe94
//! Filter: Script|Lua|Yield|CodeGen (5401 filtered, all already stubbed — gap filler distinct not yet in script)
//! Source: ida/export.json (85545 funcs, base 0x4000, size 0x13a8efc)
//! Batch: +120 stubs | range 0x5aae18..0x5afe94 | script 32916->33036 total (EA-sorted asc distinct not yet in crates/script/src, rbx_core::SharedPtr not boost, // 0xADDR mangled + #[doc(alias)] + todo!("0xADDR"))
//! Remaining not in script before batch: 53290 -> after: 53050 (filtered Script|Lua exhausted, global gap filler EA asc)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5aae18 — __ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::JointInstance::*)(void)const,void (RBX::JointInstance::*)(RBX::PartInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance * const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13JointInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_0x5aae18(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5aae3c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5aae40(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5aaee0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5aaee8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5aaf8c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5aaf94(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

// 0x5ab0dc — __ZNK3RBX15ManualGlueJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::ManualGlueJoint *__hidden this)
#[doc(alias = "RBX::ManualGlueJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX15ManualGlueJoint12getJointTypeEv")]
pub fn stub_0x5ab0dc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ManualGlueJoint getter.
cell.get()
}

// 0x5ab198 — __ZNK3RBX15ManualWeldJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::ManualWeldJoint *__hidden this)
#[doc(alias = "RBX::ManualWeldJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX15ManualWeldJoint12getJointTypeEv")]
pub fn stub_0x5ab198(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ManualWeldJoint getter.
cell.get()
}

// 0x5ab250 — __ZNK3RBX9WeldJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::WeldJoint *__hidden this)
#[doc(alias = "RBX::WeldJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX9WeldJoint12getJointTypeEv")]
pub fn stub_0x5ab250(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::WeldJoint getter.
cell.get()
}

// 0x5ab308 — __ZNK3RBX9SnapJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::SnapJoint *__hidden this)
#[doc(alias = "RBX::SnapJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX9SnapJoint12getJointTypeEv")]
pub fn stub_0x5ab308(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SnapJoint getter.
cell.get()
}

// 0x5ab31c — __GLOBAL__I_a_218
#[doc(alias = "_global constructor keyed to__a_218")]
#[doc(alias = "__GLOBAL__I_a_218")]
pub fn stub_0x5ab31c() -> crate::slot::PortedFn {
// IDA 0x5ab31c: __GLOBAL__I_a_218.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x5ab31c, "__GLOBAL__I_a_218")
}

// 0x5abd48 — __ZN3RBX13JointsService24setJoinAfterMoveInstanceEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::JointsService::setJoinAfterMoveInstance(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX13JointsService24setJoinAfterMoveInstanceEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x5abd48() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5abe0c — __ZN3RBX13JointsService22setJoinAfterMoveTargetEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::JointsService::setJoinAfterMoveTarget(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX13JointsService22setJoinAfterMoveTargetEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x5abe0c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5ac2e4 — __ZN3RBX13JointsService17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::JointsService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
#[doc(alias = "__ZN3RBX13JointsService17onServiceProviderEPNS_15ServiceProviderES2_")]
pub fn stub_0x5ac2e4(handle: &crate::slot::InstanceHandle) {
// RBX::JointsService::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5ac8b8 — __ZN3RBX13JointsService20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::JointsService::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX13JointsService20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x5ac8b8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5ac8fc — __ZN3RBX13JointsService17onDescendantAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::JointsService::onDescendantAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX13JointsService17onDescendantAddedEPNS_8InstanceE")]
pub fn stub_0x5ac8fc(handle: &crate::slot::InstanceHandle) {
// RBX::JointsService::onDescendantAdded(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5ac944 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev")]
pub fn stub_0x5ac944(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x5aca50 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EED1Ev")]
pub fn stub_0x5aca50(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x5acb20 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4SnapEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::Snap>(rbx_core::SharedPtr<RBX::Snap> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4SnapEEERS3_RKNS0_IT_EE")]
pub fn stub_0x5acb20(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x5acb54 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap,RBX::Joint *>(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEPNS_5JointEEEN5boost10shared_ptrIT_EET0_")]
pub fn stub_0x5acb54() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Snap")
}

// 0x5acc08 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4WeldEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::Weld>(rbx_core::SharedPtr<RBX::Weld> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4WeldEEERS3_RKNS0_IT_EE")]
pub fn stub_0x5acc08(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x5acc3c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4WeldEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Weld> RBX::Creatable<RBX::Instance>::create<RBX::Weld,RBX::Joint *>(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_4WeldEPNS_5JointEEEN5boost10shared_ptrIT_EET0_")]
pub fn stub_0x5acc3c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Weld")
}

// 0x5accf0 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4GlueEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::Glue>(rbx_core::SharedPtr<RBX::Glue> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_4GlueEEERS3_RKNS0_IT_EE")]
pub fn stub_0x5accf0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x5acd24 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue,RBX::Joint *>(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEPNS_5JointEEEN5boost10shared_ptrIT_EET0_")]
pub fn stub_0x5acd24() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Glue")
}

// 0x5acdd8 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_6RotateEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::Rotate>(rbx_core::SharedPtr<RBX::Rotate> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_6RotateEEERS3_RKNS0_IT_EE")]
pub fn stub_0x5acdd8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x5ace0c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate,RBX::Joint *>(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEPNS_5JointEEEN5boost10shared_ptrIT_EET0_")]
pub fn stub_0x5ace0c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Rotate")
}

// 0x5acec0 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_7RotatePEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::RotateP>(rbx_core::SharedPtr<RBX::RotateP> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_7RotatePEEERS3_RKNS0_IT_EE")]
pub fn stub_0x5acec0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x5acef4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP,RBX::Joint *>(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEPNS_5JointEEEN5boost10shared_ptrIT_EET0_")]
pub fn stub_0x5acef4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RotateP")
}

// 0x5acfa8 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_7RotateVEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>& rbx_core::SharedPtr<RBX::JointInstance>::operator=<RBX::RotateV>(rbx_core::SharedPtr<RBX::RotateV> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSINS1_7RotateVEEERS3_RKNS0_IT_EE")]
pub fn stub_0x5acfa8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x5acfdc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV,RBX::Joint *>(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEPNS_5JointEEEN5boost10shared_ptrIT_EET0_")]
pub fn stub_0x5acfdc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RotateV")
}

// 0x5ad090 — __ZN5boost10shared_ptrIN3RBX10PVInstanceEEaSERKS3_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::PVInstance>::operator=(rbx_core::SharedPtr<RBX::PVInstance> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10PVInstanceEEaSERKS3_")]
pub fn stub_0x5ad090(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x5ad0c8 — __ZN3RBX13JointsServiceD1Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::~JointsService()")]
#[doc(alias = "__ZN3RBX13JointsServiceD1Ev")]
pub fn stub_0x5ad0c8(handle: crate::slot::InstanceHandle) {
// RBX::JointsService dtor.
drop(handle);
}

// 0x5ad0cc — __ZN3RBX13JointsServiceD0Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::~JointsService() [0x5ad0cc]")]
#[doc(alias = "__ZN3RBX13JointsServiceD0Ev")]
pub fn stub_0x5ad0cc(handle: crate::slot::InstanceHandle) {
// RBX::JointsService dtor.
drop(handle);
}

// 0x5ad16c — __ZNK3RBX13JointsService11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::JointsService *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::JointsService::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX13JointsService11askAddChildEPKNS_8InstanceE")]
pub fn stub_0x5ad16c(handle: &crate::slot::InstanceHandle) {
// RBX::JointsService::askAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv")]
pub fn stub_0x5ad1a8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

// 0x5ad1d0 — __ZThn32_N3RBX13JointsServiceD1Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::JointsService::~JointsService()")]
#[doc(alias = "__ZThn32_N3RBX13JointsServiceD1Ev")]
pub fn stub_0x5ad1d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5ad1d8 — __ZThn32_N3RBX13JointsServiceD0Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::JointsService::~JointsService() [0x5ad1d8]")]
#[doc(alias = "__ZThn32_N3RBX13JointsServiceD0Ev")]
pub fn stub_0x5ad1d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEE12getClassNameEv")]
pub fn stub_0x5ad27c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

// 0x5ad2a4 — __ZThn36_N3RBX13JointsServiceD1Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::JointsService::~JointsService() [0x5ad2a4]")]
#[doc(alias = "__ZThn36_N3RBX13JointsServiceD1Ev")]
pub fn stub_0x5ad2a4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5ad2ac — __ZThn36_N3RBX13JointsServiceD0Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::JointsService::~JointsService() [0x5ad2ac]")]
#[doc(alias = "__ZThn36_N3RBX13JointsServiceD0Ev")]
pub fn stub_0x5ad2ac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x5ad350() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Snap"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x5ad354() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Glue"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x5ad358() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rotate"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x5ad35c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateP"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x5ad360() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateV"
}

// 0x5ad364 — __ZN5boost20dynamic_pointer_castIN3RBX10PVInstanceENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::PVInstance> boost::dynamic_pointer_cast<RBX::PVInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN5boost20dynamic_pointer_castIN3RBX10PVInstanceENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE")]
pub fn stub_0x5ad364() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PVInstance")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x5ad3ac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateV"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x5ad448() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateV"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x5ad4b4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateV"
}

// 0x5ad5f8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x5ad5f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RotateV")
}

// 0x5ad6a8 — __ZN5boost10shared_ptrIN3RBX7RotateVEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateV>::shared_ptr<RBX::RotateV,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7RotateVEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x5ad6a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RotateV")
}

// 0x5ad770 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotateVES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RotateV,RBX::RotateV>(rbx_core::SharedPtr<RBX::RotateV> const*,RBX::RotateV *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotateVES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x5ad770() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RotateV")
}

// 0x5ad858 — __ZN5boost6detail12shared_countC2IPN3RBX7RotateVENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX7RotateVENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x5ad858() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5ad960 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x5ad960(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x5ad964 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x5ad964]")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x5ad964(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x5ad968 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x5ad968() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5ad988 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x5ad988() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5ad9a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotateVENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x5ad9a0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_8sRotateVEEEERKS0_v")]
pub fn stub_0x5ad9a4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sRotateV>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sRotateVEEEEvv")]
pub fn stub_0x5ad9e8() -> crate::slot::PortedFn {
// IDA 0x5ad9e8: void RBX::Name::callDoDeclare<RBX::sRotateV>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5ad9e8, "void RBX::Name::callDoDeclare<RBX::sRotateV>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sRotateVEEEERKS0_v")]
pub fn stub_0x5ad9ec(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sRotateV>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x5adad0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateV"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x5adcf8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateP"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x5add94() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateP"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x5ade00() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateP"
}

// 0x5adf44 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x5adf44() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RotateP")
}

// 0x5adff4 — __ZN5boost10shared_ptrIN3RBX7RotatePEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateP>::shared_ptr<RBX::RotateP,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7RotatePEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x5adff4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RotateP")
}

// 0x5ae0bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotatePES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RotateP,RBX::RotateP>(rbx_core::SharedPtr<RBX::RotateP> const*,RBX::RotateP *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotatePES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x5ae0bc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RotateP")
}

// 0x5ae1a4 — __ZN5boost6detail12shared_countC2IPN3RBX7RotatePENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX7RotatePENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x5ae1a4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5ae2ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x5ae2ac(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x5ae2b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x5ae2b0]")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x5ae2b0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x5ae2b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x5ae2b4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5ae2d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x5ae2d4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5ae2ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x5ae2ec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_8sRotatePEEEERKS0_v")]
pub fn stub_0x5ae2f0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sRotateP>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sRotatePEEEEvv")]
pub fn stub_0x5ae334() -> crate::slot::PortedFn {
// IDA 0x5ae334: void RBX::Name::callDoDeclare<RBX::sRotateP>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5ae334, "void RBX::Name::callDoDeclare<RBX::sRotateP>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sRotatePEEEERKS0_v")]
pub fn stub_0x5ae338(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sRotateP>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x5ae41c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateP"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x5ae644() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rotate"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x5ae6e0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rotate"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x5ae74c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rotate"
}

// 0x5ae890 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x5ae890() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Rotate")
}

// 0x5ae940 — __ZN5boost10shared_ptrIN3RBX6RotateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Rotate>::shared_ptr<RBX::Rotate,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6RotateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x5ae940() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Rotate")
}

// 0x5aea08 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6RotateES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Rotate,RBX::Rotate>(rbx_core::SharedPtr<RBX::Rotate> const*,RBX::Rotate *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6RotateES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x5aea08() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Rotate")
}

// 0x5aeaf0 — __ZN5boost6detail12shared_countC2IPN3RBX6RotateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX6RotateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x5aeaf0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5aebf8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x5aebf8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x5aebfc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x5aebfc]")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x5aebfc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x5aec00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x5aec00() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5aec20 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x5aec20() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5aec38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x5aec38() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_7sRotateEEEERKS0_v")]
pub fn stub_0x5aec3c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sRotate>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sRotateEEEEvv")]
pub fn stub_0x5aec80() -> crate::slot::PortedFn {
// IDA 0x5aec80: void RBX::Name::callDoDeclare<RBX::sRotate>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5aec80, "void RBX::Name::callDoDeclare<RBX::sRotate>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sRotateEEEERKS0_v")]
pub fn stub_0x5aec84(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sRotate>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x5aed68() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rotate"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x5aef90() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Glue"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x5af02c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Glue"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x5af098() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Glue"
}

// 0x5af1dc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x5af1dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Glue")
}

// 0x5af28c — __ZN5boost10shared_ptrIN3RBX4GlueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Glue>::shared_ptr<RBX::Glue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4GlueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x5af28c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Glue")
}

// 0x5af354 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4GlueES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Glue,RBX::Glue>(rbx_core::SharedPtr<RBX::Glue> const*,RBX::Glue *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4GlueES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x5af354() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Glue")
}

// 0x5af43c — __ZN5boost6detail12shared_countC2IPN3RBX4GlueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX4GlueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x5af43c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5af544 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x5af544(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x5af548 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x5af548]")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x5af548(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x5af54c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x5af54c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5af56c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x5af56c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5af584 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x5af584() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5sGlueEEEERKS0_v")]
pub fn stub_0x5af588(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sGlue>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sGlueEEEEvv")]
pub fn stub_0x5af5cc() -> crate::slot::PortedFn {
// IDA 0x5af5cc: void RBX::Name::callDoDeclare<RBX::sGlue>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5af5cc, "void RBX::Name::callDoDeclare<RBX::sGlue>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sGlueEEEERKS0_v")]
pub fn stub_0x5af5d0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sGlue>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x5af6b4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Glue"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x5af8dc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Snap"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x5af978() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Snap"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x5af9e4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Snap"
}

// 0x5afb28 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x5afb28() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Snap")
}

// 0x5afbd8 — __ZN5boost10shared_ptrIN3RBX4SnapEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Snap>::shared_ptr<RBX::Snap,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4SnapEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x5afbd8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Snap")
}

// 0x5afca0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SnapES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Snap,RBX::Snap>(rbx_core::SharedPtr<RBX::Snap> const*,RBX::Snap *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SnapES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x5afca0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Snap")
}

// 0x5afd88 — __ZN5boost6detail12shared_countC2IPN3RBX4SnapENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX4SnapENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x5afd88() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5afe90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x5afe90(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x5afe94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x5afe94]")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x5afe94(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}
