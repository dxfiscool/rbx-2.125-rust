//! Skeletons for early boost-backed rbx primitives selected from `ida/export.json`.
//!
//! These are compile-only Rust cutover points. They intentionally keep the IDA
//! address, mangled symbol, and original boost-heavy spelling next to each stub.

use crate::SharedPtr;

pub struct SignalSlot;
pub struct Connection;
pub struct ThreadHandle(pub std::thread::JoinHandle<()>);
pub struct ThreadResourceErrorClone;

pub type StringCallback = Box<dyn Fn(String) + Send + Sync + 'static>;
pub type VoidCallback = Box<dyn Fn() + Send + Sync + 'static>;
pub type PropertyDescriptorCallback = Box<dyn Fn(*const ()) + Send + Sync + 'static>;

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
// 0xf574 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)
pub fn next_property_descriptor_signal_slot_f574(_slot: SharedPtr<SignalSlot>) -> SharedPtr<SignalSlot> {
    todo!("0xf574 __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")]
// 0x2c8c0 — __ZN3rbx7signals6signalIFvSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)
pub fn connect_string_signal_2c8c0(_callback: StringCallback) -> Connection {
    todo!("0x2c8c0 __ZN3rbx7signals6signalIFvSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "boost::thread::thread<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>> &&)")]
// 0x2dc24 — __ZN5boost6threadC2INS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEEOT_
// was: boost::thread::thread<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>> &&)
pub fn spawn_bound_game_thread_2dc24(_game: SharedPtr<()>) -> ThreadHandle {
    todo!("0x2dc24 __ZN5boost6threadC2INS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEEOT_")
}

#[doc(alias = "boost::thread::thread<boost::function0<void>>(boost::function0<void> &&)")]
// 0x3073c — __ZN5boost6threadC2INS_9function0IvEEEEOT_
// was: boost::thread::thread<boost::function0<void>>(boost::function0<void> &&)
pub fn spawn_function_thread_3073c(_callback: VoidCallback) -> ThreadHandle {
    todo!("0x3073c __ZN5boost6threadC2INS_9function0IvEEEEOT_")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string)>::slot*)")]
// 0x31e24 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string)>::slot*)
pub fn assign_string_signal_slot_ptr_31e24(_slot: SharedPtr<SignalSlot>) -> SharedPtr<SignalSlot> {
    todo!("0x31e24 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::callable<rbx::signals::signal<void ()(std::string)>*>(boost::function<void ()(std::string)> const&,rbx::signals::signal<void ()(std::string)>*)")]
// 0x31fc0 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
// was: rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::callable<rbx::signals::signal<void ()(std::string)>*>(boost::function<void ()(std::string)> const&,rbx::signals::signal<void ()(std::string)>*)
pub fn new_string_signal_callable_31fc0(_callback: StringCallback) -> SharedPtr<SignalSlot> {
    todo!("0x31fc0 __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::function<void ()(std::string)>>::~callable_slot()")]
// 0x320bc — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost8functionIS2_EEED0Ev
// was: rbx::signals::signal<void ()(std::string)>::callable_slot<boost::function<void ()(std::string)>>::~callable_slot()
pub fn drop_string_signal_callable_slot_320bc() {
    todo!("0x320bc __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost8functionIS2_EEED0Ev")
}

#[doc(alias = "non_virtual_thunk_to rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::call(std::string)")]
// 0x32194 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_E4callESs
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::call(std::string)
pub fn call_string_signal_callable_slot_32194(_slot: &SignalSlot, _value: String) {
    todo!("0x32194 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_E4callESs")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()")]
// 0x3219c — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()
pub fn drop_string_signal_callable_3219c() {
    todo!("0x3219c __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>> const&)")]
// 0x3a278 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>> const&)
pub fn connect_property_descriptor_signal_3a278(_callback: PropertyDescriptorCallback) -> Connection {
    todo!("0x3a278 __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>> const&)")]
// 0x3a390 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>> const&)
pub fn connect_void_signal_3a390(_callback: VoidCallback) -> Connection {
    todo!("0x3a390 __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "void boost::intrusive_ptr_add_weak_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
// 0x3c010 — __ZN5boost26intrusive_ptr_add_weak_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// was: void boost::intrusive_ptr_add_weak_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)
pub fn intrusive_ptr_add_signal_islot_weak_ref_3c010(_slot: &SignalSlot) {
    todo!("0x3c010 __ZN5boost26intrusive_ptr_add_weak_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(void)>::slot>::operator=(rbx::signals::signal<void ()(void)>::slot*)")]
// 0x3c0c8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(void)>::slot>::operator=(rbx::signals::signal<void ()(void)>::slot*)
pub fn assign_void_signal_slot_ptr_3c0c8(_slot: SharedPtr<SignalSlot>) -> SharedPtr<SignalSlot> {
    todo!("0x3c0c8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSEPS6_")
}

#[doc(alias = "boost::thread_resource_error::~thread_resource_error()")]
// 0x3c928 — __ZN5boost21thread_resource_errorD1Ev
// was: boost::thread_resource_error::~thread_resource_error()
pub fn drop_thread_resource_error_3c928() {
    todo!("0x3c928 __ZN5boost21thread_resource_errorD1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
// 0x3c958 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED2Ev
// was: boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()
pub fn drop_thread_resource_error_injector_3c958() {
    todo!("0x3c958 __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED2Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
// 0x3c998 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()
pub fn drop_thread_resource_error_injector_3c998() {
    todo!("0x3c998 __ZThn20_N5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
// 0x3c9e0 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()
pub fn new_thread_resource_error_clone_impl_3c9e0() -> ThreadResourceErrorClone {
    todo!("0x3c9e0 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
// 0x3ca28 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()
pub fn new_thread_resource_error_clone_impl_3ca28() -> ThreadResourceErrorClone {
    todo!("0x3ca28 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const")]
// 0x3ca70 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const
pub fn clone_thread_resource_error_3ca70(_source: &ThreadResourceErrorClone) -> ThreadResourceErrorClone {
    todo!("0x3ca70 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
// 0x3cb30 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()
pub fn new_thread_resource_error_clone_impl_3cb30() -> ThreadResourceErrorClone {
    todo!("0x3cb30 __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const")]
// 0x3cb38 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const
pub fn clone_thread_resource_error_3cb38(_source: &ThreadResourceErrorClone) -> ThreadResourceErrorClone {
    todo!("0x3cb38 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
// 0x3cb48 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED0Ev
// was: boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()
pub fn drop_thread_resource_error_injector_3cb48() {
    todo!("0x3cb48 __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::thread_resource_error> const&)")]
// 0x3cb60 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS4_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::thread_resource_error> const&)
pub fn new_thread_resource_error_clone_impl_3cb60() -> ThreadResourceErrorClone {
    todo!("0x3cb60 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS4_")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()")]
// 0x3cdb8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED1Ev
// was: rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()
pub fn drop_void_signal_callable_slot_3cdb8() {
    todo!("0x3cdb8 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()")]
// 0x3ce64 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED0Ev
// was: rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()
pub fn drop_void_signal_callable_slot_3ce64() {
    todo!("0x3ce64 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)")]
// 0x3cf18 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)
pub fn call_void_signal_callable_slot_3cf18(_slot: &SignalSlot) {
    todo!("0x3cf18 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "non_virtual_thunk_to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)")]
// 0x3cf20 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)
pub fn call_void_signal_callable_slot_3cf20(_slot: &SignalSlot) {
    todo!("0x3cf20 __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()")]
// 0x3d0e4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()
pub fn drop_void_signal_callable_3d0e4() {
    todo!("0x3d0e4 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()")]
// 0x3d190 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()
pub fn drop_void_signal_callable_3d190() {
    todo!("0x3d190 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED0Ev")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> const&)")]
// 0x3d508 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEEaSERKSC_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> const&)
pub fn assign_property_descriptor_signal_slot_ptr_3d508(_slot: SharedPtr<SignalSlot>) -> SharedPtr<SignalSlot> {
    todo!("0x3d508 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEEaSERKSC_")
}
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
// was: rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)
pub fn signals_signal_with_args_call_b76c() {
    todo!("0xb76c __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ResolutionPreset>(RBX::CRenderSettings::ResolutionPreset const&)")]
// 0xc90c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16ResolutionPresetEEERS3_RKT_
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ResolutionPreset>(RBX::CRenderSettings::ResolutionPreset const&)
pub fn placement_any_placement_any_assign_c90c() {
    todo!("0xc90c __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16ResolutionPresetEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::singleton(void)")]
// 0xc95c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE9singletonEv
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::singleton(void)
pub fn typed_holder_singleton_c95c() {
    todo!("0xc95c __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::construct_func(char const*,char *)")]
// 0xc9c8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE14construct_funcEPKcPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::construct_func(char const*,char *)
pub fn typed_holder_construct_func_c9c8() {
    todo!("0xc9c8 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::destruct_func(char *)")]
// 0xc9d4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE13destruct_funcEPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::destruct_func(char *)
pub fn typed_holder_destruct_func_c9d4() {
    todo!("0xc9d4 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE13destruct_funcEPc")
}

#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()")]
// 0xcb94 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
// was: boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()
pub fn exception_detail_refcount_ptr_drop_refcount_ptr_cb94() {
    todo!("0xcb94 __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::QualityLevel>(RBX::CRenderSettings::QualityLevel const&)")]
// 0xceec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12QualityLevelEEERS3_RKT_
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::QualityLevel>(RBX::CRenderSettings::QualityLevel const&)
pub fn placement_any_placement_any_assign_ceec() {
    todo!("0xceec __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12QualityLevelEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::singleton(void)")]
// 0xcf3c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE9singletonEv
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::singleton(void)
pub fn typed_holder_singleton_cf3c() {
    todo!("0xcf3c __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::construct_func(char const*,char *)")]
// 0xcfa8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE14construct_funcEPKcPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::construct_func(char const*,char *)
pub fn typed_holder_construct_func_cfa8() {
    todo!("0xcfa8 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::destruct_func(char *)")]
// 0xcfb4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE13destruct_funcEPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::destruct_func(char *)
pub fn typed_holder_destruct_func_cfb4() {
    todo!("0xcfb4 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE13destruct_funcEPc")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ShadowMode>(RBX::CRenderSettings::ShadowMode const&)")]
// 0xd42c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings10ShadowModeEEERS3_RKT_
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ShadowMode>(RBX::CRenderSettings::ShadowMode const&)
pub fn placement_any_placement_any_assign_d42c() {
    todo!("0xd42c __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings10ShadowModeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::singleton(void)")]
// 0xd47c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE9singletonEv
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::singleton(void)
pub fn typed_holder_singleton_d47c() {
    todo!("0xd47c __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::construct_func(char const*,char *)")]
// 0xd4e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE14construct_funcEPKcPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::construct_func(char const*,char *)
pub fn typed_holder_construct_func_d4e8() {
    todo!("0xd4e8 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::destruct_func(char *)")]
// 0xd4f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE13destruct_funcEPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::destruct_func(char *)
pub fn typed_holder_destruct_func_d4f4() {
    todo!("0xd4f4 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE13destruct_funcEPc")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AntialiasingMode>(RBX::CRenderSettings::AntialiasingMode const&)")]
// 0xd96c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16AntialiasingModeEEERS3_RKT_
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AntialiasingMode>(RBX::CRenderSettings::AntialiasingMode const&)
pub fn placement_any_placement_any_assign_d96c() {
    todo!("0xd96c __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16AntialiasingModeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::singleton(void)")]
// 0xd9bc — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE9singletonEv
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::singleton(void)
pub fn typed_holder_singleton_d9bc() {
    todo!("0xd9bc __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::construct_func(char const*,char *)")]
// 0xda28 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE14construct_funcEPKcPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::construct_func(char const*,char *)
pub fn typed_holder_construct_func_da28() {
    todo!("0xda28 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::destruct_func(char *)")]
// 0xda34 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE13destruct_funcEPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::destruct_func(char *)
pub fn typed_holder_destruct_func_da34() {
    todo!("0xda34 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE13destruct_funcEPc")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::FrameRateManagerMode>(RBX::CRenderSettings::FrameRateManagerMode const&)")]
// 0xdeac — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings20FrameRateManagerModeEEERS3_RKT_
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::FrameRateManagerMode>(RBX::CRenderSettings::FrameRateManagerMode const&)
pub fn placement_any_placement_any_assign_deac() {
    todo!("0xdeac __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings20FrameRateManagerModeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::singleton(void)")]
// 0xdefc — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE9singletonEv
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::singleton(void)
pub fn typed_holder_singleton_defc() {
    todo!("0xdefc __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::construct_func(char const*,char *)")]
// 0xdf68 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE14construct_funcEPKcPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::construct_func(char const*,char *)
pub fn typed_holder_construct_func_df68() {
    todo!("0xdf68 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::destruct_func(char *)")]
// 0xdf74 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE13destruct_funcEPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::destruct_func(char *)
pub fn typed_holder_destruct_func_df74() {
    todo!("0xdf74 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE13destruct_funcEPc")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::GraphicsMode>(RBX::CRenderSettings::GraphicsMode const&)")]
// 0xe3ec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12GraphicsModeEEERS3_RKT_
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::GraphicsMode>(RBX::CRenderSettings::GraphicsMode const&)
pub fn placement_any_placement_any_assign_e3ec() {
    todo!("0xe3ec __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12GraphicsModeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::singleton(void)")]
// 0xe43c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE9singletonEv
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::singleton(void)
pub fn typed_holder_singleton_e43c() {
    todo!("0xe43c __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::construct_func(char const*,char *)")]
// 0xe4a8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE14construct_funcEPKcPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::construct_func(char const*,char *)
pub fn typed_holder_construct_func_e4a8() {
    todo!("0xe4a8 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::destruct_func(char *)")]
// 0xe4b4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE13destruct_funcEPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::destruct_func(char *)
pub fn typed_holder_destruct_func_e4b4() {
    todo!("0xe4b4 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE13destruct_funcEPc")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)")]
// 0xe92c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings9AASamplesEEERS3_RKT_
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)
pub fn placement_any_placement_any_assign_e92c() {
    todo!("0xe92c __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings9AASamplesEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)")]
// 0xe97c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE9singletonEv
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)
pub fn typed_holder_singleton_e97c() {
    todo!("0xe97c __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::construct_func(char const*,char *)")]
// 0xe9e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE14construct_funcEPKcPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::construct_func(char const*,char *)
pub fn typed_holder_construct_func_e9e8() {
    todo!("0xe9e8 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::destruct_func(char *)")]
// 0xe9f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE13destruct_funcEPc
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::destruct_func(char *)
pub fn typed_holder_destruct_func_e9f4() {
    todo!("0xe9f4 __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE13destruct_funcEPc")
}

#[doc(alias = "boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")]
// 0xef04 — __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)
pub fn shared_ptr_creatable_create_ef04() {
    todo!("0xef04 __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0xefb4 — __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn shared_ptr_shared_ptr_efb4() {
    todo!("0xefb4 __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
// 0xefd8 — __ZNK5boost6detail15sp_counted_base9use_countEv
// was: boost::detail::sp_counted_base::use_count(void)const
pub fn sp_counted_base_use_count_efd8() {
    todo!("0xefd8 __ZNK5boost6detail15sp_counted_base9use_countEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0xf098 — __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn shared_count_shared_count_f098() {
    todo!("0xf098 __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0xf198 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn sp_counted_impl_pd_drop_sp_counted_impl_pd_f198() {
    todo!("0xf198 __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0xf19c — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn sp_counted_impl_pd_dispose_f19c() {
    todo!("0xf19c __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0xf1bc — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn sp_counted_impl_pd_get_deleter_f1bc() {
    todo!("0xf1bc __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0xf1d4 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn sp_counted_impl_pd_get_untyped_deleter_f1d4() {
    todo!("0xf1d4 __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
// 0xf6dc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)
pub fn signals_signal_on_error_f6dc() {
    todo!("0xf6dc __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception")
}

#[doc(alias = "boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
// 0x17aac — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_
// was: boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)
pub fn shared_ptr_shared_ptr_17aac() {
    todo!("0x17aac __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_")
}

#[doc(alias = "boost::shared_ptr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(boost::shared_ptr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")]
// 0x17b80 — __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE
// was: boost::shared_ptr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(boost::shared_ptr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)
pub fn shared_ptr_shared_ptr_17b80() {
    todo!("0x17b80 __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE")
}

#[doc(alias = "boost::shared_ptr<RBX::Game>::shared_ptr<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")]
// 0x2c9a8 — __ZN5boost10shared_ptrIN3RBX4GameEEC1INS1_16SecurePlayerGameEEEPT_
// was: boost::shared_ptr<RBX::Game>::shared_ptr<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)
pub fn shared_ptr_shared_ptr_2c9a8() {
    todo!("0x2c9a8 __ZN5boost10shared_ptrIN3RBX4GameEEC1INS1_16SecurePlayerGameEEEPT_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<char const*,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,std::string const&,boost::shared_ptr<RBX::Game>,char const*,boost::shared_ptr<RBX::Game>>(void (*)(std::string const&,boost::shared_ptr<RBX::Game>),char const*,boost::shared_ptr<RBX::Game>)")]
// 0x2ca7c — __ZN5boost4bindIvRKSsNS_10shared_ptrIN3RBX4GameEEEPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// was: boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<char const*,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,std::string const&,boost::shared_ptr<RBX::Game>,char const*,boost::shared_ptr<RBX::Game>>(void (*)(std::string const&,boost::shared_ptr<RBX::Game>),char const*,boost::shared_ptr<RBX::Game>)
pub fn bind_t_list_av_2_bind_char_shared_ptr_2ca7c() {
    todo!("0x2ca7c __ZN5boost4bindIvRKSsNS_10shared_ptrIN3RBX4GameEEEPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_3<int,char const*,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,int,std::string const&,boost::shared_ptr<RBX::Game>,int,char const*,boost::shared_ptr<RBX::Game>>(void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),int,char const*,boost::shared_ptr<RBX::Game>)")]
// 0x2cb64 — __ZN5boost4bindIviRKSsNS_10shared_ptrIN3RBX4GameEEEiPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
// was: boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_3<int,char const*,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,int,std::string const&,boost::shared_ptr<RBX::Game>,int,char const*,boost::shared_ptr<RBX::Game>>(void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),int,char const*,boost::shared_ptr<RBX::Game>)
pub fn bind_t_list_av_3_bind_int_char_shared_ptr_2cb64() {
    todo!("0x2cb64 __ZN5boost4bindIviRKSsNS_10shared_ptrIN3RBX4GameEEEiPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list_av_3<int,boost::shared_ptr<RBX::Game>,JoinGameRequest>::type> boost::bind<void,int,boost::shared_ptr<RBX::Game>,JoinGameRequest,int,boost::shared_ptr<RBX::Game>,JoinGameRequest>(void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),int,boost::shared_ptr<RBX::Game>,JoinGameRequest)")]
// 0x2cc54 — __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestiS4_S5_EENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_
// was: boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list_av_3<int,boost::shared_ptr<RBX::Game>,JoinGameRequest>::type> boost::bind<void,int,boost::shared_ptr<RBX::Game>,JoinGameRequest,int,boost::shared_ptr<RBX::Game>,JoinGameRequest>(void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),int,boost::shared_ptr<RBX::Game>,JoinGameRequest)
pub fn bind_t_list_av_3_bind_int_shared_ptr_joingamerequest_2cc54() {
    todo!("0x2cc54 __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestiS4_S5_EENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<int,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,int,boost::shared_ptr<RBX::Game>,int,boost::shared_ptr<RBX::Game>>(void (*)(int,boost::shared_ptr<RBX::Game>),int,boost::shared_ptr<RBX::Game>)")]
// 0x2cd44 — __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEEiS4_EENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
// was: boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<int,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,int,boost::shared_ptr<RBX::Game>,int,boost::shared_ptr<RBX::Game>>(void (*)(int,boost::shared_ptr<RBX::Game>),int,boost::shared_ptr<RBX::Game>)
pub fn bind_t_list_av_2_bind_int_shared_ptr_2cd44() {
    todo!("0x2cd44 __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEEiS4_EENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>,std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>(void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>)")]
// 0x2ce2c — __ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
// was: boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>,std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>(void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>)
pub fn bind_string_string_string_robloxpageviewcontroller_shared_ptr_2ce2c() {
    todo!("0x2ce2c __ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list_av_3<RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *>::type> boost::bind<void,RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *,RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *>(void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *)")]
// 0x2d280 — __ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
// was: boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list_av_3<RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *>::type> boost::bind<void,RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *,RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *>(void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *)
pub fn bind_t_list_av_3_bind_robloxview_shared_ptr_functionmarshaller_2d280() {
    todo!("0x2d280 __ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_")
}

#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)")]
// 0x2d544 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)
pub fn value_value_list3_value_value_value_2d544() {
    todo!("0x2d544 __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x2d644 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn bind_t_list3_value_value_value_manage_2d644() {
    todo!("0x2d644 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// 0x2d660 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEvPNS8_9DataModelEE6invokeERNS1_15function_bufferESN_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn list3_value_value_value_datamodel_invoke_2d660() {
    todo!("0x2d660 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEvPNS8_9DataModelEE6invokeERNS1_15function_bufferESN_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// 0x2d884 — __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn value_value_value_call_list1_int_2d884() {
    todo!("0x2d884 __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x2d964 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn bind_t_list3_value_value_value_manager_2d964() {
    todo!("0x2d964 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::list3(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")]
// 0x2da9c — __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
// was: boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::list3(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)
pub fn list3_value_value_value_list3_2da9c() {
    todo!("0x2da9c __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::storage3(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")]
// 0x2db54 — __ZN5boost3_bi8storage3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
// was: boost::_bi::storage3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::storage3(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)
pub fn storage3_value_value_value_storage3_2db54() {
    todo!("0x2db54 __ZN5boost3_bi8storage3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_")
}

#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>&&)")]
// 0x2dfac — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEC2EOSK_
// was: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>&&)
pub fn list5_value_value_value_value_value_2dfac() {
    todo!("0x2dfac __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEC2EOSK_")
}

#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::~thread_data()")]
// 0x2e0f4 — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEED1Ev
// was: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::~thread_data()
pub fn value_value_value_value_value_drop_thread_data_2e0f4() {
    todo!("0x2e0f4 __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEED1Ev")
}

#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::~thread_data()")]
// 0x2e1bc — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEED0Ev
// was: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::~thread_data()
pub fn value_value_value_value_value_drop_thread_data_2e1bc() {
    todo!("0x2e1bc __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEED0Ev")
}

#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::run(void)")]
// 0x2e284 — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEE3runEv
// was: boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::run(void)
pub fn value_value_value_value_value_run_2e284() {
    todo!("0x2e284 __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEE3runEv")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)")]
// 0x2e2a0 — __ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvSsSsSsP8NSObjectSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
pub fn value_value_value_call_list0_int_2e2a0() {
    todo!("0x2e2a0 __ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvSsSsSsP8NSObjectSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>> *)const")]
// 0x2e518 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS6_5list5INS6_5valueISsEESI_SI_NSH_IP24RobloxPageViewControllerEENSH_ISD_EEEEEEEEEEvPKNSA_IT_EEPT0_
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>> *)const
pub fn list5_value_value_value_value_value_2e518() {
    todo!("0x2e518 __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS6_5list5INS6_5valueISsEESI_SI_NSH_IP24RobloxPageViewControllerEENSH_ISD_EEEEEEEEEEvPKNSA_IT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>> *)")]
// 0x2e5ec — __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS4_5list5INS4_5valueISsEESG_SG_NSF_IP24RobloxPageViewControllerEENSF_ISB_EEEEEEEEEEPT_
// was: boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>> *)
pub fn list5_value_value_value_value_value_2e5ec() {
    todo!("0x2e5ec __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS4_5list5INS4_5valueISsEESG_SG_NSF_IP24RobloxPageViewControllerEENSF_ISB_EEEEEEEEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::~sp_counted_impl_p()")]
// 0x2e6e0 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEED1Ev
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::~sp_counted_impl_p()
pub fn value_value_value_value_value_drop_sp_counted_impl_p_2e6e0() {
    todo!("0x2e6e0 __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::~sp_counted_impl_p()")]
// 0x2e6e4 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEED0Ev
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::~sp_counted_impl_p()
pub fn value_value_value_value_value_drop_sp_counted_impl_p_2e6e4() {
    todo!("0x2e6e4 __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::dispose(void)")]
// 0x2e6e8 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE7disposeEv
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::dispose(void)
pub fn value_value_value_value_value_dispose_2e6e8() {
    todo!("0x2e6e8 __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::get_deleter(std::type_info const&)")]
// 0x2e6f8 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::get_deleter(std::type_info const&)
pub fn value_value_value_value_value_get_deleter_2e6f8() {
    todo!("0x2e6f8 __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::get_untyped_deleter(void)")]
// 0x2e6fc — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>::get_untyped_deleter(void)
pub fn value_value_value_value_value_get_untyped_deleter_2e6fc() {
    todo!("0x2e6fc __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")]
// 0x2e700 — __ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
// was: boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
pub fn value_value_value_value_value_list5_2e700() {
    todo!("0x2e700 __ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")]
// 0x2e970 — __ZN5boost3_bi8storage5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
// was: boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
pub fn value_value_value_value_value_storage5_2e970() {
    todo!("0x2e970 __ZN5boost3_bi8storage5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>)")]
// 0x2ebbc — __ZN5boost3_bi8storage4INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEEEC2ES3_S3_S3_S6_
// was: boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>)
pub fn storage4_storage4_2ebbc() {
    todo!("0x2ebbc __ZN5boost3_bi8storage4INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEEEC2ES3_S3_S3_S6_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
// 0x2edec — __ZN5boost3_bi8storage3INS0_5valueISsEES3_S3_EC2ES3_S3_S3_
// was: boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)
pub fn storage3_storage3_2edec() {
    todo!("0x2edec __ZN5boost3_bi8storage3INS0_5valueISsEES3_S3_EC2ES3_S3_S3_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
// 0x2efb4 — __ZN5boost3_bi8storage2INS0_5valueISsEES3_EC2ES3_S3_
// was: boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)
pub fn storage2_storage2_2efb4() {
    todo!("0x2efb4 __ZN5boost3_bi8storage2INS0_5valueISsEES3_EC2ES3_S3_")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)")]
// 0x2f1d8 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)
pub fn list2_value_value_list2_value_value_2f1d8() {
    todo!("0x2f1d8 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x2f2d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn functor_manager_bind_t_list2_value_value_manage_2f2d0() {
    todo!("0x2f2d0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x2f2ec — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn void_function_obj_invoker0_bind_t_list2_value_value_invoke_2f2ec() {
    todo!("0x2f2ec __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)")]
// 0x2f4fc — __ZN5boost3_bi5list2INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFviS7_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
pub fn list2_value_value_call_list0_int_2f4fc() {
    todo!("0x2f4fc __ZN5boost3_bi5list2INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFviS7_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x2f5d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn functor_manager_bind_t_list2_value_value_manager_2f5d4() {
    todo!("0x2f5d4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")]
// 0x2f708 — __ZN5boost3_bi5list2INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S8_
// was: boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
pub fn list2_value_value_list2_2f708() {
    todo!("0x2f708 __ZN5boost3_bi5list2INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S8_")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>)")]
// 0x2f8bc — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>)
pub fn value_value_list3_value_value_value_2f8bc() {
    todo!("0x2f8bc __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x2f9bc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn bind_t_list3_value_value_value_manage_2f9bc() {
    todo!("0x2f9bc __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x2f9d8 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn bind_t_list3_value_value_value_invoke_2f9d8() {
    todo!("0x2f9d8 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::operator()<void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest) &,boost::_bi::list0 &,int)")]
// 0x2fbf4 — __ZN5boost3_bi5list3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEclIPFviS7_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::operator()<void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest) &,boost::_bi::list0 &,int)
pub fn value_value_value_call_list0_int_2fbf4() {
    todo!("0x2fbf4 __ZN5boost3_bi5list3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEclIPFviS7_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x2fcd4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn bind_t_list3_value_value_value_manager_2fcd4() {
    todo!("0x2fcd4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::list3(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)")]
// 0x2fe0c — __ZN5boost3_bi5list3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEC2ES3_S8_SA_
// was: boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::list3(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)
pub fn list3_value_value_value_list3_2fe0c() {
    todo!("0x2fe0c __ZN5boost3_bi5list3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEC2ES3_S8_SA_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::storage3(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)")]
// 0x2fec4 — __ZN5boost3_bi8storage3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEC2ES3_S8_SA_
// was: boost::_bi::storage3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::storage3(boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)
pub fn storage3_value_value_value_storage3_2fec4() {
    todo!("0x2fec4 __ZN5boost3_bi8storage3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEC2ES3_S8_SA_")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)")]
// 0x30080 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)
pub fn value_value_list3_value_value_value_30080() {
    todo!("0x30080 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x3017c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn bind_t_list3_value_value_value_manage_3017c() {
    todo!("0x3017c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x30198 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn bind_t_list3_value_value_value_invoke_30198() {
    todo!("0x30198 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)")]
// 0x303b8 — __ZN5boost3_bi5list3INS0_5valueIiEENS2_IPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFviRKSsSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
pub fn value_value_value_call_list0_int_303b8() {
    todo!("0x303b8 __ZN5boost3_bi5list3INS0_5valueIiEENS2_IPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFviRKSsSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x30534 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn bind_t_list3_value_value_value_manager_30534() {
    todo!("0x30534 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list3(boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")]
// 0x3066c — __ZN5boost3_bi5list3INS0_5valueIiEENS2_IPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S6_SB_
// was: boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list3(boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
pub fn list3_value_value_value_list3_3066c() {
    todo!("0x3066c __ZN5boost3_bi5list3INS0_5valueIiEENS2_IPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S6_SB_")
}

#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>&&)")]
// 0x30878 — __ZN5boost6detail11thread_dataINS_9function0IvEEEC2EOS3_
// was: boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>&&)
pub fn thread_data_thread_data_30878() {
    todo!("0x30878 __ZN5boost6detail11thread_dataINS_9function0IvEEEC2EOS3_")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)")]
// 0x30a24 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)
pub fn list2_value_value_list2_value_value_30a24() {
    todo!("0x30a24 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x30b1c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn functor_manager_bind_t_list2_value_value_manage_30b1c() {
    todo!("0x30b1c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x30b38 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn void_function_obj_invoker0_bind_t_list2_value_value_invoke_30b38() {
    todo!("0x30b38 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string const&,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)")]
// 0x30d3c — __ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvRKSsS9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string const&,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
pub fn list2_value_value_call_list0_int_30d3c() {
    todo!("0x30d3c __ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvRKSsS9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x30eac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn functor_manager_bind_t_list2_value_value_manager_30eac() {
    todo!("0x30eac __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")]
// 0x30fe0 — __ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
// was: boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
pub fn list2_value_value_list2_30fe0() {
    todo!("0x30fe0 __ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")]
// 0x310a8 — __ZN5boost6detail12shared_countC2IN3RBX16SecurePlayerGameEEEPT_
// was: boost::detail::shared_count::shared_count<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)
pub fn shared_count_shared_count_310a8() {
    todo!("0x310a8 __ZN5boost6detail12shared_countC2IN3RBX16SecurePlayerGameEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")]
// 0x3119c — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()
pub fn sp_counted_impl_p_drop_sp_counted_impl_p_3119c() {
    todo!("0x3119c __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED1Ev")
}
