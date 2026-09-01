//! rendering shard 391 — 100 stubs 0x5826b0..0x58ac14 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 42311->42411 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x5826b0..0x58ac14 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5826b0 — __ZN3RBX13InsertService34backendInsertAssetVersionRequestedESsii
// type: void __fastcall(int, const std::string *, int)
#[doc(alias = "__ZN3RBX13InsertService34backendInsertAssetVersionRequestedESsii")]
#[doc(alias = "RBX::InsertService::backendInsertAssetVersionRequested(std::string,int,int)")]
// was: __ZN3RBX13InsertService34backendInsertAssetVersionRequestedESsii
pub fn stub_5826b0() -> ! {
    todo!("0x5826b0 RBX::InsertService::backendInsertAssetVersionRequested(std::string,int,int)")
}

// 0x582ab8 — __ZN3RBX13InsertService18insertResultsReadyESsN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, pthread_mutex_t *, char, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX13InsertService18insertResultsReadyESsN5boost10shared_ptrINS_8InstanceEEE")]
#[doc(alias = "RBX::InsertService::insertResultsReady(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX13InsertService18insertResultsReadyESsN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_582ab8() -> ! {
    todo!("0x582ab8 RBX::InsertService::insertResultsReady(std::string,boost::shared_ptr<RBX::Instance>)")
}

// 0x582c5c — __ZN3RBX13InsertService18insertResultsErrorESsSs
#[doc(alias = "__ZN3RBX13InsertService18insertResultsErrorESsSs")]
#[doc(alias = "RBX::InsertService::insertResultsError(std::string,std::string)")]
// was: __ZN3RBX13InsertService18insertResultsErrorESsSs
pub fn stub_582c5c() -> ! {
    todo!("0x582c5c RBX::InsertService::insertResultsError(std::string,std::string)")
}

// 0x582e5c — __ZN3RBX13InsertService16privateLoadAssetEibN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE
#[doc(alias = "__ZN3RBX13InsertService16privateLoadAssetEibN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE")]
#[doc(alias = "RBX::InsertService::privateLoadAsset(int,bool,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13InsertService16privateLoadAssetEibN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE
pub fn stub_582e5c() -> ! {
    todo!("0x582e5c RBX::InsertService::privateLoadAsset(int,bool,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>)")
}

// 0x583644 — __ZN3RBX13InsertService18backendInsertReadyESsN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "__ZN3RBX13InsertService18backendInsertReadyESsN5boost10shared_ptrINS_8InstanceEEE")]
#[doc(alias = "RBX::InsertService::backendInsertReady(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX13InsertService18backendInsertReadyESsN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_583644() -> ! {
    todo!("0x583644 RBX::InsertService::backendInsertReady(std::string,boost::shared_ptr<RBX::Instance>)")
}

// 0x583850 — __ZN3RBX13InsertService24BackendInsertReadyHelperEN5boost8weak_ptrIS0_EESsNS1_10shared_ptrINS_8InstanceEEE
#[doc(alias = "__ZN3RBX13InsertService24BackendInsertReadyHelperEN5boost8weak_ptrIS0_EESsNS1_10shared_ptrINS_8InstanceEEE")]
#[doc(alias = "RBX::InsertService::BackendInsertReadyHelper(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX13InsertService24BackendInsertReadyHelperEN5boost8weak_ptrIS0_EESsNS1_10shared_ptrINS_8InstanceEEE
pub fn stub_583850() -> ! {
    todo!("0x583850 RBX::InsertService::BackendInsertReadyHelper(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>)")
}

// 0x5839f0 — __ZN3RBX13InsertService10safeInsertENS_9ContentIdEN5boost8functionIFvNS2_10shared_ptrINS_8InstanceEEEEEE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZN3RBX13InsertService10safeInsertENS_9ContentIdEN5boost8functionIFvNS2_10shared_ptrINS_8InstanceEEEEEE")]
#[doc(alias = "RBX::InsertService::safeInsert(RBX::ContentId,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: __ZN3RBX13InsertService10safeInsertENS_9ContentIdEN5boost8functionIFvNS2_10shared_ptrINS_8InstanceEEEEEE
pub fn stub_5839f0() -> ! {
    todo!("0x5839f0 RBX::InsertService::safeInsert(RBX::ContentId,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)")
}

// 0x583d54 — __ZN3RBX13InsertService29RemoteInsertItemsLoadedHelperEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultENS1_10shared_ptrISt6vectorINS6_INS_8InstanceEEESaIS9_EEEENS1_8functionIFvS9_EEE
// type: int __fastcall(int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN3RBX13InsertService29RemoteInsertItemsLoadedHelperEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultENS1_10shared_ptrISt6vectorINS6_INS_8InstanceEEESaIS9_EEEENS1_8functionIFvS9_EEE")]
#[doc(alias = "RBX::InsertService::RemoteInsertItemsLoadedHelper(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: __ZN3RBX13InsertService29RemoteInsertItemsLoadedHelperEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultENS1_10shared_ptrISt6vectorINS6_INS_8InstanceEEESaIS9_EEEENS1_8functionIFvS9_EEE
pub fn stub_583d54() -> ! {
    todo!("0x583d54 RBX::InsertService::RemoteInsertItemsLoadedHelper(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)")
}

// 0x583e90 — __ZN3RBX13InsertService23remoteInsertItemsLoadedENS_14AsyncHttpQueue13RequestResultEN5boost10shared_ptrISt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS3_8functionIFvS7_EEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN3RBX13InsertService23remoteInsertItemsLoadedENS_14AsyncHttpQueue13RequestResultEN5boost10shared_ptrISt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS3_8functionIFvS7_EEE")]
#[doc(alias = "RBX::InsertService::remoteInsertItemsLoaded(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: __ZN3RBX13InsertService23remoteInsertItemsLoadedENS_14AsyncHttpQueue13RequestResultEN5boost10shared_ptrISt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS3_8functionIFvS7_EEE
pub fn stub_583e90() -> ! {
    todo!("0x583e90 RBX::InsertService::remoteInsertItemsLoaded(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)")
}

// 0x5841e8 — __ZN3RBXL29UnsafeScriptStripperCollectorEPSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS4_EEPNS_25ScriptInformationProviderES4_
// type: int __fastcall(int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN3RBXL29UnsafeScriptStripperCollectorEPSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS4_EEPNS_25ScriptInformationProviderES4_")]
#[doc(alias = "RBX::UnsafeScriptStripperCollector(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,RBX::ScriptInformationProvider *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBXL29UnsafeScriptStripperCollectorEPSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS4_EEPNS_25ScriptInformationProviderES4_
pub fn stub_5841e8() -> ! {
    todo!("0x5841e8 RBX::UnsafeScriptStripperCollector(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,RBX::ScriptInformationProvider *,boost::shared_ptr<RBX::Instance>)")
}

// 0x584390 — __ZN3RBXL20unsafeScriptStripperEN5boost8weak_ptrINS_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS0_10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEfNS6_ISC_EENS0_8functionIFvS8_EEENS1_INS_25ScriptInformationProviderEEES8_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBXL20unsafeScriptStripperEN5boost8weak_ptrINS_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS0_10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEfNS6_ISC_EENS0_8functionIFvS8_EEENS1_INS_25ScriptInformationProviderEEES8_")]
#[doc(alias = "RBX::unsafeScriptStripper(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBXL20unsafeScriptStripperEN5boost8weak_ptrINS_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS0_10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEfNS6_ISC_EENS0_8functionIFvS8_EEENS1_INS_25ScriptInformationProviderEEES8_
pub fn stub_584390() -> ! {
    todo!("0x584390 RBX::unsafeScriptStripper(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>)")
}

// 0x584e40 — __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrINS_8InstanceEEEEEvNS2_8functionIFvNS0_7VariantEEEET_
#[doc(alias = "__ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrINS_8InstanceEEEEEvNS2_8functionIFvNS0_7VariantEEEET_")]
#[doc(alias = "void RBX::Reflection::resume_adapter<rbx_core::SharedPtr<RBX::Instance>>(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrINS_8InstanceEEEEEvNS2_8functionIFvNS0_7VariantEEEET_
pub fn stub_584e40() -> ! {
    todo!("0x584e40 void RBX::Reflection::resume_adapter<boost::shared_ptr<RBX::Instance>>(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Instance>)")
}

// 0x585108 — __ZN3RBXL24handleScriptInfoResponseENS_25ScriptInformationProvider13RequestResultEfN5boost8weak_ptrINS_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS2_10shared_ptrINS_8InstanceEEESt6vectorISA_SaISA_EEEEfNS8_ISE_EENS2_8functionIFvSA_EEENS3_IS0_EESA_
// type: int __fastcall(int, int, int, int, float, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN3RBXL24handleScriptInfoResponseENS_25ScriptInformationProvider13RequestResultEfN5boost8weak_ptrINS_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS2_10shared_ptrINS_8InstanceEEESt6vectorISA_SaISA_EEEEfNS8_ISE_EENS2_8functionIFvSA_EEENS3_IS0_EESA_")]
#[doc(alias = "RBX::handleScriptInfoResponse(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBXL24handleScriptInfoResponseENS_25ScriptInformationProvider13RequestResultEfN5boost8weak_ptrINS_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS2_10shared_ptrINS_8InstanceEEESt6vectorISA_SaISA_EEEEfNS8_ISE_EENS2_8functionIFvSA_EEENS3_IS0_EESA_
pub fn stub_585108() -> ! {
    todo!("0x585108 RBX::handleScriptInfoResponse(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>)")
}

// 0x585a10 — __ZN3RBXL18CallResultFunctionEN5boost8functionIFvNS0_10shared_ptrINS_8InstanceEEEEEES4_
#[doc(alias = "__ZN3RBXL18CallResultFunctionEN5boost8functionIFvNS0_10shared_ptrINS_8InstanceEEEEEES4_")]
#[doc(alias = "RBX::CallResultFunction(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBXL18CallResultFunctionEN5boost8functionIFvNS0_10shared_ptrINS_8InstanceEEEEEES4_
pub fn stub_585a10() -> ! {
    todo!("0x585a10 RBX::CallResultFunction(boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::shared_ptr<RBX::Instance>)")
}

// 0x585adc — __ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsNS_9ContentIdEEN3rbx13remote_signalIS4_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsNS_9ContentIdEEN3rbx13remote_signalIS4_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,RBX::ContentId),rbx::remote_signal<void ()(std::string,RBX::ContentId)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsNS_9ContentIdEEN3rbx13remote_signalIS4_EEED1Ev
pub fn stub_585adc() -> ! {
    todo!("0x585adc RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,RBX::ContentId),rbx::remote_signal<void ()(std::string,RBX::ContentId)>>::~RemoteEventDesc()")
}

// 0x585b00 — __ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsiiEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsiiEN3rbx13remote_signalIS3_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsiiEN3rbx13remote_signalIS3_EEED1Ev
pub fn stub_585b00() -> ! {
    todo!("0x585b00 RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::~RemoteEventDesc()")
}

// 0x585b24 — __ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED1Ev
pub fn stub_585b24() -> ! {
    todo!("0x585b24 RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>>::~RemoteEventDesc()")
}

// 0x585b48 — __ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEED1Ev
pub fn stub_585b48() -> ! {
    todo!("0x585b48 RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>>::~RemoteEventDesc()")
}

// 0x585c2c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EED1Ev
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(std::string,int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,2>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EED1Ev
pub fn stub_585c2c() -> ! {
    todo!("0x585c2c RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(std::string,int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,2>::~BoundYieldFuncDesc()")
}

// 0x585c74 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,0>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EED1Ev
pub fn stub_585c74() -> ! {
    todo!("0x585c74 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,0>::~BoundYieldFuncDesc()")
}

// 0x585c98 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EED1Ev
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EED1Ev
pub fn stub_585c98() -> ! {
    todo!("0x585c98 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::~BoundYieldFuncDesc()")
}

// 0x585d20 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EED1Ev
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<RBX::Instance> ()(int),rbx_core::SharedPtr<RBX::Instance>,1>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EED1Ev
pub fn stub_585d20() -> ! {
    todo!("0x585d20 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<RBX::Instance> ()(int),boost::shared_ptr<RBX::Instance>,1>::~BoundYieldFuncDesc()")
}

// 0x585d60 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
pub fn stub_585d60() -> ! {
    todo!("0x585d60 RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0x585e6c — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEaSERKS6_
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEaSERKS6_")]
#[doc(alias = "boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>::operator=(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)")]
// was: __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEaSERKS6_
pub fn stub_585e6c() -> ! {
    todo!("0x585e6c boost::function<void ()(boost::shared_ptr<RBX::Instance>)>::operator=(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&)")
}

// 0x585f30 — __ZNSt3mapISsN3RBX13InsertService8CallbackESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
// type: int(void)
#[doc(alias = "__ZNSt3mapISsN3RBX13InsertService8CallbackESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")]
#[doc(alias = "std::map<std::string,RBX::InsertService::Callback,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::operator[](std::string const&)")]
// was: __ZNSt3mapISsN3RBX13InsertService8CallbackESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
pub fn stub_585f30() -> ! {
    todo!("0x585f30 std::map<std::string,RBX::InsertService::Callback,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::operator[](std::string const&)")
}

// 0x5861fc — __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_13InsertServiceEFvSsiiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceESsii
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_13InsertServiceEFvSsiiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceESsii")]
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::InsertService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::replicateEvent(RBX::Reflection::EventSource *,std::string,int,int)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_13InsertServiceEFvSsiiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceESsii
pub fn stub_5861fc() -> ! {
    todo!("0x5861fc RBX::Reflection::RemoteEventDescImpl<3,RBX::InsertService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::replicateEvent(RBX::Reflection::EventSource *,std::string,int,int)")
}

// 0x586390 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_13InsertServiceEFvSsiiEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ssii
// type: int __fastcall(int, int, std::string *, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_13InsertServiceEFvSsiiEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ssii")]
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::InsertService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::InsertService::*>::fireEvent(RBX::InsertService*,std::string,int,int)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi3ENS_13InsertServiceEFvSsiiEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ssii
pub fn stub_586390() -> ! {
    todo!("0x586390 RBX::Reflection::EventDescImpl<3,RBX::InsertService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::InsertService::*>::fireEvent(RBX::InsertService*,std::string,int,int)const")
}

// 0x5864b4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ModelInstanceEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ModelInstanceEEERS3_RKNS0_IT_EE")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ModelInstance>(rbx_core::SharedPtr<RBX::ModelInstance> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ModelInstanceEEERS3_RKNS0_IT_EE
pub fn stub_5864b4() -> ! {
    todo!("0x5864b4 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ModelInstance>(boost::shared_ptr<RBX::ModelInstance> const&)")
}

// 0x5864e8 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_SsS6_
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_SsS6_")]
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::InsertService*,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_SsS6_
pub fn stub_5864e8() -> ! {
    todo!("0x5864e8 RBX::Reflection::RemoteEventDescImpl<2,RBX::InsertService,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::InsertService*,std::string,boost::shared_ptr<RBX::Instance>)")
}

// 0x58673c — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS2_8InstanceEEES4_SsNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSC_T0_T1_T2_ENSA_9list_av_3IT3_T4_T5_E4typeEEESH_SJ_SK_SL_
// type: int(void)
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS2_8InstanceEEES4_SsNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSC_T0_T1_T2_ENSA_9list_av_3IT3_T4_T5_E4typeEEESH_SJ_SK_SL_")]
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::InsertService>,std::string,boost::arg<1>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::InsertService>,std::string,boost::arg<1>>(void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx_core::WeakPtr<RBX::InsertService>,std::string,boost::arg<1>)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS2_8InstanceEEES4_SsNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSC_T0_T1_T2_ENSA_9list_av_3IT3_T4_T5_E4typeEEESH_SJ_SK_SL_
pub fn stub_58673c() -> ! {
    todo!("0x58673c boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_3<boost::weak_ptr<RBX::InsertService>,std::string,boost::arg<1>>::type> boost::bind<void,boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>,boost::weak_ptr<RBX::InsertService>,std::string,boost::arg<1>>(void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::weak_ptr<RBX::InsertService>,std::string,boost::arg<1>)")
}

// 0x586988 — __ZN3RBX9weak_fromINS_13InsertServiceEEEN5boost8weak_ptrIT_EEPS4_
// type: int(void)
#[doc(alias = "__ZN3RBX9weak_fromINS_13InsertServiceEEEN5boost8weak_ptrIT_EEPS4_")]
#[doc(alias = "rbx_core::WeakPtr<RBX::InsertService> RBX::weak_from<RBX::InsertService>(RBX::InsertService*)")]
// was: __ZN3RBX9weak_fromINS_13InsertServiceEEEN5boost8weak_ptrIT_EEPS4_
pub fn stub_586988() -> ! {
    todo!("0x586988 boost::weak_ptr<RBX::InsertService> RBX::weak_from<RBX::InsertService>(RBX::InsertService*)")
}

// 0x586b90 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEE21fireAndReplicateEventEPS2_SsSs
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEE21fireAndReplicateEventEPS2_SsSs")]
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>>::fireAndReplicateEvent(RBX::InsertService*,std::string,std::string)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEE21fireAndReplicateEventEPS2_SsSs
pub fn stub_586b90() -> ! {
    todo!("0x586b90 RBX::Reflection::RemoteEventDescImpl<2,RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>>::fireAndReplicateEvent(RBX::InsertService*,std::string,std::string)")
}

// 0x586e58 — __ZN3RBX11shared_fromINS_25ScriptInformationProviderEEEN5boost10shared_ptrIT_EEPS4_
// type: int(void)
#[doc(alias = "__ZN3RBX11shared_fromINS_25ScriptInformationProviderEEEN5boost10shared_ptrIT_EEPS4_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptInformationProvider> RBX::shared_from<RBX::ScriptInformationProvider>(RBX::ScriptInformationProvider*)")]
// was: __ZN3RBX11shared_fromINS_25ScriptInformationProviderEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_586e58() -> ! {
    todo!("0x586e58 boost::shared_ptr<RBX::ScriptInformationProvider> RBX::shared_from<RBX::ScriptInformationProvider>(RBX::ScriptInformationProvider*)")
}

// 0x586fc8 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13InsertServiceEEENS2_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS7_INS2_8InstanceEEESaISA_EEEENS_8functionIFvSA_EEES4_NS_3argILi1EEENSH_ILi2EEESG_EENS_3_bi6bind_tIT_PFSM_T0_T1_T2_T3_ENSK_9list_av_4IT4_T5_T6_T7_E4typeEEESS_SU_SV_SW_SX_
// type: int __fastcall(int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX13InsertServiceEEENS2_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS7_INS2_8InstanceEEESaISA_EEEENS_8functionIFvSA_EEES4_NS_3argILi1EEENSH_ILi2EEESG_EENS_3_bi6bind_tIT_PFSM_T0_T1_T2_T3_ENSK_9list_av_4IT4_T5_T6_T7_E4typeEEESS_SU_SV_SW_SX_")]
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list_av_4<rbx_core::WeakPtr<RBX::InsertService>,boost::arg<1>,boost::arg<2>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::InsertService>,boost::arg<1>,boost::arg<2>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),rbx_core::WeakPtr<RBX::InsertService>,boost::arg<1>,boost::arg<2>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX13InsertServiceEEENS2_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS7_INS2_8InstanceEEESaISA_EEEENS_8functionIFvSA_EEES4_NS_3argILi1EEENSH_ILi2EEESG_EENS_3_bi6bind_tIT_PFSM_T0_T1_T2_T3_ENSK_9list_av_4IT4_T5_T6_T7_E4typeEEESS_SU_SV_SW_SX_
pub fn stub_586fc8() -> ! {
    todo!("0x586fc8 boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list_av_4<boost::weak_ptr<RBX::InsertService>,boost::arg<1>,boost::arg<2>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>::type> boost::bind<void,boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::InsertService>,boost::arg<1>,boost::arg<2>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>(void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::weak_ptr<RBX::InsertService>,boost::arg<1>,boost::arg<2>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)")
}

// 0x5871a4 — __ZN3RBX13InsertServiceD1Ev
// type: void __fastcall(RBX::InsertService *__hidden this)
#[doc(alias = "__ZN3RBX13InsertServiceD1Ev")]
#[doc(alias = "RBX::InsertService::~InsertService()")]
// was: __ZN3RBX13InsertServiceD1Ev
pub fn stub_5871a4() -> ! {
    todo!("0x5871a4 RBX::InsertService::~InsertService()")
}

// 0x5871a8 — __ZN3RBX13InsertServiceD0Ev
// type: void __fastcall(RBX::InsertService *__hidden this)
#[doc(alias = "__ZN3RBX13InsertServiceD0Ev")]
#[doc(alias = "RBX::InsertService::~InsertService()")]
// was: __ZN3RBX13InsertServiceD0Ev
pub fn stub_5871a8() -> ! {
    todo!("0x5871a8 RBX::InsertService::~InsertService()")
}

// 0x587258 — __ZThn32_N3RBX13InsertServiceD1Ev
// type: void __fastcall(RBX::InsertService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX13InsertServiceD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::InsertService::~InsertService()")]
// was: __ZThn32_N3RBX13InsertServiceD1Ev
pub fn stub_587258() -> ! {
    todo!("0x587258 non-virtual thunk to RBX::InsertService::~InsertService()")
}

// 0x587260 — __ZThn32_N3RBX13InsertServiceD0Ev
// type: void __fastcall(RBX::InsertService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX13InsertServiceD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::InsertService::~InsertService()")]
// was: __ZThn32_N3RBX13InsertServiceD0Ev
pub fn stub_587260() -> ! {
    todo!("0x587260 non-virtual thunk to RBX::InsertService::~InsertService()")
}

// 0x587314 — __ZThn36_N3RBX13InsertServiceD1Ev
// type: void __fastcall(RBX::InsertService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX13InsertServiceD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::InsertService::~InsertService()")]
// was: __ZThn36_N3RBX13InsertServiceD1Ev
pub fn stub_587314() -> ! {
    todo!("0x587314 non-virtual thunk to RBX::InsertService::~InsertService()")
}

// 0x58731c — __ZThn36_N3RBX13InsertServiceD0Ev
// type: void __fastcall(RBX::InsertService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX13InsertServiceD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::InsertService::~InsertService()")]
// was: __ZThn36_N3RBX13InsertServiceD0Ev
pub fn stub_58731c() -> ! {
    todo!("0x58731c non-virtual thunk to RBX::InsertService::~InsertService()")
}

// 0x5873c0 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS0_IFvS7_EEEENSE_5list4INSE_5valueISI_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS0_IFvS7_EEEENSE_5list4INSE_5valueISI_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS0_IFvS7_EEEENSE_5list4INSE_5valueISI_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS0_IFvS7_EEEENSE_5list4INSE_5valueISI_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
pub fn stub_5873c0() -> ! {
    todo!("0x5873c0 __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS0_IFvS7_EEEENSE_5list4INSE_5valueISI_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")
}

// 0x58751c — __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
pub fn stub_58751c() -> ! {
    todo!("0x58751c __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")
}

// 0x587678 — __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEEvT_")]
#[doc(alias = "void boost::function2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>)")]
// was: __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEEvT_
pub fn stub_587678() -> ! {
    todo!("0x587678 void boost::function2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>)")
}

// 0x5877e8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
pub fn stub_5877e8() -> ! {
    todo!("0x5877e8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x587804 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEvSA_SH_E6invokeERNS1_15function_bufferESA_SH_
// type: int __fastcall(int, int, char)
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEvSA_SH_E6invokeERNS1_15function_bufferESA_SH_")]
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>,void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEvSA_SH_E6invokeERNS1_15function_bufferESA_SH_
pub fn stub_587804() -> ! {
    todo!("0x587804 boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>,void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)")
}

// 0x587820 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEbT_RNS1_15function_bufferE")]
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_587820() -> ! {
    todo!("0x587820 bool boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>,boost::detail::function::function_buffer &)const")
}

// 0x587980 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_587980() -> ! {
    todo!("0x587980 bool boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x587adc — __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS6_INS3_8InstanceEEESaIS9_EEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13InsertServiceEEES5_SC_NS_8functionIFvS9_EEEENSF_5list4INSF_5valueISJ_EENS_3argILi1EEENSS_ILi2EEENSQ_ISM_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_587adc() -> ! {
    todo!("0x587adc void boost::detail::function::basic_vtable2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x587be8 — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultENSC_ISt6vectorISE_SaISE_EEEESG_ENS0_5list2IRSL_RSP_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultENSC_ISt6vectorISE_SaISE_EEEESG_ENS0_5list2IRSL_RSP_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>) &,boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>&> &,int)")]
// was: __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultENSC_ISt6vectorISE_SaISE_EEEESG_ENS0_5list2IRSL_RSP_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_587be8() -> ! {
    todo!("0x587be8 void boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>::operator()<void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>) &,boost::_bi::list2<RBX::AsyncHttpQueue::RequestResult&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>&> &,int)")
}

// 0x587d3c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_587d3c() -> ! {
    todo!("0x587d3c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x587ef0 — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEC2ES7_S9_SA_SH_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEC2ES7_S9_SA_SH_")]
#[doc(alias = "boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>::list4(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>)")]
// was: __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEC2ES7_S9_SA_SH_
pub fn stub_587ef0() -> ! {
    todo!("0x587ef0 boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>::list4(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>)")
}

// 0x587ff4 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEC2ES7_S9_SA_SH_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEC2ES7_S9_SA_SH_")]
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFvNS_10shared_ptrINS4_8InstanceEEEEEEEEEC2ES7_S9_SA_SH_
pub fn stub_587ff4() -> ! {
    todo!("0x587ff4 boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>)")
}

// 0x5880f4 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_")]
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
pub fn stub_5880f4() -> ! {
    todo!("0x5880f4 boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>)")
}

// 0x5881c4 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEEEC2ES7_S9_
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEEEC2ES7_S9_")]
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::arg<1>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEEEC2ES7_S9_
pub fn stub_5881c4() -> ! {
    todo!("0x5881c4 boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>)")
}

// 0x5882a4 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_SsSs
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_SsSs")]
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string)> RBX::InsertService::*>::fireEvent(RBX::InsertService*,std::string,std::string)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_SsSs
pub fn stub_5882a4() -> ! {
    todo!("0x5882a4 RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string)> RBX::InsertService::*>::fireEvent(RBX::InsertService*,std::string,std::string)const")
}

// 0x58844c — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceESsSs
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceESsSs")]
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>>::replicateEvent(RBX::Reflection::EventSource *,std::string,std::string)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceESsSs
pub fn stub_58844c() -> ! {
    todo!("0x58844c RBX::Reflection::RemoteEventDescImpl<2,RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>>::replicateEvent(RBX::Reflection::EventSource *,std::string,std::string)")
}

// 0x5885b8 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS8_5list3INS8_5valueISC_EENSG_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS8_5list3INS8_5valueISC_EENSG_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS8_5list3INS8_5valueISC_EENSG_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS8_5list3INS8_5valueISC_EENSG_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
pub fn stub_5885b8() -> ! {
    todo!("0x5885b8 __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS8_5list3INS8_5valueISC_EENSG_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0x588740 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_588740() -> ! {
    todo!("0x588740 __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

// 0x5888cc — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEEvT_
// type: int(void)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEEvT_")]
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEEvT_
pub fn stub_5888cc() -> ! {
    todo!("0x5888cc void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>)")
}

// 0x588a64 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
pub fn stub_588a64() -> ! {
    todo!("0x588a64 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x588a80 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")]
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
pub fn stub_588a80() -> ! {
    todo!("0x588a80 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)")
}

// 0x588a98 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int(void)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_588a98() -> ! {
    todo!("0x588a98 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x588c20 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, void *, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_588c20() -> ! {
    todo!("0x588c20 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x588da4 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_13InsertServiceEEESsS6_ENS9_5list3INS9_5valueISD_EENSH_ISsEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_588da4() -> ! {
    todo!("0x588da4 void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x588eac — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEclIPFvS6_SsNS_10shared_ptrINS4_8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEclIPFvS6_SsNS_10shared_ptrINS4_8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::operator()<void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEclIPFvS6_SsNS_10shared_ptrINS4_8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_588eac() -> ! {
    todo!("0x588eac void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")
}

// 0x58905c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::InsertService>,std::string,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEESsNS_10shared_ptrINS6_8InstanceEEEENS3_5list3INS3_5valueIS8_EENSF_ISsEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_58905c() -> ! {
    todo!("0x58905c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::InsertService>,std::string,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x5891f8 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_
// type: int(void)
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_")]
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)")]
// was: __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_
pub fn stub_5891f8() -> ! {
    todo!("0x5891f8 boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::list3(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)")
}

// 0x589364 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_
// type: int __fastcall(int)
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_")]
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_
pub fn stub_589364() -> ! {
    todo!("0x589364 boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)")
}

// 0x5894d0 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEEEC2ES7_S8_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEEEC2ES7_S8_")]
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::InsertService>>,boost::_bi::value<std::string>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEEEC2ES7_S8_
pub fn stub_5894d0() -> ! {
    todo!("0x5894d0 boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>)")
}

// 0x5895d8 — __ZN5boost10shared_ptrIN3RBX13InsertServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: int(void)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13InsertServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
#[doc(alias = "rbx_core::SharedPtr<RBX::InsertService>::shared_ptr<RBX::InsertService>(rbx_core::WeakPtr<RBX::InsertService> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIN3RBX13InsertServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
pub fn stub_5895d8() -> ! {
    todo!("0x5895d8 boost::shared_ptr<RBX::InsertService>::shared_ptr<RBX::InsertService>(boost::weak_ptr<RBX::InsertService> const&,boost::detail::sp_nothrow_tag)")
}

// 0x589654 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_SsS6_
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_SsS6_")]
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::InsertService::*>::fireEvent(RBX::InsertService*,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_SsS6_
pub fn stub_589654() -> ! {
    todo!("0x589654 RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::InsertService::*>::fireEvent(RBX::InsertService*,std::string,boost::shared_ptr<RBX::Instance>)const")
}

// 0x5897c8 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceESsS6_
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceESsS6_")]
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_13InsertServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceESsS6_
pub fn stub_5897c8() -> ! {
    todo!("0x5897c8 RBX::Reflection::RemoteEventDescImpl<2,RBX::InsertService,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,std::string,boost::shared_ptr<RBX::Instance>)")
}

// 0x589934 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE24safe_static_do_get_mutexEv
pub fn stub_589934() -> ! {
    todo!("0x589934 rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::safe_static_do_get_mutex(void)")
}

// 0x589a2c — __ZN3rbx7signals16signal_with_argsILi3EFvSsiiEEclESsii
// type: int(void)
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi3EFvSsiiEEclESsii")]
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,int,int)>::operator()(std::string,int,int)")]
// was: __ZN3rbx7signals16signal_with_argsILi3EFvSsiiEEclESsii
pub fn stub_589a2c() -> ! {
    todo!("0x589a2c rbx::signals::signal_with_args<3,void ()(std::string,int,int)>::operator()(std::string,int,int)")
}

// 0x589cb4 — __ZN3rbx7signals6signalIFvSsiiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvSsiiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvSsiiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_589cb4() -> ! {
    todo!("0x589cb4 rbx::signals::signal<void ()(std::string,int,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot> &)")
}

// 0x589e14 — __ZN3rbx7signals6signalIFvSsiiEE8on_errorERSt9exception
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvSsiiEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvSsiiEE8on_errorERSt9exception
pub fn stub_589e14() -> ! {
    todo!("0x589e14 rbx::signals::signal<void ()(std::string,int,int)>::on_error(std::exception &)")
}

// 0x589e3c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSERKS7_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSERKS7_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSERKS7_
pub fn stub_589e3c() -> ! {
    todo!("0x589e3c boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot> const&)")
}

// 0x589e60 — __ZN3rbx7signals6signalIFvSsiiEE24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvSsiiEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsiiEE24safe_static_do_get_mutexEv
pub fn stub_589e60() -> ! {
    todo!("0x589e60 rbx::signals::signal<void ()(std::string,int,int)>::safe_static_do_get_mutex(void)")
}

// 0x589f58 — __ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
// was: __ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_589f58() -> ! {
    todo!("0x589f58 rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")
}

// 0x589fcc — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev
pub fn stub_589fcc() -> ! {
    todo!("0x589fcc rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x589ff8 — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev
pub fn stub_589ff8() -> ! {
    todo!("0x589ff8 rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x58a0cc — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs
pub fn stub_58a0cc() -> ! {
    todo!("0x58a0cc rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::call(std::string,std::string)")
}

// 0x58a0e8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs
pub fn stub_58a0e8() -> ! {
    todo!("0x58a0e8 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::call(std::string,std::string)")
}

// 0x58a104 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsSsEENS0_5list2IRSsSG_EEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsSsEENS0_5list2IRSsSG_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list2<std::string &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string> &,boost::_bi::list2<std::string &,std::string &> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsSsEENS0_5list2IRSsSG_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_58a104() -> ! {
    todo!("0x58a104 void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list2<std::string &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string> &,boost::_bi::list2<std::string &,std::string &> &,int)")
}

// 0x58a2ac — __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsSsEclEPS3_SsSs
// type: int(void)
#[doc(alias = "__ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsSsEclEPS3_SsSs")]
#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>::operator()(RBX::InsertService*,std::string,std::string)const")]
// was: __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsSsEclEPS3_SsSs
pub fn stub_58a2ac() -> ! {
    todo!("0x58a2ac boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>::operator()(RBX::InsertService*,std::string,std::string)const")
}

// 0x58a470 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev
pub fn stub_58a470() -> ! {
    todo!("0x58a470 rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::~callable()")
}

// 0x58a49c — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev
pub fn stub_58a49c() -> ! {
    todo!("0x58a49c rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::~callable()")
}

// 0x58a570 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
// was: __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_58a570() -> ! {
    todo!("0x58a570 rbx::signals::connection rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")
}

// 0x58a5e4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_
pub fn stub_58a5e4() -> ! {
    todo!("0x58a5e4 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot*)")
}

// 0x58a608 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEED1Ev
pub fn stub_58a608() -> ! {
    todo!("0x58a608 rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x58a634 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_13InsertServiceESsS6_EENSA_5list3INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEEEEEEED0Ev
pub fn stub_58a634() -> ! {
    todo!("0x58a634 rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x58a708 — __ZNK3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot9connectedEv
#[doc(alias = "__ZNK3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot9connectedEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot9connectedEv
pub fn stub_58a708() -> ! {
    todo!("0x58a708 rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot::connected(void)const")
}

// 0x58a714 — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_E4callESsS7_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_E4callESsS7_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_E4callESsS7_
pub fn stub_58a714() -> ! {
    todo!("0x58a714 rbx::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,boost::shared_ptr<RBX::Instance>)>::call(std::string,boost::shared_ptr<RBX::Instance>)")
}

// 0x58a730 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_E4callESsS7_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_E4callESsS7_")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_E4callESsS7_
pub fn stub_58a730() -> ! {
    todo!("0x58a730 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,boost::shared_ptr<RBX::Instance>)>::call(std::string,boost::shared_ptr<RBX::Instance>)")
}

// 0x58a74c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS_10shared_ptrINS3_8InstanceEEEEENS0_5list2IRSsRSG_EEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS_10shared_ptrINS3_8InstanceEEEEENS0_5list2IRSsRSG_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list2<std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS_10shared_ptrINS3_8InstanceEEEEENS0_5list2IRSsRSG_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_58a74c() -> ! {
    todo!("0x58a74c void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<std::string &,boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list2<std::string &,boost::shared_ptr<RBX::Instance>&> &,int)")
}

// 0x58a8c0 — __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS_10shared_ptrINS2_8InstanceEEEEclEPS3_SsS6_
// type: int(void)
#[doc(alias = "__ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS_10shared_ptrINS2_8InstanceEEEEclEPS3_SsS6_")]
#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::InsertService*,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS_10shared_ptrINS2_8InstanceEEEEclEPS3_SsS6_
pub fn stub_58a8c0() -> ! {
    todo!("0x58a8c0 boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>::operator()(RBX::InsertService*,std::string,boost::shared_ptr<RBX::Instance>)const")
}

// 0x58aa4c — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot22safe_static_init_mutexEv
pub fn stub_58aa4c() -> ! {
    todo!("0x58aa4c rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")
}

// 0x58aa50 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot24safe_static_do_get_mutexEv
pub fn stub_58aa50() -> ! {
    todo!("0x58aa50 rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")
}

// 0x58ab40 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD0Ev
pub fn stub_58ab40() -> ! {
    todo!("0x58ab40 rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot::~slot()")
}

// 0x58ac14 — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_ED1Ev
pub fn stub_58ac14() -> ! {
    todo!("0x58ac14 rbx::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,boost::shared_ptr<RBX::Instance>)>::~callable()")
}

