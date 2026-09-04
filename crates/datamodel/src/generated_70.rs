// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted, true uncovered after existing shards
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x50b39c..0x524a9c | total filtered 10215, remaining 1596->1496 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias
// Shard: 70 EA-sorted ascending next uncovered gap from 0x50b39c

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
// 0x50b39c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SelectionEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Selection> RBX::Creatable<RBX::Instance>::create<RBX::Selection>(void)")]
// was: boost::shared_ptr<RBX::Selection> RBX::Creatable<RBX::Instance>::create<RBX::Selection>(void)
pub use crate::instance::stub_0x50b39c as stub_50b39c;
// 0x50b44c — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9SelectionEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Selection>(rbx_core::SharedPtr<RBX::Selection> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::Selection>(boost::shared_ptr<RBX::Selection> const&)
pub use crate::instance::stub_0x50b44c as stub_50b44c;
// 0x50b688 — __ZN5boost10shared_ptrIN3RBX9SelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Selection>::shared_ptr<RBX::Selection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Selection>::shared_ptr<RBX::Selection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x50b688 as stub_50b688;
// 0x50b838 — __ZN5boost6detail12shared_countC2IPN3RBX9SelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x50b838 as stub_50b838;
// 0x50b940 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x50b940 as stub_50b940;
// 0x50b948 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x50b948 as stub_50b948;
// 0x50b968 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x50b968 as stub_50b968;
// 0x50b980 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x50b980 as stub_50b980;
// 0x50caa0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>> *)
pub use crate::instance::stub_0x50caa0 as stub_50caa0;
// 0x50cac8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>> *)
pub use crate::instance::stub_0x50cac8 as stub_50cac8;
// 0x50dd7c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19GlobalBasicSettingsEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalBasicSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalBasicSettings>(void)")]
// was: boost::shared_ptr<RBX::GlobalBasicSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalBasicSettings>(void)
pub use crate::instance::stub_0x50dd7c as stub_50dd7c;
// 0x50de2c — __ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalBasicSettings>::shared_ptr<RBX::GlobalBasicSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::GlobalBasicSettings>::shared_ptr<RBX::GlobalBasicSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x50de2c as stub_50de2c;
// 0x50dfdc — __ZN5boost6detail12shared_countC2IPN3RBX19GlobalBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x50dfdc as stub_50dfdc;
// 0x50e0e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x50e0e4 as stub_50e0e4;
// 0x50e0e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x50e0e8 as stub_50e0e8;
// 0x50e0ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x50e0ec as stub_50e0ec;
// 0x50e10c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x50e10c as stub_50e10c;
// 0x50e124 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x50e124 as stub_50e124;
// 0x50e12c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_22GlobalAdvancedSettingsEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalAdvancedSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalAdvancedSettings>(void)")]
// was: boost::shared_ptr<RBX::GlobalAdvancedSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalAdvancedSettings>(void)
pub use crate::instance::stub_0x50e12c as stub_50e12c;
// 0x50e1dc — __ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalAdvancedSettings>::shared_ptr<RBX::GlobalAdvancedSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::GlobalAdvancedSettings>::shared_ptr<RBX::GlobalAdvancedSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x50e1dc as stub_50e1dc;
// 0x50e38c — __ZN5boost6detail12shared_countC2IPN3RBX22GlobalAdvancedSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x50e38c as stub_50e38c;
// 0x50e494 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x50e494 as stub_50e494;
// 0x50e498 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x50e498 as stub_50e498;
// 0x50e49c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x50e49c as stub_50e49c;
// 0x50e4bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x50e4bc as stub_50e4bc;
// 0x50e4d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x50e4d4 as stub_50e4d4;
// 0x50e4d8 — __ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_iESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,int>,std::_Select1st<std::pair<RBX::InstanceHandle const,int>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::InstanceHandle const,int>> *)")]
pub use crate::instance::stub_0x50e4d8 as stub_50e4d8;
// 0x50e500 — __ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_iESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,int>,std::_Select1st<std::pair<RBX::InstanceHandle const,int>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,int>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::InstanceHandle const,int>> *)")]
pub use crate::instance::stub_0x50e500 as stub_50e500;
// 0x512308 — __ZN3RBX10GuiBuilder8buildGuiEPNS_5AdornEPNS_9WorkspaceEb
#[doc(alias = "RBX::GuiBuilder::buildGui(RBX::Adorn *,RBX::Workspace *,bool)")]
pub use crate::instance::stub_0x512308 as stub_512308;
// 0x51e768 — __ZN3RBX10GuiBuilder10InitializeEPNS_9DataModelE
#[doc(alias = "RBX::GuiBuilder::Initialize(RBX::DataModel *)")]
pub use crate::instance::stub_0x51e768 as stub_51e768;
// 0x520c28 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15EquationDisplayESsSsEEN5boost10shared_ptrIT_EET0_T1_
#[doc(alias = "rbx_core::SharedPtr<RBX::EquationDisplay> RBX::Creatable<RBX::Instance>::create<RBX::EquationDisplay,std::string,std::string>(std::string,std::string)")]
// was: boost::shared_ptr<RBX::EquationDisplay> RBX::Creatable<RBX::Instance>::create<RBX::EquationDisplay,std::string,std::string>(std::string,std::string)
pub use crate::instance::stub_0x520c28 as stub_520c28;
// 0x520ef4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5FrameEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Frame> RBX::Creatable<RBX::Instance>::create<RBX::Frame>(void)")]
// was: boost::shared_ptr<RBX::Frame> RBX::Creatable<RBX::Instance>::create<RBX::Frame>(void)
pub use crate::instance::stub_0x520ef4 as stub_520ef4;
// 0x520fa8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15NotificationBoxEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationBox> RBX::Creatable<RBX::Instance>::create<RBX::NotificationBox>(void)")]
// was: boost::shared_ptr<RBX::NotificationBox> RBX::Creatable<RBX::Instance>::create<RBX::NotificationBox>(void)
pub use crate::instance::stub_0x520fa8 as stub_520fa8;
// 0x52105c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14GuiImageButtonEPNS_4VerbEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiImageButton> RBX::Creatable<RBX::Instance>::create<RBX::GuiImageButton,RBX::Verb *>(RBX::Verb *)")]
// was: boost::shared_ptr<RBX::GuiImageButton> RBX::Creatable<RBX::Instance>::create<RBX::GuiImageButton,RBX::Verb *>(RBX::Verb *)
pub use crate::instance::stub_0x52105c as stub_52105c;
// 0x521138 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13RelativePanelENS_6LayoutEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RelativePanel> RBX::Creatable<RBX::Instance>::create<RBX::RelativePanel,RBX::Layout>(RBX::Layout)")]
// was: boost::shared_ptr<RBX::RelativePanel> RBX::Creatable<RBX::Instance>::create<RBX::RelativePanel,RBX::Layout>(RBX::Layout)
pub use crate::instance::stub_0x521138 as stub_521138;
// 0x5211ec — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatOutputEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatOutput> RBX::Creatable<RBX::Instance>::create<RBX::ChatOutput>(void)")]
// was: boost::shared_ptr<RBX::ChatOutput> RBX::Creatable<RBX::Instance>::create<RBX::ChatOutput>(void)
pub use crate::instance::stub_0x5211ec as stub_5211ec;
// 0x5212a0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatWidgetESsSsEEN5boost10shared_ptrIT_EET0_T1_
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatWidget> RBX::Creatable<RBX::Instance>::create<RBX::ChatWidget,std::string,std::string>(std::string,std::string)")]
// was: boost::shared_ptr<RBX::ChatWidget> RBX::Creatable<RBX::Instance>::create<RBX::ChatWidget,std::string,std::string>(std::string,std::string)
pub use crate::instance::stub_0x5212a0 as stub_5212a0;
// 0x5213fc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ChatButtonEPNS_5AdornEPKciEEN5boost10shared_ptrIT_EET0_T1_T2_
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatButton> RBX::Creatable<RBX::Instance>::create<RBX::ChatButton,RBX::Adorn *,char const*,int>(RBX::Adorn *,char const*,int)")]
// was: boost::shared_ptr<RBX::ChatButton> RBX::Creatable<RBX::Instance>::create<RBX::ChatButton,RBX::Adorn *,char const*,int>(RBX::Adorn *,char const*,int)
pub use crate::instance::stub_0x5213fc as stub_5213fc;
// 0x521594 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11TextDisplayEPKcS6_EEN5boost10shared_ptrIT_EET0_T1_
#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay> RBX::Creatable<RBX::Instance>::create<RBX::TextDisplay,char const*,char const*>(char const*,char const*)")]
// was: boost::shared_ptr<RBX::TextDisplay> RBX::Creatable<RBX::Instance>::create<RBX::TextDisplay,char const*,char const*>(char const*,char const*)
pub use crate::instance::stub_0x521594 as stub_521594;
// 0x52177c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15EquationDisplayEPKcS6_EEN5boost10shared_ptrIT_EET0_T1_
#[doc(alias = "rbx_core::SharedPtr<RBX::EquationDisplay> RBX::Creatable<RBX::Instance>::create<RBX::EquationDisplay,char const*,char const*>(char const*,char const*)")]
// was: boost::shared_ptr<RBX::EquationDisplay> RBX::Creatable<RBX::Instance>::create<RBX::EquationDisplay,char const*,char const*>(char const*,char const*)
pub use crate::instance::stub_0x52177c as stub_52177c;
// 0x521b38 — __ZN5boost10shared_ptrIN3RBX15EquationDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::EquationDisplay>::shared_ptr<RBX::EquationDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::EquationDisplay>::shared_ptr<RBX::EquationDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x521b38 as stub_521b38;
// 0x521ce8 — __ZN5boost6detail12shared_countC2IPN3RBX15EquationDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x521ce8 as stub_521ce8;
// 0x521df0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x521df0 as stub_521df0;
// 0x521df4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x521df4 as stub_521df4;
// 0x521df8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x521df8 as stub_521df8;
// 0x521e18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x521e18 as stub_521e18;
// 0x521e30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15EquationDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::EquationDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x521e30 as stub_521e30;
// 0x521e34 — __ZN5boost10shared_ptrIN3RBX11TextDisplayEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay>::shared_ptr<RBX::TextDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::TextDisplay>::shared_ptr<RBX::TextDisplay,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x521e34 as stub_521e34;
// 0x521fe4 — __ZN5boost6detail12shared_countC2IPN3RBX11TextDisplayENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x521fe4 as stub_521fe4;
// 0x5220ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5220ec as stub_5220ec;
// 0x5220f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5220f0 as stub_5220f0;
// 0x5220f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x5220f4 as stub_5220f4;
// 0x522114 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x522114 as stub_522114;
// 0x52212c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TextDisplayENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextDisplay *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x52212c as stub_52212c;
// 0x522ff0 — __ZN5boost10shared_ptrIN3RBX10ChatButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatButton>::shared_ptr<RBX::ChatButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ChatButton>::shared_ptr<RBX::ChatButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x522ff0 as stub_522ff0;
// 0x5231a0 — __ZN5boost6detail12shared_countC2IPN3RBX10ChatButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x5231a0 as stub_5231a0;
// 0x5232a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5232a8 as stub_5232a8;
// 0x5232ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5232ac as stub_5232ac;
// 0x5232b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x5232b0 as stub_5232b0;
// 0x5232d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x5232d0 as stub_5232d0;
// 0x5232e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x5232e8 as stub_5232e8;
// 0x5232ec — __ZN5boost10shared_ptrIN3RBX10ChatWidgetEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatWidget>::shared_ptr<RBX::ChatWidget,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ChatWidget>::shared_ptr<RBX::ChatWidget,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x5232ec as stub_5232ec;
// 0x52349c — __ZN5boost6detail12shared_countC2IPN3RBX10ChatWidgetENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x52349c as stub_52349c;
// 0x5235a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5235a4 as stub_5235a4;
// 0x5235a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5235a8 as stub_5235a8;
// 0x5235ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x5235ac as stub_5235ac;
// 0x5235cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x5235cc as stub_5235cc;
// 0x5235e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatWidgetENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatWidget *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x5235e4 as stub_5235e4;
// 0x5235e8 — __ZN5boost10shared_ptrIN3RBX10ChatOutputEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatOutput>::shared_ptr<RBX::ChatOutput,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ChatOutput>::shared_ptr<RBX::ChatOutput,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x5235e8 as stub_5235e8;
// 0x523798 — __ZN5boost6detail12shared_countC2IPN3RBX10ChatOutputENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x523798 as stub_523798;
// 0x5238a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5238a0 as stub_5238a0;
// 0x5238a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5238a4 as stub_5238a4;
// 0x5238a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x5238a8 as stub_5238a8;
// 0x5238c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x5238c8 as stub_5238c8;
// 0x5238e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x5238e0 as stub_5238e0;
// 0x523ccc — __ZN5boost10shared_ptrIN3RBX13RelativePanelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RelativePanel>::shared_ptr<RBX::RelativePanel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::RelativePanel>::shared_ptr<RBX::RelativePanel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x523ccc as stub_523ccc;
// 0x523e7c — __ZN5boost6detail12shared_countC2IPN3RBX13RelativePanelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x523e7c as stub_523e7c;
// 0x523f84 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x523f84 as stub_523f84;
// 0x523f88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x523f88 as stub_523f88;
// 0x523f8c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x523f8c as stub_523f8c;
// 0x523fac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x523fac as stub_523fac;
// 0x523fc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x523fc4 as stub_523fc4;
// 0x524378 — __ZN5boost6detail12shared_countC2IPN3RBX17GameBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x524378 as stub_524378;
// 0x524480 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x524480 as stub_524480;
// 0x524484 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x524484 as stub_524484;
// 0x5244a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x5244a4 as stub_5244a4;
// 0x5244bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x5244bc as stub_5244bc;
// 0x5244c0 — __ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiImageButton>::shared_ptr<RBX::GuiImageButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::GuiImageButton>::shared_ptr<RBX::GuiImageButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x5244c0 as stub_5244c0;
// 0x524670 — __ZN5boost6detail12shared_countC2IPN3RBX14GuiImageButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x524670 as stub_524670;
// 0x524778 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x524778 as stub_524778;
// 0x52477c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x52477c as stub_52477c;
// 0x524780 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x524780 as stub_524780;
// 0x5247a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x5247a0 as stub_5247a0;
// 0x5247b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x5247b8 as stub_5247b8;
// 0x5247bc — __ZN5boost10shared_ptrIN3RBX15NotificationBoxEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationBox>::shared_ptr<RBX::NotificationBox,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::NotificationBox>::shared_ptr<RBX::NotificationBox,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x5247bc as stub_5247bc;
// 0x52496c — __ZN5boost6detail12shared_countC2IPN3RBX15NotificationBoxENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x52496c as stub_52496c;
// 0x524a74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x524a74 as stub_524a74;
// 0x524a78 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x524a78 as stub_524a78;
// 0x524a7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x524a7c as stub_524a7c;
// 0x524a9c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x524a9c as stub_524a9c;
