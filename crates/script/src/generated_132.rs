// Auto-generated skeletons for rbx-script — filler EA-sorted asc next 68 uncovered (global)
// Filter: Script|Lua|Yield|lua (5401 filtered, all stubbed, 0 remaining)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +68 stubs | range 0x9e9bf0..0xa37008 EA-sorted asc next 68 uncovered (workspace uncovered 68->0, filler 7864->7932, rbx_core::SharedPtr not boost) [skeleton batch]
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem() [0x9e9bf0]")]
pub fn stub_0x9e9bf0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Network::Replicator::StatsItem::update(void)")]
pub fn stub_0x9e9c98(handle: &crate::slot::InstanceHandle) {
// RBX::Network::Replicator::StatsItem::update() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::Replicator::StatsItem::~StatsItem() [0x9ea2b8]")]
pub fn stub_0x9ea2b8(handle: crate::slot::InstanceHandle) {
// RBX::Network::Replicator::StatsItem dtor.
drop(handle);
}

#[doc(alias = "RBX::Network::Replicator::StatsItem::~StatsItem() [0x9ea2c4]")]
pub fn stub_0x9ea2c4(handle: crate::slot::InstanceHandle) {
// RBX::Network::Replicator::StatsItem dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::StatsItem::~StatsItem()")]
pub fn stub_0x9ea364(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::StatsItem::~StatsItem() [0x9ea370]")]
pub fn stub_0x9ea370(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::StatsItem::~StatsItem() [0x9ea414]")]
pub fn stub_0x9ea414(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::StatsItem::~StatsItem() [0x9ea420]")]
pub fn stub_0x9ea420(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::WeakPtr<RBX::Network::Player> RBX::weak_from<RBX::Network::Player>(RBX::Network::Player*)")]
pub fn stub_0xa20ac8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Network::Player")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::construct_func(char const*,char *)")]
pub fn stub_0xa22f2c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::construct_func(c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::destruct_func(char *)")]
pub fn stub_0xa22f38(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::destruct_func(ch~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::ChatOption>::construct_func(char const*,char *)")]
pub fn stub_0xa23424(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Network::Players::ChatOption>::construct_func(char ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::ChatOption>::destruct_func(char *)")]
pub fn stub_0xa23430(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Network::Players::ChatOption>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>>,std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType> const&)")]
pub fn stub_0xa24fec(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType> const&)")]
pub fn stub_0xa251a0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Network::Players::PlayerChatType,std::allocator<RBX::Network::Players::PlayerChatType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::Players::PlayerChatType*,std::vector<RBX::Network::Players::PlayerChatType,std::allocator<RBX::Network::Players::PlayerChatType>>>,RBX::Network::Players::PlayerChatType const&)")]
pub fn stub_0xa25290(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::Network::Players::PlayerChatType,std::allocator<RBX::Network::Players::PlayerChatType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Network::Players::PlayerChatType*,std::vector<RBX::Network::Players::PlayerChatType,std::allocator<RBX::Network::Players::PlayerChatType>>>,unsigned long,RBX::Network::Players::PlayerChatType const&)")]
pub fn stub_0xa253a0(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>>,std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption> const&)")]
pub fn stub_0xa2554c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption> const&)")]
pub fn stub_0xa25700(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Network::Players::ChatOption,std::allocator<RBX::Network::Players::ChatOption>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::Players::ChatOption*,std::vector<RBX::Network::Players::ChatOption,std::allocator<RBX::Network::Players::ChatOption>>>,RBX::Network::Players::ChatOption const&)")]
pub fn stub_0xa257f0(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::Network::Players::ChatOption,std::allocator<RBX::Network::Players::ChatOption>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Network::Players::ChatOption*,std::vector<RBX::Network::Players::ChatOption,std::allocator<RBX::Network::Players::ChatOption>>>,unsigned long,RBX::Network::Players::ChatOption const&)")]
pub fn stub_0xa25900(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_0xa27768(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>>::~callable_slot() [0xa27774]")]
pub fn stub_0xa27774(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::call(bool,int)")]
pub fn stub_0xa279b4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0xa279b4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::call(bool,int)")]
pub fn stub_0xa279dc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0xa279dc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list2<bool &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int> &,boost::_bi::list2<bool &,int &> &,int)")]
pub fn stub_0xa27a04(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xa27a04: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>::operator()(RBX::Network::Players*,bool,rbx_core::WeakPtr<RBX::Network::Player>,int)const")]
pub fn stub_0xa27be8() -> crate::slot::BindPiece {
// boost::bind fragment (mf3) composing a host BoundCall.
crate::slot::BindPiece::new("mf3")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()")]
pub fn stub_0xa27f9c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0xa27f9c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable() [0xa28174]")]
pub fn stub_0xa28174(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0xa28174: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable() [0xa28180]")]
pub fn stub_0xa28180(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0xa28180: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::list4(boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>)")]
pub fn stub_0xa284b0() -> crate::slot::BindPiece {
// boost::bind fragment (list4) composing a host BoundCall.
crate::slot::BindPiece::new("list4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::storage4(boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>)")]
pub fn stub_0xa28674() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")]
pub fn stub_0xa28cd4(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot() [0xa28d30]")]
pub fn stub_0xa28d30(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(std::string)>::call(std::string)")]
pub fn stub_0xa28fc4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0xa28fc4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(std::string)>::call(std::string)")]
pub fn stub_0xa28fe0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0xa28fe0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>::operator()<boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool> &,boost::_bi::list1<std::string &> &,int)")]
pub fn stub_0xa28ffc(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xa28ffc: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>>::~callable_slot()")]
pub fn stub_0xa292a8(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>>::~callable_slot() [0xa29304]")]
pub fn stub_0xa29304(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>,0,void ()(void)>::call(void)")]
pub fn stub_0xa29410(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0xa29410: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>,0,void ()(void)>::call(void)")]
pub fn stub_0xa29430(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0xa29430: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
pub fn stub_0xa2a8e8(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot() [0xa2a944]")]
pub fn stub_0xa2a944(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)")]
pub fn stub_0xa2abd8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0xa2abd8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)")]
pub fn stub_0xa2abf4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0xa2abf4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list3<std::string &,std::string &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string> &,boost::_bi::list3<std::string &,std::string &,std::string &> &,int)")]
pub fn stub_0xa2ac10(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xa2ac10: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>::operator()(RBX::Network::Players*,int,std::string,std::string,std::string)const")]
pub fn stub_0xa2ae4c() -> crate::slot::BindPiece {
// boost::bind fragment (mf4) composing a host BoundCall.
crate::slot::BindPiece::new("mf4")
}

#[doc(alias = "RBX::Network::Client * RBX::ServiceProvider::find<RBX::Network::Client>(void)const")]
pub fn stub_0xa2d2d0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::Network::Client"))
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::Client>(void)")]
pub fn stub_0xa2d8a8() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "RBX::Network::Server * RBX::ServiceProvider::find<RBX::Network::Server>(void)const")]
pub fn stub_0xa2e90c() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::Network::Server"))
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::Server>(void)")]
pub fn stub_0xa2eee4() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>)")]
pub fn stub_0xa30694() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::Players>(void)")]
pub fn stub_0xa329a8() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::slot> &)")]
pub fn stub_0xa34c4c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (RBX::Network::AbuseReport)>::slot")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Network::AbuseReport)>::fireItem(rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::slot *,RBX::Network::AbuseReport)")]
pub fn stub_0xa34e60(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<1, void (RBX::Network::AbuseReport)>::fireItem(rbx::signals~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::mutex(void)")]
pub fn stub_0xa351d4(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Network::AbuseReport)>::mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::slot> const&)")]
pub fn stub_0xa352e8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::safe_static_init_mutex(void)")]
pub fn stub_0xa3539c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Network::AbuseReport)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::mutex(void)")]
pub fn stub_0xa35488() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RakNet::BitStream")
}

#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::safe_static_init_mutex(void)")]
pub fn stub_0xa355a0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RakNet::BitStream")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RakNet::Packet *>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RakNet::Packet *>>>)")]
pub fn stub_0xa361b0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)")]
pub fn stub_0xa36784() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RakNet::Packet *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0xa368fc(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RakNet::Packet *>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0xa36920(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RakNet::Packet *>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RakNet::Packet *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xa36938(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RakNet::Packet *>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *> &,boost::_bi::list0 &,int)")]
pub fn stub_0xa36c8c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xa36c8c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "void boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>::call<rbx_core::SharedPtr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *>(rbx_core::SharedPtr<RBX::Network::Players> &,void const*,std::string &,std::string &,RakNet::Packet * &)const")]
pub fn stub_0xa36e40() -> crate::slot::BindPiece {
// boost::bind fragment (mf3) composing a host BoundCall.
crate::slot::BindPiece::new("mf3")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RakNet::Packet *>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xa37008(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}
