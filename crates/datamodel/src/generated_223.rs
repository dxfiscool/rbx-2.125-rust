// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted asc gap filler not yet in crates/datamodel/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x430b28..0x439cec | total filtered 10215, remaining 2191->2071 after batch, datamodel distinct 18845->18965
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
// 0x430b28 — __ZN3RBX9DataModel20scoped_write_requestD1Ev
// type: void __fastcall(RBX::DataModel::scoped_write_request *__hidden this)
#[doc(alias = "RBX::DataModel::scoped_write_request::~scoped_write_request()")]
pub use crate::instance::stub_0x430b28 as stub_430b28;
// 0x430b2c — __ZN3RBX9DataModel20scoped_write_requestD2Ev
// type: void __fastcall(RBX::DataModel::scoped_write_request *__hidden this)
#[doc(alias = "RBX::DataModel::scoped_write_request::~scoped_write_request()")]
pub use crate::instance::stub_0x430b2c as stub_430b2c;
// 0x430c18 — __ZN3RBX9DataModel19scoped_read_requestC1EPNS_8InstanceE
// type: int __fastcall(RBX::DataModel::scoped_read_request *this, RBX::Instance *)
#[doc(alias = "RBX::DataModel::scoped_read_request::scoped_read_request(RBX::Instance *)")]
pub use crate::instance::stub_0x430c18 as stub_430c18;
// 0x430c1c — __ZN3RBX9DataModel19scoped_read_requestC2EPNS_8InstanceE
// type: RBX::DataModel::scoped_read_request *__fastcall(RBX::DataModel::scoped_read_request *this, RBX::Instance *)
#[doc(alias = "RBX::DataModel::scoped_read_request::scoped_read_request(RBX::Instance *)")]
pub use crate::instance::stub_0x430c1c as stub_430c1c;
// 0x430d0c — __ZN3RBX9DataModel19scoped_read_requestD1Ev
// type: void __fastcall(RBX::DataModel::scoped_read_request *__hidden this)
#[doc(alias = "RBX::DataModel::scoped_read_request::~scoped_read_request()")]
pub use crate::instance::stub_0x430d0c as stub_430d0c;
// 0x430d10 — __ZN3RBX9DataModel19scoped_read_requestD2Ev
// type: void __fastcall(RBX::DataModel::scoped_read_request *__hidden this)
#[doc(alias = "RBX::DataModel::scoped_read_request::~scoped_read_request()")]
pub use crate::instance::stub_0x430d10 as stub_430d10;
// 0x430df4 — __ZN3RBX9DataModel24allHackFlagsOredTogetherEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::allHackFlagsOredTogether(void)")]
pub use crate::instance::stub_0x430df4 as stub_430df4;
// 0x431268 — __ZN5boost8functionIFvPN3RBX9DataModelEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "boost::function<void ()(RBX::DataModel *)>::~function()")]
pub use crate::instance::stub_0x431268 as stub_431268;
// 0x431278 — __ZN3RBX9DataModel11loadPluginsEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::loadPlugins(void)")]
pub use crate::instance::stub_0x431278 as stub_431278;
// 0x431288 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x431288 as stub_431288;
// 0x4312ac — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::~EventDesc()
pub use crate::instance::stub_0x4312ac as stub_4312ac;
// 0x4312d0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()
pub use crate::instance::stub_0x4312d0 as stub_4312d0;
// 0x431310 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()
pub use crate::instance::stub_0x431310 as stub_431310;
// 0x431350 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x431350 as stub_431350;
// 0x431390 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x431390 as stub_431390;
// 0x4313d0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvbELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x4313d0 as stub_4313d0;
// 0x431410 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x431410 as stub_431410;
// 0x431434 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x431434 as stub_431434;
// 0x431474 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()")]
pub use crate::instance::stub_0x431474 as stub_431474;
// 0x4314b4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::~BoundYieldFuncDesc()")]
pub use crate::instance::stub_0x4314b4 as stub_4314b4;
// 0x4314fc — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x4314fc as stub_4314fc;
// 0x431544 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x431544 as stub_431544;
// 0x431594 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()
pub use crate::instance::stub_0x431594 as stub_431594;
// 0x4315b8 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x4315b8 as stub_4315b8;
// 0x431618 — __ZNK3RBX9DataModel19getIsPersonalServerEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getIsPersonalServer(void)const")]
pub use crate::instance::stub_0x431618 as stub_431618;
// 0x431620 — __ZN3RBX9DataModel19setIsPersonalServerEb
// type: int __fastcall(int this, bool)
#[doc(alias = "RBX::DataModel::setIsPersonalServer(bool)")]
pub use crate::instance::stub_0x431620 as stub_431620;
// 0x431628 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEbED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::~PropDescriptor()")]
pub use crate::instance::stub_0x431628 as stub_431628;
// 0x43164c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::~BoundYieldFuncDesc()")]
pub use crate::instance::stub_0x43164c as stub_43164c;
// 0x431768 — __ZN3RBX9DataModel22setUiMessageBrickCountEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::setUiMessageBrickCount(void)")]
pub use crate::instance::stub_0x431768 as stub_431768;
// 0x43177c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x43177c as stub_43177c;
// 0x4317c4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x4317c4 as stub_4317c4;
// 0x431804 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x431804 as stub_431804;
// 0x43184c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x43184c as stub_43184c;
// 0x431894 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x431894 as stub_431894;
// 0x4318d4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x4318d4 as stub_4318d4;
// 0x43191c — __ZNK3RBX9DataModel12getWorkspaceEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getWorkspace(void)const")]
pub use crate::instance::stub_0x43191c as stub_43191c;
// 0x431924 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::~RefPropDescriptor()")]
pub use crate::instance::stub_0x431924 as stub_431924;
// 0x431950 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::~RefPropDescriptor()")]
pub use crate::instance::stub_0x431950 as stub_431950;
// 0x43197c — __ZNK3RBX9DataModel10getPlaceIDEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getPlaceID(void)const")]
pub use crate::instance::stub_0x43197c as stub_43197c;
// 0x431984 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::~PropDescriptor()")]
pub use crate::instance::stub_0x431984 as stub_431984;
// 0x4319a8 — __ZNK3RBX9DataModel15getPlaceVersionEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getPlaceVersion(void)const")]
pub use crate::instance::stub_0x4319a8 as stub_4319a8;
// 0x4319b0 — __ZNK3RBX9DataModel12getCreatorIDEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getCreatorID(void)const")]
pub use crate::instance::stub_0x4319b0 as stub_4319b0;
// 0x4319b8 — __ZNK3RBX9DataModel14getCreatorTypeEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getCreatorType(void)const")]
pub use crate::instance::stub_0x4319b8 as stub_4319b8;
// 0x4319c0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::~EnumPropDescriptor()")]
pub use crate::instance::stub_0x4319c0 as stub_4319c0;
// 0x4319e4 — __ZNK3RBX9DataModel8getGenreEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getGenre(void)const")]
pub use crate::instance::stub_0x4319e4 as stub_4319e4;
// 0x4319ec — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::~EnumPropDescriptor()")]
pub use crate::instance::stub_0x4319ec as stub_4319ec;
// 0x431a10 — __ZNK3RBX9DataModel19getGearGenreSettingEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getGearGenreSetting(void)const")]
pub use crate::instance::stub_0x431a10 as stub_431a10;
// 0x431a18 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::~EnumPropDescriptor()")]
pub use crate::instance::stub_0x431a18 as stub_431a18;
// 0x431a3c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x431a3c as stub_431a3c;
// 0x431a7c — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::~EventDesc()")]
pub use crate::instance::stub_0x431a7c as stub_431a7c;
// 0x431aa0 — __ZNK3RBX9DataModel8getJobIdEv
// type: int __fastcall(RBX::DataModel *this, int)
#[doc(alias = "RBX::DataModel::getJobId(void)const")]
pub use crate::instance::stub_0x431aa0 as stub_431aa0;
// 0x431ab0 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelESsED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::~PropDescriptor()")]
pub use crate::instance::stub_0x431ab0 as stub_431ab0;
// 0x431ad4 — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::~EventDesc()")]
pub use crate::instance::stub_0x431ad4 as stub_431ad4;
// 0x431af8 — __ZN3RBX9DataModel15getIsGameLoadedEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getIsGameLoaded(void)")]
pub use crate::instance::stub_0x431af8 as stub_431af8;
// 0x431b00 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::~BoundFuncDesc()")]
pub use crate::instance::stub_0x431b00 as stub_431b00;
// 0x431b48 — __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::addPair(RBX::DataModel::CreatorType,char const*)")]
pub use crate::instance::stub_0x431b48 as stub_431b48;
// 0x431ea8 — __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel11CreatorTypeEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::DataModel::CreatorType & RBX::Reflection::Variant::genericConvert<RBX::DataModel::CreatorType>(void)")]
pub use crate::instance::stub_0x431ea8 as stub_431ea8;
// 0x432094 — __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::addPair(RBX::DataModel::Genre,char const*)")]
pub use crate::instance::stub_0x432094 as stub_432094;
// 0x4323f4 — __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel5GenreEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::DataModel::Genre & RBX::Reflection::Variant::genericConvert<RBX::DataModel::Genre>(void)")]
pub use crate::instance::stub_0x4323f4 as stub_4323f4;
// 0x4325e0 — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::addPair(RBX::DataModel::GearGenreSetting,char const*)")]
pub use crate::instance::stub_0x4325e0 as stub_4325e0;
// 0x432940 — __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel16GearGenreSettingEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::DataModel::GearGenreSetting & RBX::Reflection::Variant::genericConvert<RBX::DataModel::GearGenreSetting>(void)")]
pub use crate::instance::stub_0x432940 as stub_432940;
// 0x432b2c — __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::addPair(RBX::DataModel::GearType,char const*)")]
pub use crate::instance::stub_0x432b2c as stub_432b2c;
// 0x432e8c — __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel8GearTypeEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::DataModel::GearType & RBX::Reflection::Variant::genericConvert<RBX::DataModel::GearType>(void)")]
pub use crate::instance::stub_0x432e8c as stub_432e8c;
// 0x433078 — __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::addPair(RBX::Instance::SaveFilter,char const*)")]
pub use crate::instance::stub_0x433078 as stub_433078;
// 0x4333d8 — __ZN3RBX10Reflection7Variant14genericConvertINS_8Instance10SaveFilterEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::Instance::SaveFilter & RBX::Reflection::Variant::genericConvert<RBX::Instance::SaveFilter>(void)")]
pub use crate::instance::stub_0x4333d8 as stub_4333d8;
// 0x4335c4 — __ZN3RBX15ServiceProvider4findINS_5VisitEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Visit * RBX::ServiceProvider::find<RBX::Visit>(RBX::Instance const*)")]
pub use crate::instance::stub_0x4335c4 as stub_4335c4;
// 0x4335e0 — __ZN5boost10shared_ptrIN3RBX9DataModel10GenericJobEEaSERKS4_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel::GenericJob>::operator=(rbx_core::SharedPtr<RBX::DataModel::GenericJob> const&)")]
// was: boost::shared_ptr<RBX::DataModel::GenericJob>::operator=(boost::shared_ptr<RBX::DataModel::GenericJob> const&)
pub use crate::instance::stub_0x4335e0 as stub_4335e0;
// 0x433618 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9DataModelEPNS_4VerbEPS4_EEN5boost10shared_ptrIT_EET0_T1_
// type: void __fastcall(int, RBX::Verb *, RBX::DataModel *)
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel> RBX::Creatable<RBX::Instance>::create<RBX::DataModel,RBX::Verb *,RBX::DataModel*>(RBX::Verb *,RBX::DataModel*)")]
// was: boost::shared_ptr<RBX::DataModel> RBX::Creatable<RBX::Instance>::create<RBX::DataModel,RBX::Verb *,RBX::DataModel*>(RBX::Verb *,RBX::DataModel*)
pub use crate::instance::stub_0x433618 as stub_433618;
// 0x4337d0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9WorkspaceEPNS_9DataModelEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Workspace> RBX::Creatable<RBX::Instance>::create<RBX::Workspace,RBX::DataModel *>(RBX::DataModel *)")]
// was: boost::shared_ptr<RBX::Workspace> RBX::Creatable<RBX::Instance>::create<RBX::Workspace,RBX::DataModel *>(RBX::DataModel *)
pub use crate::instance::stub_0x4337d0 as stub_4337d0;
// 0x43388c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7GuiRootEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiRoot> RBX::Creatable<RBX::Instance>::create<RBX::GuiRoot>(void)")]
// was: boost::shared_ptr<RBX::GuiRoot> RBX::Creatable<RBX::Instance>::create<RBX::GuiRoot>(void)
pub use crate::instance::stub_0x43388c as stub_43388c;
// 0x434398 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9PlayerHUDEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::PlayerHUD> RBX::Creatable<RBX::Instance>::create<RBX::PlayerHUD>(void)")]
// was: boost::shared_ptr<RBX::PlayerHUD> RBX::Creatable<RBX::Instance>::create<RBX::PlayerHUD>(void)
pub use crate::instance::stub_0x434398 as stub_434398;
// 0x434edc — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_9DataModelES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::DataModel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::DataModel*>,boost::arg<1>>> const&)")]
pub use crate::instance::stub_0x434edc as stub_434edc;
// 0x436fa0 — __ZN3rbx22timestamped_safe_queueIN5boost8functionIFvPN3RBX9DataModelEEEEE4pushERKS7_
// type: void __fastcall(int)
#[doc(alias = "rbx::timestamped_safe_queue<boost::function<void ()(RBX::DataModel *)>>::push(boost::function<void ()(RBX::DataModel *)> const&)")]
pub use crate::instance::stub_0x436fa0 as stub_436fa0;
// 0x4377f8 — __ZNK3RBX8Instance22countDescendantsOfTypeIS0_EEiv
// type: int __fastcall(const shared_count *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "int RBX::Instance::countDescendantsOfType<RBX::Instance>(void)const")]
pub use crate::instance::stub_0x4377f8 as stub_4377f8;
// 0x437914 — __ZNK3RBX8Instance22countDescendantsOfTypeINS_12PartInstanceEEEiv
// type: int __fastcall(const shared_count *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "int RBX::Instance::countDescendantsOfType<RBX::PartInstance>(void)const")]
pub use crate::instance::stub_0x437914 as stub_437914;
// 0x437a30 — __ZNK3RBX8Instance22countDescendantsOfTypeINS_10BaseScriptEEEiv
// type: int __fastcall(const shared_count *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "int RBX::Instance::countDescendantsOfType<RBX::BaseScript>(void)const")]
pub use crate::instance::stub_0x437a30 as stub_437a30;
// 0x4387f0 — __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::~EnumDesc()")]
pub use crate::instance::stub_0x4387f0 as stub_4387f0;
// 0x4387f4 — __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::~EnumDesc()")]
pub use crate::instance::stub_0x4387f4 as stub_4387f4;
// 0x438894 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::lookup(char const*)const")]
pub use crate::instance::stub_0x438894 as stub_438894;
// 0x4388c4 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::lookup(RBX::Reflection::Variant const&)const")]
pub use crate::instance::stub_0x4388c4 as stub_4388c4;
// 0x4388e4 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub use crate::instance::stub_0x4388e4 as stub_4388e4;
// 0x438918 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToString(unsigned long,std::string &)const")]
pub use crate::instance::stub_0x438918 as stub_438918;
// 0x438a5c — __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::~EnumDesc()")]
pub use crate::instance::stub_0x438a5c as stub_438a5c;
// 0x438a60 — __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::~EnumDesc()")]
pub use crate::instance::stub_0x438a60 as stub_438a60;
// 0x438b00 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::lookup(char const*)const")]
pub use crate::instance::stub_0x438b00 as stub_438b00;
// 0x438b30 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::lookup(RBX::Reflection::Variant const&)const")]
pub use crate::instance::stub_0x438b30 as stub_438b30;
// 0x438b50 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub use crate::instance::stub_0x438b50 as stub_438b50;
// 0x438b84 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToString(unsigned long,std::string &)const")]
pub use crate::instance::stub_0x438b84 as stub_438b84;
// 0x438cc8 — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::~EnumDesc()")]
pub use crate::instance::stub_0x438cc8 as stub_438cc8;
// 0x438ccc — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::~EnumDesc()")]
pub use crate::instance::stub_0x438ccc as stub_438ccc;
// 0x438d6c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::lookup(char const*)const")]
pub use crate::instance::stub_0x438d6c as stub_438d6c;
// 0x438d9c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::lookup(RBX::Reflection::Variant const&)const")]
pub use crate::instance::stub_0x438d9c as stub_438d9c;
// 0x438dbc — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub use crate::instance::stub_0x438dbc as stub_438dbc;
// 0x438df0 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToString(unsigned long,std::string &)const")]
pub use crate::instance::stub_0x438df0 as stub_438df0;
// 0x438f34 — __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::~EnumDesc()")]
pub use crate::instance::stub_0x438f34 as stub_438f34;
// 0x438f38 — __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::~EnumDesc()")]
pub use crate::instance::stub_0x438f38 as stub_438f38;
// 0x438fd8 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::lookup(char const*)const")]
pub use crate::instance::stub_0x438fd8 as stub_438fd8;
// 0x439008 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::lookup(RBX::Reflection::Variant const&)const")]
pub use crate::instance::stub_0x439008 as stub_439008;
// 0x439028 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub use crate::instance::stub_0x439028 as stub_439028;
// 0x43905c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToString(unsigned long,std::string &)const")]
pub use crate::instance::stub_0x43905c as stub_43905c;
// 0x4391a0 — __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::~EnumDesc()")]
pub use crate::instance::stub_0x4391a0 as stub_4391a0;
// 0x4391a4 — __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::~EnumDesc()")]
pub use crate::instance::stub_0x4391a4 as stub_4391a4;
// 0x439244 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::lookup(char const*)const")]
pub use crate::instance::stub_0x439244 as stub_439244;
// 0x439274 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::lookup(RBX::Reflection::Variant const&)const")]
pub use crate::instance::stub_0x439274 as stub_439274;
// 0x439294 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub use crate::instance::stub_0x439294 as stub_439294;
// 0x4392c8 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToString(unsigned long,std::string &)const")]
pub use crate::instance::stub_0x4392c8 as stub_4392c8;
// 0x43940c — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToString(RBX::Instance::SaveFilter const&)const")]
pub use crate::instance::stub_0x43940c as stub_43940c;
// 0x4395ac — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Instance10SaveFilterEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Instance::SaveFilter>(RBX::Instance::SaveFilter const&)")]
pub use crate::instance::stub_0x4395ac as stub_4395ac;
// 0x4395fc — __ZN3rbx14implementation12typed_holderIN3RBX8Instance10SaveFilterEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Instance::SaveFilter>::singleton(void)")]
pub use crate::instance::stub_0x4395fc as stub_4395fc;
// 0x439668 — __ZN3rbx14implementation12typed_holderIN3RBX8Instance10SaveFilterEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Instance::SaveFilter>::construct_func(char const*,char *)")]
pub use crate::instance::stub_0x439668 as stub_439668;
// 0x439674 — __ZN3rbx14implementation12typed_holderIN3RBX8Instance10SaveFilterEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Instance::SaveFilter>::destruct_func(char *)")]
pub use crate::instance::stub_0x439674 as stub_439674;
// 0x439678 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToItem(RBX::Instance::SaveFilter const&)const")]
pub use crate::instance::stub_0x439678 as stub_439678;
// 0x439744 — __ZN3rbx8any_castIRKN3RBX8Instance10SaveFilterENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Instance::SaveFilter const& rbx::any_cast<RBX::Instance::SaveFilter const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub use crate::instance::stub_0x439744 as stub_439744;
// 0x439834 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToValue(RBX::Name const&,RBX::Instance::SaveFilter&)const")]
pub use crate::instance::stub_0x439834 as stub_439834;
// 0x4398b0 — __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::~EnumDesc()")]
pub use crate::instance::stub_0x4398b0 as stub_4398b0;
// 0x439a84 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToString(RBX::DataModel::GearType const&)const")]
pub use crate::instance::stub_0x439a84 as stub_439a84;
// 0x439c24 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel8GearTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::GearType>(RBX::DataModel::GearType const&)")]
pub use crate::instance::stub_0x439c24 as stub_439c24;
// 0x439c74 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearType>::singleton(void)")]
pub use crate::instance::stub_0x439c74 as stub_439c74;
// 0x439ce0 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearType>::construct_func(char const*,char *)")]
pub use crate::instance::stub_0x439ce0 as stub_439ce0;
// 0x439cec — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearType>::destruct_func(char *)")]
pub use crate::instance::stub_0x439cec as stub_439cec;
