//! core bg19 — 100 core stubs EA-sorted asc distinct not in /tmp/global_eas.txt.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua, exclude boost) global distinct — next 100 uncovered 0xf3ec14..0xf41254.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr not emitted (boost funcs excluded); single quotes, backticks, double quotes removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::_Vector_base<RBX::Verb *,std::allocator<RBX::Verb *>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX4VerbESaIS2_EE11_M_allocateEm")]
// 0xf3ec14 — j___ZNSt12_Vector_baseIPN3RBX4VerbESaIS2_EE11_M_allocateEm
// type: 
pub fn stub_0xf3ec14() -> ! {
    todo!("0xf3ec14 j___ZNSt12_Vector_baseIPN3RBX4VerbESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Verb **,std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>>,RBX::Verb * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX4VerbESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf3ec24 — j___ZNSt6vectorIPN3RBX4VerbESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf3ec24() -> ! {
    todo!("0xf3ec24 j___ZNSt6vectorIPN3RBX4VerbESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>::push_back(RBX::Verb * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX4VerbESaIS2_EE9push_backERKS2_")]
// 0xf3ec34 — j___ZNSt6vectorIPN3RBX4VerbESaIS2_EE9push_backERKS2_
// type: 
pub fn stub_0xf3ec34() -> ! {
    todo!("0xf3ec34 j___ZNSt6vectorIPN3RBX4VerbESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "RBX::MergeBinder::resolveRefs(void)")]
#[doc(alias = "j___ZN3RBX11MergeBinder11resolveRefsEv")]
// 0xf3f084 — j___ZN3RBX11MergeBinder11resolveRefsEv
// type: _DWORD __fastcall(RBX::MergeBinder *__hidden this)
pub fn stub_0xf3f084() -> ! {
    todo!("0xf3f084 j___ZN3RBX11MergeBinder11resolveRefsEv")
}

#[doc(alias = "std::_Vector_base<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX11MergeBinder9IDREFItemESaIS2_EE11_M_allocateEm")]
// 0xf3f294 — j___ZNSt12_Vector_baseIN3RBX11MergeBinder9IDREFItemESaIS2_EE11_M_allocateEm
// type: 
pub fn stub_0xf3f294() -> ! {
    todo!("0xf3f294 j___ZNSt12_Vector_baseIN3RBX11MergeBinder9IDREFItemESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::MergeBinder::IDREFItem * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::MergeBinder::IDREFItem *,RBX::MergeBinder::IDREFItem *>(RBX::MergeBinder::IDREFItem *,RBX::MergeBinder::IDREFItem *,RBX::MergeBinder::IDREFItem *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11MergeBinder9IDREFItemES6_EET0_T_S8_S7_")]
// 0xf3f2a4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11MergeBinder9IDREFItemES6_EET0_T_S8_S7_
// type: 
pub fn stub_0xf3f2a4() -> ! {
    todo!("0xf3f2a4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11MergeBinder9IDREFItemES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::MergeBinder::IDREFItem*,std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>>,RBX::MergeBinder::IDREFItem const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf3f2b4 — j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf3f2b4() -> ! {
    todo!("0xf3f2b4 j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::_M_erase_at_end(RBX::MergeBinder::IDREFItem*)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE15_M_erase_at_endEPS2_")]
// 0xf3f2c4 — j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE15_M_erase_at_endEPS2_
// type: 
pub fn stub_0xf3f2c4() -> ! {
    todo!("0xf3f2c4 j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE15_M_erase_at_endEPS2_")
}

#[doc(alias = "std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::push_back(RBX::MergeBinder::IDREFItem const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE9push_backERKS2_")]
// 0xf3f2d4 — j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE9push_backERKS2_
// type: 
pub fn stub_0xf3f2d4() -> ! {
    todo!("0xf3f2d4 j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::~vector()")]
#[doc(alias = "j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EED2Ev")]
// 0xf3f2e4 — j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EED2Ev
// type: int __fastcall(_DWORD)
pub fn stub_0xf3f2e4() -> ! {
    todo!("0xf3f2e4 j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EED2Ev")
}

#[doc(alias = "RBX::TopMenuBar::TopMenuBar(void)")]
#[doc(alias = "j___ZN3RBX10TopMenuBarC2Ev")]
// 0xf3f384 — j___ZN3RBX10TopMenuBarC2Ev
// type: _DWORD __fastcall(RBX::TopMenuBar *__hidden this)
pub fn stub_0xf3f384() -> ! {
    todo!("0xf3f384 j___ZN3RBX10TopMenuBarC2Ev")
}

#[doc(alias = "RBX::RelativePanel::RelativePanel(RBX::Layout const&)")]
#[doc(alias = "j___ZN3RBX13RelativePanelC2ERKNS_6LayoutE")]
// 0xf3f3b4 — j___ZN3RBX13RelativePanelC2ERKNS_6LayoutE
// type: 
pub fn stub_0xf3f3b4() -> ! {
    todo!("0xf3f3b4 j___ZN3RBX13RelativePanelC2ERKNS_6LayoutE")
}

#[doc(alias = "RBX::UnifiedWidget::UnifiedWidget(void)")]
#[doc(alias = "j___ZN3RBX13UnifiedWidgetC2Ev")]
// 0xf3f3c4 — j___ZN3RBX13UnifiedWidgetC2Ev
// type: _DWORD __fastcall(RBX::UnifiedWidget *__hidden this)
pub fn stub_0xf3f3c4() -> ! {
    todo!("0xf3f3c4 j___ZN3RBX13UnifiedWidgetC2Ev")
}

#[doc(alias = "RBX::UnifiedImageWidget::UnifiedImageWidget(RBX::Adorn *,std::string const&,int)")]
#[doc(alias = "j___ZN3RBX18UnifiedImageWidgetC2EPNS_5AdornERKSsi")]
// 0xf3f414 — j___ZN3RBX18UnifiedImageWidgetC2EPNS_5AdornERKSsi
// type: _DWORD __fastcall(RBX::UnifiedImageWidget *__hidden this, RBX::Adorn *, const std::string *, int)
pub fn stub_0xf3f414() -> ! {
    todo!("0xf3f414 j___ZN3RBX18UnifiedImageWidgetC2EPNS_5AdornERKSsi")
}

#[doc(alias = "rbx::remote_signal<void ()(RBX::UDim2)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev")]
// 0xf3fcc4 — j___ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf3fcc4() -> ! {
    todo!("0xf3fcc4 j___ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(int,int)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFviiEEC2Ev")]
// 0xf3fcd4 — j___ZN3rbx13remote_signalIFviiEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf3fcd4() -> ! {
    todo!("0xf3fcd4 j___ZN3rbx13remote_signalIFviiEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(void)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvvEEC2Ev")]
// 0xf3fce4 — j___ZN3rbx13remote_signalIFvvEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf3fce4() -> ! {
    todo!("0xf3fce4 j___ZN3rbx13remote_signalIFvvEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(void)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvvEED2Ev")]
// 0xf3fcf4 — j___ZN3rbx13remote_signalIFvvEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
pub fn stub_0xf3fcf4() -> ! {
    todo!("0xf3fcf4 j___ZN3rbx13remote_signalIFvvEED2Ev")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::UDim2)>::operator()(RBX::UDim2)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX5UDim2EEEclES3_")]
// 0xf3fd04 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX5UDim2EEEclES3_
// type: 
pub fn stub_0xf3fd04() -> ! {
    todo!("0xf3fd04 j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX5UDim2EEEclES3_")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(int,int)>::operator()(int,int)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi2EFviiEEclEii")]
// 0xf3fd14 — j___ZN3rbx7signals16signal_with_argsILi2EFviiEEclEii
// type: 
pub fn stub_0xf3fd14() -> ! {
    todo!("0xf3fd14 j___ZN3rbx7signals16signal_with_argsILi2EFviiEEclEii")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot24safe_static_do_get_mutexEv")]
// 0xf3fd34 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot24safe_static_do_get_mutexEv
// type: int(void)
pub fn stub_0xf3fd34() -> ! {
    todo!("0xf3fd34 j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::insert(rbx::signals::signal<void ()(RBX::UDim2)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6insertEPNS5_4slotE")]
// 0xf3fd44 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf3fd44() -> ! {
    todo!("0xf3fd44 j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6insertEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::remove(rbx::signals::signal<void ()(RBX::UDim2)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6removeEPNS5_4slotE")]
// 0xf3fd54 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf3fd54() -> ! {
    todo!("0xf3fd54 j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6removeEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE8on_errorERSt9exception")]
// 0xf3fd84 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE8on_errorERSt9exception
// type: 
pub fn stub_0xf3fd84() -> ! {
    todo!("0xf3fd84 j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFviiEE4slot24safe_static_do_get_mutexEv")]
// 0xf3fda4 — j___ZN3rbx7signals6signalIFviiEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf3fda4() -> ! {
    todo!("0xf3fda4 j___ZN3rbx7signals6signalIFviiEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::insert(rbx::signals::signal<void ()(int,int)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFviiEE6insertEPNS3_4slotE")]
// 0xf3fdb4 — j___ZN3rbx7signals6signalIFviiEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf3fdb4() -> ! {
    todo!("0xf3fdb4 j___ZN3rbx7signals6signalIFviiEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::remove(rbx::signals::signal<void ()(int,int)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFviiEE6removeEPNS3_4slotE")]
// 0xf3fdc4 — j___ZN3rbx7signals6signalIFviiEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf3fdc4() -> ! {
    todo!("0xf3fdc4 j___ZN3rbx7signals6signalIFviiEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFviiEE8on_errorERSt9exception")]
// 0xf3fe04 — j___ZN3rbx7signals6signalIFviiEE8on_errorERSt9exception
// type: 
pub fn stub_0xf3fe04() -> ! {
    todo!("0xf3fe04 j___ZN3rbx7signals6signalIFviiEE8on_errorERSt9exception")
}

#[doc(alias = "RBX::UDim2 const& rbx::any_cast<RBX::UDim2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf3fe84 — j___ZN3rbx8any_castIRKN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0xf3fe84() -> ! {
    todo!("0xf3fe84 j___ZN3rbx8any_castIRKN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::push_back(RBX::GuiObject::TweenStatus const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_")]
// 0xf40554 — j___ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_
// type: 
pub fn stub_0xf40554() -> ! {
    todo!("0xf40554 j___ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingStyle*,std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>>,RBX::GuiObject::TweenEasingStyle const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf40564 — j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0xf40564() -> ! {
    todo!("0xf40564 j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingStyle*,std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>>,unsigned long,RBX::GuiObject::TweenEasingStyle const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf40574 — j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: 
pub fn stub_0xf40574() -> ! {
    todo!("0xf40574 j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::resize(unsigned long,RBX::GuiObject::TweenEasingStyle)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE6resizeEmS2_")]
// 0xf40584 — j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE6resizeEmS2_
// type: 
pub fn stub_0xf40584() -> ! {
    todo!("0xf40584 j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::push_back(RBX::GuiObject::TweenEasingStyle const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE9push_backERKS2_")]
// 0xf40594 — j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE9push_backERKS2_
// type: 
pub fn stub_0xf40594() -> ! {
    todo!("0xf40594 j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingDirection*,std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>>,RBX::GuiObject::TweenEasingDirection const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf405a4 — j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0xf405a4() -> ! {
    todo!("0xf405a4 j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingDirection*,std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>>,unsigned long,RBX::GuiObject::TweenEasingDirection const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf405b4 — j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: 
pub fn stub_0xf405b4() -> ! {
    todo!("0xf405b4 j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::resize(unsigned long,RBX::GuiObject::TweenEasingDirection)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE6resizeEmS2_")]
// 0xf405c4 — j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE6resizeEmS2_
// type: 
pub fn stub_0xf405c4() -> ! {
    todo!("0xf405c4 j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::push_back(RBX::GuiObject::TweenEasingDirection const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE9push_backERKS2_")]
// 0xf405d4 — j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE9push_backERKS2_
// type: 
pub fn stub_0xf405d4() -> ! {
    todo!("0xf405d4 j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiButton::Style> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0xf405e4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: 
pub fn stub_0xf405e4() -> ! {
    todo!("0xf405e4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::pair<RBX::Name const* const,RBX::GuiButton::Style> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0xf405f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf405f4() -> ! {
    todo!("0xf405f4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiButton::Style> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0xf40604 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: 
pub fn stub_0xf40604() -> ! {
    todo!("0xf40604 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0xf40614 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: 
pub fn stub_0xf40614() -> ! {
    todo!("0xf40614 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0xf40624 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf40624() -> ! {
    todo!("0xf40624 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0xf40634 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: 
pub fn stub_0xf40634() -> ! {
    todo!("0xf40634 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0xf40644 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: 
pub fn stub_0xf40644() -> ! {
    todo!("0xf40644 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0xf40654 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf40654() -> ! {
    todo!("0xf40654 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0xf40664 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: 
pub fn stub_0xf40664() -> ! {
    todo!("0xf40664 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0xf40674 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: 
pub fn stub_0xf40674() -> ! {
    todo!("0xf40674 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0xf40684 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf40684() -> ! {
    todo!("0xf40684 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0xf40694 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: 
pub fn stub_0xf40694() -> ! {
    todo!("0xf40694 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "RBX::GuiService::~GuiService()")]
#[doc(alias = "j___ZN3RBX10GuiServiceD2Ev")]
// 0xf406a4 — j___ZN3RBX10GuiServiceD2Ev
// type: void __fastcall(RBX::GuiService *__hidden this)
pub fn stub_0xf406a4() -> ! {
    todo!("0xf406a4 j___ZN3RBX10GuiServiceD2Ev")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiService::SpecialKey>(RBX::GuiService::SpecialKey const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService10SpecialKeyEEERS3_RKT_")]
// 0xf409f4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService10SpecialKeyEEERS3_RKT_
// type: 
pub fn stub_0xf409f4() -> ! {
    todo!("0xf409f4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService10SpecialKeyEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiService::CenterDialogType>(RBX::GuiService::CenterDialogType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService16CenterDialogTypeEEERS3_RKT_")]
// 0xf40a04 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService16CenterDialogTypeEEERS3_RKT_
// type: 
pub fn stub_0xf40a04() -> ! {
    todo!("0xf40a04 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService16CenterDialogTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::SpecialKey>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE9singletonEv")]
// 0xf40a14 — j___ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE9singletonEv
// type: int(void)
pub fn stub_0xf40a14() -> ! {
    todo!("0xf40a14 j___ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::CenterDialogType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE9singletonEv")]
// 0xf40a24 — j___ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE9singletonEv
// type: 
pub fn stub_0xf40a24() -> ! {
    todo!("0xf40a24 j___ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE9singletonEv")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::GuiService::SpecialKey,std::string)>::operator()(RBX::GuiService::SpecialKey,std::string)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX10GuiService10SpecialKeyESsEEclES4_Ss")]
// 0xf40a34 — j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX10GuiService10SpecialKeyESsEEclES4_Ss
// type: 
pub fn stub_0xf40a34() -> ! {
    todo!("0xf40a34 j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX10GuiService10SpecialKeyESsEEclES4_Ss")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,std::string)>::fireItem(rbx::signals::signal<void ()(std::string,std::string)>::slot *,std::string,std::string)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi2EFvSsSsEE8fireItemEPNS0_6signalIS2_E4slotESsSs")]
// 0xf40a44 — j___ZN3rbx7signals16signal_with_argsILi2EFvSsSsEE8fireItemEPNS0_6signalIS2_E4slotESsSs
// type: 
pub fn stub_0xf40a44() -> ! {
    todo!("0xf40a44 j___ZN3rbx7signals16signal_with_argsILi2EFvSsSsEE8fireItemEPNS0_6signalIS2_E4slotESsSs")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,std::string)>::operator()(std::string,std::string)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi2EFvSsSsEEclESsSs")]
// 0xf40a54 — j___ZN3rbx7signals16signal_with_argsILi2EFvSsSsEEclESsSs
// type: 
pub fn stub_0xf40a54() -> ! {
    todo!("0xf40a54 j___ZN3rbx7signals16signal_with_argsILi2EFvSsSsEEclESsSs")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13disconnectAllEv")]
// 0xf40a64 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf40a64() -> ! {
    todo!("0xf40a64 j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE24safe_static_do_get_mutexEv")]
// 0xf40a74 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE24safe_static_do_get_mutexEv
// type: int(void)
pub fn stub_0xf40a74() -> ! {
    todo!("0xf40a74 j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot24safe_static_do_get_mutexEv")]
// 0xf40a94 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot24safe_static_do_get_mutexEv
// type: int(void)
pub fn stub_0xf40a94() -> ! {
    todo!("0xf40a94 j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::insert(rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6insertEPNS6_4slotE")]
// 0xf40aa4 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf40aa4() -> ! {
    todo!("0xf40aa4 j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::remove(rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6removeEPNS6_4slotE")]
// 0xf40ab4 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf40ab4() -> ! {
    todo!("0xf40ab4 j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE8on_errorERSt9exception")]
// 0xf40ad4 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE8on_errorERSt9exception
// type: 
pub fn stub_0xf40ad4() -> ! {
    todo!("0xf40ad4 j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsSsEE13disconnectAllEv")]
// 0xf40ae4 — j___ZN3rbx7signals6signalIFvSsSsEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf40ae4() -> ! {
    todo!("0xf40ae4 j___ZN3rbx7signals6signalIFvSsSsEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsSsEE24safe_static_do_get_mutexEv")]
// 0xf40af4 — j___ZN3rbx7signals6signalIFvSsSsEE24safe_static_do_get_mutexEv
// type: int(void)
pub fn stub_0xf40af4() -> ! {
    todo!("0xf40af4 j___ZN3rbx7signals6signalIFvSsSsEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsSsEE4slot24safe_static_do_get_mutexEv")]
// 0xf40b14 — j___ZN3rbx7signals6signalIFvSsSsEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf40b14() -> ! {
    todo!("0xf40b14 j___ZN3rbx7signals6signalIFvSsSsEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::insert(rbx::signals::signal<void ()(std::string,std::string)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsSsEE6insertEPNS3_4slotE")]
// 0xf40b24 — j___ZN3rbx7signals6signalIFvSsSsEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf40b24() -> ! {
    todo!("0xf40b24 j___ZN3rbx7signals6signalIFvSsSsEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::remove(rbx::signals::signal<void ()(std::string,std::string)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsSsEE6removeEPNS3_4slotE")]
// 0xf40b34 — j___ZN3rbx7signals6signalIFvSsSsEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf40b34() -> ! {
    todo!("0xf40b34 j___ZN3rbx7signals6signalIFvSsSsEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsSsEE8on_errorERSt9exception")]
// 0xf40b54 — j___ZN3rbx7signals6signalIFvSsSsEE8on_errorERSt9exception
// type: 
pub fn stub_0xf40b54() -> ! {
    todo!("0xf40b54 j___ZN3rbx7signals6signalIFvSsSsEE8on_errorERSt9exception")
}

#[doc(alias = "RBX::GuiService::SpecialKey * rbx::any_cast<RBX::GuiService::SpecialKey,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX10GuiService10SpecialKeyENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf40b64 — j___ZN3rbx8any_castIN3RBX10GuiService10SpecialKeyENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf40b64() -> ! {
    todo!("0xf40b64 j___ZN3rbx8any_castIN3RBX10GuiService10SpecialKeyENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::GuiService::CenterDialogType * rbx::any_cast<RBX::GuiService::CenterDialogType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf40b74 — j___ZN3rbx8any_castIN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf40b74() -> ! {
    todo!("0xf40b74 j___ZN3rbx8any_castIN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::GuiService::SpecialKey const& rbx::any_cast<RBX::GuiService::SpecialKey const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf40b84 — j___ZN3rbx8any_castIRKN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf40b84() -> ! {
    todo!("0xf40b84 j___ZN3rbx8any_castIRKN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::GuiService::CenterDialogType const& rbx::any_cast<RBX::GuiService::CenterDialogType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf40b94 — j___ZN3rbx8any_castIRKN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf40b94() -> ! {
    todo!("0xf40b94 j___ZN3rbx8any_castIRKN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::GuiService::SpecialKey & rbx::any_cast<RBX::GuiService::SpecialKey &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf40ba4 — j___ZN3rbx8any_castIRN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf40ba4() -> ! {
    todo!("0xf40ba4 j___ZN3rbx8any_castIRN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::GuiService::CenterDialogType & rbx::any_cast<RBX::GuiService::CenterDialogType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf40bb4 — j___ZN3rbx8any_castIRN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf40bb4() -> ! {
    todo!("0xf40bb4 j___ZN3rbx8any_castIRN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Vector_base<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX10GuiService10SpecialKeyESaIS2_EE11_M_allocateEm")]
// 0xf410d4 — j___ZNSt12_Vector_baseIN3RBX10GuiService10SpecialKeyESaIS2_EE11_M_allocateEm
// type: 
pub fn stub_0xf410d4() -> ! {
    todo!("0xf410d4 j___ZNSt12_Vector_baseIN3RBX10GuiService10SpecialKeyESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX10GuiService16CenterDialogTypeESaIS2_EE11_M_allocateEm")]
// 0xf410e4 — j___ZNSt12_Vector_baseIN3RBX10GuiService16CenterDialogTypeESaIS2_EE11_M_allocateEm
// type: 
pub fn stub_0xf410e4() -> ! {
    todo!("0xf410e4 j___ZNSt12_Vector_baseIN3RBX10GuiService16CenterDialogTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::GuiService::SpecialKey * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *>(RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService10SpecialKeyES6_EET0_T_S8_S7_")]
// 0xf410f4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService10SpecialKeyES6_EET0_T_S8_S7_
// type: 
pub fn stub_0xf410f4() -> ! {
    todo!("0xf410f4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService10SpecialKeyES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::GuiService::CenterDialogType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiService::CenterDialogType *,RBX::GuiService::CenterDialogType *>(RBX::GuiService::CenterDialogType *,RBX::GuiService::CenterDialogType *,RBX::GuiService::CenterDialogType *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService16CenterDialogTypeES6_EET0_T_S8_S7_")]
// 0xf41104 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService16CenterDialogTypeES6_EET0_T_S8_S7_
// type: 
pub fn stub_0xf41104() -> ! {
    todo!("0xf41104 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService16CenterDialogTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::GuiService::CenterDialogType,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::operator[](RBX::GuiService::CenterDialogType const&)")]
#[doc(alias = "j___ZNSt3mapIN3RBX10GuiService16CenterDialogTypeESt4listIPNS1_13DialogWrapperESaIS5_EESt4lessIS2_ESaISt4pairIKS2_S7_EEEixERSB_")]
// 0xf41114 — j___ZNSt3mapIN3RBX10GuiService16CenterDialogTypeESt4listIPNS1_13DialogWrapperESaIS5_EESt4lessIS2_ESaISt4pairIKS2_S7_EEEixERSB_
// type: int __fastcall(int, int, int, int, void *, int, int, void *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xf41114() -> ! {
    todo!("0xf41114 j___ZNSt3mapIN3RBX10GuiService16CenterDialogTypeESt4listIPNS1_13DialogWrapperESaIS5_EESt4lessIS2_ESaISt4pairIKS2_S7_EEEixERSB_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GuiService::SpecialKey,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_10GuiService10SpecialKeyESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf41134 — j___ZNSt3mapIPKN3RBX4NameENS0_10GuiService10SpecialKeyESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: 
pub fn stub_0xf41134() -> ! {
    todo!("0xf41134 j___ZNSt3mapIPKN3RBX4NameENS0_10GuiService10SpecialKeyESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GuiService::CenterDialogType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_10GuiService16CenterDialogTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf41144 — j___ZNSt3mapIPKN3RBX4NameENS0_10GuiService16CenterDialogTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: 
pub fn stub_0xf41144() -> ! {
    todo!("0xf41144 j___ZNSt3mapIPKN3RBX4NameENS0_10GuiService16CenterDialogTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "void std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::_M_initialize_dispatch<std::_List_const_iterator<RBX::GuiService::DialogWrapper *>>(std::_List_const_iterator<RBX::GuiService::DialogWrapper *>,std::_List_const_iterator<RBX::GuiService::DialogWrapper *>,std::__false_type)")]
#[doc(alias = "j___ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE22_M_initialize_dispatchISt20_List_const_iteratorIS3_EEEvT_S9_St12__false_type")]
// 0xf41154 — j___ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE22_M_initialize_dispatchISt20_List_const_iteratorIS3_EEEvT_S9_St12__false_type
// type: 
pub fn stub_0xf41154() -> ! {
    todo!("0xf41154 j___ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE22_M_initialize_dispatchISt20_List_const_iteratorIS3_EEEvT_S9_St12__false_type")
}

#[doc(alias = "std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::remove(RBX::GuiService::DialogWrapper * const&)")]
#[doc(alias = "j___ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE6removeERKS3_")]
// 0xf41164 — j___ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE6removeERKS3_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf41164() -> ! {
    todo!("0xf41164 j___ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE6removeERKS3_")
}

#[doc(alias = "std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::list(std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>> const&)")]
#[doc(alias = "j___ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EEC2ERKS5_")]
// 0xf41174 — j___ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EEC2ERKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf41174() -> ! {
    todo!("0xf41174 j___ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EEC2ERKS5_")
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiService::SpecialKey*,std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>>,RBX::GuiService::SpecialKey const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf41184 — j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0xf41184() -> ! {
    todo!("0xf41184 j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiService::SpecialKey*,std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>>,unsigned long,RBX::GuiService::SpecialKey const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf41194 — j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: 
pub fn stub_0xf41194() -> ! {
    todo!("0xf41194 j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::resize(unsigned long,RBX::GuiService::SpecialKey)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE6resizeEmS2_")]
// 0xf411a4 — j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE6resizeEmS2_
// type: 
pub fn stub_0xf411a4() -> ! {
    todo!("0xf411a4 j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::push_back(RBX::GuiService::SpecialKey const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE9push_backERKS2_")]
// 0xf411b4 — j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE9push_backERKS2_
// type: 
pub fn stub_0xf411b4() -> ! {
    todo!("0xf411b4 j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiService::CenterDialogType*,std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>>,RBX::GuiService::CenterDialogType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf411c4 — j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0xf411c4() -> ! {
    todo!("0xf411c4 j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiService::CenterDialogType*,std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>>,unsigned long,RBX::GuiService::CenterDialogType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf411d4 — j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: 
pub fn stub_0xf411d4() -> ! {
    todo!("0xf411d4 j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::resize(unsigned long,RBX::GuiService::CenterDialogType)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE6resizeEmS2_")]
// 0xf411e4 — j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE6resizeEmS2_
// type: 
pub fn stub_0xf411e4() -> ! {
    todo!("0xf411e4 j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::push_back(RBX::GuiService::CenterDialogType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE9push_backERKS2_")]
// 0xf411f4 — j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE9push_backERKS2_
// type: 
pub fn stub_0xf411f4() -> ! {
    todo!("0xf411f4 j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert_unique(RBX::GuiService::SpecialKey const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")]
// 0xf41204 — j___ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
// type: 
pub fn stub_0xf41204() -> ! {
    todo!("0xf41204 j___ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_erase(std::_Rb_tree_node<RBX::GuiService::SpecialKey> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xf41214 — j___ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf41214() -> ! {
    todo!("0xf41214 j___ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::GuiService::SpecialKey const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// 0xf41224 — j___ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: 
pub fn stub_0xf41224() -> ! {
    todo!("0xf41224 j___ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_create_node(std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE14_M_create_nodeERKSA_")]
// 0xf41234 — j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE14_M_create_nodeERKSA_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf41234() -> ! {
    todo!("0xf41234 j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE14_M_create_nodeERKSA_")
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E")]
// 0xf41244 — j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E
// type: 
pub fn stub_0xf41244() -> ! {
    todo!("0xf41244 j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E")
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert_unique(std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueERKSA_")]
// 0xf41254 — j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueERKSA_
// type: int __fastcall(int, int, int)
pub fn stub_0xf41254() -> ! {
    todo!("0xf41254 j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueERKSA_")
}
