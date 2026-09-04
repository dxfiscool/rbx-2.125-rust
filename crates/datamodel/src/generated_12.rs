// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0xef04..0x25c83c | total filtered 10215, remaining 7646 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::data_model::{
    ContentId, DataModel, DataModelCallback, DataModelConnection, DataModelJobBase,
    DataModelSignal, DataModelSlot, DataModelSlotFn, ExecuteError, FunctionMarshaller,
    LegacyLock, ManualResetEvent, RenderJob, UploadErrorCallback, UploadSuccessCallback,
    ViewFlagMarshallerBind, ViewGameFn, ViewGameMarshallerBind, RENDER_JOB_INTERVAL,
};
use crate::generated_05::{FunctorOp, Instance, SaveFilter};
use crate::generated_296::OverlayDataModel;
use crate::instance::{
    Camera, CRenderSettingsItem, ControllerService, LoginService, ObjcInstanceBind,
    Players, RunService, TaskSchedulerSettings,
};
use rbx_core::WeakPtr;
use rbx_core::shared_ptr::{ControlBlockPd, CreatableInstanceDeleter};

/// Rust model of the `RobloxView` workspace-binding surface behind
/// `RobloxView::bindWorkspace` (IDA `0x380a4`): the attached model/overlay
/// plus the enabled flag stored through the `+13` vtable slot (0x381d6).
/// The `+3`/`+4` routing slots collapse into the stored handles.
#[derive(Default)]
pub struct WorkspaceBinding {
    pub model: Option<SharedPtr<DataModel>>,
    pub overlay: Option<SharedPtr<OverlayDataModel>>,
    pub active: bool,
}
/// Rust model of the `DataModel*` objc bind triple behind 0x4bf6c/0x4bfcc:
/// same 12-byte buffer shape as `instance::ObjcInstanceBind`, but the
/// callee takes the raw `DataModel*` — `arg<1>` rides the call frame
/// unretained (cf. IDA 0x4bfcc tail-call).
#[derive(Clone, Copy)]
pub struct ObjcDataModelBind {
    pub func: fn(*mut (), *mut (), *mut DataModel),
    pub target: *mut (),
    pub selector: *mut (),
}

// 0xef04 — __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")]
// was: boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)
pub fn stub_ef04() -> SharedPtr<CRenderSettingsItem> {
    // IDA 0xef04: default-construct + single-owner adoption. Canonical body
    // lives at `crate::instance::stub_0xef04` (same bytes); delegate so the
    // two shards cannot drift.
    crate::instance::stub_0xef04()
}

// 0xefb4 — __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_efb4(ptr: *mut CRenderSettingsItem, deleter: CreatableInstanceDeleter) -> SharedPtr<CRenderSettingsItem> {
    // IDA 0xefb4: store px + `shared_count` ctor + null-px `accept_owner`
    // skip. See `crate::instance::stub_0xefb4`.
    // SAFETY: `ptr` must be null or a live model-space pointer owned by the caller.
    crate::instance::stub_0xefb4(ptr, deleter)
}

// 0xf098 — __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f098(ptr: *mut CRenderSettingsItem, deleter: CreatableInstanceDeleter) -> ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter> {
    // IDA 0xf098: `new sp_counted_impl_pd` with use/weak counts at 1.
    // See `crate::instance::stub_0xf098`.
    // SAFETY: `ptr` must be a live model-space pointer owned by the caller.
    crate::instance::stub_0xf098(ptr, deleter)
}

// 0xf198 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_f198(block: *mut ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter>) {
    // IDA 0xf198: `BX LR` — empty. See `crate::instance::stub_0xf198`.
    crate::instance::stub_0xf198(block)
}

// 0xf19c — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_f19c(block: *mut ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter>) {
    // IDA 0xf19c: `predelete` + null-px early-out + deleter delete.
    // See `crate::instance::stub_0xf19c`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0xf19c(block)
}

// 0xf1bc — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_f1bc(block: *const ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter>, type_name: &str) -> Option<CreatableInstanceDeleter> {
    // IDA 0xf1bc: deleter-name `strcmp`, `this + 0x10` on hit.
    // See `crate::instance::stub_0xf1bc`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0xf1bc(block, type_name)
}

// 0xf1d4 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_f1d4(block: *const ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter>) -> CreatableInstanceDeleter {
    // IDA 0xf1d4: unconditional `this + 0x10`.
    // See `crate::instance::stub_0xf1d4`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0xf1d4(block)
}

// 0x179e8 — __ZN3RBX9DataModel10serverSaveEv
#[doc(alias = "RBX::DataModel::serverSave(void)")]
pub fn stub_179e8(model: &DataModel) {
    // IDA 0x179e8: single `BX LR` — the mobile build's body is empty.
    // Canonical body lives at `crate::data_model::stub_0x179e8`.
    crate::data_model::stub_0x179e8(model)
}

// 0x179ec — __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE
#[doc(alias = "RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)")]
pub fn stub_179ec(model: &DataModel, id: ContentId, done: Box<dyn FnOnce(bool) + Send>) {
    // IDA 0x179ec: single `BX LR` — async save is stripped; the completion
    // is never invoked. See `crate::data_model::stub_0x179ec`.
    crate::data_model::stub_0x179ec(model, id, done)
}

// 0x179f0 — __ZN3RBX9DataModel12internalSaveENS_9ContentIdE
#[doc(alias = "RBX::DataModel::internalSave(RBX::ContentId)")]
pub fn stub_179f0(model: &DataModel, id: ContentId) {
    // IDA 0x179f0: single `BX LR`. See `crate::data_model::stub_0x179f0`.
    crate::data_model::stub_0x179f0(model, id)
}

// 0x179f4 — __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
#[doc(alias = "RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
// was: RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
pub fn stub_179f4(
    model: &DataModel,
    url: &str,
    filter: SaveFilter,
    on_success: UploadSuccessCallback,
    on_error: UploadErrorCallback,
) {
    // IDA 0x179f4: hollowed mobile build — empty-result build + drop,
    // neither callback fires. See `crate::data_model::stub_0x179f4`.
    crate::data_model::stub_0x179f4(model, url, filter, on_success, on_error)
}

// 0x2ba54 — __ZL16executeUrlScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeUrlScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
// was: executeUrlScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
pub fn stub_2ba54(model: &SharedPtr<DataModel>, url: &str) -> Result<(), ExecuteError> {
    // IDA 0x2ba54: `Impersonator(7)` + non-URL early-out + `LegacyLock` +
    // `ContentProvider::getContent` + `executeSignedScript`. See
    // `crate::data_model::stub_0x2ba54`.
    crate::data_model::stub_0x2ba54(model, url)
}

// 0x2bdb0 — __ZL19executeSignedScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeSignedScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
// was: executeSignedScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
pub fn stub_2bdb0(model: &SharedPtr<DataModel>, script: &str) -> Result<(), ExecuteError> {
    // IDA 0x2bdb0: `verifyScriptSignature` + `executeScript`. See
    // `crate::data_model::stub_0x2bdb0`.
    crate::data_model::stub_0x2bdb0(model, script)
}

// 0x2bf74 — __ZL13executeScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
// was: executeScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
pub fn stub_2bf74(model: &SharedPtr<DataModel>, script: &str) -> Result<(), ExecuteError> {
    // IDA 0x2bf74: `LegacyLock(dm, 1)` + `create<ScriptContext>` +
    // `fromTrustedSource` + `executeInNewThread(7, "Start Script")`. See
    // `crate::data_model::stub_0x2bf74`.
    crate::data_model::stub_0x2bf74(model, script)
}

// 0x2d544 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)
pub fn stub_2d544(slot: &mut DataModelCallback, bind: ViewGameMarshallerBind, invoke: ViewGameFn) {
    // IDA 0x2d544: retain bound game + install vtable + release old buffer.
    // See `crate::data_model::stub_0x2d544`.
    crate::data_model::stub_0x2d544(slot, bind, invoke)
}

// 0x2d660 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEvPNS8_9DataModelEE6invokeERNS1_15function_bufferESN_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_2d660(slot: &DataModelCallback, dm: *mut DataModel) {
    // IDA 0x2d660: stored-vtable `invoke` tail-calling `list3::operator()`.
    // See `crate::data_model::stub_0x2d660`.
    // SAFETY: `dm` must point to a valid `DataModel`.
    crate::data_model::stub_0x2d660(slot, dm)
}

// 0x2d67c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const
pub fn stub_2d67c(slot: &mut DataModelCallback, bind: ViewGameMarshallerBind, invoke: ViewGameFn) -> bool {
    // IDA 0x2d67c: `basic_vtable::assign_to` — same copy as 0x2d544, always
    // reports success. See `crate::data_model::stub_0x2d67c`.
    crate::data_model::stub_0x2d67c(slot, bind, invoke)
}

// 0x2d768 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_2d768(slot: &mut DataModelCallback, bind: ViewGameMarshallerBind, invoke: ViewGameFn) -> bool {
    // IDA 0x2d768: the `function_obj_tag` overload of 0x2d67c — identical
    // body. See `crate::data_model::stub_0x2d768`.
    crate::data_model::stub_0x2d768(slot, bind, invoke)
}

// 0x2d884 — __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_2d884(bind: &ViewGameMarshallerBind, dm: *mut DataModel, invoke: ViewGameFn) {
    // IDA 0x2d884: retain game + `fn(view, game, marshaller, dm)` + release.
    // See `crate::data_model::stub_0x2d884`.
    // SAFETY: `dm` must point to a valid `DataModel`.
    crate::data_model::stub_0x2d884(bind, dm, invoke)
}

// 0x31348 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn stub_31348(bind: &ViewFlagMarshallerBind, invoke: fn(view: *const (), flag: i8, marshaller: *const ())) {
    // IDA 0x31348: buffer loads + tail-call, incoming `DataModel*`
    // discarded. See `crate::data_model::stub_0x31348`.
    crate::data_model::stub_0x31348(bind, invoke)
}

// 0x31678 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LoginServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)")]
// was: boost::shared_ptr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)
pub fn stub_31678() -> SharedPtr<LoginService> {
    // IDA 0x31678: `operator new(0x70)` + `LoginService()` default ctor +
    // adoption. See `crate::instance::stub_0x31678`.
    crate::instance::stub_0x31678()
}

// 0x31728 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12LoginServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const&)
pub fn stub_31728(dst: &mut SharedPtr<Instance>, src: &SharedPtr<Instance>) -> SharedPtr<Instance> {
    // IDA 0x31728 (decompiled): `shared_ptr<Instance>::operator=` from
    // `shared_ptr<LoginService>` — `shared_count` copy addrefs the source
    // (0x31782), the adjusted px is stored (0x3178e), the old pi is
    // released (0x31790-0x317a0). The `LoginService*` → `Instance*`
    // adjustment is unmodeled (no hierarchy yet), so the source arrives
    // post-adjustment — the same convention as
    // `data_model::stub_0x3a2ec`. Clone-assign + return is the addref /
    // store / release / return-`*this` path.
    *dst = SharedPtr::clone(src);
    SharedPtr::clone(dst)
}

// 0x319ec — __ZN5boost10shared_ptrIN3RBX12LoginServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_319ec(ptr: *mut LoginService, deleter: CreatableInstanceDeleter) -> SharedPtr<LoginService> {
    // IDA 0x319ec: store px + `shared_count` ctor + null-px skip. See
    // `crate::instance::stub_0x319ec`.
    // SAFETY: `ptr` must be null or a live model-space pointer owned by the caller.
    crate::instance::stub_0x319ec(ptr, deleter)
}

// 0x31aec — __ZN5boost6detail12shared_countC2IPN3RBX12LoginServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_31aec(ptr: *mut LoginService, deleter: CreatableInstanceDeleter) -> ControlBlockPd<LoginService, CreatableInstanceDeleter> {
    // IDA 0x31aec: `new sp_counted_impl_pd` with counts at 1. See
    // `crate::instance::stub_0x31aec`.
    // SAFETY: `ptr` must be a live model-space pointer owned by the caller.
    crate::instance::stub_0x31aec(ptr, deleter)
}

// 0x31bec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_31bec(block: *mut ControlBlockPd<LoginService, CreatableInstanceDeleter>) {
    // IDA 0x31bec: `BX LR` — empty. See `crate::instance::stub_0x31bec`.
    crate::instance::stub_0x31bec(block)
}

// 0x31bf0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_31bf0(block: *mut ControlBlockPd<LoginService, CreatableInstanceDeleter>) {
    // IDA 0x31bf0: D0 storage release. See `crate::instance::stub_0x31bf0`.
    // SAFETY: `block` must be a live box pointer never used again.
    crate::instance::stub_0x31bf0(block)
}

// 0x31bf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_31bf4(block: *mut ControlBlockPd<LoginService, CreatableInstanceDeleter>) {
    // IDA 0x31bf4: `dispose` runs deleter + owned delete. See
    // `crate::instance::stub_0x31bf4`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x31bf4(block)
}

// 0x31c14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_31c14(block: *const ControlBlockPd<LoginService, CreatableInstanceDeleter>, type_name: &str) -> Option<CreatableInstanceDeleter> {
    // IDA 0x31c14: deleter-name `strcmp`, `this + 0x10` on hit. See
    // `crate::instance::stub_0x31c14`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x31c14(block, type_name)
}

// 0x31c2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_31c2c(block: *const ControlBlockPd<LoginService, CreatableInstanceDeleter>) -> CreatableInstanceDeleter {
    // IDA 0x31c2c: unconditional `this + 0x10`. See
    // `crate::instance::stub_0x31c2c`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x31c2c(block)
}

// 0x31cd0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
pub fn stub_31cd0(src: &ObjcInstanceBind, dst: &mut ObjcInstanceBind, op: FunctorOp) -> bool {
    // IDA 0x31cd0: `functor_manager::manage` over the objc bind triple —
    // clone / destroy / move dispatch. See `crate::instance::stub_0x31cd0`.
    crate::instance::stub_0x31cd0(src, dst, op)
}

// 0x31d30 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)
pub fn stub_31d30(bind: &ObjcInstanceBind, arg: &SharedPtr<Instance>) {
    // IDA 0x31d30: buffer unwrap + tail-call into `list3::operator()`.
    // See `crate::instance::stub_0x31d30`.
    crate::instance::stub_0x31d30(bind, arg)
}

// 0x31d48 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)
pub fn stub_31d48(bind: &ObjcInstanceBind, arg: &SharedPtr<Instance>) {
    // IDA 0x31d48: `objc_msgSend` dispatch on the retained bind triple.
    // See `crate::instance::stub_0x31d48`.
    crate::instance::stub_0x31d48(bind, arg)
}

// 0x324fc — __ZN5boost10shared_ptrIN3RBX21TaskSchedulerSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_324fc(ptr: *mut TaskSchedulerSettings, deleter: CreatableInstanceDeleter) -> SharedPtr<TaskSchedulerSettings> {
    // IDA 0x324fc: store px + `shared_count` ctor + null-px skip. See
    // `crate::instance::stub_0x324fc`.
    // SAFETY: `ptr` must be null or a live model-space pointer owned by the caller.
    crate::instance::stub_0x324fc(ptr, deleter)
}

// 0x325fc — __ZN5boost6detail12shared_countC2IPN3RBX21TaskSchedulerSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_325fc(ptr: *mut TaskSchedulerSettings, deleter: CreatableInstanceDeleter) -> ControlBlockPd<TaskSchedulerSettings, CreatableInstanceDeleter> {
    // IDA 0x325fc: `new sp_counted_impl_pd` with counts at 1. See
    // `crate::instance::stub_0x325fc`.
    // SAFETY: `ptr` must be a live model-space pointer owned by the caller.
    crate::instance::stub_0x325fc(ptr, deleter)
}

// 0x326fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_326fc(block: *mut ControlBlockPd<TaskSchedulerSettings, CreatableInstanceDeleter>) {
    // IDA 0x326fc: `BX LR` — empty. See `crate::instance::stub_0x326fc`.
    crate::instance::stub_0x326fc(block)
}

// 0x32700 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_32700(block: *mut ControlBlockPd<TaskSchedulerSettings, CreatableInstanceDeleter>) {
    // IDA 0x32700: `dispose` runs deleter + owned delete. See
    // `crate::instance::stub_0x32700`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x32700(block)
}

// 0x33454 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_33454(block: *const ControlBlockPd<Players, CreatableInstanceDeleter>, type_name: &str) -> Option<CreatableInstanceDeleter> {
    // IDA 0x33454: deleter-name `strcmp`, `this + 0x10` on hit. See
    // `crate::instance::stub_0x33454`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x33454(block, type_name)
}

// 0x3346c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_3346c(block: *const ControlBlockPd<Players, CreatableInstanceDeleter>) -> CreatableInstanceDeleter {
    // IDA 0x3346c: unconditional `this + 0x10`. See
    // `crate::instance::stub_0x3346c`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x3346c(block)
}

// 0x380a4 — __ZN10RobloxView13bindWorkspaceEN5boost10shared_ptrIN3RBX8ViewBaseEEENS1_INS2_9DataModelEEENS1_INS2_16OverlayDataModelEEE
#[doc(alias = "RobloxView::bindWorkspace(rbx_core::SharedPtr<RBX::ViewBase>,rbx_core::SharedPtr<RBX::DataModel>,rbx_core::SharedPtr<RBX::OverlayDataModel>)")]
// was: RobloxView::bindWorkspace(boost::shared_ptr<RBX::ViewBase>,boost::shared_ptr<RBX::DataModel>,boost::shared_ptr<RBX::OverlayDataModel>)
pub fn stub_380a4(binding: &mut WorkspaceBinding, model: &SharedPtr<DataModel>, overlay: &Option<SharedPtr<OverlayDataModel>>) {
    // IDA 0x380a4 (decompiled): null-overlay skips the first half (0x380d2);
    // each half takes `LegacyLock(target, 1)` (0x38112/0x38180) and routes
    // the handle through the view vtable (`+4` overlay at 0x3814c, `+3`
    // model at 0x381ba), then the `+13` slot stores 1 (0x381d6) and both
    // guards drop. The routing slots collapse into the stored handles; the
    // overlay guard retains via clone since `LegacyLock` only models
    // `DataModel`.
    if let Some(o) = overlay {
        let _overlay_guard = SharedPtr::clone(o);
        binding.overlay = Some(SharedPtr::clone(o));
    }
    let _guard = LegacyLock::new(model, LegacyLock::WRITE_TASK);
    binding.model = Some(SharedPtr::clone(model));
    binding.active = true;
}

// 0x3a2ec — __ZN5boost10shared_ptrIN3RBX9DataModelEEaSINS1_16OverlayDataModelEEERS3_ONS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>& rbx_core::SharedPtr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(rbx_core::SharedPtr<RBX::OverlayDataModel> &&)")]
// was: boost::shared_ptr<RBX::DataModel>& boost::shared_ptr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(boost::shared_ptr<RBX::OverlayDataModel> &&)
pub fn stub_3a2ec(dst: &mut Option<SharedPtr<DataModel>>, src: &mut Option<SharedPtr<DataModel>>) {
    // IDA 0x3a2ec: move-assign from `shared_ptr<OverlayDataModel>&&` —
    // steal, null source, release previous. `OverlayDataModel*` needs no
    // adjustment, so both sides arrive post-adjustment. See
    // `crate::data_model::stub_0x3a2ec`.
    crate::data_model::stub_0x3a2ec(dst, src)
}

// 0x3a798 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6CameraEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)")]
// was: boost::shared_ptr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)
pub fn stub_3a798() -> SharedPtr<Camera> {
    // IDA 0x3a798: `operator new` + `Camera()` default ctor + adoption.
    // See `crate::instance::stub_0x3a798`.
    crate::instance::stub_0x3a798()
}

// 0x3aa10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3aa10(block: *mut ControlBlockPd<Camera, CreatableInstanceDeleter>) {
    // IDA 0x3aa10: D0 storage release. See `crate::instance::stub_0x3aa10`.
    // SAFETY: `block` must be a live box pointer never used again.
    crate::instance::stub_0x3aa10(block)
}

// 0x3aa18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_3aa18(block: *const ControlBlockPd<Camera, CreatableInstanceDeleter>, type_name: &str) -> Option<CreatableInstanceDeleter> {
    // IDA 0x3aa18: deleter-name `strcmp`, `this + 0x10` on hit. See
    // `crate::instance::stub_0x3aa18`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x3aa18(block, type_name)
}

// 0x3afe0 — __ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_3afe0(ptr: *mut RunService, deleter: CreatableInstanceDeleter) -> SharedPtr<RunService> {
    // IDA 0x3afe0: store px + `shared_count` ctor + null-px skip. See
    // `crate::instance::stub_0x3afe0`.
    // SAFETY: `ptr` must be null or a live model-space pointer owned by the caller.
    crate::instance::stub_0x3afe0(ptr, deleter)
}

// 0x3b008 — __ZN5boost6detail12shared_countC2IPN3RBX10RunServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_3b008(ptr: *mut RunService, deleter: CreatableInstanceDeleter) -> ControlBlockPd<RunService, CreatableInstanceDeleter> {
    // IDA 0x3b008: `new sp_counted_impl_pd` with counts at 1. See
    // `crate::instance::stub_0x3b008`.
    // SAFETY: `ptr` must be a live model-space pointer owned by the caller.
    crate::instance::stub_0x3b008(ptr, deleter)
}

// 0x3b108 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3b108(block: *mut ControlBlockPd<RunService, CreatableInstanceDeleter>) {
    // IDA 0x3b108: `BX LR` — empty. See `crate::instance::stub_0x3b108`.
    crate::instance::stub_0x3b108(block)
}

// 0x3b110 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_3b110(block: *mut ControlBlockPd<RunService, CreatableInstanceDeleter>) {
    // IDA 0x3b110: `dispose` runs deleter + owned delete. See
    // `crate::instance::stub_0x3b110`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x3b110(block)
}

// 0x3b130 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_3b130(block: *const ControlBlockPd<RunService, CreatableInstanceDeleter>, type_name: &str) -> Option<CreatableInstanceDeleter> {
    // IDA 0x3b130: deleter-name `strcmp`, `this + 0x10` on hit. See
    // `crate::instance::stub_0x3b130`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x3b130(block, type_name)
}

// 0x3b148 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_3b148(block: *const ControlBlockPd<RunService, CreatableInstanceDeleter>) -> CreatableInstanceDeleter {
    // IDA 0x3b148: unconditional `this + 0x10`. See
    // `crate::instance::stub_0x3b148`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x3b148(block)
}

// 0x3b674 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17ControllerServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)")]
// was: boost::shared_ptr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)
pub fn stub_3b674() -> SharedPtr<ControllerService> {
    // IDA 0x3b674: `operator new` + `ControllerService()` default ctor +
    // adoption. See `crate::instance::stub_0x3b674`.
    crate::instance::stub_0x3b674()
}

// 0x3b724 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17ControllerServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ControllerService>(rbx_core::SharedPtr<RBX::ControllerService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ControllerService>(boost::shared_ptr<RBX::ControllerService> const&)
pub fn stub_3b724(dst: &mut SharedPtr<Instance>, src: &SharedPtr<Instance>) -> SharedPtr<Instance> {
    // IDA 0x3b724: `shared_ptr<Instance>::operator=` from
    // `shared_ptr<ControllerService>` — same addref / store / release /
    // return-`*this` path as 0x31728; the source arrives post-adjustment.
    *dst = SharedPtr::clone(src);
    SharedPtr::clone(dst)
}

// 0x3b9e8 — __ZN5boost10shared_ptrIN3RBX17ControllerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_3b9e8(ptr: *mut ControllerService, deleter: CreatableInstanceDeleter) -> SharedPtr<ControllerService> {
    // IDA 0x3b9e8: store px + `shared_count` ctor + null-px skip. See
    // `crate::instance::stub_0x3b9e8`.
    // SAFETY: `ptr` must be null or a live model-space pointer owned by the caller.
    crate::instance::stub_0x3b9e8(ptr, deleter)
}

// 0x3ba10 — __ZN5boost6detail12shared_countC2IPN3RBX17ControllerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_3ba10(ptr: *mut ControllerService, deleter: CreatableInstanceDeleter) -> ControlBlockPd<ControllerService, CreatableInstanceDeleter> {
    // IDA 0x3ba10: `new sp_counted_impl_pd` with counts at 1. See
    // `crate::instance::stub_0x3ba10`.
    // SAFETY: `ptr` must be a live model-space pointer owned by the caller.
    crate::instance::stub_0x3ba10(ptr, deleter)
}

// 0x3bb10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3bb10(block: *mut ControlBlockPd<ControllerService, CreatableInstanceDeleter>) {
    // IDA 0x3bb10: `BX LR` — empty. See `crate::instance::stub_0x3bb10`.
    crate::instance::stub_0x3bb10(block)
}

// 0x3bb18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_3bb18(block: *mut ControlBlockPd<ControllerService, CreatableInstanceDeleter>) {
    // IDA 0x3bb18: `dispose` runs deleter + owned delete. See
    // `crate::instance::stub_0x3bb18`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x3bb18(block)
}

// 0x3bb38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_3bb38(block: *const ControlBlockPd<ControllerService, CreatableInstanceDeleter>, type_name: &str) -> Option<CreatableInstanceDeleter> {
    // IDA 0x3bb38: deleter-name `strcmp`, `this + 0x10` on hit. See
    // `crate::instance::stub_0x3bb38`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x3bb38(block, type_name)
}

// 0x3bb50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_3bb50(block: *const ControlBlockPd<ControllerService, CreatableInstanceDeleter>) -> CreatableInstanceDeleter {
    // IDA 0x3bb50: unconditional `this + 0x10`. See
    // `crate::instance::stub_0x3bb50`.
    // SAFETY: `block` must point to a valid block.
    crate::instance::stub_0x3bb50(block)
}

// 0x3bbf8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>::operator=(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: boost::shared_ptr<RBX::Instance>::operator=(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_3bbf8(dst: &mut SharedPtr<Instance>, src: &SharedPtr<Instance>) -> SharedPtr<Instance> {
    // IDA 0x3bbf8: `shared_ptr<Instance>::operator=(const
    // shared_ptr<Instance>&)` — same addref / store / release /
    // return-`*this` path as 0x31728, same-type instantiation.
    *dst = SharedPtr::clone(src);
    SharedPtr::clone(dst)
}

// 0x3e190 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3e190(block: *mut ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter>) {
    // IDA 0x3e190: D0 storage release only. See `crate::instance::stub_0x3e190`.
    // (Same release shape as 0x459f0c.)
    // SAFETY: `block` must be a live box pointer never used again.
    crate::instance::stub_0x3e190(block)
}

// 0x3ecf0 — __ZN10RobloxView9RenderJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerEN5boost10shared_ptrINS1_9DataModelEEE
#[doc(alias = "RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)")]
// was: RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,boost::shared_ptr<RBX::DataModel>)
pub fn stub_3ecf0(
    view: *const (),
    marshaller: *const FunctionMarshaller,
    model: &SharedPtr<DataModel>,
) -> RenderJob {
    // IDA 0x3ecf0 (decompiled): `DataModelJob("Render", TaskType 2, false,
    // arbiter, 0.02)` (0x3ed50-0x3ed86); vtable install (0x3eda4);
    // view/marshaller words; `weak_ptr(dm)` (0x3edce); `CEvent(false)`
    // (0x3ede6).
    RenderJob {
        base: DataModelJobBase {
            name: "Render",
            task_type: 2,
            flag: false,
            arbiter: None,
            interval: RENDER_JOB_INTERVAL,
        },
        view,
        marshaller,
        model: SharedPtr::downgrade(model),
        event: ManualResetEvent::new(false),
    }
}

// 0x40318 — __ZN5boost8weak_ptrIN3RBX9DataModelEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::WeakPtr<RBX::DataModel>::weak_ptr<RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)")]
// was: boost::weak_ptr<RBX::DataModel>::weak_ptr<RBX::DataModel>(boost::shared_ptr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)
pub fn stub_40318(model: &SharedPtr<DataModel>) -> WeakPtr<DataModel> {
    // IDA 0x40318: `weak_ptr<DataModel>` converting ctor — shares the
    // control block without retaining. `Arc::downgrade` is the same
    // non-owning take.
    SharedPtr::downgrade(model)
}

// 0x49e7c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)")]
pub fn stub_49e7c(signal: &DataModelSignal, callback: DataModelSlotFn) -> DataModelConnection {
    // IDA 0x49e7c (decompiled): 32-byte `callable` slot allocation (0x49eb6)
    // + `callable` ctor (0x49ede) + `insert` (0x49f06) + weak-ref install
    // (0x49f12-0x49f18). The slot starts unlinked; `insert` links it.
    let slot = SharedPtr::new(DataModelSlot::new(callback));
    signal.insert(&slot);
    DataModelConnection::new(&slot)
}

// 0x4b164 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6insertEPNS6_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
pub fn stub_4b164(signal: &DataModelSignal, slot: &SharedPtr<DataModelSlot>) {
    // IDA 0x4b164: `signal::insert` — links the slot into the list.
    signal.insert(slot)
}

// 0x4b374 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSEPS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)")]
pub fn stub_4b374(dst: &mut Option<SharedPtr<DataModelSlot>>, src: &Option<SharedPtr<DataModelSlot>>) {
    // IDA 0x4b374 (decompiled): null-checked addref (0x4b3be-0x4b3c8),
    // store (0x4b3d0), release of the previous (0x4b3d2-0x4b3d8).
    // Clone-assign + drop is the same path; the raw `slot*` is the
    // nullable `Option`.
    *dst = src.clone();
}

// 0x4b418 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)")]
pub fn stub_4b418(dst: &mut Option<SharedPtr<DataModelSlot>>, src: &Option<SharedPtr<DataModelSlot>>) {
    // IDA 0x4b418 (decompiled): same addref / store / release path as
    // 0x4b374 for the `const intrusive_ptr&` overload.
    *dst = src.clone();
}

// 0x4b4bc — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)")]
pub fn stub_4b4bc() {
    // IDA 0x4b4bc: `signal::safe_static_init_mutex` — one-time init of the
    // signal static mutex. The mutex lives in `DataModelSignal`, so init is
    // construction; nothing to do here.
}

// 0x4b4c0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)")]
pub fn stub_4b4c0() {
    // IDA 0x4b4c0: `signal::safe_static_do_get_mutex` — returns the signal
    // static mutex. Callers use the mutex inside `DataModelSignal`; the
    // accessor itself collapses.
}

// 0x4b5b8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,rbx::signals::signal<void ()(RBX::DataModel *)>*)")]
pub fn stub_4b5b8(callback: DataModelSlotFn) -> SharedPtr<DataModelSlot> {
    // IDA 0x4b5b8 (decompiled): zero the words (0x4b5ea-0x4b612) +
    // `assign_to_own` copy of the function (0x4b638). A fresh unlinked slot
    // with the cloned closure is the same state.
    SharedPtr::new(DataModelSlot::new(callback))
}

// 0x4b6b4 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
pub fn stub_4b6b4(_slot: &SharedPtr<DataModelSlot>) {
    // IDA 0x4b6b4: `callable_slot::~callable_slot` D1 — vtable reset +
    // `function::clear`; storage is kept by the D1. Drop glue, no-op.
}

// 0x4b788 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
pub fn stub_4b788(_slot: &SharedPtr<DataModelSlot>) {
    // IDA 0x4b788: `callable_slot::~callable_slot` D0 — same teardown as
    // D1, then `operator delete`. The free collapses into Rust ownership
    // (caller drops the handle).
}

// 0x4b860 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)")]
pub fn stub_4b860(signal: &DataModelSignal, slot: &SharedPtr<DataModelSlot>) {
    // IDA 0x4b860 (decompiled): linked-check (0x4b88a), static-mutex take
    // (0x4b8ca-0x4b8ec), unlink + `remove` (0x4b8f0-0x4b8fe), unlock. The
    // mutex take collapses into `DataModelSignal`'s own lock.
    if slot.is_linked() {
        slot.set_linked(false);
        signal.remove(slot);
    }
}

// 0x4b970 — __ZNK3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const")]
pub fn stub_4b970(slot: &DataModelSlot) -> bool {
    // IDA 0x4b970 (decompiled): `return *(a1 + 12) != 0` — the link word.
    slot.is_linked()
}

// 0x4b97c — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
pub fn stub_4b97c(slot: &DataModelSlot, dm: *mut DataModel) {
    // IDA 0x4b97c (decompiled): `call` tail-calls
    // `function1::operator()` on the `+16` stored function. Same path as
    // `DataModelSignal::emit` for one slot.
    // SAFETY: `dm` must point to a valid `DataModel`.
    slot.call(dm)
}

// 0x4b984 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
pub fn stub_4b984(slot: &DataModelSlot, dm: *mut DataModel) {
    // IDA 0x4b984: `ZThn4` non-virtual thunk adjusting into 0x4b97c. The
    // adjustment collapses; direct forward.
    // SAFETY: same contract as 0x4b97c.
    stub_4b97c(slot, dm)
}

// 0x4b98c — __ZNK5boost9function1IvPN3RBX9DataModelEEclES3_
#[doc(alias = "boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const")]
pub fn stub_4b98c(callback: &Option<DataModelSlotFn>, dm: *mut DataModel) {
    // IDA 0x4b98c (decompiled): empty function throws
    // `boost::bad_function_call` (0x4b9da-0x4ba1e) — a throw becomes a
    // panic here; else the stored-vtable invoke runs (0x4b9ec).
    // SAFETY: `dm` must point to a valid `DataModel`.
    match callback {
        Some(invoke) => invoke(dm),
        None => panic!("boost::bad_function_call"),
    }
}

// 0x4ba50 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6removeEPNS6_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
pub fn stub_4ba50(signal: &DataModelSignal, slot: &SharedPtr<DataModelSlot>) {
    // IDA 0x4ba50 (decompiled): `!intrusive_ptr_expired` asserts
    // (0x4ba64-0x4ba98, 0x4baee-0x4bafa) that fall through, then the
    // linked-list unlink (0x4baca-0x4bae2). The asserts collapse into the
    // retain-filter in `DataModelSignal::remove`.
    signal.remove(slot)
}

// 0x4bb40 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)")]
pub fn stub_4bb40() {
    // IDA 0x4bb40: `slot::safe_static_init_mutex` — same collapse as
    // 0x4b4bc for the slot static mutex.
}

// 0x4bb44 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_4bb44() {
    // IDA 0x4bb44: `slot::safe_static_do_get_mutex` — same collapse as
    // 0x4b4c0.
}

// 0x4bc34 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
pub fn stub_4bc34(_slot: &SharedPtr<DataModelSlot>) {
    // IDA 0x4bc34: `callable::~callable` D1 — same drop-glue shape as
    // 0x4b6b4.
}

// 0x4bd08 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
pub fn stub_4bd08(_slot: &SharedPtr<DataModelSlot>) {
    // IDA 0x4bd08: `callable::~callable` D0 — same shape as 0x4b788; the
    // free collapses into Rust ownership.
}

// 0x4bde0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
pub fn stub_4bde0(_slot: &SharedPtr<DataModelSlot>) {
    // IDA 0x4bde0: `slot::~slot` D1 — vtable reset + member drops, storage
    // kept. Drop glue, no-op (same treatment as `DataModelSlot::drop`,
    // which unlinks).
}

// 0x4be8c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
pub fn stub_4be8c(_slot: &SharedPtr<DataModelSlot>) {
    // IDA 0x4be8c: `slot::~slot` D0 — same teardown as D1, then `operator
    // delete`. The free collapses into Rust ownership.
}

// 0x4bf3c — __ZN5boost9function1IvPN3RBX9DataModelEE13assign_to_ownERKS4_
#[doc(alias = "boost::function1<void,RBX::DataModel *>::assign_to_own(boost::function1<void,RBX::DataModel *> const&)")]
pub fn stub_4bf3c(dst: &mut Option<DataModelSlotFn>, src: &Option<DataModelSlotFn>) {
    // IDA 0x4bf3c (decompiled): null skip (0x4bf3c), small-functor word
    // copy (0x4bf44-0x4bf54) vs vtable clone into the buffer (0x4bf6a).
    // `Arc` clone-assign is the same copy-or-share path.
    *dst = src.clone();
}

// 0x4bf6c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_4bf6c(src: &ObjcInstanceBind, dst: &mut ObjcInstanceBind, op: FunctorOp) -> bool {
    // IDA 0x4bf6c: `functor_manager::manage` over the `DataModel*` objc
    // bind triple — byte-identical buffer discipline to 0x31cd0 (the
    // callee word differs, the manage path does not). Delegate so the two
    // instantiations cannot drift.
    crate::instance::stub_0x31cd0(src, dst, op)
}

// 0x4bfcc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::DataModel>::invoke(boost::detail::function::function_buffer &,RBX::DataModel)")]
pub fn stub_4bfcc(bind: &ObjcDataModelBind, dm: *mut DataModel) {
    // IDA 0x4bfcc: buffer unwrap + tail-call into the `DataModel*`
    // `list3::operator()` — `(bind.func)(bound_target, bound_selector,
    // dm)`. The raw `arg<1>` rides the call frame unretained, unlike the
    // `SharedPtr` variant at 0x31d48.
    // SAFETY: `dm` must point to a valid `DataModel` for the call duration.
    (bind.func)(bind.target, bind.selector, dm)
}

// 0x258688 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11HttpServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)")]
// was: boost::shared_ptr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)
pub fn stub_258688() -> ! {
    todo!("0x258688 rbx_core::SharedPtr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)")
}

// 0x258738 — __ZN5boost10shared_ptrIN3RBX11HttpServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_258738() -> ! {
    todo!("0x258738 rbx_core::SharedPtr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2588e8 — __ZN5boost6detail12shared_countC2IPN3RBX11HttpServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_2588e8() -> ! {
    todo!("0x2588e8 boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2589f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2589f0() -> ! {
    todo!("0x2589f0 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2589f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2589f4() -> ! {
    todo!("0x2589f4 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2589f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_2589f8() -> ! {
    todo!("0x2589f8 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x258a18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_258a18() -> ! {
    todo!("0x258a18 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x258a30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_258a30() -> ! {
    todo!("0x258a30 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x25bc38 — __ZNK3RBX5Light12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Light::askSetParent(RBX::Instance const*)const")]
pub fn stub_25bc38() -> ! {
    todo!("0x25bc38 RBX::Light::askSetParent(RBX::Instance const*)const")
}

// 0x25bc60 — __ZNK3RBX5Light11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Light::askAddChild(RBX::Instance const*)const")]
pub fn stub_25bc60() -> ! {
    todo!("0x25bc60 RBX::Light::askAddChild(RBX::Instance const*)const")
}

// 0x25c4d0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SpotLightEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)")]
// was: boost::shared_ptr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)
pub fn stub_25c4d0() -> ! {
    todo!("0x25c4d0 rbx_core::SharedPtr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)")
}

// 0x25c580 — __ZN5boost10shared_ptrIN3RBX9SpotLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_25c580() -> ! {
    todo!("0x25c580 rbx_core::SharedPtr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x25c730 — __ZN5boost6detail12shared_countC2IPN3RBX9SpotLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_25c730() -> ! {
    todo!("0x25c730 boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x25c838 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_25c838() -> ! {
    todo!("0x25c838 boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x25c83c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_25c83c() -> ! {
    todo!("0x25c83c boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}