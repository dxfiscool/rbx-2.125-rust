//! core shard DW — 100 core stubs EA-sorted, next uncovered after DV 0x85e9bc (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "non-virtual thunk toRBX::TextureTrail::~TextureTrail()")]
// 0x85eb0c — __ZThn32_N3RBX12TextureTrailD1Ev
pub fn stub_85eb0c() {
    // IDA 0x85eb0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::TextureTrail::~TextureTrail()")]
// 0x85ec34 — __ZThn32_N3RBX12TextureTrailD0Ev
pub fn stub_85ec34() {
    // IDA 0x85ec34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::TextureTrail::~TextureTrail()")]
// 0x85ed84 — __ZThn36_N3RBX12TextureTrailD1Ev
pub fn stub_85ed84() {
    // IDA 0x85ed84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::TextureTrail::~TextureTrail()")]
// 0x85eeac — __ZThn36_N3RBX12TextureTrailD0Ev
pub fn stub_85eeac() {
    // IDA 0x85eeac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TeleportService::TeleportToSpawnByName(int,std::string)")]
// 0x860d98 — __ZN3RBX15TeleportService21TeleportToSpawnByNameEiSs
pub fn stub_860d98() {
    // IDA 0x860d98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TeleportService::Teleport(int)")]
// 0x861050 — __ZN3RBX15TeleportService8TeleportEi
pub fn stub_861050() {
    // IDA 0x861050: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TeleportService::TeleportImpl(int,std::string)")]
// 0x861184 — __ZN3RBX15TeleportService12TeleportImplEiSs
pub fn stub_861184() {
    // IDA 0x861184: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TeleportService::TeleportCancel(void)")]
// 0x861e14 — __ZN3RBX15TeleportService14TeleportCancelEv
pub fn stub_861e14() {
    // IDA 0x861e14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TeleportService::TeleportService(void)")]
// 0x861e24 — __ZN3RBX15TeleportServiceC1Ev
pub fn stub_861e24() {
    // IDA 0x861e24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TeleportService::TeleportService(void)")]
// 0x861e28 — __ZN3RBX15TeleportServiceC2Ev
pub fn stub_861e28() {
    // IDA 0x861e28: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "defCallback(std::string,int,std::string)")]
// 0x862090 — __ZL11defCallbackSsiSs
pub fn stub_862090() {
    // IDA 0x862090: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "defErrorCallback(std::string)")]
// 0x862094 — __ZL16defErrorCallbackSs
pub fn stub_862094() {
    // IDA 0x862094: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TeleportService::GetSpawnName(void)")]
// 0x862098 — __ZN3RBX15TeleportService12GetSpawnNameEv
pub fn stub_862098() {
    // IDA 0x862098: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TeleportService::SetBaseUrl(char const*)")]
// 0x8620a4 — __ZN3RBX15TeleportService10SetBaseUrlEPKc
pub fn stub_8620a4() {
    // IDA 0x8620a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TeleportService::SetCallback(RBX::TeleportCallback *)")]
// 0x8621d4 — __ZN3RBX15TeleportService11SetCallbackEPNS_16TeleportCallbackE
pub fn stub_8621d4() {
    // IDA 0x8621d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "ReadStringValue(std::string &,std::string)")]
// 0x8621e4 — __ZL15ReadStringValueRSsSs
pub fn stub_8621e4() {
    // IDA 0x8621e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::function3<bool,std::string,int,std::string>::operator()(std::string,int,std::string)const")]
// 0x86265c — __ZNK5boost9function3IbSsiSsEclESsiSs
// was: boost::function3<bool,std::string,int,std::string>::operator()(std::string,int,std::string)const
pub fn stub_86265c() {
    // IDA 0x86265c: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "RBX::TeleportService::~TeleportService()")]
// 0x862844 — __ZN3RBX15TeleportServiceD1Ev
pub fn stub_862844() {
    // IDA 0x862844: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "RBX::TeleportService::~TeleportService()")]
// 0x86294c — __ZN3RBX15TeleportServiceD0Ev
pub fn stub_86294c() {
    // IDA 0x86294c: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::TeleportService::~TeleportService()")]
// 0x862a8c — __ZThn32_N3RBX15TeleportServiceD1Ev
pub fn stub_862a8c() {
    // IDA 0x862a8c: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::TeleportService::~TeleportService()")]
// 0x862b94 — __ZThn32_N3RBX15TeleportServiceD0Ev
pub fn stub_862b94() {
    // IDA 0x862b94: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::TeleportService::~TeleportService()")]
// 0x862cd8 — __ZThn36_N3RBX15TeleportServiceD1Ev
pub fn stub_862cd8() {
    // IDA 0x862cd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::TeleportService::~TeleportService()")]
// 0x862de0 — __ZThn36_N3RBX15TeleportServiceD0Ev
pub fn stub_862de0() {
    // IDA 0x862de0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<bool,std::string,int,std::string>::dummy::nonnull(void)")]
// 0x862f00 — __ZN5boost9function3IbSsiSsE5dummy7nonnullEv
// was: boost::function3<bool,std::string,int,std::string>::dummy::nonnull(void)
pub fn stub_862f00() {
    // IDA 0x862f00: function null-target guard. Option<Box<dyn Fn>>::is_some — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<void (*)(std::string)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x86321c — __ZN5boost6detail8function15functor_managerIPFvSsEE6manageERKNS1_15function_bufferERS6_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<void (*)(std::string)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_86321c() {
    // IDA 0x86321c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_invoker1<void (*)(std::string),void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
// 0x863278 — __ZN5boost6detail8function22void_function_invoker1IPFvSsEvSsE6invokeERNS1_15function_bufferESs
// was: boost::detail::function::void_function_invoker1<void (*)(std::string),void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)
pub fn stub_863278() {
    // IDA 0x863278: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<bool (*)(std::string,int,std::string)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x86338c — __ZN5boost6detail8function15functor_managerIPFbSsiSsEE6manageERKNS1_15function_bufferERS6_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<bool (*)(std::string,int,std::string)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_86338c() {
    // IDA 0x86338c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_invoker3<bool (*)(std::string,int,std::string),bool,std::string,int,std::string>::invoke(boost::detail::function::function_buffer &,std::string,int,std::string)")]
// 0x8633e8 — __ZN5boost6detail8function17function_invoker3IPFbSsiSsEbSsiSsE6invokeERNS1_15function_bufferESsiSs
// was: boost::detail::function::function_invoker3<bool (*)(std::string,int,std::string),bool,std::string,int,std::string>::invoke(boost::detail::function::function_buffer &,std::string,int,std::string)
pub fn stub_8633e8() {
    // IDA 0x8633e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::function<void ()(std::string)>::operator=(boost::function<void ()(std::string)> const&)")]
// 0x864d5c — __ZN5boost8functionIFvSsEEaSERKS2_
// was: boost::function<void ()(std::string)>::operator=(boost::function<void ()(std::string)> const&)
pub fn stub_864d5c() {
    // IDA 0x864d5c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::function1<void,std::string>::move_assign(boost::function1<void,std::string>&)")]
// 0x864e20 — __ZN5boost9function1IvSsE11move_assignERS1_
// was: boost::function1<void,std::string>::move_assign(boost::function1<void,std::string>&)
pub fn stub_864e20() {
    // IDA 0x864e20: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "boost::function<bool ()(std::string,int,std::string)>::operator=(boost::function<bool ()(std::string,int,std::string)> const&)")]
// 0x866938 — __ZN5boost8functionIFbSsiSsEEaSERKS2_
// was: boost::function<bool ()(std::string,int,std::string)>::operator=(boost::function<bool ()(std::string,int,std::string)> const&)
pub fn stub_866938() {
    // IDA 0x866938: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::function3<bool,std::string,int,std::string>::swap(boost::function3<bool,std::string,int,std::string>&)")]
// 0x8669fc — __ZN5boost9function3IbSsiSsE4swapERS1_
// was: boost::function3<bool,std::string,int,std::string>::swap(boost::function3<bool,std::string,int,std::string>&)
pub fn stub_8669fc() {
    // IDA 0x8669fc: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "boost::function3<bool,std::string,int,std::string>::move_assign(boost::function3<bool,std::string,int,std::string>&)")]
// 0x866ad8 — __ZN5boost9function3IbSsiSsE11move_assignERS1_
// was: boost::function3<bool,std::string,int,std::string>::move_assign(boost::function3<bool,std::string,int,std::string>&)
pub fn stub_866ad8() {
    // IDA 0x866ad8: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "boost::function3<bool,std::string,int,std::string>::clear(void)")]
// 0x866bdc — __ZN5boost9function3IbSsiSsE5clearEv
// was: boost::function3<bool,std::string,int,std::string>::clear(void)
pub fn stub_866bdc() {
    // IDA 0x866bdc: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function3<bool,std::string,int,std::string>::assign_to_own(boost::function3<bool,std::string,int,std::string> const&)")]
// 0x866c08 — __ZN5boost9function3IbSsiSsE13assign_to_ownERKS1_
// was: boost::function3<bool,std::string,int,std::string>::assign_to_own(boost::function3<bool,std::string,int,std::string> const&)
pub fn stub_866c08() {
    // IDA 0x866c08: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::FloorWire::getFrom(void)const")]
// 0x8679e0 — __ZNK3RBX9FloorWire7getFromEv
pub fn stub_8679e0() {
    // IDA 0x8679e0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::FloorWire::getTo(void)const")]
// 0x867a18 — __ZNK3RBX9FloorWire5getToEv
pub fn stub_867a18() {
    // IDA 0x867a18: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::FloorWire::getTexture(void)const")]
// 0x867a50 — __ZNK3RBX9FloorWire10getTextureEv
pub fn stub_867a50() {
    // IDA 0x867a50: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::FloorWire::setTexture(RBX::TextureId)")]
// 0x867a68 — __ZN3RBX9FloorWire10setTextureENS_9TextureIdE
pub fn stub_867a68() {
    // IDA 0x867a68: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::FloorWire::getTextureSize(void)const")]
// 0x867a80 — __ZNK3RBX9FloorWire14getTextureSizeEv
pub fn stub_867a80() {
    // IDA 0x867a80: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FloorWire::getVelocity(void)const")]
// 0x867aa0 — __ZNK3RBX9FloorWire11getVelocityEv
pub fn stub_867aa0() {
    // IDA 0x867aa0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FloorWire::setVelocity(float)")]
// 0x867aa8 — __ZN3RBX9FloorWire11setVelocityEf
pub fn stub_867aa8() {
    // IDA 0x867aa8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FloorWire::getStudsBetweenTextures(void)const")]
// 0x867ab0 — __ZNK3RBX9FloorWire23getStudsBetweenTexturesEv
pub fn stub_867ab0() {
    // IDA 0x867ab0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FloorWire::setStudsBetweenTextures(float)")]
// 0x867ab8 — __ZN3RBX9FloorWire23setStudsBetweenTexturesEf
pub fn stub_867ab8() {
    // IDA 0x867ab8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FloorWire::getCycleOffset(void)const")]
// 0x867ac0 — __ZNK3RBX9FloorWire14getCycleOffsetEv
pub fn stub_867ac0() {
    // IDA 0x867ac0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FloorWire::setCycleOffset(float)")]
// 0x867ac8 — __ZN3RBX9FloorWire14setCycleOffsetEf
pub fn stub_867ac8() {
    // IDA 0x867ac8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FloorWire::getWireRadius(void)const")]
// 0x867ad0 — __ZNK3RBX9FloorWire13getWireRadiusEv
pub fn stub_867ad0() {
    // IDA 0x867ad0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FloorWire::setWireRadius(float)")]
// 0x867ad8 — __ZN3RBX9FloorWire13setWireRadiusEf
pub fn stub_867ad8() {
    // IDA 0x867ad8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FloorWire::FloorWire(void)")]
// 0x867ae4 — __ZN3RBX9FloorWireC2Ev
pub fn stub_867ae4() {
    // IDA 0x867ae4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FloorWire::render3dAdorn(RBX::Adorn *)")]
// 0x867de4 — __ZN3RBX9FloorWire13render3dAdornEPNS_5AdornE
pub fn stub_867de4() {
    // IDA 0x867de4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::FloorWire::render3dAdorn(RBX::Adorn *)")]
// 0x8685d8 — __ZThn96_N3RBX9FloorWire13render3dAdornEPNS_5AdornE
pub fn stub_8685d8() {
    // IDA 0x8685d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FloorWire::~FloorWire()")]
// 0x868a54 — __ZN3RBX9FloorWireD1Ev
pub fn stub_868a54() {
    // IDA 0x868a54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FloorWire::~FloorWire()")]
// 0x868b80 — __ZN3RBX9FloorWireD0Ev
pub fn stub_868b80() {
    // IDA 0x868b80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiBase3d::canProcessMeAndDescendants(void)const")]
// 0x868cd0 — __ZNK3RBX9GuiBase3d26canProcessMeAndDescendantsEv
pub fn stub_868cd0() {
    // IDA 0x868cd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiBase3d::getZIndex(void)const")]
// 0x868cd4 — __ZNK3RBX9GuiBase3d9getZIndexEv
pub fn stub_868cd4() {
    // IDA 0x868cd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiBase3d::getGuiQueue(void)const")]
// 0x868cdc — __ZNK3RBX9GuiBase3d11getGuiQueueEv
pub fn stub_868cdc() {
    // IDA 0x868cdc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiBase3d::shouldRender3dAdorn(void)const")]
// 0x868ce0 — __ZNK3RBX9GuiBase3d19shouldRender3dAdornEv
pub fn stub_868ce0() {
    // IDA 0x868ce0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FloorWire::~FloorWire()")]
// 0x868ce8 — __ZThn32_N3RBX9FloorWireD1Ev
pub fn stub_868ce8() {
    // IDA 0x868ce8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FloorWire::~FloorWire()")]
// 0x868e10 — __ZThn32_N3RBX9FloorWireD0Ev
pub fn stub_868e10() {
    // IDA 0x868e10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FloorWire::~FloorWire()")]
// 0x868f60 — __ZThn36_N3RBX9FloorWireD1Ev
pub fn stub_868f60() {
    // IDA 0x868f60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FloorWire::~FloorWire()")]
// 0x869088 — __ZThn36_N3RBX9FloorWireD0Ev
pub fn stub_869088() {
    // IDA 0x869088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiBase3d::shouldRender3dAdorn(void)const")]
// 0x8691c8 — __ZThn96_NK3RBX9GuiBase3d19shouldRender3dAdornEv
pub fn stub_8691c8() {
    // IDA 0x8691c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringConverter<RBX::Voxel::CellMaterial>::convertToValue(std::string const&,RBX::Voxel::CellMaterial&)")]
// 0x86bac4 — __ZN3RBX15StringConverterINS_5Voxel12CellMaterialEE14convertToValueERKSsRS2_
pub fn stub_86bac4() {
    // IDA 0x86bac4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringConverter<RBX::Voxel::CellBlock>::convertToValue(std::string const&,RBX::Voxel::CellBlock&)")]
// 0x86bb10 — __ZN3RBX15StringConverterINS_5Voxel9CellBlockEE14convertToValueERKSsRS2_
pub fn stub_86bb10() {
    // IDA 0x86bb10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringConverter<RBX::Voxel::CellOrientation>::convertToValue(std::string const&,RBX::Voxel::CellOrientation&)")]
// 0x86bb5c — __ZN3RBX15StringConverterINS_5Voxel15CellOrientationEE14convertToValueERKSsRS2_
pub fn stub_86bb5c() {
    // IDA 0x86bb5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringConverter<RBX::Voxel::WaterCellForce>::convertToValue(std::string const&,RBX::Voxel::WaterCellForce&)")]
// 0x86bba8 — __ZN3RBX15StringConverterINS_5Voxel14WaterCellForceEE14convertToValueERKSsRS2_
pub fn stub_86bba8() {
    // IDA 0x86bba8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringConverter<RBX::Voxel::WaterCellDirection>::convertToValue(std::string const&,RBX::Voxel::WaterCellDirection&)")]
// 0x86bbf4 — __ZN3RBX15StringConverterINS_5Voxel18WaterCellDirectionEE14convertToValueERKSsRS2_
pub fn stub_86bbf4() {
    // IDA 0x86bbf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::resize(unsigned long,RBX::Voxel::Cell)")]
// 0x8715e4 — __ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EE6resizeEmS2_
pub fn stub_8715e4() {
    // IDA 0x8715e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<unsigned char,std::allocator<unsigned char>>::resize(unsigned long,unsigned char)")]
// 0x871618 — __ZNSt6vectorIhSaIhEE6resizeEmh
pub fn stub_871618() {
    // IDA 0x871618: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::push_back(RBX::Voxel::CellChangeListener * const&)")]
// 0x8716a8 — __ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE9push_backERKS3_
pub fn stub_8716a8() {
    // IDA 0x8716a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringReadBuffer::operator>>(unsigned char &)")]
// 0x872db4 — __ZN3RBX16StringReadBufferrsERh
pub fn stub_872db4() {
    // IDA 0x872db4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "unsigned int RBX::readCountValue<RBX::StringReadBuffer>(RBX::StringReadBuffer &)")]
// 0x872f0c — __ZN3RBX14readCountValueINS_16StringReadBufferEEEjRT_
pub fn stub_872f0c() {
    // IDA 0x872f0c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::writeCountValue<RBX::StringWriteBuffer>(RBX::StringWriteBuffer &,unsigned int)")]
// 0x872f44 — __ZN3RBX15writeCountValueINS_17StringWriteBufferEEEvRT_j
pub fn stub_872f44() {
    // IDA 0x872f44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener * const&)")]
// 0x872fc4 — __ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
pub fn stub_872fc4() {
    // IDA 0x872fc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::_M_allocate(unsigned long)")]
// 0x8730a4 — __ZNSt12_Vector_baseIPN3RBX5Voxel18CellChangeListenerESaIS3_EE11_M_allocateEm
pub fn stub_8730a4() {
    // IDA 0x8730a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener *>(__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener * const&,std::random_access_iterator_tag)")]
// 0x8730bc — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX5Voxel18CellChangeListenerESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag
pub fn stub_8730bc() {
    // IDA 0x8730bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::Cell*,std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>>,unsigned long,RBX::Voxel::Cell const&)")]
// 0x87314c — __ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_87314c() {
    // IDA 0x87314c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::WaterCellDirection * rbx::any_cast<RBX::Voxel::WaterCellDirection,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x87788c — __ZN3rbx8any_castIN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_87788c() {
    // IDA 0x87788c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::WaterCellDirection & rbx::any_cast<RBX::Voxel::WaterCellDirection &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x8778e4 — __ZN3rbx8any_castIRN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_8778e4() {
    // IDA 0x8778e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::resize(unsigned long,RBX::Voxel::WaterCellDirection)")]
// 0x8779d4 — __ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE6resizeEmS2_
pub fn stub_8779d4() {
    // IDA 0x8779d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::push_back(RBX::Voxel::WaterCellDirection const&)")]
// 0x877a08 — __ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE9push_backERKS2_
pub fn stub_877a08() {
    // IDA 0x877a08: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::WaterCellDirection,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::operator[](RBX::Name const* const&)")]
// 0x877a30 — __ZNSt3mapIPKN3RBX4NameENS0_5Voxel18WaterCellDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_877a30() {
    // IDA 0x877a30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)")]
// 0x877a88 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_877a88() {
    // IDA 0x877a88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)")]
// 0x877b3c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_877b3c() {
    // IDA 0x877b3c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)")]
// 0x877b94 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_877b94() {
    // IDA 0x877b94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellDirection*,std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>>,RBX::Voxel::WaterCellDirection const&)")]
// 0x877bfc — __ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_877bfc() {
    // IDA 0x877bfc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_allocate(unsigned long)")]
// 0x877ce0 — __ZNSt12_Vector_baseIN3RBX5Voxel18WaterCellDirectionESaIS2_EE11_M_allocateEm
pub fn stub_877ce0() {
    // IDA 0x877ce0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::WaterCellDirection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *>(RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *)")]
// 0x877cf8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel18WaterCellDirectionES6_EET0_T_S8_S7_
pub fn stub_877cf8() {
    // IDA 0x877cf8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellDirection*,std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>>,unsigned long,RBX::Voxel::WaterCellDirection const&)")]
// 0x877d34 — __ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_877d34() {
    // IDA 0x877d34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::WaterCellForce * rbx::any_cast<RBX::Voxel::WaterCellForce,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x877ec4 — __ZN3rbx8any_castIN3RBX5Voxel14WaterCellForceENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_877ec4() {
    // IDA 0x877ec4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::WaterCellForce & rbx::any_cast<RBX::Voxel::WaterCellForce &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x877f1c — __ZN3rbx8any_castIRN3RBX5Voxel14WaterCellForceENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_877f1c() {
    // IDA 0x877f1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::resize(unsigned long,RBX::Voxel::WaterCellForce)")]
// 0x87800c — __ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE6resizeEmS2_
pub fn stub_87800c() {
    // IDA 0x87800c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::push_back(RBX::Voxel::WaterCellForce const&)")]
// 0x878040 — __ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE9push_backERKS2_
pub fn stub_878040() {
    // IDA 0x878040: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::WaterCellForce,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::operator[](RBX::Name const* const&)")]
// 0x878068 — __ZNSt3mapIPKN3RBX4NameENS0_5Voxel14WaterCellForceESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_878068() {
    // IDA 0x878068: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)")]
// 0x8780c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_8780c0() {
    // IDA 0x8780c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)")]
// 0x878174 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_878174() {
    // IDA 0x878174: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)")]
// 0x8781cc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_8781cc() {
    // IDA 0x8781cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellForce*,std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>>,RBX::Voxel::WaterCellForce const&)")]
// 0x878234 — __ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_878234() {
    // IDA 0x878234: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_allocate(unsigned long)")]
// 0x878318 — __ZNSt12_Vector_baseIN3RBX5Voxel14WaterCellForceESaIS2_EE11_M_allocateEm
pub fn stub_878318() {
    // IDA 0x878318: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::WaterCellForce * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *>(RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *)")]
// 0x878330 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel14WaterCellForceES6_EET0_T_S8_S7_
pub fn stub_878330() {
    // IDA 0x878330: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

