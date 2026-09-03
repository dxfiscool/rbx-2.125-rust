//! boost_core_f — 150 boost stubs (EA-ordered, next uncovered after boost_core_e).
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost", sorted by EA, next 150 uncovered.
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; sanitized alias uses `rbx_core::SharedPtr` not `boost::`.
//! Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr.

#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>)")]
// 0x4fd848 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS6_5list2INS6_5valueIPSA_EENSF_ISsEEEEEEEEvT_
pub fn stub_4fd848() -> ! {
    todo!("0x4fd848 __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS6_5list2INS6_5valueIPSA_EENSF_ISsEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x4fd994 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
pub fn stub_4fd994() -> ! {
    todo!("0x4fd994 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// 0x4fd9b0 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_
pub fn stub_4fd9b0() -> ! {
    todo!("0x4fd9b0 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// 0x4fd9c4 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_4GameERKSsEENS8_5list2INS8_5valueIPSC_EENSH_ISsEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_4fd9c4() -> ! {
    todo!("0x4fd9c4 __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_4GameERKSsEENS8_5list2INS8_5valueIPSC_EENSH_ISsEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x4fdb00 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_4GameERKSsEENS8_5list2INS8_5valueIPSC_EENSH_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_4fdb00() -> ! {
    todo!("0x4fdb00 __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_4GameERKSsEENS8_5list2INS8_5valueIPSC_EENSH_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x4fdc38 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_4GameERKSsEENS8_5list2INS8_5valueIPSC_EENSH_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_4fdc38() -> ! {
    todo!("0x4fdc38 __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_4GameERKSsEENS8_5list2INS8_5valueIPSC_EENSH_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>::operator()<RBX::DataModel *>(RBX::DataModel * &)")]
// 0x4fdd08 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS0_5list2INS0_5valueIPS5_EENSA_ISsEEEEEclIPNS4_9DataModelEEEvRT_
pub fn stub_4fdd08() -> ! {
    todo!("0x4fdd08 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS0_5list2INS0_5valueIPS5_EENSA_ISsEEEEEclIPNS4_9DataModelEEEvRT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x4fdd20 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_4fdd20() -> ! {
    todo!("0x4fdd20 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Game *>,boost::_bi::value<std::string>>::list2(boost::_bi::value<RBX::Game *>,boost::_bi::value<std::string>)")]
// 0x4fde5c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4GameEEENS2_ISsEEEC2ES6_S7_
pub fn stub_4fde5c() -> ! {
    todo!("0x4fde5c __ZN5boost3_bi5list2INS0_5valueIPN3RBX4GameEEENS2_ISsEEEC2ES6_S7_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CommonVerbs>::shared_ptr<RBX::CommonVerbs>(RBX::CommonVerbs *)")]
// 0x4fe078 — __ZN5boost10shared_ptrIN3RBX11CommonVerbsEEC2IS2_EEPT_
// was: boost::shared_ptr<RBX::CommonVerbs>::shared_ptr<RBX::CommonVerbs>(RBX::CommonVerbs *)
pub fn stub_4fe078<T>(px: Box<T>) -> crate::SharedPtr<T> {
    // IDA 0x4fe078: px = p; pi = new sp_counted_impl_p<CommonVerbs>(p), checked_delete(p) on throw.
    crate::shared_ptr::shared_ptr_from_raw(px)
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CommonVerbs>(RBX::CommonVerbs *)")]
// 0x4fe14c — __ZN5boost6detail12shared_countC2IN3RBX11CommonVerbsEEEPT_
pub fn stub_4fe14c<T>(px: Box<T>) -> crate::shared_ptr::ControlBlockP<T> {
    // IDA 0x4fe14c: new 0x10; use = 1; weak = 1; vtable set; px = p.
    crate::shared_ptr::ControlBlockP::new(px)
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::~sp_counted_impl_p()")]
// 0x4fec88 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEED1Ev
pub fn stub_4fec88<T>(_block: &mut crate::shared_ptr::ControlBlockP<T>) {
    // IDA 0x4fec88: empty — base class handles release.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::~sp_counted_impl_p()")]
// 0x4fec8c — __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEED0Ev
pub fn stub_4fec8c<T>(block: Box<crate::shared_ptr::ControlBlockP<T>>) {
    // IDA 0x4fec8c (thunk): operator delete(this).
    drop(block);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::dispose(void)")]
// 0x4fec90 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEE7disposeEv
pub fn stub_4fec90<T>(block: &mut crate::shared_ptr::ControlBlockP<T>) {
    // IDA 0x4fec90: px = this+12; if (px) { CommonVerbs::~CommonVerbs(px); operator delete(px); }
    block.dispose();
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::get_deleter(std::type_info const&)")]
// 0x4fed34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEE11get_deleterERKSt9type_info
pub fn stub_4fed34<T>(block: &crate::shared_ptr::ControlBlockP<T>) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x4fed34: return 0 — a _p block never carries a deleter.
    block.get_deleter()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::get_untyped_deleter(void)")]
// 0x4fed38 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11CommonVerbsEE19get_untyped_deleterEv
pub fn stub_4fed38<T>(block: &crate::shared_ptr::ControlBlockP<T>) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x4fed38: return 0.
    block.get_untyped_deleter()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptInformationProvider> RBX::Creatable<RBX::Instance>::create<RBX::ScriptInformationProvider>(void)")]
// 0x4fed3c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_25ScriptInformationProviderEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ScriptInformationProvider> RBX::Creatable<RBX::Instance>::create<RBX::ScriptInformationProvider>(void)
pub fn stub_4fed3c() -> ! {
    todo!("0x4fed3c __ZN3RBX9CreatableINS_8InstanceEE6createINS_25ScriptInformationProviderEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ScriptInformationProvider>(rbx_core::SharedPtr<RBX::ScriptInformationProvider> const&)")]
// 0x4fedec — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_25ScriptInformationProviderEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ScriptInformationProvider>(boost::shared_ptr<RBX::ScriptInformationProvider> const&)
pub fn stub_4fedec() -> ! {
    todo!("0x4fedec __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_25ScriptInformationProviderEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4fee20 — __ZN5boost6detail12shared_countC2IPN3RBX25ScriptInformationProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4fee20() -> ! {
    todo!("0x4fee20 __ZN5boost6detail12shared_countC2IPN3RBX25ScriptInformationProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4fef28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX25ScriptInformationProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4fef28() -> ! {
    todo!("0x4fef28 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX25ScriptInformationProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4fef2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX25ScriptInformationProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4fef2c() -> ! {
    todo!("0x4fef2c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX25ScriptInformationProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GameSettings> RBX::Creatable<RBX::Instance>::create<RBX::GameSettings>(void)")]
// 0x4fefb8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12GameSettingsEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::GameSettings> RBX::Creatable<RBX::Instance>::create<RBX::GameSettings>(void)
pub fn stub_4fefb8() -> ! {
    todo!("0x4fefb8 __ZN3RBX9CreatableINS_8InstanceEE6createINS_12GameSettingsEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GameSettings>::shared_ptr<RBX::GameSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4ff068 — __ZN5boost10shared_ptrIN3RBX12GameSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::GameSettings>::shared_ptr<RBX::GameSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4ff068() -> ! {
    todo!("0x4ff068 __ZN5boost10shared_ptrIN3RBX12GameSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4ff130 — __ZN5boost6detail12shared_countC2IPN3RBX12GameSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4ff130() -> ! {
    todo!("0x4ff130 __ZN5boost6detail12shared_countC2IPN3RBX12GameSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4ff238 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4ff238() -> ! {
    todo!("0x4ff238 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4ff23c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4ff23c() -> ! {
    todo!("0x4ff23c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12GameSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ProfanityFilter>::shared_ptr<RBX::ProfanityFilter>(RBX::ProfanityFilter *)")]
// 0x4ff388 — __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEC2IS2_EEPT_
// was: boost::shared_ptr<RBX::ProfanityFilter>::shared_ptr<RBX::ProfanityFilter>(RBX::ProfanityFilter *)
pub fn stub_4ff388() -> ! {
    todo!("0x4ff388 __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEC2IS2_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ProfanityFilter>(RBX::ProfanityFilter *)")]
// 0x4ff45c — __ZN5boost6detail12shared_countC2IN3RBX15ProfanityFilterEEEPT_
pub fn stub_4ff45c() -> ! {
    todo!("0x4ff45c __ZN5boost6detail12shared_countC2IN3RBX15ProfanityFilterEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::~sp_counted_impl_p()")]
// 0x4ff568 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEED1Ev
pub fn stub_4ff568() -> ! {
    todo!("0x4ff568 __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::~sp_counted_impl_p()")]
// 0x4ff56c — __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEED0Ev
pub fn stub_4ff56c() -> ! {
    todo!("0x4ff56c __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::dispose(void)")]
// 0x4ff570 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE7disposeEv
pub fn stub_4ff570() -> ! {
    todo!("0x4ff570 __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::get_deleter(std::type_info const&)")]
// 0x4ff614 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE11get_deleterERKSt9type_info
pub fn stub_4ff614() -> ! {
    todo!("0x4ff614 __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::get_untyped_deleter(void)")]
// 0x4ff618 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE19get_untyped_deleterEv
pub fn stub_4ff618() -> ! {
    todo!("0x4ff618 __ZN5boost6detail17sp_counted_impl_pIN3RBX15ProfanityFilterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ProfanityFilter>::shared_ptr<RBX::ProfanityFilter>(rbx_core::WeakPtr<RBX::ProfanityFilter> const&,boost::detail::sp_nothrow_tag)")]
// 0x4ff61c — __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::ProfanityFilter>::shared_ptr<RBX::ProfanityFilter>(boost::weak_ptr<RBX::ProfanityFilter> const&,boost::detail::sp_nothrow_tag)
pub fn stub_4ff61c() -> ! {
    todo!("0x4ff61c __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ProfanityFilter>::~weak_ptr()")]
// 0x4ff700 — __ZN5boost8weak_ptrIN3RBX15ProfanityFilterEED1Ev
// was: boost::weak_ptr<RBX::ProfanityFilter>::~weak_ptr()
pub fn stub_4ff700() -> ! {
    todo!("0x4ff700 __ZN5boost8weak_ptrIN3RBX15ProfanityFilterEED1Ev")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x4ff818 — __ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_4ff818() -> ! {
    todo!("0x4ff818 __ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::GameSettings,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::GameSettings::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x502760 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12GameSettingsEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<1,RBX::GameSettings,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::GameSettings::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_502760() -> ! {
    todo!("0x502760 __ZNK3RBX10Reflection13EventDescImplILi1ENS_12GameSettingsEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "RBX::GeometryService::getPartsTouchingExtentsWithIgnore(RBX::Extents const&,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const*,int,G3D::Array<RBX::PartInstance *,10,32ul> &)")]
// 0x505188 — __ZN3RBX15GeometryService33getPartsTouchingExtentsWithIgnoreERKNS_7ExtentsEPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS8_EEiRN3G3D5ArrayIPNS_12PartInstanceELi10ELm32EEE
// was: RBX::GeometryService::getPartsTouchingExtentsWithIgnore(RBX::Extents const&,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const*,int,G3D::Array<RBX::PartInstance *,10,32ul> &)
pub fn stub_505188() -> ! {
    todo!("0x505188 __ZN3RBX15GeometryService33getPartsTouchingExtentsWithIgnoreERKNS_7ExtentsEPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS8_EEiRN3G3D5ArrayIPNS_12PartInstanceELi10ELm32EEE")
}

#[doc(alias = "addInstanceToIgnorePrimitiveSet(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &)")]
// 0x505420 — __ZL31addInstanceToIgnorePrimitiveSetN5boost10shared_ptrIN3RBX8InstanceEEERNS_9unordered13unordered_setIPKNS1_9PrimitiveENS_4hashIS8_EESt8equal_toIS8_ESaIS8_EEE
// was: addInstanceToIgnorePrimitiveSet(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &)
pub fn stub_505420() -> ! {
    todo!("0x505420 __ZL31addInstanceToIgnorePrimitiveSetN5boost10shared_ptrIN3RBX8InstanceEEERNS_9unordered13unordered_setIPKNS1_9PrimitiveENS_4hashIS8_EESt8equal_toIS8_ESaIS8_EEE")
}

#[doc(alias = "RBX::GeometryService::getHitLocationFilterDescendents(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const*,RBX::RbxRay,RBX::Primitive **,RBX::CellID &,bool)")]
// 0x505704 — __ZN3RBX15GeometryService31getHitLocationFilterDescendentsEPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EENS_6RbxRayEPPNS_9PrimitiveERNS_6CellIDEb
// was: RBX::GeometryService::getHitLocationFilterDescendents(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const*,RBX::RbxRay,RBX::Primitive **,RBX::CellID &,bool)
pub fn stub_505704() -> ! {
    todo!("0x505704 __ZN3RBX15GeometryService31getHitLocationFilterDescendentsEPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EENS_6RbxRayEPPNS_9PrimitiveERNS_6CellIDEb")
}

#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>> const&)const")]
// 0x505a68 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS2_9unordered13unordered_setIPKNS_9PrimitiveENS2_4hashISB_EESt8equal_toISB_ESaISB_EEEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISH_EEEEEEEEvRKT_
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>> const&)const
pub fn stub_505a68() -> ! {
    todo!("0x505a68 __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS2_9unordered13unordered_setIPKNS_9PrimitiveENS2_4hashISB_EESt8equal_toISB_ESaISB_EEEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISH_EEEEEEEEvRKT_")
}

#[doc(alias = "G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<RBX::Instance>(RBX::Instance *,RBX::RbxRay,rbx_core::SharedPtr<RBX::PartInstance> &,RBX::CellID &,bool)")]
// 0x505bcc — __ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsINS_8InstanceEEEN3G3D7Vector3EPT_NS_6RbxRayERN5boost10shared_ptrINS_12PartInstanceEEERNS_6CellIDEb
// was: G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<RBX::Instance>(RBX::Instance *,RBX::RbxRay,boost::shared_ptr<RBX::PartInstance> &,RBX::CellID &,bool)
pub fn stub_505bcc() -> ! {
    todo!("0x505bcc __ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsINS_8InstanceEEEN3G3D7Vector3EPT_NS_6RbxRayERN5boost10shared_ptrINS_12PartInstanceEEERNS_6CellIDEb")
}

#[doc(alias = "G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const *,RBX::RbxRay,rbx_core::SharedPtr<RBX::PartInstance> &,RBX::CellID &,bool)")]
// 0x505d08 — __ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsIKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS6_EEEEN3G3D7Vector3EPT_NS_6RbxRayERNS4_INS_12PartInstanceEEERNS_6CellIDEb
// was: G3D::Vector3 RBX::GeometryService::getHitLocationPartFilterDescendents<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const *,RBX::RbxRay,boost::shared_ptr<RBX::PartInstance> &,RBX::CellID &,bool)
pub fn stub_505d08() -> ! {
    todo!("0x505d08 __ZN3RBX15GeometryService35getHitLocationPartFilterDescendentsIKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS6_EEEEN3G3D7Vector3EPT_NS_6RbxRayERNS4_INS_12PartInstanceEEERNS_6CellIDEb")
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// 0x5067c4 — __ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperINS_9unordered13unordered_setIPKN3RBX9PrimitiveENS_4hashISA_EESt8equal_toISA_ESaISA_EEEEEEclIPFvNS_10shared_ptrINS7_8InstanceEEERSG_ENS0_5list1IRKSM_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>>&) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_5067c4() -> ! {
    todo!("0x5067c4 __ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperINS_9unordered13unordered_setIPKN3RBX9PrimitiveENS_4hashISA_EESt8equal_toISA_ESaISA_EEEEEEclIPFvNS_10shared_ptrINS7_8InstanceEEERSG_ENS0_5list1IRKSM_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::delete_buckets(void)")]
// 0x506898 — __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
pub fn stub_506898() -> ! {
    todo!("0x506898 __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Primitive const*>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Primitive const*>>(RBX::Primitive const* const&,boost::unordered::detail::emplace_args1<RBX::Primitive const*> const&)")]
// 0x5068e8 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
pub fn stub_5068e8() -> ! {
    todo!("0x5068e8 __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::reserve_for_insert(unsigned long)")]
// 0x506a78 — __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
pub fn stub_506a78() -> ! {
    todo!("0x506a78 __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::create_buckets(unsigned long)")]
// 0x506ac8 — __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
pub fn stub_506ac8() -> ! {
    todo!("0x506ac8 __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::min_buckets_for_size(unsigned long)const")]
// 0x506bf0 — __ZNK5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
pub fn stub_506bf0() -> ! {
    todo!("0x506bf0 __ZNK5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::rehash_impl(unsigned long)")]
// 0x506c80 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
pub fn stub_506c80() -> ! {
    todo!("0x506c80 __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x506cac — __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE
pub fn stub_506cac() -> ! {
    todo!("0x506cac __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>>>::construct(void)")]
// 0x506d00 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIPKN3RBX9PrimitiveEEEEE9constructEv
pub fn stub_506d00() -> ! {
    todo!("0x506d00 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIPKN3RBX9PrimitiveEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Primitive const*>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::find_node_impl<RBX::Primitive const*,std::equal_to<RBX::Primitive const*>>(unsigned long,RBX::Primitive const* const&,std::equal_to<RBX::Primitive const*> const&)const")]
// 0x506d38 — __ZNK5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_
pub fn stub_506d38() -> ! {
    todo!("0x506d38 __ZNK5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_")
}

#[doc(alias = "resetChild(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x508cd4 — __ZL10resetChildN5boost10shared_ptrIN3RBX8InstanceEEE
// was: resetChild(boost::shared_ptr<RBX::Instance>)
pub fn stub_508cd4() -> ! {
    todo!("0x508cd4 __ZL10resetChildN5boost10shared_ptrIN3RBX8InstanceEEE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()")]
// 0x508e58 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()
pub fn stub_508e58() -> ! {
    todo!("0x508e58 __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalAdvancedSettings>::~shared_ptr()")]
// 0x509048 — __ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEED1Ev
// was: boost::shared_ptr<RBX::GlobalAdvancedSettings>::~shared_ptr()
pub fn stub_509048() -> ! {
    todo!("0x509048 __ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEED1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalBasicSettings>::~shared_ptr()")]
// 0x50905c — __ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEED1Ev
// was: boost::shared_ptr<RBX::GlobalBasicSettings>::~shared_ptr()
pub fn stub_50905c() -> ! {
    todo!("0x50905c __ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEED1Ev")
}

#[doc(alias = "void RBX::Instance::visitChildren<void (*)(rbx_core::SharedPtr<RBX::Instance>)>(void (*)(rbx_core::SharedPtr<RBX::Instance>) const&)const")]
// 0x509094 — __ZNK3RBX8Instance13visitChildrenIPFvN5boost10shared_ptrIS0_EEEEEvRKT_
// was: void RBX::Instance::visitChildren<void (*)(boost::shared_ptr<RBX::Instance>)>(void (*)(boost::shared_ptr<RBX::Instance>) const&)const
pub fn stub_509094() -> ! {
    todo!("0x509094 __ZNK3RBX8Instance13visitChildrenIPFvN5boost10shared_ptrIS0_EEEEEvRKT_")
}

#[doc(alias = "rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> const&)")]
// 0x50a0e8 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEESH_EEPT_RKNS_10shared_ptrIT0_EE
// was: rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> const&)
pub fn stub_50a0e8() -> ! {
    todo!("0x50a0e8 __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEESH_EEPT_RKNS_10shared_ptrIT0_EE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>,std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0x50a148 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_50a148() -> ! {
    todo!("0x50a148 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>,std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0x50a180 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_50a180() -> ! {
    todo!("0x50a180 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()")]
// 0x50a250 — __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEED1Ev
pub fn stub_50a250() -> ! {
    todo!("0x50a250 __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_deleter(std::type_info const&)")]
// 0x50a280 — __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE11get_deleterERKSt9type_info
pub fn stub_50a280() -> ! {
    todo!("0x50a280 __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_untyped_deleter(void)")]
// 0x50a298 — __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE19get_untyped_deleterEv
pub fn stub_50a298() -> ! {
    todo!("0x50a298 __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Selection> RBX::Creatable<RBX::Instance>::create<RBX::Selection>(void)")]
// 0x50b39c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SelectionEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Selection> RBX::Creatable<RBX::Instance>::create<RBX::Selection>(void)
pub fn stub_50b39c() -> ! {
    todo!("0x50b39c __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SelectionEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Selection>(rbx_core::SharedPtr<RBX::Selection> const&)")]
// 0x50b44c — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9SelectionEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::Selection>(boost::shared_ptr<RBX::Selection> const&)
pub fn stub_50b44c() -> ! {
    todo!("0x50b44c __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9SelectionEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Selection>::shared_ptr<RBX::Selection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x50b688 — __ZN5boost10shared_ptrIN3RBX9SelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Selection>::shared_ptr<RBX::Selection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_50b688() -> ! {
    todo!("0x50b688 __ZN5boost10shared_ptrIN3RBX9SelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Selection,RBX::Selection>(rbx_core::SharedPtr<RBX::Selection> const*,RBX::Selection *)const")]
// 0x50b750 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9SelectionES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Selection,RBX::Selection>(boost::shared_ptr<RBX::Selection> const*,RBX::Selection *)const
pub fn stub_50b750() -> ! {
    todo!("0x50b750 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9SelectionES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x50b838 — __ZN5boost6detail12shared_countC2IPN3RBX9SelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_50b838() -> ! {
    todo!("0x50b838 __ZN5boost6detail12shared_countC2IPN3RBX9SelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x50b940 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_50b940() -> ! {
    todo!("0x50b940 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x50b948 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_50b948() -> ! {
    todo!("0x50b948 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x50b968 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_50b968() -> ! {
    todo!("0x50b968 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x50b980 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_50b980() -> ! {
    todo!("0x50b980 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>> *)")]
// 0x50caa0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>> *)
pub fn stub_50caa0() -> ! {
    todo!("0x50caa0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>> *)")]
// 0x50cac8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>> *)
pub fn stub_50cac8() -> ! {
    todo!("0x50cac8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x50d538 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EEC2EMS2_FSI_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_50d538() -> ! {
    todo!("0x50d538 __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EEC2EMS2_FSI_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()")]
// 0x50d63c — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()
pub fn stub_50d63c() -> ! {
    todo!("0x50d63c __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x50d6f0 — __ZNK3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_50d6f0() -> ! {
    todo!("0x50d6f0 __ZNK3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::GlobalAdvancedSettings,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::GlobalAdvancedSettings*,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),RBX::Reflection::Variant&)")]
// 0x50d714 — __ZN3RBX10Reflection11Call0HelperINS_22GlobalAdvancedSettingsEMS2_FN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvESI_E4callEPS2_SK_RS7_
// was: RBX::Reflection::Call0Helper<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::GlobalAdvancedSettings*,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),RBX::Reflection::Variant&)
pub fn stub_50d714() -> ! {
    todo!("0x50d714 __ZN3RBX10Reflection11Call0HelperINS_22GlobalAdvancedSettingsEMS2_FN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvESI_E4callEPS2_SK_RS7_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalBasicSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalBasicSettings>(void)")]
// 0x50dd7c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19GlobalBasicSettingsEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::GlobalBasicSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalBasicSettings>(void)
pub fn stub_50dd7c() -> ! {
    todo!("0x50dd7c __ZN3RBX9CreatableINS_8InstanceEE6createINS_19GlobalBasicSettingsEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalBasicSettings>::shared_ptr<RBX::GlobalBasicSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x50de2c — __ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::GlobalBasicSettings>::shared_ptr<RBX::GlobalBasicSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_50de2c() -> ! {
    todo!("0x50de2c __ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GlobalBasicSettings,RBX::GlobalBasicSettings>(rbx_core::SharedPtr<RBX::GlobalBasicSettings> const*,RBX::GlobalBasicSettings *)const")]
// 0x50def4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19GlobalBasicSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GlobalBasicSettings,RBX::GlobalBasicSettings>(boost::shared_ptr<RBX::GlobalBasicSettings> const*,RBX::GlobalBasicSettings *)const
pub fn stub_50def4() -> ! {
    todo!("0x50def4 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19GlobalBasicSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x50dfdc — __ZN5boost6detail12shared_countC2IPN3RBX19GlobalBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_50dfdc() -> ! {
    todo!("0x50dfdc __ZN5boost6detail12shared_countC2IPN3RBX19GlobalBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x50e0e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_50e0e4() -> ! {
    todo!("0x50e0e4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x50e0e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_50e0e8() -> ! {
    todo!("0x50e0e8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x50e0ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_50e0ec() -> ! {
    todo!("0x50e0ec __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x50e10c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_50e10c() -> ! {
    todo!("0x50e10c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x50e124 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_50e124() -> ! {
    todo!("0x50e124 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalAdvancedSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalAdvancedSettings>(void)")]
// 0x50e12c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_22GlobalAdvancedSettingsEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::GlobalAdvancedSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalAdvancedSettings>(void)
pub fn stub_50e12c() -> ! {
    todo!("0x50e12c __ZN3RBX9CreatableINS_8InstanceEE6createINS_22GlobalAdvancedSettingsEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalAdvancedSettings>::shared_ptr<RBX::GlobalAdvancedSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x50e1dc — __ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::GlobalAdvancedSettings>::shared_ptr<RBX::GlobalAdvancedSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_50e1dc() -> ! {
    todo!("0x50e1dc __ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GlobalAdvancedSettings,RBX::GlobalAdvancedSettings>(rbx_core::SharedPtr<RBX::GlobalAdvancedSettings> const*,RBX::GlobalAdvancedSettings *)const")]
// 0x50e2a4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_22GlobalAdvancedSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GlobalAdvancedSettings,RBX::GlobalAdvancedSettings>(boost::shared_ptr<RBX::GlobalAdvancedSettings> const*,RBX::GlobalAdvancedSettings *)const
pub fn stub_50e2a4() -> ! {
    todo!("0x50e2a4 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_22GlobalAdvancedSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x50e38c — __ZN5boost6detail12shared_countC2IPN3RBX22GlobalAdvancedSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_50e38c() -> ! {
    todo!("0x50e38c __ZN5boost6detail12shared_countC2IPN3RBX22GlobalAdvancedSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x50e494 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_50e494() -> ! {
    todo!("0x50e494 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x50e498 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_50e498() -> ! {
    todo!("0x50e498 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x50e49c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_50e49c() -> ! {
    todo!("0x50e49c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x50e4bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_50e4bc() -> ! {
    todo!("0x50e4bc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x50e4d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_50e4d4() -> ! {
    todo!("0x50e4d4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::DescribedBase> RBX::shared_from<RBX::Reflection::DescribedBase>(RBX::Reflection::DescribedBase*)")]
// 0x50edb8 — __ZN3RBX11shared_fromINS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPS5_
// was: boost::shared_ptr<RBX::Reflection::DescribedBase> RBX::shared_from<RBX::Reflection::DescribedBase>(RBX::Reflection::DescribedBase*)
pub fn stub_50edb8() -> ! {
    todo!("0x50edb8 __ZN3RBX11shared_fromINS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPS5_")
}

#[doc(alias = "RBX::GuiBuilder::buildChatMenu(RBX::ChatOption *,std::string,rbx_core::SharedPtr<RBX::UnifiedWidget>)")]
// 0x520754 — __ZN3RBX10GuiBuilder13buildChatMenuEPNS_10ChatOptionESsN5boost10shared_ptrINS_13UnifiedWidgetEEE
// was: RBX::GuiBuilder::buildChatMenu(RBX::ChatOption *,std::string,boost::shared_ptr<RBX::UnifiedWidget>)
pub fn stub_520754() -> ! {
    todo!("0x520754 __ZN3RBX10GuiBuilder13buildChatMenuEPNS_10ChatOptionESsN5boost10shared_ptrINS_13UnifiedWidgetEEE")
}

#[doc(alias = "RBX::GuiBuilder::updatePerformanceBasedStat(rbx_core::SharedPtr<RBX::TextDisplay>,float,float,float,float,bool)")]
// 0x520b54 — __ZN3RBX10GuiBuilder26updatePerformanceBasedStatEN5boost10shared_ptrINS_11TextDisplayEEEffffb
// was: RBX::GuiBuilder::updatePerformanceBasedStat(boost::shared_ptr<RBX::TextDisplay>,float,float,float,float,bool)
pub fn stub_520b54() -> ! {
    todo!("0x520b54 __ZN3RBX10GuiBuilder26updatePerformanceBasedStatEN5boost10shared_ptrINS_11TextDisplayEEEffffb")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::EquationDisplay> RBX::Creatable<RBX::Instance>::create<RBX::EquationDisplay,std::string,std::string>(std::string,std::string)")]
// 0x520c28 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15EquationDisplayESsSsEEN5boost10shared_ptrIT_EET0_T1_
// was: boost::shared_ptr<RBX::EquationDisplay> RBX::Creatable<RBX::Instance>::create<RBX::EquationDisplay,std::string,std::string>(std::string,std::string)
pub fn stub_520c28() -> ! {
    todo!("0x520c28 __ZN3RBX9CreatableINS_8InstanceEE6createINS_15EquationDisplayESsSsEEN5boost10shared_ptrIT_EET0_T1_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay>::operator=(rbx_core::SharedPtr<RBX::TextDisplay> const&)")]
// 0x520ce0 — __ZN5boost10shared_ptrIN3RBX11TextDisplayEEaSERKS3_
// was: boost::shared_ptr<RBX::TextDisplay>::operator=(boost::shared_ptr<RBX::TextDisplay> const&)
pub fn stub_520ce0() -> ! {
    todo!("0x520ce0 __ZN5boost10shared_ptrIN3RBX11TextDisplayEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TopMenuBar>::operator=(rbx_core::SharedPtr<RBX::TopMenuBar> const&)")]
// 0x520ebc — __ZN5boost10shared_ptrIN3RBX10TopMenuBarEEaSERKS3_
// was: boost::shared_ptr<RBX::TopMenuBar>::operator=(boost::shared_ptr<RBX::TopMenuBar> const&)
pub fn stub_520ebc() -> ! {
    todo!("0x520ebc __ZN5boost10shared_ptrIN3RBX10TopMenuBarEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Frame> RBX::Creatable<RBX::Instance>::create<RBX::Frame>(void)")]
// 0x520ef4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5FrameEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Frame> RBX::Creatable<RBX::Instance>::create<RBX::Frame>(void)
pub fn stub_520ef4() -> ! {
    todo!("0x520ef4 __ZN3RBX9CreatableINS_8InstanceEE6createINS_5FrameEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationBox> RBX::Creatable<RBX::Instance>::create<RBX::NotificationBox>(void)")]
// 0x520fa8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15NotificationBoxEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::NotificationBox> RBX::Creatable<RBX::Instance>::create<RBX::NotificationBox>(void)
pub fn stub_520fa8() -> ! {
    todo!("0x520fa8 __ZN3RBX9CreatableINS_8InstanceEE6createINS_15NotificationBoxEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiImageButton> RBX::Creatable<RBX::Instance>::create<RBX::GuiImageButton,RBX::Verb *>(RBX::Verb *)")]
// 0x52105c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14GuiImageButtonEPNS_4VerbEEEN5boost10shared_ptrIT_EET0_
// was: boost::shared_ptr<RBX::GuiImageButton> RBX::Creatable<RBX::Instance>::create<RBX::GuiImageButton,RBX::Verb *>(RBX::Verb *)
pub fn stub_52105c() -> ! {
    todo!("0x52105c __ZN3RBX9CreatableINS_8InstanceEE6createINS_14GuiImageButtonEPNS_4VerbEEEN5boost10shared_ptrIT_EET0_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RelativePanel> RBX::Creatable<RBX::Instance>::create<RBX::RelativePanel,RBX::Layout>(RBX::Layout)")]
// 0x521138 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13RelativePanelENS_6LayoutEEEN5boost10shared_ptrIT_EET0_
// was: boost::shared_ptr<RBX::RelativePanel> RBX::Creatable<RBX::Instance>::create<RBX::RelativePanel,RBX::Layout>(RBX::Layout)
pub fn stub_521138() -> ! {
    todo!("0x521138 __ZN3RBX9CreatableINS_8InstanceEE6createINS_13RelativePanelENS_6LayoutEEEN5boost10shared_ptrIT_EET0_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatOutput> RBX::Creatable<RBX::Instance>::create<RBX::ChatOutput>(void)")]
// 0x5211ec — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatOutputEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ChatOutput> RBX::Creatable<RBX::Instance>::create<RBX::ChatOutput>(void)
pub fn stub_5211ec() -> ! {
    todo!("0x5211ec __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatOutputEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatWidget> RBX::Creatable<RBX::Instance>::create<RBX::ChatWidget,std::string,std::string>(std::string,std::string)")]
// 0x5212a0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatWidgetESsSsEEN5boost10shared_ptrIT_EET0_T1_
// was: boost::shared_ptr<RBX::ChatWidget> RBX::Creatable<RBX::Instance>::create<RBX::ChatWidget,std::string,std::string>(std::string,std::string)
pub fn stub_5212a0() -> ! {
    todo!("0x5212a0 __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatWidgetESsSsEEN5boost10shared_ptrIT_EET0_T1_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatButton> RBX::Creatable<RBX::Instance>::create<RBX::ChatButton,RBX::Adorn *,char const*,int>(RBX::Adorn *,char const*,int)")]
// 0x5213fc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatButtonEPNS_5AdornEPKciEEN5boost10shared_ptrIT_EET0_T1_T2_
// was: boost::shared_ptr<RBX::ChatButton> RBX::Creatable<RBX::Instance>::create<RBX::ChatButton,RBX::Adorn *,char const*,int>(RBX::Adorn *,char const*,int)
pub fn stub_5213fc() -> ! {
    todo!("0x5213fc __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatButtonEPNS_5AdornEPKciEEN5boost10shared_ptrIT_EET0_T1_T2_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay> RBX::Creatable<RBX::Instance>::create<RBX::TextDisplay,char const*,char const*>(char const*,char const*)")]
// 0x521594 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11TextDisplayEPKcS6_EEN5boost10shared_ptrIT_EET0_T1_
// was: boost::shared_ptr<RBX::TextDisplay> RBX::Creatable<RBX::Instance>::create<RBX::TextDisplay,char const*,char const*>(char const*,char const*)
pub fn stub_521594() -> ! {
    todo!("0x521594 __ZN3RBX9CreatableINS_8InstanceEE6createINS_11TextDisplayEPKcS6_EEN5boost10shared_ptrIT_EET0_T1_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::EquationDisplay> RBX::Creatable<RBX::Instance>::create<RBX::EquationDisplay,char const*,char const*>(char const*,char const*)")]
// 0x52177c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15EquationDisplayEPKcS6_EEN5boost10shared_ptrIT_EET0_T1_
// was: boost::shared_ptr<RBX::EquationDisplay> RBX::Creatable<RBX::Instance>::create<RBX::EquationDisplay,char const*,char const*>(char const*,char const*)
pub fn stub_52177c() -> ! {
    todo!("0x52177c __ZN3RBX9CreatableINS_8InstanceEE6createINS_15EquationDisplayEPKcS6_EEN5boost10shared_ptrIT_EET0_T1_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay> RBX::shared_from_dynamic_cast<RBX::TextDisplay,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")]
// 0x521964 — __ZN3RBX24shared_from_dynamic_castINS_11TextDisplayENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS4_23enable_shared_from_thisIT0_EE
// was: boost::shared_ptr<RBX::TextDisplay> RBX::shared_from_dynamic_cast<RBX::TextDisplay,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)
pub fn stub_521964() -> ! {
    todo!("0x521964 __ZN3RBX24shared_from_dynamic_castINS_11TextDisplayENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS4_23enable_shared_from_thisIT0_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay> boost::dynamic_pointer_cast<RBX::TextDisplay,RBX::Reflection::DescribedBase>(rbx_core::SharedPtr<RBX::Reflection::DescribedBase> const&)")]
// 0x521af0 — __ZN5boost20dynamic_pointer_castIN3RBX11TextDisplayENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE
// was: boost::shared_ptr<RBX::TextDisplay> boost::dynamic_pointer_cast<RBX::TextDisplay,RBX::Reflection::DescribedBase>(boost::shared_ptr<RBX::Reflection::DescribedBase> const&)
pub fn stub_521af0() -> ! {
    todo!("0x521af0 __ZN5boost20dynamic_pointer_castIN3RBX11TextDisplayENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::EquationDisplay>::shared_ptr<RBX::EquationDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x521b38 — __ZN5boost10shared_ptrIN3RBX15EquationDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::EquationDisplay>::shared_ptr<RBX::EquationDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_521b38() -> ! {
    todo!("0x521b38 __ZN5boost10shared_ptrIN3RBX15EquationDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::EquationDisplay,RBX::EquationDisplay>(rbx_core::SharedPtr<RBX::EquationDisplay> const*,RBX::EquationDisplay *)const")]
// 0x521c00 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15EquationDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::EquationDisplay,RBX::EquationDisplay>(boost::shared_ptr<RBX::EquationDisplay> const*,RBX::EquationDisplay *)const
pub fn stub_521c00() -> ! {
    todo!("0x521c00 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15EquationDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x521ce8 — __ZN5boost6detail12shared_countC2IPN3RBX15EquationDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_521ce8() -> ! {
    todo!("0x521ce8 __ZN5boost6detail12shared_countC2IPN3RBX15EquationDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x521df0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_521df0() -> ! {
    todo!("0x521df0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x521df4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_521df4() -> ! {
    todo!("0x521df4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x521df8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_521df8() -> ! {
    todo!("0x521df8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x521e18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_521e18() -> ! {
    todo!("0x521e18 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x521e30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_521e30() -> ! {
    todo!("0x521e30 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay>::shared_ptr<RBX::TextDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x521e34 — __ZN5boost10shared_ptrIN3RBX11TextDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::TextDisplay>::shared_ptr<RBX::TextDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_521e34() -> ! {
    todo!("0x521e34 __ZN5boost10shared_ptrIN3RBX11TextDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextDisplay,RBX::TextDisplay>(rbx_core::SharedPtr<RBX::TextDisplay> const*,RBX::TextDisplay *)const")]
// 0x521efc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11TextDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextDisplay,RBX::TextDisplay>(boost::shared_ptr<RBX::TextDisplay> const*,RBX::TextDisplay *)const
pub fn stub_521efc() -> ! {
    todo!("0x521efc __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11TextDisplayES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x521fe4 — __ZN5boost6detail12shared_countC2IPN3RBX11TextDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_521fe4() -> ! {
    todo!("0x521fe4 __ZN5boost6detail12shared_countC2IPN3RBX11TextDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x5220ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5220ec() -> ! {
    todo!("0x5220ec __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x5220f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5220f0() -> ! {
    todo!("0x5220f0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x5220f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5220f4() -> ! {
    todo!("0x5220f4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x522114 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_522114() -> ! {
    todo!("0x522114 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x52212c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_52212c() -> ! {
    todo!("0x52212c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatButton>::shared_ptr<RBX::ChatButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x522ff0 — __ZN5boost10shared_ptrIN3RBX10ChatButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ChatButton>::shared_ptr<RBX::ChatButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_522ff0() -> ! {
    todo!("0x522ff0 __ZN5boost10shared_ptrIN3RBX10ChatButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatButton,RBX::ChatButton>(rbx_core::SharedPtr<RBX::ChatButton> const*,RBX::ChatButton *)const")]
// 0x5230b8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatButton,RBX::ChatButton>(boost::shared_ptr<RBX::ChatButton> const*,RBX::ChatButton *)const
pub fn stub_5230b8() -> ! {
    todo!("0x5230b8 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x5231a0 — __ZN5boost6detail12shared_countC2IPN3RBX10ChatButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_5231a0() -> ! {
    todo!("0x5231a0 __ZN5boost6detail12shared_countC2IPN3RBX10ChatButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x5232a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5232a8() -> ! {
    todo!("0x5232a8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x5232ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5232ac() -> ! {
    todo!("0x5232ac __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x5232b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5232b0() -> ! {
    todo!("0x5232b0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x5232d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5232d0() -> ! {
    todo!("0x5232d0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x5232e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5232e8() -> ! {
    todo!("0x5232e8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatWidget>::shared_ptr<RBX::ChatWidget,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x5232ec — __ZN5boost10shared_ptrIN3RBX10ChatWidgetEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ChatWidget>::shared_ptr<RBX::ChatWidget,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5232ec() -> ! {
    todo!("0x5232ec __ZN5boost10shared_ptrIN3RBX10ChatWidgetEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatWidget,RBX::ChatWidget>(rbx_core::SharedPtr<RBX::ChatWidget> const*,RBX::ChatWidget *)const")]
// 0x5233b4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatWidgetES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatWidget,RBX::ChatWidget>(boost::shared_ptr<RBX::ChatWidget> const*,RBX::ChatWidget *)const
pub fn stub_5233b4() -> ! {
    todo!("0x5233b4 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatWidgetES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x52349c — __ZN5boost6detail12shared_countC2IPN3RBX10ChatWidgetENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_52349c() -> ! {
    todo!("0x52349c __ZN5boost6detail12shared_countC2IPN3RBX10ChatWidgetENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x5235a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5235a4() -> ! {
    todo!("0x5235a4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x5235a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5235a8() -> ! {
    todo!("0x5235a8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x5235ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5235ac() -> ! {
    todo!("0x5235ac __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x5235cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5235cc() -> ! {
    todo!("0x5235cc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x5235e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5235e4() -> ! {
    todo!("0x5235e4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatOutput>::shared_ptr<RBX::ChatOutput,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x5235e8 — __ZN5boost10shared_ptrIN3RBX10ChatOutputEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ChatOutput>::shared_ptr<RBX::ChatOutput,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5235e8() -> ! {
    todo!("0x5235e8 __ZN5boost10shared_ptrIN3RBX10ChatOutputEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatOutput,RBX::ChatOutput>(rbx_core::SharedPtr<RBX::ChatOutput> const*,RBX::ChatOutput *)const")]
// 0x5236b0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatOutputES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatOutput,RBX::ChatOutput>(boost::shared_ptr<RBX::ChatOutput> const*,RBX::ChatOutput *)const
pub fn stub_5236b0() -> ! {
    todo!("0x5236b0 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ChatOutputES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x523798 — __ZN5boost6detail12shared_countC2IPN3RBX10ChatOutputENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_523798() -> ! {
    todo!("0x523798 __ZN5boost6detail12shared_countC2IPN3RBX10ChatOutputENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x5238a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5238a0() -> ! {
    todo!("0x5238a0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}
