// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x44b344..0x450f8c | script 25153->25253 distinct (filler 0x44b344 asc, not-in-script 60493->60393 )
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x44b344() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x44b364() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x44b37c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TeleportService> RBX::Creatable<RBX::Instance>::create<RBX::TeleportService>(void)")]
pub fn stub_0x44b594() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TeleportService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::TeleportService>(rbx_core::SharedPtr<RBX::TeleportService> const&)")]
pub fn stub_0x44b644(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TeleportService>::shared_ptr<RBX::TeleportService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44b880() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TeleportService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TeleportService,RBX::TeleportService>(rbx_core::SharedPtr<RBX::TeleportService> const*,RBX::TeleportService *)const")]
pub fn stub_0x44b948() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TeleportService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44ba34() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44bb3c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x44bb40]")]
pub fn stub_0x44bb40(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x44bb44() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x44bb64() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x44bb7c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CookiesService> RBX::Creatable<RBX::Instance>::create<RBX::CookiesService>(void)")]
pub fn stub_0x44bd94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CookiesService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::CookiesService>(rbx_core::SharedPtr<RBX::CookiesService> const&)")]
pub fn stub_0x44be44(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CookiesService>::shared_ptr<RBX::CookiesService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44c080() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CookiesService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CookiesService,RBX::CookiesService>(rbx_core::SharedPtr<RBX::CookiesService> const*,RBX::CookiesService *)const")]
pub fn stub_0x44c148() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CookiesService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44c234() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44c33c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x44c340]")]
pub fn stub_0x44c340(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x44c344() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x44c364() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x44c37c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DebrisService> RBX::Creatable<RBX::Instance>::create<RBX::DebrisService>(void)")]
pub fn stub_0x44c864() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DebrisService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::DebrisService>(rbx_core::SharedPtr<RBX::DebrisService> const&)")]
pub fn stub_0x44c914(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DebrisService>::shared_ptr<RBX::DebrisService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44cb50() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DebrisService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DebrisService,RBX::DebrisService>(rbx_core::SharedPtr<RBX::DebrisService> const*,RBX::DebrisService *)const")]
pub fn stub_0x44cc18() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DebrisService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44cd04() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44ce0c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x44ce10]")]
pub fn stub_0x44ce10(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x44ce14() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x44ce34() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x44ce4c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GamePassService> RBX::Creatable<RBX::Instance>::create<RBX::GamePassService>(void)")]
pub fn stub_0x44d064() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GamePassService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::GamePassService>(rbx_core::SharedPtr<RBX::GamePassService> const&)")]
pub fn stub_0x44d114(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GamePassService>::shared_ptr<RBX::GamePassService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44d350() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GamePassService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GamePassService,RBX::GamePassService>(rbx_core::SharedPtr<RBX::GamePassService> const*,RBX::GamePassService *)const")]
pub fn stub_0x44d418() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GamePassService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44d504() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44d60c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x44d610]")]
pub fn stub_0x44d610(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x44d614() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x44d634() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x44d64c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::SocialService> RBX::Creatable<RBX::Instance>::create<RBX::SocialService>(void)")]
pub fn stub_0x44d864() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::SocialService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::SocialService>(rbx_core::SharedPtr<RBX::SocialService> const&)")]
pub fn stub_0x44d914(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::SocialService>::shared_ptr<RBX::SocialService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44db50() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::SocialService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SocialService,RBX::SocialService>(rbx_core::SharedPtr<RBX::SocialService> const*,RBX::SocialService *)const")]
pub fn stub_0x44dc18() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::SocialService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44dd04() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44de0c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x44de10]")]
pub fn stub_0x44de10(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x44de14() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x44de34() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x44de4c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::InsertService> RBX::Creatable<RBX::Instance>::create<RBX::InsertService>(void)")]
pub fn stub_0x44def4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::InsertService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::InsertService>::shared_ptr<RBX::InsertService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44dfa8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::InsertService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44e074() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::InsertService>(rbx_core::SharedPtr<RBX::InsertService> const&)")]
pub fn stub_0x44e180(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RenderHooksService> RBX::Creatable<RBX::Instance>::create<RBX::RenderHooksService>(void)")]
pub fn stub_0x44e308() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RenderHooksService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::RenderHooksService>(rbx_core::SharedPtr<RBX::RenderHooksService> const&)")]
pub fn stub_0x44e3b8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RenderHooksService>::shared_ptr<RBX::RenderHooksService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44e5f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RenderHooksService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RenderHooksService,RBX::RenderHooksService>(rbx_core::SharedPtr<RBX::RenderHooksService> const*,RBX::RenderHooksService *)const")]
pub fn stub_0x44e6bc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RenderHooksService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44e7a8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44e8b0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x44e8b4]")]
pub fn stub_0x44e8b4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x44e8b8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x44e8d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x44e8f0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FriendService> RBX::Creatable<RBX::Instance>::create<RBX::FriendService>(void)")]
pub fn stub_0x44e968() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FriendService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FriendService>::shared_ptr<RBX::FriendService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44ea18() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FriendService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44eae4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::FriendService>(rbx_core::SharedPtr<RBX::FriendService> const&)")]
pub fn stub_0x44ed20(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryService> RBX::Creatable<RBX::Instance>::create<RBX::GeometryService>(void)")]
pub fn stub_0x44f018() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GeometryService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::GeometryService>(rbx_core::SharedPtr<RBX::GeometryService> const&)")]
pub fn stub_0x44f0c8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryService>::shared_ptr<RBX::GeometryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44f224() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GeometryService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GeometryService,RBX::GeometryService>(rbx_core::SharedPtr<RBX::GeometryService> const*,RBX::GeometryService *)const")]
pub fn stub_0x44f2ec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GeometryService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44f3d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44f4e0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x44f4e4]")]
pub fn stub_0x44f4e4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x44f4e8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x44f508() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x44f520() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BadgeService> RBX::Creatable<RBX::Instance>::create<RBX::BadgeService>(void)")]
pub fn stub_0x44f638() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BadgeService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BadgeService>::shared_ptr<RBX::BadgeService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44f6ec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BadgeService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44f7b8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x44f8c0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x44f8c8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x44f8e8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x44f900() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::BadgeService>(rbx_core::SharedPtr<RBX::BadgeService> const&)")]
pub fn stub_0x44fdcc(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsService> RBX::Creatable<RBX::Instance>::create<RBX::PhysicsService>(void)")]
pub fn stub_0x45012c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PhysicsService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::PhysicsService>(rbx_core::SharedPtr<RBX::PhysicsService> const&)")]
pub fn stub_0x4501dc(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::erase(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator)")]
pub fn stub_0x45079c(handle: &crate::slot::InstanceHandle) {
// RBX::Intrusive::Set<RBX::PartInstance, RBX::PhysicsService>::erase(RBX::Intrusive::Set<RBX~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator::operator++(void)")]
pub fn stub_0x4507d0(handle: &crate::slot::InstanceHandle) {
// RBX::Intrusive::Set<RBX::PartInstance, RBX::PhysicsService>::Iterator::operator++() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Hook::remove(void)")]
pub fn stub_0x450988(handle: &crate::slot::InstanceHandle) {
// RBX::Intrusive::Set<RBX::PartInstance, RBX::PhysicsService>::Hook::remove() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator::Iterator(RBX::PartInstance*)")]
pub fn stub_0x450b14() -> crate::slot::InstanceHandle {
// RBX::Intrusive::Set ctor.
crate::slot::InstanceHandle::new("RBX::Intrusive::Set")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x450dc0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::PhysicsService, RBX::sPhysicsService, RBX::NonFactoryProdu~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x450ee0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x450ee4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x450f84(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x450f8c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}
