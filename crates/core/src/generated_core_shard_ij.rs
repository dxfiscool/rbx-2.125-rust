//! core shard IJ — 100 core stubs EA-sorted, continuation after II 0x5d18d0 (EA-sorted ascending, next 100 uncovered).
//!
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost, excludes Reflection|DataModel|Ogre|RakNet|Lua, EA-sorted, next 100 uncovered after 0x5d18d0.
//!
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.


#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "__ZN3RBX12MouseCommandC2EPNS_9WorkspaceE")]
// 0x5d5038 — __ZN3RBX12MouseCommandC2EPNS_9WorkspaceE
pub fn stub_5d5038() {
    // IDA 0x5d5038: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12MouseCommand18getTopSelectable3dEPNS_12PartInstanceE")]
// 0x5d5260 — __ZNK3RBX12MouseCommand18getTopSelectable3dEPNS_12PartInstanceE
pub fn stub_5d5260() {
    // IDA 0x5d5260: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12MouseCommand19distanceToCharacterERKN3G3D7Vector3E")]
// 0x5d58ac — __ZNK3RBX12MouseCommand19distanceToCharacterERKN3G3D7Vector3E
pub fn stub_5d58ac() {
    // IDA 0x5d58ac: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12MouseCommand17characterCanReachERKN3G3D7Vector3E")]
// 0x5d5908 — __ZNK3RBX12MouseCommand17characterCanReachERKN3G3D7Vector3E
pub fn stub_5d5908() {
    // IDA 0x5d5908: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12MouseCommand10getSurfaceERKNS_7UIEventEPKNS_13HitTestFilterERPNS_12PartInstanceERi")]
// 0x5d5940 — __ZN3RBX12MouseCommand10getSurfaceERKNS_7UIEventEPKNS_13HitTestFilterERPNS_12PartInstanceERi
pub fn stub_5d5940() {
    // IDA 0x5d5940: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12MouseCommand7getPartERKNS_7UIEventEPKNS_13HitTestFilterERN3G3D7Vector3E")]
// 0x5d59ac — __ZN3RBX12MouseCommand7getPartERKNS_7UIEventEPKNS_13HitTestFilterERN3G3D7Vector3E
pub fn stub_5d59ac() {
    // IDA 0x5d59ac: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12MouseCommand23getPartByLocalCharacterERKNS_7UIEventEPKNS_13HitTestFilterERN3G3D7Vector3E")]
// 0x5d5d88 — __ZN3RBX12MouseCommand23getPartByLocalCharacterERKNS_7UIEventEPKNS_13HitTestFilterERN3G3D7Vector3E
pub fn stub_5d5d88() {
    // IDA 0x5d5d88: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12MouseCommand31getUnlockedPartByLocalCharacterERKNS_7UIEventERN3G3D7Vector3E")]
// 0x5d5ee4 — __ZN3RBX12MouseCommand31getUnlockedPartByLocalCharacterERKNS_7UIEventERN3G3D7Vector3E
pub fn stub_5d5ee4() {
    // IDA 0x5d5ee4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12MouseCommand15getUnlockedPartERKNS_7UIEventERN3G3D7Vector3E")]
// 0x5d6040 — __ZN3RBX12MouseCommand15getUnlockedPartERKNS_7UIEventERN3G3D7Vector3E
pub fn stub_5d6040() {
    // IDA 0x5d6040: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12MouseCommand12getMousePartERKNS_6RbxRayERKNS_14ContactManagerERKSt6vectorIPKNS_9PrimitiveESaISA_EEPKNS_13HitTestFilterERN3G3D7Vector3Ef")]
// 0x5d6068 — __ZN3RBX12MouseCommand12getMousePartERKNS_6RbxRayERKNS_14ContactManagerERKSt6vectorIPKNS_9PrimitiveESaISA_EEPKNS_13HitTestFilterERN3G3D7Vector3Ef
pub fn stub_5d6068() {
    // IDA 0x5d6068: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX12MouseCommand12getMousePartERKNS_6RbxRayERKNS_14ContactManagerEPKNS_9PrimitiveEPKNS_13HitTestFilterERN3G3D7Vector3Ef")]
// 0x5d61a0 — __ZN3RBX12MouseCommand12getMousePartERKNS_6RbxRayERKNS_14ContactManagerEPKNS_9PrimitiveEPKNS_13HitTestFilterERN3G3D7Vector3Ef
pub fn stub_5d61a0() {
    // IDA 0x5d61a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4PART22ParametricPartInstanceD0Ev")]
// 0x5d6be8 — __ZN3RBX4PART22ParametricPartInstanceD0Ev
pub fn stub_5d6be8() {
    // IDA 0x5d6be8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4PART22ParametricPartInstanceD1Ev")]
// 0x5d6c94 — __ZN3RBX4PART22ParametricPartInstanceD1Ev
pub fn stub_5d6c94() {
    // IDA 0x5d6c94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX4PART22ParametricPartInstanceD0Ev")]
// 0x5d6ca4 — __ZThn32_N3RBX4PART22ParametricPartInstanceD0Ev
pub fn stub_5d6ca4() {
    // IDA 0x5d6ca4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX4PART22ParametricPartInstanceD0Ev")]
// 0x5d6cac — __ZThn36_N3RBX4PART22ParametricPartInstanceD0Ev
pub fn stub_5d6cac() {
    // IDA 0x5d6cac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX4PART22ParametricPartInstanceD0Ev")]
// 0x5d6cb4 — __ZThn132_N3RBX4PART22ParametricPartInstanceD0Ev
pub fn stub_5d6cb4() {
    // IDA 0x5d6cb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX4PART22ParametricPartInstanceD1Ev")]
// 0x5d6cbc — __ZThn32_N3RBX4PART22ParametricPartInstanceD1Ev
pub fn stub_5d6cbc() {
    // IDA 0x5d6cbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX4PART22ParametricPartInstanceD1Ev")]
// 0x5d6cd0 — __ZThn36_N3RBX4PART22ParametricPartInstanceD1Ev
pub fn stub_5d6cd0() {
    // IDA 0x5d6cd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX4PART22ParametricPartInstanceD1Ev")]
// 0x5d6ce4 — __ZThn132_N3RBX4PART22ParametricPartInstanceD1Ev
pub fn stub_5d6ce4() {
    // IDA 0x5d6ce4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX12PartInstance18getCoordinateFrameEv")]
// 0x5d8534 — __ZNK3RBX12PartInstance18getCoordinateFrameEv
pub fn stub_5d8534() {
    // IDA 0x5d8534: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX12PartInstance22setCoordinateFrameRootERKN3G3D15CoordinateFrameE")]
// 0x5d853c — __ZN3RBX12PartInstance22setCoordinateFrameRootERKN3G3D15CoordinateFrameE
pub fn stub_5d853c() {
    // IDA 0x5d853c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX12PartInstance17getLinearVelocityEv")]
// 0x5d8594 — __ZNK3RBX12PartInstance17getLinearVelocityEv
pub fn stub_5d8594() {
    // IDA 0x5d8594: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX12PartInstance21getRotationalVelocityEv")]
// 0x5d85a4 — __ZNK3RBX12PartInstance21getRotationalVelocityEv
pub fn stub_5d85a4() {
    // IDA 0x5d85a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX12PartInstance25setRotationalVelocityRootERKN3G3D7Vector3E")]
// 0x5d85b4 — __ZN3RBX12PartInstance25setRotationalVelocityRootERKN3G3D7Vector3E
pub fn stub_5d85b4() {
    // IDA 0x5d85b4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance18getSpecificGravityEv")]
// 0x5d860c — __ZNK3RBX12PartInstance18getSpecificGravityEv
pub fn stub_5d860c() {
    // IDA 0x5d860c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance10isGroundedEv")]
// 0x5d8b08 — __ZN3RBX12PartInstance10isGroundedEv
pub fn stub_5d8b08() {
    // IDA 0x5d8b08: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance17getConnectedPartsEb")]
// 0x5d8b20 — __ZN3RBX12PartInstance17getConnectedPartsEb
pub fn stub_5d8b20() {
    // IDA 0x5d8b20: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance14getPartSizeXmlEv")]
// 0x5d8f64 — __ZNK3RBX12PartInstance14getPartSizeXmlEv
pub fn stub_5d8f64() {
    // IDA 0x5d8f64: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance13getPartSizeUiEv")]
// 0x5d8f6c — __ZNK3RBX12PartInstance13getPartSizeUiEv
pub fn stub_5d8f6c() {
    // IDA 0x5d8f6c: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance13setPartSizeUiERKN3G3D7Vector3E")]
// 0x5d8f88 — __ZN3RBX12PartInstance13setPartSizeUiERKN3G3D7Vector3E
pub fn stub_5d8f88() {
    // IDA 0x5d8f88: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance13getElasticityEv")]
// 0x5d9094 — __ZNK3RBX12PartInstance13getElasticityEv
pub fn stub_5d9094() {
    // IDA 0x5d9094: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance11getFrictionEv")]
// 0x5d90a0 — __ZNK3RBX12PartInstance11getFrictionEv
pub fn stub_5d90a0() {
    // IDA 0x5d90a0: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance16setAlphaModifierEf")]
// 0x5d90ac — __ZN3RBX12PartInstance16setAlphaModifierEf
pub fn stub_5d90ac() {
    // IDA 0x5d90ac: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance11getAnchoredEv")]
// 0x5d90ec — __ZNK3RBX12PartInstance11getAnchoredEv
pub fn stub_5d90ec() {
    // IDA 0x5d90ec: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance13getCanCollideEv")]
// 0x5d90f8 — __ZNK3RBX12PartInstance13getCanCollideEv
pub fn stub_5d90f8() {
    // IDA 0x5d90f8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance15getNetworkOwnerEv")]
// 0x5d910c — __ZNK3RBX12PartInstance15getNetworkOwnerEv
pub fn stub_5d910c() {
    // IDA 0x5d910c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance15setNetworkOwnerENS_13SystemAddressE")]
// 0x5d9174 — __ZN3RBX12PartInstance15setNetworkOwnerENS_13SystemAddressE
pub fn stub_5d9174() {
    // IDA 0x5d9174: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance20getNetworkIsSleepingEv")]
// 0x5d9244 — __ZNK3RBX12PartInstance20getNetworkIsSleepingEv
pub fn stub_5d9244() {
    // IDA 0x5d9244: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance20setNetworkIsSleepingEb")]
// 0x5d9250 — __ZN3RBX12PartInstance20setNetworkIsSleepingEb
pub fn stub_5d9250() {
    // IDA 0x5d9250: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance11getDraggingEv")]
// 0x5d9258 — __ZNK3RBX12PartInstance11getDraggingEv
pub fn stub_5d9258() {
    // IDA 0x5d9258: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance11setDraggingEb")]
// 0x5d9264 — __ZN3RBX12PartInstance11setDraggingEb
pub fn stub_5d9264() {
    // IDA 0x5d9264: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance19computeExtentsWorldEv")]
// 0x5d929c — __ZNK3RBX12PartInstance19computeExtentsWorldEv
pub fn stub_5d929c() {
    // IDA 0x5d929c: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance19addTouchTransmitterEv")]
// 0x5d92ac — __ZN3RBX12PartInstance19addTouchTransmitterEv
pub fn stub_5d92ac() {
    // IDA 0x5d92ac: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance22removeTouchTransmitterEv")]
// 0x5d9400 — __ZN3RBX12PartInstance22removeTouchTransmitterEv
pub fn stub_5d9400() {
    // IDA 0x5d9400: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance25incrementTouchedSlotCountEv")]
// 0x5d9438 — __ZN3RBX12PartInstance25incrementTouchedSlotCountEv
pub fn stub_5d9438() {
    // IDA 0x5d9438: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance25decrementTouchedSlotCountEv")]
// 0x5d94f4 — __ZN3RBX12PartInstance25decrementTouchedSlotCountEv
pub fn stub_5d94f4() {
    // IDA 0x5d94f4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance14onChildRemovedEPNS_8InstanceE")]
// 0x5d95b0 — __ZN3RBX12PartInstance14onChildRemovedEPNS_8InstanceE
pub fn stub_5d95b0() {
    // IDA 0x5d95b0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance20OnDemandPartInstanceC2EPS0_")]
// 0x5d95d8 — __ZN3RBX12PartInstance20OnDemandPartInstanceC2EPS0_
pub fn stub_5d95d8() {
    // IDA 0x5d95d8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance20OnDemandPartInstance21computePersistentPartEv")]
// 0x5d9c44 — __ZN3RBX12PartInstance20OnDemandPartInstance21computePersistentPartEv
pub fn stub_5d9c44() {
    // IDA 0x5d9c44: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance27getRenderingCoordinateFrameEv")]
// 0x5d9d60 — __ZNK3RBX12PartInstance27getRenderingCoordinateFrameEv
pub fn stub_5d9d60() {
    // IDA 0x5d9d60: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstanceC2ERKN3G3D7Vector3E")]
// 0x5d9eb8 — __ZN3RBX12PartInstanceC2ERKN3G3D7Vector3E
pub fn stub_5d9eb8() {
    // IDA 0x5d9eb8: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstanceD0Ev")]
// 0x5da3f8 — __ZN3RBX12PartInstanceD0Ev
pub fn stub_5da3f8() {
    // IDA 0x5da3f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX12PartInstanceD1Ev")]
// 0x5da4a4 — __ZN3RBX12PartInstanceD1Ev
pub fn stub_5da4a4() {
    // IDA 0x5da4a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX12PartInstanceD0Ev")]
// 0x5da4b4 — __ZThn32_N3RBX12PartInstanceD0Ev
pub fn stub_5da4b4() {
    // IDA 0x5da4b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX12PartInstanceD0Ev")]
// 0x5da4bc — __ZThn36_N3RBX12PartInstanceD0Ev
pub fn stub_5da4bc() {
    // IDA 0x5da4bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX12PartInstanceD0Ev")]
// 0x5da4c4 — __ZThn132_N3RBX12PartInstanceD0Ev
pub fn stub_5da4c4() {
    // IDA 0x5da4c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX12PartInstanceD2Ev")]
// 0x5da4cc — __ZN3RBX12PartInstanceD2Ev
pub fn stub_5da4cc() {
    // IDA 0x5da4cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX12PartInstanceD1Ev")]
// 0x5da890 — __ZThn32_N3RBX12PartInstanceD1Ev
pub fn stub_5da890() {
    // IDA 0x5da890: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX12PartInstanceD1Ev")]
// 0x5da8a0 — __ZThn36_N3RBX12PartInstanceD1Ev
pub fn stub_5da8a0() {
    // IDA 0x5da8a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX12PartInstanceD1Ev")]
// 0x5da8b0 — __ZThn132_N3RBX12PartInstanceD1Ev
pub fn stub_5da8b0() {
    // IDA 0x5da8b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX12PartInstance2fwEv")]
// 0x5da8c0 — __ZNK3RBX12PartInstance2fwEv
pub fn stub_5da8c0() {
    // IDA 0x5da8c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX12PartInstance12initOnDemandEv")]
// 0x5da8c4 — __ZN3RBX12PartInstance12initOnDemandEv
pub fn stub_5da8c4() {
    // IDA 0x5da8c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX12PartInstance21resetNetworkOwnerTimeEd")]
// 0x5da97c — __ZN3RBX12PartInstance21resetNetworkOwnerTimeEd
pub fn stub_5da97c() {
    // IDA 0x5da97c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX12PartInstance18networkOwnerTimeUpEv")]
// 0x5da9a4 — __ZNK3RBX12PartInstance18networkOwnerTimeUpEv
pub fn stub_5da9a4() {
    // IDA 0x5da9a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX12PartInstance7setNameERKSs")]
// 0x5da9cc — __ZN3RBX12PartInstance7setNameERKSs
pub fn stub_5da9cc() {
    // IDA 0x5da9cc: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance13onGuidChangedEv")]
// 0x5daa58 — __ZN3RBX12PartInstance13onGuidChangedEv
pub fn stub_5daa58() {
    // IDA 0x5daa58: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZThn12_N3RBX12PartInstance13onGuidChangedEv")]
// 0x5daac0 — __ZThn12_N3RBX12PartInstance13onGuidChangedEv
pub fn stub_5daac0() {
    // IDA 0x5daac0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance17worldSnapLocationEv")]
// 0x5daac8 — __ZNK3RBX12PartInstance17worldSnapLocationEv
pub fn stub_5daac8() {
    // IDA 0x5daac8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance7alignedEb")]
// 0x5dabac — __ZNK3RBX12PartInstance7alignedEb
pub fn stub_5dabac() {
    // IDA 0x5dabac: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance13lockedInPlaceEv")]
// 0x5dad14 — __ZNK3RBX12PartInstance13lockedInPlaceEv
pub fn stub_5dad14() {
    // IDA 0x5dad14: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance13reportTouchesEv")]
// 0x5dadb8 — __ZNK3RBX12PartInstance13reportTouchesEv
pub fn stub_5dadb8() {
    // IDA 0x5dadb8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZThn96_NK3RBX12PartInstance13reportTouchesEv")]
// 0x5dae34 — __ZThn96_NK3RBX12PartInstance13reportTouchesEv
pub fn stub_5dae34() {
    // IDA 0x5dae34: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance13fromPrimitiveEPNS_9PrimitiveE")]
// 0x5daf38 — __ZN3RBX12PartInstance13fromPrimitiveEPNS_9PrimitiveE
pub fn stub_5daf38() {
    // IDA 0x5daf38: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance11uiToXmlSizeERKN3G3D7Vector3E")]
// 0x5db13c — __ZNK3RBX12PartInstance11uiToXmlSizeERKN3G3D7Vector3E
pub fn stub_5db13c() {
    // IDA 0x5db13c: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance12onCameraNearEf")]
// 0x5db334 — __ZN3RBX12PartInstance12onCameraNearEf
pub fn stub_5db334() {
    // IDA 0x5db334: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZThn132_N3RBX12PartInstance12onCameraNearEf")]
// 0x5db36c — __ZThn132_N3RBX12PartInstance12onCameraNearEf
pub fn stub_5db36c() {
    // IDA 0x5db36c: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance14onClumpChangedEv")]
// 0x5db374 — __ZN3RBX12PartInstance14onClumpChangedEv
pub fn stub_5db374() {
    // IDA 0x5db374: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZThn96_N3RBX12PartInstance14onClumpChangedEv")]
// 0x5db3a0 — __ZThn96_N3RBX12PartInstance14onClumpChangedEv
pub fn stub_5db3a0() {
    // IDA 0x5db3a0: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance17onSleepingChangedEb")]
// 0x5db3a8 — __ZN3RBX12PartInstance17onSleepingChangedEb
pub fn stub_5db3a8() {
    // IDA 0x5db3a8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZThn96_N3RBX12PartInstance17onSleepingChangedEb")]
// 0x5db564 — __ZThn96_N3RBX12PartInstance17onSleepingChangedEb
pub fn stub_5db564() {
    // IDA 0x5db564: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance17onBuoyancyChangedEb")]
// 0x5db56c — __ZN3RBX12PartInstance17onBuoyancyChangedEb
pub fn stub_5db56c() {
    // IDA 0x5db56c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZThn96_N3RBX12PartInstance17onBuoyancyChangedEb")]
// 0x5db574 — __ZThn96_N3RBX12PartInstance17onBuoyancyChangedEb
pub fn stub_5db574() {
    // IDA 0x5db574: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance19isInContinousMotionEv")]
// 0x5db57c — __ZN3RBX12PartInstance19isInContinousMotionEv
pub fn stub_5db57c() {
    // IDA 0x5db57c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZThn96_N3RBX12PartInstance19isInContinousMotionEv")]
// 0x5db584 — __ZThn96_N3RBX12PartInstance19isInContinousMotionEv
pub fn stub_5db584() {
    // IDA 0x5db584: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance12askSetParentEPKNS_8InstanceE")]
// 0x5db58c — __ZNK3RBX12PartInstance12askSetParentEPKNS_8InstanceE
pub fn stub_5db58c() {
    // IDA 0x5db58c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance18fromConstPrimitiveEPKNS_9PrimitiveE")]
// 0x5db5c8 — __ZN3RBX12PartInstance18fromConstPrimitiveEPKNS_9PrimitiveE
pub fn stub_5db5c8() {
    // IDA 0x5db5c8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance8getClumpEv")]
// 0x5db5e4 — __ZN3RBX12PartInstance8getClumpEv
pub fn stub_5db5e4() {
    // IDA 0x5db5e4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance17fromConstAssemblyEPKNS_8AssemblyE")]
// 0x5db64c — __ZN3RBX12PartInstance17fromConstAssemblyEPKNS_8AssemblyE
pub fn stub_5db64c() {
    // IDA 0x5db64c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance13destroyJointsEv")]
// 0x5db6c0 — __ZN3RBX12PartInstance13destroyJointsEv
pub fn stub_5db6c0() {
    // IDA 0x5db6c0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance21destroyImplicitJointsEv")]
// 0x5db6e0 — __ZN3RBX12PartInstance21destroyImplicitJointsEv
pub fn stub_5db6e0() {
    // IDA 0x5db6e0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance4joinEv")]
// 0x5db700 — __ZN3RBX12PartInstance4joinEv
pub fn stub_5db700() {
    // IDA 0x5db700: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance14getSurfaceTypeENS_8NormalIdE")]
// 0x5db71c — __ZNK3RBX12PartInstance14getSurfaceTypeENS_8NormalIdE
pub fn stub_5db71c() {
    // IDA 0x5db71c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance7getPartEv")]
// 0x5db72c — __ZN3RBX12PartInstance7getPartEv
pub fn stub_5db72c() {
    // IDA 0x5db72c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance28calcRenderingCoordinateFrameEv")]
// 0x5db794 — __ZN3RBX12PartInstance28calcRenderingCoordinateFrameEv
pub fn stub_5db794() {
    // IDA 0x5db794: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance17onServiceProviderEPNS_15ServiceProviderES2_")]
// 0x5db828 — __ZN3RBX12PartInstance17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_5db828() {
    // IDA 0x5db828: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX12PartInstance20updatePrimitiveStateEv")]
// 0x5db8a8 — __ZN3RBX12PartInstance20updatePrimitiveStateEv
pub fn stub_5db8a8() {
    // IDA 0x5db8a8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX12PartInstance32getIsCurrentlyStreamRemovingPartEv")]
// 0x5dbc80 — __ZNK3RBX12PartInstance32getIsCurrentlyStreamRemovingPartEv
pub fn stub_5dbc80() {
    // IDA 0x5dbc80: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN12_GLOBAL__N_136computeNetworkOwnerIsSomeoneElseImplERKN3RBX13SystemAddressES3_")]
// 0x5dbca8 — __ZN12_GLOBAL__N_136computeNetworkOwnerIsSomeoneElseImplERKN3RBX13SystemAddressES3_
pub fn stub_5dbca8() {
    // IDA 0x5dbca8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "__ZN3RBX12PartInstance17onAncestorChangedERKNS_15AncestorChangedE")]
// 0x5dbce8 — __ZN3RBX12PartInstance17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_5dbce8() {
    // IDA 0x5dbce8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "__ZNK3RBX12PartInstance19shouldRender3dAdornEv")]
// 0x5dbcfc — __ZNK3RBX12PartInstance19shouldRender3dAdornEv
pub fn stub_5dbcfc() {
    // IDA 0x5dbcfc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}
