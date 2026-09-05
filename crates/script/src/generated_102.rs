// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0xf2a9d4..0xf2b194 | 4911->5011 covered, 390 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "rbx_core::WeakPtr<RBX::Script> RBX::weak_from<RBX::Script>(RBX::Script*) [0xf2a9d4]")]
pub fn stub_0xf2a9d4(handle: &crate::slot::InstanceHandle) {
// rbx_core::WeakPtr<RBX::Script> RBX::weak_from<RBX::Script>(RBX::Script*) [0xf2a9d4] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::disconnectAll(void) [0xf2a9f4]")]
pub fn stub_0xf2a9f4(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void ()(lua_State *)>::disconnectAll(void) [0xf2a9f4] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RuntimeScriptService>::shared_ptr<RBX::RuntimeScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2aa34]")]
pub fn stub_0xf2aa34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RuntimeScriptService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script>::shared_ptr<RBX::Script>(rbx_core::WeakPtr<RBX::Script> const&,boost::detail::sp_nothrow_tag) [0xf2aa44]")]
pub fn stub_0xf2aa44() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script>::shared_ptr<RBX::Script,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2aa54]")]
pub fn stub_0xf2aa54() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::RuntimeScriptService>(rbx_core::SharedPtr<RBX::RuntimeScriptService> const&) [0xf2aa64]")]
pub fn stub_0xf2aa64(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>::list6(boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>) [0xf2aa74]")]
pub fn stub_0xf2aa74() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool) &,boost::_bi::list1<RBX::DataModel *&> &,int) [0xf2aa84]")]
pub fn stub_0xf2aa84(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2aa84: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>::list7(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>) [0xf2aa94]")]
pub fn stub_0xf2aa94() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "void boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>::operator()<void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list5<RBX::ScriptInformationProvider::RequestResult&,bool &,bool &,float &,bool &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool) &,boost::_bi::list5<RBX::ScriptInformationProvider::RequestResult&,bool &,bool &,float &,bool &> &,int) [0xf2aaa4]")]
pub fn stub_0xf2aaa4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2aaa4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>) [0xf2aab4]")]
pub fn stub_0xf2aab4() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>) [0xf2aac4]")]
pub fn stub_0xf2aac4() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>) [0xf2aad4]")]
pub fn stub_0xf2aad4() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>> const&) [0xf2aae4]")]
pub fn stub_0xf2aae4() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>) [0xf2aaf4]")]
pub fn stub_0xf2aaf4() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>>::~storage3() [0xf2ab04]")]
pub fn stub_0xf2ab04() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>) [0xf2ab14]")]
pub fn stub_0xf2ab14() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>) [0xf2ab24]")]
pub fn stub_0xf2ab24() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>) [0xf2ab34]")]
pub fn stub_0xf2ab34() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>) [0xf2ab44]")]
pub fn stub_0xf2ab44() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>) [0xf2ab54]")]
pub fn stub_0xf2ab54() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>) [0xf2ab64]")]
pub fn stub_0xf2ab64() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>::storage7(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>) [0xf2ab74]")]
pub fn stub_0xf2ab74() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool>(void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool) [0xf2ab84]")]
pub fn stub_0xf2ab84() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list_av_7<rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool,rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>(void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>) [0xf2ab94]")]
pub fn stub_0xf2ab94() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2aba4]")]
pub fn stub_0xf2aba4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2abb4]")]
pub fn stub_0xf2abb4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2abc4]")]
pub fn stub_0xf2abc4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2abd4]")]
pub fn stub_0xf2abd4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "j___ZN5boost8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_9DataModelEEENS9_INS1_6ScriptEEESsS3_bbbENS7_5list7INS7_5valueISB_EENSH_ISD_EENSH_ISsEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi5EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2abe4() -> crate::slot::PortedFn {
// IDA 0xf2abe4: j___ZN5boost8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_9Da~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2abe4, "j___ZN5boost8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEC2INS_3_bi6bind_tIvPFv~")
}

#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_6ScriptEEESsNS1_25ScriptInformationProvider13RequestResultEbbbENS7_5list6INS7_5valueISB_EENSH_ISsEENSH_ISD_EENSH_IbEESL_SL_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2abf4() -> crate::slot::PortedFn {
// IDA 0xf2abf4: j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_6ScriptEEESsNS1_25ScriptInformationPro~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2abf4, "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_6ScriptEEESsNS1_25~")
}

#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>) [0xf2ac04]")]
pub fn stub_0xf2ac04(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_6ScriptEEESsNS1_25ScriptInformationProvider13RequestResultEbbbENS6_5list6INS6_5valueISA_EENSG_ISsEENSG_ISC_EENSG_IbEESK_SK_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2ac14() -> crate::slot::PortedFn {
// IDA 0xf2ac14: j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_6ScriptEEESsNS1_25ScriptInformationProv~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2ac14, "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_6ScriptEEESsNS1_25S~")
}

#[doc(alias = "void boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>) [0xf2ac24]")]
pub fn stub_0xf2ac24(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_9DataModelEEENS8_INS1_6ScriptEEESsS3_bbbENS6_5list7INS6_5valueISA_EENSG_ISC_EENSG_ISsEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi5EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2ac34() -> crate::slot::PortedFn {
// IDA 0xf2ac34: j___ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_9Dat~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2ac34, "j___ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbEC2INS_3_bi6bind_tIvPFvN~")
}

#[doc(alias = "j___ZNK3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0xf2ac44() -> crate::slot::PortedFn {
// IDA 0xf2ac44: j___ZNK3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7Creator12getClassNameEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2ac44, "j___ZNK3RBX14FactoryProductINS_6ScriptENS_10BaseScriptELZNS_7sScriptEENS_8InstanceEE7Creator12getCla~")
}

#[doc(alias = "RBX::RuntimeScriptService * RBX::ServiceProvider::find<RBX::RuntimeScriptService>(void)const [0xf2ac54]")]
pub fn stub_0xf2ac54() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::RuntimeScriptService"))
}

#[doc(alias = "RBX::RuntimeScriptService * RBX::ServiceProvider::create<RBX::RuntimeScriptService>(void)const [0xf2ac64]")]
pub fn stub_0xf2ac64() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::RuntimeScriptService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RuntimeScriptService,RBX::RuntimeScriptService>(rbx_core::SharedPtr<RBX::RuntimeScriptService> const*,RBX::RuntimeScriptService *)const [0xf2ac74]")]
pub fn stub_0xf2ac74() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RuntimeScriptService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Script,RBX::Script>(rbx_core::SharedPtr<RBX::Script> const*,RBX::Script *)const [0xf2ac84]")]
pub fn stub_0xf2ac84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf2ac94]")]
pub fn stub_0xf2ac94(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const [0xf2aca4]")]
pub fn stub_0xf2aca4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2acb4]")]
pub fn stub_0xf2acb4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf2acc4]")]
pub fn stub_0xf2acc4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>,boost::detail::function::function_buffer &)const [0xf2acd4]")]
pub fn stub_0xf2acd4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,rbx_core::WeakPtr<RBX::Script>,std::string,RBX::ScriptInformationProvider::RequestResult,bool,bool,bool),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<5>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2ace4]")]
pub fn stub_0xf2ace4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "LuaProfiler::StringCache::getCall(char const*,char const*,char const*,int) [0xf2acf4]")]
pub fn stub_0xf2acf4() -> crate::slot::PortedFn {
// IDA 0xf2acf4: LuaProfiler::StringCache::getCall(char const*,char const*,char const*,int) [0xf2acf4].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2acf4, "LuaProfiler::StringCache::getCall(char const*,char const*,char const*,int) [0xf2acf4]")
}

#[doc(alias = "LuaProfiler::StringCache::getBegin(std::string const&) [0xf2ad04]")]
pub fn stub_0xf2ad04() -> crate::slot::PortedFn {
// IDA 0xf2ad04: LuaProfiler::StringCache::getBegin(std::string const&) [0xf2ad04].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2ad04, "LuaProfiler::StringCache::getBegin(std::string const&) [0xf2ad04]")
}

#[doc(alias = "LuaProfiler::getResumePosition(lua_State *,int) [0xf2ad14]")]
pub fn stub_0xf2ad14() -> crate::slot::PortedFn {
// IDA 0xf2ad14: LuaProfiler::getResumePosition(lua_State *,int) [0xf2ad14].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2ad14, "LuaProfiler::getResumePosition(lua_State *,int) [0xf2ad14]")
}

#[doc(alias = "LuaProfiler::hookCall(lua_State *,lua_Debug *) [0xf2ad24]")]
pub fn stub_0xf2ad24() -> crate::slot::PortedFn {
// IDA 0xf2ad24: LuaProfiler::hookCall(lua_State *,lua_Debug *) [0xf2ad24].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2ad24, "LuaProfiler::hookCall(lua_State *,lua_Debug *) [0xf2ad24]")
}

#[doc(alias = "LuaProfiler::LuaProfiler(lua_State *,int) [0xf2ad34]")]
pub fn stub_0xf2ad34() -> crate::slot::PortedFn {
// IDA 0xf2ad34: LuaProfiler::LuaProfiler(lua_State *,int) [0xf2ad34].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2ad34, "LuaProfiler::LuaProfiler(lua_State *,int) [0xf2ad34]")
}

#[doc(alias = "LuaProfiler::~LuaProfiler() [0xf2ad44]")]
pub fn stub_0xf2ad44() -> crate::slot::PortedFn {
// IDA 0xf2ad44: LuaProfiler::~LuaProfiler() [0xf2ad44].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2ad44, "LuaProfiler::~LuaProfiler() [0xf2ad44]")
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::ScriptContext::*)(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::ScriptContext*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::ScriptContext::*)(void),RBX::Reflection::Variant&) [0xf2ad64]")]
pub fn stub_0xf2ad64() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> c~")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ScriptContext::*)(bool),bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::ScriptContext*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ScriptContext::*)(bool),RBX::Reflection::Variant &,bool const&) [0xf2ad74]")]
pub fn stub_0xf2ad74() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::Call2Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),std::string,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0xf2ad84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::Call2Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(std::string,std::string),std::string,std::string,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&) [0xf2ad94]")]
pub fn stub_0xf2ad94(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call2Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(std::string,s~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Call3Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(int,rbx_core::SharedPtr<RBX::Instance>,std::string),int,rbx_core::SharedPtr<RBX::Instance>,std::string,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(int,rbx_core::SharedPtr<RBX::Instance>,std::string),RBX::Reflection::Variant &,int const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&)")]
pub fn stub_0xf2ada4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf2adb4]")]
pub fn stub_0xf2adb4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ScriptContext::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2adc4]")]
pub fn stub_0xf2adc4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::ScriptContext::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2add4]")]
pub fn stub_0xf2add4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> c~")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0xf2ade4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::ScriptContext::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xf2adf4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,std::string),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf2ae04]")]
pub fn stub_0xf2ae04() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,std::string),2>::BoundFuncDesc(void (RBX::ScriptContext::*)(std::string,std::string),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2ae14]")]
pub fn stub_0xf2ae14() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf2ae24]")]
pub fn stub_0xf2ae24() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(bool),1>::BoundFuncDesc(void (RBX::ScriptContext::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2ae34]")]
pub fn stub_0xf2ae34() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(double),1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf2ae44]")]
pub fn stub_0xf2ae44() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(double),1>::BoundFuncDesc(void (RBX::ScriptContext::*)(double),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2ae54]")]
pub fn stub_0xf2ae54() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf2ae64]")]
pub fn stub_0xf2ae64() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int),1>::BoundFuncDesc(void (RBX::ScriptContext::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2ae74]")]
pub fn stub_0xf2ae74() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0xf2ae84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::BoundFuncDesc(void (RBX::ScriptContext::*)(int,rbx_core::SharedPtr<RBX::Instance>,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xf2ae94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(void),0>::BoundFuncDesc(void (RBX::ScriptContext::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2aea4]")]
pub fn stub_0xf2aea4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptContext", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ScriptContext>(char const*,char const*,bool RBX::ScriptContext::*,void (RBX::ScriptContext::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions) [0xf2aef4]")]
pub fn stub_0xf2aef4() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ScriptContext>(char const*,char const*,int RBX::ScriptContext::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions) [0xf2af04]")]
pub fn stub_0xf2af04() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf2af14() -> crate::slot::PortedFn {
// IDA 0xf2af14: j___ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCore~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2af14, "j___ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10Bas~")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf2af24() -> crate::slot::PortedFn {
// IDA 0xf2af24: j___ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sS~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2af24, "j___ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS~")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_9Scripting14ScriptDebuggerELZNS2_15sScriptDebuggerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_15sScriptDebuggerEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EE15classDescriptorEv")]
pub fn stub_0xf2af74() -> crate::slot::PortedFn {
// IDA 0xf2af74: j___ZN3RBX10Reflection9DescribedINS_9Scripting14ScriptDebuggerELZNS2_15sScriptDebuggerEENS_14FactoryProductIS3_NS_8Insta~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2af74, "j___ZN3RBX10Reflection9DescribedINS_9Scripting14ScriptDebuggerELZNS2_15sScriptDebuggerEENS_14Factory~")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xf2af84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::EventDesc(rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2af94]")]
pub fn stub_0xf2af94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BaseScript> RBX::shared_from<RBX::BaseScript>(RBX::BaseScript*) [0xf2afa4]")]
pub fn stub_0xf2afa4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BaseScript")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptContext> RBX::shared_from<RBX::ScriptContext>(RBX::ScriptContext*) [0xf2afb4]")]
pub fn stub_0xf2afb4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "RBX::LuaStatsItem::create(RBX::ScriptContext *) [0xf2afd4]")]
pub fn stub_0xf2afd4(handle: &crate::slot::InstanceHandle) {
// RBX::LuaStatsItem::create(RBX::ScriptContext *) [0xf2afd4] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LuaStatsItem::LuaStatsItem(RBX::ScriptContext *) [0xf2afe4]")]
pub fn stub_0xf2afe4() -> crate::slot::InstanceHandle {
// RBX::LuaStatsItem ctor.
crate::slot::InstanceHandle::new("RBX::LuaStatsItem")
}

#[doc(alias = "RBX::ScriptContext::ScriptStart::ScriptStart(RBX::ScriptContext::ScriptStart const&) [0xf2aff4]")]
pub fn stub_0xf2aff4() -> crate::slot::InstanceHandle {
// RBX::ScriptContext::ScriptStart ctor.
crate::slot::InstanceHandle::new("RBX::ScriptContext::ScriptStart")
}

#[doc(alias = "RBX::ScriptContext::ScriptStart::~ScriptStart() [0xf2b004]")]
pub fn stub_0xf2b004(handle: crate::slot::InstanceHandle) {
// RBX::ScriptContext::ScriptStart dtor.
drop(handle);
}

#[doc(alias = "RBX::ScriptContext::ScriptStart::operator=(RBX::ScriptContext::ScriptStart const&) [0xf2b014]")]
pub fn stub_0xf2b014(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::ScriptStart::operator=(RBX::ScriptContext::ScriptStart const&) [0xf2b0~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0xf2b024() -> crate::slot::PortedFn {
// IDA 0xf2b024: j___ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7Creator~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b024, "j___ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0xf2b034() -> crate::slot::PortedFn {
// IDA 0xf2b034: j___ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7Creator~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b034, "j___ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E15isNullClassNameEv")]
pub fn stub_0xf2b064() -> crate::slot::PortedFn {
// IDA 0xf2b064: j___ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E15isNullClassNameEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b064, "j___ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E15isNullClassN~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E17static_getCreatorEv")]
pub fn stub_0xf2b074() -> crate::slot::PortedFn {
// IDA 0xf2b074: j___ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E17static_getCreatorEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b074, "j___ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E17static_getCr~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorD2Ev")]
pub fn stub_0xf2b084() -> crate::slot::PortedFn {
// IDA 0xf2b084: j___ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorD2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b084, "j___ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorD2Ev")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E17static_getCreatorEv")]
pub fn stub_0xf2b094() -> crate::slot::PortedFn {
// IDA 0xf2b094: j___ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E17static_getCreatorEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b094, "j___ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E17s~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7CreatorC2Ev")]
pub fn stub_0xf2b0a4() -> crate::slot::PortedFn {
// IDA 0xf2b0a4: j___ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7CreatorC2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b0a4, "j___ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7Cr~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E17static_getCreatorEv")]
pub fn stub_0xf2b0b4() -> crate::slot::PortedFn {
// IDA 0xf2b0b4: j___ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E17static_getCreatorEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b0b4, "j___ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E1~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7CreatorC2Ev")]
pub fn stub_0xf2b0c4() -> crate::slot::PortedFn {
// IDA 0xf2b0c4: j___ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7CreatorC2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b0c4, "j___ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7CreatorD2Ev")]
pub fn stub_0xf2b0d4() -> crate::slot::PortedFn {
// IDA 0xf2b0d4: j___ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7CreatorD2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b0d4, "j___ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7~")
}

#[doc(alias = "RBX::ScriptContext * RBX::ServiceProvider::find<RBX::ScriptContext>(RBX::Instance const*) [0xf2b134]")]
pub fn stub_0xf2b134() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::ScriptContext"))
}

#[doc(alias = "RBX::WaitingScriptsJob::WaitingScriptsJob(rbx_core::SharedPtr<RBX::ScriptContext>)")]
pub fn stub_0xf2b184() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "RBX::RuntimeScriptService::~RuntimeScriptService() [0xf2b194]")]
pub fn stub_0xf2b194(handle: crate::slot::InstanceHandle) {
// RBX::RuntimeScriptService dtor.
drop(handle);
}
