//! core-A: 150 boost stubs — filtered boost:: namespace.
//! First half addresses < 0x30000 are already fully covered (84 funcs, 84/84 in boost_skeletons.rs).
//! This batch continues the next 150 boost stubs in EA order (0x452f48..0x461d6c) so `cargo check` stays green.
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost".
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; signatures use `rbx_core::SharedPtr` not `boost::`.
use crate::SharedPtr;

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService> RBX::Creatable<RBX::Instance>::create<RBX::RunService>(void)")]
// 0x452f48 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10RunServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::RunService> RBX::Creatable<RBX::Instance>::create<RBX::RunService>(void)
pub fn stub_452f48<T>(make: impl FnOnce() -> Box<T>) -> SharedPtr<T> {
    // IDA 0x452f48: v1 = new(0xAC); RunService::RunService(v1); shared_ptr<RunService, Deleter>(out, v1).
    crate::shared_ptr::shared_ptr_from_raw(make())
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::RunService>(rbx_core::SharedPtr<RBX::RunService> const&)")]
// 0x452ff8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10RunServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::RunService>(boost::shared_ptr<RBX::RunService> const&)
pub fn stub_452ff8<T>(dst: &mut SharedPtr<T>, src: &SharedPtr<T>) {
    // IDA 0x452ff8: pi = src.pi_; tmp.addref(src.pi_); dst.px = pi; old = dst.count; dst.count = tmp; release(old).
    // Clone-then-assign is exactly addref-new/release-old; derived-to-base needs a coerce site in the caller.
    *dst = SharedPtr::clone(src);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CoreGuiService>::shared_ptr<RBX::CoreGuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x453040 — __ZN5boost10shared_ptrIN3RBX14CoreGuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::CoreGuiService>::shared_ptr<RBX::CoreGuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_453040<T>(raw: Box<T>) -> SharedPtr<T> {
    // IDA 0x453040: *out = p; shared_count(out+1, p, deleter); if (p) _internal_accept_owner(p+40, out, p).
    // Arc adoption covers the block; the weak owner is wired on first downgrade (cf. stub_0x453108).
    crate::shared_ptr::shared_ptr_from_raw(raw)
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreGuiService,RBX::CoreGuiService>(rbx_core::SharedPtr<RBX::CoreGuiService> const*,RBX::CoreGuiService *)const")]
// 0x453108 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CoreGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreGuiService,RBX::CoreGuiService>(boost::shared_ptr<RBX::CoreGuiService> const*,RBX::CoreGuiService *)const
pub fn stub_453108<T>(owner: &mut crate::WeakPtr<T>, shared: &SharedPtr<T>) {
    // IDA 0x453108: if (!weak.use_count()) { if (px) px += 36; weak = shared; } — adopt the owner once.
    if owner.upgrade().is_none() {
        *owner = SharedPtr::downgrade(shared);
    }
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4531f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CoreGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4531f8<T, D>(block: Box<crate::shared_ptr::ControlBlockPd<T, D>>) {
    // IDA 0x4531f8 [thunk]: operator delete(a1) — frees the 0x14 control block after D1 ran.
    drop(block);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterGuiService> RBX::Creatable<RBX::Instance>::create<RBX::StarterGuiService>(void)")]
// 0x453374 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17StarterGuiServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::StarterGuiService> RBX::Creatable<RBX::Instance>::create<RBX::StarterGuiService>(void)
pub fn stub_453374<T>(make: impl FnOnce() -> Box<T>) -> SharedPtr<T> {
    // IDA 0x453374: v1 = new(0x94); StarterGuiService::StarterGuiService(v1); shared_ptr<StarterGuiService, Deleter>(out, v1).
    crate::shared_ptr::shared_ptr_from_raw(make())
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::StarterGuiService>(rbx_core::SharedPtr<RBX::StarterGuiService> const&)")]
// 0x453424 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17StarterGuiServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::StarterGuiService>(boost::shared_ptr<RBX::StarterGuiService> const&)
pub fn stub_453424<T>(dst: &mut SharedPtr<T>, src: &SharedPtr<T>) {
    // IDA 0x453424: pi = src.pi_; tmp.addref(src.pi_); dst.px = pi; old = dst.count; dst.count = tmp; release(old).
    // Clone-then-assign is exactly addref-new/release-old; derived-to-base needs a coerce site in the caller.
    *dst = SharedPtr::clone(src);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterGuiService>::shared_ptr<RBX::StarterGuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x453660 — __ZN5boost10shared_ptrIN3RBX17StarterGuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::StarterGuiService>::shared_ptr<RBX::StarterGuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_453660<T>(raw: Box<T>) -> SharedPtr<T> {
    // IDA 0x453660: *out = p; shared_count(out+1, p, deleter); if (p) _internal_accept_owner(p+40, out, p).
    // Arc adoption covers the block; the weak owner is wired on first downgrade (cf. stub_0x453728).
    crate::shared_ptr::shared_ptr_from_raw(raw)
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterGuiService,RBX::StarterGuiService>(rbx_core::SharedPtr<RBX::StarterGuiService> const*,RBX::StarterGuiService *)const")]
// 0x453728 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17StarterGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterGuiService,RBX::StarterGuiService>(boost::shared_ptr<RBX::StarterGuiService> const*,RBX::StarterGuiService *)const
pub fn stub_453728<T>(owner: &mut crate::WeakPtr<T>, shared: &SharedPtr<T>) {
    // IDA 0x453728: if (!weak.use_count()) { if (px) px += 36; weak = shared; } — adopt the owner once.
    if owner.upgrade().is_none() {
        *owner = SharedPtr::downgrade(shared);
    }
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x453814 — __ZN5boost6detail12shared_countC2IPN3RBX17StarterGuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_453814<T>(px: Box<T>) -> crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x453814: *out = 0; b = new(0x14); b.use = 1; b.weak = 1; b.vtable = off_1247B78; b.px = p; *out = b.
    crate::shared_ptr::ControlBlockPd::new(px, crate::shared_ptr::CreatableInstanceDeleter)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x453920 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17StarterGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_453920<T, D>(block: Box<crate::shared_ptr::ControlBlockPd<T, D>>) {
    // IDA 0x453920 [thunk]: operator delete(a1) — frees the 0x14 control block after D1 ran.
    drop(block);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x453924 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17StarterGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_453924<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
    type_name: &str,
) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x453924: r = this+16; if (ti.name != "N3RBX9CreatableINS_8InstanceEE7DeleterE") return 0; return r.
    block.get_deleter(type_name)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x45393c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17StarterGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_45393c<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
) -> crate::shared_ptr::CreatableInstanceDeleter {
    // IDA 0x45393c: return this+16 — unconditionally the stored deleter.
    block.get_untyped_deleter()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterPackService>::shared_ptr<RBX::StarterPackService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x453d60 — __ZN5boost10shared_ptrIN3RBX18StarterPackServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::StarterPackService>::shared_ptr<RBX::StarterPackService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_453d60<T>(raw: Box<T>) -> SharedPtr<T> {
    // IDA 0x453d60: *out = p; shared_count(out+1, p, deleter); if (p) _internal_accept_owner(p+40, out, p).
    // Arc adoption covers the block; the weak owner is wired on first downgrade (cf. stub_0x453e28).
    crate::shared_ptr::shared_ptr_from_raw(raw)
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterPackService,RBX::StarterPackService>(rbx_core::SharedPtr<RBX::StarterPackService> const*,RBX::StarterPackService *)const")]
// 0x453e28 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18StarterPackServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterPackService,RBX::StarterPackService>(boost::shared_ptr<RBX::StarterPackService> const*,RBX::StarterPackService *)const
pub fn stub_453e28<T>(owner: &mut crate::WeakPtr<T>, shared: &SharedPtr<T>) {
    // IDA 0x453e28: if (!weak.use_count()) { if (px) px += 36; weak = shared; } — adopt the owner once.
    if owner.upgrade().is_none() {
        *owner = SharedPtr::downgrade(shared);
    }
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x453f18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18StarterPackServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_453f18<T, D>(block: Box<crate::shared_ptr::ControlBlockPd<T, D>>) {
    // IDA 0x453f18 [thunk]: operator delete(a1) — frees the 0x14 control block after D1 ran.
    drop(block);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PlayerHUD>::shared_ptr<RBX::PlayerHUD,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x453fc0 — __ZN5boost10shared_ptrIN3RBX9PlayerHUDEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::PlayerHUD>::shared_ptr<RBX::PlayerHUD,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_453fc0<T>(raw: Box<T>) -> SharedPtr<T> {
    // IDA 0x453fc0: *out = p; shared_count(out+1, p, deleter); if (p) _internal_accept_owner(p+40, out, p).
    // Arc adoption covers the block; the weak owner is wired on first downgrade (cf. stub_0x454088).
    crate::shared_ptr::shared_ptr_from_raw(raw)
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PlayerHUD,RBX::PlayerHUD>(rbx_core::SharedPtr<RBX::PlayerHUD> const*,RBX::PlayerHUD *)const")]
// 0x454088 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9PlayerHUDES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PlayerHUD,RBX::PlayerHUD>(boost::shared_ptr<RBX::PlayerHUD> const*,RBX::PlayerHUD *)const
pub fn stub_454088<T>(owner: &mut crate::WeakPtr<T>, shared: &SharedPtr<T>) {
    // IDA 0x454088: if (!weak.use_count()) { if (px) px += 36; weak = shared; } — adopt the owner once.
    if owner.upgrade().is_none() {
        *owner = SharedPtr::downgrade(shared);
    }
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x454174 — __ZN5boost6detail12shared_countC2IPN3RBX9PlayerHUDENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_454174<T>(px: Box<T>) -> crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x454174: decompile timed out; identical template to 0x453814 (IDA type `_DWORD*(_DWORD*,int,int,int,void*,int)`):
    // *out = 0; b = new(0x14); b.use = 1; b.weak = 1; b.vtable set; b.px = p; *out = b.
    crate::shared_ptr::ControlBlockPd::new(px, crate::shared_ptr::CreatableInstanceDeleter)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x45427c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_45427c() {
    // IDA 0x45427c: D1 body is empty (`;`) — no member action; D0 (stub_0x454280) frees the block.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x454280 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_454280<T, D>(block: Box<crate::shared_ptr::ControlBlockPd<T, D>>) {
    // IDA 0x454280 [thunk]: operator delete(a1) — frees the 0x14 control block after D1 ran.
    drop(block);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x454284 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_454284<T>(value: Box<T>, predelete: impl FnOnce(&T)) {
    // IDA 0x454284: v2 = *(this+12); predelete(v2); if (v2) v2->release() (virtual +8); D0 then frees the block.
    predelete(&value);
    drop(value);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4542a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4542a4<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
    type_name: &str,
) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x4542a4: r = this+16; if (ti.name != "N3RBX9CreatableINS_8InstanceEE7DeleterE") return 0; return r.
    block.get_deleter(type_name)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4542bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4542bc<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
) -> crate::shared_ptr::CreatableInstanceDeleter {
    // IDA 0x4542bc: return this+16 — unconditionally the stored deleter.
    block.get_untyped_deleter()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalBackpack> RBX::Creatable<RBX::Instance>::create<RBX::LocalBackpack>(void)")]
// 0x454434 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13LocalBackpackEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::LocalBackpack> RBX::Creatable<RBX::Instance>::create<RBX::LocalBackpack>(void)
pub fn stub_454434<T>(make: impl FnOnce() -> Box<T>) -> SharedPtr<T> {
    // IDA 0x454434: v1 = new(0x98); LocalBackpack::LocalBackpack(v1); shared_ptr<LocalBackpack, Deleter>(out, v1).
    crate::shared_ptr::shared_ptr_from_raw(make())
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LocalBackpack>(rbx_core::SharedPtr<RBX::LocalBackpack> const&)")]
// 0x4544e4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13LocalBackpackEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LocalBackpack>(boost::shared_ptr<RBX::LocalBackpack> const&)
pub fn stub_4544e4<T>(dst: &mut SharedPtr<T>, src: &SharedPtr<T>) {
    // IDA 0x4544e4: pi = src.pi_; tmp.addref(src.pi_); dst.px = pi; old = dst.count; dst.count = tmp; release(old).
    // Clone-then-assign is exactly addref-new/release-old; derived-to-base needs a coerce site in the caller.
    *dst = SharedPtr::clone(src);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalBackpack>::shared_ptr<RBX::LocalBackpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x454720 — __ZN5boost10shared_ptrIN3RBX13LocalBackpackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::LocalBackpack>::shared_ptr<RBX::LocalBackpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_454720<T>(raw: Box<T>) -> SharedPtr<T> {
    // IDA 0x454720: *out = p; shared_count(out+1, p, deleter); if (p) _internal_accept_owner(p+40, out, p).
    // Arc adoption covers the block; the weak owner is wired on first downgrade (cf. stub_0x4547e8).
    crate::shared_ptr::shared_ptr_from_raw(raw)
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalBackpack,RBX::LocalBackpack>(rbx_core::SharedPtr<RBX::LocalBackpack> const*,RBX::LocalBackpack *)const")]
// 0x4547e8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13LocalBackpackES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalBackpack,RBX::LocalBackpack>(boost::shared_ptr<RBX::LocalBackpack> const*,RBX::LocalBackpack *)const
pub fn stub_4547e8<T>(owner: &mut crate::WeakPtr<T>, shared: &SharedPtr<T>) {
    // IDA 0x4547e8: if (!weak.use_count()) { if (px) px += 36; weak = shared; } — adopt the owner once.
    if owner.upgrade().is_none() {
        *owner = SharedPtr::downgrade(shared);
    }
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4548d4 — __ZN5boost6detail12shared_countC2IPN3RBX13LocalBackpackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4548d4<T>(px: Box<T>) -> crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x4548d4: *out = 0; b = new(0x14); b.use = 1; b.weak = 1; b.vtable set; b.px = p; *out = b.
    crate::shared_ptr::ControlBlockPd::new(px, crate::shared_ptr::CreatableInstanceDeleter)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4549dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4549dc() {
    // IDA 0x4549dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4549e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4549e0<T, D>(block: Box<crate::shared_ptr::ControlBlockPd<T, D>>) {
    // IDA 0x4549e0 [thunk]: operator delete(a1) — frees the 0x14 control block after D1 ran.
    drop(block);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4549e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4549e4<T>(value: Box<T>, predelete: impl FnOnce(&T)) {
    // IDA 0x4549e4: v2 = *(this+12); predelete(v2); if (v2) v2->release() (virtual +8); D0 then frees the block.
    predelete(&value);
    drop(value);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x454a04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_454a04<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
    type_name: &str,
) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x454a04: r = this+16; if (ti.name != "N3RBX9CreatableINS_8InstanceEE7DeleterE") return 0; return r.
    block.get_deleter(type_name)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x454a1c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_454a1c<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
) -> crate::shared_ptr::CreatableInstanceDeleter {
    // IDA 0x454a1c: return this+16 — unconditionally the stored deleter.
    block.get_untyped_deleter()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MarketplaceService> RBX::Creatable<RBX::Instance>::create<RBX::MarketplaceService>(void)")]
// 0x454ca4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18MarketplaceServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::MarketplaceService> RBX::Creatable<RBX::Instance>::create<RBX::MarketplaceService>(void)
pub fn stub_454ca4<T>(make: impl FnOnce() -> Box<T>) -> SharedPtr<T> {
    // IDA 0x454ca4: v1 = new(0x98); MarketplaceService::MarketplaceService(v1); shared_ptr<MarketplaceService, Deleter>(out, v1).
    crate::shared_ptr::shared_ptr_from_raw(make())
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x454d58 — __ZN5boost6detail12shared_countC2IPN3RBX18MarketplaceServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_454d58<T>(px: Box<T>) -> crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x454d58: *out = 0; b = new(0x14); b.use = 1; b.weak = 1; b.vtable set; b.px = p; *out = b.
    crate::shared_ptr::ControlBlockPd::new(px, crate::shared_ptr::CreatableInstanceDeleter)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x454e60 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_454e60() {
    // IDA 0x454e60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x454e64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_454e64<T, D>(block: Box<crate::shared_ptr::ControlBlockPd<T, D>>) {
    // IDA 0x454e64 [thunk]: operator delete(a1) — frees the 0x14 control block after D1 ran.
    drop(block);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x454e68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_454e68<T>(value: Box<T>, predelete: impl FnOnce(&T)) {
    // IDA 0x454e68: v2 = *(this+12); predelete(v2); if (v2) v2->release() (virtual +8); D0 then frees the block.
    predelete(&value);
    drop(value);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x454e88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_454e88<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
    type_name: &str,
) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x454e88: r = this+16; if (ti.name != "N3RBX9CreatableINS_8InstanceEE7DeleterE") return 0; return r.
    block.get_deleter(type_name)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x454ea0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_454ea0<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
) -> crate::shared_ptr::CreatableInstanceDeleter {
    // IDA 0x454ea0: return this+16 — unconditionally the stored deleter.
    block.get_untyped_deleter()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatService> RBX::Creatable<RBX::Instance>::create<RBX::ChatService>(void)")]
// 0x45562c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11ChatServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ChatService> RBX::Creatable<RBX::Instance>::create<RBX::ChatService>(void)
pub fn stub_45562c<T>(make: impl FnOnce() -> Box<T>) -> SharedPtr<T> {
    // IDA 0x45562c: v1 = new(0x68); ChatService::ChatService(v1); shared_ptr<ChatService, Deleter>(out, v1).
    crate::shared_ptr::shared_ptr_from_raw(make())
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatService>::shared_ptr<RBX::ChatService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4556dc — __ZN5boost10shared_ptrIN3RBX11ChatServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ChatService>::shared_ptr<RBX::ChatService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4556dc<T>(raw: Box<T>) -> SharedPtr<T> {
    // IDA 0x4556dc: *out = p; shared_count(out+1, p, deleter); if (p) _internal_accept_owner(p+40, out, p).
    // Arc adoption covers the block; the weak owner is wired on first downgrade (cf. stub_0x4557a4).
    crate::shared_ptr::shared_ptr_from_raw(raw)
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const*,RBX::ChatService *)const")]
// 0x4557a4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11ChatServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const*,RBX::ChatService *)const
pub fn stub_4557a4<T>(owner: &mut crate::WeakPtr<T>, shared: &SharedPtr<T>) {
    // IDA 0x4557a4: if (!weak.use_count()) { if (px) px += 36; weak = shared; } — adopt the owner once.
    if owner.upgrade().is_none() {
        *owner = SharedPtr::downgrade(shared);
    }
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x455890 — __ZN5boost6detail12shared_countC2IPN3RBX11ChatServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_455890<T>(px: Box<T>) -> crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x455890: *out = 0; b = new(0x14); b.use = 1; b.weak = 1; b.vtable set; b.px = p; *out = b.
    crate::shared_ptr::ControlBlockPd::new(px, crate::shared_ptr::CreatableInstanceDeleter)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x455998 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_455998() {
    // IDA 0x455998: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x45599c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_45599c<T, D>(block: Box<crate::shared_ptr::ControlBlockPd<T, D>>) {
    // IDA 0x45599c [thunk]: operator delete(a1) — frees the 0x14 control block after D1 ran.
    drop(block);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4559a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4559a0<T>(value: Box<T>, predelete: impl FnOnce(&T)) {
    // IDA 0x4559a0: v2 = *(this+12); predelete(v2); if (v2) v2->release() (virtual +8); D0 then frees the block.
    predelete(&value);
    drop(value);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4559c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4559c0<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
    type_name: &str,
) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x4559c0: r = this+16; if (ti.name != "N3RBX9CreatableINS_8InstanceEE7DeleterE") return 0; return r.
    block.get_deleter(type_name)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4559d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4559d8<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
) -> crate::shared_ptr::CreatableInstanceDeleter {
    // IDA 0x4559d8: return this+16 — unconditionally the stored deleter.
    block.get_untyped_deleter()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const&)")]
// 0x455ea4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_11ChatServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const&)
pub fn stub_455ea4<T>(dst: &mut SharedPtr<T>, src: &SharedPtr<T>) {
    // IDA 0x455ea4: pi = src.pi_; tmp.addref(src.pi_); dst.px = pi; old = dst.count; dst.count = tmp; release(old).
    // Clone-then-assign is exactly addref-new/release-old; derived-to-base needs a coerce site in the caller.
    *dst = SharedPtr::clone(src);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void)")]
// 0x456090 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10GuiServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void)
pub fn stub_456090<T>(make: impl FnOnce() -> Box<T>) -> SharedPtr<T> {
    // IDA 0x456090: v1 = new(0xEC); GuiService::GuiService(v1); shared_ptr<GuiService, Deleter>(out, v1).
    crate::shared_ptr::shared_ptr_from_raw(make())
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const&)")]
// 0x456140 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10GuiServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const&)
pub fn stub_456140<T>(dst: &mut SharedPtr<T>, src: &SharedPtr<T>) {
    // IDA 0x456140: pi = src.pi_; tmp.addref(src.pi_); dst.px = pi; old = dst.count; dst.count = tmp; release(old).
    // Clone-then-assign is exactly addref-new/release-old; derived-to-base needs a coerce site in the caller.
    *dst = SharedPtr::clone(src);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x456174 — __ZN5boost10shared_ptrIN3RBX10GuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_456174<T>(raw: Box<T>) -> SharedPtr<T> {
    // IDA 0x456174: *out = p; shared_count(out+1, p, deleter); if (p) _internal_accept_owner(p+40, out, p).
    // Arc adoption covers the block; the weak owner is wired on first downgrade (cf. stub_0x45623c).
    crate::shared_ptr::shared_ptr_from_raw(raw)
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const*,RBX::GuiService *)const")]
// 0x45623c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10GuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const*,RBX::GuiService *)const
pub fn stub_45623c<T>(owner: &mut crate::WeakPtr<T>, shared: &SharedPtr<T>) {
    // IDA 0x45623c: if (!weak.use_count()) { if (px) px += 36; weak = shared; } — adopt the owner once.
    if owner.upgrade().is_none() {
        *owner = SharedPtr::downgrade(shared);
    }
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x456328 — __ZN5boost6detail12shared_countC2IPN3RBX10GuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_456328<T>(px: Box<T>) -> crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x456328: *out = 0; b = new(0x14); b.use = 1; b.weak = 1; b.vtable set; b.px = p; *out = b.
    crate::shared_ptr::ControlBlockPd::new(px, crate::shared_ptr::CreatableInstanceDeleter)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x456430 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_456430() {
    // IDA 0x456430: D1 body is a single `BX LR` (disasm) — member dtors already ran; Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x456434 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_456434<T, D>(block: Box<crate::shared_ptr::ControlBlockPd<T, D>>) {
    // IDA 0x456434 [thunk]: operator delete(a1) — frees the 0x14 control block after D1 ran.
    drop(block);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x456438 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_456438<T>(value: Box<T>, predelete: impl FnOnce(&T)) {
    // IDA 0x456438: v2 = *(this+12); predelete(v2); if (v2) v2->release() (virtual +8); D0 then frees the block.
    predelete(&value);
    drop(value);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x456458 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_456458<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
    type_name: &str,
) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x456458: r = this+16; if (ti.name != "N3RBX9CreatableINS_8InstanceEE7DeleterE") return 0; return r.
    block.get_deleter(type_name)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x456470 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_456470<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
) -> crate::shared_ptr::CreatableInstanceDeleter {
    // IDA 0x456470: return this+16 — unconditionally the stored deleter.
    block.get_untyped_deleter()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void)")]
// 0x4565e8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_24KeyframeSequenceProviderEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void)
pub fn stub_4565e8<T>(make: impl FnOnce() -> Box<T>) -> SharedPtr<T> {
    // IDA 0x4565e8: v1 = new(0xD4); KeyframeSequenceProvider::KeyframeSequenceProvider(v1); shared_ptr<KSP, Deleter>(out, v1).
    crate::shared_ptr::shared_ptr_from_raw(make())
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const&)")]
// 0x456698 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_24KeyframeSequenceProviderEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const&)
pub fn stub_456698<T>(dst: &mut SharedPtr<T>, src: &SharedPtr<T>) {
    // IDA 0x456698: pi = src.pi_; tmp.addref(src.pi_); dst.px = pi; old = dst.count; dst.count = tmp; release(old).
    // Clone-then-assign is exactly addref-new/release-old; derived-to-base needs a coerce site in the caller.
    *dst = SharedPtr::clone(src);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4567f4 — __ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4567f4<T>(raw: Box<T>) -> SharedPtr<T> {
    // IDA 0x4567f4: *out = p; shared_count(out+1, p, deleter); if (p) _internal_accept_owner(p+40, out, p).
    // Arc adoption covers the block; the weak owner is wired on first downgrade (cf. stub_0x4568bc).
    crate::shared_ptr::shared_ptr_from_raw(raw)
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const")]
// 0x4568bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_24KeyframeSequenceProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const
pub fn stub_4568bc<T>(owner: &mut crate::WeakPtr<T>, shared: &SharedPtr<T>) {
    // IDA 0x4568bc: if (!weak.use_count()) { if (px) px += 36; weak = shared; } — adopt the owner once.
    if owner.upgrade().is_none() {
        *owner = SharedPtr::downgrade(shared);
    }
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4569a8 — __ZN5boost6detail12shared_countC2IPN3RBX24KeyframeSequenceProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4569a8<T>(px: Box<T>) -> crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x4569a8: *out = 0; b = new(0x14); b.use = 1; b.weak = 1; b.vtable set; b.px = p; *out = b.
    crate::shared_ptr::ControlBlockPd::new(px, crate::shared_ptr::CreatableInstanceDeleter)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x456ab0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_456ab0() {
    // IDA 0x456ab0: D1 per-family twin of 0x456430 (single-block dtor) — member dtors already ran; Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x456ab4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_456ab4<T, D>(block: Box<crate::shared_ptr::ControlBlockPd<T, D>>) {
    // IDA 0x456ab4 [thunk]: operator delete(a1) — frees the 0x14 control block after D1 ran.
    drop(block);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x456ab8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_456ab8<T>(value: Box<T>, predelete: impl FnOnce(&T)) {
    // IDA 0x456ab8: v2 = *(this+12); predelete(v2); if (v2) v2->release() (virtual +8); D0 then frees the block.
    predelete(&value);
    drop(value);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x456ad8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_456ad8<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
    type_name: &str,
) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x456ad8: r = this+16; if (ti.name != "N3RBX9CreatableINS_8InstanceEE7DeleterE") return 0; return r.
    block.get_deleter(type_name)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x456af0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_456af0<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
) -> crate::shared_ptr::CreatableInstanceDeleter {
    // IDA 0x456af0: return this+16 — unconditionally the stored deleter.
    block.get_untyped_deleter()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void)")]
// 0x456d08 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ContentFilterEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void)
pub fn stub_456d08<T>(make: impl FnOnce() -> Box<T>) -> SharedPtr<T> {
    // IDA 0x456d08: v1 = new(0x9C); ContentFilter::ContentFilter(v1); shared_ptr<ContentFilter, Deleter>(out, v1).
    crate::shared_ptr::shared_ptr_from_raw(make())
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const&)")]
// 0x456db8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ContentFilterEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const&)
pub fn stub_456db8<T>(dst: &mut SharedPtr<T>, src: &SharedPtr<T>) {
    // IDA 0x456db8: pi = src.pi_; tmp.addref(src.pi_); dst.px = pi; old = dst.count; dst.count = tmp; release(old).
    // Clone-then-assign is exactly addref-new/release-old; derived-to-base needs a coerce site in the caller.
    *dst = SharedPtr::clone(src);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x456ff4 — __ZN5boost10shared_ptrIN3RBX13ContentFilterEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_456ff4<T>(raw: Box<T>) -> SharedPtr<T> {
    // IDA 0x456ff4: *out = p; shared_count(out+1, p, deleter); if (p) _internal_accept_owner(p+40, out, p).
    // Arc adoption covers the block; the weak owner is wired on first downgrade (cf. stub_0x4570bc).
    crate::shared_ptr::shared_ptr_from_raw(raw)
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const*,RBX::ContentFilter *)const")]
// 0x4570bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ContentFilterES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const*,RBX::ContentFilter *)const
pub fn stub_4570bc<T>(owner: &mut crate::WeakPtr<T>, shared: &SharedPtr<T>) {
    // IDA 0x4570bc: if (!weak.use_count()) { if (px) px += 36; weak = shared; } — adopt the owner once.
    if owner.upgrade().is_none() {
        *owner = SharedPtr::downgrade(shared);
    }
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4571a8 — __ZN5boost6detail12shared_countC2IPN3RBX13ContentFilterENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4571a8<T>(px: Box<T>) -> crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x4571a8: *out = 0; b = new(0x14); b.use = 1; b.weak = 1; b.vtable set; b.px = p; *out = b.
    crate::shared_ptr::ControlBlockPd::new(px, crate::shared_ptr::CreatableInstanceDeleter)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4572b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4572b0() {
    // IDA 0x4572b0: D1 per-family twin of 0x456430 (single-block dtor) — member dtors already ran; Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4572b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4572b4<T, D>(block: Box<crate::shared_ptr::ControlBlockPd<T, D>>) {
    // IDA 0x4572b4 [thunk]: operator delete(a1) — frees the 0x14 control block after D1 ran.
    drop(block);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4572b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4572b8<T>(value: Box<T>, predelete: impl FnOnce(&T)) {
    // IDA 0x4572b8: v2 = *(this+12); predelete(v2); if (v2) v2->release() (virtual +8); D0 then frees the block.
    predelete(&value);
    drop(value);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4572d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4572d8<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
    type_name: &str,
) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x4572d8: r = this+16; if (ti.name != "N3RBX9CreatableINS_8InstanceEE7DeleterE") return 0; return r.
    block.get_deleter(type_name)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4572f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4572f0<T>(
    block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>,
) -> crate::shared_ptr::CreatableInstanceDeleter {
    // IDA 0x4572f0: return this+16 — unconditionally the stored deleter.
    block.get_untyped_deleter()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)")]
// 0x457398 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14delete_bucketsEv
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)
pub fn stub_457398() {
    // IDA 0x457398: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)")]
// 0x4573e4 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeIjEEE
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)
pub fn stub_4573e4() {
    // IDA 0x4573e4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiRoot>::shared_ptr<RBX::GuiRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x457c98 — __ZN5boost10shared_ptrIN3RBX7GuiRootEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::GuiRoot>::shared_ptr<RBX::GuiRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_457c98() {
    // IDA 0x457c98: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiRoot,RBX::GuiRoot>(rbx_core::SharedPtr<RBX::GuiRoot> const*,RBX::GuiRoot *)const")]
// 0x457d60 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7GuiRootES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiRoot,RBX::GuiRoot>(boost::shared_ptr<RBX::GuiRoot> const*,RBX::GuiRoot *)const
pub fn stub_457d60() {
    // IDA 0x457d60: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x457e4c — __ZN5boost6detail12shared_countC2IPN3RBX7GuiRootENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_457e4c() {
    // IDA 0x457e4c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x457f54 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_457f54() {
    // IDA 0x457f54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x457f58 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_457f58() {
    // IDA 0x457f58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x457f5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_457f5c() {
    // IDA 0x457f5c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x457f7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_457f7c() {
    // IDA 0x457f7c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x457f94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_457f94() {
    // IDA 0x457f94: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Workspace>::shared_ptr<RBX::Workspace,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x457f98 — __ZN5boost10shared_ptrIN3RBX9WorkspaceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Workspace>::shared_ptr<RBX::Workspace,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_457f98() {
    // IDA 0x457f98: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Workspace,RBX::Workspace>(rbx_core::SharedPtr<RBX::Workspace> const*,RBX::Workspace *)const")]
// 0x458060 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9WorkspaceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Workspace,RBX::Workspace>(boost::shared_ptr<RBX::Workspace> const*,RBX::Workspace *)const
pub fn stub_458060() {
    // IDA 0x458060: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x45814c — __ZN5boost6detail12shared_countC2IPN3RBX9WorkspaceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_45814c() {
    // IDA 0x45814c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x458254 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_458254() {
    // IDA 0x458254: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x458258 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_458258() {
    // IDA 0x458258: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x45825c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_45825c() {
    // IDA 0x45825c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x45827c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_45827c() {
    // IDA 0x45827c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x458294 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_458294() {
    // IDA 0x458294: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChangeHistoryService> RBX::Creatable<RBX::Instance>::create<RBX::ChangeHistoryService>(void)")]
// 0x45847c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_20ChangeHistoryServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ChangeHistoryService> RBX::Creatable<RBX::Instance>::create<RBX::ChangeHistoryService>(void)
pub fn stub_45847c() {
    // IDA 0x45847c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChangeHistoryService>::shared_ptr<RBX::ChangeHistoryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x45852c — __ZN5boost10shared_ptrIN3RBX20ChangeHistoryServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ChangeHistoryService>::shared_ptr<RBX::ChangeHistoryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_45852c() {
    // IDA 0x45852c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChangeHistoryService,RBX::ChangeHistoryService>(rbx_core::SharedPtr<RBX::ChangeHistoryService> const*,RBX::ChangeHistoryService *)const")]
// 0x4585f4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ChangeHistoryServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChangeHistoryService,RBX::ChangeHistoryService>(boost::shared_ptr<RBX::ChangeHistoryService> const*,RBX::ChangeHistoryService *)const
pub fn stub_4585f4() {
    // IDA 0x4585f4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4586e0 — __ZN5boost6detail12shared_countC2IPN3RBX20ChangeHistoryServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4586e0() {
    // IDA 0x4586e0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4587e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4587e8() {
    // IDA 0x4587e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4587f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4587f0() {
    // IDA 0x4587f0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x458810 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_458810() {
    // IDA 0x458810: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x458828 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_458828() {
    // IDA 0x458828: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::~vector()")]
// 0x458e44 — __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EED2Ev
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::~vector()
pub fn stub_458e44() {
    // IDA 0x458e44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>::shared_ptr<RBX::DataModel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x458f10 — __ZN5boost10shared_ptrIN3RBX9DataModelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::DataModel>::shared_ptr<RBX::DataModel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_458f10() {
    // IDA 0x458f10: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DataModel,RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const*,RBX::DataModel *)const")]
// 0x458fd8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9DataModelES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DataModel,RBX::DataModel>(boost::shared_ptr<RBX::DataModel> const*,RBX::DataModel *)const
pub fn stub_458fd8() {
    // IDA 0x458fd8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4590c4 — __ZN5boost6detail12shared_countC2IPN3RBX9DataModelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4590c4() {
    // IDA 0x4590c4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4591cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4591cc() {
    // IDA 0x4591cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4591d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4591d0() {
    // IDA 0x4591d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4591d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4591d4() {
    // IDA 0x4591d4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4591f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4591f4() {
    // IDA 0x4591f4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x45920c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_45920c() {
    // IDA 0x45920c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel::GenericJob>::shared_ptr<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)")]
// 0x459210 — __ZN5boost10shared_ptrIN3RBX9DataModel10GenericJobEEC2IS3_EEPT_
// was: boost::shared_ptr<RBX::DataModel::GenericJob>::shared_ptr<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)
pub fn stub_459210() {
    // IDA 0x459210: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::DataModel::GenericJob,RBX::DataModel::GenericJob>(rbx_core::SharedPtr<RBX::DataModel::GenericJob> const*,RBX::DataModel::GenericJob *)const")]
// 0x4592f8 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_9DataModel10GenericJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::DataModel::GenericJob,RBX::DataModel::GenericJob>(boost::shared_ptr<RBX::DataModel::GenericJob> const*,RBX::DataModel::GenericJob *)const
pub fn stub_4592f8() {
    // IDA 0x4592f8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)")]
// 0x4593dc — __ZN5boost6detail12shared_countC2IN3RBX9DataModel10GenericJobEEEPT_
// was: boost::detail::shared_count::shared_count<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)
pub fn stub_4593dc() {
    // IDA 0x4593dc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()")]
// 0x4594d4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()
pub fn stub_4594d4() {
    // IDA 0x4594d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()")]
// 0x4594d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEED0Ev
// was: boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()
pub fn stub_4594d8() {
    // IDA 0x4594d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::dispose(void)")]
// 0x4594dc — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::dispose(void)
pub fn stub_4594dc() {
    // IDA 0x4594dc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::get_deleter(std::type_info const&)")]
// 0x4594ec — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::get_deleter(std::type_info const&)
pub fn stub_4594ec() {
    // IDA 0x4594ec: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::get_untyped_deleter(void)")]
// 0x4594f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::get_untyped_deleter(void)
pub fn stub_4594f0() {
    // IDA 0x4594f0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Visit> RBX::Creatable<RBX::Instance>::create<RBX::Visit>(void)")]
// 0x459b9c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5VisitEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Visit> RBX::Creatable<RBX::Instance>::create<RBX::Visit>(void)
pub fn stub_459b9c() {
    // IDA 0x459b9c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Visit>::shared_ptr<RBX::Visit,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x459c4c — __ZN5boost10shared_ptrIN3RBX5VisitEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Visit>::shared_ptr<RBX::Visit,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_459c4c() {
    // IDA 0x459c4c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Visit,RBX::Visit>(rbx_core::SharedPtr<RBX::Visit> const*,RBX::Visit *)const")]
// 0x459d14 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5VisitES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Visit,RBX::Visit>(boost::shared_ptr<RBX::Visit> const*,RBX::Visit *)const
pub fn stub_459d14() {
    // IDA 0x459d14: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x459e00 — __ZN5boost6detail12shared_countC2IPN3RBX5VisitENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_459e00() {
    // IDA 0x459e00: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x459f08 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_459f08() {
    // IDA 0x459f08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x459f0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_459f0c() {
    // IDA 0x459f0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x459f10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_459f10() {
    // IDA 0x459f10: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x459f30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_459f30() {
    // IDA 0x459f30: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x459f48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_459f48() {
    // IDA 0x459f48: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x45cf14 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<1,RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_45cf14() {
    // IDA 0x45cf14: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// 0x45d108 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
pub fn stub_45d108() {
    // IDA 0x45d108: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x45d228 — __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_45d228() {
    // IDA 0x45d228: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function1<void,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// 0x45d310 — __ZN5boost9function1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
// was: void boost::function1<void,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
pub fn stub_45d310() {
    // IDA 0x45d310: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x45d408 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_45d408() {
    // IDA 0x45d408: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x45d428 — __ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_45d428() {
    // IDA 0x45d428: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::callable<rbx::signals::signal<void ()(bool)>*>(boost::function<void ()(bool)> const&,rbx::signals::signal<void ()(bool)>*)")]
// 0x45d710 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::callable<rbx::signals::signal<void ()(bool)>*>(boost::function<void ()(bool)> const&,rbx::signals::signal<void ()(bool)>*)
pub fn stub_45d710() {
    // IDA 0x45d710: boost template instantiation (mangled-only context). Per Boost map (AGENTS.md section 4) — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")]
// 0x45d810 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)
pub fn stub_45d810() {
    // IDA 0x45d810: boost template instantiation (mangled-only context). Per Boost map (AGENTS.md section 4) — carrier no-op.
}

#[doc(alias = "non_virtual_thunk_to rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")]
// 0x45d818 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)
pub fn stub_45d818() {
    // IDA 0x45d818: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x45dd2c — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_45dd2c() {
    // IDA 0x45dd2c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")]
// 0x45dfb8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()
pub fn stub_45dfb8() {
    // IDA 0x45dfb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")]
// 0x45e0c8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()
pub fn stub_45e0c8() {
    // IDA 0x45e0c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DataModel::GearType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearType> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x45e4ec — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel8GearTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::DataModel::GearType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearType> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_45e4ec() {
    // IDA 0x45e4ec: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearType &,boost::enable_if<boost::is_enum<RBX::DataModel::GearType>,void>::type *)")]
// 0x45e67c — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel8GearTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// was: bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearType &,boost::enable_if<boost::is_enum<RBX::DataModel::GearType>,void>::type *)
pub fn stub_45e67c() {
    // IDA 0x45e67c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::DataModel::GearGenreSetting RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearGenreSetting,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearGenreSetting> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearGenreSetting,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x4618d8 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel16GearGenreSettingELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::DataModel::GearGenreSetting RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearGenreSetting,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearGenreSetting> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearGenreSetting,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_4618d8() {
    // IDA 0x4618d8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearGenreSetting>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearGenreSetting &,boost::enable_if<boost::is_enum<RBX::DataModel::GearGenreSetting>,void>::type *)")]
// 0x461a68 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel16GearGenreSettingEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// was: bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearGenreSetting>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearGenreSetting &,boost::enable_if<boost::is_enum<RBX::DataModel::GearGenreSetting>,void>::type *)
pub fn stub_461a68() {
    // IDA 0x461a68: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::DataModel::Genre RBX::Reflection::ArgHelper::getArg<RBX::DataModel::Genre,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::Genre> const&,boost::disable_if<boost::is_same<RBX::DataModel::Genre,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x461d6c — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel5GenreELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::DataModel::Genre RBX::Reflection::ArgHelper::getArg<RBX::DataModel::Genre,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::Genre> const&,boost::disable_if<boost::is_same<RBX::DataModel::Genre,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_461d6c() {
    // IDA 0x461d6c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[cfg(test)]
mod batch_a_tests {
    use super::*;
    use crate::shared_ptr::{ControlBlockPd, CreatableInstanceDeleter, CREATABLE_INSTANCE_DELETER_TYPE_NAME};

    #[test]
    fn create_assign_accept_owner_round_trip() {
        let made = stub_452f48(|| Box::new(7u32));
        assert_eq!(*made, 7);
        let mut dst = stub_453040(Box::new(1u32));
        stub_452ff8(&mut dst, &made);
        assert_eq!(*dst, 7);
        assert_eq!(SharedPtr::strong_count(&dst), 2);
        let mut owner: crate::WeakPtr<u32> = Default::default();
        stub_453108(&mut owner, &made);
        assert_eq!(*owner.upgrade().unwrap(), 7);
        stub_453108(&mut owner, &dst);
        assert_eq!(*owner.upgrade().unwrap(), 7);
    }

    #[test]
    fn control_block_lifecycle_matches_decompile() {
        let block = stub_453814(Box::new(9u32));
        assert_eq!(block.use_count(), 1);
        assert_eq!(*block.get().unwrap(), 9);
        assert!(stub_453924(&block, CREATABLE_INSTANCE_DELETER_TYPE_NAME).is_some());
        assert!(stub_453924(&block, "bogus").is_none());
        let _ = stub_45393c(&block);
        let seen = std::cell::Cell::new(false);
        stub_454284(Box::new(3u32), |v| {
            assert_eq!(*v, 3);
            seen.set(true);
        });
        assert!(seen.get());
        stub_4531f8(Box::new(ControlBlockPd::new(Box::new(0u8), CreatableInstanceDeleter)));
        stub_45427c();
    }
}
