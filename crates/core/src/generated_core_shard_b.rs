//! core shard B — 100 core stubs EA-sorted, next uncovered after shard A.
//! Source: ida/export.json filtered where demangled contains boost::|RBX::Signals|shared_ptr|weak_ptr|function|bind, excluding Reflection/Instance/Ogre/RakNet/Network, EA-sorted, next 100 after existing stubs.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#[doc(alias = "-[ControlView bindToUserInputService:]")]
// 0x481cc — -[ControlView bindToUserInputService:] — -[ControlView bindToUserInputService:]
pub fn stub_0x481cc() {
    // IDA 0x481cc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[ControlView bindUserInputService]")]
// 0x48604 — -[ControlView bindUserInputService] — -[ControlView bindUserInputService]
pub fn stub_0x48604() {
    // IDA 0x48604: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::lua_tofunction(lua_State *,int)")]
// 0x2ca520 — __ZN3RBX3Lua14lua_tofunctionEP9lua_Statei — RBX::Lua::lua_tofunction(lua_State *,int)
pub fn stub_0x2ca520() {
    // IDA 0x2ca520: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::lua_pushfunction(lua_State *,RBX::Lua::WeakFunctionRef const&)")]
// 0x2ca52c — __ZN3RBX3Lua16lua_pushfunctionEP9lua_StateRKNS0_15WeakFunctionRefE — RBX::Lua::lua_pushfunction(lua_State *,RBX::Lua::WeakFunctionRef const&)
pub fn stub_0x2ca52c() {
    // IDA 0x2ca52c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::callable_slot<boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>>::~callable_slot()")]
// 0x632a38 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE13callable_slotIN5boost8functionIS5_EEED0Ev — rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::callable_slot<boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>>::~callable_slot()
pub fn stub_0x632a38() {
    // IDA 0x632a38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::call(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)")]
// 0x632c84 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_S5_ — rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::call(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)
pub fn stub_0x632c84() {
    // IDA 0x632c84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::call(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)")]
// 0x632c8c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_S5_ — non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::call(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::call(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)
pub fn stub_0x632c8c() {
    // IDA 0x632c8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::operator()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)const")]
// 0x632c94 — __ZNK5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_EclES3_S3_ — boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::operator()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)const
pub fn stub_0x632c94() {
    // IDA 0x632c94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::~callable()")]
// 0x632f40 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_ED1Ev — rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::~callable()
pub fn stub_0x632f40() {
    // IDA 0x632f40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::~callable()")]
// 0x633050 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_ED0Ev — rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::~callable()
pub fn stub_0x633050() {
    // IDA 0x633050: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to_own(boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState> const&)")]
// 0x633280 — __ZN5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_E13assign_to_ownERKS4_ — boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to_own(boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState> const&)
pub fn stub_0x633280() {
    // IDA 0x633280: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SocialService::getRankInGroup(int,int,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0x6396c4 — __ZN3RBX13SocialService14getRankInGroupEiiN5boost8functionIFviEEENS2_IFvSsEEE — RBX::SocialService::getRankInGroup(int,int,boost::function<void ()(int)>,boost::function<void ()(std::string)>)
pub fn stub_0x6396c4() {
    // IDA 0x6396c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SocialService::getRoleInGroup(int,int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0x639910 — __ZN3RBX13SocialService14getRoleInGroupEiiN5boost8functionIFvSsEEES4_ — RBX::SocialService::getRoleInGroup(int,int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
pub fn stub_0x639910() {
    // IDA 0x639910: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SocialService::isFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x639b5c — __ZN3RBX13SocialService13isFriendsWithEiiN5boost8functionIFvbEEENS2_IFvSsEEE — RBX::SocialService::isFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
pub fn stub_0x639b5c() {
    // IDA 0x639b5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SocialService::isBestFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x639da8 — __ZN3RBX13SocialService17isBestFriendsWithEiiN5boost8functionIFvbEEENS2_IFvSsEEE — RBX::SocialService::isBestFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
pub fn stub_0x639da8() {
    // IDA 0x639da8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::SocialService::isInGroup(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x639ff4 — __ZN3RBX13SocialService9isInGroupEiiN5boost8functionIFvbEEENS2_IFvSsEEE — RBX::SocialService::isInGroup(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
pub fn stub_0x639ff4() {
    // IDA 0x639ff4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::SocialService::dispatchRequest<int>(std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0x63a5e0 — __ZN3RBX13SocialService15dispatchRequestIiEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE — void RBX::SocialService::dispatchRequest<int>(std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)
pub fn stub_0x63a5e0() {
    // IDA 0x63a5e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::SocialService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0x63a888 — __ZN3RBX13SocialService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE — void RBX::SocialService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
pub fn stub_0x63a888() {
    // IDA 0x63a888: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::SocialService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x63ab30 — __ZN3RBX13SocialService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE — void RBX::SocialService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
pub fn stub_0x63ab30() {
    // IDA 0x63ab30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Stats::StatsService::addHeader(rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>)")]
// 0x646628 — __ZN3RBX5Stats12StatsService9addHeaderEN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEE — RBX::Stats::StatsService::addHeader(rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>)
// was: RBX::Stats::StatsService::addHeader(boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>)
pub fn stub_0x646628() {
    // IDA 0x646628: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Stats::StatsService::postReportWithUrl(std::string const&,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>)")]
// 0x646cbc — __ZN3RBX5Stats12StatsService17postReportWithUrlERKSsN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEE — RBX::Stats::StatsService::postReportWithUrl(std::string const&,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>)
// was: RBX::Stats::StatsService::postReportWithUrl(std::string const&,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>)
pub fn stub_0x646cbc() {
    // IDA 0x646cbc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Stats::StatsService::postReport(rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>)")]
// 0x6471c4 — __ZN3RBX5Stats12StatsService10postReportEN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEE — RBX::Stats::StatsService::postReport(rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>)
// was: RBX::Stats::StatsService::postReport(boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>)
pub fn stub_0x6471c4() {
    // IDA 0x6471c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Stats::StatsService::reportJob(rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &)")]
// 0x64732c — __ZN3RBX5Stats12StatsService9reportJobEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEENS3_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERb — RBX::Stats::StatsService::reportJob(rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &)
// was: RBX::Stats::StatsService::reportJob(boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &)
pub fn stub_0x64732c() {
    // IDA 0x64732c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>)")]
// 0x64ac68 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_5Stats12StatsServiceES8_NS3_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEENSE_5list4INSE_5valueIPSJ_EENS2_3argILi1EEENST_ISP_EENS2_17reference_wrapperIbEEEEEEET0_T_S14_S13_ — boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>)
// was: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>)
pub fn stub_0x64ac68() {
    // IDA 0x64ac68: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list_av_4<RBX::Stats::StatsService*,boost::arg<1>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>>::type> boost::bind<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &,RBX::Stats::StatsService*,boost::arg<1>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>>(void (RBX::Stats::StatsService::*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &),RBX::Stats::StatsService*,boost::arg<1>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>)")]
// 0x64acd4 — __ZN5boost4bindIvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS1_13TaskScheduler3JobEEENS4_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbPS3_NS_3argILi1EEESE_NS_17reference_wrapperIbEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISN_T0_T1_T2_T3_EENSL_9list_av_4IT4_T5_T6_T7_E4typeEEEMSQ_FSN_SR_SS_ST_ESW_SX_SY_SZ_ — boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list_av_4<RBX::Stats::StatsService*,boost::arg<1>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>>::type> boost::bind<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &,RBX::Stats::StatsService*,boost::arg<1>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>>(void (RBX::Stats::StatsService::*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &),RBX::Stats::StatsService*,boost::arg<1>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>)
// was: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list_av_4<RBX::Stats::StatsService*,boost::arg<1>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>>::type> boost::bind<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &,RBX::Stats::StatsService*,boost::arg<1>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>>(void (RBX::Stats::StatsService::*)(boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &),RBX::Stats::StatsService*,boost::arg<1>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>)
pub fn stub_0x64acd4() {
    // IDA 0x64acd4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::function0<float>::operator()(void)const")]
// 0x64cf58 — __ZNK5boost9function0IfEclEv — boost::function0<float>::operator()(void)const
pub fn stub_0x64cf58() {
    // IDA 0x64cf58: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::function0<float>::clear(void)")]
// 0x64d020 — __ZN5boost9function0IfE5clearEv — boost::function0<float>::clear(void)
pub fn stub_0x64d020() {
    // IDA 0x64d020: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::function0<unsigned long>::operator()(void)const")]
// 0x64e354 — __ZNK5boost9function0ImEclEv — boost::function0<unsigned long>::operator()(void)const
pub fn stub_0x64e354() {
    // IDA 0x64e354: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::function0<unsigned long>::clear(void)")]
// 0x64e418 — __ZN5boost9function0ImE5clearEv — boost::function0<unsigned long>::clear(void)
pub fn stub_0x64e418() {
    // IDA 0x64e418: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<unsigned long const&,unsigned long const& (*)(unsigned long const*),boost::_bi::list1<boost::_bi::value<unsigned long const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x64e448 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKmPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE — boost::detail::function::functor_manager<boost::_bi::bind_t<unsigned long const&,unsigned long const& (*)(unsigned long const*),boost::_bi::list1<boost::_bi::value<unsigned long const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x64e448() {
    // IDA 0x64e448: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<unsigned long const&,unsigned long const& (*)(unsigned long const*),boost::_bi::list1<boost::_bi::value<unsigned long const*>>>,unsigned long>::invoke(boost::detail::function::function_buffer &)")]
// 0x64e4a8 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKmPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEmE6invokeERNS1_15function_bufferE — boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<unsigned long const&,unsigned long const& (*)(unsigned long const*),boost::_bi::list1<boost::_bi::value<unsigned long const*>>>,unsigned long>::invoke(boost::detail::function::function_buffer &)
pub fn stub_0x64e4a8() {
    // IDA 0x64e4a8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)")]
// 0x652a78 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEixERS5_ — boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)
pub fn stub_0x652a78() {
    // IDA 0x652a78: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::construct_with_value<boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>>(boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>> const&)")]
// 0x652cb0 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE20construct_with_valueINS1_13emplace_args3INS0_21piecewise_construct_tENS_6tuples5tupleISsNSF_9null_typeESH_SH_SH_SH_SH_SH_SH_SH_EENSG_ISH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EEEEEEvRKT_ — void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::construct_with_value<boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>>(boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>> const&)
pub fn stub_0x652cb0() {
    // IDA 0x652cb0: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0x652cd4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)
pub fn stub_0x652cd4() {
    // IDA 0x652cd4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::~node_constructor()")]
// 0x652d24 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEED2Ev — boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::~node_constructor()
pub fn stub_0x652d24() {
    // IDA 0x652d24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0x652d40 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)
pub fn stub_0x652d40() {
    // IDA 0x652d40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// 0x652e68 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const
pub fn stub_0x652e68() {
    // IDA 0x652e68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// 0x652ef8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm — boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)
pub fn stub_0x652ef8() {
    // IDA 0x652ef8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x652f24 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISE_EEPNS1_10ptr_bucketE — boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)
pub fn stub_0x652f24() {
    // IDA 0x652f24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::construct(void)")]
// 0x652f7c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE9constructEv — boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::construct(void)
pub fn stub_0x652f7c() {
    // IDA 0x652f7c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0x652fe0 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSD_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS8_EEEEmRKT_RKT0_ — boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const
pub fn stub_0x652fe0() {
    // IDA 0x652fe0: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0x65304c — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_ — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const
pub fn stub_0x65304c() {
    // IDA 0x65304c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::operator()<boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&> &,int)")]
// 0x65308c — __ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEclINS_4_mfi3mf3IvS5_NSA_IKNS3_13TaskScheduler3JobEEESG_RbEENS0_5list1IRSR_EEEEvNS0_4typeIvEERT_RT0_i — void boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::operator()<boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&> &,int)
// was: void boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::operator()<boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &> &,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job const>&> &,int)
pub fn stub_0x65308c() {
    // IDA 0x65308c: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>::operator()(RBX::Stats::StatsService*,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &)const")]
// 0x6531ac — __ZNK5boost4_mfi3mf3IvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS2_13TaskScheduler3JobEEENS5_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEclEPS4_S9_SF_SG_ — boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>::operator()(RBX::Stats::StatsService*,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &)const
// was: boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>::operator()(RBX::Stats::StatsService*,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &)const
pub fn stub_0x6531ac() {
    // IDA 0x6531ac: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::list4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)")]
// 0x6532e4 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_ — boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::list4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)
// was: boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::list4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)
pub fn stub_0x6532e4() {
    // IDA 0x6532e4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::storage4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)")]
// 0x6533c4 — __ZN5boost3_bi8storage4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_ — boost::_bi::storage4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::storage4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)
// was: boost::_bi::storage4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::storage4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)
pub fn stub_0x6533c4() {
    // IDA 0x6533c4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
// 0x6534f8 — __ZN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEC2IS5_EEPT_ — rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> *)
// was: boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> *)
pub fn stub_0x6534f8() {
    // IDA 0x6534f8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
// 0x6535cc — __ZN5boost6detail12shared_countC2ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEPT_ — boost::detail::shared_count::shared_count<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> *)
pub fn stub_0x6535cc() {
    // IDA 0x6535cc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")]
// 0x6536c4 — __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEED1Ev — boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()
pub fn stub_0x6536c4() {
    // IDA 0x6536c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")]
// 0x6536c8 — __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEED0Ev — boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()
pub fn stub_0x6536c8() {
    // IDA 0x6536c8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::dispose(void)")]
// 0x6536cc — __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE7disposeEv — boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::dispose(void)
pub fn stub_0x6536cc() {
    // IDA 0x6536cc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::get_deleter(std::type_info const&)")]
// 0x6536dc — __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::get_deleter(std::type_info const&)
pub fn stub_0x6536dc() {
    // IDA 0x6536dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::get_untyped_deleter(void)")]
// 0x6536e0 — __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::get_untyped_deleter(void)
pub fn stub_0x6536e0() {
    // IDA 0x6536e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<void (*)(std::string *,std::exception *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x6536e8 — __ZN5boost6detail8function15functor_managerIPFvPSsPSt9exceptionEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE — boost::detail::function::functor_manager<void (*)(std::string *,std::exception *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x6536e8() {
    // IDA 0x6536e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0x6562e4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)
pub fn stub_0x6562e4() {
    // IDA 0x6562e4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0x65631c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)
pub fn stub_0x65631c() {
    // IDA 0x65631c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)")]
// 0x665064 — __ZN3RBX11shared_fromINS_4TeamEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)
// was: boost::shared_ptr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)
pub fn stub_0x665064() {
    // IDA 0x665064: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(char const*,bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0x668884 — __ZN3rbx7signals6signalIFvPKcbEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(char const*,bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>> const&)
pub fn stub_0x668884() {
    // IDA 0x668884: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::operator()(rbx_core::SharedPtr<RBX::TextBox>)")]
// 0x6688f8 — __ZN3rbx7signals16signal_with_argsILi1EFvN5boost10shared_ptrIN3RBX7TextBoxEEEEEclES6_ — rbx::signals::signal_with_args<1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::operator()(rbx_core::SharedPtr<RBX::TextBox>)
// was: rbx::signals::signal_with_args<1,void ()(boost::shared_ptr<RBX::TextBox>)>::operator()(boost::shared_ptr<RBX::TextBox>)
pub fn stub_0x6688f8() {
    // IDA 0x6688f8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox> RBX::shared_from<RBX::TextBox>(RBX::TextBox*)")]
// 0x668adc — __ZN3RBX11shared_fromINS_7TextBoxEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::TextBox> RBX::shared_from<RBX::TextBox>(RBX::TextBox*)
// was: boost::shared_ptr<RBX::TextBox> RBX::shared_from<RBX::TextBox>(RBX::TextBox*)
pub fn stub_0x668adc() {
    // IDA 0x668adc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot> &)")]
// 0x66996c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE — rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot> &)
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot> &)
pub fn stub_0x66996c() {
    // IDA 0x66996c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::on_error(std::exception &)")]
// 0x669acc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE8on_errorERSt9exception — rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::on_error(std::exception &)
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::on_error(std::exception &)
pub fn stub_0x669acc() {
    // IDA 0x669acc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(rbx::signals::signal<void ()(char const*,bool)>::slot*)")]
// 0x669d00 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKcbEE4slotEEaSEPS8_ — boost::intrusive_ptr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(rbx::signals::signal<void ()(char const*,bool)>::slot*)
pub fn stub_0x669d00() {
    // IDA 0x669d00: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x669d24 — __ZN3rbx7signals6signalIFvPKcbEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEED1Ev — rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
pub fn stub_0x669d24() {
    // IDA 0x669d24: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x669d50 — __ZN3rbx7signals6signalIFvPKcbEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEED0Ev — rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
pub fn stub_0x669d50() {
    // IDA 0x669d50: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)")]
// 0x669f40 — __ZN3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_E4callES4_b — rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)
pub fn stub_0x669f40() {
    // IDA 0x669f40: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)")]
// 0x669f68 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_E4callES4_b — non-virtual thunk torbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)
pub fn stub_0x669f68() {
    // IDA 0x669f68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::TextBox *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list2<char const*&,bool &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool> &,boost::_bi::list2<char const*&,bool &> &,int)")]
// 0x669f90 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX7TextBoxEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_PKcbEENS0_5list2IRSF_RbEEEEvNS0_4typeIvEERT_RT0_i — void boost::_bi::list3<boost::_bi::value<RBX::TextBox *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list2<char const*&,bool &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool> &,boost::_bi::list2<char const*&,bool &> &,int)
pub fn stub_0x669f90() {
    // IDA 0x669f90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()")]
// 0x66a2a0 — __ZN3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_ED1Ev — rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()
pub fn stub_0x66a2a0() {
    // IDA 0x66a2a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()")]
// 0x66a2cc — __ZN3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_ED0Ev — rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()
pub fn stub_0x66a2cc() {
    // IDA 0x66a2cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::scoped_ptr<RBX::GuiObject::Tween>::~scoped_ptr()")]
// 0x66afbc — __ZN5boost10scoped_ptrIN3RBX9GuiObject5TweenEED2Ev — boost::scoped_ptr<RBX::GuiObject::Tween>::~scoped_ptr()
pub fn stub_0x66afbc() {
    // IDA 0x66afbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::clear(void)")]
// 0x66b068 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5clearEv — boost::function1<void,RBX::GuiObject::TweenStatus>::clear(void)
pub fn stub_0x66b068() {
    // IDA 0x66b068: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot> const&)")]
// 0x66b358 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiEE4slotEEaSERKS7_ — boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot> const&)
pub fn stub_0x66b358() {
    // IDA 0x66b358: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> const&)")]
// 0x66b73c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSERKS9_ — boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> const&)
pub fn stub_0x66b73c() {
    // IDA 0x66b73c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TimerService::delay(boost::function0<void>,double)")]
// 0x67d650 — __ZN3RBX12TimerService5delayEN5boost9function0IvEEd — RBX::TimerService::delay(boost::function0<void>,double)
pub fn stub_0x67d650() {
    // IDA 0x67d650: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function0<void>::operator=(boost::function0<void> const&)")]
// 0x67d8fc — __ZN5boost9function0IvEaSERKS1_ — boost::function0<void>::operator=(boost::function0<void> const&)
pub fn stub_0x67d8fc() {
    // IDA 0x67d8fc: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Mouse>::operator=(rbx_core::SharedPtr<RBX::Mouse> const&)")]
// 0x682a28 — __ZN5boost10shared_ptrIN3RBX5MouseEEaSERKS3_ — rbx_core::SharedPtr<RBX::Mouse>::operator=(rbx_core::SharedPtr<RBX::Mouse> const&)
// was: boost::shared_ptr<RBX::Mouse>::operator=(boost::shared_ptr<RBX::Mouse> const&)
pub fn stub_0x682a28() {
    // IDA 0x682a28: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ToolMouseCommand>::shared_ptr<RBX::ToolMouseCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x683b2c — __ZN5boost10shared_ptrIN3RBX16ToolMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_ — rbx_core::SharedPtr<RBX::ToolMouseCommand>::shared_ptr<RBX::ToolMouseCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)
// was: boost::shared_ptr<RBX::ToolMouseCommand>::shared_ptr<RBX::ToolMouseCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x683b2c() {
    // IDA 0x683b2c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::ToolMouseCommand,RBX::ToolMouseCommand>(rbx_core::SharedPtr<RBX::ToolMouseCommand> const*,RBX::ToolMouseCommand *)const")]
// 0x683bf4 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16ToolMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::ToolMouseCommand,RBX::ToolMouseCommand>(rbx_core::SharedPtr<RBX::ToolMouseCommand> const*,RBX::ToolMouseCommand *)const
// was: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::ToolMouseCommand,RBX::ToolMouseCommand>(boost::shared_ptr<RBX::ToolMouseCommand> const*,RBX::ToolMouseCommand *)const
pub fn stub_0x683bf4() {
    // IDA 0x683bf4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x683cd8 — __ZN5boost6detail12shared_countC2IPN3RBX16ToolMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x683cd8() {
    // IDA 0x683cd8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x683dd0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x683dd0() {
    // IDA 0x683dd0: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x683dd4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x683dd4() {
    // IDA 0x683dd4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x683dd8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)
pub fn stub_0x683dd8() {
    // IDA 0x683dd8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x683de8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_0x683de8() {
    // IDA 0x683de8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x683e00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)
pub fn stub_0x683e00() {
    // IDA 0x683e00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>)")]
// 0x6857a0 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_ — boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>)
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::list2(boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>)
pub fn stub_0x6857a0() {
    // IDA 0x6857a0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>)")]
// 0x685870 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_ — boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>)
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>)
pub fn stub_0x685870() {
    // IDA 0x685870: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>> const&)")]
// 0x6891ec — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>> const&)
pub fn stub_0x6891ec() {
    // IDA 0x6891ec: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>>::~callable_slot()")]
// 0x689420 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev — rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>>::~callable_slot()
pub fn stub_0x689420() {
    // IDA 0x689420: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>>::~callable_slot()")]
// 0x68944c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev — rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>>::~callable_slot()
pub fn stub_0x68944c() {
    // IDA 0x68944c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::call(void)")]
// 0x689520 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::call(void)
pub fn stub_0x689520() {
    // IDA 0x689520: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::call(void)")]
// 0x689528 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv — non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::call(void)
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::call(void)
pub fn stub_0x689528() {
    // IDA 0x689528: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>::operator()(void)")]
// 0x689530 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS0_5list1INS0_5valueIPS5_EEEEEclEv — boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>::operator()(void)
pub fn stub_0x689530() {
    // IDA 0x689530: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::~callable()")]
// 0x689548 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::~callable()
pub fn stub_0x689548() {
    // IDA 0x689548: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::~callable()")]
// 0x689574 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>,0,void ()(void)>::~callable()
pub fn stub_0x689574() {
    // IDA 0x689574: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DecalTool> RBX::shared_from<RBX::DecalTool>(RBX::DecalTool*)")]
// 0x68c5bc — __ZN3RBX11shared_fromINS_9DecalToolEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::DecalTool> RBX::shared_from<RBX::DecalTool>(RBX::DecalTool*)
// was: boost::shared_ptr<RBX::DecalTool> RBX::shared_from<RBX::DecalTool>(RBX::DecalTool*)
pub fn stub_0x68c5bc() {
    // IDA 0x68c5bc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::scoped_ptr<RBX::TouchDebouncer>::~scoped_ptr()")]
// 0x68e938 — __ZN5boost10scoped_ptrIN3RBX14TouchDebouncerEED2Ev — boost::scoped_ptr<RBX::TouchDebouncer>::~scoped_ptr()
pub fn stub_0x68e938() {
    // IDA 0x68e938: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Controller::bindButton(RBX::Controller::Button,std::string)")]
// 0x690298 — __ZN3RBX10Controller10bindButtonENS0_6ButtonESs — RBX::Controller::bindButton(RBX::Controller::Button,std::string)
pub fn stub_0x690298() {
    // IDA 0x690298: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Controller::unbindButton(RBX::Controller::Button)")]
// 0x6907a0 — __ZN3RBX10Controller12unbindButtonENS0_6ButtonE — RBX::Controller::unbindButton(RBX::Controller::Button)
pub fn stub_0x6907a0() {
    // IDA 0x6907a0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::VehicleController::onSteppedKeyboardInput(rbx_core::SharedPtr<RBX::VehicleSeat>)")]
// 0x692234 — __ZN3RBX17VehicleController22onSteppedKeyboardInputEN5boost10shared_ptrINS_11VehicleSeatEEE — RBX::VehicleController::onSteppedKeyboardInput(rbx_core::SharedPtr<RBX::VehicleSeat>)
// was: RBX::VehicleController::onSteppedKeyboardInput(boost::shared_ptr<RBX::VehicleSeat>)
pub fn stub_0x692234() {
    // IDA 0x692234: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}
