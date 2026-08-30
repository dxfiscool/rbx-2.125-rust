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

// --- remaining batch (50) from ida/export.json: boost:: not yet stubbed, sorted by ea ---

#[doc(alias = "RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)")]
// 0x179ec — __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE
// was: RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)
pub fn stub_0x179ec() -> ! {
    todo!("0x179ec __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE")
}

#[doc(alias = "RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
// 0x179f4 — __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
// was: RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
pub fn stub_0x179f4() -> ! {
    todo!("0x179f4 __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE")
}

#[doc(alias = "joinGameWithJoinScript(std::string const&,boost::shared_ptr<RBX::Game>)")]
// 0x26990 — __ZL22joinGameWithJoinScriptRKSsN5boost10shared_ptrIN3RBX4GameEEE
// was: joinGameWithJoinScript(std::string const&,boost::shared_ptr<RBX::Game>)
pub fn stub_0x26990() -> ! {
    todo!("0x26990 __ZL22joinGameWithJoinScriptRKSsN5boost10shared_ptrIN3RBX4GameEEE")
}

#[doc(alias = "joinLocalGame(int,std::string const&,boost::shared_ptr<RBX::Game>)")]
// 0x26dd4 — __ZL13joinLocalGameiRKSsN5boost10shared_ptrIN3RBX4GameEEE
// was: joinLocalGame(int,std::string const&,boost::shared_ptr<RBX::Game>)
pub fn stub_0x26dd4() -> ! {
    todo!("0x26dd4 __ZL13joinLocalGameiRKSsN5boost10shared_ptrIN3RBX4GameEEE")
}

#[doc(alias = "loadLocalApp(std::string const&,boost::shared_ptr<RBX::Game>)")]
// 0x27268 — __ZL12loadLocalAppRKSsN5boost10shared_ptrIN3RBX4GameEEE
// was: loadLocalApp(std::string const&,boost::shared_ptr<RBX::Game>)
pub fn stub_0x27268() -> ! {
    todo!("0x27268 __ZL12loadLocalAppRKSsN5boost10shared_ptrIN3RBX4GameEEE")
}

#[doc(alias = "joinGamePlaceId(int,boost::shared_ptr<RBX::Game>,JoinGameRequest)")]
// 0x278a8 — __ZL15joinGamePlaceIdiN5boost10shared_ptrIN3RBX4GameEEE15JoinGameRequest
// was: joinGamePlaceId(int,boost::shared_ptr<RBX::Game>,JoinGameRequest)
pub fn stub_0x278a8() -> ! {
    todo!("0x278a8 __ZL15joinGamePlaceIdiN5boost10shared_ptrIN3RBX4GameEEE15JoinGameRequest")
}

#[doc(alias = "joinGamePlaceIdSolo(int,boost::shared_ptr<RBX::Game>)")]
// 0x28d98 — __ZL19joinGamePlaceIdSoloiN5boost10shared_ptrIN3RBX4GameEEE
// was: joinGamePlaceIdSolo(int,boost::shared_ptr<RBX::Game>)
pub fn stub_0x28d98() -> ! {
    todo!("0x28d98 __ZL19joinGamePlaceIdSoloiN5boost10shared_ptrIN3RBX4GameEEE")
}

#[doc(alias = "joinGameTeleport(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>)")]
// 0x2a350 — __ZL16joinGameTeleportSsSsSsP8NSObjectN5boost10shared_ptrIN3RBX4GameEEE
// was: joinGameTeleport(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>)
pub fn stub_0x2a350() -> ! {
    todo!("0x2a350 __ZL16joinGameTeleportSsSsSsP8NSObjectN5boost10shared_ptrIN3RBX4GameEEE")
}

#[doc(alias = "finishTeleport(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *)")]
// 0x2aba4 — __ZL14finishTeleportP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEEPNS3_18FunctionMarshallerE
// was: finishTeleport(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *)
pub fn stub_0x2aba4() -> ! {
    todo!("0x2aba4 __ZL14finishTeleportP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEEPNS3_18FunctionMarshallerE")
}

#[doc(alias = "finishTeleportHelper(RobloxView *,boost::shared_ptr<RBX::Game>)")]
// 0x2b754 — __ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE
// was: finishTeleportHelper(RobloxView *,boost::shared_ptr<RBX::Game>)
pub fn stub_0x2b754() -> ! {
    todo!("0x2b754 __ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE")
}

#[doc(alias = "executeUrlScript(boost::shared_ptr<RBX::DataModel>,std::string const&)")]
// 0x2ba54 — __ZL16executeUrlScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
// was: executeUrlScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
pub fn stub_0x2ba54() -> ! {
    todo!("0x2ba54 __ZL16executeUrlScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs")
}

#[doc(alias = "executeSignedScript(boost::shared_ptr<RBX::DataModel>,std::string const&)")]
// 0x2bdb0 — __ZL19executeSignedScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
// was: executeSignedScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
pub fn stub_0x2bdb0() -> ! {
    todo!("0x2bdb0 __ZL19executeSignedScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs")
}

#[doc(alias = "executeScript(boost::shared_ptr<RBX::DataModel>,std::string const&)")]
// 0x2bf74 — __ZL13executeScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
// was: executeScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
pub fn stub_0x2bf74() -> ! {
    todo!("0x2bf74 __ZL13executeScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const")]
// 0x2d67c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x2d67c() -> ! {
    todo!("0x2d67c __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x2d768 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x2d768() -> ! {
    todo!("0x2d768 __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
// 0x2f300 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIiEENSE_ISA_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x2f300() -> ! {
    todo!("0x2f300 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIiEENSE_ISA_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x2f3e8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIiEENSE_ISA_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x2f3e8() -> ! {
    todo!("0x2f3e8 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIiEENSE_ISA_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &)const")]
// 0x2f9ec — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS5_5list3INS5_5valueIiEENSF_ISA_EENSF_ISB_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x2f9ec() -> ! {
    todo!("0x2f9ec __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS5_5list3INS5_5valueIiEENSF_ISA_EENSF_ISB_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x2fad8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS5_5list3INS5_5valueIiEENSF_ISA_EENSF_ISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x2fad8() -> ! {
    todo!("0x2fad8 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS5_5list3INS5_5valueIiEENSF_ISA_EENSF_ISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
// 0x301ac — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list3INS5_5valueIiEENSG_IPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x301ac() -> ! {
    todo!("0x301ac __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list3INS5_5valueIiEENSG_IPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x30298 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list3INS5_5valueIiEENSG_IPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x30298() -> ! {
    todo!("0x30298 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list3INS5_5valueIiEENSG_IPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
// 0x30b40 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x30b40() -> ! {
    todo!("0x30b40 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x30c28 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x30c28() -> ! {
    todo!("0x30c28 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")]
// 0x311a0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED0Ev
// was: boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()
pub fn stub_0x311a0() -> ! {
    todo!("0x311a0 __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::dispose(void)")]
// 0x311a4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::dispose(void)
pub fn stub_0x311a4() -> ! {
    todo!("0x311a4 __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_deleter(std::type_info const&)")]
// 0x311b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_deleter(std::type_info const&)
pub fn stub_0x311b4() -> ! {
    todo!("0x311b4 __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_untyped_deleter(void)")]
// 0x311b8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_untyped_deleter(void)
pub fn stub_0x311b8() -> ! {
    todo!("0x311b8 __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::UnsecuredStudioGame>(RBX::UnsecuredStudioGame *)")]
// 0x311bc — __ZN5boost6detail12shared_countC2IN3RBX19UnsecuredStudioGameEEEPT_
// was: boost::detail::shared_count::shared_count<RBX::UnsecuredStudioGame>(RBX::UnsecuredStudioGame *)
pub fn stub_0x311bc() -> ! {
    todo!("0x311bc __ZN5boost6detail12shared_countC2IN3RBX19UnsecuredStudioGameEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()")]
// 0x312b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()
pub fn stub_0x312b0() -> ! {
    todo!("0x312b0 __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()")]
// 0x312b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEED0Ev
// was: boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()
pub fn stub_0x312b4() -> ! {
    todo!("0x312b4 __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::dispose(void)")]
// 0x312b8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::dispose(void)
pub fn stub_0x312b8() -> ! {
    todo!("0x312b8 __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_deleter(std::type_info const&)")]
// 0x312c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_deleter(std::type_info const&)
pub fn stub_0x312c8() -> ! {
    todo!("0x312c8 __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_untyped_deleter(void)")]
// 0x312cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_untyped_deleter(void)
pub fn stub_0x312cc() -> ! {
    todo!("0x312cc __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x312d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x312d0() -> ! {
    todo!("0x312d0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// 0x31348 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_0x31348() -> ! {
    todo!("0x31348 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_")
}

#[doc(alias = "boost::shared_ptr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)")]
// 0x31678 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LoginServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)
pub fn stub_0x31678() -> ! {
    todo!("0x31678 __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LoginServiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const&)")]
// 0x31728 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12LoginServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const&)
pub fn stub_0x31728() -> ! {
    todo!("0x31728 __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12LoginServiceEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "boost::shared_ptr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x319ec — __ZN5boost10shared_ptrIN3RBX12LoginServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x319ec() -> ! {
    todo!("0x319ec __ZN5boost10shared_ptrIN3RBX12LoginServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LoginService,RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const*,RBX::LoginService *)const")]
// 0x31a10 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LoginServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LoginService,RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const*,RBX::LoginService *)const
pub fn stub_0x31a10() -> ! {
    todo!("0x31a10 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LoginServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x31aec — __ZN5boost6detail12shared_countC2IPN3RBX12LoginServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x31aec() -> ! {
    todo!("0x31aec __ZN5boost6detail12shared_countC2IPN3RBX12LoginServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x31bec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x31bec() -> ! {
    todo!("0x31bec __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x31bf0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x31bf0() -> ! {
    todo!("0x31bf0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x31bf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_0x31bf4() -> ! {
    todo!("0x31bf4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x31c14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_0x31c14() -> ! {
    todo!("0x31c14 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x31c2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_0x31c2c() -> ! {
    todo!("0x31c2c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
// 0x31cd0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x31cd0() -> ! {
    todo!("0x31cd0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)")]
// 0x31d30 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)
pub fn stub_0x31d30() -> ! {
    todo!("0x31d30 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)")]
// 0x31d48 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)
pub fn stub_0x31d48() -> ! {
    todo!("0x31d48 __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
// 0x32270 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x32270() -> ! {
    todo!("0x32270 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
// 0x322d0 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)
pub fn stub_0x322d0() -> ! {
    todo!("0x322d0 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs")
}

#[doc(alias = "std::length_error::~length_error()")]
// 0x9b2c — __ZNSt12length_errorD1Ev
// was: std::length_error::~length_error()
pub fn stub_9b2c() -> ! {
    todo!("0x9b2c __ZNSt12length_errorD1Ev")
}

#[doc(alias = "std::out_of_range::~out_of_range()")]
// 0x9b30 — __ZNSt12out_of_rangeD0Ev
// was: std::out_of_range::~out_of_range()
pub fn stub_9b30() -> ! {
    todo!("0x9b30 __ZNSt12out_of_rangeD0Ev")
}

#[doc(alias = "std::out_of_range::~out_of_range()")]
// 0x9b44 — __ZNSt12out_of_rangeD2Ev
// was: std::out_of_range::~out_of_range()
pub fn stub_9b44() -> ! {
    todo!("0x9b44 __ZNSt12out_of_rangeD2Ev")
}

#[doc(alias = "RBX::CRenderSettings::getAutoQualityLevel(void)const")]
// 0xb474 — __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv
// was: RBX::CRenderSettings::getAutoQualityLevel(void)const
pub fn stub_b474() -> ! {
    todo!("0xb474 __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv")
}

#[doc(alias = "RBX::CRenderSettings::getEnableFRM(void)const")]
// 0xb49c — __ZNK3RBX15CRenderSettings12getEnableFRMEv
// was: RBX::CRenderSettings::getEnableFRM(void)const
pub fn stub_b49c() -> ! {
    todo!("0xb49c __ZNK3RBX15CRenderSettings12getEnableFRMEv")
}

#[doc(alias = "RBX::CRenderSettings::getResolutionPreference(void)const")]
// 0xb4a4 — __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv
// was: RBX::CRenderSettings::getResolutionPreference(void)const
pub fn stub_b4a4() -> ! {
    todo!("0xb4a4 __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv")
}

#[doc(alias = "RBX::CRenderSettings::getMaxQualityLevel(void)")]
// 0xb4cc — __ZN3RBX15CRenderSettings18getMaxQualityLevelEv
// was: RBX::CRenderSettings::getMaxQualityLevel(void)
pub fn stub_b4cc() -> ! {
    todo!("0xb4cc __ZN3RBX15CRenderSettings18getMaxQualityLevelEv")
}

#[doc(alias = "RBX::CRenderSettings::getTextureCacheSize(void)const")]
// 0xb4f4 — __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv
// was: RBX::CRenderSettings::getTextureCacheSize(void)const
pub fn stub_b4f4() -> ! {
    todo!("0xb4f4 __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv")
}

#[doc(alias = "RBX::CRenderSettings::getMeshCacheSize(void)const")]
// 0xb4f8 — __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv
// was: RBX::CRenderSettings::getMeshCacheSize(void)const
pub fn stub_b4f8() -> ! {
    todo!("0xb4f8 __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv")
}

#[doc(alias = "RBX::CRenderSettings::getEagerBulkExecution(void)const")]
// 0xb8b0 — __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv
// was: RBX::CRenderSettings::getEagerBulkExecution(void)const
pub fn stub_b8b0() -> ! {
    todo!("0xb8b0 __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv")
}

#[doc(alias = "RBX::CRenderSettings::ResolutionPreset const& rbx::any_cast<RBX::CRenderSettings::ResolutionPreset const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xcaa4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings16ResolutionPresetENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// was: RBX::CRenderSettings::ResolutionPreset const& rbx::any_cast<RBX::CRenderSettings::ResolutionPreset const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_caa4() -> ! {
    todo!("0xcaa4 __ZN3rbx8any_castIRKN3RBX15CRenderSettings16ResolutionPresetENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CRenderSettings::QualityLevel const& rbx::any_cast<RBX::CRenderSettings::QualityLevel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xd084 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12QualityLevelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// was: RBX::CRenderSettings::QualityLevel const& rbx::any_cast<RBX::CRenderSettings::QualityLevel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_d084() -> ! {
    todo!("0xd084 __ZN3rbx8any_castIRKN3RBX15CRenderSettings12QualityLevelENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CRenderSettings::ShadowMode const& rbx::any_cast<RBX::CRenderSettings::ShadowMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xd5c4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings10ShadowModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// was: RBX::CRenderSettings::ShadowMode const& rbx::any_cast<RBX::CRenderSettings::ShadowMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_d5c4() -> ! {
    todo!("0xd5c4 __ZN3rbx8any_castIRKN3RBX15CRenderSettings10ShadowModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CRenderSettings::AntialiasingMode const& rbx::any_cast<RBX::CRenderSettings::AntialiasingMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xdb04 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings16AntialiasingModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// was: RBX::CRenderSettings::AntialiasingMode const& rbx::any_cast<RBX::CRenderSettings::AntialiasingMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_db04() -> ! {
    todo!("0xdb04 __ZN3rbx8any_castIRKN3RBX15CRenderSettings16AntialiasingModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode const& rbx::any_cast<RBX::CRenderSettings::FrameRateManagerMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xe044 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings20FrameRateManagerModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// was: RBX::CRenderSettings::FrameRateManagerMode const& rbx::any_cast<RBX::CRenderSettings::FrameRateManagerMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_e044() -> ! {
    todo!("0xe044 __ZN3rbx8any_castIRKN3RBX15CRenderSettings20FrameRateManagerModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CRenderSettings::GraphicsMode const& rbx::any_cast<RBX::CRenderSettings::GraphicsMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xe584 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12GraphicsModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// was: RBX::CRenderSettings::GraphicsMode const& rbx::any_cast<RBX::CRenderSettings::GraphicsMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_e584() -> ! {
    todo!("0xe584 __ZN3rbx8any_castIRKN3RBX15CRenderSettings12GraphicsModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xeac4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings9AASamplesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// was: RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_eac4() -> ! {
    todo!("0xeac4 __ZN3rbx8any_castIRKN3RBX15CRenderSettings9AASamplesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)")]
// 0x142b8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// was: std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)
pub fn stub_142b8() -> ! {
    todo!("0x142b8 __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
// 0x14310 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)
pub fn stub_14310() -> ! {
    todo!("0x14310 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
// 0x143c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)
pub fn stub_143c4() -> ! {
    todo!("0x143c4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
// 0x1441c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)
pub fn stub_1441c() -> ! {
    todo!("0x1441c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)")]
// 0x14484 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_
// was: std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)
pub fn stub_14484() -> ! {
    todo!("0x14484 __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)")]
// 0x144b8 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_
// was: std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)
pub fn stub_144b8() -> ! {
    todo!("0x144b8 __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)")]
// 0x144e0 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// was: std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)
pub fn stub_144e0() -> ! {
    todo!("0x144e0 __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)")]
// 0x145c4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm
// was: std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)
pub fn stub_145c4() -> ! {
    todo!("0x145c4 __ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)")]
// 0x145dc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_
// was: RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)
pub fn stub_145dc() -> ! {
    todo!("0x145dc __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)")]
// 0x14618 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// was: std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)
pub fn stub_14618() -> ! {
    todo!("0x14618 __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)")]
// 0x147a8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// was: std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)
pub fn stub_147a8() -> ! {
    todo!("0x147a8 __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
// 0x14800 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)
pub fn stub_14800() -> ! {
    todo!("0x14800 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
// 0x148b4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)
pub fn stub_148b4() -> ! {
    todo!("0x148b4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
// 0x1490c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)
pub fn stub_1490c() -> ! {
    todo!("0x1490c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)")]
// 0x14974 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_
// was: std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)
pub fn stub_14974() -> ! {
    todo!("0x14974 __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)")]
// 0x149a8 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_
// was: std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)
pub fn stub_149a8() -> ! {
    todo!("0x149a8 __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)")]
// 0x149d0 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// was: std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)
pub fn stub_149d0() -> ! {
    todo!("0x149d0 __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)")]
// 0x14ab4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm
// was: std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)
pub fn stub_14ab4() -> ! {
    todo!("0x14ab4 __ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)")]
// 0x14acc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_
// was: RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)
pub fn stub_14acc() -> ! {
    todo!("0x14acc __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)")]
// 0x14b08 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// was: std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)
pub fn stub_14b08() -> ! {
    todo!("0x14b08 __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)")]
// 0x14c98 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_
// was: std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)
pub fn stub_14c98() -> ! {
    todo!("0x14c98 __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)")]
// 0x14ccc — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_
// was: std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)
pub fn stub_14ccc() -> ! {
    todo!("0x14ccc __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)")]
// 0x14cf4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// was: std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)
pub fn stub_14cf4() -> ! {
    todo!("0x14cf4 __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
// 0x14d4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)
pub fn stub_14d4c() -> ! {
    todo!("0x14d4c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
// 0x14e00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)
pub fn stub_14e00() -> ! {
    todo!("0x14e00 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
// 0x14e58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)
pub fn stub_14e58() -> ! {
    todo!("0x14e58 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,RBX::CRenderSettings::ShadowMode const&)")]
// 0x14ec0 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// was: std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,RBX::CRenderSettings::ShadowMode const&)
pub fn stub_14ec0() -> ! {
    todo!("0x14ec0 __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_allocate(unsigned long)")]
// 0x14fa4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings10ShadowModeESaIS2_EE11_M_allocateEm
// was: std::_Vector_base<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_allocate(unsigned long)
pub fn stub_14fa4() -> ! {
    todo!("0x14fa4 __ZNSt12_Vector_baseIN3RBX15CRenderSettings10ShadowModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::CRenderSettings::ShadowMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *>(RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *)")]
// 0x14fbc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings10ShadowModeES6_EET0_T_S8_S7_
// was: RBX::CRenderSettings::ShadowMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *>(RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *)
pub fn stub_14fbc() -> ! {
    todo!("0x14fbc __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings10ShadowModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,unsigned long,RBX::CRenderSettings::ShadowMode const&)")]
// 0x14ff8 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// was: std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,unsigned long,RBX::CRenderSettings::ShadowMode const&)
pub fn stub_14ff8() -> ! {
    todo!("0x14ff8 __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::resize(unsigned long,RBX::CRenderSettings::AntialiasingMode)")]
// 0x15188 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE6resizeEmS2_
// was: std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::resize(unsigned long,RBX::CRenderSettings::AntialiasingMode)
pub fn stub_15188() -> ! {
    todo!("0x15188 __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::push_back(RBX::CRenderSettings::AntialiasingMode const&)")]
// 0x151bc — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE9push_backERKS2_
// was: std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::push_back(RBX::CRenderSettings::AntialiasingMode const&)
pub fn stub_151bc() -> ! {
    todo!("0x151bc __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AntialiasingMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::operator[](RBX::Name const* const&)")]
// 0x151e4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16AntialiasingModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// was: std::map<RBX::Name const*,RBX::CRenderSettings::AntialiasingMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::operator[](RBX::Name const* const&)
pub fn stub_151e4() -> ! {
    todo!("0x151e4 __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16AntialiasingModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
// 0x1523c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)
pub fn stub_1523c() -> ! {
    todo!("0x1523c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
// 0x152f0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)
pub fn stub_152f0() -> ! {
    todo!("0x152f0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
// 0x15348 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)
pub fn stub_15348() -> ! {
    todo!("0x15348 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,RBX::CRenderSettings::AntialiasingMode const&)")]
// 0x153b0 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// was: std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,RBX::CRenderSettings::AntialiasingMode const&)
pub fn stub_153b0() -> ! {
    todo!("0x153b0 __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_allocate(unsigned long)")]
// 0x15494 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE11_M_allocateEm
// was: std::_Vector_base<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_allocate(unsigned long)
pub fn stub_15494() -> ! {
    todo!("0x15494 __ZNSt12_Vector_baseIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::CRenderSettings::AntialiasingMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *>(RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *)")]
// 0x154ac — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16AntialiasingModeES6_EET0_T_S8_S7_
// was: RBX::CRenderSettings::AntialiasingMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *>(RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *)
pub fn stub_154ac() -> ! {
    todo!("0x154ac __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16AntialiasingModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,unsigned long,RBX::CRenderSettings::AntialiasingMode const&)")]
// 0x154e8 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// was: std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,unsigned long,RBX::CRenderSettings::AntialiasingMode const&)
pub fn stub_154e8() -> ! {
    todo!("0x154e8 __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::resize(unsigned long,RBX::CRenderSettings::FrameRateManagerMode)")]
// 0x15678 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE6resizeEmS2_
// was: std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::resize(unsigned long,RBX::CRenderSettings::FrameRateManagerMode)
pub fn stub_15678() -> ! {
    todo!("0x15678 __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::push_back(RBX::CRenderSettings::FrameRateManagerMode const&)")]
// 0x156ac — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE9push_backERKS2_
// was: std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::push_back(RBX::CRenderSettings::FrameRateManagerMode const&)
pub fn stub_156ac() -> ! {
    todo!("0x156ac __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::FrameRateManagerMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::operator[](RBX::Name const* const&)")]
// 0x156d4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings20FrameRateManagerModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// was: std::map<RBX::Name const*,RBX::CRenderSettings::FrameRateManagerMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::operator[](RBX::Name const* const&)
pub fn stub_156d4() -> ! {
    todo!("0x156d4 __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings20FrameRateManagerModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
// 0x1572c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)
pub fn stub_1572c() -> ! {
    todo!("0x1572c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
// 0x157e0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)
pub fn stub_157e0() -> ! {
    todo!("0x157e0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
// 0x15838 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)
pub fn stub_15838() -> ! {
    todo!("0x15838 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,RBX::CRenderSettings::FrameRateManagerMode const&)")]
// 0x158a0 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// was: std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,RBX::CRenderSettings::FrameRateManagerMode const&)
pub fn stub_158a0() -> ! {
    todo!("0x158a0 __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_allocate(unsigned long)")]
// 0x15984 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE11_M_allocateEm
// was: std::_Vector_base<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_allocate(unsigned long)
pub fn stub_15984() -> ! {
    todo!("0x15984 __ZNSt12_Vector_baseIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *>(RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *)")]
// 0x1599c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings20FrameRateManagerModeES6_EET0_T_S8_S7_
// was: RBX::CRenderSettings::FrameRateManagerMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *>(RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *)
pub fn stub_1599c() -> ! {
    todo!("0x1599c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings20FrameRateManagerModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,unsigned long,RBX::CRenderSettings::FrameRateManagerMode const&)")]
// 0x159d8 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// was: std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,unsigned long,RBX::CRenderSettings::FrameRateManagerMode const&)
pub fn stub_159d8() -> ! {
    todo!("0x159d8 __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::resize(unsigned long,RBX::CRenderSettings::GraphicsMode)")]
// 0x15b68 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE6resizeEmS2_
// was: std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::resize(unsigned long,RBX::CRenderSettings::GraphicsMode)
pub fn stub_15b68() -> ! {
    todo!("0x15b68 __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::GraphicsMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::operator[](RBX::Name const* const&)")]
// 0x15b9c — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12GraphicsModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// was: std::map<RBX::Name const*,RBX::CRenderSettings::GraphicsMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::operator[](RBX::Name const* const&)
pub fn stub_15b9c() -> ! {
    todo!("0x15b9c __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12GraphicsModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
// 0x15bf4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)
pub fn stub_15bf4() -> ! {
    todo!("0x15bf4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
// 0x15ca8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)
pub fn stub_15ca8() -> ! {
    todo!("0x15ca8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
// 0x15d00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)
pub fn stub_15d00() -> ! {
    todo!("0x15d00 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,unsigned long,RBX::CRenderSettings::GraphicsMode const&)")]
// 0x15d68 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// was: std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,unsigned long,RBX::CRenderSettings::GraphicsMode const&)
pub fn stub_15d68() -> ! {
    todo!("0x15d68 __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_allocate(unsigned long)")]
// 0x15ef8 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm
// was: std::_Vector_base<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_allocate(unsigned long)
pub fn stub_15ef8() -> ! {
    todo!("0x15ef8 __ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::CRenderSettings::GraphicsMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *>(RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *)")]
// 0x15f10 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_
// was: RBX::CRenderSettings::GraphicsMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *>(RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *)
pub fn stub_15f10() -> ! {
    todo!("0x15f10 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::push_back(RBX::CRenderSettings::GraphicsMode const&)")]
// 0x15f4c — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_
// was: std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::push_back(RBX::CRenderSettings::GraphicsMode const&)
pub fn stub_15f4c() -> ! {
    todo!("0x15f4c __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,RBX::CRenderSettings::GraphicsMode const&)")]
// 0x15f74 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// was: std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,RBX::CRenderSettings::GraphicsMode const&)
pub fn stub_15f74() -> ! {
    todo!("0x15f74 __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::resize(unsigned long,RBX::CRenderSettings::AASamples)")]
// 0x16058 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_
// was: std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::resize(unsigned long,RBX::CRenderSettings::AASamples)
pub fn stub_16058() -> ! {
    todo!("0x16058 __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::push_back(RBX::CRenderSettings::AASamples const&)")]
// 0x1608c — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_
// was: std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::push_back(RBX::CRenderSettings::AASamples const&)
pub fn stub_1608c() -> ! {
    todo!("0x1608c __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AASamples,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::operator[](RBX::Name const* const&)")]
// 0x160b4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// was: std::map<RBX::Name const*,RBX::CRenderSettings::AASamples,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::operator[](RBX::Name const* const&)
pub fn stub_160b4() -> ! {
    todo!("0x160b4 __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
// 0x1610c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)
pub fn stub_1610c() -> ! {
    todo!("0x1610c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
// 0x161c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)
pub fn stub_161c0() -> ! {
    todo!("0x161c0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
// 0x16218 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)
pub fn stub_16218() -> ! {
    todo!("0x16218 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,RBX::CRenderSettings::AASamples const&)")]
// 0x16280 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// was: std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,RBX::CRenderSettings::AASamples const&)
pub fn stub_16280() -> ! {
    todo!("0x16280 __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_allocate(unsigned long)")]
// 0x16364 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm
// was: std::_Vector_base<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_allocate(unsigned long)
pub fn stub_16364() -> ! {
    todo!("0x16364 __ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::CRenderSettings::AASamples * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *>(RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *)")]
// 0x1637c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_
// was: RBX::CRenderSettings::AASamples * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *>(RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *)
pub fn stub_1637c() -> ! {
    todo!("0x1637c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,unsigned long,RBX::CRenderSettings::AASamples const&)")]
// 0x163b8 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// was: std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,unsigned long,RBX::CRenderSettings::AASamples const&)
pub fn stub_163b8() -> ! {
    todo!("0x163b8 __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>> *)")]
// 0x16d34 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>> *)
pub fn stub_16d34() -> ! {
    todo!("0x16d34 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>> *)")]
// 0x16d5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>> *)
pub fn stub_16d5c() -> ! {
    todo!("0x16d5c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>> *)")]
// 0x16d84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>> *)
pub fn stub_16d84() -> ! {
    todo!("0x16d84 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>> *)")]
// 0x16dac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>> *)
pub fn stub_16dac() -> ! {
    todo!("0x16dac __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>> *)")]
// 0x16dd4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>> *)
pub fn stub_16dd4() -> ! {
    todo!("0x16dd4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>> *)")]
// 0x16dfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>> *)
pub fn stub_16dfc() -> ! {
    todo!("0x16dfc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>> *)")]
// 0x16e24 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>> *)
pub fn stub_16e24() -> ! {
    todo!("0x16e24 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::map<std::string,void (*)(char const*),std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::operator[](std::string const&)")]
// 0x23a04 — __ZNSt3mapISsPFvPKcESt4lessISsESaISt4pairIKSsS3_EEEixERS7_
// was: std::map<std::string,void (*)(char const*),std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::operator[](std::string const&)
pub fn stub_23a04() -> ! {
    todo!("0x23a04 __ZNSt3mapISsPFvPKcESt4lessISsESaISt4pairIKSsS3_EEEixERS7_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,void (*)(char const*)>>,std::pair<std::string const,void (*)(char const*)> const&)")]
// 0x24274 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// was: std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,void (*)(char const*)>>,std::pair<std::string const,void (*)(char const*)> const&)
pub fn stub_24274() -> ! {
    todo!("0x24274 __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,void (*)(char const*)> const&)")]
// 0x24360 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_
// was: std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,void (*)(char const*)> const&)
pub fn stub_24360() -> ! {
    todo!("0x24360 __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::pair<std::string const,void (*)(char const*)> const&)")]
// 0x243b0 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueERKS6_
// was: std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::pair<std::string const,void (*)(char const*)> const&)
pub fn stub_243b0() -> ! {
    todo!("0x243b0 __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueERKS6_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_create_node(std::pair<std::string const,void (*)(char const*)> const&)")]
// 0x24434 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE14_M_create_nodeERKS6_
// was: std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_create_node(std::pair<std::string const,void (*)(char const*)> const&)
pub fn stub_24434() -> ! {
    todo!("0x24434 __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE14_M_create_nodeERKS6_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::lower_bound(std::string const&)")]
// 0x24510 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE11lower_boundERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::lower_bound(std::string const&)
pub fn stub_24510() -> ! {
    todo!("0x24510 __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE11lower_boundERS1_")
}
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ViewBase>(RBX::ViewBase *)")]
// 0x3db4c — __ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_
// was: boost::detail::shared_count::shared_count<RBX::ViewBase>(RBX::ViewBase *)
pub fn stub_3db4c() -> ! {
    todo!("0x3db4c __ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::~sp_counted_impl_p()")]
// 0x3dc40 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::ViewBase>::~sp_counted_impl_p()
pub fn stub_3dc40() -> ! {
    todo!("0x3dc40 __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::~sp_counted_impl_p()")]
// 0x3dc44 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED0Ev
// was: boost::detail::sp_counted_impl_p<RBX::ViewBase>::~sp_counted_impl_p()
pub fn stub_3dc44() -> ! {
    todo!("0x3dc44 __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::dispose(void)")]
// 0x3dc48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::ViewBase>::dispose(void)
pub fn stub_3dc48() -> ! {
    todo!("0x3dc48 __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_deleter(std::type_info const&)")]
// 0x3dc58 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_deleter(std::type_info const&)
pub fn stub_3dc58() -> ! {
    todo!("0x3dc58 __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_untyped_deleter(void)")]
// 0x3dc5c — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_untyped_deleter(void)
pub fn stub_3dc5c() -> ! {
    todo!("0x3dc5c __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(boost::shared_ptr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const")]
// 0x3dc60 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(boost::shared_ptr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const
pub fn stub_3dc60() -> ! {
    todo!("0x3dc60 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
// 0x3dd34 — __ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_
// was: boost::detail::shared_count::shared_count<RobloxView::RenderJob>(RobloxView::RenderJob *)
pub fn stub_3dd34() -> ! {
    todo!("0x3dd34 __ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
// 0x3de28 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED1Ev
// was: boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()
pub fn stub_3de28() -> ! {
    todo!("0x3de28 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
// 0x3de2c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED0Ev
// was: boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()
pub fn stub_3de2c() -> ! {
    todo!("0x3de2c __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::dispose(void)")]
// 0x3de30 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::dispose(void)
pub fn stub_3de30() -> ! {
    todo!("0x3de30 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_deleter(std::type_info const&)")]
// 0x3de40 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_deleter(std::type_info const&)
pub fn stub_3de40() -> ! {
    todo!("0x3de40 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_untyped_deleter(void)")]
// 0x3de44 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_untyped_deleter(void)
pub fn stub_3de44() -> ! {
    todo!("0x3de44 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(boost::shared_ptr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const")]
// 0x3de48 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(boost::shared_ptr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const
pub fn stub_3de48() -> ! {
    todo!("0x3de48 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
// 0x3df1c — __ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_
// was: boost::detail::shared_count::shared_count<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)
pub fn stub_3df1c() -> ! {
    todo!("0x3df1c __ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()")]
// 0x3e010 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED1Ev
// was: boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()
pub fn stub_3e010() -> ! {
    todo!("0x3e010 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()")]
// 0x3e014 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED0Ev
// was: boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()
pub fn stub_3e014() -> ! {
    todo!("0x3e014 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::dispose(void)")]
// 0x3e018 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::dispose(void)
pub fn stub_3e018() -> ! {
    todo!("0x3e018 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_deleter(std::type_info const&)")]
// 0x3e028 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_deleter(std::type_info const&)
pub fn stub_3e028() -> ! {
    todo!("0x3e028 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_untyped_deleter(void)")]
// 0x3e02c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_untyped_deleter(void)
pub fn stub_3e02c() -> ! {
    todo!("0x3e02c __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x3e030 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_3e030() -> ! {
    todo!("0x3e030 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x3e090 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_3e090() -> ! {
    todo!("0x3e090 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>::operator()(void)")]
// 0x3e094 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// was: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>::operator()(void)
pub fn stub_3e094() -> ! {
    todo!("0x3e094 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")
}

#[doc(alias = "boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x3e198 — __ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// was: boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_3e198() -> ! {
    todo!("0x3e198 __ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "boost::singleton_pool<XmlElement,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x3e238 — __ZN5boost14singleton_poolI10XmlElementLj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// was: boost::singleton_pool<XmlElement,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_3e238() -> ! {
    todo!("0x3e238 __ZN5boost14singleton_poolI10XmlElementLj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "boost::function0<void>::assign_to_own(boost::function0<void> const&)")]
// 0x3e288 — __ZN5boost9function0IvE13assign_to_ownERKS1_
// was: boost::function0<void>::assign_to_own(boost::function0<void> const&)
pub fn stub_3e288() -> ! {
    todo!("0x3e288 __ZN5boost9function0IvE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::exception_detail::bad_exception_::~bad_exception_()")]
// 0x3e2b8 — __ZN5boost16exception_detail14bad_exception_D1Ev
// was: boost::exception_detail::bad_exception_::~bad_exception_()
pub fn stub_3e2b8() -> ! {
    todo!("0x3e2b8 __ZN5boost16exception_detail14bad_exception_D1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone(void)const")]
// 0x3e2e8 — __ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone(void)const
pub fn stub_3e2e8() -> ! {
    todo!("0x3e2e8 __ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_> const&,boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone_tag)")]
// 0x3e3a8 — __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EEC1ERKS3_NS3_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_> const&,boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone_tag)
pub fn stub_3e3a8() -> ! {
    todo!("0x3e3a8 __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EEC1ERKS3_NS3_9clone_tagE")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::bad_exception_::~bad_exception_()")]
// 0x3e528 — __ZThn20_N5boost16exception_detail14bad_exception_D0Ev
// was: `non-virtual thunk to'boost::exception_detail::bad_exception_::~bad_exception_()
pub fn stub_3e528() -> ! {
    todo!("0x3e528 __ZThn20_N5boost16exception_detail14bad_exception_D0Ev")
}

#[doc(alias = "boost::shared_ptr<boost::exception_detail::clone_base const>::shared_ptr<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_> *)")]
// 0x3e558 — __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_14bad_exception_EEEEEPT_
// was: boost::shared_ptr<boost::exception_detail::clone_base const>::shared_ptr<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_> *)
pub fn stub_3e558() -> ! {
    todo!("0x3e558 __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_14bad_exception_EEEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>::~sp_counted_impl_p()")]
// 0x3e640 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEED1Ev
// was: boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>::~sp_counted_impl_p()
pub fn stub_3e640() -> ! {
    todo!("0x3e640 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_impl(boost::exception_detail::bad_alloc_ const&)")]
// 0x3e648 — __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS2_
// was: boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_impl(boost::exception_detail::bad_alloc_ const&)
pub fn stub_3e648() -> ! {
    todo!("0x3e648 __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS2_")
}

#[doc(alias = "boost::exception_detail::bad_alloc_::~bad_alloc_()")]
// 0x3e7c8 — __ZN5boost16exception_detail10bad_alloc_D1Ev
// was: boost::exception_detail::bad_alloc_::~bad_alloc_()
pub fn stub_3e7c8() -> ! {
    todo!("0x3e7c8 __ZN5boost16exception_detail10bad_alloc_D1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone(void)const")]
// 0x3e7f8 — __ZNK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone(void)const
pub fn stub_3e7f8() -> ! {
    todo!("0x3e7f8 __ZNK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::rethrow(void)const")]
// 0x3e8b8 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_10bad_alloc_EE7rethrowEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::rethrow(void)const
pub fn stub_3e8b8() -> ! {
    todo!("0x3e8b8 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_10bad_alloc_EE7rethrowEv")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()")]
// 0x3e8c8 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED0Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()
pub fn stub_3e8c8() -> ! {
    todo!("0x3e8c8 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> const&,boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_tag)")]
// 0x3e900 — __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS3_NS3_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> const&,boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_tag)
pub fn stub_3e900() -> ! {
    todo!("0x3e900 __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS3_NS3_9clone_tagE")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::bad_alloc_::~bad_alloc_()")]
// 0x3ea80 — __ZThn20_N5boost16exception_detail10bad_alloc_D0Ev
// was: `non-virtual thunk to'boost::exception_detail::bad_alloc_::~bad_alloc_()
pub fn stub_3ea80() -> ! {
    todo!("0x3ea80 __ZThn20_N5boost16exception_detail10bad_alloc_D0Ev")
}

#[doc(alias = "boost::shared_ptr<boost::exception_detail::clone_base const>::shared_ptr<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> *)")]
// 0x3eab0 — __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_10bad_alloc_EEEEEPT_
// was: boost::shared_ptr<boost::exception_detail::clone_base const>::shared_ptr<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> *)
pub fn stub_3eab0() -> ! {
    todo!("0x3eab0 __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_10bad_alloc_EEEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::dispose(void)")]
// 0x3eb98 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE7disposeEv
// was: boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::dispose(void)
pub fn stub_3eb98() -> ! {
    todo!("0x3eb98 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::get_deleter(std::type_info const&)")]
// 0x3eba8 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::get_deleter(std::type_info const&)
pub fn stub_3eba8() -> ! {
    todo!("0x3eba8 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE11get_deleterERKSt9type_info")
}

#[doc(alias = "void boost::intrusive_ptr_weak_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
// 0x3ebb8 — __ZN5boost26intrusive_ptr_weak_releaseIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// was: void boost::intrusive_ptr_weak_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)
pub fn stub_3ebb8() -> ! {
    todo!("0x3ebb8 __ZN5boost26intrusive_ptr_weak_releaseIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE")
}

#[doc(alias = "RobloxView::RenderJob::getMetricValue(std::string const&)const")]
// 0x3f598 — __ZNK10RobloxView9RenderJob14getMetricValueERKSs
// was: RobloxView::RenderJob::getMetricValue(std::string const&)const
pub fn stub_3f598() -> ! {
    todo!("0x3f598 __ZNK10RobloxView9RenderJob14getMetricValueERKSs")
}

#[doc(alias = "RobloxView::RenderJob::getMetric(std::string const&)const")]
// 0x3f700 — __ZNK10RobloxView9RenderJob9getMetricERKSs
// was: RobloxView::RenderJob::getMetric(std::string const&)const
pub fn stub_3f700() -> ! {
    todo!("0x3f700 __ZNK10RobloxView9RenderJob9getMetricERKSs")
}

#[doc(alias = "non_virtual_thunk_to RobloxView::RenderJob::getMetric(std::string const&)const")]
// 0x3fa94 — __ZThn480_NK10RobloxView9RenderJob9getMetricERKSs
// was: `non-virtual thunk to'RobloxView::RenderJob::getMetric(std::string const&)const
pub fn stub_3fa94() -> ! {
    todo!("0x3fa94 __ZThn480_NK10RobloxView9RenderJob9getMetricERKSs")
}

#[doc(alias = "non_virtual_thunk_to RobloxView::RenderJob::getMetricValue(std::string const&)const")]
// 0x3faa4 — __ZThn480_NK10RobloxView9RenderJob14getMetricValueERKSs
// was: `non-virtual thunk to'RobloxView::RenderJob::getMetricValue(std::string const&)const
pub fn stub_3faa4() -> ! {
    todo!("0x3faa4 __ZThn480_NK10RobloxView9RenderJob14getMetricValueERKSs")
}

#[doc(alias = "boost::bad_weak_ptr::~bad_weak_ptr()")]
// 0x3fcf8 — __ZN5boost12bad_weak_ptrD0Ev
// was: boost::bad_weak_ptr::~bad_weak_ptr()
pub fn stub_3fcf8() -> ! {
    todo!("0x3fcf8 __ZN5boost12bad_weak_ptrD0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
// 0x3fd10 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()
pub fn stub_3fd10() -> ! {
    todo!("0x3fd10 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
// 0x3fd38 — __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev
// was: boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()
pub fn stub_3fd38() -> ! {
    todo!("0x3fd38 __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
// 0x3fd60 — __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()
pub fn stub_3fd60() -> ! {
    todo!("0x3fd60 __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
// 0x3fd88 — __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()
pub fn stub_3fd88() -> ! {
    todo!("0x3fd88 __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const")]
// 0x3fdb8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const
pub fn stub_3fdb8() -> ! {
    todo!("0x3fdb8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
// 0x3fee0 — __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()
pub fn stub_3fee0() -> ! {
    todo!("0x3fee0 __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const")]
// 0x3ff18 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const
pub fn stub_3ff18() -> ! {
    todo!("0x3ff18 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
// 0x3ff28 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()
pub fn stub_3ff28() -> ! {
    todo!("0x3ff28 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
// 0x3ff60 — __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev
// was: boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()
pub fn stub_3ff60() -> ! {
    todo!("0x3ff60 __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
// 0x3ff90 — __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()
pub fn stub_3ff90() -> ! {
    todo!("0x3ff90 __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_tag)")]
// 0x3ffc0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEEC1ERKS5_NS5_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_tag)
pub fn stub_3ffc0() -> ! {
    todo!("0x3ffc0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x40160 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_40160() -> ! {
    todo!("0x40160 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x401dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_401dc() -> ! {
    todo!("0x401dc __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x401f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_401f0() -> ! {
    todo!("0x401f0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x40270 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_40270() -> ! {
    todo!("0x40270 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double> &,boost::_bi::list0 &,int)")]
// 0x4027c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double> &,boost::_bi::list0 &,int)
pub fn stub_4027c() -> ! {
    todo!("0x4027c __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x402a8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_402a8() -> ! {
    todo!("0x402a8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x40308 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_40308() -> ! {
    todo!("0x40308 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "boost::function0<void>::clear(void)")]
// 0x406e0 — __ZN5boost9function0IvE5clearEv
// was: boost::function0<void>::clear(void)
pub fn stub_406e0() -> ! {
    todo!("0x406e0 __ZN5boost9function0IvE5clearEv")
}

#[doc(alias = "SimpleJSON::DefaultHandler(std::string const&,std::string const&)")]
// 0x43360 — __ZN10SimpleJSON14DefaultHandlerERKSsS1_
// was: SimpleJSON::DefaultHandler(std::string const&,std::string const&)
pub fn stub_43360() -> ! {
    todo!("0x43360 __ZN10SimpleJSON14DefaultHandlerERKSsS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,void (*)(char const*)>> *)")]
// 0x43364 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,void (*)(char const*)>> *)
pub fn stub_43364() -> ! {
    todo!("0x43364 __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")
}

#[doc(alias = "RBX::FunctionMarshaller::Execute(boost::function<void ()(void)>,RBX::CEvent *)")]
// 0x43a98 — __ZN3RBX18FunctionMarshaller7ExecuteEN5boost8functionIFvvEEEPNS_6CEventE
// was: RBX::FunctionMarshaller::Execute(boost::function<void ()(void)>,RBX::CEvent *)
pub fn stub_43a98() -> ! {
    todo!("0x43a98 __ZN3RBX18FunctionMarshaller7ExecuteEN5boost8functionIFvvEEEPNS_6CEventE")
}

#[doc(alias = "RBX::FunctionMarshaller::Submit(boost::function<void ()(void)>)")]
// 0x43b98 — __ZN3RBX18FunctionMarshaller6SubmitEN5boost8functionIFvvEEE
// was: RBX::FunctionMarshaller::Submit(boost::function<void ()(void)>)
pub fn stub_43b98() -> ! {
    todo!("0x43b98 __ZN3RBX18FunctionMarshaller6SubmitEN5boost8functionIFvvEEE")
}

#[doc(alias = "std::map<unsigned int,RBX::FunctionMarshaller *,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::operator[](unsigned int const&)")]
// 0x43d14 — __ZNSt3mapIjPN3RBX18FunctionMarshallerESt4lessIjESaISt4pairIKjS2_EEEixERS6_
// was: std::map<unsigned int,RBX::FunctionMarshaller *,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::operator[](unsigned int const&)
pub fn stub_43d14() -> ! {
    todo!("0x43d14 __ZNSt3mapIjPN3RBX18FunctionMarshallerESt4lessIjESaISt4pairIKjS2_EEEixERS6_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(unsigned int const&)")]
// 0x43d6c — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseERS1_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(unsigned int const&)
pub fn stub_43d6c() -> ! {
    todo!("0x43d6c __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseERS1_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::equal_range(unsigned int const&)")]
// 0x43d94 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE11equal_rangeERS1_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::equal_range(unsigned int const&)
pub fn stub_43d94() -> ! {
    todo!("0x43d94 __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE11equal_rangeERS1_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>)")]
// 0x43de0 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>)
pub fn stub_43de0() -> ! {
    todo!("0x43de0 __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,RBX::FunctionMarshaller *>> *)")]
// 0x43e40 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,RBX::FunctionMarshaller *>> *)
pub fn stub_43e40() -> ! {
    todo!("0x43e40 __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
// 0x43e68 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)
pub fn stub_43e68() -> ! {
    todo!("0x43e68 __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
// 0x43f1c — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)
pub fn stub_43f1c() -> ! {
    todo!("0x43f1c __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
// 0x43f74 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)
pub fn stub_43f74() -> ! {
    todo!("0x43f74 __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "boost::unique_lock<boost::recursive_mutex>::lock(void)")]
// 0x43fdc — __ZN5boost11unique_lockINS_15recursive_mutexEE4lockEv
// was: boost::unique_lock<boost::recursive_mutex>::lock(void)
pub fn stub_43fdc() -> ! {
    todo!("0x43fdc __ZN5boost11unique_lockINS_15recursive_mutexEE4lockEv")
}

#[doc(alias = "boost::recursive_mutex::recursive_mutex(void)")]
// 0x442bc — __ZN5boost15recursive_mutexC2Ev
// was: boost::recursive_mutex::recursive_mutex(void)
pub fn stub_442bc() -> ! {
    todo!("0x442bc __ZN5boost15recursive_mutexC2Ev")
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::~_Deque_base()")]
// 0x44564 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EED2Ev
// was: std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::~_Deque_base()
pub fn stub_44564() -> ! {
    todo!("0x44564 __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EED2Ev")
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_initialize_map(unsigned long)")]
// 0x44590 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE17_M_initialize_mapEm
// was: std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_initialize_map(unsigned long)
pub fn stub_44590() -> ! {
    todo!("0x44590 __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_allocate_map(unsigned long)")]
// 0x446e8 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_allocate_mapEm
// was: std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_allocate_map(unsigned long)
pub fn stub_446e8() -> ! {
    todo!("0x446e8 __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_allocate_mapEm")
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_create_nodes(boost::function<void ()(void)> ***,boost::function<void ()(void)> ***)")]
// 0x44700 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_create_nodesEPPS4_S8_
// was: std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_create_nodes(boost::function<void ()(void)> ***,boost::function<void ()(void)> ***)
pub fn stub_44700() -> ! {
    todo!("0x44700 __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_create_nodesEPPS4_S8_")
}

#[doc(alias = "std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::deque(std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>> const&)")]
// 0x447f4 — __ZNSt5dequeIPN5boost8functionIFvvEEESaIS4_EEC2ERKS6_
// was: std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::deque(std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>> const&)
pub fn stub_447f4() -> ! {
    todo!("0x447f4 __ZNSt5dequeIPN5boost8functionIFvvEEESaIS4_EEC2ERKS6_")
}

#[doc(alias = "std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>>(std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>)")]
// 0x44888 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN5boost8functionIFvvEEERKS8_PS9_ES3_IS8_RS8_PS8_EEET0_T_SH_SG_
// was: std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>>(std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>)
pub fn stub_44888() -> ! {
    todo!("0x44888 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN5boost8functionIFvvEEERKS8_PS9_ES3_IS8_RS8_PS8_EEET0_T_SH_SG_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)")]
// 0x4546c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)
pub fn stub_4546c() -> ! {
    todo!("0x4546c __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::insert(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
// 0x45554 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6insertEPNS6_4slotE
// was: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::insert(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)
pub fn stub_45554() -> ! {
    todo!("0x45554 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6insertEPNS6_4slotE")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)")]
// 0x45764 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)
pub fn stub_45764() -> ! {
    todo!("0x45764 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)")]
// 0x45808 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)
pub fn stub_45808() -> ! {
    todo!("0x45808 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_do_get_mutex(void)")]
// 0x458ac — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_do_get_mutex(void)
pub fn stub_458ac() -> ! {
    todo!("0x458ac __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)")]
// 0x459a4 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
// was: rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)
pub fn stub_459a4() -> ! {
    todo!("0x459a4 __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
// 0x45aa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev
// was: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()
pub fn stub_45aa0() -> ! {
    todo!("0x45aa0 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
// 0x45b74 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED0Ev
// was: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()
pub fn stub_45b74() -> ! {
    todo!("0x45b74 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::disconnect(void)")]
// 0x45c4c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot10disconnectEv
// was: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::disconnect(void)
pub fn stub_45c4c() -> ! {
    todo!("0x45c4c __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::connected(void)const")]
// 0x45d5c — __ZNK3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot9connectedEv
// was: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::connected(void)const
pub fn stub_45d5c() -> ! {
    todo!("0x45d5c __ZNK3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
// 0x45d68 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
// was: rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)
pub fn stub_45d68() -> ! {
    todo!("0x45d68 __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_")
}

#[doc(alias = "non_virtual_thunk_to rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
// 0x45d98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)
pub fn stub_45d98() -> ! {
    todo!("0x45d98 __ZThn4_N3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_")
}

#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::operator()(bool,void *,RBX::UIEvent)const")]
// 0x45dc8 — __ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_
// was: boost::function3<void,bool,void *,RBX::UIEvent>::operator()(bool,void *,RBX::UIEvent)const
pub fn stub_45dc8() -> ! {
    todo!("0x45dc8 __ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::remove(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
// 0x45eb0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6removeEPNS6_4slotE
// was: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::remove(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)
pub fn stub_45eb0() -> ! {
    todo!("0x45eb0 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_init_mutex(void)")]
// 0x45fa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_init_mutex(void)
pub fn stub_45fa0() -> ! {
    todo!("0x45fa0 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_do_get_mutex(void)")]
// 0x45fa4 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_do_get_mutex(void)
pub fn stub_45fa4() -> ! {
    todo!("0x45fa4 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
// 0x46094 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()
pub fn stub_46094() -> ! {
    todo!("0x46094 __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
// 0x46168 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()
pub fn stub_46168() -> ! {
    todo!("0x46168 __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
// 0x46240 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD1Ev
// was: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()
pub fn stub_46240() -> ! {
    todo!("0x46240 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
// 0x462ec — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD0Ev
// was: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()
pub fn stub_462ec() -> ! {
    todo!("0x462ec __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD0Ev")
}

#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::assign_to_own(boost::function3<void,bool,void *,RBX::UIEvent> const&)")]
// 0x4639c — __ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_
// was: boost::function3<void,bool,void *,RBX::UIEvent>::assign_to_own(boost::function3<void,bool,void *,RBX::UIEvent> const&)
pub fn stub_4639c() -> ! {
    todo!("0x4639c __ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>&,boost::detail::function::functor_manager_operation_type)")]
// 0x463cc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>&,boost::detail::function::functor_manager_operation_type)
pub fn stub_463cc() -> ! {
    todo!("0x463cc __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>,void,bool,objc_selector *,RBX>::invoke(boost::detail::function::function_buffer &,bool,objc_selector *,RBX)")]
// 0x4642c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEvbS8_SA_E6invokeERNS1_15function_bufferEbS8_SA_
// was: boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>,void,bool,objc_selector *,RBX>::invoke(boost::detail::function::function_buffer &,bool,objc_selector *,RBX)
pub fn stub_4642c() -> ! {
    todo!("0x4642c __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEvbS8_SA_E6invokeERNS1_15function_bufferEbS8_SA_")
}

#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::clear(void)")]
// 0x46464 — __ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv
// was: boost::function3<void,bool,void *,RBX::UIEvent>::clear(void)
pub fn stub_46464() -> ! {
    todo!("0x46464 __ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")]
// 0x46c18 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)
pub fn stub_46c18() -> ! {
    todo!("0x46c18 __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
// 0x46c8c — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEED1Ev
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()
pub fn stub_46c8c() -> ! {
    todo!("0x46c8c __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
// 0x46d38 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEED0Ev
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()
pub fn stub_46d38() -> ! {
    todo!("0x46d38 __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x46de8 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
pub fn stub_46de8() -> ! {
    todo!("0x46de8 __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")
}

#[doc(alias = "non_virtual_thunk_torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x46df8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
pub fn stub_46df8() -> ! {
    todo!("0x46df8 __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// 0x46e08 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
pub fn stub_46e08() -> ! {
    todo!("0x46e08 __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// 0x46eb4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
pub fn stub_46eb4() -> ! {
    todo!("0x46eb4 __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&)")]
// 0x49f64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&)
pub fn stub_49f64() -> ! {
    todo!("0x49f64 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*)")]
// 0x4a04c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_EC2IPS9_EERKSD_T_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*)
pub fn stub_4a04c() -> ! {
    todo!("0x4a04c __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_EC2IPS9_EERKSD_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x4a148 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
pub fn stub_4a148() -> ! {
    todo!("0x4a148 __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_")
}

#[doc(alias = "non_virtual_thunk_torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x4a150 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
pub fn stub_4a150() -> ! {
    todo!("0x4a150 __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_")
}

#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::Reflection::PropertyDescriptor const*)const")]
// 0x4a158 — __ZNK5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEclES5_
// was: boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::Reflection::PropertyDescriptor const*)const
pub fn stub_4a158() -> ! {
    todo!("0x4a158 __ZNK5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEclES5_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
// 0x4a21c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKN3RBX10Reflection18PropertyDescriptorEENS3_5list3INS3_5valueIS6_EENSG_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
pub fn stub_4a21c() -> ! {
    todo!("0x4a21c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKN3RBX10Reflection18PropertyDescriptorEENS3_5list3INS3_5valueIS6_EENSG_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Reflection::PropertyDescriptor const>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::PropertyDescriptor const)")]
// 0x4a27c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKN3RBX10Reflection18PropertyDescriptorEENS3_5list3INS3_5valueIS6_EENSG_IS7_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Reflection::PropertyDescriptor const>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::PropertyDescriptor const)
pub fn stub_4a27c() -> ! {
    todo!("0x4a27c __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKN3RBX10Reflection18PropertyDescriptorEENS3_5list3INS3_5valueIS6_EENSG_IS7_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_")
}

#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)")]
// 0x4a28c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6insertEPNS8_4slotE
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)
pub fn stub_4a28c() -> ! {
    todo!("0x4a28c __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6insertEPNS8_4slotE")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot*)")]
// 0x4a49c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot*)
pub fn stub_4a49c() -> ! {
    todo!("0x4a49c __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_")
}

#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::safe_static_init_mutex(void)")]
// 0x4a540 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::safe_static_init_mutex(void)
pub fn stub_4a540() -> ! {
    todo!("0x4a540 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>*>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>*)")]
// 0x4a544 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>*>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>*)
pub fn stub_4a544() -> ! {
    todo!("0x4a544 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>::~callable_slot()")]
// 0x4a640 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>::~callable_slot()
pub fn stub_4a640() -> ! {
    todo!("0x4a640 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>::~callable_slot()")]
// 0x4a714 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>::~callable_slot()
pub fn stub_4a714() -> ! {
    todo!("0x4a714 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::disconnect(void)")]
// 0x4a7ec — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot10disconnectEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::disconnect(void)
pub fn stub_4a7ec() -> ! {
    todo!("0x4a7ec __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::connected(void)const")]
// 0x4a8fc — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot9connectedEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::connected(void)const
pub fn stub_4a8fc() -> ! {
    todo!("0x4a8fc __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)")]
// 0x4a908 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)
pub fn stub_4a908() -> ! {
    todo!("0x4a908 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_")
}

#[doc(alias = "non_virtual_thunk_torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)")]
// 0x4a9dc — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)
pub fn stub_4a9dc() -> ! {
    todo!("0x4a9dc __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_")
}

#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::TextBox>>::operator()(boost::shared_ptr<RBX::TextBox>)const")]
// 0x4a9e4 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::operator()(boost::shared_ptr<RBX::TextBox>)const
pub fn stub_4a9e4() -> ! {
    todo!("0x4a9e4 __ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_")
}

#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)")]
// 0x4aaf4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6removeEPNS8_4slotE
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)
pub fn stub_4aaf4() -> ! {
    todo!("0x4aaf4 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6removeEPNS8_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)")]
// 0x4abe4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)
pub fn stub_4abe4() -> ! {
    todo!("0x4abe4 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)")]
// 0x4abe8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)
pub fn stub_4abe8() -> ! {
    todo!("0x4abe8 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::~callable()")]
// 0x4acd8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::~callable()
pub fn stub_4acd8() -> ! {
    todo!("0x4acd8 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::~callable()")]
// 0x4adac — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::~callable()
pub fn stub_4adac() -> ! {
    todo!("0x4adac __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::~slot()")]
// 0x4ae84 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::~slot()
pub fn stub_4ae84() -> ! {
    todo!("0x4ae84 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::~slot()")]
// 0x4af30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::~slot()
pub fn stub_4af30() -> ! {
    todo!("0x4af30 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD0Ev")
}

#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::TextBox>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::TextBox>> const&)")]
// 0x4afe0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::TextBox>> const&)
pub fn stub_4afe0() -> ! {
    todo!("0x4afe0 __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
// 0x4b010 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
pub fn stub_4b010() -> ! {
    todo!("0x4b010 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::TextBox>::invoke(boost::detail::function::function_buffer &,RBX::TextBox)")]
// 0x4b070 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::TextBox>::invoke(boost::detail::function::function_buffer &,RBX::TextBox)
pub fn stub_4b070() -> ! {
    todo!("0x4b070 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)")]
// 0x4b088 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX7TextBoxEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)
pub fn stub_4b088() -> ! {
    todo!("0x4b088 __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX7TextBoxEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::clear(void)")]
// 0x4bfdc — __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEE5clearEv
// was: boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::clear(void)
pub fn stub_4bfdc() -> ! {
    todo!("0x4bfdc __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEE5clearEv")
}

#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::TextBox>>::clear(void)")]
// 0x4c008 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::clear(void)
pub fn stub_4c008() -> ! {
    todo!("0x4c008 __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv")
}

#[doc(alias = "boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox>&&)")]
// 0x4d238 — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_
// was: boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox>&&)
pub fn stub_4d238() -> ! {
    todo!("0x4d238 __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_")
}

#[doc(alias = "boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox> const&)")]
// 0x4d2dc — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_
// was: boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox> const&)
pub fn stub_4d2dc() -> ! {
    todo!("0x4d2dc __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot> &)")]
// 0x4ee0c — __ZN3rbx7signals6signalIFvSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot> &)
pub fn stub_4ee0c() -> ! {
    todo!("0x4ee0c __ZN3rbx7signals6signalIFvSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")]
// 0x4f470 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)
pub fn stub_4f470() -> ! {
    todo!("0x4f470 __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
// 0x4f4e4 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEED1Ev
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()
pub fn stub_4f4e4() -> ! {
    todo!("0x4f4e4 __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
// 0x4f590 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEED0Ev
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()
pub fn stub_4f590() -> ! {
    todo!("0x4f590 __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x4f640 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
pub fn stub_4f640() -> ! {
    todo!("0x4f640 __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")
}

#[doc(alias = "non_virtual_thunk_torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x4f650 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
pub fn stub_4f650() -> ! {
    todo!("0x4f650 __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// 0x4f660 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
pub fn stub_4f660() -> ! {
    todo!("0x4f660 __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// 0x4f70c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
pub fn stub_4f70c() -> ! {
    todo!("0x4f70c __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev")
}

#[doc(alias = "std::vector<void *,std::allocator<void *>>::~vector()")]
// 0x62f08 — __ZNSt6vectorIPvSaIS0_EED1Ev
// was: std::vector<void *,std::allocator<void *>>::~vector()
pub fn stub_62f08() -> ! {
    todo!("0x62f08 __ZNSt6vectorIPvSaIS0_EED1Ev")
}

#[doc(alias = "std::vector<void *,std::allocator<void *>>::push_back(void * const&)")]
// 0x62f1c — __ZNSt6vectorIPvSaIS0_EE9push_backERKS0_
// was: std::vector<void *,std::allocator<void *>>::push_back(void * const&)
pub fn stub_62f1c() -> ! {
    todo!("0x62f1c __ZNSt6vectorIPvSaIS0_EE9push_backERKS0_")
}

#[doc(alias = "std::vector<void *,std::allocator<void *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<void **,std::vector<void *,std::allocator<void *>>>,void * const&)")]
// 0x62f48 — __ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_
// was: std::vector<void *,std::allocator<void *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<void **,std::vector<void *,std::allocator<void *>>>,void * const&)
pub fn stub_62f48() -> ! {
    todo!("0x62f48 __ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_")
}

#[doc(alias = "std::_Vector_base<void *,std::allocator<void *>>::_M_allocate(unsigned long)")]
// 0x63028 — __ZNSt12_Vector_baseIPvSaIS0_EE11_M_allocateEm
// was: std::_Vector_base<void *,std::allocator<void *>>::_M_allocate(unsigned long)
pub fn stub_63028() -> ! {
    todo!("0x63028 __ZNSt12_Vector_baseIPvSaIS0_EE11_M_allocateEm")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::connect<boost::function<void ()(RBX::StandardOutMessage const&)>>(boost::function<void ()(RBX::StandardOutMessage const&)> const&)")]
// 0x64bc0 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE7connectIN5boost8functionIS6_EEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::connect<boost::function<void ()(RBX::StandardOutMessage const&)>>(boost::function<void ()(RBX::StandardOutMessage const&)> const&)
pub fn stub_64bc0() -> ! {
    todo!("0x64bc0 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE7connectIN5boost8functionIS6_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::insert(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
// 0x64ca8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6insertEPNS7_4slotE
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::insert(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)
pub fn stub_64ca8() -> ! {
    todo!("0x64ca8 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6insertEPNS7_4slotE")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)")]
// 0x64eb8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)
pub fn stub_64eb8() -> ! {
    todo!("0x64eb8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSEPSA_")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)")]
// 0x64f5c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSERKSB_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)
pub fn stub_64f5c() -> ! {
    todo!("0x64f5c __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSERKSB_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_init_mutex(void)")]
// 0x65000 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_init_mutex(void)
pub fn stub_65000() -> ! {
    todo!("0x65000 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_do_get_mutex(void)")]
// 0x65004 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_do_get_mutex(void)
pub fn stub_65004() -> ! {
    todo!("0x65004 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*>(boost::function<void ()(RBX::StandardOutMessage const&)> const&,rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*)")]
// 0x650fc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_EC2IPS8_EERKSC_T_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*>(boost::function<void ()(RBX::StandardOutMessage const&)> const&,rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*)
pub fn stub_650fc() -> ! {
    todo!("0x650fc __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_EC2IPS8_EERKSC_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")]
// 0x651f8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED1Ev
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()
pub fn stub_651f8() -> ! {
    todo!("0x651f8 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")]
// 0x652cc — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED0Ev
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()
pub fn stub_652cc() -> ! {
    todo!("0x652cc __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::disconnect(void)")]
// 0x653a4 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot10disconnectEv
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::disconnect(void)
pub fn stub_653a4() -> ! {
    todo!("0x653a4 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::connected(void)const")]
// 0x654b4 — __ZNK3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot9connectedEv
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::connected(void)const
pub fn stub_654b4() -> ! {
    todo!("0x654b4 __ZNK3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")]
// 0x654c0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)
pub fn stub_654c0() -> ! {
    todo!("0x654c0 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_")
}

#[doc(alias = "non_virtual_thunk_torbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")]
// 0x654c8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)
pub fn stub_654c8() -> ! {
    todo!("0x654c8 __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_")
}

#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::operator()(RBX::StandardOutMessage const&)const")]
// 0x654d0 — __ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_
// was: boost::function1<void,RBX::StandardOutMessage const&>::operator()(RBX::StandardOutMessage const&)const
pub fn stub_654d0() -> ! {
    todo!("0x654d0 __ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::remove(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
// 0x65594 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6removeEPNS7_4slotE
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::remove(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)
pub fn stub_65594() -> ! {
    todo!("0x65594 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6removeEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_init_mutex(void)")]
// 0x65684 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_init_mutex(void)
pub fn stub_65684() -> ! {
    todo!("0x65684 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_do_get_mutex(void)")]
// 0x65688 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_do_get_mutex(void)
pub fn stub_65688() -> ! {
    todo!("0x65688 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")]
// 0x65778 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()
pub fn stub_65778() -> ! {
    todo!("0x65778 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")]
// 0x6584c — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()
pub fn stub_6584c() -> ! {
    todo!("0x6584c __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")]
// 0x65924 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD1Ev
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()
pub fn stub_65924() -> ! {
    todo!("0x65924 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")]
// 0x659d0 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD0Ev
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()
pub fn stub_659d0() -> ! {
    todo!("0x659d0 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD0Ev")
}

#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::assign_to_own(boost::function1<void,RBX::StandardOutMessage const&> const&)")]
// 0x65a80 — __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE13assign_to_ownERKS5_
// was: boost::function1<void,RBX::StandardOutMessage const&>::assign_to_own(boost::function1<void,RBX::StandardOutMessage const&> const&)
pub fn stub_65a80() -> ! {
    todo!("0x65a80 __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE13assign_to_ownERKS5_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
// 0x65ab0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorRKN3RBX18StandardOutMessageEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
pub fn stub_65ab0() -> ! {
    todo!("0x65ab0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorRKN3RBX18StandardOutMessageEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::StandardOutMessage const>::invoke(boost::detail::function::function_buffer &,RBX::StandardOutMessage const)")]
// 0x65b10 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorRKN3RBX18StandardOutMessageEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::StandardOutMessage const>::invoke(boost::detail::function::function_buffer &,RBX::StandardOutMessage const)
pub fn stub_65b10() -> ! {
    todo!("0x65b10 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorRKN3RBX18StandardOutMessageEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")
}

#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::clear(void)")]
// 0x65b20 — __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE5clearEv
// was: boost::function1<void,RBX::StandardOutMessage const&>::clear(void)
pub fn stub_65b20() -> ! {
    todo!("0x65b20 __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE5clearEv")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::find(int const&)")]
// 0x109b9c — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE4findERS1_
// was: std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::find(int const&)
pub fn stub_109b9c() -> ! {
    todo!("0x109b9c __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>> const&,std::less<int> const&)")]
// 0x109bf8 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_
// was: std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>> const&,std::less<int> const&)
pub fn stub_109bf8() -> ! {
    todo!("0x109bf8 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_Rb_tree_impl<std::less<std::string>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<std::string const,FITAG *>>> const&,std::less<std::string> const&)")]
// 0x109c38 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_Rb_tree_impl<std::less<std::string>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<std::string const,FITAG *>>> const&,std::less<std::string> const&)
pub fn stub_109c38() -> ! {
    todo!("0x109c38 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::lower_bound(int const&)")]
// 0x109c78 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE11lower_boundERS1_
// was: std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::lower_bound(int const&)
pub fn stub_109c78() -> ! {
    todo!("0x109c78 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE11lower_boundERS1_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_create_node(std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *> const&)")]
// 0x109d98 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE14_M_create_nodeERKSC_
// was: std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_create_node(std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *> const&)
pub fn stub_109d98() -> ! {
    todo!("0x109d98 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE14_M_create_nodeERKSC_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *> const&)")]
// 0x109dc8 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSC_
// was: std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *> const&)
pub fn stub_109dc8() -> ! {
    todo!("0x109dc8 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSC_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_insert_unique(std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *> const&)")]
// 0x109e4c — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueERKSC_
// was: std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_insert_unique(std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *> const&)
pub fn stub_109e4c() -> ! {
    todo!("0x109e4c __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueERKSC_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_create_node(std::pair<std::string const,FITAG *> const&)")]
// 0x109f3c — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE14_M_create_nodeERKS4_
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_create_node(std::pair<std::string const,FITAG *> const&)
pub fn stub_109f3c() -> ! {
    todo!("0x109f3c __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE14_M_create_nodeERKS4_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,FITAG *>> *)")]
// 0x10a03c — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,FITAG *>> *)
pub fn stub_10a03c() -> ! {
    todo!("0x10a03c __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,FITAG *>> *)")]
// 0x10a0e4 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,FITAG *>> *)
pub fn stub_10a0e4() -> ! {
    todo!("0x10a0e4 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>> *)")]
// 0x10a124 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
// was: std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>> *)
pub fn stub_10a124() -> ! {
    todo!("0x10a124 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,FITAG *>>)")]
// 0x10a160 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseESt17_Rb_tree_iteratorIS4_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,FITAG *>>)
pub fn stub_10a160() -> ! {
    todo!("0x10a160 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseESt17_Rb_tree_iteratorIS4_E")
}


#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::erase(std::_Rb_tree_iterator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>)")]
// 0x10a198 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE5eraseESt17_Rb_tree_iteratorISC_E
// was: std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::erase(std::_Rb_tree_iterator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>)
pub fn stub_10a198() -> ! {
    todo!("0x10a198 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE5eraseESt17_Rb_tree_iteratorISC_E")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *> const&)")]
// 0x10a1c8 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
// was: std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *> const&)
pub fn stub_10a1c8() -> ! {
    todo!("0x10a1c8 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::upper_bound(std::string const&)")]
// 0x10a2ec — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11upper_boundERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::upper_bound(std::string const&)
pub fn stub_10a2ec() -> ! {
    todo!("0x10a2ec __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11upper_boundERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::lower_bound(std::string const&)")]
// 0x10a334 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11lower_boundERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::lower_bound(std::string const&)
pub fn stub_10a334() -> ! {
    todo!("0x10a334 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11lower_boundERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::equal_range(std::string const&)")]
// 0x10a37c — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11equal_rangeERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::equal_range(std::string const&)
pub fn stub_10a37c() -> ! {
    todo!("0x10a37c __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11equal_rangeERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::find(std::string const&)")]
// 0x10a3c4 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE4findERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::find(std::string const&)
pub fn stub_10a3c4() -> ! {
    todo!("0x10a3c4 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,FITAG *> const&)")]
// 0x10a43c — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,FITAG *> const&)
pub fn stub_10a43c() -> ! {
    todo!("0x10a43c __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_insert_unique(std::pair<std::string const,FITAG *> const&)")]
// 0x10a4c0 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueERKS4_
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_insert_unique(std::pair<std::string const,FITAG *> const&)
pub fn stub_10a4c0() -> ! {
    todo!("0x10a4c0 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueERKS4_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,FITAG *>>,std::pair<std::string const,FITAG *> const&)")]
// 0x10a584 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,FITAG *>>,std::pair<std::string const,FITAG *> const&)
pub fn stub_10a584() -> ! {
    todo!("0x10a584 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,FITAG *>>,std::_Rb_tree_iterator<std::pair<std::string const,FITAG *>>)")]
// 0x10a6e4 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseESt17_Rb_tree_iteratorIS4_ESC_
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,FITAG *>>,std::_Rb_tree_iterator<std::pair<std::string const,FITAG *>>)
pub fn stub_10a6e4() -> ! {
    todo!("0x10a6e4 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseESt17_Rb_tree_iteratorIS4_ESC_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::erase(std::string const&)")]
// 0x10a760 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::erase(std::string const&)
pub fn stub_10a760() -> ! {
    todo!("0x10a760 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseERS1_")
}

#[doc(alias = "std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::operator[](std::string const&)")]
// 0x10a7a8 — __ZNSt3mapISsP5FITAGSt4lessISsESaISt4pairIKSsS1_EEEixERS5_
// was: std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::operator[](std::string const&)
pub fn stub_10a7a8() -> ! {
    todo!("0x10a7a8 __ZNSt3mapISsP5FITAGSt4lessISsESaISt4pairIKSsS1_EEEixERS5_")
}

#[doc(alias = "std::map<int,std::map*<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG>>>,FITAG *<int>,std::allocator<std::less<std::string><int const,std::map*<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG>>>>>>::operator[](int const&)")]
// 0x10a8e4 — __ZNSt3mapIiPS_ISsP5FITAGSt4lessISsESaISt4pairIKSsS1_EEES2_IiESaIS4_IKiS9_EEEixERSB_
// was: std::map<int,std::map*<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG>>>,FITAG *<int>,std::allocator<std::less<std::string><int const,std::map*<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG>>>>>>::operator[](int const&)
pub fn stub_10a8e4() -> ! {
    todo!("0x10a8e4 __ZNSt3mapIiPS_ISsP5FITAGSt4lessISsESaISt4pairIKSsS1_EEES2_IiESaIS4_IKiS9_EEEixERSB_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::find(int const&)")]
// 0x1118a0 — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE4findERS1_
// was: std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::find(int const&)
pub fn stub_1118a0() -> ! {
    todo!("0x1118a0 __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,PluginNode *>>> const&,std::less<int> const&)")]
// 0x1118fc — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_
// was: std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,PluginNode *>>> const&,std::less<int> const&)
pub fn stub_1118fc() -> ! {
    todo!("0x1118fc __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::lower_bound(int const&)")]
// 0x11193c — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE11lower_boundERS1_
// was: std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::lower_bound(int const&)
pub fn stub_11193c() -> ! {
    todo!("0x11193c __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE11lower_boundERS1_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,PluginNode *>> *)")]
// 0x111970 — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// was: std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,PluginNode *>> *)
pub fn stub_111970() -> ! {
    todo!("0x111970 __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const,PluginNode *>>>::allocate(unsigned long,void const*)")]
// 0x1119ac — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiP10PluginNodeEEE8allocateEmPKv
// was: __gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const,PluginNode *>>>::allocate(unsigned long,void const*)
pub fn stub_1119ac() -> ! {
    todo!("0x1119ac __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiP10PluginNodeEEE8allocateEmPKv")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_create_node(std::pair<int const,PluginNode *> const&)")]
// 0x1119dc — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE14_M_create_nodeERKS4_
// was: std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_create_node(std::pair<int const,PluginNode *> const&)
pub fn stub_1119dc() -> ! {
    todo!("0x1119dc __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE14_M_create_nodeERKS4_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,PluginNode *> const&)")]
// 0x111a0c — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// was: std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,PluginNode *> const&)
pub fn stub_111a0c() -> ! {
    todo!("0x111a0c __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert_unique(std::pair<int const,PluginNode *> const&)")]
// 0x111a90 — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE16_M_insert_uniqueERKS4_
// was: std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert_unique(std::pair<int const,PluginNode *> const&)
pub fn stub_111a90() -> ! {
    todo!("0x111a90 __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE16_M_insert_uniqueERKS4_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,PluginNode *>>,std::pair<int const,PluginNode *> const&)")]
// 0x111b50 — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// was: std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,PluginNode *>>,std::pair<int const,PluginNode *> const&)
pub fn stub_111b50() -> ! {
    todo!("0x111b50 __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")
}

#[doc(alias = "std::map<int,PluginNode *,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::operator[](int const&)")]
// 0x111c74 — __ZNSt3mapIiP10PluginNodeSt4lessIiESaISt4pairIKiS1_EEEixERS5_
// was: std::map<int,PluginNode *,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::operator[](int const&)
pub fn stub_111c74() -> ! {
    todo!("0x111c74 __ZNSt3mapIiP10PluginNodeSt4lessIiESaISt4pairIKiS1_EEEixERS5_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>> const&,std::less<int> const&)")]
// 0x1c4b48 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_
// was: std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>> const&,std::less<int> const&)
pub fn stub_1c4b48() -> ! {
    todo!("0x1c4b48 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::lower_bound(int const&)")]
// 0x1c4b88 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE11lower_boundERS1_
// was: std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::lower_bound(int const&)
pub fn stub_1c4b88() -> ! {
    todo!("0x1c4b88 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE11lower_boundERS1_")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_Rb_tree_impl<std::less<unsigned short>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>>> const&,std::less<unsigned short> const&)")]
// 0x1c4bbc — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_Rb_tree_impl<std::less<unsigned short>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>>> const&,std::less<unsigned short> const&)
pub fn stub_1c4bbc() -> ! {
    todo!("0x1c4bbc __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::lower_bound(unsigned short const&)")]
// 0x1c4bfc — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE11lower_boundERS1_
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::lower_bound(unsigned short const&)
pub fn stub_1c4bfc() -> ! {
    todo!("0x1c4bfc __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE11lower_boundERS1_")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>>>::allocate(unsigned long,void const*)")]
// 0x1c4c30 — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKtP10tagTagInfoEEE8allocateEmPKv
// was: __gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>>>::allocate(unsigned long,void const*)
pub fn stub_1c4c30() -> ! {
    todo!("0x1c4c30 __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKtP10tagTagInfoEEE8allocateEmPKv")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_create_node(std::pair<unsigned short const,tagTagInfo *> const&)")]
// 0x1c4c60 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE14_M_create_nodeERKS4_
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_create_node(std::pair<unsigned short const,tagTagInfo *> const&)
pub fn stub_1c4c60() -> ! {
    todo!("0x1c4c60 __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE14_M_create_nodeERKS4_")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,tagTagInfo *> const&)")]
// 0x1c4c90 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,tagTagInfo *> const&)
pub fn stub_1c4c90() -> ! {
    todo!("0x1c4c90 __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::allocate(unsigned long,void const*)")]
// 0x1c4d14 — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS2_IKtS6_EEEEEE8allocateEmPKv
// was: __gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::allocate(unsigned long,void const*)
pub fn stub_1c4d14() -> ! {
    todo!("0x1c4d14 __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS2_IKtS6_EEEEEE8allocateEmPKv")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_create_node(std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
// 0x1c4d44 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE14_M_create_nodeERKSC_
// was: std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_create_node(std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)
pub fn stub_1c4d44() -> ! {
    todo!("0x1c4d44 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE14_M_create_nodeERKSC_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
// 0x1c4d74 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSC_
// was: std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)
pub fn stub_1c4d74() -> ! {
    todo!("0x1c4d74 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSC_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert_unique(std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
// 0x1c4df8 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueERKSC_
// was: std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert_unique(std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)
pub fn stub_1c4df8() -> ! {
    todo!("0x1c4df8 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueERKSC_")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>> *)")]
// 0x1c4eb8 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>> *)
pub fn stub_1c4eb8() -> ! {
    todo!("0x1c4eb8 __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>> *)")]
// 0x1c4ef4 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
// was: std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>> *)
pub fn stub_1c4ef4() -> ! {
    todo!("0x1c4ef4 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
// 0x1c4f30 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
// was: std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)
pub fn stub_1c4f30() -> ! {
    todo!("0x1c4f30 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_")
}

#[doc(alias = "std::map<int,std::map*<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo>>>,tagTagInfo *<int>,std::allocator<std::less<unsigned short><int const,std::map*<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo>>>>>>::operator[](int const&)")]
// 0x1c5054 — __ZNSt3mapIiPS_ItP10tagTagInfoSt4lessItESaISt4pairIKtS1_EEES2_IiESaIS4_IKiS9_EEEixERSB_
// was: std::map<int,std::map*<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo>>>,tagTagInfo *<int>,std::allocator<std::less<unsigned short><int const,std::map*<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo>>>>>>::operator[](int const&)
pub fn stub_1c5054() -> ! {
    todo!("0x1c5054 __ZNSt3mapIiPS_ItP10tagTagInfoSt4lessItESaISt4pairIKtS1_EEES2_IiESaIS4_IKiS9_EEEixERSB_")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert_unique(std::pair<unsigned short const,tagTagInfo *> const&)")]
// 0x1c50c0 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE16_M_insert_uniqueERKS4_
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert_unique(std::pair<unsigned short const,tagTagInfo *> const&)
pub fn stub_1c50c0() -> ! {
    todo!("0x1c50c0 __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE16_M_insert_uniqueERKS4_")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,tagTagInfo *>>,std::pair<unsigned short const,tagTagInfo *> const&)")]
// 0x1c5180 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,tagTagInfo *>>,std::pair<unsigned short const,tagTagInfo *> const&)
pub fn stub_1c5180() -> ! {
    todo!("0x1c5180 __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")
}

#[doc(alias = "std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::operator[](unsigned short const&)")]
// 0x1c52a4 — __ZNSt3mapItP10tagTagInfoSt4lessItESaISt4pairIKtS1_EEEixERS5_
// was: std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::operator[](unsigned short const&)
pub fn stub_1c52a4() -> ! {
    todo!("0x1c52a4 __ZNSt3mapItP10tagTagInfoSt4lessItESaISt4pairIKtS1_EEEixERS5_")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::size(void)const")]
// 0x1c7340 — __ZNKSt6vectorISsSaISsEE4sizeEv
// was: std::vector<std::string,std::allocator<std::string>>::size(void)const
pub fn stub_1c7340() -> ! {
    todo!("0x1c7340 __ZNKSt6vectorISsSaISsEE4sizeEv")
}

#[doc(alias = "std::__deque_buf_size(unsigned long)")]
// 0x1c8d60 — __ZSt16__deque_buf_sizem
// was: std::__deque_buf_size(unsigned long)
pub fn stub_1c8d60() -> ! {
    todo!("0x1c8d60 __ZSt16__deque_buf_sizem")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_destroy_data(std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::allocator<unsigned short> const&)")]
// 0x1c8d84 — __ZNSt5dequeItSaItEE15_M_destroy_dataESt15_Deque_iteratorItRtPtES5_RKS0_
// was: std::deque<unsigned short,std::allocator<unsigned short>>::_M_destroy_data(std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::allocator<unsigned short> const&)
pub fn stub_1c8d84() -> ! {
    todo!("0x1c8d84 __ZNSt5dequeItSaItEE15_M_destroy_dataESt15_Deque_iteratorItRtPtES5_RKS0_")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_destroy_data(std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::allocator<unsigned char *> const&)")]
// 0x1c8d88 — __ZNSt5dequeIPhSaIS0_EE15_M_destroy_dataESt15_Deque_iteratorIS0_RS0_PS0_ES6_RKS1_
// was: std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_destroy_data(std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::allocator<unsigned char *> const&)
pub fn stub_1c8d88() -> ! {
    todo!("0x1c8d88 __ZNSt5dequeIPhSaIS0_EE15_M_destroy_dataESt15_Deque_iteratorIS0_RS0_PS0_ES6_RKS1_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::find(unsigned int const&)")]
// 0x1c8d8c — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE4findERS1_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::find(unsigned int const&)
pub fn stub_1c8d8c() -> ! {
    todo!("0x1c8d8c __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_Rb_tree_impl<std::less<unsigned int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<unsigned int const,int>>> const&,std::less<unsigned int> const&)")]
// 0x1c8de8 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE13_Rb_tree_implIS6_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS2_EERKS6_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_Rb_tree_impl<std::less<unsigned int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<unsigned int const,int>>> const&,std::less<unsigned int> const&)
pub fn stub_1c8de8() -> ! {
    todo!("0x1c8de8 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE13_Rb_tree_implIS6_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS2_EERKS6_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::lower_bound(unsigned int const&)")]
// 0x1c8e28 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE11lower_boundERS1_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::lower_bound(unsigned int const&)
pub fn stub_1c8e28() -> ! {
    todo!("0x1c8e28 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE11lower_boundERS1_")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::_M_set_node(unsigned short **)")]
// 0x1c8e5c — __ZNSt15_Deque_iteratorItRtPtE11_M_set_nodeEPS1_
// was: std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::_M_set_node(unsigned short **)
pub fn stub_1c8e5c() -> ! {
    todo!("0x1c8e5c __ZNSt15_Deque_iteratorItRtPtE11_M_set_nodeEPS1_")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::operator--(void)")]
// 0x1c8e8c — __ZNSt15_Deque_iteratorItRtPtEmmEv
// was: std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::operator--(void)
pub fn stub_1c8e8c() -> ! {
    todo!("0x1c8e8c __ZNSt15_Deque_iteratorItRtPtEmmEv")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::difference_type std::operator-<unsigned char *,unsigned char *&,unsigned char **>(std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> const&,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> const&)")]
// 0x1c8ecc — __ZStmiIPhRS0_PS0_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS7_SA_
// was: std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::difference_type std::operator-<unsigned char *,unsigned char *&,unsigned char **>(std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> const&,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> const&)
pub fn stub_1c8ecc() -> ! {
    todo!("0x1c8ecc __ZStmiIPhRS0_PS0_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS7_SA_")
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::difference_type std::operator-<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> const&,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> const&)")]
// 0x1c8f1c — __ZStmiIN6TagLib7MDMODELERS1_PS1_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS8_SB_
// was: std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::difference_type std::operator-<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> const&,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> const&)
pub fn stub_1c8f1c() -> ! {
    todo!("0x1c8f1c __ZStmiIN6TagLib7MDMODELERS1_PS1_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS8_SB_")
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>::operator++(void)")]
// 0x1c8f6c — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERKS1_PS2_EppEv
// was: std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>::operator++(void)
pub fn stub_1c8f6c() -> ! {
    todo!("0x1c8f6c __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERKS1_PS2_EppEv")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::operator++(void)")]
// 0x1c8fc4 — __ZNSt15_Deque_iteratorItRtPtEppEv
// was: std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::operator++(void)
pub fn stub_1c8fc4() -> ! {
    todo!("0x1c8fc4 __ZNSt15_Deque_iteratorItRtPtEppEv")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>::difference_type std::operator-<unsigned short,unsigned short const&,unsigned short const*>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*> const&,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*> const&)")]
// 0x1c9004 — __ZStmiItRKtPS0_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS7_SA_
// was: std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>::difference_type std::operator-<unsigned short,unsigned short const&,unsigned short const*>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*> const&,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*> const&)
pub fn stub_1c9004() -> ! {
    todo!("0x1c9004 __ZStmiItRKtPS0_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS7_SA_")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>::difference_type std::operator-<unsigned char *,unsigned char * const&,unsigned char * const*>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*> const&,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*> const&)")]
// 0x1c9054 — __ZStmiIPhRKS0_PS1_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS8_SB_
// was: std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>::difference_type std::operator-<unsigned char *,unsigned char * const&,unsigned char * const*>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*> const&,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*> const&)
pub fn stub_1c9054() -> ! {
    todo!("0x1c9054 __ZStmiIPhRKS0_PS1_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS8_SB_")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned int const,int>>>::allocate(unsigned long,void const*)")]
// 0x1c90a4 — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKjiEEE8allocateEmPKv
// was: __gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned int const,int>>>::allocate(unsigned long,void const*)
pub fn stub_1c90a4() -> ! {
    todo!("0x1c90a4 __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKjiEEE8allocateEmPKv")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_create_node(std::pair<unsigned int const,int> const&)")]
// 0x1c90d4 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE14_M_create_nodeERKS2_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_create_node(std::pair<unsigned int const,int> const&)
pub fn stub_1c90d4() -> ! {
    todo!("0x1c90d4 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_allocate_map(unsigned long)")]
// 0x1c9184 — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE15_M_allocate_mapEm
// was: std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_allocate_map(unsigned long)
pub fn stub_1c9184() -> ! {
    todo!("0x1c9184 __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE15_M_allocate_mapEm")
}

#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_allocate_map(unsigned long)")]
// 0x1c924c — __ZNSt11_Deque_baseIPhSaIS0_EE15_M_allocate_mapEm
// was: std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_allocate_map(unsigned long)
pub fn stub_1c924c() -> ! {
    todo!("0x1c924c __ZNSt11_Deque_baseIPhSaIS0_EE15_M_allocate_mapEm")
}

#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_allocate_map(unsigned long)")]
// 0x1c9314 — __ZNSt11_Deque_baseItSaItEE15_M_allocate_mapEm
// was: std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_allocate_map(unsigned long)
pub fn stub_1c9314() -> ! {
    todo!("0x1c9314 __ZNSt11_Deque_baseItSaItEE15_M_allocate_mapEm")
}

#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_nodes(TagLib::MDMODEL**,TagLib::MDMODEL**)")]
// 0x1c93bc — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE16_M_destroy_nodesEPPS1_S5_
// was: std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_nodes(TagLib::MDMODEL**,TagLib::MDMODEL**)
pub fn stub_1c93bc() -> ! {
    todo!("0x1c93bc __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE16_M_destroy_nodesEPPS1_S5_")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_pop_back_aux(void)")]
// 0x1c94ac — __ZNSt5dequeItSaItEE15_M_pop_back_auxEv
// was: std::deque<unsigned short,std::allocator<unsigned short>>::_M_pop_back_aux(void)
pub fn stub_1c94ac() -> ! {
    todo!("0x1c94ac __ZNSt5dequeItSaItEE15_M_pop_back_auxEv")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,int>> *)")]
// 0x1c94e0 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,int>> *)
pub fn stub_1c94e0() -> ! {
    todo!("0x1c94e0 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::~_Deque_base()")]
// 0x1c951c — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EED2Ev
// was: std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::~_Deque_base()
pub fn stub_1c951c() -> ! {
    todo!("0x1c951c __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EED2Ev")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,int> const&)")]
// 0x1c9550 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,int> const&)
pub fn stub_1c9550() -> ! {
    todo!("0x1c9550 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "TagLib::MDMODEL * * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<TagLib::MDMODEL *>(TagLib::MDMODEL * const*,TagLib::MDMODEL * const*,TagLib::MDMODEL * *)")]
// 0x1c95d4 — __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_
// was: TagLib::MDMODEL * * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<TagLib::MDMODEL *>(TagLib::MDMODEL * const*,TagLib::MDMODEL * const*,TagLib::MDMODEL * *)
pub fn stub_1c95d4() -> ! {
    todo!("0x1c95d4 __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_")
}

#[doc(alias = "TagLib::MDMODEL * * std::__copy<true,std::random_access_iterator_tag>::copy<TagLib::MDMODEL *>(TagLib::MDMODEL * const*,TagLib::MDMODEL * const*,TagLib::MDMODEL * *)")]
// 0x1c9604 — __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_
// was: TagLib::MDMODEL * * std::__copy<true,std::random_access_iterator_tag>::copy<TagLib::MDMODEL *>(TagLib::MDMODEL * const*,TagLib::MDMODEL * const*,TagLib::MDMODEL * *)
pub fn stub_1c9604() -> ! {
    todo!("0x1c9604 __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_")
}

#[doc(alias = "unsigned short * * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<unsigned short *>(unsigned short * const*,unsigned short * const*,unsigned short * *)")]
// 0x1c9630 — __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPtEEPT_PKS4_S7_S5_
// was: unsigned short * * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<unsigned short *>(unsigned short * const*,unsigned short * const*,unsigned short * *)
pub fn stub_1c9630() -> ! {
    todo!("0x1c9630 __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPtEEPT_PKS4_S7_S5_")
}

#[doc(alias = "unsigned short * * std::__copy<true,std::random_access_iterator_tag>::copy<unsigned short *>(unsigned short * const*,unsigned short * const*,unsigned short * *)")]
// 0x1c9660 — __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPtEEPT_PKS4_S7_S5_
// was: unsigned short * * std::__copy<true,std::random_access_iterator_tag>::copy<unsigned short *>(unsigned short * const*,unsigned short * const*,unsigned short * *)
pub fn stub_1c9660() -> ! {
    todo!("0x1c9660 __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPtEEPT_PKS4_S7_S5_")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_reallocate_map(unsigned long,bool)")]
// 0x1c968c — __ZNSt5dequeItSaItEE17_M_reallocate_mapEmb
// was: std::deque<unsigned short,std::allocator<unsigned short>>::_M_reallocate_map(unsigned long,bool)
pub fn stub_1c968c() -> ! {
    todo!("0x1c968c __ZNSt5dequeItSaItEE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_reserve_map_at_back(unsigned long)")]
// 0x1c97b4 — __ZNSt5dequeItSaItEE22_M_reserve_map_at_backEm
// was: std::deque<unsigned short,std::allocator<unsigned short>>::_M_reserve_map_at_back(unsigned long)
pub fn stub_1c97b4() -> ! {
    todo!("0x1c97b4 __ZNSt5dequeItSaItEE22_M_reserve_map_at_backEm")
}

#[doc(alias = "unsigned char ** * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<unsigned char **>(unsigned char ** const*,unsigned char ** const*,unsigned char ** *)")]
// 0x1c97e8 — __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPPhEEPT_PKS5_S8_S6_
// was: unsigned char ** * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<unsigned char **>(unsigned char ** const*,unsigned char ** const*,unsigned char ** *)
pub fn stub_1c97e8() -> ! {
    todo!("0x1c97e8 __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPPhEEPT_PKS5_S8_S6_")
}

#[doc(alias = "unsigned char ** * std::__copy<true,std::random_access_iterator_tag>::copy<unsigned char **>(unsigned char ** const*,unsigned char ** const*,unsigned char ** *)")]
// 0x1c9818 — __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPPhEEPT_PKS5_S8_S6_
// was: unsigned char ** * std::__copy<true,std::random_access_iterator_tag>::copy<unsigned char **>(unsigned char ** const*,unsigned char ** const*,unsigned char ** *)
pub fn stub_1c9818() -> ! {
    todo!("0x1c9818 __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPPhEEPT_PKS5_S8_S6_")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::back(void)")]
// 0x1c9844 — __ZNSt5dequeItSaItEE4backEv
// was: std::deque<unsigned short,std::allocator<unsigned short>>::back(void)
pub fn stub_1c9844() -> ! {
    todo!("0x1c9844 __ZNSt5dequeItSaItEE4backEv")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert_unique(std::pair<unsigned int const,int> const&)")]
// 0x1c9884 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueERKS2_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert_unique(std::pair<unsigned int const,int> const&)
pub fn stub_1c9884() -> ! {
    todo!("0x1c9884 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,int>>,std::pair<unsigned int const,int> const&)")]
// 0x1c9944 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,int>>,std::pair<unsigned int const,int> const&)
pub fn stub_1c9944() -> ! {
    todo!("0x1c9944 __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::_M_set_node(TagLib::MDMODEL**)")]
// 0x1c9a68 — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_E11_M_set_nodeEPS3_
// was: std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::_M_set_node(TagLib::MDMODEL**)
pub fn stub_1c9a68() -> ! {
    todo!("0x1c9a68 __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_E11_M_set_nodeEPS3_")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_reallocate_map(unsigned long,bool)")]
// 0x1c9a98 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE17_M_reallocate_mapEmb
// was: std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_reallocate_map(unsigned long,bool)
pub fn stub_1c9a98() -> ! {
    todo!("0x1c9a98 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_reserve_map_at_back(unsigned long)")]
// 0x1c9bc0 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE22_M_reserve_map_at_backEm
// was: std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_reserve_map_at_back(unsigned long)
pub fn stub_1c9bc0() -> ! {
    todo!("0x1c9bc0 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE22_M_reserve_map_at_backEm")
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::operator++(void)")]
// 0x1c9bf4 — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EppEv
// was: std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::operator++(void)
pub fn stub_1c9bf4() -> ! {
    todo!("0x1c9bf4 __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EppEv")
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::__uninitialized_copy_aux<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::__false_type)")]
// 0x1c9c34 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_St12__false_type
// was: std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::__uninitialized_copy_aux<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::__false_type)
pub fn stub_1c9c34() -> ! {
    todo!("0x1c9c34 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_St12__false_type")
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::uninitialized_copy<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>)")]
// 0x1c9ca4 — __ZSt18uninitialized_copyISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_
// was: std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::uninitialized_copy<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>)
pub fn stub_1c9ca4() -> ! {
    todo!("0x1c9ca4 __ZSt18uninitialized_copyISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_")
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::__uninitialized_copy_a<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,TagLib::MDMODEL>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::allocator<TagLib::MDMODEL>)")]
// 0x1c9d24 — __ZSt22__uninitialized_copy_aISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_ES2_ET0_T_SB_SA_SaIT1_E
// was: std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::__uninitialized_copy_a<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,TagLib::MDMODEL>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::allocator<TagLib::MDMODEL>)
pub fn stub_1c9d24() -> ! {
    todo!("0x1c9d24 __ZSt22__uninitialized_copy_aISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_ES2_ET0_T_SB_SA_SaIT1_E")
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::operator--(void)")]
// 0x1c9da0 — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EmmEv
// was: std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::operator--(void)
pub fn stub_1c9da0() -> ! {
    todo!("0x1c9da0 __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EmmEv")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::back(void)")]
// 0x1c9de0 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE4backEv
// was: std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::back(void)
pub fn stub_1c9de0() -> ! {
    todo!("0x1c9de0 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE4backEv")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>::operator++(void)")]
// 0x1c9e20 — __ZNSt15_Deque_iteratorItRKtPS0_EppEv
// was: std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>::operator++(void)
pub fn stub_1c9e20() -> ! {
    todo!("0x1c9e20 __ZNSt15_Deque_iteratorItRKtPS0_EppEv")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
// 0x1c9e78 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorItRKtPS4_ES3_ItRtPtEEET0_T_SC_SB_
// was: std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)
pub fn stub_1c9e78() -> ! {
    todo!("0x1c9e78 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorItRKtPS4_ES3_ItRtPtEEET0_T_SC_SB_")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy_aux<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
// 0x1ca124 — __ZSt10__copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_
// was: std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy_aux<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)
pub fn stub_1ca124() -> ! {
    todo!("0x1ca124 __ZSt10__copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy_normal<false,false>::__copy_n<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
// 0x1ca1a0 — __ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorItRKtPS3_ES2_ItRtPtEEET0_T_SB_SA_
// was: std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy_normal<false,false>::__copy_n<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)
pub fn stub_1ca1a0() -> ! {
    todo!("0x1ca1a0 __ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorItRKtPS3_ES2_ItRtPtEEET0_T_SB_SA_")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
// 0x1ca21c — __ZSt4copyISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_
// was: std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)
pub fn stub_1ca21c() -> ! {
    todo!("0x1ca21c __ZSt4copyISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__uninitialized_copy_aux<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::__true_type)")]
// 0x1ca298 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_St11__true_type
// was: std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__uninitialized_copy_aux<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::__true_type)
pub fn stub_1ca298() -> ! {
    todo!("0x1ca298 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_St11__true_type")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::uninitialized_copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
// 0x1ca314 — __ZSt18uninitialized_copyISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_
// was: std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::uninitialized_copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)
pub fn stub_1ca314() -> ! {
    todo!("0x1ca314 __ZSt18uninitialized_copyISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__uninitialized_copy_a<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,unsigned short>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::allocator<unsigned short>)")]
// 0x1ca394 — __ZSt22__uninitialized_copy_aISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEtET0_T_S9_S8_SaIT1_E
// was: std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__uninitialized_copy_a<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,unsigned short>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::allocator<unsigned short>)
pub fn stub_1ca394() -> ! {
    todo!("0x1ca394 __ZSt22__uninitialized_copy_aISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEtET0_T_S9_S8_SaIT1_E")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>::operator++(void)")]
// 0x1ca410 — __ZNSt15_Deque_iteratorIPhRKS0_PS1_EppEv
// was: std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>::operator++(void)
pub fn stub_1ca410() -> ! {
    todo!("0x1ca410 __ZNSt15_Deque_iteratorIPhRKS0_PS1_EppEv")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::difference_type std::operator-<unsigned short,unsigned short &,unsigned short *>(std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> const&,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> const&)")]
// 0x1ca468 — __ZStmiItRtPtENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS6_S9_
// was: std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::difference_type std::operator-<unsigned short,unsigned short &,unsigned short *>(std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> const&,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> const&)
pub fn stub_1ca468() -> ! {
    todo!("0x1ca468 __ZStmiItRtPtENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS6_S9_")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::_M_set_node(unsigned char ***)")]
// 0x1ca4b8 — __ZNSt15_Deque_iteratorIPhRS0_PS0_E11_M_set_nodeEPS2_
// was: std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::_M_set_node(unsigned char ***)
pub fn stub_1ca4b8() -> ! {
    todo!("0x1ca4b8 __ZNSt15_Deque_iteratorIPhRS0_PS0_E11_M_set_nodeEPS2_")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::operator++(void)")]
// 0x1ca4e8 — __ZNSt15_Deque_iteratorIPhRS0_PS0_EppEv
// was: std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::operator++(void)
pub fn stub_1ca4e8() -> ! {
    todo!("0x1ca4e8 __ZNSt15_Deque_iteratorIPhRS0_PS0_EppEv")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
// 0x1ca528 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPhRKS4_PS5_ES3_IS4_RS4_PS4_EEET0_T_SD_SC_
// was: std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)
pub fn stub_1ca528() -> ! {
    todo!("0x1ca528 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPhRKS4_PS5_ES3_IS4_RS4_PS4_EEET0_T_SD_SC_")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__copy_aux<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
// 0x1ca7d4 — __ZSt10__copy_auxISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_
// was: std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__copy_aux<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)
pub fn stub_1ca7d4() -> ! {
    todo!("0x1ca7d4 __ZSt10__copy_auxISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__copy_normal<false,false>::__copy_n<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
// 0x1ca850 — __ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorIPhRKS3_PS4_ES2_IS3_RS3_PS3_EEET0_T_SC_SB_
// was: std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__copy_normal<false,false>::__copy_n<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)
pub fn stub_1ca850() -> ! {
    todo!("0x1ca850 __ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorIPhRKS3_PS4_ES2_IS3_RS3_PS3_EEET0_T_SC_SB_")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::copy<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
// 0x1ca8cc — __ZSt4copyISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_
// was: std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::copy<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)
pub fn stub_1ca8cc() -> ! {
    todo!("0x1ca8cc __ZSt4copyISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__uninitialized_copy_aux<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::__true_type)")]
// 0x1ca948 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_St11__true_type
// was: std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__uninitialized_copy_aux<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::__true_type)
pub fn stub_1ca948() -> ! {
    todo!("0x1ca948 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_St11__true_type")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::uninitialized_copy<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
// 0x1ca9c4 — __ZSt18uninitialized_copyISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_
// was: std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::uninitialized_copy<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)
pub fn stub_1ca9c4() -> ! {
    todo!("0x1ca9c4 __ZSt18uninitialized_copyISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_EET0_T_SA_S9_")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__uninitialized_copy_a<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,unsigned char *>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::allocator<unsigned char *>)")]
// 0x1caa44 — __ZSt22__uninitialized_copy_aISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_ES1_ET0_T_SA_S9_SaIT1_E
// was: std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__uninitialized_copy_a<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,unsigned char *>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::allocator<unsigned char *>)
pub fn stub_1caa44() -> ! {
    todo!("0x1caa44 __ZSt22__uninitialized_copy_aISt15_Deque_iteratorIPhRKS1_PS2_ES0_IS1_RS1_PS1_ES1_ET0_T_SA_S9_SaIT1_E")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_reallocate_map(unsigned long,bool)")]
// 0x1caac0 — __ZNSt5dequeIPhSaIS0_EE17_M_reallocate_mapEmb
// was: std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_reallocate_map(unsigned long,bool)
pub fn stub_1caac0() -> ! {
    todo!("0x1caac0 __ZNSt5dequeIPhSaIS0_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_reserve_map_at_back(unsigned long)")]
// 0x1cabe8 — __ZNSt5dequeIPhSaIS0_EE22_M_reserve_map_at_backEm
// was: std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_reserve_map_at_back(unsigned long)
pub fn stub_1cabe8() -> ! {
    todo!("0x1cabe8 __ZNSt5dequeIPhSaIS0_EE22_M_reserve_map_at_backEm")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_push_back_aux(unsigned char * const&)")]
// 0x1cac1c — __ZNSt5dequeIPhSaIS0_EE16_M_push_back_auxERKS0_
// was: std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_push_back_aux(unsigned char * const&)
pub fn stub_1cac1c() -> ! {
    todo!("0x1cac1c __ZNSt5dequeIPhSaIS0_EE16_M_push_back_auxERKS0_")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::push_back(unsigned char * const&)")]
// 0x1cac80 — __ZNSt5dequeIPhSaIS0_EE9push_backERKS0_
// was: std::deque<unsigned char *,std::allocator<unsigned char *>>::push_back(unsigned char * const&)
pub fn stub_1cac80() -> ! {
    todo!("0x1cac80 __ZNSt5dequeIPhSaIS0_EE9push_backERKS0_")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::operator--(void)")]
// 0x1cacc4 — __ZNSt15_Deque_iteratorIPhRS0_PS0_EmmEv
// was: std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::operator--(void)
pub fn stub_1cacc4() -> ! {
    todo!("0x1cacc4 __ZNSt15_Deque_iteratorIPhRS0_PS0_EmmEv")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::back(void)")]
// 0x1cad04 — __ZNSt5dequeIPhSaIS0_EE4backEv
// was: std::deque<unsigned char *,std::allocator<unsigned char *>>::back(void)
pub fn stub_1cad04() -> ! {
    todo!("0x1cad04 __ZNSt5dequeIPhSaIS0_EE4backEv")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_push_back_aux(unsigned short const&)")]
// 0x1cad44 — __ZNSt5dequeItSaItEE16_M_push_back_auxERKt
// was: std::deque<unsigned short,std::allocator<unsigned short>>::_M_push_back_aux(unsigned short const&)
pub fn stub_1cad44() -> ! {
    todo!("0x1cad44 __ZNSt5dequeItSaItEE16_M_push_back_auxERKt")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::push_back(unsigned short const&)")]
// 0x1cada8 — __ZNSt5dequeItSaItEE9push_backERKt
// was: std::deque<unsigned short,std::allocator<unsigned short>>::push_back(unsigned short const&)
pub fn stub_1cada8() -> ! {
    todo!("0x1cada8 __ZNSt5dequeItSaItEE9push_backERKt")
}

#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_destroy_nodes(unsigned short **,unsigned short **)")]
// 0x1cadec — __ZNSt11_Deque_baseItSaItEE16_M_destroy_nodesEPPtS3_
// was: std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_destroy_nodes(unsigned short **,unsigned short **)
pub fn stub_1cadec() -> ! {
    todo!("0x1cadec __ZNSt11_Deque_baseItSaItEE16_M_destroy_nodesEPPtS3_")
}

#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::~_Deque_base()")]
// 0x1caedc — __ZNSt11_Deque_baseItSaItEED2Ev
// was: std::_Deque_base<unsigned short,std::allocator<unsigned short>>::~_Deque_base()
pub fn stub_1caedc() -> ! {
    todo!("0x1caedc __ZNSt11_Deque_baseItSaItEED2Ev")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::~deque()")]
// 0x1caf10 — __ZNSt5dequeItSaItEED2Ev
// was: std::deque<unsigned short,std::allocator<unsigned short>>::~deque()
pub fn stub_1caf10() -> ! {
    todo!("0x1caf10 __ZNSt5dequeItSaItEED2Ev")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_push_back_aux(TagLib::MDMODEL const&)")]
// 0x1caf80 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE16_M_push_back_auxERKS1_
// was: std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_push_back_aux(TagLib::MDMODEL const&)
pub fn stub_1caf80() -> ! {
    todo!("0x1caf80 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE16_M_push_back_auxERKS1_")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::push_back(TagLib::MDMODEL const&)")]
// 0x1cafe4 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE9push_backERKS1_
// was: std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::push_back(TagLib::MDMODEL const&)
pub fn stub_1cafe4() -> ! {
    todo!("0x1cafe4 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_pop_back_aux(void)")]
// 0x1cb028 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE15_M_pop_back_auxEv
// was: std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_pop_back_aux(void)
pub fn stub_1cb028() -> ! {
    todo!("0x1cb028 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE15_M_pop_back_auxEv")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_pop_back_aux(void)")]
// 0x1cb05c — __ZNSt5dequeIPhSaIS0_EE15_M_pop_back_auxEv
// was: std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_pop_back_aux(void)
pub fn stub_1cb05c() -> ! {
    todo!("0x1cb05c __ZNSt5dequeIPhSaIS0_EE15_M_pop_back_auxEv")
}

#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_destroy_nodes(unsigned char ***,unsigned char ***)")]
// 0x1cb090 — __ZNSt11_Deque_baseIPhSaIS0_EE16_M_destroy_nodesEPPS0_S4_
// was: std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_destroy_nodes(unsigned char ***,unsigned char ***)
pub fn stub_1cb090() -> ! {
    todo!("0x1cb090 __ZNSt11_Deque_baseIPhSaIS0_EE16_M_destroy_nodesEPPS0_S4_")
}

#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::~_Deque_base()")]
// 0x1cb180 — __ZNSt11_Deque_baseIPhSaIS0_EED2Ev
// was: std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::~_Deque_base()
pub fn stub_1cb180() -> ! {
    todo!("0x1cb180 __ZNSt11_Deque_baseIPhSaIS0_EED2Ev")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::~deque()")]
// 0x1cb1b4 — __ZNSt5dequeIPhSaIS0_EED2Ev
// was: std::deque<unsigned char *,std::allocator<unsigned char *>>::~deque()
pub fn stub_1cb1b4() -> ! {
    todo!("0x1cb1b4 __ZNSt5dequeIPhSaIS0_EED2Ev")
}

#[doc(alias = "std::map<unsigned int,int,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::operator[](unsigned int const&)")]
// 0x1cb224 — __ZNSt3mapIjiSt4lessIjESaISt4pairIKjiEEEixERS3_
// was: std::map<unsigned int,int,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::operator[](unsigned int const&)
pub fn stub_1cb224() -> ! {
    todo!("0x1cb224 __ZNSt3mapIjiSt4lessIjESaISt4pairIKjiEEEixERS3_")
}

#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_create_nodes(unsigned short **,unsigned short **)")]
// 0x1cb290 — __ZNSt11_Deque_baseItSaItEE15_M_create_nodesEPPtS3_
// was: std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_create_nodes(unsigned short **,unsigned short **)
pub fn stub_1cb290() -> ! {
    todo!("0x1cb290 __ZNSt11_Deque_baseItSaItEE15_M_create_nodesEPPtS3_")
}

#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_initialize_map(unsigned long)")]
// 0x1cb510 — __ZNSt11_Deque_baseItSaItEE17_M_initialize_mapEm
// was: std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_initialize_map(unsigned long)
pub fn stub_1cb510() -> ! {
    todo!("0x1cb510 __ZNSt11_Deque_baseItSaItEE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_Deque_base(std::allocator<unsigned short> const&,unsigned long)")]
// 0x1cb6e0 — __ZNSt11_Deque_baseItSaItEEC2ERKS0_m
// was: std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_Deque_base(std::allocator<unsigned short> const&,unsigned long)
pub fn stub_1cb6e0() -> ! {
    todo!("0x1cb6e0 __ZNSt11_Deque_baseItSaItEEC2ERKS0_m")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::deque(std::deque<unsigned short,std::allocator<unsigned short>> const&)")]
// 0x1cb7b0 — __ZNSt5dequeItSaItEEC2ERKS1_
// was: std::deque<unsigned short,std::allocator<unsigned short>>::deque(std::deque<unsigned short,std::allocator<unsigned short>> const&)
pub fn stub_1cb7b0() -> ! {
    todo!("0x1cb7b0 __ZNSt5dequeItSaItEEC2ERKS1_")
}

#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_create_nodes(TagLib::MDMODEL**,TagLib::MDMODEL**)")]
// 0x1cb878 — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE15_M_create_nodesEPPS1_S5_
// was: std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_create_nodes(TagLib::MDMODEL**,TagLib::MDMODEL**)
pub fn stub_1cb878() -> ! {
    todo!("0x1cb878 __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE15_M_create_nodesEPPS1_S5_")
}

#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_initialize_map(unsigned long)")]
// 0x1cbaf8 — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE17_M_initialize_mapEm
// was: std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_initialize_map(unsigned long)
pub fn stub_1cbaf8() -> ! {
    todo!("0x1cbaf8 __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_Deque_base(std::allocator<TagLib::MDMODEL> const&,unsigned long)")]
// 0x1cbcc8 — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EEC2ERKS2_m
// was: std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_Deque_base(std::allocator<TagLib::MDMODEL> const&,unsigned long)
pub fn stub_1cbcc8() -> ! {
    todo!("0x1cbcc8 __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EEC2ERKS2_m")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::deque(std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>> const&)")]
// 0x1cbd98 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EEC2ERKS3_
// was: std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::deque(std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>> const&)
pub fn stub_1cbd98() -> ! {
    todo!("0x1cbd98 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EEC2ERKS3_")
}

#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_create_nodes(unsigned char ***,unsigned char ***)")]
// 0x1cbe60 — __ZNSt11_Deque_baseIPhSaIS0_EE15_M_create_nodesEPPS0_S4_
// was: std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_create_nodes(unsigned char ***,unsigned char ***)
pub fn stub_1cbe60() -> ! {
    todo!("0x1cbe60 __ZNSt11_Deque_baseIPhSaIS0_EE15_M_create_nodesEPPS0_S4_")
}

#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_initialize_map(unsigned long)")]
// 0x1cc0e0 — __ZNSt11_Deque_baseIPhSaIS0_EE17_M_initialize_mapEm
// was: std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_initialize_map(unsigned long)
pub fn stub_1cc0e0() -> ! {
    todo!("0x1cc0e0 __ZNSt11_Deque_baseIPhSaIS0_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_Deque_base(std::allocator<unsigned char *> const&,unsigned long)")]
// 0x1cc2b0 — __ZNSt11_Deque_baseIPhSaIS0_EEC2ERKS1_m
// was: std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_Deque_base(std::allocator<unsigned char *> const&,unsigned long)
pub fn stub_1cc2b0() -> ! {
    todo!("0x1cc2b0 __ZNSt11_Deque_baseIPhSaIS0_EEC2ERKS1_m")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::deque(std::deque<unsigned char *,std::allocator<unsigned char *>> const&)")]
// 0x1cc380 — __ZNSt5dequeIPhSaIS0_EEC2ERKS2_
// was: std::deque<unsigned char *,std::allocator<unsigned char *>>::deque(std::deque<unsigned char *,std::allocator<unsigned char *>> const&)
pub fn stub_1cc380() -> ! {
    todo!("0x1cc380 __ZNSt5dequeIPhSaIS0_EEC2ERKS2_")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data_aux(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>)")]
// 0x1cc448 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE19_M_destroy_data_auxESt15_Deque_iteratorIS1_RS1_PS1_ES7_
// was: std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data_aux(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>)
pub fn stub_1cc448() -> ! {
    todo!("0x1cc448 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE19_M_destroy_data_auxESt15_Deque_iteratorIS1_RS1_PS1_ES7_")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data_dispatch(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::__false_type)")]
// 0x1cc450 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE24_M_destroy_data_dispatchESt15_Deque_iteratorIS1_RS1_PS1_ES7_St12__false_type
// was: std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data_dispatch(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::__false_type)
pub fn stub_1cc450() -> ! {
    todo!("0x1cc450 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE24_M_destroy_data_dispatchESt15_Deque_iteratorIS1_RS1_PS1_ES7_St12__false_type")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::allocator<TagLib::MDMODEL> const&)")]
// 0x1cc4b0 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE15_M_destroy_dataESt15_Deque_iteratorIS1_RS1_PS1_ES7_RKS2_
// was: std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::allocator<TagLib::MDMODEL> const&)
pub fn stub_1cc4b0() -> ! {
    todo!("0x1cc4b0 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE15_M_destroy_dataESt15_Deque_iteratorIS1_RS1_PS1_ES7_RKS2_")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::~deque()")]
// 0x1cc508 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EED2Ev
// was: std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::~deque()
pub fn stub_1cc508() -> ! {
    todo!("0x1cc508 __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EED2Ev")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::find(std::string const&)")]
// 0x232bf4 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::find(std::string const&)
pub fn stub_232bf4() -> ! {
    todo!("0x232bf4 __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_")
}

#[doc(alias = "std::vector<OIS::MultiTouchState,std::allocator<OIS::MultiTouchState>>::size(void)const")]
// 0x2331f8 — __ZNKSt6vectorIN3OIS15MultiTouchStateESaIS1_EE4sizeEv
// was: std::vector<OIS::MultiTouchState,std::allocator<OIS::MultiTouchState>>::size(void)const
pub fn stub_2331f8() -> ! {
    todo!("0x2331f8 __ZNKSt6vectorIN3OIS15MultiTouchStateESaIS1_EE4sizeEv")
}

#[doc(alias = "OIS::MultiTouchState * std::uninitialized_copy<OIS::MultiTouchState *,OIS::MultiTouchState *>(OIS::MultiTouchState *,OIS::MultiTouchState *,OIS::MultiTouchState *)")]
// 0x233214 — __ZSt18uninitialized_copyIPN3OIS15MultiTouchStateES2_ET0_T_S4_S3_
// was: OIS::MultiTouchState * std::uninitialized_copy<OIS::MultiTouchState *,OIS::MultiTouchState *>(OIS::MultiTouchState *,OIS::MultiTouchState *,OIS::MultiTouchState *)
pub fn stub_233214() -> ! {
    todo!("0x233214 __ZSt18uninitialized_copyIPN3OIS15MultiTouchStateES2_ET0_T_S4_S3_")
}

#[doc(alias = "OIS::MultiTouchState * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<OIS::MultiTouchState *,OIS::MultiTouchState *>(OIS::MultiTouchState *,OIS::MultiTouchState *,OIS::MultiTouchState *)")]
// 0x23344c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3OIS15MultiTouchStateES5_EET0_T_S7_S6_
// was: OIS::MultiTouchState * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<OIS::MultiTouchState *,OIS::MultiTouchState *>(OIS::MultiTouchState *,OIS::MultiTouchState *,OIS::MultiTouchState *)
pub fn stub_23344c() -> ! {
    todo!("0x23344c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3OIS15MultiTouchStateES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::vector<OIS::MultiTouchState,std::allocator<OIS::MultiTouchState>>::_M_insert_aux(__gnu_cxx::__normal_iterator<OIS::MultiTouchState*,std::vector<OIS::MultiTouchState,std::allocator<OIS::MultiTouchState>>>,OIS::MultiTouchState const&)")]
// 0x233714 — __ZNSt6vectorIN3OIS15MultiTouchStateESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// was: std::vector<OIS::MultiTouchState,std::allocator<OIS::MultiTouchState>>::_M_insert_aux(__gnu_cxx::__normal_iterator<OIS::MultiTouchState*,std::vector<OIS::MultiTouchState,std::allocator<OIS::MultiTouchState>>>,OIS::MultiTouchState const&)
pub fn stub_233714() -> ! {
    todo!("0x233714 __ZNSt6vectorIN3OIS15MultiTouchStateESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<OIS::MultiTouchState,std::allocator<OIS::MultiTouchState>>::push_back(OIS::MultiTouchState const&)")]
// 0x233920 — __ZNSt6vectorIN3OIS15MultiTouchStateESaIS1_EE9push_backERKS1_
// was: std::vector<OIS::MultiTouchState,std::allocator<OIS::MultiTouchState>>::push_back(OIS::MultiTouchState const&)
pub fn stub_233920() -> ! {
    todo!("0x233920 __ZNSt6vectorIN3OIS15MultiTouchStateESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "boost::detail::get_once_per_thread_epoch(void)")]
// 0x235884 — __ZN5boost6detail25get_once_per_thread_epochEv
// was: boost::detail::get_once_per_thread_epoch(void)
pub fn stub_235884() -> ! {
    todo!("0x235884 __ZN5boost6detail25get_once_per_thread_epochEv")
}

#[doc(alias = "boost::detail::thread_data_base::~thread_data_base()")]
// 0x2358fc — __ZN5boost6detail16thread_data_baseD0Ev
// was: boost::detail::thread_data_base::~thread_data_base()
pub fn stub_2358fc() -> ! {
    todo!("0x2358fc __ZN5boost6detail16thread_data_baseD0Ev")
}

#[doc(alias = "boost::detail::thread_data_base::~thread_data_base()")]
// 0x235910 — __ZN5boost6detail16thread_data_baseD1Ev
// was: boost::detail::thread_data_base::~thread_data_base()
pub fn stub_235910() -> ! {
    todo!("0x235910 __ZN5boost6detail16thread_data_baseD1Ev")
}

#[doc(alias = "boost::detail::thread_data_base::~thread_data_base()")]
// 0x23591c — __ZN5boost6detail16thread_data_baseD2Ev
// was: boost::detail::thread_data_base::~thread_data_base()
pub fn stub_23591c() -> ! {
    todo!("0x23591c __ZN5boost6detail16thread_data_baseD2Ev")
}

#[doc(alias = "boost::detail::get_current_thread_data(void)")]
// 0x235d7c — __ZN5boost6detail23get_current_thread_dataEv
// was: boost::detail::get_current_thread_data(void)
pub fn stub_235d7c() -> ! {
    todo!("0x235d7c __ZN5boost6detail23get_current_thread_dataEv")
}

#[doc(alias = "boost::detail::anonymous_namespace::create_current_thread_tls_key(void)")]
// 0x235da4 — __ZN5boost6detail12_GLOBAL__N_129create_current_thread_tls_keyEv
// was: boost::detail::anonymous_namespace::create_current_thread_tls_key(void)
pub fn stub_235da4() -> ! {
    todo!("0x235da4 __ZN5boost6detail12_GLOBAL__N_129create_current_thread_tls_keyEv")
}

#[doc(alias = "boost::thread::start_thread_noexcept(void)")]
// 0x235dc4 — __ZN5boost6thread21start_thread_noexceptEv
// was: boost::thread::start_thread_noexcept(void)
pub fn stub_235dc4() -> ! {
    todo!("0x235dc4 __ZN5boost6thread21start_thread_noexceptEv")
}

#[doc(alias = "boost::thread::join_noexcept(void)")]
// 0x2360c4 — __ZN5boost6thread13join_noexceptEv
// was: boost::thread::join_noexcept(void)
pub fn stub_2360c4() -> ! {
    todo!("0x2360c4 __ZN5boost6thread13join_noexceptEv")
}

#[doc(alias = "boost::thread::do_try_join_until_noexcept(timespec const&,bool &)")]
// 0x236318 — __ZN5boost6thread26do_try_join_until_noexceptERK8timespecRb
// was: boost::thread::do_try_join_until_noexcept(timespec const&,bool &)
pub fn stub_236318() -> ! {
    todo!("0x236318 __ZN5boost6thread26do_try_join_until_noexceptERK8timespecRb")
}

#[doc(alias = "boost::thread::detach(void)")]
// 0x236598 — __ZN5boost6thread6detachEv
// was: boost::thread::detach(void)
pub fn stub_236598() -> ! {
    todo!("0x236598 __ZN5boost6thread6detachEv")
}

#[doc(alias = "boost::this_thread::hiden::sleep_until(timespec const&)")]
// 0x2366b0 — __ZN5boost11this_thread5hiden11sleep_untilERK8timespec
// was: boost::this_thread::hiden::sleep_until(timespec const&)
pub fn stub_2366b0() -> ! {
    todo!("0x2366b0 __ZN5boost11this_thread5hiden11sleep_untilERK8timespec")
}

#[doc(alias = "boost::thread::native_handle(void)")]
// 0x2368cc — __ZN5boost6thread13native_handleEv
// was: boost::thread::native_handle(void)
pub fn stub_2368cc() -> ! {
    todo!("0x2368cc __ZN5boost6thread13native_handleEv")
}

#[doc(alias = "boost::this_thread::interruption_point(void)")]
// 0x236a00 — __ZN5boost11this_thread18interruption_pointEv
// was: boost::this_thread::interruption_point(void)
pub fn stub_236a00() -> ! {
    todo!("0x236a00 __ZN5boost11this_thread18interruption_pointEv")
}

#[doc(alias = "boost::this_thread::disable_interruption::disable_interruption(void)")]
// 0x236b14 — __ZN5boost11this_thread20disable_interruptionC1Ev
// was: boost::this_thread::disable_interruption::disable_interruption(void)
pub fn stub_236b14() -> ! {
    todo!("0x236b14 __ZN5boost11this_thread20disable_interruptionC1Ev")
}

#[doc(alias = "boost::this_thread::disable_interruption::~disable_interruption()")]
// 0x236c14 — __ZN5boost11this_thread20disable_interruptionD1Ev
// was: boost::this_thread::disable_interruption::~disable_interruption()
pub fn stub_236c14() -> ! {
    todo!("0x236c14 __ZN5boost11this_thread20disable_interruptionD1Ev")
}

#[doc(alias = "boost::anonymous_namespace::get_or_make_current_thread_data(void)")]
// 0x236d04 — __ZN5boost12_GLOBAL__N_131get_or_make_current_thread_dataEv
// was: boost::anonymous_namespace::get_or_make_current_thread_data(void)
pub fn stub_236d04() -> ! {
    todo!("0x236d04 __ZN5boost12_GLOBAL__N_131get_or_make_current_thread_dataEv")
}

#[doc(alias = "boost::detail::get_tss_data(void const*)")]
// 0x236ec0 — __ZN5boost6detail12get_tss_dataEPKv
// was: boost::detail::get_tss_data(void const*)
pub fn stub_236ec0() -> ! {
    todo!("0x236ec0 __ZN5boost6detail12get_tss_dataEPKv")
}

#[doc(alias = "boost::detail::add_new_tss_node(void const*,boost::shared_ptr<boost::detail::tss_cleanup_function>,void *)")]
// 0x236f30 — __ZN5boost6detail16add_new_tss_nodeEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPv
// was: boost::detail::add_new_tss_node(void const*,boost::shared_ptr<boost::detail::tss_cleanup_function>,void *)
pub fn stub_236f30() -> ! {
    todo!("0x236f30 __ZN5boost6detail16add_new_tss_nodeEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPv")
}

#[doc(alias = "boost::detail::set_tss_data(void const*,boost::shared_ptr<boost::detail::tss_cleanup_function>,void *,bool)")]
// 0x237130 — __ZN5boost6detail12set_tss_dataEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPvb
// was: boost::detail::set_tss_data(void const*,boost::shared_ptr<boost::detail::tss_cleanup_function>,void *,bool)
pub fn stub_237130() -> ! {
    todo!("0x237130 __ZN5boost6detail12set_tss_dataEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPvb")
}

#[doc(alias = "boost::anonymous_namespace::externally_launched_thread::~externally_launched_thread()")]
// 0x237348 — __ZN5boost12_GLOBAL__N_126externally_launched_threadD1Ev
// was: boost::anonymous_namespace::externally_launched_thread::~externally_launched_thread()
pub fn stub_237348() -> ! {
    todo!("0x237348 __ZN5boost12_GLOBAL__N_126externally_launched_threadD1Ev")
}

#[doc(alias = "boost::anonymous_namespace::externally_launched_thread::~externally_launched_thread()")]
// 0x237354 — __ZN5boost12_GLOBAL__N_126externally_launched_threadD0Ev
// was: boost::anonymous_namespace::externally_launched_thread::~externally_launched_thread()
pub fn stub_237354() -> ! {
    todo!("0x237354 __ZN5boost12_GLOBAL__N_126externally_launched_threadD0Ev")
}

#[doc(alias = "boost::anonymous_namespace::externally_launched_thread::run(void)")]
// 0x237368 — __ZN5boost12_GLOBAL__N_126externally_launched_thread3runEv
// was: boost::anonymous_namespace::externally_launched_thread::run(void)
pub fn stub_237368() -> ! {
    todo!("0x237368 __ZN5boost12_GLOBAL__N_126externally_launched_thread3runEv")
}

#[doc(alias = "boost::anonymous_namespace::externally_launched_thread::notify_all_at_thread_exit(boost::condition_variable *,boost::mutex *)")]
// 0x23736c — __ZN5boost12_GLOBAL__N_126externally_launched_thread25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE
// was: boost::anonymous_namespace::externally_launched_thread::notify_all_at_thread_exit(boost::condition_variable *,boost::mutex *)
pub fn stub_23736c() -> ! {
    todo!("0x23736c __ZN5boost12_GLOBAL__N_126externally_launched_thread25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE")
}

#[doc(alias = "boost::shared_ptr<boost::detail::thread_data_base>::operator=(boost::shared_ptr<boost::detail::thread_data_base> const&)")]
// 0x2374bc — __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEaSERKS3_
// was: boost::shared_ptr<boost::detail::thread_data_base>::operator=(boost::shared_ptr<boost::detail::thread_data_base> const&)
pub fn stub_2374bc() -> ! {
    todo!("0x2374bc __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEaSERKS3_")
}

#[doc(alias = "boost::shared_ptr<boost::detail::tss_cleanup_function>::operator=(boost::shared_ptr<boost::detail::tss_cleanup_function> const&)")]
// 0x2375b0 — __ZN5boost10shared_ptrINS_6detail20tss_cleanup_functionEEaSERKS3_
// was: boost::shared_ptr<boost::detail::tss_cleanup_function>::operator=(boost::shared_ptr<boost::detail::tss_cleanup_function> const&)
pub fn stub_2375b0() -> ! {
    todo!("0x2375b0 __ZN5boost10shared_ptrINS_6detail20tss_cleanup_functionEEaSERKS3_")
}

#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::erase(std::_Rb_tree_iterator<std::pair<void const* const,boost::detail::tss_data_node>>,std::_Rb_tree_iterator<std::pair<void const* const,boost::detail::tss_data_node>>)")]
// 0x2376a4 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_
// was: std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::erase(std::_Rb_tree_iterator<std::pair<void const* const,boost::detail::tss_data_node>>,std::_Rb_tree_iterator<std::pair<void const* const,boost::detail::tss_data_node>>)
pub fn stub_2376a4() -> ! {
    todo!("0x2376a4 __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_")
}

#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_erase(std::_Rb_tree_node<std::pair<void const* const,boost::detail::tss_data_node>> *)")]
// 0x237798 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// was: std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_erase(std::_Rb_tree_node<std::pair<void const* const,boost::detail::tss_data_node>> *)
pub fn stub_237798() -> ! {
    todo!("0x237798 __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_insert_unique(std::pair<void const* const,boost::detail::tss_data_node> const&)")]
// 0x237848 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_
// was: std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_insert_unique(std::pair<void const* const,boost::detail::tss_data_node> const&)
pub fn stub_237848() -> ! {
    todo!("0x237848 __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_create_node(std::pair<void const* const,boost::detail::tss_data_node> const&)")]
// 0x2378fc — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE14_M_create_nodeERKS7_
// was: std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_create_node(std::pair<void const* const,boost::detail::tss_data_node> const&)
pub fn stub_2378fc() -> ! {
    todo!("0x2378fc __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE14_M_create_nodeERKS7_")
}

#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data_base>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data_base *)const")]
// 0x2379ec — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data_base>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data_base *)const
pub fn stub_2379ec() -> ! {
    todo!("0x2379ec __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::~sp_counted_impl_p()")]
// 0x237b40 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED1Ev
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::~sp_counted_impl_p()
pub fn stub_237b40() -> ! {
    todo!("0x237b40 __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::~sp_counted_impl_p()")]
// 0x237b44 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED0Ev
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::~sp_counted_impl_p()
pub fn stub_237b44() -> ! {
    todo!("0x237b44 __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::dispose(void)")]
// 0x237b50 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE7disposeEv
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::dispose(void)
pub fn stub_237b50() -> ! {
    todo!("0x237b50 __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_deleter(std::type_info const&)")]
// 0x237b64 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_deleter(std::type_info const&)
pub fn stub_237b64() -> ! {
    todo!("0x237b64 __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_untyped_deleter(void)")]
// 0x237b68 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_untyped_deleter(void)
pub fn stub_237b68() -> ! {
    todo!("0x237b68 __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::future_object_base::mark_finished_internal(boost::unique_lock<boost::mutex> &)")]
// 0x237b6c — __ZN5boost6detail18future_object_base22mark_finished_internalERNS_11unique_lockINS_5mutexEEE
// was: boost::detail::future_object_base::mark_finished_internal(boost::unique_lock<boost::mutex> &)
pub fn stub_237b6c() -> ! {
    todo!("0x237b6c __ZN5boost6detail18future_object_base22mark_finished_internalERNS_11unique_lockINS_5mutexEEE")
}

#[doc(alias = "boost::filesystem::detail::symlink_status(boost::filesystem::path const&,boost::system::error_code *)")]
// 0x237d60 — __ZN5boost10filesystem6detail14symlink_statusERKNS0_4pathEPNS_6system10error_codeE
// was: boost::filesystem::detail::symlink_status(boost::filesystem::path const&,boost::system::error_code *)
pub fn stub_237d60() -> ! {
    todo!("0x237d60 __ZN5boost10filesystem6detail14symlink_statusERKNS0_4pathEPNS_6system10error_codeE")
}

#[doc(alias = "boost::filesystem::detail::current_path(boost::system::error_code *)")]
// 0x237fa4 — __ZN5boost10filesystem6detail12current_pathEPNS_6system10error_codeE
// was: boost::filesystem::detail::current_path(boost::system::error_code *)
pub fn stub_237fa4() -> ! {
    todo!("0x237fa4 __ZN5boost10filesystem6detail12current_pathEPNS_6system10error_codeE")
}

#[doc(alias = "anonymous_namespace::error(bool,boost::filesystem::path const&,boost::system::error_code *,std::string const&)")]
// 0x238258 — __ZN12_GLOBAL__N_15errorEbRKN5boost10filesystem4pathEPNS0_6system10error_codeERKSs
// was: anonymous_namespace::error(bool,boost::filesystem::path const&,boost::system::error_code *,std::string const&)
pub fn stub_238258() -> ! {
    todo!("0x238258 __ZN12_GLOBAL__N_15errorEbRKN5boost10filesystem4pathEPNS0_6system10error_codeERKSs")
}

#[doc(alias = "boost::filesystem::detail::initial_path(boost::system::error_code *)")]
// 0x23837c — __ZN5boost10filesystem6detail12initial_pathEPNS_6system10error_codeE
// was: boost::filesystem::detail::initial_path(boost::system::error_code *)
pub fn stub_23837c() -> ! {
    todo!("0x23837c __ZN5boost10filesystem6detail12initial_pathEPNS_6system10error_codeE")
}

#[doc(alias = "boost::filesystem::detail::is_empty(boost::filesystem::path const&,boost::system::error_code *)")]
// 0x23852c — __ZN5boost10filesystem6detail8is_emptyERKNS0_4pathEPNS_6system10error_codeE
// was: boost::filesystem::detail::is_empty(boost::filesystem::path const&,boost::system::error_code *)
pub fn stub_23852c() -> ! {
    todo!("0x23852c __ZN5boost10filesystem6detail8is_emptyERKNS0_4pathEPNS_6system10error_codeE")
}

#[doc(alias = "boost::filesystem::detail::remove(boost::filesystem::path const&,boost::system::error_code *)")]
// 0x2386d4 — __ZN5boost10filesystem6detail6removeERKNS0_4pathEPNS_6system10error_codeE
// was: boost::filesystem::detail::remove(boost::filesystem::path const&,boost::system::error_code *)
pub fn stub_2386d4() -> ! {
    todo!("0x2386d4 __ZN5boost10filesystem6detail6removeERKNS0_4pathEPNS_6system10error_codeE")
}

#[doc(alias = "anonymous_namespace::remove_file_or_directory(boost::filesystem::path const&,boost::filesystem::file_type,boost::system::error_code *)")]
// 0x2388a8 — __ZN12_GLOBAL__N_124remove_file_or_directoryERKN5boost10filesystem4pathENS1_9file_typeEPNS0_6system10error_codeE
// was: anonymous_namespace::remove_file_or_directory(boost::filesystem::path const&,boost::filesystem::file_type,boost::system::error_code *)
pub fn stub_2388a8() -> ! {
    todo!("0x2388a8 __ZN12_GLOBAL__N_124remove_file_or_directoryERKN5boost10filesystem4pathENS1_9file_typeEPNS0_6system10error_codeE")
}

#[doc(alias = "boost::filesystem::detail::status(boost::filesystem::path const&,boost::system::error_code *)")]
// 0x238adc — __ZN5boost10filesystem6detail6statusERKNS0_4pathEPNS_6system10error_codeE
// was: boost::filesystem::detail::status(boost::filesystem::path const&,boost::system::error_code *)
pub fn stub_238adc() -> ! {
    todo!("0x238adc __ZN5boost10filesystem6detail6statusERKNS0_4pathEPNS_6system10error_codeE")
}

#[doc(alias = "boost::filesystem::detail::system_complete(boost::filesystem::path const&,boost::system::error_code *)")]
// 0x238d18 — __ZN5boost10filesystem6detail15system_completeERKNS0_4pathEPNS_6system10error_codeE
// was: boost::filesystem::detail::system_complete(boost::filesystem::path const&,boost::system::error_code *)
pub fn stub_238d18() -> ! {
    todo!("0x238d18 __ZN5boost10filesystem6detail15system_completeERKNS0_4pathEPNS_6system10error_codeE")
}

#[doc(alias = "boost::filesystem::directory_entry::m_get_status(boost::system::error_code *)const")]
// 0x238f14 — __ZNK5boost10filesystem15directory_entry12m_get_statusEPNS_6system10error_codeE
// was: boost::filesystem::directory_entry::m_get_status(boost::system::error_code *)const
pub fn stub_238f14() -> ! {
    todo!("0x238f14 __ZNK5boost10filesystem15directory_entry12m_get_statusEPNS_6system10error_codeE")
}

#[doc(alias = "boost::filesystem::detail::dir_itr_close(void *&,void *&)")]
// 0x238f80 — __ZN5boost10filesystem6detail13dir_itr_closeERPvS3_
// was: boost::filesystem::detail::dir_itr_close(void *&,void *&)
pub fn stub_238f80() -> ! {
    todo!("0x238f80 __ZN5boost10filesystem6detail13dir_itr_closeERPvS3_")
}

#[doc(alias = "boost::filesystem::detail::directory_iterator_construct(boost::filesystem::directory_iterator &,boost::filesystem::path const&,boost::system::error_code *)")]
// 0x238fd4 — __ZN5boost10filesystem6detail28directory_iterator_constructERNS0_18directory_iteratorERKNS0_4pathEPNS_6system10error_codeE
// was: boost::filesystem::detail::directory_iterator_construct(boost::filesystem::directory_iterator &,boost::filesystem::path const&,boost::system::error_code *)
pub fn stub_238fd4() -> ! {
    todo!("0x238fd4 __ZN5boost10filesystem6detail28directory_iterator_constructERNS0_18directory_iteratorERKNS0_4pathEPNS_6system10error_codeE")
}

#[doc(alias = "boost::filesystem::detail::directory_iterator_increment(boost::filesystem::directory_iterator &,boost::system::error_code *)")]
// 0x239668 — __ZN5boost10filesystem6detail28directory_iterator_incrementERNS0_18directory_iteratorEPNS_6system10error_codeE
// was: boost::filesystem::detail::directory_iterator_increment(boost::filesystem::directory_iterator &,boost::system::error_code *)
pub fn stub_239668() -> ! {
    todo!("0x239668 __ZN5boost10filesystem6detail28directory_iterator_incrementERNS0_18directory_iteratorEPNS_6system10error_codeE")
}

#[doc(alias = "boost::filesystem::directory_iterator::~directory_iterator()")]
// 0x239b34 — __ZN5boost10filesystem18directory_iteratorD1Ev
// was: boost::filesystem::directory_iterator::~directory_iterator()
pub fn stub_239b34() -> ! {
    todo!("0x239b34 __ZN5boost10filesystem18directory_iteratorD1Ev")
}

#[doc(alias = "boost::filesystem::filesystem_error::~filesystem_error()")]
// 0x239bc8 — __ZN5boost10filesystem16filesystem_errorD1Ev
// was: boost::filesystem::filesystem_error::~filesystem_error()
pub fn stub_239bc8() -> ! {
    todo!("0x239bc8 __ZN5boost10filesystem16filesystem_errorD1Ev")
}

#[doc(alias = "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::system::error_code)")]
// 0x239cc8 — __ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE
// was: boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::system::error_code)
pub fn stub_239cc8() -> ! {
    todo!("0x239cc8 __ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE")
}

#[doc(alias = "boost::filesystem::filesystem_error::~filesystem_error()")]
// 0x239e90 — __ZN5boost10filesystem16filesystem_errorD0Ev
// was: boost::filesystem::filesystem_error::~filesystem_error()
pub fn stub_239e90() -> ! {
    todo!("0x239e90 __ZN5boost10filesystem16filesystem_errorD0Ev")
}

#[doc(alias = "boost::filesystem::filesystem_error::what(void)const")]
// 0x239f94 — __ZNK5boost10filesystem16filesystem_error4whatEv
// was: boost::filesystem::filesystem_error::what(void)const
pub fn stub_239f94() -> ! {
    todo!("0x239f94 __ZNK5boost10filesystem16filesystem_error4whatEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<boost::filesystem::filesystem_error::m_imp,boost::filesystem::filesystem_error::m_imp>(boost::shared_ptr<boost::filesystem::filesystem_error::m_imp> *,boost::filesystem::filesystem_error::m_imp *,boost::detail::shared_count &)")]
// 0x23a11c — __ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// was: void boost::detail::sp_pointer_construct<boost::filesystem::filesystem_error::m_imp,boost::filesystem::filesystem_error::m_imp>(boost::shared_ptr<boost::filesystem::filesystem_error::m_imp> *,boost::filesystem::filesystem_error::m_imp *,boost::detail::shared_count &)
pub fn stub_23a11c() -> ! {
    todo!("0x23a11c __ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::~sp_counted_impl_p()")]
// 0x23a2bc — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED1Ev
// was: boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::~sp_counted_impl_p()
pub fn stub_23a2bc() -> ! {
    todo!("0x23a2bc __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::~sp_counted_impl_p()")]
// 0x23a2c0 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED0Ev
// was: boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::~sp_counted_impl_p()
pub fn stub_23a2c0() -> ! {
    todo!("0x23a2c0 __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::dispose(void)")]
// 0x23a2cc — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE7disposeEv
// was: boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::dispose(void)
pub fn stub_23a2cc() -> ! {
    todo!("0x23a2cc __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_deleter(std::type_info const&)")]
// 0x23a38c — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_deleter(std::type_info const&)
pub fn stub_23a38c() -> ! {
    todo!("0x23a38c __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_untyped_deleter(void)")]
// 0x23a390 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_untyped_deleter(void)
pub fn stub_23a390() -> ! {
    todo!("0x23a390 __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::filesystem::path const&,boost::system::error_code)")]
// 0x23a394 — __ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE
// was: boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::filesystem::path const&,boost::system::error_code)
pub fn stub_23a394() -> ! {
    todo!("0x23a394 __ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE")
}

#[doc(alias = "boost::filesystem::path::operator/=(boost::filesystem::path const&)")]
// 0x23a630 — __ZN5boost10filesystem4pathdVERKS1_
// was: boost::filesystem::path::operator/=(boost::filesystem::path const&)
pub fn stub_23a630() -> ! {
    todo!("0x23a630 __ZN5boost10filesystem4pathdVERKS1_")
}

#[doc(alias = "boost::filesystem::path::m_append_separator_if_needed(void)")]
// 0x23a7b8 — __ZN5boost10filesystem4path28m_append_separator_if_neededEv
// was: boost::filesystem::path::m_append_separator_if_needed(void)
pub fn stub_23a7b8() -> ! {
    todo!("0x23a7b8 __ZN5boost10filesystem4path28m_append_separator_if_neededEv")
}

#[doc(alias = "boost::filesystem::path::operator/=(char const*)")]
// 0x23a830 — __ZN5boost10filesystem4pathdVEPKc
// was: boost::filesystem::path::operator/=(char const*)
pub fn stub_23a830() -> ! {
    todo!("0x23a830 __ZN5boost10filesystem4pathdVEPKc")
}

#[doc(alias = "boost::filesystem::path::m_erase_redundant_separator(unsigned long)")]
// 0x23a9d4 — __ZN5boost10filesystem4path27m_erase_redundant_separatorEm
// was: boost::filesystem::path::m_erase_redundant_separator(unsigned long)
pub fn stub_23a9d4() -> ! {
    todo!("0x23a9d4 __ZN5boost10filesystem4path27m_erase_redundant_separatorEm")
}

#[doc(alias = "boost::filesystem::path::remove_filename(void)")]
// 0x23aa2c — __ZN5boost10filesystem4path15remove_filenameEv
// was: boost::filesystem::path::remove_filename(void)
pub fn stub_23aa2c() -> ! {
    todo!("0x23aa2c __ZN5boost10filesystem4path15remove_filenameEv")
}

#[doc(alias = "boost::filesystem::path::m_parent_path_end(void)const")]
// 0x23aa60 — __ZNK5boost10filesystem4path17m_parent_path_endEv
// was: boost::filesystem::path::m_parent_path_end(void)const
pub fn stub_23aa60() -> ! {
    todo!("0x23aa60 __ZNK5boost10filesystem4path17m_parent_path_endEv")
}

#[doc(alias = "boost::filesystem::path::root_directory(void)const")]
// 0x23ab64 — __ZNK5boost10filesystem4path14root_directoryEv
// was: boost::filesystem::path::root_directory(void)const
pub fn stub_23ab64() -> ! {
    todo!("0x23ab64 __ZNK5boost10filesystem4path14root_directoryEv")
}

#[doc(alias = "boost::filesystem::path::parent_path(void)const")]
// 0x23abe8 — __ZNK5boost10filesystem4path11parent_pathEv
// was: boost::filesystem::path::parent_path(void)const
pub fn stub_23abe8() -> ! {
    todo!("0x23abe8 __ZNK5boost10filesystem4path11parent_pathEv")
}

#[doc(alias = "boost::filesystem::path::codecvt(void)")]
// 0x23ac1c — __ZN5boost10filesystem4path7codecvtEv
// was: boost::filesystem::path::codecvt(void)
pub fn stub_23ac1c() -> ! {
    todo!("0x23ac1c __ZN5boost10filesystem4path7codecvtEv")
}

#[doc(alias = "boost::filesystem::path::~path()")]
// 0x23ac2c — __ZN5boost10filesystem4pathD1Ev
// was: boost::filesystem::path::~path()
pub fn stub_23ac2c() -> ! {
    todo!("0x23ac2c __ZN5boost10filesystem4pathD1Ev")
}

#[doc(alias = "std::locale::locale<boost::filesystem::detail::utf8_codecvt_facet>(std::locale const&,boost::filesystem::detail::utf8_codecvt_facet *)")]
// 0x23ac78 — __ZNSt6localeC2IN5boost10filesystem6detail18utf8_codecvt_facetEEERKS_PT_
// was: std::locale::locale<boost::filesystem::detail::utf8_codecvt_facet>(std::locale const&,boost::filesystem::detail::utf8_codecvt_facet *)
pub fn stub_23ac78() -> ! {
    todo!("0x23ac78 __ZNSt6localeC2IN5boost10filesystem6detail18utf8_codecvt_facetEEERKS_PT_")
}

#[doc(alias = "boost::filesystem::path::path<char const*>(char const*,char const*)")]
// 0x23adc4 — __ZN5boost10filesystem4pathC2IPKcEET_S5_
// was: boost::filesystem::path::path<char const*>(char const*,char const*)
pub fn stub_23adc4() -> ! {
    todo!("0x23adc4 __ZN5boost10filesystem4pathC2IPKcEET_S5_")
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_unshift(__mbstate_t &,char *,char *,char *&)const")]
// 0x23af94 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet10do_unshiftER11__mbstate_tPcS5_RS5_
// was: boost::filesystem::detail::utf8_codecvt_facet::do_unshift(__mbstate_t &,char *,char *,char *&)const
pub fn stub_23af94() -> ! {
    todo!("0x23af94 __ZNK5boost10filesystem6detail18utf8_codecvt_facet10do_unshiftER11__mbstate_tPcS5_RS5_")
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_encoding(void)const")]
// 0x23af9c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet11do_encodingEv
// was: boost::filesystem::detail::utf8_codecvt_facet::do_encoding(void)const
pub fn stub_23af9c() -> ! {
    todo!("0x23af9c __ZNK5boost10filesystem6detail18utf8_codecvt_facet11do_encodingEv")
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_always_noconv(void)const")]
// 0x23afa0 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet16do_always_noconvEv
// was: boost::filesystem::detail::utf8_codecvt_facet::do_always_noconv(void)const
pub fn stub_23afa0() -> ! {
    todo!("0x23afa0 __ZNK5boost10filesystem6detail18utf8_codecvt_facet16do_always_noconvEv")
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_max_length(void)const")]
// 0x23afa4 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet13do_max_lengthEv
// was: boost::filesystem::detail::utf8_codecvt_facet::do_max_length(void)const
pub fn stub_23afa4() -> ! {
    todo!("0x23afa4 __ZNK5boost10filesystem6detail18utf8_codecvt_facet13do_max_lengthEv")
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_in(__mbstate_t &,char const*,char const*,char const*&,wchar_t *,wchar_t *,wchar_t *&)const")]
// 0x23b14c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet5do_inER11__mbstate_tPKcS6_RS6_PwS8_RS8_
// was: boost::filesystem::detail::utf8_codecvt_facet::do_in(__mbstate_t &,char const*,char const*,char const*&,wchar_t *,wchar_t *,wchar_t *&)const
pub fn stub_23b14c() -> ! {
    todo!("0x23b14c __ZNK5boost10filesystem6detail18utf8_codecvt_facet5do_inER11__mbstate_tPKcS6_RS6_PwS8_RS8_")
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_out(__mbstate_t &,wchar_t const*,wchar_t const*,wchar_t const*&,char *,char *,char *&)const")]
// 0x23b2d0 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet6do_outER11__mbstate_tPKwS6_RS6_PcS8_RS8_
// was: boost::filesystem::detail::utf8_codecvt_facet::do_out(__mbstate_t &,wchar_t const*,wchar_t const*,wchar_t const*&,char *,char *,char *&)const
pub fn stub_23b2d0() -> ! {
    todo!("0x23b2d0 __ZNK5boost10filesystem6detail18utf8_codecvt_facet6do_outER11__mbstate_tPKwS6_RS6_PcS8_RS8_")
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_length(__mbstate_t const&,char const*,char const*,unsigned long)const")]
// 0x23b43c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet9do_lengthERK11__mbstate_tPKcS7_m
// was: boost::filesystem::detail::utf8_codecvt_facet::do_length(__mbstate_t const&,char const*,char const*,unsigned long)const
pub fn stub_23b43c() -> ! {
    todo!("0x23b43c __ZNK5boost10filesystem6detail18utf8_codecvt_facet9do_lengthERK11__mbstate_tPKcS7_m")
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()")]
// 0x23b4ac — __ZN5boost10filesystem6detail18utf8_codecvt_facetD1Ev
// was: boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()
pub fn stub_23b4ac() -> ! {
    todo!("0x23b4ac __ZN5boost10filesystem6detail18utf8_codecvt_facetD1Ev")
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()")]
// 0x23b4b8 — __ZN5boost10filesystem6detail18utf8_codecvt_facetD0Ev
// was: boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()
pub fn stub_23b4b8() -> ! {
    todo!("0x23b4b8 __ZN5boost10filesystem6detail18utf8_codecvt_facetD0Ev")
}

#[doc(alias = "boost::system::generic_category(void)")]
// 0x23b4cc — __ZN5boost6system16generic_categoryEv
// was: boost::system::generic_category(void)
pub fn stub_23b4cc() -> ! {
    todo!("0x23b4cc __ZN5boost6system16generic_categoryEv")
}

#[doc(alias = "boost::system::system_category(void)")]
// 0x23b508 — __ZN5boost6system15system_categoryEv
// was: boost::system::system_category(void)
pub fn stub_23b508() -> ! {
    todo!("0x23b508 __ZN5boost6system15system_categoryEv")
}

#[doc(alias = "boost::system::error_category::default_error_condition(int)const")]
// 0x23ca3c — __ZNK5boost6system14error_category23default_error_conditionEi
// was: boost::system::error_category::default_error_condition(int)const
pub fn stub_23ca3c() -> ! {
    todo!("0x23ca3c __ZNK5boost6system14error_category23default_error_conditionEi")
}

#[doc(alias = "boost::system::error_category::equivalent(int,boost::system::error_condition const&)const")]
// 0x23ca44 — __ZNK5boost6system14error_category10equivalentEiRKNS0_15error_conditionE
// was: boost::system::error_category::equivalent(int,boost::system::error_condition const&)const
pub fn stub_23ca44() -> ! {
    todo!("0x23ca44 __ZNK5boost6system14error_category10equivalentEiRKNS0_15error_conditionE")
}

#[doc(alias = "boost::system::error_category::equivalent(boost::system::error_code const&,int)const")]
// 0x23ca70 — __ZNK5boost6system14error_category10equivalentERKNS0_10error_codeEi
// was: boost::system::error_category::equivalent(boost::system::error_code const&,int)const
pub fn stub_23ca70() -> ! {
    todo!("0x23ca70 __ZNK5boost6system14error_category10equivalentERKNS0_10error_codeEi")
}

#[doc(alias = "boost::iostreams::detail::gzip_header::process(char)")]
// 0x23cb64 — __ZN5boost9iostreams6detail11gzip_header7processEc
// was: boost::iostreams::detail::gzip_header::process(char)
pub fn stub_23cb64() -> ! {
    todo!("0x23cb64 __ZN5boost9iostreams6detail11gzip_header7processEc")
}

#[doc(alias = "boost::iostreams::detail::gzip_header::reset(void)")]
// 0x23cef0 — __ZN5boost9iostreams6detail11gzip_header5resetEv
// was: boost::iostreams::detail::gzip_header::reset(void)
pub fn stub_23cef0() -> ! {
    todo!("0x23cef0 __ZN5boost9iostreams6detail11gzip_header5resetEv")
}

#[doc(alias = "boost::iostreams::detail::gzip_footer::process(char)")]
// 0x23cf2c — __ZN5boost9iostreams6detail11gzip_footer7processEc
// was: boost::iostreams::detail::gzip_footer::process(char)
pub fn stub_23cf2c() -> ! {
    todo!("0x23cf2c __ZN5boost9iostreams6detail11gzip_footer7processEc")
}

#[doc(alias = "boost::iostreams::detail::gzip_footer::reset(void)")]
// 0x23cf7c — __ZN5boost9iostreams6detail11gzip_footer5resetEv
// was: boost::iostreams::detail::gzip_footer::reset(void)
pub fn stub_23cf7c() -> ! {
    todo!("0x23cf7c __ZN5boost9iostreams6detail11gzip_footer5resetEv")
}

#[doc(alias = "boost::iostreams::zlib_error::check(int)")]
// 0x23cf8c — __ZN5boost9iostreams10zlib_error5checkEi
// was: boost::iostreams::zlib_error::check(int)
pub fn stub_23cf8c() -> ! {
    todo!("0x23cf8c __ZN5boost9iostreams10zlib_error5checkEi")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::zlib_base(void)")]
// 0x23d0c8 — __ZN5boost9iostreams6detail9zlib_baseC2Ev
// was: boost::iostreams::detail::zlib_base::zlib_base(void)
pub fn stub_23d0c8() -> ! {
    todo!("0x23d0c8 __ZN5boost9iostreams6detail9zlib_baseC2Ev")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::~zlib_base()")]
// 0x23d0e8 — __ZN5boost9iostreams6detail9zlib_baseD2Ev
// was: boost::iostreams::detail::zlib_base::~zlib_base()
pub fn stub_23d0e8() -> ! {
    todo!("0x23d0e8 __ZN5boost9iostreams6detail9zlib_baseD2Ev")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::before(char const*&,char const*,char *&,char *)")]
// 0x23d0fc — __ZN5boost9iostreams6detail9zlib_base6beforeERPKcS4_RPcS6_
// was: boost::iostreams::detail::zlib_base::before(char const*&,char const*,char *&,char *)
pub fn stub_23d0fc() -> ! {
    todo!("0x23d0fc __ZN5boost9iostreams6detail9zlib_base6beforeERPKcS4_RPcS6_")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::after(char const*&,char *&,bool)")]
// 0x23d120 — __ZN5boost9iostreams6detail9zlib_base5afterERPKcRPcb
// was: boost::iostreams::detail::zlib_base::after(char const*&,char *&,bool)
pub fn stub_23d120() -> ! {
    todo!("0x23d120 __ZN5boost9iostreams6detail9zlib_base5afterERPKcRPcb")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::xdeflate(int)")]
// 0x23d180 — __ZN5boost9iostreams6detail9zlib_base8xdeflateEi
// was: boost::iostreams::detail::zlib_base::xdeflate(int)
pub fn stub_23d180() -> ! {
    todo!("0x23d180 __ZN5boost9iostreams6detail9zlib_base8xdeflateEi")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::xinflate(int)")]
// 0x23d18c — __ZN5boost9iostreams6detail9zlib_base8xinflateEi
// was: boost::iostreams::detail::zlib_base::xinflate(int)
pub fn stub_23d18c() -> ! {
    todo!("0x23d18c __ZN5boost9iostreams6detail9zlib_base8xinflateEi")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::reset(bool,bool)")]
// 0x23d198 — __ZN5boost9iostreams6detail9zlib_base5resetEbb
// was: boost::iostreams::detail::zlib_base::reset(bool,bool)
pub fn stub_23d198() -> ! {
    todo!("0x23d198 __ZN5boost9iostreams6detail9zlib_base5resetEbb")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::do_init(boost::iostreams::zlib_params const&,bool,void * (*)(void *,unsigned int,unsigned int),void (*)(void *,void *),void *)")]
// 0x23d1c8 — __ZN5boost9iostreams6detail9zlib_base7do_initERKNS0_11zlib_paramsEbPFPvS6_jjEPFvS6_S6_ES6_
// was: boost::iostreams::detail::zlib_base::do_init(boost::iostreams::zlib_params const&,bool,void * (*)(void *,unsigned int,unsigned int),void (*)(void *,void *),void *)
pub fn stub_23d1c8() -> ! {
    todo!("0x23d1c8 __ZN5boost9iostreams6detail9zlib_base7do_initERKNS0_11zlib_paramsEbPFPvS6_jjEPFvS6_S6_ES6_")
}

#[doc(alias = "void boost::throw_exception<boost::iostreams::zlib_error>(boost::iostreams::zlib_error const&)")]
// 0x23d238 — __ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_
// was: void boost::throw_exception<boost::iostreams::zlib_error>(boost::iostreams::zlib_error const&)
pub fn stub_23d238() -> ! {
    todo!("0x23d238 __ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_")
}

#[doc(alias = "boost::iostreams::zlib_error::~zlib_error()")]
// 0x23d390 — __ZN5boost9iostreams10zlib_errorD1Ev
// was: boost::iostreams::zlib_error::~zlib_error()
pub fn stub_23d390() -> ! {
    todo!("0x23d390 __ZN5boost9iostreams10zlib_errorD1Ev")
}

#[doc(alias = "boost::iostreams::zlib_error::~zlib_error()")]
// 0x23d39c — __ZN5boost9iostreams10zlib_errorD0Ev
// was: boost::iostreams::zlib_error::~zlib_error()
pub fn stub_23d39c() -> ! {
    todo!("0x23d39c __ZN5boost9iostreams10zlib_errorD0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
// 0x23d3b0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()
pub fn stub_23d3b0() -> ! {
    todo!("0x23d3b0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
// 0x23d468 — __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev
// was: boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()
pub fn stub_23d468() -> ! {
    todo!("0x23d468 __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev")
}

#[doc(alias = "non_virtual_thunk_toboost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
// 0x23d520 — __ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()
pub fn stub_23d520() -> ! {
    todo!("0x23d520 __ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev")
}

#[doc(alias = "non_virtual_thunk_toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
// 0x23d5d8 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()
pub fn stub_23d5d8() -> ! {
    todo!("0x23d5d8 __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev")
}

#[doc(alias = "virtual_thunk_toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
// 0x23d690 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()
pub fn stub_23d690() -> ! {
    todo!("0x23d690 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
// 0x23d75c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()
pub fn stub_23d75c() -> ! {
    todo!("0x23d75c __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const")]
// 0x23d818 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const
pub fn stub_23d818() -> ! {
    todo!("0x23d818 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const")]
// 0x23d8d4 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const
pub fn stub_23d8d4() -> ! {
    todo!("0x23d8d4 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv")
}

#[doc(alias = "non_virtual_thunk_toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
// 0x23d984 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()
pub fn stub_23d984() -> ! {
    todo!("0x23d984 __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev")
}

#[doc(alias = "virtual_thunk_toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const")]
// 0x23da40 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const
pub fn stub_23da40() -> ! {
    todo!("0x23da40 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv")
}

#[doc(alias = "virtual_thunk_toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const")]
// 0x23db04 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const
pub fn stub_23db04() -> ! {
    todo!("0x23db04 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv")
}

#[doc(alias = "virtual_thunk_toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
// 0x23db14 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()
pub fn stub_23db14() -> ! {
    todo!("0x23db14 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&)")]
// 0x23dbe8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&)
pub fn stub_23dbe8() -> ! {
    todo!("0x23dbe8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
// 0x23dd30 — __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED0Ev
// was: boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()
pub fn stub_23dd30() -> ! {
    todo!("0x23dd30 __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED0Ev")
}

#[doc(alias = "non_virtual_thunk_toboost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
// 0x23ddec — __ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()
pub fn stub_23ddec() -> ! {
    todo!("0x23ddec __ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_tag)")]
// 0x23dea8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_NS6_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_tag)
pub fn stub_23dea8() -> ! {
    todo!("0x23dea8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::iostreams::zlib_error> const&)")]
// 0x23e044 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS5_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::iostreams::zlib_error> const&)
pub fn stub_23e044() -> ! {
    todo!("0x23e044 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS5_")
}

#[doc(alias = "RBX::trim_trailing_slashes(std::string const&)")]
// 0x23e52c — __ZN3RBX21trim_trailing_slashesERKSs
// was: RBX::trim_trailing_slashes(std::string const&)
pub fn stub_23e52c() -> ! {
    todo!("0x23e52c __ZN3RBX21trim_trailing_slashesERKSs")
}

#[doc(alias = "RBX::Debugable::dump(std::ostream &)")]
// 0x23e5f8 — __ZN3RBX9Debugable4dumpERSo
// was: RBX::Debugable::dump(std::ostream &)
pub fn stub_23e5f8() -> ! {
    todo!("0x23e5f8 __ZN3RBX9Debugable4dumpERSo")
}

#[doc(alias = "RBX::Log::timeStamp(std::basic_ofstream<char,std::char_traits<char>> &,bool)")]
// 0x23e678 — __ZN3RBX3Log9timeStampERSt14basic_ofstreamIcSt11char_traitsIcEEb
// was: RBX::Log::timeStamp(std::basic_ofstream<char,std::char_traits<char>> &,bool)
pub fn stub_23e678() -> ! {
    todo!("0x23e678 __ZN3RBX3Log9timeStampERSt14basic_ofstreamIcSt11char_traitsIcEEb")
}

#[doc(alias = "boost::date_time::gregorian_calendar_base<boost::date_time::year_month_day_base<boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day>,unsigned int>::from_day_number(unsigned int)")]
// 0x23ec04 — __ZN5boost9date_time23gregorian_calendar_baseINS0_19year_month_day_baseINS_9gregorian9greg_yearENS3_10greg_monthENS3_8greg_dayEEEjE15from_day_numberEj
// was: boost::date_time::gregorian_calendar_base<boost::date_time::year_month_day_base<boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day>,unsigned int>::from_day_number(unsigned int)
pub fn stub_23ec04() -> ! {
    todo!("0x23ec04 __ZN5boost9date_time23gregorian_calendar_baseINS0_19year_month_day_baseINS_9gregorian9greg_yearENS3_10greg_monthENS3_8greg_dayEEEjE15from_day_numberEj")
}

#[doc(alias = "boost::date_time::second_clock<boost::posix_time::ptime>::create_time(tm *)")]
// 0x23ecfc — __ZN5boost9date_time12second_clockINS_10posix_time5ptimeEE11create_timeEP2tm
// was: boost::date_time::second_clock<boost::posix_time::ptime>::create_time(tm *)
pub fn stub_23ecfc() -> ! {
    todo!("0x23ecfc __ZN5boost9date_time12second_clockINS_10posix_time5ptimeEE11create_timeEP2tm")
}

#[doc(alias = "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::date(void)const")]
// 0x23ef20 — __ZNK5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEE4dateEv
// was: boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::date(void)const
pub fn stub_23ef20() -> ! {
    todo!("0x23ef20 __ZNK5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEE4dateEv")
}

#[doc(alias = "RBX::thread_wrapper(boost::function0<void> const&,char const*)")]
// 0x23f50c — __ZN3RBX14thread_wrapperERKN5boost9function0IvEEPKc
// was: RBX::thread_wrapper(boost::function0<void> const&,char const*)
pub fn stub_23f50c() -> ! {
    todo!("0x23f50c __ZN3RBX14thread_wrapperERKN5boost9function0IvEEPKc")
}

#[doc(alias = "RBX::thread_function(boost::function0<void> const&,std::string)")]
// 0x23f8f0 — __ZN3RBXL15thread_functionERKN5boost9function0IvEESs
// was: RBX::thread_function(boost::function0<void> const&,std::string)
pub fn stub_23f8f0() -> ! {
    todo!("0x23f8f0 __ZN3RBXL15thread_functionERKN5boost9function0IvEESs")
}

#[doc(alias = "RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*)")]
// 0x23fa10 — __ZN3RBX13worker_threadC1ERKN5boost9function0INS0_11work_resultEEEPKc
// was: RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*)
pub fn stub_23fa10() -> ! {
    todo!("0x23fa10 __ZN3RBX13worker_threadC1ERKN5boost9function0INS0_11work_resultEEEPKc")
}

#[doc(alias = "RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*)")]
// 0x23fa1c — __ZN3RBX13worker_threadC2ERKN5boost9function0INS0_11work_resultEEEPKc
// was: RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*)
pub fn stub_23fa1c() -> ! {
    todo!("0x23fa1c __ZN3RBX13worker_threadC2ERKN5boost9function0INS0_11work_resultEEEPKc")
}

#[doc(alias = "RBX::worker_thread::threadProc(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&)")]
// 0x23ffb0 — __ZN3RBX13worker_thread10threadProcEN5boost10shared_ptrINS0_4dataEEERKNS1_9function0INS0_11work_resultEEE
// was: RBX::worker_thread::threadProc(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&)
pub fn stub_23ffb0() -> ! {
    todo!("0x23ffb0 __ZN3RBX13worker_thread10threadProcEN5boost10shared_ptrINS0_4dataEEERKNS1_9function0INS0_11work_resultEEE")
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::~thread_specific_ptr()")]
// 0x2403cc — __ZN5boost19thread_specific_ptrISsED1Ev
// was: boost::thread_specific_ptr<std::string>::~thread_specific_ptr()
pub fn stub_2403cc() -> ! {
    todo!("0x2403cc __ZN5boost19thread_specific_ptrISsED1Ev")
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::reset(std::string *)")]
// 0x2403d8 — __ZN5boost19thread_specific_ptrISsE5resetEPSs
// was: boost::thread_specific_ptr<std::string>::reset(std::string *)
pub fn stub_2403d8() -> ! {
    todo!("0x2403d8 __ZN5boost19thread_specific_ptrISsE5resetEPSs")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list_av_2<boost::function0<void>,std::string>::type> boost::bind<void,boost::function0<void> const&,std::string,boost::function0<void>,std::string>(void (*)(boost::function0<void> const&,std::string),boost::function0<void>,std::string)")]
// 0x2404f4 — __ZN5boost4bindIvRKNS_9function0IvEESsS2_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
// was: boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list_av_2<boost::function0<void>,std::string>::type> boost::bind<void,boost::function0<void> const&,std::string,boost::function0<void>,std::string>(void (*)(boost::function0<void> const&,std::string),boost::function0<void>,std::string)
pub fn stub_2404f4() -> ! {
    todo!("0x2404f4 __ZN5boost4bindIvRKNS_9function0IvEESsS2_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list_av_2<boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>::type> boost::bind<void,boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&,boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>(void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>)")]
// 0x2407fc — __ZN5boost4bindIvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS3_11work_resultEEES5_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list_av_2<boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>::type> boost::bind<void,boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&,boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>(void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>)
pub fn stub_2407fc() -> ! {
    todo!("0x2407fc __ZN5boost4bindIvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS3_11work_resultEEES5_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_")
}

#[doc(alias = "void boost::condition_variable_any::wait<boost::unique_lock<boost::mutex>>(boost::unique_lock<boost::mutex> &)")]
// 0x240a54 — __ZN5boost22condition_variable_any4waitINS_11unique_lockINS_5mutexEEEEEvRT_
// was: void boost::condition_variable_any::wait<boost::unique_lock<boost::mutex>>(boost::unique_lock<boost::mutex> &)
pub fn stub_240a54() -> ! {
    todo!("0x240a54 __ZN5boost22condition_variable_any4waitINS_11unique_lockINS_5mutexEEEEEvRT_")
}

#[doc(alias = "void boost::throw_exception<boost::condition_error>(boost::condition_error const&)")]
// 0x240c80 — __ZN5boost15throw_exceptionINS_15condition_errorEEEvRKT_
// was: void boost::throw_exception<boost::condition_error>(boost::condition_error const&)
pub fn stub_240c80() -> ! {
    todo!("0x240c80 __ZN5boost15throw_exceptionINS_15condition_errorEEEvRKT_")
}

#[doc(alias = "boost::condition_error::~condition_error()")]
// 0x241040 — __ZN5boost15condition_errorD1Ev
// was: boost::condition_error::~condition_error()
pub fn stub_241040() -> ! {
    todo!("0x241040 __ZN5boost15condition_errorD1Ev")
}

#[doc(alias = "boost::condition_error::~condition_error()")]
// 0x2410a0 — __ZN5boost15condition_errorD0Ev
// was: boost::condition_error::~condition_error()
pub fn stub_2410a0() -> ! {
    todo!("0x2410a0 __ZN5boost15condition_errorD0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")]
// 0x241108 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()
pub fn stub_241108() -> ! {
    todo!("0x241108 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED1Ev")
}

#[doc(alias = "non_virtual_thunk_toboost::exception_detail::error_info_injector<boost::condition_error>::~error_info_injector()")]
// 0x241214 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_15condition_errorEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::condition_error>::~error_info_injector()
pub fn stub_241214() -> ! {
    todo!("0x241214 __ZThn20_N5boost16exception_detail19error_info_injectorINS_15condition_errorEED1Ev")
}

#[doc(alias = "non_virtual_thunk_toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")]
// 0x241324 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()
pub fn stub_241324() -> ! {
    todo!("0x241324 __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED1Ev")
}

#[doc(alias = "virtual_thunk_toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::clone(void)const")]
// 0x241430 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEE5cloneEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::clone(void)const
pub fn stub_241430() -> ! {
    todo!("0x241430 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEE5cloneEv")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>)")]
// 0x241444 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS0_INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSH_ISB_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>)
pub fn stub_241444() -> ! {
    todo!("0x241444 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS0_INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSH_ISB_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x241798 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_241798() -> ! {
    todo!("0x241798 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x2417bc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_2417bc() -> ! {
    todo!("0x2417bc __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x2417d0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS9_11work_resultEEEENS5_5list2INS5_5valueISB_EENSK_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_2417d0() -> ! {
    todo!("0x2417d0 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS9_11work_resultEEEENS5_5list2INS5_5valueISB_EENSK_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::operator()<void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) &,boost::_bi::list0 &,int)")]
// 0x241aac — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEclIPFvS7_RKSB_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::operator()<void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) &,boost::_bi::list0 &,int)
pub fn stub_241aac() -> ! {
    todo!("0x241aac __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEclIPFvS7_RKSB_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x241bbc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_241bbc() -> ! {
    todo!("0x241bbc __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
// 0x241df4 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)
pub fn stub_241df4() -> ! {
    todo!("0x241df4 __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
// 0x241f98 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)
pub fn stub_241f98() -> ! {
    todo!("0x241f98 __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>::bind_t(void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>> const&)")]
// 0x242144 — __ZN5boost3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS4_11work_resultEEEENS0_5list2INS0_5valueIS6_EENSF_IS9_EEEEEC2ESD_RKSI_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>::bind_t(void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>> const&)
pub fn stub_242144() -> ! {
    todo!("0x242144 __ZN5boost3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS4_11work_resultEEEENS0_5list2INS0_5valueIS6_EENSF_IS9_EEEEEC2ESD_RKSI_")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::worker_thread::data,RBX::worker_thread::data>(boost::shared_ptr<RBX::worker_thread::data> *,RBX::worker_thread::data *,boost::detail::shared_count &)")]
// 0x242284 — __ZN5boost6detail20sp_pointer_constructIN3RBX13worker_thread4dataES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// was: void boost::detail::sp_pointer_construct<RBX::worker_thread::data,RBX::worker_thread::data>(boost::shared_ptr<RBX::worker_thread::data> *,RBX::worker_thread::data *,boost::detail::shared_count &)
pub fn stub_242284() -> ! {
    todo!("0x242284 __ZN5boost6detail20sp_pointer_constructIN3RBX13worker_thread4dataES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()")]
// 0x2423c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()
pub fn stub_2423c8() -> ! {
    todo!("0x2423c8 __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()")]
// 0x2423cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED0Ev
// was: boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()
pub fn stub_2423cc() -> ! {
    todo!("0x2423cc __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::dispose(void)")]
// 0x2423d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::dispose(void)
pub fn stub_2423d8() -> ! {
    todo!("0x2423d8 __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_deleter(std::type_info const&)")]
// 0x2424bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_deleter(std::type_info const&)
pub fn stub_2424bc() -> ! {
    todo!("0x2424bc __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_untyped_deleter(void)")]
// 0x2424c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_untyped_deleter(void)
pub fn stub_2424c0() -> ! {
    todo!("0x2424c0 __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>)")]
// 0x2424c4 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_SsENS3_5list2INS3_5valueIS1_EENSA_ISsEEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>)
pub fn stub_2424c4() -> ! {
    todo!("0x2424c4 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_SsENS3_5list2INS3_5valueIS1_EENSA_ISsEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x242818 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_242818() -> ! {
    todo!("0x242818 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x24283c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_24283c() -> ! {
    todo!("0x24283c __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x242958 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS5_5list2INS5_5valueIS8_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_242958() -> ! {
    todo!("0x242958 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS5_5list2INS5_5valueIS8_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x242be8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_242be8() -> ! {
    todo!("0x242be8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")]
// 0x242e08 — __ZN5boost3_bi5list2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_
// was: boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)
pub fn stub_242e08() -> ! {
    todo!("0x242e08 __ZN5boost3_bi5list2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")]
// 0x242fc0 — __ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_
// was: boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)
pub fn stub_242fc0() -> ! {
    todo!("0x242fc0 __ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_")
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::~thread_specific_ptr()")]
// 0x24316c — __ZN5boost19thread_specific_ptrISsED2Ev
// was: boost::thread_specific_ptr<std::string>::~thread_specific_ptr()
pub fn stub_24316c() -> ! {
    todo!("0x24316c __ZN5boost19thread_specific_ptrISsED2Ev")
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::~delete_data()")]
// 0x243260 — __ZN5boost19thread_specific_ptrISsE11delete_dataD1Ev
// was: boost::thread_specific_ptr<std::string>::delete_data::~delete_data()
pub fn stub_243260() -> ! {
    todo!("0x243260 __ZN5boost19thread_specific_ptrISsE11delete_dataD1Ev")
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::~delete_data()")]
// 0x243264 — __ZN5boost19thread_specific_ptrISsE11delete_dataD0Ev
// was: boost::thread_specific_ptr<std::string>::delete_data::~delete_data()
pub fn stub_243264() -> ! {
    todo!("0x243264 __ZN5boost19thread_specific_ptrISsE11delete_dataD0Ev")
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::operator()(void *)")]
// 0x243270 — __ZN5boost19thread_specific_ptrISsE11delete_dataclEPv
// was: boost::thread_specific_ptr<std::string>::delete_data::operator()(void *)
pub fn stub_243270() -> ! {
    todo!("0x243270 __ZN5boost19thread_specific_ptrISsE11delete_dataclEPv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()")]
// 0x2432c4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED1Ev
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()
pub fn stub_2432c4() -> ! {
    todo!("0x2432c4 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()")]
// 0x2432c8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED0Ev
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()
pub fn stub_2432c8() -> ! {
    todo!("0x2432c8 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::dispose(void)")]
// 0x2432d4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::dispose(void)
pub fn stub_2432d4() -> ! {
    todo!("0x2432d4 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_deleter(std::type_info const&)")]
// 0x2432e8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_deleter(std::type_info const&)
pub fn stub_2432e8() -> ! {
    todo!("0x2432e8 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_untyped_deleter(void)")]
// 0x243300 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_untyped_deleter(void)
pub fn stub_243300() -> ! {
    todo!("0x243300 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::condition_variable_any::condition_variable_any(void)")]
// 0x243304 — __ZN5boost22condition_variable_anyC2Ev
// was: boost::condition_variable_any::condition_variable_any(void)
pub fn stub_243304() -> ! {
    todo!("0x243304 __ZN5boost22condition_variable_anyC2Ev")
}

#[doc(alias = "global constructor keyed to_a_44")]
// 0x2434dc — __GLOBAL__I_a_44
// was: `global constructor keyed to'_a_44
pub fn stub_2434dc() -> ! {
    todo!("0x2434dc __GLOBAL__I_a_44")
}

#[doc(alias = "RBX::CEvent::Wait(void)")]
// 0x2435a4 — __ZN3RBX6CEvent4WaitEv
// was: RBX::CEvent::Wait(void)
pub fn stub_2435a4() -> ! {
    todo!("0x2435a4 __ZN3RBX6CEvent4WaitEv")
}

#[doc(alias = "RBX::CEvent::WaitForSingleObject(RBX::CEvent&,int)")]
// 0x2435b4 — __ZN3RBX6CEvent19WaitForSingleObjectERS0_i
// was: RBX::CEvent::WaitForSingleObject(RBX::CEvent&,int)
pub fn stub_2435b4() -> ! {
    todo!("0x2435b4 __ZN3RBX6CEvent19WaitForSingleObjectERS0_i")
}

#[doc(alias = "RBX::CEvent::Wait(int)")]
// 0x24381c — __ZN3RBX6CEvent4WaitEi
// was: RBX::CEvent::Wait(int)
pub fn stub_24381c() -> ! {
    todo!("0x24381c __ZN3RBX6CEvent4WaitEi")
}

#[doc(alias = "RBX::CEvent::~CEvent()")]
// 0x243830 — __ZN3RBX6CEventD1Ev
// was: RBX::CEvent::~CEvent()
pub fn stub_243830() -> ! {
    todo!("0x243830 __ZN3RBX6CEventD1Ev")
}

#[doc(alias = "RBX::CEvent::~CEvent()")]
// 0x24383c — __ZN3RBX6CEventD2Ev
// was: RBX::CEvent::~CEvent()
pub fn stub_24383c() -> ! {
    todo!("0x24383c __ZN3RBX6CEventD2Ev")
}

#[doc(alias = "RBX::CEvent::CEvent(bool)")]
// 0x243944 — __ZN3RBX6CEventC1Eb
// was: RBX::CEvent::CEvent(bool)
pub fn stub_243944() -> ! {
    todo!("0x243944 __ZN3RBX6CEventC1Eb")
}

#[doc(alias = "RBX::CEvent::Set(void)")]
// 0x243a30 — __ZN3RBX6CEvent3SetEv
// was: RBX::CEvent::Set(void)
pub fn stub_243a30() -> ! {
    todo!("0x243a30 __ZN3RBX6CEvent3SetEv")
}

#[doc(alias = "boost::condition_variable::do_wait_until(boost::unique_lock<boost::mutex> &,timespec const&)")]
// 0x243b84 — __ZN5boost18condition_variable13do_wait_untilERNS_11unique_lockINS_5mutexEEERK8timespec
// was: boost::condition_variable::do_wait_until(boost::unique_lock<boost::mutex> &,timespec const&)
pub fn stub_243b84() -> ! {
    todo!("0x243b84 __ZN5boost18condition_variable13do_wait_untilERNS_11unique_lockINS_5mutexEEERK8timespec")
}

#[doc(alias = "global constructor keyed to_a_45")]
// 0x243dd0 — __GLOBAL__I_a_45
// was: `global constructor keyed to'_a_45
pub fn stub_243dd0() -> ! {
    todo!("0x243dd0 __GLOBAL__I_a_45")
}

#[doc(alias = "RBX::Limits::Countable::Countable(void)")]
// 0x243e98 — __ZN3RBX6Limits9CountableC2Ev
// was: RBX::Limits::Countable::Countable(void)
pub fn stub_243e98() -> ! {
    todo!("0x243e98 __ZN3RBX6Limits9CountableC2Ev")
}

#[doc(alias = "RBX::Limits::Counter::add(RBX::Limits::Countable *)")]
// 0x244088 — __ZN3RBX6Limits7Counter3addEPNS0_9CountableE
// was: RBX::Limits::Counter::add(RBX::Limits::Countable *)
pub fn stub_244088() -> ! {
    todo!("0x244088 __ZN3RBX6Limits7Counter3addEPNS0_9CountableE")
}

#[doc(alias = "RBX::Limits::Countable::~Countable()")]
// 0x244200 — __ZN3RBX6Limits9CountableD2Ev
// was: RBX::Limits::Countable::~Countable()
pub fn stub_244200() -> ! {
    todo!("0x244200 __ZN3RBX6Limits9CountableD2Ev")
}

#[doc(alias = "RBX::Limits::Counter::getCurrentCount(void)")]
// 0x2442c4 — __ZN3RBX6Limits7Counter15getCurrentCountEv
// was: RBX::Limits::Counter::getCurrentCount(void)
pub fn stub_2442c4() -> ! {
    todo!("0x2442c4 __ZN3RBX6Limits7Counter15getCurrentCountEv")
}

#[doc(alias = "RBX::Limits::Counter::canAdd(int)")]
// 0x244358 — __ZN3RBX6Limits7Counter6canAddEi
// was: RBX::Limits::Counter::canAdd(int)
pub fn stub_244358() -> ! {
    todo!("0x244358 __ZN3RBX6Limits7Counter6canAddEi")
}

#[doc(alias = "RBX::Limits::Counter::Activator::Activator(boost::shared_ptr<RBX::Limits::Counter>)")]
// 0x244384 — __ZN3RBX6Limits7Counter9ActivatorC1EN5boost10shared_ptrIS1_EE
// was: RBX::Limits::Counter::Activator::Activator(boost::shared_ptr<RBX::Limits::Counter>)
// now: RBX::Limits::Counter::Activator::Activator(rbx_core::SharedPtr<RBX::Limits::Counter>) // rbx_core::SharedPtr
pub fn stub_244384() -> ! {
    todo!("0x244384 __ZN3RBX6Limits7Counter9ActivatorC1EN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::Limits::Counter::Activator::Activator(boost::shared_ptr<RBX::Limits::Counter>)")]
// 0x244390 — __ZN3RBX6Limits7Counter9ActivatorC2EN5boost10shared_ptrIS1_EE
// was: RBX::Limits::Counter::Activator::Activator(boost::shared_ptr<RBX::Limits::Counter>)
// now: RBX::Limits::Counter::Activator::Activator(rbx_core::SharedPtr<RBX::Limits::Counter>) // rbx_core::SharedPtr
pub fn stub_244390() -> ! {
    todo!("0x244390 __ZN3RBX6Limits7Counter9ActivatorC2EN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::Limits::Counter::Activator::~Activator()")]
// 0x2445fc — __ZN3RBX6Limits7Counter9ActivatorD1Ev
// was: RBX::Limits::Counter::Activator::~Activator()
pub fn stub_2445fc() -> ! {
    todo!("0x2445fc __ZN3RBX6Limits7Counter9ActivatorD1Ev")
}

#[doc(alias = "RBX::Limits::Counter::Activator::~Activator()")]
// 0x244608 — __ZN3RBX6Limits7Counter9ActivatorD2Ev
// was: RBX::Limits::Counter::Activator::~Activator()
pub fn stub_244608() -> ! {
    todo!("0x244608 __ZN3RBX6Limits7Counter9ActivatorD2Ev")
}

#[doc(alias = "boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::reset(boost::shared_ptr<RBX::Limits::Counter>*)")]
// 0x24480c — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE5resetEPS5_
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::reset(boost::shared_ptr<RBX::Limits::Counter>*)
// now: boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::reset(rbx_core::SharedPtr<RBX::Limits::Counter>*) // rbx_core::SharedPtr
pub fn stub_24480c() -> ! {
    todo!("0x24480c __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE5resetEPS5_")
}

#[doc(alias = "RBX::Limits::Counter::safe_static_init_current(void)")]
// 0x244928 — __ZN3RBX6Limits7Counter24safe_static_init_currentEv
// was: RBX::Limits::Counter::safe_static_init_current(void)
pub fn stub_244928() -> ! {
    todo!("0x244928 __ZN3RBX6Limits7Counter24safe_static_init_currentEv")
}

#[doc(alias = "RBX::Limits::Counter::safe_static_do_get_current(void)")]
// 0x244934 — __ZN3RBX6Limits7Counter26safe_static_do_get_currentEv
// was: RBX::Limits::Counter::safe_static_do_get_current(void)
pub fn stub_244934() -> ! {
    todo!("0x244934 __ZN3RBX6Limits7Counter26safe_static_do_get_currentEv")
}

#[doc(alias = "rbx::thread_specific_shared_ptr<RBX::Limits::Counter>::~thread_specific_shared_ptr()")]
// 0x244ab8 — __ZN3rbx26thread_specific_shared_ptrIN3RBX6Limits7CounterEED1Ev
// was: rbx::thread_specific_shared_ptr<RBX::Limits::Counter>::~thread_specific_shared_ptr()
pub fn stub_244ab8() -> ! {
    todo!("0x244ab8 __ZN3rbx26thread_specific_shared_ptrIN3RBX6Limits7CounterEED1Ev")
}

#[doc(alias = "boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::~thread_specific_ptr()")]
// 0x244ac8 — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEED2Ev
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::~thread_specific_ptr()
// now: boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::~thread_specific_ptr() // rbx_core::SharedPtr
pub fn stub_244ac8() -> ! {
    todo!("0x244ac8 __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEED2Ev")
}

#[doc(alias = "boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::~delete_data()")]
// 0x244bbc — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD1Ev
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::~delete_data()
// now: boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::~delete_data() // rbx_core::SharedPtr
pub fn stub_244bbc() -> ! {
    todo!("0x244bbc __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD1Ev")
}

#[doc(alias = "boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::~delete_data()")]
// 0x244bc0 — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD0Ev
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::~delete_data()
// now: boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::~delete_data() // rbx_core::SharedPtr
pub fn stub_244bc0() -> ! {
    todo!("0x244bc0 __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD0Ev")
}

#[doc(alias = "boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::operator()(void *)")]
// 0x244bcc — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataclEPv
// was: boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data::operator()(void *)
// now: boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::operator()(void *) // rbx_core::SharedPtr
pub fn stub_244bcc() -> ! {
    todo!("0x244bcc __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataclEPv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")]
// 0x244c74 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED1Ev
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()
// now: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd() // rbx_core::SharedPtr
pub fn stub_244c74() -> ! {
    todo!("0x244c74 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")]
// 0x244c78 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED0Ev
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()
// now: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd() // rbx_core::SharedPtr
pub fn stub_244c78() -> ! {
    todo!("0x244c78 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::dispose(void)")]
// 0x244c84 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::dispose(void)
// now: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::dispose(void) // rbx_core::SharedPtr
pub fn stub_244c84() -> ! {
    todo!("0x244c84 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::get_deleter(std::type_info const&)")]
// 0x244c98 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::get_deleter(std::type_info const&)
// now: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_deleter(std::type_info const&) // rbx_core::SharedPtr
pub fn stub_244c98() -> ! {
    todo!("0x244c98 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::get_untyped_deleter(void)")]
// 0x244cb0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<boost::shared_ptr<RBX::Limits::Counter>>::delete_data>>::get_untyped_deleter(void)
// now: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_untyped_deleter(void) // rbx_core::SharedPtr
pub fn stub_244cb0() -> ! {
    todo!("0x244cb0 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE19get_untyped_deleterEv")
}

#[doc(alias = "global constructor keyed to_a_46")]
// 0x244cb4 — __GLOBAL__I_a_46
// was: `global constructor keyed to'_a_46
pub fn stub_244cb4() -> ! {
    todo!("0x244cb4 __GLOBAL__I_a_46")
}

#[doc(alias = "RBX::roblox_allocator::malloc(unsigned long)")]
// 0x244d7c — __ZN3RBX16roblox_allocator6mallocEm
// was: RBX::roblox_allocator::malloc(unsigned long)
pub fn stub_244d7c() -> ! {
    todo!("0x244d7c __ZN3RBX16roblox_allocator6mallocEm")
}

#[doc(alias = "RBX::roblox_allocator::free(char *)")]
// 0x244dac — __ZN3RBX16roblox_allocator4freeEPc
// was: RBX::roblox_allocator::free(char *)
pub fn stub_244dac() -> ! {
    todo!("0x244dac __ZN3RBX16roblox_allocator4freeEPc")
}

#[doc(alias = "std::vector<unsigned long *,std::allocator<unsigned long *>>::~vector()")]
// 0x244db8 — __ZNSt6vectorIPmSaIS0_EED1Ev
// was: std::vector<unsigned long *,std::allocator<unsigned long *>>::~vector()
pub fn stub_244db8() -> ! {
    todo!("0x244db8 __ZNSt6vectorIPmSaIS0_EED1Ev")
}

#[doc(alias = "std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::~vector()")]
// 0x244dcc — __ZNSt6vectorIPFbvESaIS1_EED1Ev
// was: std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::~vector()
pub fn stub_244dcc() -> ! {
    todo!("0x244dcc __ZNSt6vectorIPFbvESaIS1_EED1Ev")
}

#[doc(alias = "global constructor keyed to_a_47")]
// 0x244de0 — __GLOBAL__I_a_47
// was: `global constructor keyed to'_a_47
pub fn stub_244de0() -> ! {
    todo!("0x244de0 __GLOBAL__I_a_47")
}

#[doc(alias = "rbx::signals::connection::disconnect(void)const")]
// 0x244e94 — __ZNK3rbx7signals10connection10disconnectEv
// was: rbx::signals::connection::disconnect(void)const
pub fn stub_244e94() -> ! {
    todo!("0x244e94 __ZNK3rbx7signals10connection10disconnectEv")
}

#[doc(alias = "rbx::signals::connection::connected(void)const")]
// 0x244fd4 — __ZNK3rbx7signals10connection9connectedEv
// was: rbx::signals::connection::connected(void)const
pub fn stub_244fd4() -> ! {
    todo!("0x244fd4 __ZNK3rbx7signals10connection9connectedEv")
}

#[doc(alias = "rbx::signals::connection::operator==(rbx::signals::connection const&)const")]
// 0x245118 — __ZNK3rbx7signals10connectioneqERKS1_
// was: rbx::signals::connection::operator==(rbx::signals::connection const&)const
pub fn stub_245118() -> ! {
    todo!("0x245118 __ZNK3rbx7signals10connectioneqERKS1_")
}

#[doc(alias = "rbx::signals::connection::operator!=(rbx::signals::connection const&)const")]
// 0x2452d0 — __ZNK3rbx7signals10connectionneERKS1_
// was: rbx::signals::connection::operator!=(rbx::signals::connection const&)const
pub fn stub_2452d0() -> ! {
    todo!("0x2452d0 __ZNK3rbx7signals10connectionneERKS1_")
}

#[doc(alias = "rbx::signals::connection::operator=(rbx::signals::connection const&)")]
// 0x245488 — __ZN3rbx7signals10connectionaSERKS1_
// was: rbx::signals::connection::operator=(rbx::signals::connection const&)
pub fn stub_245488() -> ! {
    todo!("0x245488 __ZN3rbx7signals10connectionaSERKS1_")
}

#[doc(alias = "boost::function<void ()(std::exception &)>::~function()")]
// 0x24551c — __ZN5boost8functionIFvRSt9exceptionEED1Ev
// was: boost::function<void ()(std::exception &)>::~function()
pub fn stub_24551c() -> ! {
    todo!("0x24551c __ZN5boost8functionIFvRSt9exceptionEED1Ev")
}

#[doc(alias = "Init::initStaticData(void)")]
// 0x245544 — __ZN4Init14initStaticDataEv
// was: Init::initStaticData(void)
pub fn stub_245544() -> ! {
    todo!("0x245544 __ZN4Init14initStaticDataEv")
}

#[doc(alias = "global constructor keyed to_a_48")]
// 0x245548 — __GLOBAL__I_a_48
// was: `global constructor keyed to'_a_48
pub fn stub_245548() -> ! {
    todo!("0x245548 __GLOBAL__I_a_48")
}

#[doc(alias = "RBX::Tasks::SequenceBase::isInhibited(RBX::TaskScheduler::Job *)")]
// 0x2456a0 — __ZN3RBX5Tasks12SequenceBase11isInhibitedEPNS_13TaskScheduler3JobE
// was: RBX::Tasks::SequenceBase::isInhibited(RBX::TaskScheduler::Job *)
pub fn stub_2456a0() -> ! {
    todo!("0x2456a0 __ZN3RBX5Tasks12SequenceBase11isInhibitedEPNS_13TaskScheduler3JobE")
}

#[doc(alias = "RBX::Tasks::SequenceBase::advance(void)")]
// 0x2456d8 — __ZN3RBX5Tasks12SequenceBase7advanceEv
// was: RBX::Tasks::SequenceBase::advance(void)
pub fn stub_2456d8() -> ! {
    todo!("0x2456d8 __ZN3RBX5Tasks12SequenceBase7advanceEv")
}

#[doc(alias = "RBX::Tasks::SequenceBase::onAdded(RBX::TaskScheduler::Job *)")]
// 0x245708 — __ZN3RBX5Tasks12SequenceBase7onAddedEPNS_13TaskScheduler3JobE
// was: RBX::Tasks::SequenceBase::onAdded(RBX::TaskScheduler::Job *)
pub fn stub_245708() -> ! {
    todo!("0x245708 __ZN3RBX5Tasks12SequenceBase7onAddedEPNS_13TaskScheduler3JobE")
}

#[doc(alias = "RBX::Tasks::SequenceBase::onRemoved(RBX::TaskScheduler::Job *)")]
// 0x2457f0 — __ZN3RBX5Tasks12SequenceBase9onRemovedEPNS_13TaskScheduler3JobE
// was: RBX::Tasks::SequenceBase::onRemoved(RBX::TaskScheduler::Job *)
pub fn stub_2457f0() -> ! {
    todo!("0x2457f0 __ZN3RBX5Tasks12SequenceBase9onRemovedEPNS_13TaskScheduler3JobE")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job **,std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>>,RBX::TaskScheduler::Job * const&)")]
// 0x245848 — __ZNSt6vectorIPN3RBX13TaskScheduler3JobESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// was: std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job **,std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>>,RBX::TaskScheduler::Job * const&)
pub fn stub_245848() -> ! {
    todo!("0x245848 __ZNSt6vectorIPN3RBX13TaskScheduler3JobESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "global constructor keyed to_a_49")]
// 0x245940 — __GLOBAL__I_a_49
// was: `global constructor keyed to'_a_49
pub fn stub_245940() -> ! {
    todo!("0x245940 __GLOBAL__I_a_49")
}

#[doc(alias = "RBX::TaskScheduler::getSchedulerDutyCyclePerThread(void)const")]
// 0x245a08 — __ZNK3RBX13TaskScheduler30getSchedulerDutyCyclePerThreadEv
// was: RBX::TaskScheduler::getSchedulerDutyCyclePerThread(void)const
pub fn stub_245a08() -> ! {
    todo!("0x245a08 __ZNK3RBX13TaskScheduler30getSchedulerDutyCyclePerThreadEv")
}

#[doc(alias = "RBX::ExclusiveArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
// 0x245ab0 — __ZN3RBX16ExclusiveArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_
// was: RBX::ExclusiveArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)
pub fn stub_245ab0() -> ! {
    todo!("0x245ab0 __ZN3RBX16ExclusiveArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_")
}

#[doc(alias = "RBX::TaskScheduler::static_init(void)")]
// 0x245b68 — __ZN3RBX13TaskScheduler11static_initEv
// was: RBX::TaskScheduler::static_init(void)
pub fn stub_245b68() -> ! {
    todo!("0x245b68 __ZN3RBX13TaskScheduler11static_initEv")
}

#[doc(alias = "RBX::TaskScheduler::~TaskScheduler()")]
// 0x245c64 — __ZN3RBX13TaskSchedulerD1Ev
// was: RBX::TaskScheduler::~TaskScheduler()
pub fn stub_245c64() -> ! {
    todo!("0x245c64 __ZN3RBX13TaskSchedulerD1Ev")
}

#[doc(alias = "RBX::TaskScheduler::singleton(void)")]
// 0x245c70 — __ZN3RBX13TaskScheduler9singletonEv
// was: RBX::TaskScheduler::singleton(void)
pub fn stub_245c70() -> ! {
    todo!("0x245c70 __ZN3RBX13TaskScheduler9singletonEv")
}

#[doc(alias = "RBX::TaskScheduler::TaskScheduler(void)")]
// 0x245c94 — __ZN3RBX13TaskSchedulerC2Ev
// was: RBX::TaskScheduler::TaskScheduler(void)
pub fn stub_245c94() -> ! {
    todo!("0x245c94 __ZN3RBX13TaskSchedulerC2Ev")
}

#[doc(alias = "RBX::TaskScheduler::sampleRunningJobCount(void)")]
// 0x246308 — __ZN3RBX13TaskScheduler21sampleRunningJobCountEv
// was: RBX::TaskScheduler::sampleRunningJobCount(void)
pub fn stub_246308() -> ! {
    todo!("0x246308 __ZN3RBX13TaskScheduler21sampleRunningJobCountEv")
}

#[doc(alias = "RBX::TaskScheduler::~TaskScheduler()")]
// 0x246358 — __ZN3RBX13TaskSchedulerD2Ev
// was: RBX::TaskScheduler::~TaskScheduler()
pub fn stub_246358() -> ! {
    todo!("0x246358 __ZN3RBX13TaskSchedulerD2Ev")
}

#[doc(alias = "RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job>,bool,boost::function<void ()(void)>)")]
// 0x2467d0 — __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEEbNS1_8functionIFvvEEE
// was: RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job>,bool,boost::function<void ()(void)>)
// now: RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,bool,boost::function<void ()(void)>) // rbx_core::SharedPtr
pub fn stub_2467d0() -> ! {
    todo!("0x2467d0 __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEEbNS1_8functionIFvvEEE")
}

#[doc(alias = "RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job> const&,boost::shared_ptr<RBX::CEvent>)")]
// 0x246a48 — __ZN3RBX13TaskScheduler6removeERKN5boost10shared_ptrINS0_3JobEEENS2_INS_6CEventEEE
// was: RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job> const&,boost::shared_ptr<RBX::CEvent>)
// now: RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&,rbx_core::SharedPtr<RBX::CEvent>) // rbx_core::SharedPtr
pub fn stub_246a48() -> ! {
    todo!("0x246a48 __ZN3RBX13TaskScheduler6removeERKN5boost10shared_ptrINS0_3JobEEENS2_INS_6CEventEEE")
}

#[doc(alias = "RBX::TaskScheduler::reschedule(boost::shared_ptr<RBX::TaskScheduler::Job>)")]
// 0x246da8 — __ZN3RBX13TaskScheduler10rescheduleEN5boost10shared_ptrINS0_3JobEEE
// was: RBX::TaskScheduler::reschedule(boost::shared_ptr<RBX::TaskScheduler::Job>)
// now: RBX::TaskScheduler::reschedule(rbx_core::SharedPtr<RBX::TaskScheduler::Job>) // rbx_core::SharedPtr
pub fn stub_246da8() -> ! {
    todo!("0x246da8 __ZN3RBX13TaskScheduler10rescheduleEN5boost10shared_ptrINS0_3JobEEE")
}

#[doc(alias = "RBX::TaskScheduler::scheduleJob(RBX::TaskScheduler::Job &)")]
// 0x246e98 — __ZN3RBX13TaskScheduler11scheduleJobERNS0_3JobE
// was: RBX::TaskScheduler::scheduleJob(RBX::TaskScheduler::Job &)
pub fn stub_246e98() -> ! {
    todo!("0x246e98 __ZN3RBX13TaskScheduler11scheduleJobERNS0_3JobE")
}

#[doc(alias = "RBX::TaskScheduler::add(boost::shared_ptr<RBX::TaskScheduler::Job>)")]
// 0x246f90 — __ZN3RBX13TaskScheduler3addEN5boost10shared_ptrINS0_3JobEEE
// was: RBX::TaskScheduler::add(boost::shared_ptr<RBX::TaskScheduler::Job>)
// now: RBX::TaskScheduler::add(rbx_core::SharedPtr<RBX::TaskScheduler::Job>) // rbx_core::SharedPtr
pub fn stub_246f90() -> ! {
    todo!("0x246f90 __ZN3RBX13TaskScheduler3addEN5boost10shared_ptrINS0_3JobEEE")
}

#[doc(alias = "RBX::TaskScheduler::incrementThreadCount(void)")]
// 0x24710c — __ZN3RBX13TaskScheduler20incrementThreadCountEv
// was: RBX::TaskScheduler::incrementThreadCount(void)
pub fn stub_24710c() -> ! {
    todo!("0x24710c __ZN3RBX13TaskScheduler20incrementThreadCountEv")
}

#[doc(alias = "RBX::TaskScheduler::decrementThreadCount(void)")]
// 0x24711c — __ZN3RBX13TaskScheduler20decrementThreadCountEv
// was: RBX::TaskScheduler::decrementThreadCount(void)
pub fn stub_24711c() -> ! {
    todo!("0x24711c __ZN3RBX13TaskScheduler20decrementThreadCountEv")
}

#[doc(alias = "RBX::TaskScheduler::getShortestSleepTime(void)const")]
// 0x247130 — __ZNK3RBX13TaskScheduler20getShortestSleepTimeEv
// was: RBX::TaskScheduler::getShortestSleepTime(void)const
pub fn stub_247130() -> ! {
    todo!("0x247130 __ZNK3RBX13TaskScheduler20getShortestSleepTimeEv")
}

#[doc(alias = "RBX::TaskScheduler::wakeSleepingJobs(void)")]
// 0x247154 — __ZN3RBX13TaskScheduler16wakeSleepingJobsEv
// was: RBX::TaskScheduler::wakeSleepingJobs(void)
pub fn stub_247154() -> ! {
    todo!("0x247154 __ZN3RBX13TaskScheduler16wakeSleepingJobsEv")
}

#[doc(alias = "RBX::TaskScheduler::findJobToRun(boost::shared_ptr<RBX::TaskScheduler::Thread>)")]
// 0x247220 — __ZN3RBX13TaskScheduler12findJobToRunEN5boost10shared_ptrINS0_6ThreadEEE
// was: RBX::TaskScheduler::findJobToRun(boost::shared_ptr<RBX::TaskScheduler::Thread>)
// now: RBX::TaskScheduler::findJobToRun(rbx_core::SharedPtr<RBX::TaskScheduler::Thread>) // rbx_core::SharedPtr
pub fn stub_247220() -> ! {
    todo!("0x247220 __ZN3RBX13TaskScheduler12findJobToRunEN5boost10shared_ptrINS0_6ThreadEEE")
}

#[doc(alias = "rbx::thread_specific_reference<RBX::TaskScheduler::Job>::~thread_specific_reference()")]
// 0x247bd8 — __ZN3rbx25thread_specific_referenceIN3RBX13TaskScheduler3JobEED1Ev
// was: rbx::thread_specific_reference<RBX::TaskScheduler::Job>::~thread_specific_reference()
pub fn stub_247bd8() -> ! {
    todo!("0x247bd8 __ZN3rbx25thread_specific_referenceIN3RBX13TaskScheduler3JobEED1Ev")
}

#[doc(alias = "RBX::TaskScheduler::Job::getDebugName(void)const")]
// 0x247be8 — __ZNK3RBX13TaskScheduler3Job12getDebugNameEv
// was: RBX::TaskScheduler::Job::getDebugName(void)const
pub fn stub_247be8() -> ! {
    todo!("0x247be8 __ZNK3RBX13TaskScheduler3Job12getDebugNameEv")
}

#[doc(alias = "RBX::RunningAverage<int,double>::sample(int)")]
// 0x247db0 — __ZN3RBX14RunningAverageIidE6sampleEi
// was: RBX::RunningAverage<int,double>::sample(int)
pub fn stub_247db0() -> ! {
    todo!("0x247db0 __ZN3RBX14RunningAverageIidE6sampleEi")
}

#[doc(alias = "RBX::ExclusiveArbiter::arbiterName(void)")]
// 0x247e74 — __ZN3RBX16ExclusiveArbiter11arbiterNameEv
// was: RBX::ExclusiveArbiter::arbiterName(void)
pub fn stub_247e74() -> ! {
    todo!("0x247e74 __ZN3RBX16ExclusiveArbiter11arbiterNameEv")
}

#[doc(alias = "RBX::ExclusiveArbiter::isThrottled(void)")]
// 0x247e90 — __ZN3RBX16ExclusiveArbiter11isThrottledEv
// was: RBX::ExclusiveArbiter::isThrottled(void)
pub fn stub_247e90() -> ! {
    todo!("0x247e90 __ZN3RBX16ExclusiveArbiter11isThrottledEv")
}

#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(boost::weak_ptr<RBX::TaskScheduler::Job> const&)")]
// 0x247e94 — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE
// was: boost::shared_ptr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(boost::weak_ptr<RBX::TaskScheduler::Job> const&)
// now: rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(boost::weak_ptr<RBX::TaskScheduler::Job> const&) // rbx_core::SharedPtr
pub fn stub_247e94() -> ! {
    todo!("0x247e94 __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE")
}

#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>)")]
// 0x247fac — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>>)
// now: std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>) // rbx_core::SharedPtr
pub fn stub_247fac() -> ! {
    todo!("0x247fac __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_")
}

#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::TaskScheduler::Job>> *)")]
// 0x248020 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::TaskScheduler::Job>> *)
// now: std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::TaskScheduler::Job>> *) // rbx_core::SharedPtr
pub fn stub_248020() -> ! {
    todo!("0x248020 __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_unique(boost::shared_ptr<RBX::TaskScheduler::Job> const&)")]
// 0x248050 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_unique(boost::shared_ptr<RBX::TaskScheduler::Job> const&)
// now: std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&) // rbx_core::SharedPtr
pub fn stub_248050() -> ! {
    todo!("0x248050 __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_")
}
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_create_node(boost::shared_ptr<RBX::TaskScheduler::Job> const&)")]
// 0x248104 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// was: std::_Rb_tree<boost::shared_ptr<RBX::TaskScheduler::Job>,boost::shared_ptr<RBX::TaskScheduler::Job>,std::_Identity<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::less<boost::shared_ptr<RBX::TaskScheduler::Job>>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_create_node(boost::shared_ptr<RBX::TaskScheduler::Job> const&)
// now: std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_create_node(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&) // rbx_core::SharedPtr
pub fn stub_248104() -> ! {
    todo!("0x248104 __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CEvent>(RBX::CEvent *)")]
// 0x248224 — __ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_
// was: boost::detail::shared_count::shared_count<RBX::CEvent>(RBX::CEvent *)
pub fn stub_248224() -> ! {
    todo!("0x248224 __ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")]
// 0x24831c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()
pub fn stub_24831c() -> ! {
    todo!("0x24831c __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")]
// 0x248320 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED0Ev
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()
pub fn stub_248320() -> ! {
    todo!("0x248320 __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::dispose(void)")]
// 0x24832c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::dispose(void)
pub fn stub_24832c() -> ! {
    todo!("0x24832c __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::get_deleter(std::type_info const&)")]
// 0x24834c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::get_deleter(std::type_info const&)
pub fn stub_24834c() -> ! {
    todo!("0x24834c __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::get_untyped_deleter(void)")]
// 0x248350 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::get_untyped_deleter(void)
pub fn stub_248350() -> ! {
    todo!("0x248350 __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::~thread_data()")]
// 0x248358 — __ZN5boost6detail11thread_dataINS_9function0IvEEED1Ev
// was: boost::detail::thread_data<boost::function0<void>>::~thread_data()
pub fn stub_248358() -> ! {
    todo!("0x248358 __ZN5boost6detail11thread_dataINS_9function0IvEEED1Ev")
}

#[doc(alias = "boost::condition_variable::condition_variable(void)")]
// 0x248448 — __ZN5boost18condition_variableC2Ev
// was: boost::condition_variable::condition_variable(void)
pub fn stub_248448() -> ! {
    todo!("0x248448 __ZN5boost18condition_variableC2Ev")
}

#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const")]
// 0x248620 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_9function0IvEEEEEEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const
// now: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const // rbx_core::SharedPtr
pub fn stub_248620() -> ! {
    todo!("0x248620 __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_9function0IvEEEEEEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::get_untyped_deleter(void)")]
// 0x248778 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::get_untyped_deleter(void)
pub fn stub_248778() -> ! {
    todo!("0x248778 __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x24877c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_24877c() -> ! {
    todo!("0x24877c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x2487dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_2487dc() -> ! {
    todo!("0x2487dc __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&)")]
// 0x2487f8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&)
pub fn stub_2487f8() -> ! {
    todo!("0x2487f8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_")
}

#[doc(alias = "boost::function0<void>::dummy::nonnull(void)")]
// 0x248938 — __ZN5boost9function0IvE5dummy7nonnullEv
// was: boost::function0<void>::dummy::nonnull(void)
pub fn stub_248938() -> ! {
    todo!("0x248938 __ZN5boost9function0IvE5dummy7nonnullEv")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::~thread_specific_ptr()")]
// 0x248a8c — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::~thread_specific_ptr()
pub fn stub_248a8c() -> ! {
    todo!("0x248a8c __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")]
// 0x248b80 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD1Ev
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()
pub fn stub_248b80() -> ! {
    todo!("0x248b80 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD1Ev")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")]
// 0x248b84 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD0Ev
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()
pub fn stub_248b84() -> ! {
    todo!("0x248b84 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD0Ev")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::operator()(void *)")]
// 0x248b90 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataclEPv
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::operator()(void *)
pub fn stub_248b90() -> ! {
    todo!("0x248b90 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataclEPv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()")]
// 0x248ba0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()
pub fn stub_248ba0() -> ! {
    todo!("0x248ba0 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()")]
// 0x248ba4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()
pub fn stub_248ba4() -> ! {
    todo!("0x248ba4 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::dispose(void)")]
// 0x248bb0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::dispose(void)
pub fn stub_248bb0() -> ! {
    todo!("0x248bb0 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_deleter(std::type_info const&)")]
// 0x248bc4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_deleter(std::type_info const&)
pub fn stub_248bc4() -> ! {
    todo!("0x248bc4 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_untyped_deleter(void)")]
// 0x248bdc — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_untyped_deleter(void)
pub fn stub_248bdc() -> ! {
    todo!("0x248bdc __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::averageDutyCycle(void)const")]
// 0x248e38 — __ZNK3RBX13TaskScheduler3Job16averageDutyCycleEv
// was: RBX::TaskScheduler::Job::averageDutyCycle(void)const
pub fn stub_248e38() -> ! {
    todo!("0x248e38 __ZNK3RBX13TaskScheduler3Job16averageDutyCycleEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::averageStepsPerSecond(void)const")]
// 0x248eb0 — __ZNK3RBX13TaskScheduler3Job21averageStepsPerSecondEv
// was: RBX::TaskScheduler::Job::averageStepsPerSecond(void)const
pub fn stub_248eb0() -> ! {
    todo!("0x248eb0 __ZNK3RBX13TaskScheduler3Job21averageStepsPerSecondEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::averageStepTime(void)const")]
// 0x248f10 — __ZNK3RBX13TaskScheduler3Job15averageStepTimeEv
// was: RBX::TaskScheduler::Job::averageStepTime(void)const
pub fn stub_248f10() -> ! {
    todo!("0x248f10 __ZNK3RBX13TaskScheduler3Job15averageStepTimeEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::averageError(void)const")]
// 0x248f20 — __ZNK3RBX13TaskScheduler3Job12averageErrorEv
// was: RBX::TaskScheduler::Job::averageError(void)const
pub fn stub_248f20() -> ! {
    todo!("0x248f20 __ZNK3RBX13TaskScheduler3Job12averageErrorEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::removeCoordinator(boost::shared_ptr<RBX::Tasks::Coordinator>)")]
// 0x248f2c — __ZN3RBX13TaskScheduler3Job17removeCoordinatorEN5boost10shared_ptrINS_5Tasks11CoordinatorEEE
// was: RBX::TaskScheduler::Job::removeCoordinator(boost::shared_ptr<RBX::Tasks::Coordinator>)
// now: RBX::TaskScheduler::Job::removeCoordinator(rbx_core::SharedPtr<RBX::Tasks::Coordinator>) // rbx_core::SharedPtr
pub fn stub_248f2c() -> ! {
    todo!("0x248f2c __ZN3RBX13TaskScheduler3Job17removeCoordinatorEN5boost10shared_ptrINS_5Tasks11CoordinatorEEE")
}

#[doc(alias = "RBX::TaskScheduler::Job::addCoordinator(boost::shared_ptr<RBX::Tasks::Coordinator>)")]
// 0x2490ac — __ZN3RBX13TaskScheduler3Job14addCoordinatorEN5boost10shared_ptrINS_5Tasks11CoordinatorEEE
// was: RBX::TaskScheduler::Job::addCoordinator(boost::shared_ptr<RBX::Tasks::Coordinator>)
// now: RBX::TaskScheduler::Job::addCoordinator(rbx_core::SharedPtr<RBX::Tasks::Coordinator>) // rbx_core::SharedPtr
pub fn stub_2490ac() -> ! {
    todo!("0x2490ac __ZN3RBX13TaskScheduler3Job14addCoordinatorEN5boost10shared_ptrINS_5Tasks11CoordinatorEEE")
}

#[doc(alias = "RBX::TaskScheduler::Job::isDisabled(void)")]
// 0x249184 — __ZN3RBX13TaskScheduler3Job10isDisabledEv
// was: RBX::TaskScheduler::Job::isDisabled(void)
pub fn stub_249184() -> ! {
    todo!("0x249184 __ZN3RBX13TaskScheduler3Job10isDisabledEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::Job(char const*,boost::shared_ptr<RBX::TaskScheduler::Arbiter>,RBX::Time::Interval)")]
// 0x249270 — __ZN3RBX13TaskScheduler3JobC2EPKcN5boost10shared_ptrINS0_7ArbiterEEENS_4Time8IntervalE
// was: RBX::TaskScheduler::Job::Job(char const*,boost::shared_ptr<RBX::TaskScheduler::Arbiter>,RBX::Time::Interval)
// now: RBX::TaskScheduler::Job::Job(char const*,rbx_core::SharedPtr<RBX::TaskScheduler::Arbiter>,RBX::Time::Interval) // rbx_core::SharedPtr
pub fn stub_249270() -> ! {
    todo!("0x249270 __ZN3RBX13TaskScheduler3JobC2EPKcN5boost10shared_ptrINS0_7ArbiterEEENS_4Time8IntervalE")
}

#[doc(alias = "RBX::TaskScheduler::Job::~Job()")]
// 0x249920 — __ZN3RBX13TaskScheduler3JobD0Ev
// was: RBX::TaskScheduler::Job::~Job()
pub fn stub_249920() -> ! {
    todo!("0x249920 __ZN3RBX13TaskScheduler3JobD0Ev")
}

#[doc(alias = "RBX::TaskScheduler::Job::~Job()")]
// 0x2499c0 — __ZN3RBX13TaskScheduler3JobD1Ev
// was: RBX::TaskScheduler::Job::~Job()
pub fn stub_2499c0() -> ! {
    todo!("0x2499c0 __ZN3RBX13TaskScheduler3JobD1Ev")
}

#[doc(alias = "RBX::TaskScheduler::Job::~Job()")]
// 0x2499cc — __ZN3RBX13TaskScheduler3JobD2Ev
// was: RBX::TaskScheduler::Job::~Job()
pub fn stub_2499cc() -> ! {
    todo!("0x2499cc __ZN3RBX13TaskScheduler3JobD2Ev")
}

#[doc(alias = "RBX::TaskScheduler::Job::computeStandardError(RBX::TaskScheduler::Job::Stats const&,double)")]
// 0x24a1f8 — __ZN3RBX13TaskScheduler3Job20computeStandardErrorERKNS1_5StatsEd
// was: RBX::TaskScheduler::Job::computeStandardError(RBX::TaskScheduler::Job::Stats const&,double)
pub fn stub_24a1f8() -> ! {
    todo!("0x24a1f8 __ZN3RBX13TaskScheduler3Job20computeStandardErrorERKNS1_5StatsEd")
}

#[doc(alias = "RBX::TaskScheduler::Job::computeStandardSleepTime(RBX::TaskScheduler::Job::Stats const&,double)")]
// 0x24a210 — __ZN3RBX13TaskScheduler3Job24computeStandardSleepTimeERKNS1_5StatsEd
// was: RBX::TaskScheduler::Job::computeStandardSleepTime(RBX::TaskScheduler::Job::Stats const&,double)
pub fn stub_24a210() -> ! {
    todo!("0x24a210 __ZN3RBX13TaskScheduler3Job24computeStandardSleepTimeERKNS1_5StatsEd")
}

#[doc(alias = "RBX::TaskScheduler::Job::Stats::Stats(RBX::TaskScheduler::Job&,RBX::Time)")]
// 0x24a408 — __ZN3RBX13TaskScheduler3Job5StatsC1ERS1_NS_4TimeE
// was: RBX::TaskScheduler::Job::Stats::Stats(RBX::TaskScheduler::Job&,RBX::Time)
pub fn stub_24a408() -> ! {
    todo!("0x24a408 __ZN3RBX13TaskScheduler3Job5StatsC1ERS1_NS_4TimeE")
}

#[doc(alias = "RBX::TaskScheduler::Job::startWaiting(void)")]
// 0x24a440 — __ZN3RBX13TaskScheduler3Job12startWaitingEv
// was: RBX::TaskScheduler::Job::startWaiting(void)
pub fn stub_24a440() -> ! {
    todo!("0x24a440 __ZN3RBX13TaskScheduler3Job12startWaitingEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::startSleeping(void)")]
// 0x24a448 — __ZN3RBX13TaskScheduler3Job13startSleepingEv
// was: RBX::TaskScheduler::Job::startSleeping(void)
pub fn stub_24a448() -> ! {
    todo!("0x24a448 __ZN3RBX13TaskScheduler3Job13startSleepingEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::updateWakeTime(void)")]
// 0x24a468 — __ZN3RBX13TaskScheduler3Job14updateWakeTimeEv
// was: RBX::TaskScheduler::Job::updateWakeTime(void)
pub fn stub_24a468() -> ! {
    todo!("0x24a468 __ZN3RBX13TaskScheduler3Job14updateWakeTimeEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::updateError(RBX::Time const&)")]
// 0x24a4c0 — __ZN3RBX13TaskScheduler3Job11updateErrorERKNS_4TimeE
// was: RBX::TaskScheduler::Job::updateError(RBX::Time const&)
pub fn stub_24a4c0() -> ! {
    todo!("0x24a4c0 __ZN3RBX13TaskScheduler3Job11updateErrorERKNS_4TimeE")
}

#[doc(alias = "RBX::TaskScheduler::Job::notifyCoordinatorsPreStep(void)")]
// 0x24a598 — __ZN3RBX13TaskScheduler3Job25notifyCoordinatorsPreStepEv
// was: RBX::TaskScheduler::Job::notifyCoordinatorsPreStep(void)
pub fn stub_24a598() -> ! {
    todo!("0x24a598 __ZN3RBX13TaskScheduler3Job25notifyCoordinatorsPreStepEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::preStep(void)")]
// 0x24a684 — __ZN3RBX13TaskScheduler3Job7preStepEv
// was: RBX::TaskScheduler::Job::preStep(void)
pub fn stub_24a684() -> ! {
    todo!("0x24a684 __ZN3RBX13TaskScheduler3Job7preStepEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::postStep(RBX::TaskScheduler::StepResult)")]
// 0x24a8b8 — __ZN3RBX13TaskScheduler3Job8postStepENS0_10StepResultE
// was: RBX::TaskScheduler::Job::postStep(RBX::TaskScheduler::StepResult)
pub fn stub_24a8b8() -> ! {
    todo!("0x24a8b8 __ZN3RBX13TaskScheduler3Job8postStepENS0_10StepResultE")
}

#[doc(alias = "RBX::TaskScheduler::Job::notifyCoordinatorsPostStep(void)")]
// 0x24ab18 — __ZN3RBX13TaskScheduler3Job26notifyCoordinatorsPostStepEv
// was: RBX::TaskScheduler::Job::notifyCoordinatorsPostStep(void)
pub fn stub_24ab18() -> ! {
    todo!("0x24ab18 __ZN3RBX13TaskScheduler3Job26notifyCoordinatorsPostStepEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::updatePriority(void)")]
// 0x24ac08 — __ZN3RBX13TaskScheduler3Job14updatePriorityEv
// was: RBX::TaskScheduler::Job::updatePriority(void)
pub fn stub_24ac08() -> ! {
    todo!("0x24ac08 __ZN3RBX13TaskScheduler3Job14updatePriorityEv")
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>::push_back(boost::shared_ptr<RBX::Tasks::Coordinator> const&)")]
// 0x24ad00 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE9push_backERKS5_
// was: std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>::push_back(boost::shared_ptr<RBX::Tasks::Coordinator> const&)
// now: std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::push_back(rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&) // rbx_core::SharedPtr
pub fn stub_24ad00() -> ! {
    todo!("0x24ad00 __ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE9push_backERKS5_")
}

#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::allocate(unsigned long)")]
// 0x24ae88 — __ZN5boost15circular_bufferIdSaIdEE8allocateEm
// was: boost::circular_buffer<double,std::allocator<double>>::allocate(unsigned long)
pub fn stub_24ae88() -> ! {
    todo!("0x24ae88 __ZN5boost15circular_bufferIdSaIdEE8allocateEm")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// 0x24afb0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()
pub fn stub_24afb0() -> ! {
    todo!("0x24afb0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const")]
// 0x24b070 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const
pub fn stub_24b070() -> ! {
    todo!("0x24b070 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const")]
// 0x24b120 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::rethrow(void)const
pub fn stub_24b120() -> ! {
    todo!("0x24b120 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE7rethrowEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// 0x24b130 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()
pub fn stub_24b130() -> ! {
    todo!("0x24b130 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// 0x24b208 — __ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED0Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()
pub fn stub_24b208() -> ! {
    todo!("0x24b208 __ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED0Ev")
}

#[doc(alias = "RBX::RunningAverage<double,double>::RunningAverage(double,double,unsigned int)")]
// 0x24b45c — __ZN3RBX14RunningAverageIddEC2Eddj
// was: RBX::RunningAverage<double,double>::RunningAverage(double,double,unsigned int)
pub fn stub_24b45c() -> ! {
    todo!("0x24b45c __ZN3RBX14RunningAverageIddEC2Eddj")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::circular_buffer<double,std::allocator<double>>>(boost::circular_buffer<double,std::allocator<double>> *)")]
// 0x24b5a4 — __ZN5boost6detail12shared_countC2INS_15circular_bufferIdSaIdEEEEEPT_
// was: boost::detail::shared_count::shared_count<boost::circular_buffer<double,std::allocator<double>>>(boost::circular_buffer<double,std::allocator<double>> *)
pub fn stub_24b5a4() -> ! {
    todo!("0x24b5a4 __ZN5boost6detail12shared_countC2INS_15circular_bufferIdSaIdEEEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()")]
// 0x24b6c8 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEED1Ev
// was: boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()
pub fn stub_24b6c8() -> ! {
    todo!("0x24b6c8 __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()")]
// 0x24b6cc — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEED0Ev
// was: boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::~sp_counted_impl_p()
pub fn stub_24b6cc() -> ! {
    todo!("0x24b6cc __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::dispose(void)")]
// 0x24b6d8 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE7disposeEv
// was: boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::dispose(void)
pub fn stub_24b6d8() -> ! {
    todo!("0x24b6d8 __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_deleter(std::type_info const&)")]
// 0x24b714 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_deleter(std::type_info const&)
pub fn stub_24b714() -> ! {
    todo!("0x24b714 __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_untyped_deleter(void)")]
// 0x24b718 — __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<boost::circular_buffer<double,std::allocator<double>>>::get_untyped_deleter(void)
pub fn stub_24b718() -> ! {
    todo!("0x24b718 __ZN5boost6detail17sp_counted_impl_pINS_15circular_bufferIdSaIdEEEE19get_untyped_deleterEv")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>> std::__find_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>,std::random_access_iterator_tag)")]
// 0x24b71c — __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESt6vectorIS7_SaIS7_EEEENS2_3_bi6bind_tIbNS2_4_mfi3mf1IbS6_PNS4_13TaskScheduler3JobEEENSD_5list2INS2_3argILi1EEENSD_5valueISJ_EEEEEEET_SS_SS_T0_St26random_access_iterator_tag
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>> std::__find_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator> *,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>,std::random_access_iterator_tag)
// now: __gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>> std::__find_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Tasks::Coordinator,RBX::TaskScheduler::Job *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::TaskScheduler::Job *>>>,std::random_access_iterator_tag) // rbx_core::SharedPtr
pub fn stub_24b71c() -> ! {
    todo!("0x24b71c __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESt6vectorIS7_SaIS7_EEEENS2_3_bi6bind_tIbNS2_4_mfi3mf1IbS6_PNS4_13TaskScheduler3JobEEENSD_5list2INS2_3argILi1EEENSD_5valueISJ_EEEEEEET_SS_SS_T0_St26random_access_iterator_tag")
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator>*,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::shared_ptr<RBX::Tasks::Coordinator> const&)")]
// 0x24b860 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// was: std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Tasks::Coordinator>*,std::vector<boost::shared_ptr<RBX::Tasks::Coordinator>,std::allocator<boost::shared_ptr<RBX::Tasks::Coordinator>>>>,boost::shared_ptr<RBX::Tasks::Coordinator> const&)
// now: std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>*,std::vector<rbx_core::SharedPtr<RBX::Tasks::Coordinator>,std::allocator<rbx_core::SharedPtr<RBX::Tasks::Coordinator>>>>,rbx_core::SharedPtr<RBX::Tasks::Coordinator> const&) // rbx_core::SharedPtr
pub fn stub_24b860() -> ! {
    todo!("0x24b860 __ZNSt6vectorIN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")
}

#[doc(alias = "boost::shared_ptr<RBX::Tasks::Coordinator> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *>(boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *)")]
// 0x24bdf8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::Tasks::Coordinator> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *>(boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *)
// now: rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *) // rbx_core::SharedPtr
pub fn stub_24bdf8() -> ! {
    todo!("0x24bdf8 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_")
}

#[doc(alias = "boost::shared_ptr<RBX::Tasks::Coordinator> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *>(boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *)")]
// 0x24beb0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::Tasks::Coordinator> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *>(boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *,boost::shared_ptr<RBX::Tasks::Coordinator> *)
// now: rbx_core::SharedPtr<RBX::Tasks::Coordinator> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *>(rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *,rbx_core::SharedPtr<RBX::Tasks::Coordinator> *) // rbx_core::SharedPtr
pub fn stub_24beb0() -> ! {
    todo!("0x24beb0 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX5Tasks11CoordinatorEEES9_EET0_T_SB_SA_")
}

#[doc(alias = "RBX::TaskScheduler::endAllThreads(void)")]
// 0x24c02c — __ZN3RBX13TaskScheduler13endAllThreadsEv
// was: RBX::TaskScheduler::endAllThreads(void)
pub fn stub_24c02c() -> ! {
    todo!("0x24c02c __ZN3RBX13TaskScheduler13endAllThreadsEv")
}

#[doc(alias = "RBX::TaskScheduler::setThreadCount(RBX::TaskScheduler::ThreadPoolConfig)")]
// 0x24c048 — __ZN3RBX13TaskScheduler14setThreadCountENS0_16ThreadPoolConfigE
// was: RBX::TaskScheduler::setThreadCount(RBX::TaskScheduler::ThreadPoolConfig)
pub fn stub_24c048() -> ! {
    todo!("0x24c048 __ZN3RBX13TaskScheduler14setThreadCountENS0_16ThreadPoolConfigE")
}

#[doc(alias = "RBX::TaskScheduler::Thread::runJob(void)")]
// 0x24c1ec — __ZN3RBX13TaskScheduler6Thread6runJobEv
// was: RBX::TaskScheduler::Thread::runJob(void)
pub fn stub_24c1ec() -> ! {
    todo!("0x24c1ec __ZN3RBX13TaskScheduler6Thread6runJobEv")
}

#[doc(alias = "RBX::TaskScheduler::conflictsWithScheduledJob(RBX::TaskScheduler::Job *)const")]
// 0x24c43c — __ZNK3RBX13TaskScheduler25conflictsWithScheduledJobEPNS0_3JobE
// was: RBX::TaskScheduler::conflictsWithScheduledJob(RBX::TaskScheduler::Job *)const
pub fn stub_24c43c() -> ! {
    todo!("0x24c43c __ZNK3RBX13TaskScheduler25conflictsWithScheduledJobEPNS0_3JobE")
}

#[doc(alias = "RBX::TaskScheduler::enableThreads(std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>> &)")]
// 0x24c5bc — __ZN3RBX13TaskScheduler13enableThreadsERSt6vectorIN5boost10shared_ptrINS0_6ThreadEEESaIS5_EE
// was: RBX::TaskScheduler::enableThreads(std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>> &)
// now: RBX::TaskScheduler::enableThreads(std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>> &) // rbx_core::SharedPtr
pub fn stub_24c5bc() -> ! {
    todo!("0x24c5bc __ZN3RBX13TaskScheduler13enableThreadsERSt6vectorIN5boost10shared_ptrINS0_6ThreadEEESaIS5_EE")
}

#[doc(alias = "RBX::TaskScheduler::Thread::loop(void)")]
// 0x24c660 — __ZN3RBX13TaskScheduler6Thread4loopEv
// was: RBX::TaskScheduler::Thread::loop(void)
pub fn stub_24c660() -> ! {
    todo!("0x24c660 __ZN3RBX13TaskScheduler6Thread4loopEv")
}

#[doc(alias = "RBX::TaskScheduler::getJobsInfo(std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>> &)")]
// 0x24cd18 — __ZN3RBX13TaskScheduler11getJobsInfoERSt6vectorIN5boost10shared_ptrIKNS0_3JobEEESaIS6_EE
// was: RBX::TaskScheduler::getJobsInfo(std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>> &)
// now: RBX::TaskScheduler::getJobsInfo(std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>> &) // rbx_core::SharedPtr
pub fn stub_24cd18() -> ! {
    todo!("0x24cd18 __ZN3RBX13TaskScheduler11getJobsInfoERSt6vectorIN5boost10shared_ptrIKNS0_3JobEEESaIS6_EE")
}

#[doc(alias = "RBX::TaskScheduler::setJobsExtendedStatsWindow(double)")]
// 0x24ce78 — __ZN3RBX13TaskScheduler26setJobsExtendedStatsWindowEd
// was: RBX::TaskScheduler::setJobsExtendedStatsWindow(double)
pub fn stub_24ce78() -> ! {
    todo!("0x24ce78 __ZN3RBX13TaskScheduler26setJobsExtendedStatsWindowEd")
}

#[doc(alias = "setJobExtendedStatsWindow(boost::shared_ptr<RBX::TaskScheduler::Job>,double)")]
// 0x24d05c — __ZL25setJobExtendedStatsWindowN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEEd
// was: setJobExtendedStatsWindow(boost::shared_ptr<RBX::TaskScheduler::Job>,double)
// now: setJobExtendedStatsWindow(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double) // rbx_core::SharedPtr
pub fn stub_24d05c() -> ! {
    todo!("0x24d05c __ZL25setJobExtendedStatsWindowN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEEd")
}

#[doc(alias = "RBX::TaskScheduler::getJobsByName(std::string const&,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>> &)")]
// 0x24d0ec — __ZN3RBX13TaskScheduler13getJobsByNameERKSsRSt6vectorIN5boost10shared_ptrIKNS0_3JobEEESaIS8_EE
// was: RBX::TaskScheduler::getJobsByName(std::string const&,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>> &)
// now: RBX::TaskScheduler::getJobsByName(std::string const&,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>> &) // rbx_core::SharedPtr
pub fn stub_24d0ec() -> ! {
    todo!("0x24d0ec __ZN3RBX13TaskScheduler13getJobsByNameERKSsRSt6vectorIN5boost10shared_ptrIKNS0_3JobEEESaIS8_EE")
}

#[doc(alias = "RBX::TaskScheduler::Thread::join(void)")]
// 0x24d284 — __ZN3RBX13TaskScheduler6Thread4joinEv
// was: RBX::TaskScheduler::Thread::join(void)
pub fn stub_24d284() -> ! {
    todo!("0x24d284 __ZN3RBX13TaskScheduler6Thread4joinEv")
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Thread> const&)")]
// 0x24d2dc — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE9push_backERKS5_
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Thread> const&)
// now: std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&) // rbx_core::SharedPtr
pub fn stub_24d2dc() -> ! {
    todo!("0x24d2dc __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE9push_backERKS5_")
}

#[doc(alias = "RBX::TaskScheduler::Thread::create(RBX::TaskScheduler*)")]
// 0x24d368 — __ZN3RBX13TaskScheduler6Thread6createEPS0_
// was: RBX::TaskScheduler::Thread::create(RBX::TaskScheduler*)
pub fn stub_24d368() -> ! {
    todo!("0x24d368 __ZN3RBX13TaskScheduler6Thread6createEPS0_")
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Job const> const&)")]
// 0x24d770 — __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE9push_backERKS6_
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Job const> const&)
// now: std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&) // rbx_core::SharedPtr
pub fn stub_24d770() -> ! {
    todo!("0x24d770 __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE9push_backERKS6_")
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Job> const&)")]
// 0x24d7fc — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE9push_backERKS5_
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Job> const&)
// now: std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&) // rbx_core::SharedPtr
pub fn stub_24d7fc() -> ! {
    todo!("0x24d7fc __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE9push_backERKS5_")
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<double>>::operator()<void (*)(boost::shared_ptr<RBX::TaskScheduler::Job>,double),boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::TaskScheduler::Job>,double) &,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job>&> &,int)")]
// 0x24d88c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIdEEEclIPFvNS_10shared_ptrIN3RBX13TaskScheduler3JobEEEdENS0_5list1IRSC_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<double>>::operator()<void (*)(boost::shared_ptr<RBX::TaskScheduler::Job>,double),boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::TaskScheduler::Job>,double) &,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job>&> &,int)
// now: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<double>>::operator()<void (*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double),boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job>&> &,int) // rbx_core::SharedPtr
pub fn stub_24d88c() -> ! {
    todo!("0x24d88c __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIdEEEclIPFvNS_10shared_ptrIN3RBX13TaskScheduler3JobEEEdENS0_5list1IRSC_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>>,boost::shared_ptr<RBX::TaskScheduler::Job> const&)")]
// 0x24d9a4 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>>,boost::shared_ptr<RBX::TaskScheduler::Job> const&)
// now: std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&) // rbx_core::SharedPtr
pub fn stub_24d9a4() -> ! {
    todo!("0x24d9a4 __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")
}

#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *>(boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *)")]
// 0x24df3c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *>(boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *)
// now: rbx_core::SharedPtr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *) // rbx_core::SharedPtr
pub fn stub_24df3c() -> ! {
    todo!("0x24df3c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES9_EET0_T_SB_SA_")
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::shared_ptr<RBX::TaskScheduler::Job const> const&)")]
// 0x24dff8 — __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::shared_ptr<RBX::TaskScheduler::Job const> const&)
// now: std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&) // rbx_core::SharedPtr
pub fn stub_24dff8() -> ! {
    todo!("0x24dff8 __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_")
}

#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *>(boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *)")]
// 0x24e590 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESA_EET0_T_SC_SB_
// was: boost::shared_ptr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *>(boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *)
// now: rbx_core::SharedPtr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *) // rbx_core::SharedPtr
pub fn stub_24e590() -> ! {
    todo!("0x24e590 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESA_EET0_T_SC_SB_")
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::sample(void)")]
// 0x24e648 — __ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE6sampleEv
// was: RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::sample(void)
pub fn stub_24e648() -> ! {
    todo!("0x24e648 __ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE6sampleEv")
}

#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *>(boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *)")]
// 0x24e6a8 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *>(boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *)
// now: rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *) // rbx_core::SharedPtr
pub fn stub_24e6a8() -> ! {
    todo!("0x24e6a8 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_")
}

#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Thread>::shared_ptr<RBX::TaskScheduler::Thread>(boost::weak_ptr<RBX::TaskScheduler::Thread> const&)")]
// 0x24e75c — __ZN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEC2IS3_EERKNS_8weak_ptrIT_EE
// was: boost::shared_ptr<RBX::TaskScheduler::Thread>::shared_ptr<RBX::TaskScheduler::Thread>(boost::weak_ptr<RBX::TaskScheduler::Thread> const&)
// now: rbx_core::SharedPtr<RBX::TaskScheduler::Thread>::shared_ptr<RBX::TaskScheduler::Thread>(rbx_core::WeakPtr<RBX::TaskScheduler::Thread> const&) // rbx_core::SharedPtr
pub fn stub_24e75c() -> ! {
    todo!("0x24e75c __ZN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEC2IS3_EERKNS_8weak_ptrIT_EE")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::reset(RBX::TaskScheduler::Job **)")]
// 0x24e870 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE5resetEPS4_
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::reset(RBX::TaskScheduler::Job **)
pub fn stub_24e870() -> ! {
    todo!("0x24e870 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE5resetEPS4_")
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Thread>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::shared_ptr<RBX::TaskScheduler::Thread> const&)")]
// 0x24e98c — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Thread>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::shared_ptr<RBX::TaskScheduler::Thread> const&)
// now: std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&) // rbx_core::SharedPtr
pub fn stub_24e98c() -> ! {
    todo!("0x24e98c __ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")
}

#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *>(boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *)")]
// 0x24ef24 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *>(boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *)
// now: rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *) // rbx_core::SharedPtr
pub fn stub_24ef24() -> ! {
    todo!("0x24ef24 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_")
}

#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::set_capacity(unsigned long)")]
// 0x24efe0 — __ZN5boost15circular_bufferIdSaIdEE12set_capacityEm
// was: boost::circular_buffer<double,std::allocator<double>>::set_capacity(unsigned long)
pub fn stub_24efe0() -> ! {
    todo!("0x24efe0 __ZN5boost15circular_bufferIdSaIdEE12set_capacityEm")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// 0x24f0d8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()
pub fn stub_24f0d8() -> ! {
    todo!("0x24f0d8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// 0x24f190 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorED1Ev
// was: boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()
pub fn stub_24f190() -> ! {
    todo!("0x24f190 __ZN5boost16exception_detail19error_info_injectorISt12length_errorED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&)")]
// 0x24f248 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&)
pub fn stub_24f248() -> ! {
    todo!("0x24f248 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_tag)")]
// 0x24f388 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_NS5_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_tag)
pub fn stub_24f388() -> ! {
    todo!("0x24f388 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list_av_1<boost::shared_ptr<RBX::TaskScheduler::Thread>>::type> boost::bind<void,RBX::TaskScheduler::Thread,boost::shared_ptr<RBX::TaskScheduler::Thread>>(void (RBX::TaskScheduler::Thread::*)(void),boost::shared_ptr<RBX::TaskScheduler::Thread>)")]
// 0x24f520 — __ZN5boost4bindIvN3RBX13TaskScheduler6ThreadENS_10shared_ptrIS3_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS8_T0_EENS6_9list_av_1IT1_E4typeEEEMSB_FS8_vESE_
// was: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list_av_1<boost::shared_ptr<RBX::TaskScheduler::Thread>>::type> boost::bind<void,RBX::TaskScheduler::Thread,boost::shared_ptr<RBX::TaskScheduler::Thread>>(void (RBX::TaskScheduler::Thread::*)(void),boost::shared_ptr<RBX::TaskScheduler::Thread>)
// now: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>::type> boost::bind<void,RBX::TaskScheduler::Thread,rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>(void (RBX::TaskScheduler::Thread::*)(void),rbx_core::SharedPtr<RBX::TaskScheduler::Thread>) // rbx_core::SharedPtr
pub fn stub_24f520() -> ! {
    todo!("0x24f520 __ZN5boost4bindIvN3RBX13TaskScheduler6ThreadENS_10shared_ptrIS3_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS8_T0_EENS6_9list_av_1IT1_E4typeEEEMSB_FS8_vESE_")
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>)")]
// 0x24f6c0 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13TaskScheduler6ThreadEEEEEEC2ES8_
// was: boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>)
// now: boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>) // rbx_core::SharedPtr
pub fn stub_24f6c0() -> ! {
    todo!("0x24f6c0 __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13TaskScheduler6ThreadEEEEEEC2ES8_")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
// 0x24f808 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
pub fn stub_24f808() -> ! {
    todo!("0x24f808 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>)")]
// 0x24f93c — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>)
// now: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>) // rbx_core::SharedPtr
pub fn stub_24f93c() -> ! {
    todo!("0x24f93c __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x24fa7c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// now: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type) // rbx_core::SharedPtr
pub fn stub_24fa7c() -> ! {
    todo!("0x24fa7c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x24faa0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,void>::invoke(boost::detail::function::function_buffer &)
// now: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,void>::invoke(boost::detail::function::function_buffer &) // rbx_core::SharedPtr
pub fn stub_24faa0() -> ! {
    todo!("0x24faa0 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const")]
// 0x24fac0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const
// now: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const // rbx_core::SharedPtr
pub fn stub_24fac0() -> ! {
    todo!("0x24fac0 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x24fbf4 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// now: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const // rbx_core::SharedPtr
pub fn stub_24fbf4() -> ! {
    todo!("0x24fbf4 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x24fdac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// now: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) // rbx_core::SharedPtr
pub fn stub_24fdac() -> ! {
    todo!("0x24fdac __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::~sp_counted_impl_p()")]
// 0x24ff48 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEED0Ev
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::~sp_counted_impl_p()
pub fn stub_24ff48() -> ! {
    todo!("0x24ff48 __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEED0Ev")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(boost::shared_ptr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const")]
// 0x24ff58 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler6ThreadEE22_internal_accept_ownerIS3_S3_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(boost::shared_ptr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const
// now: void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const // rbx_core::SharedPtr
pub fn stub_24ff58() -> ! {
    todo!("0x24ff58 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler6ThreadEE22_internal_accept_ownerIS3_S3_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskScheduler::Thread>(RBX::TaskScheduler::Thread *)")]
// 0x2500b0 — __ZN5boost6detail12shared_countC2IN3RBX13TaskScheduler6ThreadEEEPT_
// was: boost::detail::shared_count::shared_count<RBX::TaskScheduler::Thread>(RBX::TaskScheduler::Thread *)
pub fn stub_2500b0() -> ! {
    todo!("0x2500b0 __ZN5boost6detail12shared_countC2IN3RBX13TaskScheduler6ThreadEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()")]
// 0x2503f4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()
pub fn stub_2503f4() -> ! {
    todo!("0x2503f4 __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()")]
// 0x2503f8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEED0Ev
// was: boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::~sp_counted_impl_p()
pub fn stub_2503f8() -> ! {
    todo!("0x2503f8 __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::dispose(void)")]
// 0x250404 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::dispose(void)
pub fn stub_250404() -> ! {
    todo!("0x250404 __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_deleter(std::type_info const&)")]
// 0x2504a8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_deleter(std::type_info const&)
pub fn stub_2504a8() -> ! {
    todo!("0x2504a8 __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_untyped_deleter(void)")]
// 0x2504ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RBX::TaskScheduler::Thread>::get_untyped_deleter(void)
pub fn stub_2504ac() -> ! {
    todo!("0x2504ac __ZN5boost6detail17sp_counted_impl_pIN3RBX13TaskScheduler6ThreadEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::thread::timed_join(boost::posix_time::ptime const&)")]
// 0x2504b0 — __ZN5boost6thread10timed_joinERKNS_10posix_time5ptimeE
// was: boost::thread::timed_join(boost::posix_time::ptime const&)
pub fn stub_2504b0() -> ! {
    todo!("0x2504b0 __ZN5boost6thread10timed_joinERKNS_10posix_time5ptimeE")
}

#[doc(alias = "boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::add_time_duration(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::posix_time::time_duration)")]
// 0x250588 — __ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE17add_time_durationERKS5_NS3_13time_durationE
// was: boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::add_time_duration(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::posix_time::time_duration)
pub fn stub_250588() -> ! {
    todo!("0x250588 __ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE17add_time_durationERKS5_NS3_13time_durationE")
}

#[doc(alias = "void boost::throw_exception<std::runtime_error>(std::runtime_error const&)")]
// 0x2506f8 — __ZN5boost15throw_exceptionISt13runtime_errorEEvRKT_
// was: void boost::throw_exception<std::runtime_error>(std::runtime_error const&)
pub fn stub_2506f8() -> ! {
    todo!("0x2506f8 __ZN5boost15throw_exceptionISt13runtime_errorEEvRKT_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()")]
// 0x250848 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()
pub fn stub_250848() -> ! {
    todo!("0x250848 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()")]
// 0x250900 — __ZN5boost16exception_detail19error_info_injectorISt13runtime_errorED1Ev
// was: boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()
pub fn stub_250900() -> ! {
    todo!("0x250900 __ZN5boost16exception_detail19error_info_injectorISt13runtime_errorED1Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()")]
// 0x2509b8 — __ZThn8_N5boost16exception_detail19error_info_injectorISt13runtime_errorED1Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()
pub fn stub_2509b8() -> ! {
    todo!("0x2509b8 __ZThn8_N5boost16exception_detail19error_info_injectorISt13runtime_errorED1Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()")]
// 0x250a70 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()
pub fn stub_250a70() -> ! {
    todo!("0x250a70 __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()")]
// 0x250b28 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()
pub fn stub_250b28() -> ! {
    todo!("0x250b28 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone(void)const")]
// 0x250bf8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE5cloneEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone(void)const
pub fn stub_250bf8() -> ! {
    todo!("0x250bf8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE5cloneEv")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone(void)const")]
// 0x250cb8 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE5cloneEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone(void)const
pub fn stub_250cb8() -> ! {
    todo!("0x250cb8 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()")]
// 0x250d80 — __ZN5boost16exception_detail19error_info_injectorISt13runtime_errorED0Ev
// was: boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()
pub fn stub_250d80() -> ! {
    todo!("0x250d80 __ZN5boost16exception_detail19error_info_injectorISt13runtime_errorED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_tag)")]
// 0x250e40 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEEC1ERKS5_NS5_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_tag)
pub fn stub_250e40() -> ! {
    todo!("0x250e40 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "boost::CV::simple_exception_policy<unsigned short,(unsigned short)1,(unsigned short)31,boost::gregorian::bad_day_of_month>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)")]
// 0x250fc8 — __ZN5boost2CV23simple_exception_policyItLt1ELt31ENS_9gregorian16bad_day_of_monthEE8on_errorEttNS0_14violation_enumE
// was: boost::CV::simple_exception_policy<unsigned short,(unsigned short)1,(unsigned short)31,boost::gregorian::bad_day_of_month>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)
pub fn stub_250fc8() -> ! {
    todo!("0x250fc8 __ZN5boost2CV23simple_exception_policyItLt1ELt31ENS_9gregorian16bad_day_of_monthEE8on_errorEttNS0_14violation_enumE")
}

#[doc(alias = "boost::gregorian::bad_day_of_month::~bad_day_of_month()")]
// 0x25104c — __ZN5boost9gregorian16bad_day_of_monthD1Ev
// was: boost::gregorian::bad_day_of_month::~bad_day_of_month()
pub fn stub_25104c() -> ! {
    todo!("0x25104c __ZN5boost9gregorian16bad_day_of_monthD1Ev")
}

#[doc(alias = "boost::gregorian::bad_day_of_month::bad_day_of_month(void)")]
// 0x251058 — __ZN5boost9gregorian16bad_day_of_monthC2Ev
// was: boost::gregorian::bad_day_of_month::bad_day_of_month(void)
pub fn stub_251058() -> ! {
    todo!("0x251058 __ZN5boost9gregorian16bad_day_of_monthC2Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
// 0x2511a0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()
pub fn stub_2511a0() -> ! {
    todo!("0x2511a0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()")]
// 0x251258 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED1Ev
// was: boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()
pub fn stub_251258() -> ! {
    todo!("0x251258 __ZN5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED1Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()")]
// 0x251310 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()
pub fn stub_251310() -> ! {
    todo!("0x251310 __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED1Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
// 0x2513c8 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()
pub fn stub_2513c8() -> ! {
    todo!("0x2513c8 __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED1Ev")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
// 0x251480 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED1Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()
pub fn stub_251480() -> ! {
    todo!("0x251480 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone(void)const")]
// 0x251550 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE5cloneEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone(void)const
pub fn stub_251550() -> ! {
    todo!("0x251550 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE5cloneEv")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
// 0x25160c — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()
pub fn stub_25160c() -> ! {
    todo!("0x25160c __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED0Ev")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone(void)const")]
// 0x2516c8 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE5cloneEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone(void)const
pub fn stub_2516c8() -> ! {
    todo!("0x2516c8 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE5cloneEv")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::rethrow(void)const")]
// 0x25178c — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE7rethrowEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::rethrow(void)const
pub fn stub_25178c() -> ! {
    todo!("0x25178c __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE7rethrowEv")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
// 0x25179c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED0Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()
pub fn stub_25179c() -> ! {
    todo!("0x25179c __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()")]
// 0x251870 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED0Ev
// was: boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()
pub fn stub_251870() -> ! {
    todo!("0x251870 __ZN5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED0Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()")]
// 0x25192c — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()
pub fn stub_25192c() -> ! {
    todo!("0x25192c __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_tag)")]
// 0x2519e8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEEC1ERKS6_NS6_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_tag)
pub fn stub_2519e8() -> ! {
    todo!("0x2519e8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month> const&)")]
// 0x251b7c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEEC1ERKS5_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month> const&)
pub fn stub_251b7c() -> ! {
    todo!("0x251b7c __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEEC1ERKS5_")
}

#[doc(alias = "boost::CV::simple_exception_policy<unsigned short,(unsigned short)1,(unsigned short)12,boost::gregorian::bad_month>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)")]
// 0x251d10 — __ZN5boost2CV23simple_exception_policyItLt1ELt12ENS_9gregorian9bad_monthEE8on_errorEttNS0_14violation_enumE
// was: boost::CV::simple_exception_policy<unsigned short,(unsigned short)1,(unsigned short)12,boost::gregorian::bad_month>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)
pub fn stub_251d10() -> ! {
    todo!("0x251d10 __ZN5boost2CV23simple_exception_policyItLt1ELt12ENS_9gregorian9bad_monthEE8on_errorEttNS0_14violation_enumE")
}

#[doc(alias = "void boost::throw_exception<boost::gregorian::bad_month>(boost::gregorian::bad_month const&)")]
// 0x251d94 — __ZN5boost15throw_exceptionINS_9gregorian9bad_monthEEEvRKT_
// was: void boost::throw_exception<boost::gregorian::bad_month>(boost::gregorian::bad_month const&)
pub fn stub_251d94() -> ! {
    todo!("0x251d94 __ZN5boost15throw_exceptionINS_9gregorian9bad_monthEEEvRKT_")
}

#[doc(alias = "boost::gregorian::bad_month::bad_month(void)")]
// 0x251ee8 — __ZN5boost9gregorian9bad_monthC2Ev
// was: boost::gregorian::bad_month::bad_month(void)
pub fn stub_251ee8() -> ! {
    todo!("0x251ee8 __ZN5boost9gregorian9bad_monthC2Ev")
}

#[doc(alias = "boost::gregorian::bad_month::~bad_month()")]
// 0x25202c — __ZN5boost9gregorian9bad_monthD0Ev
// was: boost::gregorian::bad_month::~bad_month()
pub fn stub_25202c() -> ! {
    todo!("0x25202c __ZN5boost9gregorian9bad_monthD0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
// 0x252040 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()
pub fn stub_252040() -> ! {
    todo!("0x252040 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()")]
// 0x2520f8 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED1Ev
// was: boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()
pub fn stub_2520f8() -> ! {
    todo!("0x2520f8 __ZN5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED1Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()")]
// 0x2521b0 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()
pub fn stub_2521b0() -> ! {
    todo!("0x2521b0 __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED1Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
// 0x252268 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()
pub fn stub_252268() -> ! {
    todo!("0x252268 __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED1Ev")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
// 0x252320 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED1Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()
pub fn stub_252320() -> ! {
    todo!("0x252320 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
// 0x2523ec — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED0Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()
pub fn stub_2523ec() -> ! {
    todo!("0x2523ec __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone(void)const")]
// 0x2524a8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE5cloneEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone(void)const
pub fn stub_2524a8() -> ! {
    todo!("0x2524a8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::rethrow(void)const")]
// 0x252564 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE7rethrowEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::rethrow(void)const
pub fn stub_252564() -> ! {
    todo!("0x252564 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE7rethrowEv")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone(void)const")]
// 0x252618 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE5cloneEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone(void)const
pub fn stub_252618() -> ! {
    todo!("0x252618 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>> const&)")]
// 0x2526e0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS6_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>> const&)
pub fn stub_2526e0() -> ! {
    todo!("0x2526e0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS6_")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()")]
// 0x252820 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED0Ev
// was: boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()
pub fn stub_252820() -> ! {
    todo!("0x252820 __ZN5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_tag)")]
// 0x2528e0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS6_NS6_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_tag)
pub fn stub_2528e0() -> ! {
    todo!("0x2528e0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::CV::simple_exception_policy<unsigned short,(unsigned short)1400,(unsigned short)10000,boost::gregorian::bad_year>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)")]
// 0x252a78 — __ZN5boost2CV23simple_exception_policyItLt1400ELt10000ENS_9gregorian8bad_yearEE8on_errorEttNS0_14violation_enumE
// was: boost::CV::simple_exception_policy<unsigned short,(unsigned short)1400,(unsigned short)10000,boost::gregorian::bad_year>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)
pub fn stub_252a78() -> ! {
    todo!("0x252a78 __ZN5boost2CV23simple_exception_policyItLt1400ELt10000ENS_9gregorian8bad_yearEE8on_errorEttNS0_14violation_enumE")
}

#[doc(alias = "boost::gregorian::bad_year::~bad_year()")]
// 0x252afc — __ZN5boost9gregorian8bad_yearD1Ev
// was: boost::gregorian::bad_year::~bad_year()
pub fn stub_252afc() -> ! {
    todo!("0x252afc __ZN5boost9gregorian8bad_yearD1Ev")
}

#[doc(alias = "boost::gregorian::bad_year::bad_year(void)")]
// 0x252b08 — __ZN5boost9gregorian8bad_yearC2Ev
// was: boost::gregorian::bad_year::bad_year(void)
pub fn stub_252b08() -> ! {
    todo!("0x252b08 __ZN5boost9gregorian8bad_yearC2Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
// 0x252c50 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()
pub fn stub_252c50() -> ! {
    todo!("0x252c50 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()")]
// 0x252d08 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED1Ev
// was: boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()
pub fn stub_252d08() -> ! {
    todo!("0x252d08 __ZN5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED1Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()")]
// 0x252dc0 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()
pub fn stub_252dc0() -> ! {
    todo!("0x252dc0 __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED1Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
// 0x252e78 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()
pub fn stub_252e78() -> ! {
    todo!("0x252e78 __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED1Ev")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
// 0x252f30 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED1Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()
pub fn stub_252f30() -> ! {
    todo!("0x252f30 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone(void)const")]
// 0x253000 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE5cloneEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone(void)const
pub fn stub_253000() -> ! {
    todo!("0x253000 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE5cloneEv")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
// 0x2530bc — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()
pub fn stub_2530bc() -> ! {
    todo!("0x2530bc __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED0Ev")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone(void)const")]
// 0x253178 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE5cloneEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone(void)const
pub fn stub_253178() -> ! {
    todo!("0x253178 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE5cloneEv")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::rethrow(void)const")]
// 0x25323c — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE7rethrowEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::rethrow(void)const
pub fn stub_25323c() -> ! {
    todo!("0x25323c __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE7rethrowEv")
}

#[doc(alias = "virtual_thunk_to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
// 0x25324c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED0Ev
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()
pub fn stub_25324c() -> ! {
    todo!("0x25324c __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()")]
// 0x253320 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED0Ev
// was: boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()
pub fn stub_253320() -> ! {
    todo!("0x253320 __ZN5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED0Ev")
}

#[doc(alias = "non_virtual_thunk_to boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()")]
// 0x2533dc — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED0Ev
// was: `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()
pub fn stub_2533dc() -> ! {
    todo!("0x2533dc __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_tag)")]
// 0x253498 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEEC1ERKS6_NS6_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_tag)
pub fn stub_253498() -> ! {
    todo!("0x253498 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_year> const&)")]
// 0x25362c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEEC1ERKS5_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_year> const&)
pub fn stub_25362c() -> ! {
    todo!("0x25362c __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEEC1ERKS5_")
}

#[doc(alias = "boost::gregorian::date::date(boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day)")]
// 0x2537c0 — __ZN5boost9gregorian4dateC2ENS0_9greg_yearENS0_10greg_monthENS0_8greg_dayE
// was: boost::gregorian::date::date(boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day)
pub fn stub_2537c0() -> ! {
    todo!("0x2537c0 __ZN5boost9gregorian4dateC2ENS0_9greg_yearENS0_10greg_monthENS0_8greg_dayE")
}

#[doc(alias = "boost::thread::do_try_join_until(timespec const&)")]
// 0x2539a0 — __ZN5boost6thread17do_try_join_untilERK8timespec
// was: boost::thread::do_try_join_until(timespec const&)
pub fn stub_2539a0() -> ! {
    todo!("0x2539a0 __ZN5boost6thread17do_try_join_untilERK8timespec")
}

#[doc(alias = "boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::subtract_times(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&)")]
// 0x253af0 — __ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE14subtract_timesERKS5_S8_
// was: boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::subtract_times(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&)
pub fn stub_253af0() -> ! {
    todo!("0x253af0 __ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE14subtract_timesERKS5_S8_")
}

#[doc(alias = "void boost::algorithm::trim_if<std::string,bool (*)(char)>(std::string &,bool (*)(char))")]
// 0x255cc8 — __ZN5boost9algorithm7trim_ifISsPFbcEEEvRT_T0_
// was: void boost::algorithm::trim_if<std::string,bool (*)(char)>(std::string &,bool (*)(char))
pub fn stub_255cc8() -> ! {
    todo!("0x255cc8 __ZN5boost9algorithm7trim_ifISsPFbcEEEvRT_T0_")
}

#[doc(alias = "RBX::HttpService::userHttpGetAsync(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0x255d98 — __ZN3RBX11HttpService16userHttpGetAsyncESsN5boost8functionIFvSsEEES4_
// was: RBX::HttpService::userHttpGetAsync(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
pub fn stub_255d98() -> ! {
    todo!("0x255d98 __ZN3RBX11HttpService16userHttpGetAsyncESsN5boost8functionIFvSsEEES4_")
}

#[doc(alias = "RBX::HttpService::userHttpPostAsync(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0x25620c — __ZN3RBX11HttpService17userHttpPostAsyncESsSsNS0_15HttpContentTypeEN5boost8functionIFvSsEEES5_
// was: RBX::HttpService::userHttpPostAsync(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
pub fn stub_25620c() -> ! {
    todo!("0x25620c __ZN3RBX11HttpService17userHttpPostAsyncESsSsNS0_15HttpContentTypeEN5boost8functionIFvSsEEES5_")
}

#[doc(alias = "RBX::HttpService::encodeJSON(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
// 0x256d10 — __ZN3RBX11HttpService10encodeJSONEN5boost10shared_ptrIKNS1_9unordered13unordered_mapISsNS_10Reflection7VariantENS1_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEEE
// was: RBX::HttpService::encodeJSON(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)
// now: RBX::HttpService::encodeJSON(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>) // rbx_core::SharedPtr
pub fn stub_256d10() -> ! {
    todo!("0x256d10 __ZN3RBX11HttpService10encodeJSONEN5boost10shared_ptrIKNS1_9unordered13unordered_mapISsNS_10Reflection7VariantENS1_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEEE")
}

#[doc(alias = "RBX::HttpService::checkEverything(std::string &,boost::function<void ()(std::string)>)")]
// 0x257338 — __ZN3RBX11HttpService15checkEverythingERSsN5boost8functionIFvSsEEE
// was: RBX::HttpService::checkEverything(std::string &,boost::function<void ()(std::string)>)
pub fn stub_257338() -> ! {
    todo!("0x257338 __ZN3RBX11HttpService15checkEverythingERSsN5boost8functionIFvSsEEE")
}

#[doc(alias = "void RBX::Reflection::resume_adapter<std::string>(boost::function<void ()(RBX::Reflection::Variant)>,std::string)")]
// 0x257828 — __ZN3RBX10ReflectionL14resume_adapterISsEEvN5boost8functionIFvNS0_7VariantEEEET_
// was: void RBX::Reflection::resume_adapter<std::string>(boost::function<void ()(RBX::Reflection::Variant)>,std::string)
pub fn stub_257828() -> ! {
    todo!("0x257828 __ZN3RBX10ReflectionL14resume_adapterISsEEvN5boost8functionIFvNS0_7VariantEEEET_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc()")]
// 0x257a10 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc()
// now: RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc() // rbx_core::SharedPtr
pub fn stub_257a10() -> ! {
    todo!("0x257a10 __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc()")]
// 0x257a50 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc()
// now: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc() // rbx_core::SharedPtr
pub fn stub_257a50() -> ! {
    todo!("0x257a50 __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EED1Ev")
}

#[doc(alias = "boost::shared_ptr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)")]
// 0x258688 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11HttpServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)
// now: rbx_core::SharedPtr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void) // rbx_core::SharedPtr
pub fn stub_258688() -> ! {
    todo!("0x258688 __ZN3RBX9CreatableINS_8InstanceEE6createINS_11HttpServiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "boost::shared_ptr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x258738 — __ZN5boost10shared_ptrIN3RBX11HttpServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)
// now: rbx_core::SharedPtr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter) // rbx_core::SharedPtr
pub fn stub_258738() -> ! {
    todo!("0x258738 __ZN5boost10shared_ptrIN3RBX11HttpServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpService,RBX::HttpService>(boost::shared_ptr<RBX::HttpService> const*,RBX::HttpService *)const")]
// 0x258800 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11HttpServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpService,RBX::HttpService>(boost::shared_ptr<RBX::HttpService> const*,RBX::HttpService *)const
// now: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpService,RBX::HttpService>(rbx_core::SharedPtr<RBX::HttpService> const*,RBX::HttpService *)const // rbx_core::SharedPtr
pub fn stub_258800() -> ! {
    todo!("0x258800 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11HttpServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x2588e8 — __ZN5boost6detail12shared_countC2IPN3RBX11HttpServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_2588e8() -> ! {
    todo!("0x2588e8 __ZN5boost6detail12shared_countC2IPN3RBX11HttpServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x2589f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_2589f0() -> ! {
    todo!("0x2589f0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x2589f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_2589f4() -> ! {
    todo!("0x2589f4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x2589f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_2589f8() -> ! {
    todo!("0x2589f8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x258a18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_258a18() -> ! {
    todo!("0x258a18 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x258a30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_258a30() -> ! {
    todo!("0x258a30 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::BoundFuncDesc(std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x2596a0 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EEC2EMS2_FSsSI_EPKcSO_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::BoundFuncDesc(std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// now: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::BoundFuncDesc(std::string (RBX::HttpService::*)(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) // rbx_core::SharedPtr
pub fn stub_2596a0() -> ! {
    todo!("0x2596a0 __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EEC2EMS2_FSsSI_EPKcSO_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x259838 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EE16declareSignatureEPKcS7_
// was: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant)
// now: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant) // rbx_core::SharedPtr
pub fn stub_259838() -> ! {
    todo!("0x259838 __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EE16declareSignatureEPKcS7_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc()")]
// 0x259868 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc()
// now: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::~BoundFuncDesc() // rbx_core::SharedPtr
pub fn stub_259868() -> ! {
    todo!("0x259868 __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x259984 — __ZNK3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// now: RBX::Reflection::BoundFuncDesc<RBX::HttpService,std::string ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const // rbx_core::SharedPtr
pub fn stub_259984() -> ! {
    todo!("0x259984 __ZNK3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::HttpService,std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,std::string>::call(RBX::HttpService*,std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),RBX::Reflection::Variant&,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")]
// 0x259a6c — __ZN3RBX10Reflection11Call1HelperINS_11HttpServiceEMS2_FSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEESI_SsE4callEPS2_SK_RS7_RKSI_
// was: RBX::Reflection::Call1Helper<RBX::HttpService,std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,std::string>::call(RBX::HttpService*,std::string (RBX::HttpService::*)(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),RBX::Reflection::Variant&,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)
// now: RBX::Reflection::Call1Helper<RBX::HttpService,std::string (RBX::HttpService::*)(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,std::string>::call(RBX::HttpService*,std::string (RBX::HttpService::*)(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),RBX::Reflection::Variant&,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&) // rbx_core::SharedPtr
pub fn stub_259a6c() -> ! {
    todo!("0x259a6c __ZN3RBX10Reflection11Call1HelperINS_11HttpServiceEMS2_FSsN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEESI_SsE4callEPS2_SK_RS7_RKSI_")
}

#[doc(alias = "boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::ArgHelper::getArg<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&,boost::disable_if<boost::is_same<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x259bfc — __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISJ_EEPNS3_10disable_ifINS3_7is_sameISJ_NS4_IKNS0_5TupleEEEEEvE4typeE
// was: boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::ArgHelper::getArg<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&,boost::disable_if<boost::is_same<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
// now: rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *) // rbx_core::SharedPtr
pub fn stub_259bfc() -> ! {
    todo!("0x259bfc __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISJ_EEPNS3_10disable_ifINS3_7is_sameISJ_NS4_IKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x259dbc — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EEC2EMS2_FSI_SsEPKcSO_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// now: RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) // rbx_core::SharedPtr
pub fn stub_259dbc() -> ! {
    todo!("0x259dbc __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EEC2EMS2_FSI_SsEPKcSO_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x259f34 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EE16declareSignatureEPKcS7_
// was: RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)
// now: RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant) // rbx_core::SharedPtr
pub fn stub_259f34() -> ! {
    todo!("0x259f34 __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EE16declareSignatureEPKcS7_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc()")]
// 0x259f64 — __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc()
// now: RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::~BoundFuncDesc() // rbx_core::SharedPtr
pub fn stub_259f64() -> ! {
    todo!("0x259f64 __ZN3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x25a030 — __ZNK3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// now: RBX::Reflection::BoundFuncDesc<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const // rbx_core::SharedPtr
pub fn stub_25a030() -> ! {
    todo!("0x25a030 __ZNK3RBX10Reflection13BoundFuncDescINS_11HttpServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::HttpService*,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),RBX::Reflection::Variant&,std::string const&)")]
// 0x25a170 — __ZN3RBX10Reflection11Call1HelperINS_11HttpServiceEMS2_FN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsESsSI_E4callEPS2_SK_RS7_RSD_
// was: RBX::Reflection::Call1Helper<RBX::HttpService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::HttpService*,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),RBX::Reflection::Variant&,std::string const&)
// now: RBX::Reflection::Call1Helper<RBX::HttpService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),std::string,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::HttpService*,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::HttpService::*)(std::string),RBX::Reflection::Variant&,std::string const&) // rbx_core::SharedPtr
pub fn stub_25a170() -> ! {
    todo!("0x25a170 __ZN3RBX10Reflection11Call1HelperINS_11HttpServiceEMS2_FN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEESsESsSI_E4callEPS2_SK_RS7_RSD_")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,char const*,RBX::HttpService::HttpContentType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x25a2f0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EEC2EMS2_FvSsSsS3_N5boost8functionIFvSsEEES9_EPKcSD_SD_SD_S3_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,char const*,RBX::HttpService::HttpContentType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_25a2f0() -> ! {
    todo!("0x25a2f0 __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EEC2EMS2_FvSsSsS3_N5boost8functionIFvSsEEES9_EPKcSD_SD_SD_S3_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// 0x25a68c — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSC_IFvSsEEE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
pub fn stub_25a68c() -> ! {
    todo!("0x25a68c __ZNK3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSC_IFvSsEEE")
}

#[doc(alias = "RBX::HttpService::HttpContentType RBX::Reflection::ArgHelper::getArg<RBX::HttpService::HttpContentType,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::HttpService::HttpContentType> const&,boost::disable_if<boost::is_same<RBX::HttpService::HttpContentType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x25a958 — __ZN3RBX10Reflection9ArgHelper6getArgINS_11HttpService15HttpContentTypeELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::HttpService::HttpContentType RBX::Reflection::ArgHelper::getArg<RBX::HttpService::HttpContentType,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::HttpService::HttpContentType> const&,boost::disable_if<boost::is_same<RBX::HttpService::HttpContentType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
// now: RBX::HttpService::HttpContentType RBX::Reflection::ArgHelper::getArg<RBX::HttpService::HttpContentType,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::HttpService::HttpContentType> const&,boost::disable_if<boost::is_same<RBX::HttpService::HttpContentType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *) // rbx_core::SharedPtr
pub fn stub_25a958() -> ! {
    todo!("0x25a958 __ZN3RBX10Reflection9ArgHelper6getArgINS_11HttpService15HttpContentTypeELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::HttpService::HttpContentType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::HttpService::HttpContentType &,boost::enable_if<boost::is_enum<RBX::HttpService::HttpContentType>,void>::type *)")]
// 0x25aaec — __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_11HttpService15HttpContentTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// was: bool RBX::Reflection::ArgHelper::try_enum<3,RBX::HttpService::HttpContentType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::HttpService::HttpContentType &,boost::enable_if<boost::is_enum<RBX::HttpService::HttpContentType>,void>::type *)
pub fn stub_25aaec() -> ! {
    todo!("0x25aaec __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_11HttpService15HttpContentTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x25ab40 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EEC2EMS2_FvSsN5boost8functionIFvSsEEES8_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_25ab40() -> ! {
    todo!("0x25ab40 __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EEC2EMS2_FvSsN5boost8functionIFvSsEEES8_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// 0x25adb4 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
pub fn stub_25adb4() -> ! {
    todo!("0x25adb4 __ZNK3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE")
}

#[doc(alias = "boost::shared_ptr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)")]
// 0x25c4d0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SpotLightEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)
// now: rbx_core::SharedPtr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void) // rbx_core::SharedPtr
pub fn stub_25c4d0() -> ! {
    todo!("0x25c4d0 __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SpotLightEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "boost::shared_ptr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x25c580 — __ZN5boost10shared_ptrIN3RBX9SpotLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)
// now: rbx_core::SharedPtr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter) // rbx_core::SharedPtr
pub fn stub_25c580() -> ! {
    todo!("0x25c580 __ZN5boost10shared_ptrIN3RBX9SpotLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SpotLight,RBX::SpotLight>(boost::shared_ptr<RBX::SpotLight> const*,RBX::SpotLight *)const")]
// 0x25c648 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9SpotLightES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SpotLight,RBX::SpotLight>(boost::shared_ptr<RBX::SpotLight> const*,RBX::SpotLight *)const
// now: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SpotLight,RBX::SpotLight>(rbx_core::SharedPtr<RBX::SpotLight> const*,RBX::SpotLight *)const // rbx_core::SharedPtr
pub fn stub_25c648() -> ! {
    todo!("0x25c648 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9SpotLightES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x25c730 — __ZN5boost6detail12shared_countC2IPN3RBX9SpotLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_25c730() -> ! {
    todo!("0x25c730 __ZN5boost6detail12shared_countC2IPN3RBX9SpotLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x25c838 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_25c838() -> ! {
    todo!("0x25c838 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x25c83c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_25c83c() -> ! {
    todo!("0x25c83c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x25c840 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_25c840() -> ! {
    todo!("0x25c840 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x25c860 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_25c860() -> ! {
    todo!("0x25c860 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x25c878 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_25c878() -> ! {
    todo!("0x25c878 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::shared_ptr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void)")]
// 0x25ce80 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10PointLightEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void)
// now: rbx_core::SharedPtr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void) // rbx_core::SharedPtr
pub fn stub_25ce80() -> ! {
    todo!("0x25ce80 __ZN3RBX9CreatableINS_8InstanceEE6createINS_10PointLightEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "boost::shared_ptr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x25cf30 — __ZN5boost10shared_ptrIN3RBX10PointLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)
// now: rbx_core::SharedPtr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter) // rbx_core::SharedPtr
pub fn stub_25cf30() -> ! {
    todo!("0x25cf30 __ZN5boost10shared_ptrIN3RBX10PointLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PointLight,RBX::PointLight>(boost::shared_ptr<RBX::PointLight> const*,RBX::PointLight *)const")]
// 0x25cff8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10PointLightES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PointLight,RBX::PointLight>(boost::shared_ptr<RBX::PointLight> const*,RBX::PointLight *)const
// now: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PointLight,RBX::PointLight>(rbx_core::SharedPtr<RBX::PointLight> const*,RBX::PointLight *)const // rbx_core::SharedPtr
pub fn stub_25cff8() -> ! {
    todo!("0x25cff8 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10PointLightES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x25d0e0 — __ZN5boost6detail12shared_countC2IPN3RBX10PointLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_25d0e0() -> ! {
    todo!("0x25d0e0 __ZN5boost6detail12shared_countC2IPN3RBX10PointLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x25d1e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_25d1e8() -> ! {
    todo!("0x25d1e8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x25d1ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_25d1ec() -> ! {
    todo!("0x25d1ec __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x25d1f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_25d1f0() -> ! {
    todo!("0x25d1f0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x25d210 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_25d210() -> ! {
    todo!("0x25d210 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x25d228 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_25d228() -> ! {
    todo!("0x25d228 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x25fad0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)
pub fn stub_25fad0() -> ! {
    todo!("0x25fad0 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")
}

#[doc(alias = "void boost::throw_exception<boost::thread_resource_error>(boost::thread_resource_error const&)")]
// 0x25fc58 — __ZN5boost15throw_exceptionINS_21thread_resource_errorEEEvRKT_
// was: void boost::throw_exception<boost::thread_resource_error>(boost::thread_resource_error const&)
pub fn stub_25fc58() -> ! {
    todo!("0x25fc58 __ZN5boost15throw_exceptionINS_21thread_resource_errorEEEvRKT_")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
// 0x25fdc0 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev
// was: boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()
pub fn stub_25fdc0() -> ! {
    todo!("0x25fdc0 __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
// 0x25fdc8 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()
pub fn stub_25fdc8() -> ! {
    todo!("0x25fdc8 __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::copy_boost_exception(boost::exception *,boost::exception const*)")]
// 0x25fdd0 — __ZN5boost16exception_detail20copy_boost_exceptionEPNS_9exceptionEPKS1_
// was: boost::exception_detail::copy_boost_exception(boost::exception *,boost::exception const*)
pub fn stub_25fdd0() -> ! {
    todo!("0x25fdd0 __ZN5boost16exception_detail20copy_boost_exceptionEPNS_9exceptionEPKS1_")
}

#[doc(alias = "boost::singleton_pool<XmlAttribute,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x25ff10 — __ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// was: boost::singleton_pool<XmlAttribute,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_25ff10() -> ! {
    todo!("0x25ff10 __ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::~clone_impl()")]
// 0x25ff60 — __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::~clone_impl()
pub fn stub_25ff60() -> ! {
    todo!("0x25ff60 __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::~clone_impl()")]
// 0x25ff70 — __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EED0Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::~clone_impl()
pub fn stub_25ff70() -> ! {
    todo!("0x25ff70 __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::rethrow(void)const")]
// 0x25ff88 — __ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE7rethrowEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::rethrow(void)const
pub fn stub_25ff88() -> ! {
    todo!("0x25ff88 __ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE7rethrowEv")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::~clone_impl()")]
// 0x260098 — __ZThn20_N5boost16exception_detail10clone_implINS0_14bad_exception_EED0Ev
// was: `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::~clone_impl()
pub fn stub_260098() -> ! {
    todo!("0x260098 __ZThn20_N5boost16exception_detail10clone_implINS0_14bad_exception_EED0Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone(void)const")]
// 0x2600b0 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone(void)const
pub fn stub_2600b0() -> ! {
    todo!("0x2600b0 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count(boost::detail::shared_count const&)")]
// 0x2600c0 — __ZN5boost6detail12shared_countC1ERKS1_
// was: boost::detail::shared_count::shared_count(boost::detail::shared_count const&)
pub fn stub_2600c0() -> ! {
    todo!("0x2600c0 __ZN5boost6detail12shared_countC1ERKS1_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x260a40 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)
pub fn stub_260a40() -> ! {
    todo!("0x260a40 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x260bc8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)
pub fn stub_260bc8() -> ! {
    todo!("0x260bc8 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
// 0x260d48 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)
pub fn stub_260d48() -> ! {
    todo!("0x260d48 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::detail::sp_counted_base::release(void)")]
// 0x260d98 — __ZN5boost6detail15sp_counted_base7releaseEv
// was: boost::detail::sp_counted_base::release(void)
pub fn stub_260d98() -> ! {
    todo!("0x260d98 __ZN5boost6detail15sp_counted_base7releaseEv")
}

#[doc(alias = "boost::detail::sp_counted_base::destroy(void)")]
// 0x260e38 — __ZN5boost6detail15sp_counted_base7destroyEv
// was: boost::detail::sp_counted_base::destroy(void)
pub fn stub_260e38() -> ! {
    todo!("0x260e38 __ZN5boost6detail15sp_counted_base7destroyEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>::get_untyped_deleter(void)")]
// 0x260e48 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>::get_untyped_deleter(void)
pub fn stub_260e48() -> ! {
    todo!("0x260e48 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()")]
// 0x260e50 — __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()
pub fn stub_260e50() -> ! {
    todo!("0x260e50 __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EED1Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone(void)const")]
// 0x260e60 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv
// was: `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone(void)const
pub fn stub_260e60() -> ! {
    todo!("0x260e60 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> *)")]
// 0x260e70 — __ZN5boost6detail12shared_countC2INS_16exception_detail10clone_implINS3_10bad_alloc_EEEEEPT_
// was: boost::detail::shared_count::shared_count<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> *)
pub fn stub_260e70() -> ! {
    todo!("0x260e70 __ZN5boost6detail12shared_countC2INS_16exception_detail10clone_implINS3_10bad_alloc_EEEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::~sp_counted_impl_p()")]
// 0x260f68 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEED0Ev
// was: boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::~sp_counted_impl_p()
pub fn stub_260f68() -> ! {
    todo!("0x260f68 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::get_untyped_deleter(void)")]
// 0x260f70 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::get_untyped_deleter(void)
pub fn stub_260f70() -> ! {
    todo!("0x260f70 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::exception_ptr boost::exception_detail::get_static_exception_object<boost::exception_detail::bad_alloc_>(void)")]
// 0x261df8 — __ZN5boost16exception_detail27get_static_exception_objectINS0_10bad_alloc_EEENS_13exception_ptrEv
// was: boost::exception_ptr boost::exception_detail::get_static_exception_object<boost::exception_detail::bad_alloc_>(void)
pub fn stub_261df8() -> ! {
    todo!("0x261df8 __ZN5boost16exception_detail27get_static_exception_objectINS0_10bad_alloc_EEENS_13exception_ptrEv")
}

#[doc(alias = "boost::exception_ptr boost::exception_detail::get_static_exception_object<boost::exception_detail::bad_exception_>(void)")]
// 0x2620f0 — __ZN5boost16exception_detail27get_static_exception_objectINS0_14bad_exception_EEENS_13exception_ptrEv
// was: boost::exception_ptr boost::exception_detail::get_static_exception_object<boost::exception_detail::bad_exception_>(void)
pub fn stub_2620f0() -> ! {
    todo!("0x2620f0 __ZN5boost16exception_detail27get_static_exception_objectINS0_14bad_exception_EEENS_13exception_ptrEv")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x262ab0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)
pub fn stub_262ab0() -> ! {
    todo!("0x262ab0 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
// 0x262c34 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)
pub fn stub_262c34() -> ! {
    todo!("0x262c34 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
// 0x262c88 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)
pub fn stub_262c88() -> ! {
    todo!("0x262c88 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
// 0x262db0 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const
pub fn stub_262db0() -> ! {
    todo!("0x262db0 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
// 0x262e40 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)
pub fn stub_262e40() -> ! {
    todo!("0x262e40 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x262e6c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)
pub fn stub_262e6c() -> ! {
    todo!("0x262e6c __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>>>::construct(void)")]
// 0x262ec4 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEEEEE9constructEv
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>>>::construct(void)
pub fn stub_262ec4() -> ! {
    todo!("0x262ec4 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
// 0x262efc — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
// was: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const
pub fn stub_262efc() -> ! {
    todo!("0x262efc __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> const&)")]
// 0x26309c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> const&)
pub fn stub_26309c() -> ! {
    todo!("0x26309c __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
// 0x263160 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)
pub fn stub_263160() -> ! {
    todo!("0x263160 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
// 0x2632ac — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)
pub fn stub_2632ac() -> ! {
    todo!("0x2632ac __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
// 0x263300 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)
pub fn stub_263300() -> ! {
    todo!("0x263300 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
// 0x263428 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const
pub fn stub_263428() -> ! {
    todo!("0x263428 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
// 0x2634b8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)
pub fn stub_2634b8() -> ! {
    todo!("0x2634b8 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x2634e4 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)
pub fn stub_2634e4() -> ! {
    todo!("0x2634e4 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>>>::construct(void)")]
// 0x26353c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEEEEE9constructEv
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>>>::construct(void)
pub fn stub_26353c() -> ! {
    todo!("0x26353c __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
// 0x263574 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
// was: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const
pub fn stub_263574() -> ! {
    todo!("0x263574 __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>> const&)")]
// 0x2636dc — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>> const&)
pub fn stub_2636dc() -> ! {
    todo!("0x2636dc __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
// 0x2637a0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)
pub fn stub_2637a0() -> ! {
    todo!("0x2637a0 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
// 0x2638ec — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)
pub fn stub_2638ec() -> ! {
    todo!("0x2638ec __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
// 0x263940 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)
pub fn stub_263940() -> ! {
    todo!("0x263940 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
// 0x263a68 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const
pub fn stub_263a68() -> ! {
    todo!("0x263a68 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
// 0x263af8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)
pub fn stub_263af8() -> ! {
    todo!("0x263af8 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x263b24 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)
pub fn stub_263b24() -> ! {
    todo!("0x263b24 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>>>::construct(void)")]
// 0x263b7c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEEEEE9constructEv
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>>>::construct(void)
pub fn stub_263b7c() -> ! {
    todo!("0x263b7c __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
// 0x263bb4 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
// was: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const
pub fn stub_263bb4() -> ! {
    todo!("0x263bb4 __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>> const&)")]
// 0x263d1c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>> const&)
pub fn stub_263d1c() -> ! {
    todo!("0x263d1c __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
// 0x263df0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)
pub fn stub_263df0() -> ! {
    todo!("0x263df0 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
// 0x264158 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)
pub fn stub_264158() -> ! {
    todo!("0x264158 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
// 0x264280 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const
pub fn stub_264280() -> ! {
    todo!("0x264280 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
// 0x264310 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)
pub fn stub_264310() -> ! {
    todo!("0x264310 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x26433c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)
pub fn stub_26433c() -> ! {
    todo!("0x26433c __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>>>::construct(void)")]
// 0x264394 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEEEEE9constructEv
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>>>::construct(void)
pub fn stub_264394() -> ! {
    todo!("0x264394 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
// 0x2643cc — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
// was: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const
pub fn stub_2643cc() -> ! {
    todo!("0x2643cc __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>> const&)")]
// 0x264534 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>> const&)
pub fn stub_264534() -> ! {
    todo!("0x264534 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
// 0x264608 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)
pub fn stub_264608() -> ! {
    todo!("0x264608 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
// 0x264aec — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)
pub fn stub_264aec() -> ! {
    todo!("0x264aec __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
// 0x264c70 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)
pub fn stub_264c70() -> ! {
    todo!("0x264c70 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
// 0x264d98 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const
pub fn stub_264d98() -> ! {
    todo!("0x264d98 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
// 0x264e28 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)
pub fn stub_264e28() -> ! {
    todo!("0x264e28 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x264e54 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)
pub fn stub_264e54() -> ! {
    todo!("0x264e54 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>>>::construct(void)")]
// 0x264eac — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEEEEE9constructEv
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>>>::construct(void)
pub fn stub_264eac() -> ! {
    todo!("0x264eac __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
// 0x264ee4 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
// was: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const
pub fn stub_264ee4() -> ! {
    todo!("0x264ee4 __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>> const&)")]
// 0x26504c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>> const&)
pub fn stub_26504c() -> ! {
    todo!("0x26504c __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE")
}

#[doc(alias = "boost::thread_resource_error::~thread_resource_error()")]
// 0x2650b8 — __ZN5boost21thread_resource_errorD0Ev
// was: boost::thread_resource_error::~thread_resource_error()
pub fn stub_2650b8() -> ! {
    todo!("0x2650b8 __ZN5boost21thread_resource_errorD0Ev")
}