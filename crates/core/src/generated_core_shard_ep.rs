//! core shard EP — 100 core stubs EA-sorted, lowest uncovered 0x9ef248..0xa524a4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EO 0x9ef248).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::remove(rbx::signals::signal<void ()(int,bool,int)>::slot *)")]
// 0x9ef248 — __ZN3rbx7signals6signalIFvibiEE6removeEPNS3_4slotE
pub fn stub_9ef248() -> ! {
    todo!("0x9ef248 __ZN3rbx7signals6signalIFvibiEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::slot::safe_static_init_mutex(void)")]
// 0x9ef334 — __ZN3rbx7signals6signalIFvibiEE4slot22safe_static_init_mutexEv
pub fn stub_9ef334() -> ! {
    todo!("0x9ef334 __ZN3rbx7signals6signalIFvibiEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,bool,int)>::slot,boost::function<void ()(int,bool,int)>,3,void ()(int,bool,int)>::~callable()")]
// 0x9ef41c — __ZN3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_ED2Ev
pub fn stub_9ef41c() -> ! {
    todo!("0x9ef41c __ZN3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_ED2Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,bool,int)>::slot,boost::function<void ()(int,bool,int)>,3,void ()(int,bool,int)>::~callable()")]
// 0x9ef5b4 — __ZN3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
pub fn stub_9ef5b4() -> ! {
    todo!("0x9ef5b4 __ZN3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,bool,int)>::slot,boost::function<void ()(int,bool,int)>,3,void ()(int,bool,int)>::~callable()")]
// 0x9ef5c0 — __ZN3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
pub fn stub_9ef5c0() -> ! {
    todo!("0x9ef5c0 __ZN3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::slot::~slot()")]
// 0x9ef674 — __ZN3rbx7signals6signalIFvibiEE4slotD1Ev
pub fn stub_9ef674() -> ! {
    todo!("0x9ef674 __ZN3rbx7signals6signalIFvibiEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::slot::~slot()")]
// 0x9ef6d0 — __ZN3rbx7signals6signalIFvibiEE4slotD0Ev
pub fn stub_9ef6d0() -> ! {
    todo!("0x9ef6d0 __ZN3rbx7signals6signalIFvibiEE4slotD0Ev")
}

#[doc(alias = "anonymous namespace::onCrispEventLogged(rbx_core::SharedPtr<RBX::CrispResponse> const&)")]
// 0xa18b90 — __ZN12_GLOBAL__N_118onCrispEventLoggedERKN5boost10shared_ptrIN3RBX13CrispResponseEEE
// was: anonymous namespace::onCrispEventLogged(boost::shared_ptr<RBX::CrispResponse> const&)
pub fn stub_a18b90() -> ! {
    todo!("0xa18b90 __ZN12_GLOBAL__N_118onCrispEventLoggedERKN5boost10shared_ptrIN3RBX13CrispResponseEEE")
}

#[doc(alias = "whoCaresResponse(std::string *,std::exception *)")]
// 0xa18bc0 — __ZL16whoCaresResponsePSsPSt9exception
pub fn stub_a18bc0() -> ! {
    todo!("0xa18bc0 __ZL16whoCaresResponsePSsPSt9exception")
}

#[doc(alias = "std::range_error::~range_error()")]
// 0xa1ad98 — __ZNSt11range_errorD1Ev
pub fn stub_a1ad98() -> ! {
    todo!("0xa1ad98 __ZNSt11range_errorD1Ev")
}

#[doc(alias = "std::overflow_error::~overflow_error()")]
// 0xa1ada8 — __ZNSt14overflow_errorD2Ev
pub fn stub_a1ada8() -> ! {
    todo!("0xa1ada8 __ZNSt14overflow_errorD2Ev")
}

#[doc(alias = "XmlElement::XmlElement(RBX::Name const&)")]
// 0xa1c5a0 — __ZN10XmlElementC1ERKN3RBX4NameE
pub fn stub_a1c5a0() -> ! {
    todo!("0xa1c5a0 __ZN10XmlElementC1ERKN3RBX4NameE")
}

#[doc(alias = "void XmlElement::addAttribute<int>(RBX::Name const&,int)")]
// 0xa1c6fc — __ZN10XmlElement12addAttributeIiEEvRKN3RBX4NameET_
pub fn stub_a1c6fc() -> ! {
    todo!("0xa1c6fc __ZN10XmlElement12addAttributeIiEEvRKN3RBX4NameET_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(std::string)>::operator()(std::string)")]
// 0xa1d528 — __ZN3rbx7signals16signal_with_argsILi1EFvSsEEclESs
pub fn stub_a1d528() -> ! {
    todo!("0xa1d528 __ZN3rbx7signals16signal_with_argsILi1EFvSsEEclESs")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list_av_3<boost::function0<void>,RBX::MessageType,bool>::type> boost::bind<void,boost::function0<void> const&,RBX::MessageType,bool,boost::function0<void>,RBX::MessageType,bool>(void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::function0<void>,RBX::MessageType,bool)")]
// 0xa1e8e8 — __ZN5boost4bindIvRKNS_9function0IvEEN3RBX11MessageTypeEbS2_S6_bEENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_
pub fn stub_a1e8e8() -> ! {
    todo!("0xa1e8e8 __ZN5boost4bindIvRKNS_9function0IvEEN3RBX11MessageTypeEbS2_S6_bEENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_")
}

#[doc(alias = "std::map<int,std::set<std::string,std::less<std::string>,std::allocator<std::string>>,std::less<int>,std::allocator<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>>>::operator[](int const&)")]
// 0xa206a0 — __ZNSt3mapIiSt3setISsSt4lessISsESaISsEES1_IiESaISt4pairIKiS4_EEEixERS7_
pub fn stub_a206a0() -> ! {
    todo!("0xa206a0 __ZNSt3mapIiSt3setISsSt4lessISsESaISsEES1_IiESaISt4pairIKiS4_EEEixERS7_")
}

#[doc(alias = "boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::init(void)")]
// 0xa222e0 — __ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE4initEv
pub fn stub_a222e0() -> ! {
    todo!("0xa222e0 __ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE4initEv")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(RBX::Name const* const&)")]
// 0xa239f0 — __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEE5eraseERSA_
pub fn stub_a239f0() -> ! {
    todo!("0xa239f0 __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEE5eraseERSA_")
}

#[doc(alias = "non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
// 0xa24538 — __ZThn32_N3RBX12CylinderMeshD1Ev
// was: non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()
pub fn stub_a24538() -> ! {
    todo!("0xa24538 __ZThn32_N3RBX12CylinderMeshD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
// 0xa24548 — __ZThn36_N3RBX12CylinderMeshD0Ev
// was: non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()
pub fn stub_a24548() -> ! {
    todo!("0xa24548 __ZThn36_N3RBX12CylinderMeshD0Ev")
}

#[doc(alias = "RBX::BlockMesh::~BlockMesh()")]
// 0xa24bc8 — __ZN3RBX9BlockMeshD0Ev
pub fn stub_a24bc8() -> ! {
    todo!("0xa24bc8 __ZN3RBX9BlockMeshD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
// 0xa24cd8 — __ZThn32_N3RBX9BlockMeshD0Ev
// was: non-virtual thunk toRBX::BlockMesh::~BlockMesh()
pub fn stub_a24cd8() -> ! {
    todo!("0xa24cd8 __ZThn32_N3RBX9BlockMeshD0Ev")
}

#[doc(alias = "RBX::Teams * RBX::ServiceProvider::find<RBX::Teams>(void)const")]
// 0xa26b68 — __ZNK3RBX15ServiceProvider4findINS_5TeamsEEEPT_v
pub fn stub_a26b68() -> ! {
    todo!("0xa26b68 __ZNK3RBX15ServiceProvider4findINS_5TeamsEEEPT_v")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::insert(rbx::signals::signal<void ()(bool,int)>::slot *)")]
// 0xa27140 — __ZN3rbx7signals6signalIFvbiEE6insertEPNS3_4slotE
pub fn stub_a27140() -> ! {
    todo!("0xa27140 __ZN3rbx7signals6signalIFvbiEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::mutex(void)")]
// 0xa27404 — __ZN3rbx7signals6signalIFvbiEE5mutexEv
pub fn stub_a27404() -> ! {
    todo!("0xa27404 __ZN3rbx7signals6signalIFvbiEE5mutexEv")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,int)>::slot>::operator=(rbx::signals::signal<void ()(bool,int)>::slot*)")]
// 0xa27518 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbiEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot>::operator=(rbx::signals::signal<void ()(bool,int)>::slot*)
pub fn stub_a27518() -> ! {
    todo!("0xa27518 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbiEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,int)>::slot> const&)")]
// 0xa275cc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbiEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot> const&)
pub fn stub_a275cc() -> ! {
    todo!("0xa275cc __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbiEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::safe_static_init_mutex(void)")]
// 0xa27680 — __ZN3rbx7signals6signalIFvbiEE22safe_static_init_mutexEv
pub fn stub_a27680() -> ! {
    todo!("0xa27680 __ZN3rbx7signals6signalIFvbiEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::slot::disconnect(void)")]
// 0xa27828 — __ZN3rbx7signals6signalIFvbiEE4slot10disconnectEv
pub fn stub_a27828() -> ! {
    todo!("0xa27828 __ZN3rbx7signals6signalIFvbiEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::slot::connected(void)const")]
// 0xa279a8 — __ZNK3rbx7signals6signalIFvbiEE4slot9connectedEv
pub fn stub_a279a8() -> ! {
    todo!("0xa279a8 __ZNK3rbx7signals6signalIFvbiEE4slot9connectedEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::remove(rbx::signals::signal<void ()(bool,int)>::slot *)")]
// 0xa27dc8 — __ZN3rbx7signals6signalIFvbiEE6removeEPNS3_4slotE
pub fn stub_a27dc8() -> ! {
    todo!("0xa27dc8 __ZN3rbx7signals6signalIFvbiEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::slot::safe_static_init_mutex(void)")]
// 0xa27eb4 — __ZN3rbx7signals6signalIFvbiEE4slot22safe_static_init_mutexEv
pub fn stub_a27eb4() -> ! {
    todo!("0xa27eb4 __ZN3rbx7signals6signalIFvbiEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::slot::~slot()")]
// 0xa28234 — __ZN3rbx7signals6signalIFvbiEE4slotD1Ev
pub fn stub_a28234() -> ! {
    todo!("0xa28234 __ZN3rbx7signals6signalIFvbiEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::slot::~slot()")]
// 0xa28290 — __ZN3rbx7signals6signalIFvbiEE4slotD0Ev
pub fn stub_a28290() -> ! {
    todo!("0xa28290 __ZN3rbx7signals6signalIFvbiEE4slotD0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::mutex(void)")]
// 0xa28398 — __ZN3rbx7signals6signalIFvvEE5mutexEv
pub fn stub_a28398() -> ! {
    todo!("0xa28398 __ZN3rbx7signals6signalIFvvEE5mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::insert(rbx::signals::signal<void ()(std::string)>::slot *)")]
// 0xa28848 — __ZN3rbx7signals6signalIFvSsEE6insertEPNS3_4slotE
pub fn stub_a28848() -> ! {
    todo!("0xa28848 __ZN3rbx7signals6signalIFvSsEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::mutex(void)")]
// 0xa28b08 — __ZN3rbx7signals6signalIFvSsEE5mutexEv
pub fn stub_a28b08() -> ! {
    todo!("0xa28b08 __ZN3rbx7signals6signalIFvSsEE5mutexEv")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string)>::slot> const&)")]
// 0xa28c20 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot> const&)
pub fn stub_a28c20() -> ! {
    todo!("0xa28c20 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::slot::disconnect(void)")]
// 0xa28e38 — __ZN3rbx7signals6signalIFvSsEE4slot10disconnectEv
pub fn stub_a28e38() -> ! {
    todo!("0xa28e38 __ZN3rbx7signals6signalIFvSsEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::slot::connected(void)const")]
// 0xa28fb8 — __ZNK3rbx7signals6signalIFvSsEE4slot9connectedEv
pub fn stub_a28fb8() -> ! {
    todo!("0xa28fb8 __ZNK3rbx7signals6signalIFvSsEE4slot9connectedEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::remove(rbx::signals::signal<void ()(std::string)>::slot *)")]
// 0xa29158 — __ZN3rbx7signals6signalIFvSsEE6removeEPNS3_4slotE
pub fn stub_a29158() -> ! {
    todo!("0xa29158 __ZN3rbx7signals6signalIFvSsEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::slot::~slot()")]
// 0xa29248 — __ZN3rbx7signals6signalIFvSsEE4slotD1Ev
pub fn stub_a29248() -> ! {
    todo!("0xa29248 __ZN3rbx7signals6signalIFvSsEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::insert(rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot *)")]
// 0xa2a2c4 — __ZN3rbx7signals6signalIFvSsSsSsEE6insertEPNS3_4slotE
pub fn stub_a2a2c4() -> ! {
    todo!("0xa2a2c4 __ZN3rbx7signals6signalIFvSsSsSsEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::mutex(void)")]
// 0xa2a584 — __ZN3rbx7signals6signalIFvSsSsSsEE5mutexEv
pub fn stub_a2a584() -> ! {
    todo!("0xa2a584 __ZN3rbx7signals6signalIFvSsSsSsEE5mutexEv")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot*)")]
// 0xa2a698 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsSsEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot*)
pub fn stub_a2a698() -> ! {
    todo!("0xa2a698 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsSsEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot> const&)")]
// 0xa2a74c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsSsEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot> const&)
pub fn stub_a2a74c() -> ! {
    todo!("0xa2a74c __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsSsEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::safe_static_init_mutex(void)")]
// 0xa2a800 — __ZN3rbx7signals6signalIFvSsSsSsEE22safe_static_init_mutexEv
pub fn stub_a2a800() -> ! {
    todo!("0xa2a800 __ZN3rbx7signals6signalIFvSsSsSsEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot::disconnect(void)")]
// 0xa2aa4c — __ZN3rbx7signals6signalIFvSsSsSsEE4slot10disconnectEv
pub fn stub_a2aa4c() -> ! {
    todo!("0xa2aa4c __ZN3rbx7signals6signalIFvSsSsSsEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot::connected(void)const")]
// 0xa2abcc — __ZNK3rbx7signals6signalIFvSsSsSsEE4slot9connectedEv
pub fn stub_a2abcc() -> ! {
    todo!("0xa2abcc __ZNK3rbx7signals6signalIFvSsSsSsEE4slot9connectedEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::remove(rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot *)")]
// 0xa2b09c — __ZN3rbx7signals6signalIFvSsSsSsEE6removeEPNS3_4slotE
pub fn stub_a2b09c() -> ! {
    todo!("0xa2b09c __ZN3rbx7signals6signalIFvSsSsSsEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot::safe_static_init_mutex(void)")]
// 0xa2b188 — __ZN3rbx7signals6signalIFvSsSsSsEE4slot22safe_static_init_mutexEv
pub fn stub_a2b188() -> ! {
    todo!("0xa2b188 __ZN3rbx7signals6signalIFvSsSsSsEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot::~slot()")]
// 0xa2b26c — __ZN3rbx7signals6signalIFvSsSsSsEE4slotD1Ev
pub fn stub_a2b26c() -> ! {
    todo!("0xa2b26c __ZN3rbx7signals6signalIFvSsSsSsEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot::~slot()")]
// 0xa2b2c8 — __ZN3rbx7signals6signalIFvSsSsSsEE4slotD0Ev
pub fn stub_a2b2c8() -> ! {
    todo!("0xa2b2c8 __ZN3rbx7signals6signalIFvSsSsSsEE4slotD0Ev")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *),boost::_bi::list2<boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0xa2ba80 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionENS3_5list2INS_3argILi1EEENSB_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
pub fn stub_a2ba80() -> ! {
    todo!("0xa2ba80 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionENS3_5list2INS_3argILi1EEENSB_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *),boost::_bi::list2<boost::arg<1>,boost::arg<2>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// 0xa2bae0 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionENS3_5list2INS_3argILi1EEENSB_ILi2EEEEEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_
pub fn stub_a2bae0() -> ! {
    todo!("0xa2bae0 __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionENS3_5list2INS_3argILi1EEENSB_ILi2EEEEEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::CrispResponse> const&),boost::_bi::list1<boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0xa2baf0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_10shared_ptrIN3RBX13CrispResponseEEEENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::CrispResponse> const&),boost::_bi::list1<boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_a2baf0() -> ! {
    todo!("0xa2baf0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_10shared_ptrIN3RBX13CrispResponseEEEENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::CrispResponse> const&),boost::_bi::list1<boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::CrispResponse>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::CrispResponse>)")]
// 0xa2bb50 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvRKNS_10shared_ptrIN3RBX13CrispResponseEEEENS3_5list1INS_3argILi1EEEEEEEvS8_E6invokeERNS1_15function_bufferES8_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::CrispResponse> const&),boost::_bi::list1<boost::arg<1>>>,void,boost::shared_ptr<RBX::CrispResponse>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::CrispResponse>)
pub fn stub_a2bb50() -> ! {
    todo!("0xa2bb50 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvRKNS_10shared_ptrIN3RBX13CrispResponseEEEENS3_5list1INS_3argILi1EEEEEEEvS8_E6invokeERNS1_15function_bufferES8_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::string const&)")]
// 0xa2c370 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKSs
pub fn stub_a2c370() -> ! {
    todo!("0xa2c370 __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKSs")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::_M_erase(std::_Rb_tree_node<std::string> *)")]
// 0xa2c4ac — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE8_M_eraseEPSt13_Rb_tree_nodeISsE
pub fn stub_a2c4ac() -> ! {
    todo!("0xa2c4ac __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE8_M_eraseEPSt13_Rb_tree_nodeISsE")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::_M_copy(std::_Rb_tree_node<std::string> const*,std::_Rb_tree_node<std::string>*)")]
// 0xa2c524 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE7_M_copyEPKSt13_Rb_tree_nodeISsEPS7_
pub fn stub_a2c524() -> ! {
    todo!("0xa2c524 __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE7_M_copyEPKSt13_Rb_tree_nodeISsEPS7_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>,std::_Select1st<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>>,std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>> const&)")]
// 0xa2c6ec — __ZNSt8_Rb_treeIiSt4pairIKiSt3setISsSt4lessISsESaISsEEESt10_Select1stIS7_ES3_IiESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_a2c6ec() -> ! {
    todo!("0xa2c6ec __ZNSt8_Rb_treeIiSt4pairIKiSt3setISsSt4lessISsESaISsEEESt10_Select1stIS7_ES3_IiESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>,std::_Select1st<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>>>::_M_insert_unique(std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>> const&)")]
// 0xa2c864 — __ZNSt8_Rb_treeIiSt4pairIKiSt3setISsSt4lessISsESaISsEEESt10_Select1stIS7_ES3_IiESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_a2c864() -> ! {
    todo!("0xa2c864 __ZNSt8_Rb_treeIiSt4pairIKiSt3setISsSt4lessISsESaISsEEESt10_Select1stIS7_ES3_IiESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>,std::_Select1st<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>>>>::_M_create_node(std::pair<int const,std::set<std::string,std::less<std::string>,std::allocator<std::string>>> const&)")]
// 0xa2c918 — __ZNSt8_Rb_treeIiSt4pairIKiSt3setISsSt4lessISsESaISsEEESt10_Select1stIS7_ES3_IiESaIS7_EE14_M_create_nodeERKS7_
pub fn stub_a2c918() -> ! {
    todo!("0xa2c918 __ZNSt8_Rb_treeIiSt4pairIKiSt3setISsSt4lessISsESaISsEEESt10_Select1stIS7_ES3_IiESaIS7_EE14_M_create_nodeERKS7_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,bool)>::mutex(void)")]
// 0xa2efb0 — __ZN3rbx7signals6signalIFvSsbEE5mutexEv
pub fn stub_a2efb0() -> ! {
    todo!("0xa2efb0 __ZN3rbx7signals6signalIFvSsbEE5mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,bool)>::safe_static_init_mutex(void)")]
// 0xa2f0c8 — __ZN3rbx7signals6signalIFvSsbEE22safe_static_init_mutexEv
pub fn stub_a2f0c8() -> ! {
    todo!("0xa2f0c8 __ZN3rbx7signals6signalIFvSsbEE22safe_static_init_mutexEv")
}

#[doc(alias = "RBX::InsertService * RBX::ServiceProvider::find<RBX::InsertService>(void)const")]
// 0xa317c4 — __ZNK3RBX15ServiceProvider4findINS_13InsertServiceEEEPT_v
pub fn stub_a317c4() -> ! {
    todo!("0xa317c4 __ZNK3RBX15ServiceProvider4findINS_13InsertServiceEEEPT_v")
}

#[doc(alias = "RBX::FriendService * RBX::ServiceProvider::find<RBX::FriendService>(void)const")]
// 0xa33884 — __ZNK3RBX15ServiceProvider4findINS_13FriendServiceEEEPT_v
pub fn stub_a33884() -> ! {
    todo!("0xa33884 __ZNK3RBX15ServiceProvider4findINS_13FriendServiceEEEPT_v")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::string>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::string>>(std::string const&,boost::unordered::detail::emplace_args1<std::string> const&)")]
// 0xa34788 — __ZN5boost9unordered6detail10table_implINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISsEEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeISsEEEEbERKSsRKT_
pub fn stub_a34788() -> ! {
    todo!("0xa34788 __ZN5boost9unordered6detail10table_implINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISsEEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeISsEEEEbERKSsRKT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xa349f8 — __ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_a349f8() -> ! {
    todo!("0xa349f8 __ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xa34b98 — __ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_a34b98() -> ! {
    todo!("0xa34b98 __ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::thread_resource_error::thread_resource_error(void)")]
// 0xa35688 — __ZN5boost21thread_resource_errorC2Ev
pub fn stub_a35688() -> ! {
    todo!("0xa35688 __ZN5boost21thread_resource_errorC2Ev")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>)")]
// 0xa35924 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEEvT_
pub fn stub_a35924() -> ! {
    todo!("0xa35924 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0xa35a70 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
pub fn stub_a35a70() -> ! {
    todo!("0xa35a70 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")]
// 0xa35a98 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS5_5list3INS5_5valueIS8_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_a35a98() -> ! {
    todo!("0xa35a98 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS5_5list3INS5_5valueIS8_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xa35bd0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS5_5list3INS5_5valueIS8_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_a35bd0() -> ! {
    todo!("0xa35bd0 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS5_5list3INS5_5valueIS8_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xa35d88 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_a35d88() -> ! {
    todo!("0xa35d88 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>::storage3(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>)")]
// 0xa35f20 — __ZN5boost3_bi8storage3INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEENS2_IbEEEC2ES5_S8_S9_
pub fn stub_a35f20() -> ! {
    todo!("0xa35f20 __ZN5boost3_bi8storage3INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEENS2_IbEEEC2ES5_S8_S9_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>)")]
// 0xa36050 — __ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEEEC2ES5_S8_
pub fn stub_a36050() -> ! {
    todo!("0xa36050 __ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEEEC2ES5_S8_")
}

#[doc(alias = "char * boost::detail::lcast_put_unsigned<std::char_traits<char>,unsigned int,char>(unsigned int,char *)")]
// 0xa37ec4 — __ZN5boost6detail18lcast_put_unsignedISt11char_traitsIcEjcEEPT1_T0_S5_
pub fn stub_a37ec4() -> ! {
    todo!("0xa37ec4 __ZN5boost6detail18lcast_put_unsignedISt11char_traitsIcEjcEEPT1_T0_S5_")
}

#[doc(alias = "RBX::Allocator<XmlElement>::releaseMemory(void)")]
// 0xa3a960 — __ZN3RBX9AllocatorI10XmlElementE13releaseMemoryEv
pub fn stub_a3a960() -> ! {
    todo!("0xa3a960 __ZN3RBX9AllocatorI10XmlElementE13releaseMemoryEv")
}

#[doc(alias = "std::vector<unsigned long *,std::allocator<unsigned long *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned long **,std::vector<unsigned long *,std::allocator<unsigned long *>>>,unsigned long * const&)")]
// 0xa3a9d8 — __ZNSt6vectorIPmSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_
pub fn stub_a3a9d8() -> ! {
    todo!("0xa3a9d8 __ZNSt6vectorIPmSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::mutex(void)")]
// 0xa3e760 — __ZN3rbx7signals6signalIFvbEE5mutexEv
pub fn stub_a3e760() -> ! {
    todo!("0xa3e760 __ZN3rbx7signals6signalIFvbEE5mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::safe_static_init_mutex(void)")]
// 0xa3e878 — __ZN3rbx7signals6signalIFvbEE22safe_static_init_mutexEv
pub fn stub_a3e878() -> ! {
    todo!("0xa3e878 __ZN3rbx7signals6signalIFvbEE22safe_static_init_mutexEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::~table()")]
// 0xa3f718 — __ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEED2Ev
pub fn stub_a3f718() -> ! {
    todo!("0xa3f718 __ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEED2Ev")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendEventType>::construct_func(char const*,char *)")]
// 0xa43f88 — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE14construct_funcEPKcPc
pub fn stub_a43f88() -> ! {
    todo!("0xa43f88 __ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendEventType>::destruct_func(char *)")]
// 0xa43f94 — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE13destruct_funcEPc
pub fn stub_a43f94() -> ! {
    todo!("0xa43f94 __ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE13destruct_funcEPc")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::disconnectAll(void)")]
// 0xa48b00 — __ZN3rbx7signals6signalIFvSsEE13disconnectAllEv
pub fn stub_a48b00() -> ! {
    todo!("0xa48b00 __ZN3rbx7signals6signalIFvSsEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::function<void ()(std::string)>>::~callable_slot()")]
// 0xa49590 — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_a49590() -> ! {
    todo!("0xa49590 __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost8functionIS2_EEED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::call(std::string)")]
// 0xa495a0 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_E4callESs
pub fn stub_a495a0() -> ! {
    todo!("0xa495a0 __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_E4callESs")
}

#[doc(alias = "boost::function1<void,std::string>::operator()(std::string)const")]
// 0xa496c0 — __ZNK5boost9function1IvSsEclESs
pub fn stub_a496c0() -> ! {
    todo!("0xa496c0 __ZNK5boost9function1IvSsEclESs")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()")]
// 0xa498bc — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED2Ev
pub fn stub_a498bc() -> ! {
    todo!("0xa498bc __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED2Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()")]
// 0xa49a58 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev
pub fn stub_a49a58() -> ! {
    todo!("0xa49a58 __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev")
}

#[doc(alias = "RBX::FWValue<bool>::set(bool const&,RBX::FWRef *)")]
// 0xa50c10 — __ZN3RBX7FWValueIbE3setERKbPNS_5FWRefE
pub fn stub_a50c10() -> ! {
    todo!("0xa50c10 __ZN3RBX7FWValueIbE3setERKbPNS_5FWRefE")
}

#[doc(alias = "boost::tokenizer<boost::char_separator<char,std::char_traits<char>>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::string>::begin(void)const")]
// 0xa51130 — __ZNK5boost9tokenizerINS_14char_separatorIcSt11char_traitsIcEEEN9__gnu_cxx17__normal_iteratorIPKcSsEESsE5beginEv
pub fn stub_a51130() -> ! {
    todo!("0xa51130 __ZNK5boost9tokenizerINS_14char_separatorIcSt11char_traitsIcEEEN9__gnu_cxx17__normal_iteratorIPKcSsEESsE5beginEv")
}

#[doc(alias = "boost::tokenizer<boost::char_separator<char,std::char_traits<char>>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::string>::end(void)const")]
// 0xa51310 — __ZNK5boost9tokenizerINS_14char_separatorIcSt11char_traitsIcEEEN9__gnu_cxx17__normal_iteratorIPKcSsEESsE3endEv
pub fn stub_a51310() -> ! {
    todo!("0xa51310 __ZNK5boost9tokenizerINS_14char_separatorIcSt11char_traitsIcEEEN9__gnu_cxx17__normal_iteratorIPKcSsEESsE3endEv")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::vector(std::vector<std::string,std::allocator<std::string>> const&)")]
// 0xa514f0 — __ZNSt6vectorISsSaISsEEC2ERKS1_
pub fn stub_a514f0() -> ! {
    todo!("0xa514f0 __ZNSt6vectorISsSaISsEEC2ERKS1_")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,std::allocator<std::string>>>,std::string const&)")]
// 0xa516a0 — __ZNSt6vectorISsSaISsEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPSsS1_EERKSs
pub fn stub_a516a0() -> ! {
    todo!("0xa516a0 __ZNSt6vectorISsSaISsEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPSsS1_EERKSs")
}

#[doc(alias = "boost::token_iterator<boost::char_separator<char,std::char_traits<char>>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::string>::token_iterator(boost::char_separator<char,std::char_traits<char>>,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)")]
// 0xa51b40 — __ZN5boost14token_iteratorINS_14char_separatorIcSt11char_traitsIcEEEN9__gnu_cxx17__normal_iteratorIPKcSsEESsEC2ES4_S9_S9_
pub fn stub_a51b40() -> ! {
    todo!("0xa51b40 __ZN5boost14token_iteratorINS_14char_separatorIcSt11char_traitsIcEEEN9__gnu_cxx17__normal_iteratorIPKcSsEESsEC2ES4_S9_S9_")
}

#[doc(alias = "RBX::Http::~Http()")]
// 0xa51fe4 — __ZN3RBX4HttpD2Ev
pub fn stub_a51fe4() -> ! {
    todo!("0xa51fe4 __ZN3RBX4HttpD2Ev")
}

#[doc(alias = "void XmlElement::addAttribute<std::string>(RBX::Name const&,std::string)")]
// 0xa524a4 — __ZN10XmlElement12addAttributeISsEEvRKN3RBX4NameET_
pub fn stub_a524a4() -> ! {
    todo!("0xa524a4 __ZN10XmlElement12addAttributeISsEEvRKN3RBX4NameET_")
}
