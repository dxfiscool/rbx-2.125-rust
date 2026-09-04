// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX:: + Instance|DataModel|Workspace | batch addresses sorted
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr

#![allow(non_snake_case, dead_code, unused_variables)]
use rbx_core::WeakPtr;
/// Rust model of `RBX::DataModel` (IDA `0x28dcb8`): the game root. Only the
/// `enable_shared_from_this` weak owner is modeled so far (same `+40`
/// discipline as `Instance`, cf. `weak_from`); service tables, jobs, and
/// workspace linkage land with the first methods that touch them.
#[derive(Default)]
pub struct DataModel {
    pub weak_owner: WeakPtr<DataModel>,
    /// Byte `+3436` behind `setRemoteBuildMode` (IDA `0x419fd8`) /
    /// `getRemoteBuildMode` (IDA `0x419fe0`).
    pub remote_build_mode: bool,
    /// String at `+3440` behind `setServerSaveUrl` (IDA `0x419fe8`).
    pub server_save_url: String,
    /// String at `+3444` behind `setScreenshotSEOInfo` (IDA `0x41d3a4`).
    pub screenshot_seo: String,
    /// String at `+3448` behind `setVideoSEOInfo` (IDA `0x41d3ac`).
    pub video_seo: String,
    /// String at `+2988` behind `setUiMessage` (IDA `0x41c284`) /
    /// `clearUiMessage` (IDA `0x41c28c`).
    pub ui_message: String,
    /// Word `+865` behind `setPlaceVersion` (IDA `0x41d210`).
    pub place_version: i32,
    /// Word at `+0xD7C` behind `setPlaceID` (IDA `0x41d260`).
    pub place_id: i32,
    /// Byte `+3456` behind `activateExperimentalFeatures` (IDA `0x41d2c8`).
    pub experimental_features: bool,
    /// Words `+866`/`+867` behind `setCreatorID` (IDA `0x41d2d0`).
    pub creator_id: i32,
    pub creator_type: i32,
    /// Word `+868` behind `setGenre` (IDA `0x41d320`).
    pub genre: i32,
    /// Words `+869`/`+870` behind `setGear` (IDA `0x41d340`); `+870`
    /// doubles as the allowed-gear bitmask read by `isGearTypeAllowed`
    /// (IDA `0x41d390`, `+3480`).
    pub gear_genre: i32,
    pub gear_allowed: i32,
    /// 0-arg member signal at `+2784` fired by `setGear` (IDA `0x41d37e`).
    pub gear_changed: rbx_core::signal::Signal<()>,
    /// Arbiter member at `+184` behind `getSyncronizationArbiter` (IDA
    /// `0x41e84c`).
    pub sync_arbiter: crate::instance::SyncArbiter,
    /// Raw `IMetric*` at `+3000` behind `setNetworkMetric` (IDA `0x427db8`);
    /// unretained, hence dangerous.
    pub network_metric: *const crate::instance::IMetric,
    /// Byte `+3108` behind `gameLoaded` (IDA `0x430004`).
    pub game_loaded: bool,
    /// 0-arg member signal at `+2792` fired by `gameLoaded` (IDA `0x430018`).
    pub game_loaded_signal: rbx_core::signal::Signal<()>,
}

// 46 stubs in this file | batch range 0xef04..0x28838c

// 0x179e8 — __ZN3RBX9DataModel10serverSaveEv
#[doc(alias = "RBX::DataModel::serverSave(void)")]
// was: RBX::DataModel::serverSave(void)
pub fn stub_0x179e8() -> ! {
    todo!("0x179e8 RBX::DataModel::serverSave(void)")
}

// 0x179ec — __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE
#[doc(alias = "RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)")]
// was: RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)
pub fn stub_0x179ec() -> ! {
    todo!("0x179ec RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)")
}

// 0x179f0 — __ZN3RBX9DataModel12internalSaveENS_9ContentIdE
#[doc(alias = "RBX::DataModel::internalSave(RBX::ContentId)")]
// was: RBX::DataModel::internalSave(RBX::ContentId)
pub fn stub_0x179f0() -> ! {
    todo!("0x179f0 RBX::DataModel::internalSave(RBX::ContentId)")
}

// 0x179f4 — __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
#[doc(alias = "RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
// was: RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
pub fn stub_0x179f4() -> ! {
    todo!("0x179f4 RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x2ba54 — __ZL16executeUrlScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeUrlScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
// was: executeUrlScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
pub fn stub_0x2ba54() -> ! {
    todo!("0x2ba54 executeUrlScript(boost::shared_ptr<RBX::DataModel>,std::string const&)")
}

// 0x2bdb0 — __ZL19executeSignedScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeSignedScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
// was: executeSignedScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
pub fn stub_0x2bdb0() -> ! {
    todo!("0x2bdb0 executeSignedScript(boost::shared_ptr<RBX::DataModel>,std::string const&)")
}

// 0x2bf74 — __ZL13executeScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
// was: executeScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
pub fn stub_0x2bf74() -> ! {
    todo!("0x2bf74 executeScript(boost::shared_ptr<RBX::DataModel>,std::string const&)")
}

// 0x2d544 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)
pub fn stub_0x2d544() -> ! {
    todo!("0x2d544 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)")
}

// 0x2d660 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEvPNS8_9DataModelEE6invokeERNS1_15function_bufferESN_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_0x2d660() -> ! {
    todo!("0x2d660 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")
}

// 0x2d67c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x2d67c() -> ! {
    todo!("0x2d67c bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const")
}

// 0x2d768 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x2d768() -> ! {
    todo!("0x2d768 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x2d884 — __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_0x2d884() -> ! {
    todo!("0x2d884 void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)")
}

// 0x31348 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_0x31348() -> ! {
    todo!("0x31348 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")
}

// 0x3a2ec — __ZN5boost10shared_ptrIN3RBX9DataModelEEaSINS1_16OverlayDataModelEEERS3_ONS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>& rbx_core::SharedPtr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(rbx_core::SharedPtr<RBX::OverlayDataModel> &&)")]
// was: boost::shared_ptr<RBX::DataModel>& boost::shared_ptr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(boost::shared_ptr<RBX::OverlayDataModel> &&)
pub fn stub_0x3a2ec() -> ! {
    todo!("0x3a2ec boost::shared_ptr<RBX::DataModel>& boost::shared_ptr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(boost::shared_ptr<RBX::OverlayDataModel> &&)")
}

// 0x3ecf0 — __ZN10RobloxView9RenderJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerEN5boost10shared_ptrINS1_9DataModelEEE
#[doc(alias = "RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)")]
// was: RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,boost::shared_ptr<RBX::DataModel>)
pub fn stub_0x3ecf0() -> ! {
    todo!("0x3ecf0 RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,boost::shared_ptr<RBX::DataModel>)")
}

// 0x3f094 — __ZN10RobloxView9RenderJob16stepDataModelJobERKN3RBX13TaskScheduler3Job5StatsE
#[doc(alias = "RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
// was: RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)
pub fn stub_0x3f094() -> ! {
    todo!("0x3f094 RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x40318 — __ZN5boost8weak_ptrIN3RBX9DataModelEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::WeakPtr<RBX::DataModel>::weak_ptr<RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)")]
// was: boost::weak_ptr<RBX::DataModel>::weak_ptr<RBX::DataModel>(boost::shared_ptr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)
pub fn stub_0x40318() -> ! {
    todo!("0x40318 boost::weak_ptr<RBX::DataModel>::weak_ptr<RBX::DataModel>(boost::shared_ptr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)")
}

// 0x49e7c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)
pub fn stub_0x49e7c() -> ! {
    todo!("0x49e7c rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)")
}

// 0x4b164 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6insertEPNS6_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)
pub fn stub_0x4b164() -> ! {
    todo!("0x4b164 rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")
}

// 0x4b374 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSEPS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)
pub fn stub_0x4b374() -> ! {
    todo!("0x4b374 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)")
}

// 0x4b418 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)
pub fn stub_0x4b418() -> ! {
    todo!("0x4b418 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)")
}

// 0x4b4bc — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)
pub fn stub_0x4b4bc() -> ! {
    todo!("0x4b4bc rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)")
}

// 0x4b4c0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)
pub fn stub_0x4b4c0() -> ! {
    todo!("0x4b4c0 rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)")
}

// 0x4b5b8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,rbx::signals::signal<void ()(RBX::DataModel *)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,rbx::signals::signal<void ()(RBX::DataModel *)>*)
pub fn stub_0x4b5b8() -> ! {
    todo!("0x4b5b8 rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,rbx::signals::signal<void ()(RBX::DataModel *)>*)")
}

// 0x4b6b4 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()
pub fn stub_0x4b6b4() -> ! {
    todo!("0x4b6b4 rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")
}

// 0x4b788 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()
pub fn stub_0x4b788() -> ! {
    todo!("0x4b788 rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")
}

// 0x4b860 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)
pub fn stub_0x4b860() -> ! {
    todo!("0x4b860 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)")
}

// 0x4b970 — __ZNK3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const
pub fn stub_0x4b970() -> ! {
    todo!("0x4b970 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const")
}

// 0x4b97c — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)
pub fn stub_0x4b97c() -> ! {
    todo!("0x4b97c rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")
}

// 0x4b984 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)
pub fn stub_0x4b984() -> ! {
    todo!("0x4b984 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")
}

// 0x4b98c — __ZNK5boost9function1IvPN3RBX9DataModelEEclES3_
#[doc(alias = "boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const")]
// was: boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const
pub fn stub_0x4b98c() -> ! {
    todo!("0x4b98c boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const")
}

// 0x4ba50 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6removeEPNS6_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)
pub fn stub_0x4ba50() -> ! {
    todo!("0x4ba50 rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")
}

// 0x4bb40 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)
pub fn stub_0x4bb40() -> ! {
    todo!("0x4bb40 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)")
}

// 0x4bb44 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)
pub fn stub_0x4bb44() -> ! {
    todo!("0x4bb44 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)")
}

// 0x4bc34 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()
pub fn stub_0x4bc34() -> ! {
    todo!("0x4bc34 rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")
}

// 0x4bd08 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()
pub fn stub_0x4bd08() -> ! {
    todo!("0x4bd08 rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")
}

// 0x4bde0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()
pub fn stub_0x4bde0() -> ! {
    todo!("0x4bde0 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")
}

// 0x4be8c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()
pub fn stub_0x4be8c() -> ! {
    todo!("0x4be8c rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")
}

// 0x4bf3c — __ZN5boost9function1IvPN3RBX9DataModelEE13assign_to_ownERKS4_
#[doc(alias = "boost::function1<void,RBX::DataModel *>::assign_to_own(boost::function1<void,RBX::DataModel *> const&)")]
// was: boost::function1<void,RBX::DataModel *>::assign_to_own(boost::function1<void,RBX::DataModel *> const&)
pub fn stub_0x4bf3c() -> ! {
    todo!("0x4bf3c boost::function1<void,RBX::DataModel *>::assign_to_own(boost::function1<void,RBX::DataModel *> const&)")
}

// 0x4bf6c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x4bf6c() -> ! {
    todo!("0x4bf6c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")
}

// 0x4bfcc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::DataModel>::invoke(boost::detail::function::function_buffer &,RBX::DataModel)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::DataModel>::invoke(boost::detail::function::function_buffer &,RBX::DataModel)
pub fn stub_0x4bfcc() -> ! {
    todo!("0x4bfcc boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::DataModel>::invoke(boost::detail::function::function_buffer &,RBX::DataModel)")
}

// 0x282a48 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS6_5list2INS6_5valueISB_EENSF_ISsEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>)
pub fn stub_0x282a48() -> ! {
    todo!("0x282a48 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>)")
}

// 0x282c00 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESJ_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_0x282c00() -> ! {
    todo!("0x282c00 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")
}

// 0x282c1c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS3_3Lua13WeakThreadRefEEESsENS8_5list2INS8_5valueISD_EENSH_ISsEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x282c1c() -> ! {
    todo!("0x282c1c bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")
}

// 0x282da8 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS3_3Lua13WeakThreadRefEEESsENS8_5list2INS8_5valueISD_EENSH_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x282da8() -> ! {
    todo!("0x282da8 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x282f78 — __ZN5boost3_bi5list2INS0_5valueINS_13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEEEENS2_ISsEEEclIPFvS7_SsENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>::operator()<void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_0x282f78() -> ! {
    todo!("0x282f78 void boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>::operator()<void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string) &,boost::_bi::list1<RBX::DataModel *&> &,int)")
}

