//! boost_core_j — 150 boost stubs (EA-ordered, next uncovered after boost_core_i up to 0x577fa8).
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost", sorted by EA, next 150 uncovered.
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; sanitized alias uses `rbx_core::SharedPtr` not `boost::`.
//! Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr.


#[doc(alias = "RBX::Reflection::EventDesc<RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::~EventDesc()")]
// 0x57812c — __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev
// was: RBX::Reflection::EventDesc<RBX::HopperBin,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::HopperBin::*>::~EventDesc()
pub fn stub_57812c() -> ! {
    todo!("0x57812c __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x5781e0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<1,RBX::HopperBin,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_5781e0() -> ! {
    todo!("0x5781e0 __ZNK3RBX10Reflection13EventDescImplILi1ENS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x578344 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// was: RBX::Reflection::EventDescImpl<1,RBX::HopperBin,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_578344() -> ! {
    todo!("0x578344 __ZNK3RBX10Reflection13EventDescImplILi1ENS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x5784a4 — __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// was: RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_5784a4() -> ! {
    todo!("0x5784a4 __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")
}

#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::~remote_signal()")]
// 0x579878 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEED2Ev
// was: rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>::~remote_signal()
pub fn stub_579878() -> ! {
    todo!("0x579878 __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEED2Ev")
}

#[doc(alias = "RBX::ICharacterSubject::getNearPlaneCorners(boost::array<G3D::Vector3,4ul> &)const")]
// 0x57ab88 — __ZNK3RBX17ICharacterSubject19getNearPlaneCornersERN5boost5arrayIN3G3D7Vector3ELm4EEE
pub fn stub_57ab88() -> ! {
    todo!("0x57ab88 __ZNK3RBX17ICharacterSubject19getNearPlaneCornersERN5boost5arrayIN3G3D7Vector3ELm4EEE")
}

#[doc(alias = "RBX::ICharacterSubject::getHalfDistances(boost::array<float,4ul> &,G3D::Vector3 const&,G3D::CoordinateFrame const&)")]
// 0x57ad58 — __ZN3RBX17ICharacterSubject16getHalfDistancesERN5boost5arrayIfLm4EEERKN3G3D7Vector3ERKNS5_15CoordinateFrameE
pub fn stub_57ad58() -> ! {
    todo!("0x57ad58 __ZN3RBX17ICharacterSubject16getHalfDistancesERN5boost5arrayIfLm4EEERKN3G3D7Vector3ERKNS5_15CoordinateFrameE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Weld>::operator=(rbx_core::SharedPtr<RBX::Weld> const&)")]
// 0x57c39c — __ZN5boost10shared_ptrIN3RBX4WeldEEaSERKS3_
// was: boost::shared_ptr<RBX::Weld>::operator=(boost::shared_ptr<RBX::Weld> const&)
pub fn stub_57c39c() -> ! {
    todo!("0x57c39c __ZN5boost10shared_ptrIN3RBX4WeldEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiImageButton> RBX::Creatable<RBX::Instance>::create<RBX::GuiImageButton>(void)")]
// 0x57d5f8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14GuiImageButtonEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::GuiImageButton> RBX::Creatable<RBX::Instance>::create<RBX::GuiImageButton>(void)
pub fn stub_57d5f8() -> ! {
    todo!("0x57d5f8 __ZN3RBX9CreatableINS_8InstanceEE6createINS_14GuiImageButtonEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "RBX::InsertService::getFreeModels(std::string,int,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// 0x580750 — __ZN3RBX13InsertService13getFreeModelsESsiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
// was: RBX::InsertService::getFreeModels(std::string,int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)
pub fn stub_580750() -> ! {
    todo!("0x580750 __ZN3RBX13InsertService13getFreeModelsESsiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE")
}

#[doc(alias = "RBX::InsertService::getFreeDecals(std::string,int,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// 0x5809a0 — __ZN3RBX13InsertService13getFreeDecalsESsiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
// was: RBX::InsertService::getFreeDecals(std::string,int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)
pub fn stub_5809a0() -> ! {
    todo!("0x5809a0 __ZN3RBX13InsertService13getFreeDecalsESsiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE")
}

#[doc(alias = "RBX::InsertService::getBaseSets(boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// 0x580bf4 — __ZN3RBX13InsertService11getBaseSetsEN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
// was: RBX::InsertService::getBaseSets(boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)
pub fn stub_580bf4() -> ! {
    todo!("0x580bf4 __ZN3RBX13InsertService11getBaseSetsEN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE")
}

#[doc(alias = "RBX::InsertService::getUserSets(int,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// 0x580db4 — __ZN3RBX13InsertService11getUserSetsEiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
// was: RBX::InsertService::getUserSets(int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)
pub fn stub_580db4() -> ! {
    todo!("0x580db4 __ZN3RBX13InsertService11getUserSetsEiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE")
}

#[doc(alias = "RBX::InsertService::getCollection(int,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// 0x581004 — __ZN3RBX13InsertService13getCollectionEiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
// was: RBX::InsertService::getCollection(int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)
pub fn stub_581004() -> ! {
    todo!("0x581004 __ZN3RBX13InsertService13getCollectionEiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE")
}

#[doc(alias = "RBX::InsertService::loadAsset(int,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")]
// 0x581250 — __ZN3RBX13InsertService9loadAssetEiN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE
// was: RBX::InsertService::loadAsset(int,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>)
pub fn stub_581250() -> ! {
    todo!("0x581250 __ZN3RBX13InsertService9loadAssetEiN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE")
}

#[doc(alias = "RBX::InsertService::loadAssetVersion(int,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")]
// 0x58134c — __ZN3RBX13InsertService16loadAssetVersionEiN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE
// was: RBX::InsertService::loadAssetVersion(int,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>)
pub fn stub_58134c() -> ! {
    todo!("0x58134c __ZN3RBX13InsertService16loadAssetVersionEiN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE")
}

#[doc(alias = "RBX::InsertService::insert(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x581448 — __ZN3RBX13InsertService6insertEN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::InsertService::insert(boost::shared_ptr<RBX::Instance>)
pub fn stub_581448() -> ! {
    todo!("0x581448 __ZN3RBX13InsertService6insertEN5boost10shared_ptrINS_8InstanceEEE")
}

#[doc(alias = "RBX::InsertService::dispatchRequest(std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// 0x581ac0 — __ZN3RBX13InsertService15dispatchRequestERKSsN5boost8functionIFvNS3_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS8_EEEEEEENS4_IFvSsEEE
// was: RBX::InsertService::dispatchRequest(std::string const&,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)
pub fn stub_581ac0() -> ! {
    todo!("0x581ac0 __ZN3RBX13InsertService15dispatchRequestERKSsN5boost8functionIFvNS3_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS8_EEEEEEENS4_IFvSsEEE")
}

#[doc(alias = "RBX::InsertService::insertResultsReady(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x582ab8 — __ZN3RBX13InsertService18insertResultsReadyESsN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::InsertService::insertResultsReady(std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_582ab8() -> ! {
    todo!("0x582ab8 __ZN3RBX13InsertService18insertResultsReadyESsN5boost10shared_ptrINS_8InstanceEEE")
}

#[doc(alias = "RBX::InsertService::privateLoadAsset(int,bool,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")]
// 0x582e5c — __ZN3RBX13InsertService16privateLoadAssetEibN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE
// was: RBX::InsertService::privateLoadAsset(int,bool,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>)
pub fn stub_582e5c() -> ! {
    todo!("0x582e5c __ZN3RBX13InsertService16privateLoadAssetEibN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE")
}

#[doc(alias = "RBX::InsertService::backendInsertReady(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x583644 — __ZN3RBX13InsertService18backendInsertReadyESsN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::InsertService::backendInsertReady(std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_583644() -> ! {
    todo!("0x583644 __ZN3RBX13InsertService18backendInsertReadyESsN5boost10shared_ptrINS_8InstanceEEE")
}

#[doc(alias = "RBX::InsertService::BackendInsertReadyHelper(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x583850 — __ZN3RBX13InsertService24BackendInsertReadyHelperEN5boost8weak_ptrIS0_EESsNS1_10shared_ptrINS_8InstanceEEE
// was: RBX::InsertService::BackendInsertReadyHelper(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_583850() -> ! {
    todo!("0x583850 __ZN3RBX13InsertService24BackendInsertReadyHelperEN5boost8weak_ptrIS0_EESsNS1_10shared_ptrINS_8InstanceEEE")
}

#[doc(alias = "RBX::InsertService::safeInsert(RBX::ContentId,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// 0x5839f0 — __ZN3RBX13InsertService10safeInsertENS_9ContentIdEN5boost8functionIFvNS2_10shared_ptrINS_8InstanceEEEEEE
// was: RBX::InsertService::safeInsert(RBX::ContentId,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_5839f0() -> ! {
    todo!("0x5839f0 __ZN3RBX13InsertService10safeInsertENS_9ContentIdEN5boost8functionIFvNS2_10shared_ptrINS_8InstanceEEEEEE")
}

#[doc(alias = "RBX::InsertService::RemoteInsertItemsLoadedHelper(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// 0x583d54 — __ZN3RBX13InsertService29RemoteInsertItemsLoadedHelperEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultENS1_10shared_ptrISt6vectorINS6_INS_8InstanceEEESaIS9_EEEENS1_8functionIFvS9_EEE
// was: RBX::InsertService::RemoteInsertItemsLoadedHelper(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_583d54() -> ! {
    todo!("0x583d54 __ZN3RBX13InsertService29RemoteInsertItemsLoadedHelperEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultENS1_10shared_ptrISt6vectorINS6_INS_8InstanceEEESaIS9_EEEENS1_8functionIFvS9_EEE")
}

#[doc(alias = "RBX::InsertService::remoteInsertItemsLoaded(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// 0x583e90 — __ZN3RBX13InsertService23remoteInsertItemsLoadedENS_14AsyncHttpQueue13RequestResultEN5boost10shared_ptrISt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS3_8functionIFvS7_EEE
// was: RBX::InsertService::remoteInsertItemsLoaded(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_583e90() -> ! {
    todo!("0x583e90 __ZN3RBX13InsertService23remoteInsertItemsLoadedENS_14AsyncHttpQueue13RequestResultEN5boost10shared_ptrISt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS3_8functionIFvS7_EEE")
}

#[doc(alias = "RBX::UnsafeScriptStripperCollector(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,RBX::ScriptInformationProvider *,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x5841e8 — __ZN3RBXL29UnsafeScriptStripperCollectorEPSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS4_EEPNS_25ScriptInformationProviderES4_
// was: RBX::UnsafeScriptStripperCollector(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,RBX::ScriptInformationProvider *,boost::shared_ptr<RBX::Instance>)
pub fn stub_5841e8() -> ! {
    todo!("0x5841e8 __ZN3RBXL29UnsafeScriptStripperCollectorEPSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS4_EEPNS_25ScriptInformationProviderES4_")
}

#[doc(alias = "RBX::unsafeScriptStripper(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x584390 — __ZN3RBXL20unsafeScriptStripperEN5boost8weak_ptrINS_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS0_10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEfNS6_ISC_EENS0_8functionIFvS8_EEENS1_INS_25ScriptInformationProviderEEES8_
// was: RBX::unsafeScriptStripper(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>)
pub fn stub_584390() -> ! {
    todo!("0x584390 __ZN3RBXL20unsafeScriptStripperEN5boost8weak_ptrINS_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS0_10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEfNS6_ISC_EENS0_8functionIFvS8_EEENS1_INS_25ScriptInformationProviderEEES8_")
}

#[doc(alias = "void RBX::Reflection::resume_adapter<rbx_core::SharedPtr<RBX::Instance>>(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x584e40 — __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrINS_8InstanceEEEEEvNS2_8functionIFvNS0_7VariantEEEET_
// was: void RBX::Reflection::resume_adapter<boost::shared_ptr<RBX::Instance>>(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>)
pub fn stub_584e40() -> ! {
    todo!("0x584e40 __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrINS_8InstanceEEEEEvNS2_8functionIFvNS0_7VariantEEEET_")
}

#[doc(alias = "void RBX::Reflection::resume_adapter<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")]
// 0x584fa4 — __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS5_EEEEEEvNS2_8functionIFvS5_EEET_
// was: void RBX::Reflection::resume_adapter<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)
pub fn stub_584fa4() -> ! {
    todo!("0x584fa4 __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS5_EEEEEEvNS2_8functionIFvS5_EEET_")
}

#[doc(alias = "RBX::handleScriptInfoResponse(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x585108 — __ZN3RBXL24handleScriptInfoResponseENS_25ScriptInformationProvider13RequestResultEfN5boost8weak_ptrINS_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS2_10shared_ptrINS_8InstanceEEESt6vectorISA_SaISA_EEEEfNS8_ISE_EENS2_8functionIFvSA_EEENS3_IS0_EESA_
// was: RBX::handleScriptInfoResponse(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>)
pub fn stub_585108() -> ! {
    todo!("0x585108 __ZN3RBXL24handleScriptInfoResponseENS_25ScriptInformationProvider13RequestResultEfN5boost8weak_ptrINS_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS2_10shared_ptrINS_8InstanceEEESt6vectorISA_SaISA_EEEEfNS8_ISE_EENS2_8functionIFvSA_EEENS3_IS0_EESA_")
}

#[doc(alias = "RBX::CallResultFunction(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x585a10 — __ZN3RBXL18CallResultFunctionEN5boost8functionIFvNS0_10shared_ptrINS_8InstanceEEEEEES4_
// was: RBX::CallResultFunction(boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::shared_ptr<RBX::Instance>)
pub fn stub_585a10() -> ! {
    todo!("0x585a10 __ZN3RBXL18CallResultFunctionEN5boost8functionIFvNS0_10shared_ptrINS_8InstanceEEEEEES4_")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~RemoteEventDesc()")]
// 0x585b24 — __ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED1Ev
// was: RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>>::~RemoteEventDesc()
pub fn stub_585b24() -> ! {
    todo!("0x585b24 __ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(std::string,int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,2>::~BoundYieldFuncDesc()")]
// 0x585c2c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EED1Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(std::string,int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,2>::~BoundYieldFuncDesc()
pub fn stub_585c2c() -> ! {
    todo!("0x585c2c __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,0>::~BoundYieldFuncDesc()")]
// 0x585c74 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EED1Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,0>::~BoundYieldFuncDesc()
pub fn stub_585c74() -> ! {
    todo!("0x585c74 __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::~BoundYieldFuncDesc()")]
// 0x585c98 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EED1Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::~BoundYieldFuncDesc()
pub fn stub_585c98() -> ! {
    todo!("0x585c98 __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<RBX::Instance> ()(int),rbx_core::SharedPtr<RBX::Instance>,1>::~BoundYieldFuncDesc()")]
// 0x585d20 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EED1Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<RBX::Instance> ()(int),boost::shared_ptr<RBX::Instance>,1>::~BoundYieldFuncDesc()
pub fn stub_585d20() -> ! {
    todo!("0x585d20 __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// 0x585d60 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_585d60() -> ! {
    todo!("0x585d60 __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev")
}

#[doc(alias = "boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>::operator=(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)")]
// 0x585e6c — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEaSERKS6_
// was: boost::function<void ()(boost::shared_ptr<RBX::Instance>)>::operator=(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&)
pub fn stub_585e6c() -> ! {
    todo!("0x585e6c __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEaSERKS6_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ModelInstance>(rbx_core::SharedPtr<RBX::ModelInstance> const&)")]
// 0x5864b4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ModelInstanceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ModelInstance>(boost::shared_ptr<RBX::ModelInstance> const&)
pub fn stub_5864b4() -> ! {
    todo!("0x5864b4 __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ModelInstanceEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::InsertService*,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x5864e8 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_SsS6_
// was: RBX::Reflection::RemoteEventDescImpl<2,RBX::InsertService,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::InsertService*,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_5864e8() -> ! {
    todo!("0x5864e8 __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_SsS6_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::InsertService>,std::string,boost::arg<1>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::InsertService>,std::string,boost::arg<1>>(void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx_core::WeakPtr<RBX::InsertService>,std::string,boost::arg<1>)")]
// 0x58673c — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS2_8InstanceEEES4_SsNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSC_T0_T1_T2_ENSA_9list_av_3IT3_T4_T5_E4typeEEESH_SJ_SK_SL_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_3<boost::weak_ptr<RBX::InsertService>,std::string,boost::arg<1>>::type> boost::bind<void,boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>,boost::weak_ptr<RBX::InsertService>,std::string,boost::arg<1>>(void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::weak_ptr<RBX::InsertService>,std::string,boost::arg<1>)
pub fn stub_58673c() -> ! {
    todo!("0x58673c __ZN5boost4bindIvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS2_8InstanceEEES4_SsNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSC_T0_T1_T2_ENSA_9list_av_3IT3_T4_T5_E4typeEEESH_SJ_SK_SL_")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::InsertService> RBX::weak_from<RBX::InsertService>(RBX::InsertService*)")]
// 0x586988 — __ZN3RBX9weak_fromINS_13InsertServiceEEEN5boost8weak_ptrIT_EEPS4_
// was: boost::weak_ptr<RBX::InsertService> RBX::weak_from<RBX::InsertService>(RBX::InsertService*)
pub fn stub_586988() -> ! {
    todo!("0x586988 __ZN3RBX9weak_fromINS_13InsertServiceEEEN5boost8weak_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptInformationProvider> RBX::shared_from<RBX::ScriptInformationProvider>(RBX::ScriptInformationProvider*)")]
// 0x586e58 — __ZN3RBX11shared_fromINS_25ScriptInformationProviderEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::ScriptInformationProvider> RBX::shared_from<RBX::ScriptInformationProvider>(RBX::ScriptInformationProvider*)
pub fn stub_586e58() -> ! {
    todo!("0x586e58 __ZN3RBX11shared_fromINS_25ScriptInformationProviderEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list_av_4<rbx_core::WeakPtr<RBX::InsertService>,boost::arg<1>,boost::arg<2>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::InsertService>,boost::arg<1>,boost::arg<2>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),rbx_core::WeakPtr<RBX::InsertService>,boost::arg<1>,boost::arg<2>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// 0x586fc8 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13InsertServiceEEENS2_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS7_INS2_8InstanceEEESaISA_EEEENS_8functionIFvSA_EEES4_NS_3argILi1EEENSH_ILi2EEESG_EENS_3_bi6bind_tIT_PFSM_T0_T1_T2_T3_ENSK_9list_av_4IT4_T5_T6_T7_E4typeEEESS_SU_SV_SW_SX_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list_av_4<boost::weak_ptr<RBX::InsertService>,boost::arg<1>,boost::arg<2>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>::type> boost::bind<void,boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::InsertService>,boost::arg<1>,boost::arg<2>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>(void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::weak_ptr<RBX::InsertService>,boost::arg<1>,boost::arg<2>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_586fc8() -> ! {
    todo!("0x586fc8 __ZN5boost4bindIvNS_8weak_ptrIN3RBX13InsertServiceEEENS2_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS7_INS2_8InstanceEEESaISA_EEEENS_8functionIFvSA_EEES4_NS_3argILi1EEENSH_ILi2EEESG_EENS_3_bi6bind_tIT_PFSM_T0_T1_T2_T3_ENSK_9list_av_4IT4_T5_T6_T7_E4typeEEESS_SU_SV_SW_SX_")
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS0_IFvS7_EEEENSE_5list4INSE_5valueISI_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
// 0x5873c0 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS0_IFvS7_EEEENSE_5list4INSE_5valueISI_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
pub fn stub_5873c0() -> ! {
    todo!("0x5873c0 __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS0_IFvS7_EEEENSE_5list4INSE_5valueISI_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
// 0x58751c — __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
pub fn stub_58751c() -> ! {
    todo!("0x58751c __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>)")]
// 0x587678 — __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEEvT_
// was: void boost::function2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>)
pub fn stub_587678() -> ! {
    todo!("0x587678 __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x5877e8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_5877e8() -> ! {
    todo!("0x5877e8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>,void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
// 0x587804 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEvSA_SH_E6invokeERNS1_15function_bufferESA_SH_
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>,void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)
pub fn stub_587804() -> ! {
    todo!("0x587804 __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEvSA_SH_E6invokeERNS1_15function_bufferESA_SH_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>,boost::detail::function::function_buffer &)const")]
// 0x587820 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_587820() -> ! {
    todo!("0x587820 __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x587980 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_587980() -> ! {
    todo!("0x587980 __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x587adc — __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_587adc() -> ! {
    todo!("0x587adc __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>) &,boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>&> &,int)")]
// 0x587be8 — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultENSC_ISt6vectorISE_SaISE_EEEESG_ENS0_5list2IRSL_RSP_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>::operator()<void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>) &,boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>&> &,int)
pub fn stub_587be8() -> ! {
    todo!("0x587be8 __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultENSC_ISt6vectorISE_SaISE_EEEESG_ENS0_5list2IRSL_RSP_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x587d3c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_587d3c() -> ! {
    todo!("0x587d3c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>::list4(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>)")]
// 0x587ef0 — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEC2ES7_S9_SA_SH_
// was: boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>::list4(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>)
pub fn stub_587ef0() -> ! {
    todo!("0x587ef0 __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEC2ES7_S9_SA_SH_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>)")]
// 0x587ff4 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEC2ES7_S9_SA_SH_
// was: boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>)
pub fn stub_587ff4() -> ! {
    todo!("0x587ff4 __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEC2ES7_S9_SA_SH_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>)")]
// 0x5880f4 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>)
pub fn stub_5880f4() -> ! {
    todo!("0x5880f4 __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>)")]
// 0x5881c4 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEEEC2ES7_S9_
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>)
pub fn stub_5881c4() -> ! {
    todo!("0x5881c4 __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEEEC2ES7_S9_")
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS8_5list3INS8_5valueISC_EENSG_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// 0x5885b8 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS8_5list3INS8_5valueISC_EENSG_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
pub fn stub_5885b8() -> ! {
    todo!("0x5885b8 __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS8_5list3INS8_5valueISC_EENSG_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x588740 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_588740() -> ! {
    todo!("0x588740 __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>)")]
// 0x5888cc — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEEvT_
// was: void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>)
pub fn stub_5888cc() -> ! {
    todo!("0x5888cc __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x588a64 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_588a64() -> ! {
    todo!("0x588a64 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x588a80 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
pub fn stub_588a80() -> ! {
    todo!("0x588a80 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0x588a98 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_588a98() -> ! {
    todo!("0x588a98 __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x588c20 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_588c20() -> ! {
    todo!("0x588c20 __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x588da4 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_588da4() -> ! {
    todo!("0x588da4 __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::operator()<void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// 0x588eac — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEclIPFvS6_SsNS_10shared_ptrINS4_8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_588eac() -> ! {
    todo!("0x588eac __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEclIPFvS6_SsNS_10shared_ptrINS4_8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x58905c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_58905c() -> ! {
    todo!("0x58905c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)")]
// 0x5891f8 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_
// was: boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::list3(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)
pub fn stub_5891f8() -> ! {
    todo!("0x5891f8 __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)")]
// 0x589364 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)
pub fn stub_589364() -> ! {
    todo!("0x589364 __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>)")]
// 0x5894d0 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEEEC2ES7_S8_
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>)
pub fn stub_5894d0() -> ! {
    todo!("0x5894d0 __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEEEC2ES7_S8_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::InsertService>::shared_ptr<RBX::InsertService>(rbx_core::WeakPtr<RBX::InsertService> const&,boost::detail::sp_nothrow_tag)")]
// 0x5895d8 — __ZN5boost10shared_ptrIN3RBX13InsertServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::InsertService>::shared_ptr<RBX::InsertService>(boost::weak_ptr<RBX::InsertService> const&,boost::detail::sp_nothrow_tag)
pub fn stub_5895d8() -> ! {
    todo!("0x5895d8 __ZN5boost10shared_ptrIN3RBX13InsertServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::InsertService::*>::fireEvent(RBX::InsertService*,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
// 0x589654 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_SsS6_
// was: RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::InsertService::*>::fireEvent(RBX::InsertService*,std::string,boost::shared_ptr<RBX::Instance>)const
pub fn stub_589654() -> ! {
    todo!("0x589654 __ZNK3RBX10Reflection13EventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_SsS6_")
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x5897c8 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceESsS6_
// was: RBX::Reflection::RemoteEventDescImpl<2,RBX::InsertService,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_5897c8() -> ! {
    todo!("0x5897c8 __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceESsS6_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
// 0x589934 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::safe_static_do_get_mutex(void)
pub fn stub_589934() -> ! {
    todo!("0x589934 __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot> &)")]
// 0x589cb4 — __ZN3rbx7signals6signalIFvSsiiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_589cb4() -> ! {
    todo!("0x589cb4 __ZN3rbx7signals6signalIFvSsiiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot> const&)")]
// 0x589e3c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSERKS7_
pub fn stub_589e3c() -> ! {
    todo!("0x589e3c __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0x589f58 — __ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_589f58() -> ! {
    todo!("0x589f58 __ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x589fcc — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev
pub fn stub_589fcc() -> ! {
    todo!("0x589fcc __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x589ff8 — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev
pub fn stub_589ff8() -> ! {
    todo!("0x589ff8 __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
// 0x58a0cc — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs
pub fn stub_58a0cc() -> ! {
    todo!("0x58a0cc __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
// 0x58a0e8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs
pub fn stub_58a0e8() -> ! {
    todo!("0x58a0e8 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list2<std::string &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string> &,boost::_bi::list2<std::string &,std::string &> &,int)")]
// 0x58a104 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsSsEENS0_5list2IRSsSG_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_58a104() -> ! {
    todo!("0x58a104 __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsSsEENS0_5list2IRSsSG_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>::operator()(RBX::InsertService*,std::string,std::string)const")]
// 0x58a2ac — __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsSsEclEPS3_SsSs
pub fn stub_58a2ac() -> ! {
    todo!("0x58a2ac __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsSsEclEPS3_SsSs")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::~callable()")]
// 0x58a470 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev
pub fn stub_58a470() -> ! {
    todo!("0x58a470 __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::~callable()")]
// 0x58a49c — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev
pub fn stub_58a49c() -> ! {
    todo!("0x58a49c __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0x58a570 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)
pub fn stub_58a570() -> ! {
    todo!("0x58a570 __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
// 0x58a5e4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot*)
pub fn stub_58a5e4() -> ! {
    todo!("0x58a5e4 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x58a608 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEED1Ev
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
pub fn stub_58a608() -> ! {
    todo!("0x58a608 __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x58a634 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEED0Ev
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
pub fn stub_58a634() -> ! {
    todo!("0x58a634 __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
// 0x58a708 — __ZNK3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot9connectedEv
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot::connected(void)const
pub fn stub_58a708() -> ! {
    todo!("0x58a708 __ZNK3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x58a714 — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_E4callESsS7_
// was: rbx::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,boost::shared_ptr<RBX::Instance>)>::call(std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_58a714() -> ! {
    todo!("0x58a714 __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_E4callESsS7_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x58a730 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_E4callESsS7_
// was: `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,boost::shared_ptr<RBX::Instance>)>::call(std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_58a730() -> ! {
    todo!("0x58a730 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_E4callESsS7_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list2<std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// 0x58a74c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS_10shared_ptrINS3_8InstanceEEEEENS0_5list2IRSsRSG_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<std::string &,boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list2<std::string &,boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_58a74c() -> ! {
    todo!("0x58a74c __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS_10shared_ptrINS3_8InstanceEEEEENS0_5list2IRSsRSG_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::InsertService*,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
// 0x58a8c0 — __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS_10shared_ptrINS2_8InstanceEEEEclEPS3_SsS6_
// was: boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>::operator()(RBX::InsertService*,std::string,boost::shared_ptr<RBX::Instance>)const
pub fn stub_58a8c0() -> ! {
    todo!("0x58a8c0 __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS_10shared_ptrINS2_8InstanceEEEEclEPS3_SsS6_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
// 0x58aa4c — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_init_mutex(void)
pub fn stub_58aa4c() -> ! {
    todo!("0x58aa4c __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
// 0x58aa50 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)
pub fn stub_58aa50() -> ! {
    todo!("0x58aa50 __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// 0x58ab40 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD0Ev
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_58ab40() -> ! {
    todo!("0x58ab40 __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// 0x58ac14 — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_58ac14() -> ! {
    todo!("0x58ac14 __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// 0x58ac40 — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_58ac40() -> ! {
    todo!("0x58ac40 __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_ED0Ev")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::dummy::nonnull(void)")]
// 0x58b120 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE5dummy7nonnullEv
// was: boost::function1<void,boost::shared_ptr<RBX::Instance>>::dummy::nonnull(void)
pub fn stub_58b120() -> ! {
    todo!("0x58b120 __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE5dummy7nonnullEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
// 0x58b254 — __ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_
pub fn stub_58b254() -> ! {
    todo!("0x58b254 __ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(rbx::signals::signal<void ()(std::string,int,int)>::slot*)")]
// 0x58b2c8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSEPS6_
pub fn stub_58b2c8() -> ! {
    todo!("0x58b2c8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// 0x58b2ec — __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev
pub fn stub_58b2ec() -> ! {
    todo!("0x58b2ec __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// 0x58b318 — __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev
pub fn stub_58b318() -> ! {
    todo!("0x58b318 __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
// 0x58b508 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii
pub fn stub_58b508() -> ! {
    todo!("0x58b508 __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
// 0x58b530 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii
pub fn stub_58b530() -> ! {
    todo!("0x58b530 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list3<std::string &,int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int> &,boost::_bi::list3<std::string &,int &,int &> &,int)")]
// 0x58b558 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_SsiiEENS0_5list3IRSsRiSI_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_58b558() -> ! {
    todo!("0x58b558 __ZN5boost3_bi5list4INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_SsiiEENS0_5list3IRSsRiSI_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>::operator()(RBX::InsertService*,std::string,int,int)const")]
// 0x58b688 — __ZNK5boost4_mfi3mf3IvN3RBX13InsertServiceESsiiEclEPS3_Ssii
pub fn stub_58b688() -> ! {
    todo!("0x58b688 __ZNK5boost4_mfi3mf3IvN3RBX13InsertServiceESsiiEclEPS3_Ssii")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::~callable()")]
// 0x58b8e0 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev
pub fn stub_58b8e0() -> ! {
    todo!("0x58b8e0 __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::~callable()")]
// 0x58b90c — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev
pub fn stub_58b90c() -> ! {
    todo!("0x58b90c __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,RBX::ContentId)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0x58b9e0 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_58b9e0() -> ! {
    todo!("0x58b9e0 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot*)")]
// 0x58bc60 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSEPS8_
pub fn stub_58bc60() -> ! {
    todo!("0x58bc60 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSEPS8_")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot> const&)")]
// 0x58bc84 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSERKS9_
pub fn stub_58bc84() -> ! {
    todo!("0x58bc84 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x58bda4 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED1Ev
pub fn stub_58bda4() -> ! {
    todo!("0x58bda4 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x58bdd0 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED0Ev
pub fn stub_58bdd0() -> ! {
    todo!("0x58bdd0 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
// 0x58bfc0 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_
pub fn stub_58bfc0() -> ! {
    todo!("0x58bfc0 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
// 0x58bfdc — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_
pub fn stub_58bfdc() -> ! {
    todo!("0x58bfdc __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list2<std::string &,RBX::ContentId&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId> &,boost::_bi::list2<std::string &,RBX::ContentId&> &,int)")]
// 0x58bff8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS3_9ContentIdEEENS0_5list2IRSsRSE_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_58bff8() -> ! {
    todo!("0x58bff8 __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS3_9ContentIdEEENS0_5list2IRSsRSE_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>::operator()(RBX::InsertService*,std::string,RBX::ContentId)const")]
// 0x58c1a8 — __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS2_9ContentIdEEclEPS3_SsS4_
pub fn stub_58c1a8() -> ! {
    todo!("0x58c1a8 __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS2_9ContentIdEEclEPS3_SsS4_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::~callable()")]
// 0x58c658 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED1Ev
pub fn stub_58c658() -> ! {
    todo!("0x58c658 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::~callable()")]
// 0x58c684 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED0Ev
pub fn stub_58c684() -> ! {
    todo!("0x58c684 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED0Ev")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>> const&)")]
// 0x58c758 — __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEE13assign_to_ownERKSA_
// was: boost::function1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to_own(boost::function1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>> const&)
pub fn stub_58c758() -> ! {
    todo!("0x58c758 __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEE13assign_to_ownERKSA_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService> RBX::Creatable<RBX::Instance>::create<RBX::LuaWebService>(void)")]
// 0x58c788 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13LuaWebServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::LuaWebService> RBX::Creatable<RBX::Instance>::create<RBX::LuaWebService>(void)
pub fn stub_58c788() -> ! {
    todo!("0x58c788 __ZN3RBX9CreatableINS_8InstanceEE6createINS_13LuaWebServiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const&)")]
// 0x58c838 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13LuaWebServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LuaWebService>(boost::shared_ptr<RBX::LuaWebService> const&)
pub fn stub_58c838() -> ! {
    todo!("0x58c838 __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13LuaWebServiceEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x58c870 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_58c870() -> ! {
    todo!("0x58c870 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::remote_signal(void)")]
// 0x58ca70 — __ZN3rbx13remote_signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEEC2Ev
// was: rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::remote_signal(void)
pub fn stub_58ca70() -> ! {
    todo!("0x58ca70 __ZN3rbx13remote_signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEEC2Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::InsertService::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x58d1f8 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::InsertService::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_58d1f8() -> ! {
    todo!("0x58d1f8 __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x58d390 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// was: RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_58d390() -> ! {
    todo!("0x58d390 __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// 0x58d3c0 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_58d3c0() -> ! {
    todo!("0x58d3c0 __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x58d4dc — __ZNK3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_58d4dc() -> ! {
    todo!("0x58d4dc __ZNK3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::InsertService,void (RBX::InsertService::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::InsertService*,void (RBX::InsertService::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// 0x58d5c0 — __ZN3RBX10Reflection11Call1HelperINS_13InsertServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
// was: RBX::Reflection::Call1Helper<RBX::InsertService,void (RBX::InsertService::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::InsertService*,void (RBX::InsertService::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_58d5c0() -> ! {
    todo!("0x58d5c0 __ZN3RBX10Reflection11Call1HelperINS_13InsertServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<RBX::Instance> ()(int),rbx_core::SharedPtr<RBX::Instance>,1>::BoundYieldFuncDesc(void (RBX::InsertService::*)(int,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x58d6a8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EEC2EMS2_FviNS3_8functionIFvS6_EEENS9_IFvSsEEEEPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<RBX::Instance> ()(int),boost::shared_ptr<RBX::Instance>,1>::BoundYieldFuncDesc(void (RBX::InsertService::*)(int,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_58d6a8() -> ! {
    todo!("0x58d6a8 __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EEC2EMS2_FviNS3_8functionIFvS6_EEENS9_IFvSsEEEEPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<RBX::Instance> ()(int),rbx_core::SharedPtr<RBX::Instance>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x58d820 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EE16declareSignatureEPKcNS0_7VariantE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<RBX::Instance> ()(int),boost::shared_ptr<RBX::Instance>,1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_58d820() -> ! {
    todo!("0x58d820 __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<RBX::Instance> ()(int),rbx_core::SharedPtr<RBX::Instance>,1>::~BoundYieldFuncDesc()")]
// 0x58d850 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EED0Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<RBX::Instance> ()(int),boost::shared_ptr<RBX::Instance>,1>::~BoundYieldFuncDesc()
pub fn stub_58d850() -> ! {
    todo!("0x58d850 __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<RBX::Instance> ()(int),rbx_core::SharedPtr<RBX::Instance>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// 0x58d924 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<RBX::Instance> ()(int),boost::shared_ptr<RBX::Instance>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
pub fn stub_58d924() -> ! {
    todo!("0x58d924 __ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::BoundYieldFuncDesc(void (RBX::InsertService::*)(int,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x58de40 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EEC2EMS2_FviNS3_8functionIFvSA_EEENSD_IFvSsEEEEPKcSL_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::BoundYieldFuncDesc(void (RBX::InsertService::*)(int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_58de40() -> ! {
    todo!("0x58de40 __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EEC2EMS2_FviNS3_8functionIFvSA_EEENSD_IFvSsEEEEPKcSL_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x58dfb8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EE16declareSignatureEPKcS6_
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_58dfb8() -> ! {
    todo!("0x58dfb8 __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EE16declareSignatureEPKcS6_")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::~BoundYieldFuncDesc()")]
// 0x58dfe8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EED0Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::~BoundYieldFuncDesc()
pub fn stub_58dfe8() -> ! {
    todo!("0x58dfe8 __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// 0x58e0bc — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS6_EEENSI_IFvSsEEE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
pub fn stub_58e0bc() -> ! {
    todo!("0x58e0bc __ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS6_EEENSI_IFvSsEEE")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
// 0x58e25c — __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS4_SaIS4_EEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSH_T0_T1_ENSF_9list_av_2IT2_T3_E4typeEEESL_SN_SO_
// was: boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)
pub fn stub_58e25c() -> ! {
    todo!("0x58e25c __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS4_SaIS4_EEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSH_T0_T1_ENSF_9list_av_2IT2_T3_E4typeEEESL_SN_SO_")
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvS5_EEES9_ENSD_5list2INSD_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// 0x58e358 — __ZN5boost8functionIFvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvS5_EEES9_ENSD_5list2INSD_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
pub fn stub_58e358() -> ! {
    todo!("0x58e358 __ZN5boost8functionIFvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvS5_EEES9_ENSD_5list2INSD_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// 0x58e42c — __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
pub fn stub_58e42c() -> ! {
    todo!("0x58e42c __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")]
// 0x58e500 — __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEEvT_
// was: void boost::function1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)
pub fn stub_58e500() -> ! {
    todo!("0x58e500 __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x58e5e4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_58e5e4() -> ! {
    todo!("0x58e5e4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")]
// 0x58e600 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSG_E6invokeERNS1_15function_bufferESG_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)
pub fn stub_58e600() -> ! {
    todo!("0x58e600 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSG_E6invokeERNS1_15function_bufferESG_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0x58e618 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_58e618() -> ! {
    todo!("0x58e618 __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x58e6f0 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_58e6f0() -> ! {
    todo!("0x58e6f0 __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x58e7c0 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_58e7c0() -> ! {
    todo!("0x58e7c0 __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list1<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>) &,boost::_bi::list1<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>&> &,int)")]
// 0x58e884 — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrIKSt6vectorIS6_SaIS6_EEEEENS0_5list1IRSJ_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list1<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>) &,boost::_bi::list1<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>&> &,int)
pub fn stub_58e884() -> ! {
    todo!("0x58e884 __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrIKSt6vectorIS6_SaIS6_EEEEENS0_5list1IRSJ_EEEEvNS0_4typeIvEERT_RT0_i")
}
