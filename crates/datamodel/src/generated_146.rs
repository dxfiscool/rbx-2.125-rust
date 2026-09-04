// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX:: + Instance|DataModel|Workspace|Part|Model (13497 total, 9257 remaining) — EA-sorted asc next 100 true uncovered
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x3e1e8..0x395390 | datamodel 4140->4240 covered, workspace 25637->25737 (rbx_core::SharedPtr not boost)
// Shard: 146 EA-sorted asc true uncovered after existing shards

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
// 0x3e1e8 — __ZN5boost14singleton_poolIN3RBX16OnDemandInstanceELj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub use crate::instance::stub_0x3e1e8 as stub_3e1e8;
// 0x3f094 — __ZN10RobloxView9RenderJob16stepDataModelJobERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub use crate::instance::stub_0x3f094 as stub_3f094;
// 0x27a970 — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use crate::instance::stub_0x27a970 as stub_27a970;
// 0x27aa84 — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_gc(lua_State *)")]
pub use crate::instance::stub_0x27aa84 as stub_27aa84;
// 0x27aaac — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_eq(lua_State *)")]
pub use crate::instance::stub_0x27aaac as stub_27aaac;
// 0x27aae8 — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_tostring(lua_State *)")]
pub use crate::instance::stub_0x27aae8 as stub_27aae8;
// 0x27afcc — __ZNK3RBX3Lua13EventInstanceeqERKS1_
// type: bool __fastcall(_DWORD *, _DWORD *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Lua::EventInstance::operator==(RBX::Lua::EventInstance const&)const")]
pub use crate::instance::stub_0x27afcc as stub_27afcc;
// 0x27c004 — __ZN3RBX3Lua12ObjectBridge11newInstanceEP9lua_State
// type: int __fastcall(boost::detail::sp_counted_base *)
#[doc(alias = "RBX::Lua::ObjectBridge::newInstance(lua_State *)")]
pub use crate::instance::stub_0x27c004 as stub_27c004;
// 0x27c244 — __ZN3RBX3Lua12ObjectBridge12lockInstanceEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::ObjectBridge::lockInstance(lua_State *)")]
pub use crate::instance::stub_0x27c244 as stub_27c244;
// 0x27c254 — __ZN3RBX3Lua12ObjectBridge14unlockInstanceEP9lua_State
// type: int()
#[doc(alias = "RBX::Lua::ObjectBridge::unlockInstance(lua_State *)")]
pub use crate::instance::stub_0x27c254 as stub_27c254;
// 0x280bac — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "RBX::Lua::EventInstance* RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::pushNewObject<RBX::Lua::EventInstance>(lua_State *,RBX::Lua::EventInstance)")]
pub use crate::instance::stub_0x280bac as stub_280bac;
// 0x287acc — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_index(RBX::Lua::EventInstance const&,char const*,lua_State *)")]
pub use crate::instance::stub_0x287acc as stub_287acc;
// 0x28838c — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE11on_newindexERS2_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_newindex(RBX::Lua::EventInstance&,char const*,lua_State *)")]
pub use crate::instance::stub_0x28838c as stub_28838c;
// 0x28864c — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_tostring(RBX::Lua::EventInstance const&,lua_State *)")]
pub use crate::instance::stub_0x28864c as stub_28864c;
// 0x28ce40 — __ZN3RBX10BaseScript19computeNewWorkspaceEv
// type: _DWORD __fastcall(RBX::BaseScript *__hidden this)
#[doc(alias = "RBX::BaseScript::computeNewWorkspace(void)")]
pub use crate::instance::stub_0x28ce40 as stub_28ce40;
// 0x2a3ef4 — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_index(lua_State *)")]
pub use crate::instance::stub_0x2a3ef4 as stub_2a3ef4;
// 0x2a3f28 — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_newindex(lua_State *)")]
pub use crate::instance::stub_0x2a3f28 as stub_2a3f28;
// 0x2a4854 — __ZN3RBX3Lua12ObjectBridge28registerInstanceClassLibraryEP9lua_State
#[doc(alias = "RBX::Lua::ObjectBridge::registerInstanceClassLibrary(lua_State *)")]
pub use crate::instance::stub_0x2a4854 as stub_2a4854;
// 0x2c02a8 — __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub use crate::instance::stub_0x2c02a8 as stub_2c02a8;
// 0x2c433c — __ZN3RBX17WaitingScriptsJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::WaitingScriptsJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::WaitingScriptsJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub use crate::instance::stub_0x2c433c as stub_2c433c;
// 0x2c48c4 — __ZN3RBX5GcJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::GcJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::GcJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub use crate::instance::stub_0x2c48c4 as stub_2c48c4;
// 0x2ce618 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EEC2ERKS6_
#[doc(alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::vector(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::vector(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)
pub use crate::instance::stub_0x2ce618 as stub_2ce618;
// 0x2ce7d4 — __ZNSt12_Vector_baseIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EEC2EmRKS5_
#[doc(alias = "std::_Vector_base<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::_Vector_base(unsigned long,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>> const&)")]
// was: std::_Vector_base<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::_Vector_base(unsigned long,std::allocator<boost::weak_ptr<RBX::PartInstance>> const&)
pub use crate::instance::stub_0x2ce7d4 as stub_2ce7d4;
// 0x2cf178 — __ZN3RBX13AdvLuaDragger9mouseDownEN5boost10shared_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorINS1_8weak_ptrIS3_EESaISB_EE
#[doc(alias = "RBX::AdvLuaDragger::mouseDown(rbx_core::SharedPtr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>)")]
// was: RBX::AdvLuaDragger::mouseDown(boost::shared_ptr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>)
pub use crate::instance::stub_0x2cf178 as stub_2cf178;
// 0x2d0030 — __ZN3RBX13AdvLuaDragger15getSnapHitPointEPNS_12PartInstanceERKNS_6RbxRayERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::AdvLuaDragger *__hidden this, RBX::PartInstance *, const RBX::RbxRay *, G3D::Vector3 *)
#[doc(alias = "RBX::AdvLuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)")]
pub use crate::instance::stub_0x2d0030 as stub_2d0030;
// 0x2d4d38 — __ZN3RBX11AdvMoveTool20getGridXYUsingCameraEPNS_12PartInstanceERN3G3D7Vector3ES5_
// type: _DWORD __fastcall(RBX::AdvMoveTool *__hidden this, RBX::PartInstance *, G3D::Vector3 *, G3D::Vector3 *)
#[doc(alias = "RBX::AdvMoveTool::getGridXYUsingCamera(RBX::PartInstance *,G3D::Vector3 &,G3D::Vector3 &)")]
pub use crate::instance::stub_0x2d4d38 as stub_2d4d38;
// 0x2d5218 — __ZNSt3mapIN5boost8weak_ptrIN3RBX12PartInstanceEEEfSt4lessIS4_ESaISt4pairIKS4_fEEEixERS8_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "std::map<rbx_core::WeakPtr<RBX::PartInstance>,float,std::less<rbx_core::WeakPtr<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>>::operator[](rbx_core::WeakPtr<RBX::PartInstance> const&)")]
// was: std::map<boost::weak_ptr<RBX::PartInstance>,float,std::less<boost::weak_ptr<RBX::PartInstance>>,std::allocator<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>>::operator[](boost::weak_ptr<RBX::PartInstance> const&)
pub use crate::instance::stub_0x2d5218 as stub_2d5218;
// 0x2d5368 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::PartInstance>,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>,std::less<rbx_core::WeakPtr<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float> const&)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::PartInstance>,std::pair<boost::weak_ptr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>,std::less<boost::weak_ptr<RBX::PartInstance>>,std::allocator<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>,std::pair<boost::weak_ptr<RBX::PartInstance> const,float> const&)
pub use crate::instance::stub_0x2d5368 as stub_2d5368;
// 0x2d541c — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::PartInstance>,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>,std::less<rbx_core::WeakPtr<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float> const&)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::PartInstance>,std::pair<boost::weak_ptr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>,std::less<boost::weak_ptr<RBX::PartInstance>>,std::allocator<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<boost::weak_ptr<RBX::PartInstance> const,float> const&)
pub use crate::instance::stub_0x2d541c as stub_2d541c;
// 0x2d5468 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::PartInstance>,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>,std::less<rbx_core::WeakPtr<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>>::_M_insert_unique(std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float> const&)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::PartInstance>,std::pair<boost::weak_ptr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>,std::less<boost::weak_ptr<RBX::PartInstance>>,std::allocator<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>>::_M_insert_unique(std::pair<boost::weak_ptr<RBX::PartInstance> const,float> const&)
pub use crate::instance::stub_0x2d5468 as stub_2d5468;
// 0x2d54d0 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE14_M_create_nodeERKS7_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::PartInstance>,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>,std::less<rbx_core::WeakPtr<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>>::_M_create_node(std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float> const&)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::PartInstance>,std::pair<boost::weak_ptr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>,std::less<boost::weak_ptr<RBX::PartInstance>>,std::allocator<std::pair<boost::weak_ptr<RBX::PartInstance> const,float>>>::_M_create_node(std::pair<boost::weak_ptr<RBX::PartInstance> const,float> const&)
pub use crate::instance::stub_0x2d54d0 as stub_2d54d0;
// 0x2dc914 — __ZN3RBX7Dragger25intersectingWorldOrOthersERNS_12PartInstanceERNS_14ContactManagerEff
// type: _DWORD __fastcall(RBX::Dragger *__hidden this, RBX::PartInstance *, RBX::ContactManager *, float, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::Dragger::intersectingWorldOrOthers(RBX::PartInstance &,RBX::ContactManager &,float,float)")]
pub use crate::instance::stub_0x2dc914 as stub_2dc914;
// 0x2e0f38 — __ZN3RBX13DragUtilities13safeMoveYDropERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKN3G3D7Vector3ERNS_14ContactManagerEf
// type: void __fastcall(int, int, struct _Unwind_Exception *, int, float, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::safeMoveYDrop(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
// was: RBX::DragUtilities::safeMoveYDrop(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::Vector3 const&,RBX::ContactManager &,float)
pub use crate::instance::stub_0x2e0f38 as stub_2e0f38;
// 0x2e10d8 — __ZN3RBX13DragUtilities17partsToPrimitivesERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
// type: int __fastcall(__int64 *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::partsToPrimitives(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,G3D::Array<RBX::Primitive *,10,32ul> &)")]
// was: RBX::DragUtilities::partsToPrimitives(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::Array<RBX::Primitive *,10,32ul> &)
pub use crate::instance::stub_0x2e10d8 as stub_2e10d8;
// 0x2e1308 — __ZN3RBX13DragUtilities16hitObjectOrPlaneERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKNS_6RbxRayERKNS_14ContactManagerEb
// type: void __fastcall(int, __int64 *, int, struct _Unwind_Exception *lpuexcpt, int, struct _Unwind_Exception *lpuexcpta, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::hitObjectOrPlane(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,bool)")]
// was: RBX::DragUtilities::hitObjectOrPlane(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,bool)
pub use crate::instance::stub_0x2e1308 as stub_2e1308;
// 0x2e1628 — __ZN3RBX13DragUtilities9hitObjectERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKNS_6RbxRayERKNS_14ContactManagerERN3G3D7Vector3Eb
// type: int __fastcall(__int64 *, int, struct _Unwind_Exception *lpuexcpt, int, int, struct _Unwind_Exception *lpuexcpta, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::hitObject(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,G3D::Vector3 &,bool)")]
// was: RBX::DragUtilities::hitObject(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,G3D::Vector3 &,bool)
pub use crate::instance::stub_0x2e1628 as stub_2e1628;
// 0x2e1860 — __ZN3RBX13DragUtilities12anyPartAliveERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: int __fastcall(__int64 *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::anyPartAlive(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: RBX::DragUtilities::anyPartAlive(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)
pub use crate::instance::stub_0x2e1860 as stub_2e1860;
// 0x2e195c — __ZN3RBX13DragUtilities17partsToPrimitivesERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERS1_IPNS_9PrimitiveESaISB_EE
// type: int __fastcall(__int64 *, _DWORD *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::partsToPrimitives(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,std::vector&<RBX::Primitive *,std::allocator<RBX::Primitive>>)")]
// was: RBX::DragUtilities::partsToPrimitives(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,std::vector&<RBX::Primitive *,std::allocator<RBX::Primitive>>)
pub use crate::instance::stub_0x2e195c as stub_2e195c;
// 0x2e1b90 — __ZN3RBX13DragUtilities10pvsToPartsERKSt6vectorIPNS_10PVInstanceESaIS3_EERS1_IN5boost8weak_ptrINS_12PartInstanceEEESaISB_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::DragUtilities::pvsToParts(std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,std::vector&<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")]
// was: RBX::DragUtilities::pvsToParts(std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)
pub use crate::instance::stub_0x2e1b90 as stub_2e1b90;
// 0x2e1bf4 — __ZN3RBX13DragUtilities19unJoinFromOutsidersERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *)
#[doc(alias = "RBX::DragUtilities::unJoinFromOutsiders(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: RBX::DragUtilities::unJoinFromOutsiders(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)
pub use crate::instance::stub_0x2e1bf4 as stub_2e1bf4;
// 0x2e1cc0 — __ZN3RBX13DragUtilities15joinToOutsidersERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *)
#[doc(alias = "RBX::DragUtilities::joinToOutsiders(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: RBX::DragUtilities::joinToOutsiders(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)
pub use crate::instance::stub_0x2e1cc0 as stub_2e1cc0;
// 0x2e1d90 — __ZN3RBX13DragUtilities4joinERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::join(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: RBX::DragUtilities::join(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)
pub use crate::instance::stub_0x2e1d90 as stub_2e1d90;
// 0x2e1ed8 — __ZN3RBX13DragUtilities19joinWithInPartsOnlyERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *)
#[doc(alias = "RBX::DragUtilities::joinWithInPartsOnly(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: RBX::DragUtilities::joinWithInPartsOnly(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)
pub use crate::instance::stub_0x2e1ed8 as stub_2e1ed8;
// 0x2e1fa4 — __ZN3RBX13DragUtilities11setDraggingERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::setDragging(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: RBX::DragUtilities::setDragging(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)
pub use crate::instance::stub_0x2e1fa4 as stub_2e1fa4;
// 0x2e20f8 — __ZN3RBX13DragUtilities12stopDraggingERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::stopDragging(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: RBX::DragUtilities::stopDragging(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)
pub use crate::instance::stub_0x2e20f8 as stub_2e20f8;
// 0x2e2290 — __ZN3RBX13DragUtilities11alignToGridEPNS_12PartInstanceE
// type: int __fastcall(RBX::DragUtilities *this, RBX::PartInstance *)
#[doc(alias = "RBX::DragUtilities::alignToGrid(RBX::PartInstance *)")]
pub use crate::instance::stub_0x2e2290 as stub_2e2290;
// 0x2e2300 — __ZN3RBX13DragUtilities12moveAndCleanEPNS_12PartInstanceERKN3G3D7Vector3E
// type: int __fastcall(RBX::DragUtilities *this, RBX::PartInstance *, const G3D::Vector3 *)
#[doc(alias = "RBX::DragUtilities::moveAndClean(RBX::PartInstance *,G3D::Vector3 const&)")]
pub use crate::instance::stub_0x2e2300 as stub_2e2300;
// 0x2e23e4 — __ZN3RBX13DragUtilities5cleanEPNS_12PartInstanceE
// type: int __fastcall(RBX::DragUtilities *this, RBX::PartInstance *)
#[doc(alias = "RBX::DragUtilities::clean(RBX::PartInstance *)")]
pub use crate::instance::stub_0x2e23e4 as stub_2e23e4;
// 0x2e2400 — __ZN3RBX13DragUtilities5cleanERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::clean(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: RBX::DragUtilities::clean(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)
pub use crate::instance::stub_0x2e2400 as stub_2e2400;
// 0x2e24f0 — __ZN3RBX13DragUtilities4moveERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EEN3G3D15CoordinateFrameESB_
// type: void __fastcall(int *, _DWORD *, int)
#[doc(alias = "RBX::DragUtilities::move(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,G3D::CoordinateFrame,G3D::CoordinateFrame)")]
// was: RBX::DragUtilities::move(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::CoordinateFrame,G3D::CoordinateFrame)
pub use crate::instance::stub_0x2e24f0 as stub_2e24f0;
// 0x2e27ec — __ZN3RBX13DragUtilities14computeExtentsERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(int, __int64 *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::computeExtents(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: RBX::DragUtilities::computeExtents(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)
pub use crate::instance::stub_0x2e27ec as stub_2e27ec;
// 0x2e304c — __ZNK3RBX8GameTool13draggablePartEPKNS_12PartInstanceERKN3G3D7Vector3E
// type: int __fastcall(RBX::GameTool *this, const RBX::PartInstance *, const G3D::Vector3 *)
#[doc(alias = "RBX::GameTool::draggablePart(RBX::PartInstance const*,G3D::Vector3 const&)const")]
pub use crate::instance::stub_0x2e304c as stub_2e304c;
// 0x2e6070 — __ZN3RBX10LuaDragger9mouseDownEN5boost10shared_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorINS1_8weak_ptrIS3_EESaISB_EE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::LuaDragger::mouseDown(rbx_core::SharedPtr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>)")]
// was: RBX::LuaDragger::mouseDown(boost::shared_ptr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>)
pub use crate::instance::stub_0x2e6070 as stub_2e6070;
// 0x2e67a4 — __ZN3RBX10LuaDragger15getSnapHitPointEPNS_12PartInstanceERKNS_6RbxRayERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::LuaDragger *__hidden this, RBX::PartInstance *, const RBX::RbxRay *, G3D::Vector3 *)
#[doc(alias = "RBX::LuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)")]
pub use crate::instance::stub_0x2e67a4 as stub_2e67a4;
// 0x2e71b4 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EEaSERKS6_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::operator=(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::operator=(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)
pub use crate::instance::stub_0x2e71b4 as stub_2e71b4;
// 0x2e7eb4 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS4_S6_EEEEPS4_mT_SE_
// type: char *__fastcall(int, int, int, int)
#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance>* std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>>(unsigned long,__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>,__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>)")]
// was: boost::weak_ptr<RBX::PartInstance>* std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<boost::weak_ptr<RBX::PartInstance> const*,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>>>(unsigned long,__gnu_cxx::__normal_iterator<boost::weak_ptr<RBX::PartInstance> const*,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>>,__gnu_cxx::__normal_iterator<boost::weak_ptr<RBX::PartInstance> const*,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>>)
pub use crate::instance::stub_0x2e7eb4 as stub_2e7eb4;
// 0x2e8078 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost8weak_ptrIN3RBX12PartInstanceEEES8_EET0_T_SA_S9_
#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *>(rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *)")]
// was: boost::weak_ptr<RBX::PartInstance> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *>(boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *)
pub use crate::instance::stub_0x2e8078 as stub_2e8078;
// 0x2e80d0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN5boost8weak_ptrIN3RBX12PartInstanceEEEPS7_EET0_T_SC_SB_
#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance>* std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance>*>(rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance>*)")]
// was: boost::weak_ptr<RBX::PartInstance>* std::__copy<false,std::random_access_iterator_tag>::copy<boost::weak_ptr<RBX::PartInstance> const*,boost::weak_ptr<RBX::PartInstance>*>(boost::weak_ptr<RBX::PartInstance> const*,boost::weak_ptr<RBX::PartInstance> const*,boost::weak_ptr<RBX::PartInstance>*)
pub use crate::instance::stub_0x2e80d0 as stub_2e80d0;
// 0x2eaea0 — __ZN3RBX11MegaDraggerC1EPNS_12PartInstanceERKSt6vectorIPNS_10PVInstanceESaIS5_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
pub use crate::instance::stub_0x2eaea0 as stub_2eaea0;
// 0x2eaea4 — __ZN3RBX11MegaDraggerC2EPNS_12PartInstanceERKSt6vectorIPNS_10PVInstanceESaIS5_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
pub use crate::instance::stub_0x2eaea4 as stub_2eaea4;
// 0x2eafd4 — __ZN3RBX11MegaDraggerC1EPNS_12PartInstanceERKSt6vectorIN5boost8weak_ptrIS1_EESaIS6_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
// was: RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)
pub use crate::instance::stub_0x2eafd4 as stub_2eafd4;
// 0x2eafd8 — __ZN3RBX11MegaDraggerC2EPNS_12PartInstanceERKSt6vectorIN5boost8weak_ptrIS1_EESaIS6_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
// was: RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)
pub use crate::instance::stub_0x2eafd8 as stub_2eafd8;
// 0x2ef364 — __ZN3RBX11NewNullTool16getIndicatedPartERKNS_7UIEventERKbPPNS_12PartInstanceEPbPN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this, const RBX::UIEvent *, const bool *, RBX::PartInstance **, bool *, G3D::Vector3 *)
#[doc(alias = "RBX::NewNullTool::getIndicatedPart(RBX::UIEvent const&,bool const&,RBX::PartInstance **,bool *,G3D::Vector3 *)")]
pub use crate::instance::stub_0x2ef364 as stub_2ef364;
// 0x2f2f3c — __ZN3RBX10RunDragger11turnUprightEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::RunDragger::turnUpright(RBX::PartInstance *)")]
pub use crate::instance::stub_0x2f2f3c as stub_2f2f3c;
// 0x2f61c0 — __ZN3RBX13ArrowToolBase9findDecalEPNS_12PartInstanceERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, RBX::PartInstance *, const RBX::UIEvent *)
#[doc(alias = "RBX::ArrowToolBase::findDecal(RBX::PartInstance *,RBX::UIEvent const&)")]
pub use crate::instance::stub_0x2f61c0 as stub_2f61c0;
// 0x32305c — __ZN3RBX17HeartbeatInstance34onServiceProviderHeartbeatInstanceEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::HeartbeatInstance *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::HeartbeatInstance::onServiceProviderHeartbeatInstance(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub use crate::instance::stub_0x32305c as stub_32305c;
// 0x323238 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_17HeartbeatInstanceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>> const&)")]
pub use crate::instance::stub_0x323238 as stub_323238;
// 0x3232ac — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_17HeartbeatInstanceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>>::~callable_slot()")]
pub use crate::instance::stub_0x3232ac as stub_3232ac;
// 0x3232d8 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_17HeartbeatInstanceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>>::~callable_slot()")]
pub use crate::instance::stub_0x3232d8 as stub_3232d8;
// 0x3233ac — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
pub use crate::instance::stub_0x3233ac as stub_3233ac;
// 0x3233b4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
pub use crate::instance::stub_0x3233b4 as stub_3233b4;
// 0x3233bc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX17HeartbeatInstanceERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
pub use crate::instance::stub_0x3233bc as stub_3233bc;
// 0x3233d4 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
pub use crate::instance::stub_0x3233d4 as stub_3233d4;
// 0x323400 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
pub use crate::instance::stub_0x323400 as stub_323400;
// 0x3252f8 — __ZN3RBX18InterpolatedCFrame8setValueEPNS_12PartInstanceERKN3G3D15CoordinateFrameERKNS_10RemoteTimeE
#[doc(alias = "RBX::InterpolatedCFrame::setValue(RBX::PartInstance *,G3D::CoordinateFrame const&,RBX::RemoteTime const&)")]
pub use crate::instance::stub_0x3252f8 as stub_3252f8;
// 0x325998 — __ZN3RBX18InterpolatedCFrame12computeValueEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::InterpolatedCFrame *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::InterpolatedCFrame::computeValue(RBX::PartInstance *)")]
pub use crate::instance::stub_0x325998 as stub_325998;
// 0x369504 — __ZN3RBX13HeartbeatTask16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::HeartbeatTask *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::HeartbeatTask::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub use crate::instance::stub_0x369504 as stub_369504;
// 0x369ae0 — __ZN3RBX10PhysicsJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RBX::PhysicsJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::PhysicsJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub use crate::instance::stub_0x369ae0 as stub_369ae0;
// 0x36b370 — __ZN3RBX17HeartbeatInstanceD2Ev
// type: void __fastcall(RBX::HeartbeatInstance *__hidden this)
#[doc(alias = "RBX::HeartbeatInstance::~HeartbeatInstance()")]
pub use crate::instance::stub_0x36b370 as stub_36b370;
// 0x371250 — __ZN3RBX17HeartbeatInstanceD1Ev
// type: void __fastcall(RBX::HeartbeatInstance *__hidden this)
#[doc(alias = "RBX::HeartbeatInstance::~HeartbeatInstance()")]
pub use crate::instance::stub_0x371250 as stub_371250;
// 0x37eaa0 — __ZN3RBX10Soundscape12SoundService8SoundJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Soundscape::SoundService **this, const RBX::TaskScheduler::Job::Stats *, int, int (*)(const char *, ...))
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub use crate::instance::stub_0x37eaa0 as stub_37eaa0;
// 0x38f01c — __ZN3RBX12Accoutrement7dropAllEPNS_13ModelInstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::ModelInstance *, RBX::Accoutrement *)
#[doc(alias = "RBX::Accoutrement::dropAll(RBX::ModelInstance *)")]
pub use crate::instance::stub_0x38f01c as stub_38f01c;
// 0x38f024 — __ZN3RBX12Accoutrement13dropAllOthersEPNS_13ModelInstanceEPS0_
// type: RBX::Instance *__fastcall(RBX::Accoutrement *this, RBX::ModelInstance *, RBX::Accoutrement *)
#[doc(alias = "RBX::Accoutrement::dropAllOthers(RBX::ModelInstance *,RBX::Accoutrement*)")]
pub use crate::instance::stub_0x38f024 as stub_38f024;
// 0x38fb1c — __ZN3RBX12Accoutrement16upTo_InWorkspaceEv
// type: int __fastcall(RBX::Accoutrement *this, const RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::upTo_InWorkspace(void)")]
pub use crate::instance::stub_0x38fb1c as stub_38fb1c;
// 0x392738 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::~vector()")]
// was: std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::~vector()
pub use crate::instance::stub_0x392738 as stub_392738;
// 0x393b34 — __ZN3RBX13PartAdornment10setAdorneeEPNS_12PartInstanceE
// type: void __fastcall(RBX::PartAdornment *this, RBX::PartInstance *)
#[doc(alias = "RBX::PartAdornment::setAdornee(RBX::PartInstance *)")]
pub use crate::instance::stub_0x393b34 as stub_393b34;
// 0x393dd0 — __ZN3RBX11PVAdornment10setAdorneeEPNS_10PVInstanceE
// type: void __fastcall(RBX::PVAdornment *this, RBX::PVInstance *)
#[doc(alias = "RBX::PVAdornment::setAdornee(RBX::PVInstance *)")]
pub use crate::instance::stub_0x393dd0 as stub_393dd0;
// 0x394090 — __ZN3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::~RefPropDescriptor()")]
pub use crate::instance::stub_0x394090 as stub_394090;
// 0x3940e0 — __ZN3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::~RefPropDescriptor()")]
pub use crate::instance::stub_0x3940e0 as stub_3940e0;
// 0x39410c — __ZN3RBX11shared_fromINS_10PVInstanceEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::PVInstance> RBX::shared_from<RBX::PVInstance>(RBX::PVInstance*)")]
// was: boost::shared_ptr<RBX::PVInstance> RBX::shared_from<RBX::PVInstance>(RBX::PVInstance*)
pub use crate::instance::stub_0x39410c as stub_39410c;
// 0x394f78 — __ZN3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, char, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::RefPropDescriptor<RBX::PVInstance* (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance*)>(char const*,char const*,RBX::PVInstance* (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use crate::instance::stub_0x394f78 as stub_394f78;
// 0x39501c — __ZN3RBX10Reflection7RefTypeIPNS_10PVInstanceEE9singletonEv
// type: int *()
#[doc(alias = "RBX::Reflection::RefType<RBX::PVInstance *>::singleton(void)")]
pub use crate::instance::stub_0x39501c as stub_39501c;
// 0x395114 — __ZN3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::~RefPropDescriptor()")]
pub use crate::instance::stub_0x395114 as stub_395114;
// 0x395144 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::isReadOnly(void)const")]
pub use crate::instance::stub_0x395144 as stub_395144;
// 0x395154 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x395154 as stub_395154;
// 0x395164 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x395164 as stub_395164;
// 0x39518c — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub use crate::instance::stub_0x39518c as stub_39518c;
// 0x3952a4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub use crate::instance::stub_0x3952a4 as stub_3952a4;
// 0x39536c — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub use crate::instance::stub_0x39536c as stub_39536c;
// 0x395390 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub use crate::instance::stub_0x395390 as stub_395390;
