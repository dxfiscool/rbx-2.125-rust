// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted, true uncovered after existing shards
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x6dfb70..0xf5f454 | total filtered 10215, remaining 196->96 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias
// Shard: 86 EA-sorted ascending next uncovered gap from 0x6dfb70

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_05::{
    ATTR_CLASS, ATTR_REFERENT, TAG_ITEM, TAG_PROPERTIES, CombinedSignal, CreatorRole, Instance,
    PropertyKind, ReferenceBinder, XmlElement, borrow_shared, instance_is_a, stub_0x703568,
    stub_0x703cc0, stub_0x703dc8, stub_0x703fb0,
};
use std::collections::HashMap;
/// Rust model of `RBX::Security::Context` (IDA `0x6ffb52`,
/// `0x6ffb78`): `current()` snapshots the calling context,
/// `requirePermission` enforces one class-permission word. No permission
/// table is modelled yet, so every requirement is granted; the ancestry
/// walk itself (the observable control flow) is preserved.
pub struct SecurityContext;
impl SecurityContext {
    pub fn current() -> Self {
        SecurityContext
    }
    pub fn require_permission(&self, _permission: u32) -> bool {
        true
    }
}
/// Class permission word behind `*(classDesc + 276)` (IDA `0x6ffb5c`):
/// unmodelled classes default to `0` (granted, see `SecurityContext`).
pub fn class_permission(_class: &str) -> u32 {
    0
}

// 0x6dfb70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIfEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<float> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<float> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_6dfb70() -> ! {
    todo!("0x6dfb70 boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<float> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x6dfb88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIfEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<float> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<float> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_6dfb88() -> ! {
    todo!("0x6dfb88 boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<float> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x6dfc34 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5Stats14TypedStatsItemIdEEN5boost9function0IdEEEENS7_10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::TypedStatsItem<double>> RBX::Creatable<RBX::Instance>::create<RBX::Stats::TypedStatsItem<double>,boost::function0<double>>(boost::function0<double>)")]
// was: boost::shared_ptr<RBX::Stats::TypedStatsItem<double>> RBX::Creatable<RBX::Instance>::create<RBX::Stats::TypedStatsItem<double>,boost::function0<double>>(boost::function0<double>)
pub fn stub_6dfc34() -> ! {
    todo!("0x6dfc34 rbx_core::SharedPtr<RBX::Stats::TypedStatsItem<double>> RBX::Creatable<RBX::Instance>::create<RBX::Stats::TypedStatsItem<double>,boost::function0<double>>(boost::function0<double>)")
}

// 0x6e03d8 — __ZN5boost10shared_ptrIN3RBX5Stats14TypedStatsItemIdEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::TypedStatsItem<double>>::shared_ptr<RBX::Stats::TypedStatsItem<double>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Stats::TypedStatsItem<double>>::shared_ptr<RBX::Stats::TypedStatsItem<double>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_6e03d8() -> ! {
    todo!("0x6e03d8 rbx_core::SharedPtr<RBX::Stats::TypedStatsItem<double>>::shared_ptr<RBX::Stats::TypedStatsItem<double>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x6e058c — __ZN5boost6detail12shared_countC2IPN3RBX5Stats14TypedStatsItemIdEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_6e058c() -> ! {
    todo!("0x6e058c boost::detail::shared_count::shared_count<RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x6e0698 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIdEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_6e0698() -> ! {
    todo!("0x6e0698 boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x6f37e0 — __ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrINS_8InstanceEEEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<RBX::Instance>>(void)")]
// was: RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<boost::shared_ptr<RBX::Instance>>(void)
pub fn stub_6f37e0() -> ! {
    todo!("0x6f37e0 RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<RBX::Instance>>(void)")
}

// 0x6f38c8 — __ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>(void)")]
// was: RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>(void)
pub fn stub_6f38c8() -> ! {
    todo!("0x6f38c8 RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>(void)")
}

// 0x6f55f0 — __ZN3RBX10Reflection7Variant7convertIN5boost10shared_ptrINS_8InstanceEEEEERT_v
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> & RBX::Reflection::Variant::convert<rbx_core::SharedPtr<RBX::Instance>>(void)")]
// was: boost::shared_ptr<RBX::Instance> & RBX::Reflection::Variant::convert<boost::shared_ptr<RBX::Instance>>(void)
pub fn stub_6f55f0() -> ! {
    todo!("0x6f55f0 rbx_core::SharedPtr<RBX::Instance> & RBX::Reflection::Variant::convert<rbx_core::SharedPtr<RBX::Instance>>(void)")
}

// 0x6f592c — __ZN3RBX10Reflection7Variant7convertIN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEEERT_v
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> & RBX::Reflection::Variant::convert<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>(void)")]
// was: boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> & RBX::Reflection::Variant::convert<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>(void)
pub fn stub_6f592c() -> ! {
    todo!("0x6f592c rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> & RBX::Reflection::Variant::convert<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>(void)")
}

// 0x6f5be4 — __ZL12CastInstanceN3RBX10Reflection7VariantEN5boost10shared_ptrISt6vectorINS3_INS_8InstanceEEESaIS6_EEEE
#[doc(alias = "CastInstance(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
// was: CastInstance(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)
pub fn stub_6f5be4() -> ! {
    todo!("0x6f5be4 CastInstance(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")
}

// 0x6f7c34 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS_8InstanceEEEED1Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<RBX::Instance>>::~TType()")]
// was: RBX::Reflection::TType<boost::shared_ptr<RBX::Instance>>::~TType()
pub fn stub_6f7c34() -> ! {
    todo!("0x6f7c34 RBX::Reflection::TType<rbx_core::SharedPtr<RBX::Instance>>::~TType()")
}

// 0x6f7c38 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEEED1Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::~TType()")]
// was: RBX::Reflection::TType<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::~TType()
pub fn stub_6f7c38() -> ! {
    todo!("0x6f7c38 RBX::Reflection::TType<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::~TType()")
}

// 0x6f99d8 — __ZN3rbx8any_castIN5boost10shared_ptrIN3RBX8InstanceEEENS3_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> * rbx::any_cast<rbx_core::SharedPtr<RBX::Instance>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: boost::shared_ptr<RBX::Instance> * rbx::any_cast<boost::shared_ptr<RBX::Instance>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
pub fn stub_6f99d8() -> ! {
    todo!("0x6f99d8 rbx_core::SharedPtr<RBX::Instance> * rbx::any_cast<rbx_core::SharedPtr<RBX::Instance>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x6f9ae0 — __ZN3rbx8any_castIN5boost10shared_ptrIKSt6vectorINS2_IN3RBX8InstanceEEESaIS6_EEEENS4_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> * rbx::any_cast<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> * rbx::any_cast<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
pub fn stub_6f9ae0() -> ! {
    todo!("0x6f9ae0 rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> * rbx::any_cast<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x6f9b90 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN3RBX10Reflection7VariantESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvPFvS4_NSB_10shared_ptrIS7_INSE_INS2_8InstanceEEESaISG_EEEEENSC_5list2INSB_3argILi1EEENSC_5valueISJ_EEEEEEET0_T_SU_ST_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>)")]
// was: boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>)
pub fn stub_6f9b90() -> ! {
    todo!("0x6f9b90 boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>)")
}

// 0x6f9be8 — __ZN5boost4bindIvN3RBX10Reflection7VariantENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEENS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_ENSD_9list_av_2IT2_T3_E4typeEEESJ_SL_SM_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list_av_2<boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::type> boost::bind<void,RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
// was: boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list_av_2<boost::arg<1>,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::type> boost::bind<void,RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::arg<1>,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(void (*)(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::arg<1>,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)
pub fn stub_6f9be8() -> ! {
    todo!("0x6f9be8 boost::_bi::bind_t<void,void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list_av_2<boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::type> boost::bind<void,RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")
}

// 0x6fc9e0 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt6vectorINS3_IN3RBX8InstanceEEESaIS7_EEEEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::construct_func(char const*,char *)
pub fn stub_6fc9e0() -> ! {
    todo!("0x6fc9e0 rbx::implementation::typed_holder<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::construct_func(char const*,char *)")
}

// 0x6fca04 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_10shared_ptrISt6vectorINS5_IN3RBX8InstanceEEESaIS9_EEEEEEEclIPFvNS7_10Reflection7VariantESC_ENS0_5list1IRKSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>::operator()<void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list1<RBX::Reflection::Variant const&>>(boost::_bi::type<void>,void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>) &,boost::_bi::list1<RBX::Reflection::Variant const&> &,int)")]
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>::operator()<void (*)(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list1<RBX::Reflection::Variant const&>>(boost::_bi::type<void>,void (*)(RBX::Reflection::Variant,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>) &,boost::_bi::list1<RBX::Reflection::Variant const&> &,int)
pub fn stub_6fca04() -> ! {
    todo!("0x6fca04 void boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>::operator()<void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list1<RBX::Reflection::Variant const&>>(boost::_bi::type<void>,void (*)(RBX::Reflection::Variant,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>) &,boost::_bi::list1<RBX::Reflection::Variant const&> &,int)")
}

// 0x6fde9c — __ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>(char const*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> *)")]
// was: RBX::Reflection::Type::Type<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>(char const*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> *)
pub fn stub_6fde9c() -> ! {
    todo!("0x6fde9c RBX::Reflection::Type::Type<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>(char const*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> *)")
}

// 0x6fdf48 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEEED0Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::~TType()")]
// was: RBX::Reflection::TType<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::~TType()
pub fn stub_6fdf48() -> ! {
    todo!("0x6fdf48 RBX::Reflection::TType<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::~TType()")
}

// 0x6fdf4c — __ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrINS_8InstanceEEEEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<RBX::Instance>>(char const*,rbx_core::SharedPtr<RBX::Instance> *)")]
// was: RBX::Reflection::Type::Type<boost::shared_ptr<RBX::Instance>>(char const*,boost::shared_ptr<RBX::Instance> *)
pub fn stub_6fdf4c() -> ! {
    todo!("0x6fdf4c RBX::Reflection::Type::Type<rbx_core::SharedPtr<RBX::Instance>>(char const*,rbx_core::SharedPtr<RBX::Instance> *)")
}

// 0x6fdff8 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS_8InstanceEEEED0Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<RBX::Instance>>::~TType()")]
// was: RBX::Reflection::TType<boost::shared_ptr<RBX::Instance>>::~TType()
pub fn stub_6fdff8() -> ! {
    todo!("0x6fdff8 RBX::Reflection::TType<rbx_core::SharedPtr<RBX::Instance>>::~TType()")
}

// 0x6fe8a4 — __ZN3RBX8Instance6removeEv
#[doc(alias = "RBX::Instance::remove(void)")]
// was: RBX::Instance::remove(void)
pub fn stub_6fe8a4(child: &SharedPtr<Instance>) {
    // IDA 0x6fe8a4: `setParentInternal(this, 0, 0)` (disasm 0x6fe906), then
    // `for_each` over the pre-detach child snapshot (disasm 0x6fe92a) with
    // the `mf0` remove binder — i.e. recursive `remove` of every child.
    // The snapshot clone is the pre-detach vector read (disasm 0x6fe8c8).
    stub_6ffc98(SharedPtr::as_ptr(child) as *mut Instance, core::ptr::null(), false);
    let snapshot = child.children.clone();
    for grand in snapshot.iter() {
        stub_6fe8a4(grand);
    }
}

// 0x6fe994 — __ZN3RBX8Instance17removeAllChildrenEv
#[doc(alias = "RBX::Instance::removeAllChildren(void)")]
// was: RBX::Instance::removeAllChildren(void)
pub fn stub_6fe994(this: *mut Instance) {
    // IDA 0x6fe994: loop while the `+56` holder is set (disasm 0x6fe9be):
    // retain one child (`shared_count` copy, disasm 0x6fea04), `remove` it
    // (disasm 0x6fea12), release, repeat. Each `remove` detaches from this
    // vector, so looping until empty is the same iteration.
    // SAFETY: `this` must point to a valid `Instance` outliving the call.
    unsafe {
        while let Some(last) = (*this).children.last().cloned() {
            stub_6fe8a4(&last);
        }
    }
}

// 0x6fea84 — __ZN3RBX8Instance12waitForChildESsN5boost8functionIFvNS1_10shared_ptrIS0_EEEEENS2_IFvSsEEE
#[doc(alias = "RBX::Instance::waitForChild(std::string,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")]
// was: RBX::Instance::waitForChild(std::string,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>)
pub fn stub_6fea84() -> ! {
    todo!("0x6fea84 RBX::Instance::waitForChild(std::string,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")
}

// 0x6fee38 — __ZN3RBX8Instance15setRobloxLockedEb
#[doc(alias = "RBX::Instance::setRobloxLocked(bool)")]
// was: RBX::Instance::setRobloxLocked(bool)
pub fn stub_6fee38(this: *mut Instance, value: bool) -> bool {
    // IDA 0x6fee38: unchanged (`store + 22 == a2`, disasm 0x6fee4a) returns
    // the store word; else `FWValue<bool>::set(store + 22)` (disasm 0x6fee54,
    // collapses to the plain store) and
    // `raisePropertyChanged(propRobloxLocked)` (disasm 0x6fee64). Nonzero
    // returns map to `true`.
    // SAFETY: `this` must point to a valid `Instance`.
    unsafe {
        if (*this).roblox_locked != value {
            (*this).roblox_locked = value;
            if let Some(hook) = (*this).hooks.property_changed {
                hook(this, PropertyKind::RobloxLocked);
            }
        }
        true
    }
}

// 0x6fee6c — __ZN3RBX8Instance11createChildERKNS_4NameENS_11CreatorRoleE
#[doc(alias = "RBX::Instance::createChild(RBX::Name const&,RBX::CreatorRole)")]
// was: RBX::Instance::createChild(RBX::Name const&,RBX::CreatorRole)
pub fn stub_6fee6c(_this: *mut Instance, name: &str, role: CreatorRole) -> Option<SharedPtr<Instance>> {
    // IDA 0x6fee6c: pure forward of `(name, role)` to
    // `AbstractFactoryProduct<Instance>::create` (disasm 0x6fee78); the
    // member `this` (a2) is unused and nothing auto-parents the product.
    let mut out = None;
    stub_0x703568(&mut out as *mut _, name, role);
    out
}

// 0x6fee7c — __ZN3RBX8Instance9readChildEPK10XmlElementRNS_16IReferenceBinderENS_11CreatorRoleE
#[doc(alias = "RBX::Instance::readChild(XmlElement const*,RBX::IReferenceBinder &,RBX::CreatorRole)")]
// was: RBX::Instance::readChild(XmlElement const*,RBX::IReferenceBinder &,RBX::CreatorRole)
pub fn stub_6fee7c(
    parent: *mut Instance,
    elem: &XmlElement,
    binder: &mut ReferenceBinder,
    role: CreatorRole,
) {
    // IDA 0x6fee7c: `class` attribute lookup (disasm 0x6feebc); resolved
    // (`getValue() == 1`, disasm 0x6feefa) drives the `+44` factory virtual
    // (disasm 0x6fef0c — collapses to the creator-table lookup), then
    // `read(child)` (disasm 0x6fef1e) and `setParentInternal(child, parent,
    // 0)` (disasm 0x6fef2e). A failed create falls to the `referent`
    // attribute + binder path (disasm 0x6fef4c-0x6fef64).
    // SAFETY: `parent` must point to a valid `Instance` outliving the call.
    if let Some(class_attr) = elem.find_attribute(ATTR_CLASS) {
        let mut out = None;
        stub_0x703568(&mut out as *mut _, &class_attr.value, role);
        match out {
            Some(child) => {
                let raw = SharedPtr::as_ptr(&child) as *mut Instance;
                stub_6fefd0(raw, elem, binder, role);
                stub_6ffc98(raw, parent as *const Instance, false);
            }
            None => {
                if let Some(referent) = elem.find_attribute(ATTR_REFERENT) {
                    binder.bind(&referent.value, core::ptr::null());
                }
            }
        }
    }
}

// 0x6fefd0 — __ZN3RBX8Instance4readEPK10XmlElementRNS_16IReferenceBinderENS_11CreatorRoleE
#[doc(alias = "RBX::Instance::read(XmlElement const*,RBX::IReferenceBinder &,RBX::CreatorRole)")]
// was: RBX::Instance::read(XmlElement const*,RBX::IReferenceBinder &,RBX::CreatorRole)
pub fn stub_6fefd0(
    this: *mut Instance,
    elem: &XmlElement,
    binder: &mut ReferenceBinder,
    role: CreatorRole,
) {
    // IDA 0x6fefd0: `referent` attribute binds `(name, this + 36)` (disasm
    // 0x6feff0-0x6ff002 — the `+36` member offset collapses to `this`);
    // non-`Item` tags delegate to the `+120` subclass virtual (disasm
    // 0x6ff018-0x6ff02c); `Item` tags run each `Properties` sub-element
    // through the same `+120` slot (disasm 0x6ff03e-0x6ff052) and finish
    // with `readChildren` (disasm 0x6ff052-tail).
    // SAFETY: `this` must point to a valid `Instance` outliving the call.
    unsafe {
        if let Some(referent) = elem.find_attribute(ATTR_REFERENT) {
            binder.bind(&referent.value, this as *const Instance);
        }
        if elem.tag != TAG_ITEM {
            if let Some(hook) = (*this).hooks.read_node {
                hook(this, elem, binder);
            }
            return;
        }
        if let Some(props) = elem.find_first_child_by_tag(TAG_PROPERTIES) {
            for prop in props.children.iter() {
                if let Some(hook) = (*this).hooks.read_node {
                    hook(this, prop, binder);
                }
            }
        }
        stub_6ff070(this, elem, binder, role);
    }
}

// 0x6ff070 — __ZN3RBX8Instance12readChildrenEPK10XmlElementRNS_16IReferenceBinderENS_11CreatorRoleE
#[doc(alias = "RBX::Instance::readChildren(XmlElement const*,RBX::IReferenceBinder &,RBX::CreatorRole)")]
// was: RBX::Instance::readChildren(XmlElement const*,RBX::IReferenceBinder &,RBX::CreatorRole)
pub fn stub_6ff070(
    this: *mut Instance,
    elem: &XmlElement,
    binder: &mut ReferenceBinder,
    role: CreatorRole,
) {
    // IDA 0x6ff070: null element returns at once (disasm 0x6ff080);
    // `findFirstChildByTag(Item)` (disasm 0x6ff092) then
    // `findNextChildWithSameTag` (disasm 0x6ff0a0) until null (disasm
    // 0x6ff0b0), `readChild` per item (disasm 0x6ff0a0-call).
    // SAFETY: `this` must point to a valid `Instance` outliving the call.
    let mut next = elem.find_first_child_by_tag(TAG_ITEM).map(|e| e as *const XmlElement);
    while let Some(current) = next {
        unsafe {
            stub_6fee7c(this, &*current, binder, role);
        }
        next = elem
            .find_next_child_with_same_tag(current, TAG_ITEM)
            .map(|found| found as *const XmlElement);
    }
}

// 0x6ff0b8 — __ZN3RBX8Instance12readPropertyEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Instance::readProperty(XmlElement const*,RBX::IReferenceBinder &)")]
// was: RBX::Instance::readProperty(XmlElement const*,RBX::IReferenceBinder &)
pub fn stub_6ff0b8() -> ! {
    todo!("0x6ff0b8 RBX::Instance::readProperty(XmlElement const*,RBX::IReferenceBinder &)")
}

// 0x6ff290 — __ZN3RBX8Instance14readPropertiesEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Instance::readProperties(XmlElement const*,RBX::IReferenceBinder &)")]
// was: RBX::Instance::readProperties(XmlElement const*,RBX::IReferenceBinder &)
pub fn stub_6ff290() -> ! {
    todo!("0x6ff290 RBX::Instance::readProperties(XmlElement const*,RBX::IReferenceBinder &)")
}

// 0x6ff2b0 — __ZN3RBX8Instance13writeChildrenEP10XmlElementRKN5boost8functionIFbPS0_EEENS_11CreatorRoleENS0_10SaveFilterE
#[doc(alias = "RBX::Instance::writeChildren(XmlElement *,boost::function<bool ()(RBX::Instance*)> const&,RBX::CreatorRole,RBX::Instance::SaveFilter)")]
// was: RBX::Instance::writeChildren(XmlElement *,boost::function<bool ()(RBX::Instance*)> const&,RBX::CreatorRole,RBX::Instance::SaveFilter)
pub fn stub_6ff2b0() -> ! {
    todo!("0x6ff2b0 RBX::Instance::writeChildren(XmlElement *,boost::function<bool ()(RBX::Instance*)> const&,RBX::CreatorRole,RBX::Instance::SaveFilter)")
}

// 0x6ff3e0 — __ZNK3RBX8Instance15writePropertiesEP10XmlElement
#[doc(alias = "RBX::Instance::writeProperties(XmlElement *)const")]
// was: RBX::Instance::writeProperties(XmlElement *)const
pub fn stub_6ff3e0() -> ! {
    todo!("0x6ff3e0 RBX::Instance::writeProperties(XmlElement *)const")
}

// 0x6ff48c — __ZN3RBX8Instance8writeXmlERKN5boost8functionIFbPS0_EEENS_11CreatorRoleE
#[doc(alias = "RBX::Instance::writeXml(boost::function<bool ()(RBX::Instance*)> const&,RBX::CreatorRole)")]
// was: RBX::Instance::writeXml(boost::function<bool ()(RBX::Instance*)> const&,RBX::CreatorRole)
pub fn stub_6ff48c() -> ! {
    todo!("0x6ff48c RBX::Instance::writeXml(boost::function<bool ()(RBX::Instance*)> const&,RBX::CreatorRole)")
}

// 0x6ff77c — __ZNK3RBX8Instance21getPersistentDataCostEv
#[doc(alias = "RBX::Instance::getPersistentDataCost(void)const")]
// was: RBX::Instance::getPersistentDataCost(void)const
pub fn stub_6ff77c(this: &SharedPtr<Instance>) -> i32 {
    // IDA 0x6ff77c: `computeChildCost` binder over the children (disasm
    // 0x6ff7b2-0x6ff81e) accumulating from `4` (disasm 0x6ff7b6); the
    // `shared_count` copy/release pair is a borrow.
    persistent_data_cost(SharedPtr::as_ptr(this))
}

// 0x6ff888 — __ZN3RBXL16computeChildCostEN5boost10shared_ptrINS_8InstanceEEEPi
#[doc(alias = "RBX::computeChildCost(rbx_core::SharedPtr<RBX::Instance>,int *)")]
// was: RBX::computeChildCost(boost::shared_ptr<RBX::Instance>,int *)
pub fn stub_6ff888(child: &SharedPtr<Instance>, acc: &mut i32) -> i32 {
    // IDA 0x6ff888: `*a2 += child->vf32()` (disasm 0x6ff898, slot `+32` =
    // `getPersistentDataCost` virtual) and store back (disasm 0x6ff89a).
    // The virtual collapses into `persistent_data_cost`, which honors the
    // `data_cost` override hook.
    *acc += persistent_data_cost(SharedPtr::as_ptr(child));
    *acc
}

// 0x6ff8a0 — __ZN3RBX8Instance14onChildChangedEPS0_RKNS_15PropertyChangedE
#[doc(alias = "RBX::Instance::onChildChanged(RBX::Instance*,RBX::PropertyChanged const&)")]
// was: RBX::Instance::onChildChanged(RBX::Instance*,RBX::PropertyChanged const&)
pub fn stub_6ff8a0(child: *mut Instance) -> i32 {
    // IDA 0x6ff8a0: `parent = *(a1 + 52)` (disasm 0x6ff8a0); null parent
    // returns `0` (disasm 0x6ff8a6), else the `+112` virtual on the parent
    // (disasm 0x6ff8ac). The `PropertyChanged` arg has no model yet.
    // SAFETY: `child` must be null or point to a valid `Instance`.
    unsafe {
        let parent = if child.is_null() {
            core::ptr::null_mut()
        } else {
            (*child).parent as *mut Instance
        };
        if parent.is_null() {
            0
        } else {
            (*parent).hooks.on_child_changed.map_or(0, |hook| hook(parent))
        }
    }
}

// 0x6ff8b0 — __ZNK3RBX8Instance14findChildIndexEPKS0_
#[doc(alias = "RBX::Instance::findChildIndex(RBX::Instance const*)const")]
// was: RBX::Instance::findChildIndex(RBX::Instance const*)const
pub fn stub_6ff8b0(owner: &SharedPtr<Instance>, child: *const Instance) -> usize {
    // IDA 0x6ff8b0: `ReleaseAssert` on the `+56` holder (Instance.cpp,
    // disasm 0x6ff8ea), weak-lock of the owner (`bad_weak_ptr` throw,
    // disasm 0x6ffa10-0x6ffa4a — unreachable for a live borrow), then
    // `std::find` over the children by `shared_ptr` identity (disasm
    // 0x6ff9d6) returning `(found - begin) >> 3` (disasm 0x6ffa02) — the
    // child count on miss, no `-1` sentinel.
    let children = &owner.children;
    debug_assert!(!children.is_empty(), "0x6ff8b0: children holder");
    children
        .iter()
        .position(|candidate| SharedPtr::as_ptr(candidate) == child)
        .unwrap_or(children.len())
}

// 0x6ffa58 — __ZN3RBX8Instance29findFirstChildByNameRecursiveERKSs
#[doc(alias = "RBX::Instance::findFirstChildByNameRecursive(std::string const&)")]
// was: RBX::Instance::findFirstChildByNameRecursive(std::string const&)
pub fn stub_6ffa58(this: *const Instance, name: &str) -> *const Instance {
    // IDA 0x6ffa58: direct `findConstFirstChildByName` first (disasm
    // 0x6ffa60); on miss recurse into each `+56` child in order via the
    // `copy_on_write` snapshot (disasm 0x6ffa66-0x6ffa98), first hit wins,
    // null when the subtree has no match.
    // SAFETY: `this` must point to a valid `Instance` whose subtree outlives
    // the call.
    unsafe {
        let direct = stub_6ffa9c(this, name);
        if !direct.is_null() {
            return direct;
        }
        for child in (*this).children.clone().iter() {
            let hit = stub_6ffa58(SharedPtr::as_ptr(child), name);
            if !hit.is_null() {
                return hit;
            }
        }
        core::ptr::null()
    }
}

// 0x6ffa9c — __ZNK3RBX8Instance25findConstFirstChildByNameERKSs
#[doc(alias = "RBX::Instance::findConstFirstChildByName(std::string const&)const")]
// was: RBX::Instance::findConstFirstChildByName(std::string const&)const
pub fn stub_6ffa9c(this: *const Instance, name: &str) -> *const Instance {
    // IDA 0x6ffa9c: linear scan of the `+56` snapshot (disasm 0x6ffaa6-0x6fface)
    // comparing `string::compare(*(child + 68) + 24, name)` — the embedded
    // name — first match wins, null on miss (disasm 0x6ffade).
    // SAFETY: `this` must point to a valid `Instance` whose subtree outlives
    // the call.
    unsafe {
        for child in (*this).children.iter() {
            if (*SharedPtr::as_ptr(child)).name.text == name {
                return SharedPtr::as_ptr(child);
            }
        }
        core::ptr::null()
    }
}

// 0x6ffae0 — __ZNK3RBX8Instance19findFirstAncestorOfEPKS0_
#[doc(alias = "RBX::Instance::findFirstAncestorOf(RBX::Instance const*)const")]
// was: RBX::Instance::findFirstAncestorOf(RBX::Instance const*)const
pub fn stub_6ffae0(this: *const Instance, target: *const Instance) -> *const Instance {
    // IDA 0x6ffae0: for each direct child `c` of `this` (disasm 0x6ffb00-0x6ffb1e),
    // walk `target`'s ancestry via `+52` (disasm 0x6ffb0a-0x6ffb0c); a hit
    // retains into the out `shared_ptr` and returns `c` (disasm 0x6ffb2c-0x6ffb3e —
    // the retain collapses to the borrow). Miss zeroes the out holder and
    // returns null (disasm 0x6ffb24-0x6ffb44).
    // SAFETY: both must point to valid `Instance`s whose trees outlive the call.
    unsafe {
        for child in (*this).children.iter() {
            let candidate = SharedPtr::as_ptr(child);
            let mut cursor = target;
            while !cursor.is_null() {
                cursor = (*cursor).parent;
                if cursor == candidate {
                    return candidate;
                }
            }
        }
        core::ptr::null()
    }
}

// 0x6ffb48 — __ZNK3RBX8Instance13securityCheckEv
#[doc(alias = "RBX::Instance::securityCheck(void)const")]
// was: RBX::Instance::securityCheck(void)const
pub fn stub_6ffb48(this: *const Instance) -> bool {
    // IDA 0x6ffb48: `Context::current()` snapshot (disasm 0x6ffb52), then the
    // `securityCheck(Context &)` walk below.
    // SAFETY: `this` must point to a valid `Instance` ancestry.
    stub_6ffb68(this, &SecurityContext::current())
}

// 0x6ffb68 — __ZNK3RBX8Instance13securityCheckERNS_8Security7ContextE
#[doc(alias = "RBX::Instance::securityCheck(RBX::Security::Context &)const")]
// was: RBX::Instance::securityCheck(RBX::Security::Context &)const
pub fn stub_6ffb68(this: *const Instance, context: &SecurityContext) -> bool {
    // IDA 0x6ffb68: walk `this` to the root via `+52` (disasm 0x6ffb7c-0x6ffb80),
    // `requirePermission(ctx, *(classDesc + 276))` per instance (disasm
    // 0x6ffb78), returning the last result. Permission words default via
    // `class_permission` (see `SecurityContext`).
    // SAFETY: `this` must point to a valid `Instance` ancestry.
    unsafe {
        let mut cursor = this;
        let mut granted = true;
        while !cursor.is_null() {
            granted = context.require_permission(class_permission((*cursor).class_name));
            cursor = (*cursor).parent;
        }
        granted
    }
}

// 0x6ffb84 — __ZNK3RBX8Instance17verifySetAncestorEPKS0_S2_
#[doc(alias = "RBX::Instance::verifySetAncestor(RBX::Instance const*,RBX::Instance const*)const")]
// was: RBX::Instance::verifySetAncestor(RBX::Instance const*,RBX::Instance const*)const
pub fn stub_6ffb84(this: *const Instance, ancestor: *const Instance, descendant: *const Instance) {
    // IDA 0x6ffb84: base implementation iterated by the `Keyframe` /
    // `KeyframeSequence` / `Pose` overrides (xrefs 0x5b1ea8, 0x5b5670,
    // 0x605f18): walk the `+56` vector, calling each entry's `+60` virtual
    // with `(entry, a2, a3)` (disasm 0x6ffbf6-0x6ffc0e) — the slot is
    // `verifySetAncestor` itself, i.e. recursion into the children. The
    // `shared_count` copy/release pair is a snapshot clone; virtual dispatch
    // collapses until subclass overrides are modelled.
    // SAFETY: `this` must point to a valid `Instance` whose subtree outlives
    // the call.
    unsafe {
        let snapshot = (*this).children.clone();
        for child in snapshot.iter() {
            stub_6ffb84(SharedPtr::as_ptr(child), ancestor, descendant);
        }
    }
}

// 0x6ffc74 — __ZNK3RBX8Instance19verifyAddDescendantEPKS0_S2_
#[doc(alias = "RBX::Instance::verifyAddDescendant(RBX::Instance const*,RBX::Instance const*)const")]
// was: RBX::Instance::verifyAddDescendant(RBX::Instance const*,RBX::Instance const*)const
pub fn stub_6ffc74(this: *const Instance, descendant: *const Instance) -> *const Instance {
    // IDA 0x6ffc74: `parent = *(this + 13)` (disasm 0x6ffc74); null parent
    // returns null (disasm 0x6ffc76-0x6ffc80). When the descendant sits
    // directly under that parent (`a3->parent == parent`, disasm 0x6ffc7a-0x6ffc7e)
    // — or anywhere beneath it (chain walk, disasm 0x6ffc82-0x6ffc86) — the
    // parent is returned; otherwise the check delegates to the parent's
    // `+68` virtual (`(result, result, a3)`, disasm 0x6ffc96), i.e.
    // recursion, since the overwritten `a2` is unused. Returns the
    // conflicting ancestor or null.
    // SAFETY: both must point to valid `Instance`s whose trees outlive the call.
    unsafe {
        let parent = (*this).parent;
        if parent.is_null() {
            return core::ptr::null();
        }
        if (*descendant).parent == parent {
            return parent;
        }
        let mut cursor = (*descendant).parent;
        while !cursor.is_null() {
            cursor = (*cursor).parent;
            if cursor == parent {
                return parent;
            }
        }
        stub_6ffc74(parent, descendant)
    }
}

// 0x6ffc98 — __ZN3RBX8Instance17setParentInternalEPS0_b
#[doc(alias = "RBX::Instance::setParentInternal(RBX::Instance*,bool)")]
// was: RBX::Instance::setParentInternal(RBX::Instance*,bool)
/// Fan-out behind `RBX::Instance::signalDescendantRemoving` (IDA `0x6fff30`):
/// every ancestor from `old` to the root loses a descendant, so each live
/// (write-allocated, cf. `*(a1 + 19)` in `childRemovedSignal`) signal fires
/// with the removed child.
fn signal_descendant_removing_chain(old: *const Instance, child: &SharedPtr<Instance>) {
    // SAFETY: `old` must head a valid `Instance` ancestry outliving the call.
    unsafe {
        let mut cursor = old;
        while !cursor.is_null() {
            let ancestor = cursor as *mut Instance;
            if (*ancestor).write.is_some() {
                (*ancestor)
                    .write
                    .as_mut()
                    .unwrap()
                    .descendant_removing
                    .fire(child.clone());
            }
            cursor = (*ancestor).parent;
        }
    }
}
/// Fan-out behind `RBX::Instance::signalDescendantAdded` (IDA `0x700194`):
/// mirror image of `signal_descendant_removing_chain` along the new chain.
fn signal_descendant_added_chain(new_parent: *const Instance, child: &SharedPtr<Instance>) {
    // SAFETY: `new_parent` must head a valid `Instance` ancestry outliving the call.
    unsafe {
        let mut cursor = new_parent;
        while !cursor.is_null() {
            let ancestor = cursor as *mut Instance;
            if (*ancestor).write.is_some() {
                (*ancestor)
                    .write
                    .as_mut()
                    .unwrap()
                    .descendant_added
                    .fire(child.clone());
            }
            cursor = (*ancestor).parent;
        }
    }
}
/// Collapse of the `RBX::shared_from<Instance>` retains (IDA `0x6ffeba`,
/// `0x6ffec6`): the caller-held borrows already keep both ends alive, so the
/// retain/release pairs vanish; per-fire retains use `borrow_shared`.
pub fn stub_6ffc98(this: *mut Instance, new_parent: *const Instance, skip_lock_check: bool) -> bool {
    // IDA 0x6ffc98 (Client/App/v8tree/Instance.cpp): same-parent fast path
    // (`v11 == a2`, disasm 0x6ffcf2) returns true; locked `Parent` property
    // (`name_store + 21`, disasm 0x6ffcfc) throws `runtime_error`
    // ("The Parent property of %s is locked"); self-parent throws
    // ("Attempt to set %s as its own parent", disasm 0x6ffd08); a `this`
    // found walking the new ancestry throws the circular-reference error
    // after `ReleaseAssert(newParent)` (Instance.cpp:454, disasm 0x6ffd0c-0x6ffe52).
    // `runtime_error` throws map to panics with the same messages.
    // SAFETY: `this` must point to a valid `Instance`; both trees must
    // outlive the call with caller-held ownership (the `shared_from`
    // retains); `new_parent` must be null or valid.
    unsafe {
        let old = (*this).parent;
        if old == new_parent {
            return true;
        }
        if !skip_lock_check && (*this).parent_locked {
            panic!(
                "The Parent property of {} is locked",
                (*this).name.text
            );
        }
        if this as *const Instance == new_parent {
            panic!(
                "Attempt to set {} as its own parent",
                (*this).name.text
            );
        }
        let mut cursor = new_parent;
        while !cursor.is_null() {
            cursor = (*cursor).parent;
            if cursor == this as *const Instance {
                debug_assert!(!new_parent.is_null(), "0x6ffc98: newParent Instance.cpp:454");
                panic!(
                    "Attempt to set parent of {} to {} would result in circular reference",
                    (*this).name.text,
                    (*new_parent).name.text
                );
            }
        }
        // `FLog::InstanceTreeManipulation` line (disasm 0x6ffe7a) collapses.
        // Re-entrancy guard (byte `+64`, disasm 0x6ffe86): a live guard with
        // a changed target logs to `StandardOut` (collapses) and either
        // throws ("Something unexpectedly tried to set the parent of %s...",
        // `RBX::runtime_error`) or — under `FFlag::NoThrowOnReparenting`
        // (default false, hence `NO_THROW_ON_REPARENTING`) — unwinds to a
        // `false` return (disasm 0x6ffe8e-0x70008c).
        if (*this).in_set_parent {
            if old != new_parent {
                if !NO_THROW_ON_REPARENTING {
                    panic!(
                        "Something unexpectedly tried to set the parent of {} to {} while trying to set the parent of {}. Current parent is {}.",
                        (*this).name.text,
                        if new_parent.is_null() {
                            "NULL".to_string()
                        } else {
                            (*new_parent).name.text.clone()
                        },
                        (*this).name.text,
                        if old.is_null() {
                            "NULL".to_string()
                        } else {
                            (*old).name.text.clone()
                        },
                    );
                }
                return false;
            }
            return true;
        }
        (*this).in_set_parent = true;
        // Pre-move virtuals: `+56` (this, new) and `+60` (this, new, this)
        // (disasm 0x6ffee0-0x6ffef2).
        if let Some(hook) = (*this).hooks.changing {
            hook(this, new_parent);
        }
        if let Some(hook) = (*this).hooks.ancestry_changing {
            hook(this, new_parent, this as *const Instance);
        }
        if !new_parent.is_null() {
            let adoptive = new_parent as *mut Instance;
            // `+64` (new, this) and `+68` (new, new, this) (disasm 0x6fff04-0x6fff14).
            if let Some(hook) = (*adoptive).hooks.child_added {
                hook(adoptive, this as *const Instance);
            }
            if let Some(hook) = (*adoptive).hooks.descendant_added {
                hook(adoptive, new_parent, this as *const Instance);
            }
        }
        // Old-chain detach (disasm 0x6fff16-0x7000e0): when the new parent is
        // null or lies outside the old subtree (ancestry walk, disasm
        // 0x6fff22-0x6fff2e), the descendant-removing fan-out fires; the old
        // `+104` virtual runs; the child is erased from the old
        // copy-on-write vector — size `1` clears it (disasm 0x6fff6a-0x6fff88),
        // `>= 0x15` swap-removes (disasm 0x7000a4-0x7000c4), else ordered
        // erase (disasm 0x7000d8) — then `this->parent` zeroes (disasm 0x7000e0).
        let owned = borrow_shared(this as *const Instance);
        if !old.is_null() {
            let mut outside = new_parent.is_null();
            if !outside {
                let mut probe = new_parent;
                let mut found = false;
                while !probe.is_null() {
                    if probe == old {
                        found = true;
                        break;
                    }
                    probe = (*probe).parent;
                }
                outside = !found;
            }
            if outside {
                signal_descendant_removing_chain(old, &owned);
            }
            let previous = old as *mut Instance;
            if let Some(hook) = (*previous).hooks.removing {
                hook(previous, this as *const Instance);
            }
            let siblings = &mut (*previous).children;
            if siblings.len() == 1 {
                siblings.clear();
            } else if let Some(index) = siblings
                .iter()
                .position(|candidate| SharedPtr::as_ptr(candidate) == this as *const Instance)
            {
                if siblings.len() >= 0x15 {
                    siblings.swap_remove(index);
                } else {
                    siblings.remove(index);
                }
            }
            (*this).parent = core::ptr::null();
        }
        // Attach under the new parent (copy-on-write `push_back`, disasm
        // 0x7000e4-0x700102), then the old-side combined `kind = 1` +
        // `childRemovedSignal` + `+108` virtual (disasm 0x700106-0x70015a).
        if !new_parent.is_null() {
            (* (new_parent as *mut Instance))
                .children
                .push(owned.clone());
        }
        (*this).parent = new_parent;
        if !old.is_null() {
            let previous = old as *mut Instance;
            stub_0x703fb0(&mut (*previous).combined, 1, &owned);
            stub_0x703cc0(previous, &owned);
            if let Some(hook) = (*previous).hooks.child_removed {
                hook(previous, this as *const Instance);
            }
        }
        // New-side finish (disasm 0x700170-0x7001f2): the new `+100` virtual;
        // the old ancestry is walked for the new parent (disasm 0x700184-0x700192)
        // and only a miss fires the descendant-added fan-out; then the
        // combined `kind = 0` + `childAddedSignal`.
        if !new_parent.is_null() {
            let adoptive = new_parent as *mut Instance;
            if let Some(hook) = (*adoptive).hooks.added {
                hook(adoptive, this as *const Instance);
            }
            let mut outside = old.is_null();
            if !outside {
                let mut probe = old;
                let mut found = false;
                while !probe.is_null() {
                    if probe == new_parent {
                        found = true;
                        break;
                    }
                    probe = (*probe).parent;
                }
                outside = !found;
            }
            if outside {
                signal_descendant_added_chain(new_parent, &owned);
            }
            stub_0x703fb0(&mut (*adoptive).combined, 0, &owned);
            stub_0x703dc8(adoptive, &owned);
        }
        // `+88` ancestry-changed virtual `(this, old, new)` (disasm 0x7001fc-0x700210),
        // `raisePropertyChanged(propParent)` (disasm 0x700222), guard release
        // (disasm 0x70022a). The trailing retains release (disasm 0x700230-0x70024a).
        if let Some(hook) = (*this).hooks.ancestry_changed {
            hook(this, old, new_parent);
        }
        if let Some(hook) = (*this).hooks.property_changed {
            hook(this, PropertyKind::Parent);
        }
        (*this).in_set_parent = false;
        true
    }
}
/// `FFlag::NoThrowOnReparenting` (IDA `0x70002c`): default `false`, so the
/// re-entrant path throws.
const NO_THROW_ON_REPARENTING: bool = false;
/// `RBX::Instance::getPersistentDataCost` helper shared by `stub_6ff77c` and
/// `stub_6ff888`: the `data_cost` override or `4 + Σ children` (IDA `0x6ff77c`).
fn persistent_data_cost(this: *const Instance) -> i32 {
    // SAFETY: `this` must point to a valid `Instance` subtree.
    unsafe {
        if let Some(hook) = (*this).hooks.data_cost {
            return hook(this);
        }
        let mut total = 4;
        for child in (*this).children.iter() {
            stub_6ff888(child, &mut total);
        }
        total
    }
}

// 0xf5de64 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPN3RBX8InstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeISB_EEEEEEEvNS0_15iterator_detail8iteratorISM_EERNS1_5tableISH_EERT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>>,boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>>> &)")]
// was: void boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>>,boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>>> &)
pub fn stub_f5de64() -> ! {
    todo!("0xf5de64 void boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>>,boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>>> &)")
}

// 0xf5de74 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPN3RBX8InstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISH_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeISB_EEEERSM_RT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>>,boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>> &)")]
// was: void boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>>,boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>> &)
pub fn stub_f5de74() -> ! {
    todo!("0xf5de74 void boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>>,boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>> &)")
}

// 0xf5de84 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPN3RBX8InstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEEixERS8_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::operator[](RBX::Instance * const&)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::operator[](RBX::Instance * const&)
pub fn stub_f5de84() -> ! {
    todo!("0xf5de84 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::operator[](RBX::Instance * const&)")
}

// 0xf5df54 — j___ZN5boost9unordered6detail11node_holderISaINS1_8ptr_nodeISt4pairIKPN3RBX8InstanceENS_8weak_ptrIS6_EEEEEEED2Ev
#[doc(alias = "boost::unordered::detail::node_holder<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>>>::~node_holder()")]
// was: boost::unordered::detail::node_holder<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>>>::~node_holder()
pub fn stub_f5df54() -> ! {
    todo!("0xf5df54 boost::unordered::detail::node_holder<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>>>::~node_holder()")
}

// 0xf5df74 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPN3RBX8InstanceENS_8weak_ptrIS6_EEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>>>::construct(void)")]
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>>>::construct(void)
pub fn stub_f5df74() -> ! {
    todo!("0xf5df74 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>>>::construct(void)")
}

// 0xf5dfe4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPN3RBX8InstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::create_buckets(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::create_buckets(unsigned long)
pub fn stub_f5dfe4() -> ! {
    todo!("0xf5dfe4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::create_buckets(unsigned long)")
}

// 0xf5dff4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPN3RBX8InstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::reserve_for_insert(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::reserve_for_insert(unsigned long)
pub fn stub_f5dff4() -> ! {
    todo!("0xf5dff4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::reserve_for_insert(unsigned long)")
}

// 0xf5e004 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPN3RBX8InstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEE4initERKSI_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::init(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> const&)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::init(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> const&)
pub fn stub_f5e004() -> ! {
    todo!("0xf5e004 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::init(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> const&)")
}

// 0xf5e014 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPN3RBX8InstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEE6assignERKSI_NS1_17integral_constantIbLb0EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> const&,boost::unordered::detail::integral_constant<bool,false>)
pub fn stub_f5e014() -> ! {
    todo!("0xf5e014 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")
}

// 0xf5e024 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPN3RBX8InstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEED2Ev
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,rbx_core::WeakPtr<RBX::Instance>>>,RBX::Instance *,rbx_core::WeakPtr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::~table()")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::~table()
pub fn stub_f5e024() -> ! {
    todo!("0xf5e024 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance * const,boost::weak_ptr<RBX::Instance>>>,RBX::Instance *,boost::weak_ptr<RBX::Instance>,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::~table()")
}

// 0xf5e0d4 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_12SceneUpdaterENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SceneUpdater,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SceneUpdater*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SceneUpdater,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SceneUpdater*>,boost::arg<1>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SceneUpdater,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SceneUpdater*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SceneUpdater,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SceneUpdater*>,boost::arg<1>>> const&)const
pub fn stub_f5e0d4() -> ! {
    todo!("0xf5e0d4 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SceneUpdater,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SceneUpdater*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SceneUpdater,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SceneUpdater*>,boost::arg<1>>> const&)const")
}

// 0xf5e0e4 — j___ZNK5boost4_mfi3mf1IvN3RBX12SceneUpdaterENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
#[doc(alias = "boost::_mfi::mf1<void,RBX::SceneUpdater,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::SceneUpdater*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::SceneUpdater,boost::shared_ptr<RBX::Instance>>::operator()(RBX::SceneUpdater*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_f5e0e4() -> ! {
    todo!("0xf5e0e4 boost::_mfi::mf1<void,RBX::SceneUpdater,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::SceneUpdater*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0xf5e304 — j___ZNK5boost4_mfi3mf1IvN3RBX7Network23TopNErrorsPhysicsSenderENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_f5e304() -> ! {
    todo!("0xf5e304 boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0xf5e7f4 — j___ZN3RBX10Reflection11Call5HelperINS_7Network6ClientEMS3_FN5boost10shared_ptrINS_8InstanceEEEiSsiiiEiSsiiiS7_E4callEPS3_S9_RNS0_7VariantERKiRKSsSF_SF_SF_
#[doc(alias = "RBX::Reflection::Call5Helper<RBX::Network::Client,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),int,std::string,int,int,int,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Network::Client*,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),RBX::Reflection::Variant &,int const&,std::string const&,int const&,int const&,int const&)")]
// was: RBX::Reflection::Call5Helper<RBX::Network::Client,boost::shared_ptr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),int,std::string,int,int,int,boost::shared_ptr<RBX::Instance>>::call(RBX::Network::Client*,boost::shared_ptr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),RBX::Reflection::Variant &,int const&,std::string const&,int const&,int const&,int const&)
pub fn stub_f5e7f4() -> ! {
    todo!("0xf5e7f4 RBX::Reflection::Call5Helper<RBX::Network::Client,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),int,std::string,int,int,int,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Network::Client*,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),RBX::Reflection::Variant &,int const&,std::string const&,int const&,int const&,int const&)")
}

// 0xf5e804 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFN5boost10shared_ptrINS_8InstanceEEEiSsiiiELi5EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_SB_SC_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Client,rbx_core::SharedPtr<RBX::Instance> ()(int,std::string,int,int,int),5>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Client,boost::shared_ptr<RBX::Instance> ()(int,std::string,int,int,int),5>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
pub fn stub_f5e804() -> ! {
    todo!("0xf5e804 RBX::Reflection::BoundFuncDesc<RBX::Network::Client,rbx_core::SharedPtr<RBX::Instance> ()(int,std::string,int,int,int),5>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0xf5e814 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFN5boost10shared_ptrINS_8InstanceEEEiSsiiiELi5EEC2EMS3_FS7_iSsiiiEPKcSD_SD_SD_SD_iSD_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Client,rbx_core::SharedPtr<RBX::Instance> ()(int,std::string,int,int,int),5>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),char const*,char const*,char const*,char const*,char const*,int,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Client,boost::shared_ptr<RBX::Instance> ()(int,std::string,int,int,int),5>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),char const*,char const*,char const*,char const*,char const*,int,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f5e814() -> ! {
    todo!("0xf5e814 RBX::Reflection::BoundFuncDesc<RBX::Network::Client,rbx_core::SharedPtr<RBX::Instance> ()(int,std::string,int,int,int),5>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),char const*,char const*,char const*,char const*,char const*,int,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5e824 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFN5boost10shared_ptrINS_8InstanceEEEiSsiiiELi5EED2Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Client,rbx_core::SharedPtr<RBX::Instance> ()(int,std::string,int,int,int),5>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Client,boost::shared_ptr<RBX::Instance> ()(int,std::string,int,int,int),5>::~BoundFuncDesc()
pub fn stub_f5e824() -> ! {
    todo!("0xf5e824 RBX::Reflection::BoundFuncDesc<RBX::Network::Client,rbx_core::SharedPtr<RBX::Instance> ()(int,std::string,int,int,int),5>::~BoundFuncDesc()")
}

// 0xf5e864 — j___ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Client::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Client::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f5e864() -> ! {
    todo!("0xf5e864 RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Client::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5e894 — j___ZN3rbx7signals16signal_with_argsILi2EFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE8fireItemEPNS0_6signalIS7_E4slotESsS6_
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::signals::signal_with_args<2,void ()(std::string,boost::shared_ptr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot *,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_f5e894() -> ! {
    todo!("0xf5e894 rbx::signals::signal_with_args<2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf5e8c4 — j___ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::disconnectAll(void)
pub fn stub_f5e8c4() -> ! {
    todo!("0xf5e8c4 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")
}

// 0xf5e8d4 — j___ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE5mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::mutex(void)")]
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::mutex(void)
pub fn stub_f5e8d4() -> ! {
    todo!("0xf5e8d4 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::mutex(void)")
}

// 0xf5e8e4 — j___ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_f5e8e4() -> ! {
    todo!("0xf5e8e4 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")
}

// 0xf5e8f4 — j___ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE6removeEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_f5e8f4() -> ! {
    todo!("0xf5e8f4 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")
}

// 0xf5e934 — j___ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi2ES8_E4callESsS7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,2,void ()(std::string,boost::shared_ptr<RBX::Instance>)>::call(std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_f5e934() -> ! {
    todo!("0xf5e934 rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf5e944 — j___ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi2ES8_ED2Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,2,void ()(std::string,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_f5e944() -> ! {
    todo!("0xf5e944 rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0xf5e9d4 — j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX9DataModelEEEEEEC2ES7_
#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>)")]
// was: boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::DataModel>>)
pub fn stub_f5e9d4() -> ! {
    todo!("0xf5e9d4 boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>)")
}

// 0xf5e9e4 — j___ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEEEC2ES7_
#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::list1(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>)")]
// was: boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::list1(boost::_bi::value<boost::weak_ptr<RBX::DataModel>>)
pub fn stub_f5e9e4() -> ! {
    todo!("0xf5e9e4 boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::list1(boost::_bi::value<boost::weak_ptr<RBX::DataModel>>)")
}

// 0xf5e9f4 — j___ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEEEclIPFvS6_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::DataModel>) &,boost::_bi::list0 &,int)")]
// was: void boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::operator()<void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::DataModel>) &,boost::_bi::list0 &,int)
pub fn stub_f5e9f4() -> ! {
    todo!("0xf5e9f4 void boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::operator()<void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::DataModel>) &,boost::_bi::list0 &,int)")
}

// 0xf5ea04 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS1_8InstanceEEENS6_IS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,boost::shared_ptr<RBX::Instance> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub fn stub_f5ea04() -> ! {
    todo!("0xf5ea04 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}

// 0xf5ea24 — j___ZN5boost4bindIvNS_10shared_ptrIN3RBX9DataModelEEES4_EENS_3_bi6bind_tIT_PFS7_T0_ENS5_9list_av_1IT1_E4typeEEESA_SC_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::DataModel>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::DataModel>,rbx_core::SharedPtr<RBX::DataModel>>(void (*)(rbx_core::SharedPtr<RBX::DataModel>),rbx_core::SharedPtr<RBX::DataModel>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>),boost::_bi::list_av_1<boost::shared_ptr<RBX::DataModel>>::type> boost::bind<void,boost::shared_ptr<RBX::DataModel>,boost::shared_ptr<RBX::DataModel>>(void (*)(boost::shared_ptr<RBX::DataModel>),boost::shared_ptr<RBX::DataModel>)
pub fn stub_f5ea24() -> ! {
    todo!("0xf5ea24 boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::DataModel>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::DataModel>,rbx_core::SharedPtr<RBX::DataModel>>(void (*)(rbx_core::SharedPtr<RBX::DataModel>),rbx_core::SharedPtr<RBX::DataModel>)")
}

// 0xf5ea34 — j___ZN5boost6detail13heap_new_implINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEEENS3_5list1INS3_5valueIS8_EEEEEEEERSF_EEPT_T0_
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>&>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>&)")]
// was: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>>>&>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>>>&)
pub fn stub_f5ea34() -> ! {
    todo!("0xf5ea34 boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>&>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>&)")
}

// 0xf5ea44 — j___ZN5boost6detail13heap_new_implINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEENS3_5list1INS3_5valueIS8_EEEEEEEERSF_EEPT_T0_
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>&>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>&)")]
// was: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>&>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>&)
pub fn stub_f5ea44() -> ! {
    todo!("0xf5ea44 boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>&>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>&)")
}

// 0xf5ea54 — j___ZN5boost6detail20sp_pointer_constructINS0_16thread_data_baseENS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEEENS4_5list1INS4_5valueIS9_EEEEEEEEEEvPNS6_IT_EEPT0_RNS0_12shared_countE
#[doc(alias = "void boost::detail::sp_pointer_construct<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> *,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>> *,boost::detail::shared_count &)")]
// was: void boost::detail::sp_pointer_construct<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> *,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>>>> *,boost::detail::shared_count &)
pub fn stub_f5ea54() -> ! {
    todo!("0xf5ea54 void boost::detail::sp_pointer_construct<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> *,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>> *,boost::detail::shared_count &)")
}

// 0xf5ea64 — j___ZN5boost6detail20sp_pointer_constructINS0_16thread_data_baseENS0_11thread_dataINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEENS4_5list1INS4_5valueIS9_EEEEEEEEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
#[doc(alias = "void boost::detail::sp_pointer_construct<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> *,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>> *,boost::detail::shared_count &)")]
// was: void boost::detail::sp_pointer_construct<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> *,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>> *,boost::detail::shared_count &)
pub fn stub_f5ea64() -> ! {
    todo!("0xf5ea64 void boost::detail::sp_pointer_construct<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> *,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>> *,boost::detail::shared_count &)")
}

// 0xf5eaf4 — j___ZNK3RBX10Reflection13EventDescBaseINS_7Network6ClientEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E7connectEPNS0_11EventSourceERKNS4_8functionIS8_EE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Client::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Client::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,boost::shared_ptr<RBX::Instance>)> const&)const
pub fn stub_f5eaf4() -> ! {
    todo!("0xf5eaf4 RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Client::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)const")
}

// 0xf5eb14 — j___ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIvNS2_4_mfi3mf0IvS0_EENS3_5list1INS2_3argILi1EEEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>> const&)const")]
// was: void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>> const&)const
pub fn stub_f5eb14() -> ! {
    todo!("0xf5eb14 void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>> const&)const")
}

// 0xf5eb24 — j___ZNK3RBX8Instance25findConstFirstChildOfTypeINS_7Network16ClientReplicatorEEEPKT_v
#[doc(alias = "RBX::Network::ClientReplicator const* RBX::Instance::findConstFirstChildOfType<RBX::Network::ClientReplicator>(void)const")]
// was: RBX::Network::ClientReplicator const* RBX::Instance::findConstFirstChildOfType<RBX::Network::ClientReplicator>(void)const
pub fn stub_f5eb24() -> ! {
    todo!("0xf5eb24 RBX::Network::ClientReplicator const* RBX::Instance::findConstFirstChildOfType<RBX::Network::ClientReplicator>(void)const")
}

// 0xf5eb84 — j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEEENS6_5list1INS6_5valueISB_EEEEEEEEEEvPKNS8_IT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>> *)const")]
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>>>> *)const
pub fn stub_f5eb84() -> ! {
    todo!("0xf5eb84 void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel>>>>> *)const")
}

// 0xf5eb94 — j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEENS6_5list1INS6_5valueISB_EEEEEEEEEEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>>>> *)const")]
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>> *)const
pub fn stub_f5eb94() -> ! {
    todo!("0xf5eb94 void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>> *)const")
}

// 0xf5ebb4 — j___ZNK5boost9function2IvSsNS_10shared_ptrIN3RBX8InstanceEEEEclESsS4_
#[doc(alias = "boost::function2<void,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::function2<void,std::string,boost::shared_ptr<RBX::Instance>>::operator()(std::string,boost::shared_ptr<RBX::Instance>)const
pub fn stub_f5ebb4() -> ! {
    todo!("0xf5ebb4 boost::function2<void,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(std::string,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0xf5ebd4 — j___ZN3RBX10Reflection13DescribedBase21fastSharedDynamicCastINS_13JointInstanceENS_8InstanceEEEN5boost10shared_ptrIT_EERKNS6_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::JointInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: boost::shared_ptr<RBX::JointInstance> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::JointInstance,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_f5ebd4() -> ! {
    todo!("0xf5ebd4 rbx_core::SharedPtr<RBX::JointInstance> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::JointInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf5ecb4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_7Network16ClientReplicator15ClientStatsItemEN5boost10shared_ptrIS5_EEEENS8_IT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ClientReplicator::ClientStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::Network::ClientReplicator::ClientStatsItem,rbx_core::SharedPtr<RBX::Network::ClientReplicator>>(rbx_core::SharedPtr<RBX::Network::ClientReplicator>)")]
// was: boost::shared_ptr<RBX::Network::ClientReplicator::ClientStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::Network::ClientReplicator::ClientStatsItem,boost::shared_ptr<RBX::Network::ClientReplicator>>(boost::shared_ptr<RBX::Network::ClientReplicator>)
pub fn stub_f5ecb4() -> ! {
    todo!("0xf5ecb4 rbx_core::SharedPtr<RBX::Network::ClientReplicator::ClientStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::Network::ClientReplicator::ClientStatsItem,rbx_core::SharedPtr<RBX::Network::ClientReplicator>>(rbx_core::SharedPtr<RBX::Network::ClientReplicator>)")
}

// 0xf5ed04 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX8GuidItemINS4_8InstanceEE8RegistryEEEEENS_3argILi1EEEEclIPFvS9_NS3_IS6_EEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>>::operator()<void (*)(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>>::operator()<void (*)(boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_f5ed04() -> ! {
    todo!("0xf5ed04 void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>>::operator()<void (*)(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf5ed14 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX7Network16ClientReplicatorERKNS4_4Guid4DataEPNS4_12PartInstanceENS_10shared_ptrINS4_8InstanceEEEEENS0_5list4INS0_5valueIPS6_EENSI_IS8_EENSI_ISC_EENS_3argILi1EEEEEEclISF_EEvRKT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>>::operator()<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>>::operator()<boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_f5ed14() -> ! {
    todo!("0xf5ed14 void boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>>::operator()<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf5ed24 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX8GuidItemINS4_8InstanceEE8RegistryEEEEENS_3argILi1EEEEC2ESA_SC_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>)")]
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>)
pub fn stub_f5ed24() -> ! {
    todo!("0xf5ed24 boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>)")
}

// 0xf5ed34 — j___ZN5boost4bindIvNS_10shared_ptrIN3RBX8GuidItemINS2_8InstanceEE8RegistryEEENS1_IS4_EES7_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,boost::arg<1>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,boost::arg<1>>(void (*)(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,boost::arg<1>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>,boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_2<boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>,boost::arg<1>>::type> boost::bind<void,boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>,boost::arg<1>>(void (*)(boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>,boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>,boost::arg<1>)
pub fn stub_f5ed34() -> ! {
    todo!("0xf5ed34 boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,boost::arg<1>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,boost::arg<1>>(void (*)(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,boost::arg<1>)")
}

// 0xf5eda4 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf3IvNS_7Network16ClientReplicatorERKNS_4Guid4DataEPNS_12PartInstanceENS2_10shared_ptrIS0_EEEENS3_5list4INS3_5valueIPS8_EENSJ_ISA_EENSJ_ISE_EENS2_3argILi1EEEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>> const&)const
pub fn stub_f5eda4() -> ! {
    todo!("0xf5eda4 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>> const&)const")
}

// 0xf5edb4 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrINS_8GuidItemIS0_E8RegistryEEENS5_IS0_EEENS3_5list2INS3_5valueIS9_EENS2_3argILi1EEEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>>> const&)const
pub fn stub_f5edb4() -> ! {
    todo!("0xf5edb4 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>>,boost::arg<1>>> const&)const")
}

// 0xf5ee04 — j___ZNK5boost4_mfi3mf3IvN3RBX7Network16ClientReplicatorERKNS2_4Guid4DataEPNS2_12PartInstanceENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S8_SA_SD_
#[doc(alias = "boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::ClientReplicator*,RBX::Guid::Data const&,RBX::PartInstance *,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::ClientReplicator*,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>)const
pub fn stub_f5ee04() -> ! {
    todo!("0xf5ee04 boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::ClientReplicator*,RBX::Guid::Data const&,RBX::PartInstance *,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0xf5ee64 — j___ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE5mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::mutex(void)
pub fn stub_f5ee64() -> ! {
    todo!("0xf5ee64 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::mutex(void)")
}

// 0xf5f434 — j___ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJobC2EN5boost10shared_ptrIN6RakNet16RakPeerInterfaceEEEPNS_9DataModelE
#[doc(alias = "RBX::Network::ConcurrentRakPeer::StatsUpdateJob::StatsUpdateJob(rbx_core::SharedPtr<RakNet::RakPeerInterface>,RBX::DataModel *)")]
// was: RBX::Network::ConcurrentRakPeer::StatsUpdateJob::StatsUpdateJob(boost::shared_ptr<RakNet::RakPeerInterface>,RBX::DataModel *)
pub fn stub_f5f434() -> ! {
    todo!("0xf5f434 RBX::Network::ConcurrentRakPeer::StatsUpdateJob::StatsUpdateJob(rbx_core::SharedPtr<RakNet::RakPeerInterface>,RBX::DataModel *)")
}

// 0xf5f454 — j___ZN3RBX7Network17ConcurrentRakPeer9PacketJobC2EN5boost10shared_ptrIN6RakNet16RakPeerInterfaceEEEPNS_9DataModelE
#[doc(alias = "RBX::Network::ConcurrentRakPeer::PacketJob::PacketJob(rbx_core::SharedPtr<RakNet::RakPeerInterface>,RBX::DataModel *)")]
// was: RBX::Network::ConcurrentRakPeer::PacketJob::PacketJob(boost::shared_ptr<RakNet::RakPeerInterface>,RBX::DataModel *)
pub fn stub_f5f454() -> ! {
    todo!("0xf5f454 RBX::Network::ConcurrentRakPeer::PacketJob::PacketJob(rbx_core::SharedPtr<RakNet::RakPeerInterface>,RBX::DataModel *)")
}
