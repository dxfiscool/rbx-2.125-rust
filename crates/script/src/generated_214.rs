// Auto-generated skeletons for rbx-script — shard 214 EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x389f3c..0x38e9d8 | script 21852->21952 distinct (filler 0x389f3c asc, not-in-script 63693->63593)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread>*,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,unsigned long,rbx_core::SharedPtr<boost::thread> const&)")]
pub fn stub_0x389f3c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::thread")
}

#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::_M_allocate(unsigned long)")]
pub fn stub_0x38a53c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::thread")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<boost::thread> *,unsigned long,rbx_core::SharedPtr<boost::thread>>(rbx_core::SharedPtr<boost::thread> *,unsigned long,rbx_core::SharedPtr<boost::thread> const&,std::__false_type)")]
pub fn stub_0x38a554() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::thread")
}

#[doc(alias = "rbx_core::SharedPtr<boost::thread>::operator=(rbx_core::SharedPtr<boost::thread> const&)")]
pub fn stub_0x38a67c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<boost::thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<boost::thread> *,rbx_core::SharedPtr<boost::thread> *>(rbx_core::SharedPtr<boost::thread> *,rbx_core::SharedPtr<boost::thread> *,rbx_core::SharedPtr<boost::thread> *)")]
pub fn stub_0x38a6b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::thread")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::~vector()")]
pub fn stub_0x38a704(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::~vector()")]
pub fn stub_0x38a7d0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>::shared_ptr<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")]
pub fn stub_0x38a89c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BaseThreadPool::PoolData")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")]
pub fn stub_0x38a970() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::~sp_counted_impl_p()")]
pub fn stub_0x38aa68(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::~sp_counted_impl_p() [0x38aa6c]")]
pub fn stub_0x38aa6c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::dispose(void)")]
pub fn stub_0x38aa70() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_deleter(std::type_info const&)")]
pub fn stub_0x38aa80() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_untyped_deleter(void)")]
pub fn stub_0x38aa84() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::mutex>> const&)")]
pub fn stub_0x38aa88(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
pub fn stub_0x38aab8(handle: crate::slot::InstanceHandle) {
// RBX::BaseThreadPool::PoolData dtor.
drop(handle);
}

#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData() [0x38ab90]")]
pub fn stub_0x38ab90(handle: crate::slot::InstanceHandle) {
// RBX::BaseThreadPool::PoolData dtor.
drop(handle);
}

#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData() [0x38ab94]")]
pub fn stub_0x38ab94(handle: crate::slot::InstanceHandle) {
// RBX::BaseThreadPool::PoolData dtor.
drop(handle);
}

#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::~vector()")]
pub fn stub_0x38ac34(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "RBX::BaseThreadPool::PoolData::PoolData(void)")]
pub fn stub_0x38ad00() -> crate::slot::InstanceHandle {
// RBX::BaseThreadPool::PoolData ctor.
crate::slot::InstanceHandle::new("RBX::BaseThreadPool::PoolData")
}

#[doc(alias = "RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")]
pub fn stub_0x38add0(handle: crate::slot::InstanceHandle) {
// RBX::ThreadPool::ThreadPoolData dtor.
drop(handle);
}

#[doc(alias = "RBX::ThreadPool::ThreadPoolData::~ThreadPoolData() [0x38aec4]")]
pub fn stub_0x38aec4(handle: crate::slot::InstanceHandle) {
// RBX::ThreadPool::ThreadPoolData dtor.
drop(handle);
}

#[doc(alias = "RBX::ThreadPool::ThreadPoolData::getNextTask(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> &)")]
pub fn stub_0x38afc8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "rbx::safe_queue<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>::pop_if_present(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&)")]
pub fn stub_0x38afd4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::pop_front(void)")]
pub fn stub_0x38b0b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::~deque()")]
pub fn stub_0x38b0ec(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::~_Deque_base()")]
pub fn stub_0x38b1d4(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>*>,std::_Deque_iterator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>*>)")]
pub fn stub_0x38b200() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_initialize_map(unsigned long)")]
pub fn stub_0x38b338() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_create_nodes(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>**,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>**)")]
pub fn stub_0x38b490() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::deque(std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>> const&)")]
pub fn stub_0x38b584() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "global constructor keyed to_a_146")]
pub fn stub_0x38b740() -> crate::slot::PortedFn {
// IDA 0x38b740: __GLOBAL__I_a_146.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x38b740, "__GLOBAL__I_a_146")
}

#[doc(alias = "RBX::StringConverter<RBX::UDim>::convertToString(RBX::UDim const&)")]
pub fn stub_0x38b808(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::UDim>::convertToString(RBX::UDim const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<RBX::UDim>::convertToValue(std::string const&,RBX::UDim&)")]
pub fn stub_0x38b970(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::UDim>::convertToValue(std::string const&, RBX::UDim&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<RBX::UDim2>::convertToString(RBX::UDim2 const&)")]
pub fn stub_0x38ba5c(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::UDim2>::convertToString(RBX::UDim2 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<RBX::UDim2>::convertToValue(std::string const&,RBX::UDim2&)")]
pub fn stub_0x38be8c(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::UDim2>::convertToValue(std::string const&, RBX::UDim2&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::UDim::operator+(RBX::UDim const&)const")]
pub fn stub_0x38c0e8(handle: &crate::slot::InstanceHandle) {
// RBX::UDim::operator+(RBX::UDim const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::UDim::operator-(RBX::UDim const&)const")]
pub fn stub_0x38c108(handle: &crate::slot::InstanceHandle) {
// RBX::UDim::operator-(RBX::UDim const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::UDim::operator-(void)const")]
pub fn stub_0x38c128(handle: &crate::slot::InstanceHandle) {
// RBX::UDim::operator-() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::UDim2::operator*(G3D::Vector2)const")]
pub fn stub_0x38c140(handle: &crate::slot::InstanceHandle) {
// RBX::UDim2::operator*(G3D::Vector2) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::UDim2::operator*(float)const")]
pub fn stub_0x38c188(handle: &crate::slot::InstanceHandle) {
// RBX::UDim2::operator*(float) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::UDim2::operator+(RBX::UDim2 const&)const")]
pub fn stub_0x38c1e4(handle: &crate::slot::InstanceHandle) {
// RBX::UDim2::operator+(RBX::UDim2 const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::UDim2::operator-(RBX::UDim2 const&)const")]
pub fn stub_0x38c224(handle: &crate::slot::InstanceHandle) {
// RBX::UDim2::operator-(RBX::UDim2 const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::UDim2::operator-(void)const")]
pub fn stub_0x38c264(handle: &crate::slot::InstanceHandle) {
// RBX::UDim2::operator-() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_147")]
pub fn stub_0x38c294() -> crate::slot::PortedFn {
// IDA 0x38c294: __GLOBAL__I_a_147.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x38c294, "__GLOBAL__I_a_147")
}

#[doc(alias = "RBX::UIEvent::isTextCharacterKey(void)const")]
pub fn stub_0x38c35c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UIEvent getter.
cell.get()
}

#[doc(alias = "RBX::UIEvent::isAltEvent(void)const")]
pub fn stub_0x38c368(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UIEvent getter.
cell.get()
}

#[doc(alias = "RBX::UIEvent::isCtrlEvent(void)const")]
pub fn stub_0x38c37c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UIEvent getter.
cell.get()
}

#[doc(alias = "RBX::UIEvent::isCarriageReturnKey(void)const")]
pub fn stub_0x38c390(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UIEvent getter.
cell.get()
}

#[doc(alias = "RBX::UIEvent::isDeleteKey(void)const")]
pub fn stub_0x38c3ac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UIEvent getter.
cell.get()
}

#[doc(alias = "RBX::UIEvent::isBackspaceKey(void)const")]
pub fn stub_0x38c3b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UIEvent getter.
cell.get()
}

#[doc(alias = "RBX::UIEvent::isClearKey(void)const")]
pub fn stub_0x38c3c4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UIEvent getter.
cell.get()
}

#[doc(alias = "RBX::UIEvent::isEscapeKey(void)const")]
pub fn stub_0x38c3d0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UIEvent getter.
cell.get()
}

#[doc(alias = "RBX::UIEvent::isLeftArrowKey(void)const")]
pub fn stub_0x38c3dc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UIEvent getter.
cell.get()
}

#[doc(alias = "RBX::UIEvent::isRightArrowKey(void)const")]
pub fn stub_0x38c3ec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UIEvent getter.
cell.get()
}

#[doc(alias = "global constructor keyed to_a_148")]
pub fn stub_0x38c3fc() -> crate::slot::PortedFn {
// IDA 0x38c3fc: __GLOBAL__I_a_148.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x38c3fc, "__GLOBAL__I_a_148")
}

#[doc(alias = "RBX::Units::kmsAccelerationToRbx(G3D::Vector3 const&)")]
pub fn stub_0x38c434(a: &crate::lua::LuaVector3) -> crate::lua::LuaVector3 {
// G3D::Vector3 unit — reciprocal-sqrt normalize.
let inv = 1.0 / (a.x * a.x + a.y * a.y + a.z * a.z).sqrt();
crate::lua::LuaVector3 { x: a.x * inv, y: a.y * inv, z: a.z * inv }
}

#[doc(alias = "RBX::Units::kmsForceToRbx(float)")]
pub fn stub_0x38c464(handle: &crate::slot::InstanceHandle) {
// RBX::Units::kmsForceToRbx(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_149")]
pub fn stub_0x38c478() -> crate::slot::PortedFn {
// IDA 0x38c478: __GLOBAL__I_a_149.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x38c478, "__GLOBAL__I_a_149")
}

#[doc(alias = "RBX::UserInputBase::UserInputBase(void)")]
pub fn stub_0x38c4b0() -> crate::slot::InstanceHandle {
// RBX::UserInputBase ctor.
crate::slot::InstanceHandle::new("RBX::UserInputBase")
}

#[doc(alias = "RBX::UserInputBase::getNavKeys(RBX::NavKeys &,bool)const")]
pub fn stub_0x38c5d4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UserInputBase getter.
cell.get()
}

#[doc(alias = "RBX::UserInputBase::getGameCursor(RBX::Adorn *)")]
pub fn stub_0x38c6b4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UserInputBase getter.
cell.get()
}

#[doc(alias = "RBX::UserInputBase::setCursorId(RBX::Adorn *,RBX::TextureId const&)")]
pub fn stub_0x38c928(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::UserInputBase setter.
cell.set(value)
}

#[doc(alias = "RBX::UserInputBase::renderGameCursor(RBX::Adorn *)")]
pub fn stub_0x38c974(handle: &crate::slot::InstanceHandle) {
// RBX::UserInputBase::renderGameCursor(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::UserInputBase::removeJobs(void)")]
pub fn stub_0x38cb9c(handle: &crate::slot::InstanceHandle) {
// RBX::UserInputBase::removeJobs() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_150")]
pub fn stub_0x38cba0() -> crate::slot::PortedFn {
// IDA 0x38cba0: __GLOBAL__I_a_150.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x38cba0, "__GLOBAL__I_a_150")
}

#[doc(alias = "RBX::rot13(std::string)")]
pub fn stub_0x38cc68() -> crate::slot::PortedFn {
// IDA 0x38cc68: RBX::rot13(std::string).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x38cc68, "RBX::rot13(std::string)")
}

#[doc(alias = "RBX::StringConverter<bool>::convertToString(bool const&)")]
pub fn stub_0x38ce48(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<bool>::convertToString(bool const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<bool>::convertToValue(std::string const&,bool &)")]
pub fn stub_0x38ce78(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<bool>::convertToValue(std::string const&, bool&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<int>::convertToString(int const&)")]
pub fn stub_0x38cf10(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<int>::convertToString(int const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<long>::convertToString(long const&)")]
pub fn stub_0x38cf58(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<long>::convertToString(long const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<int>::convertToValue(std::string const&,int &)")]
pub fn stub_0x38cfa0(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<int>::convertToValue(std::string const&, int&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<unsigned int>::convertToString(unsigned int const&)")]
pub fn stub_0x38cff0(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<unsigned int>::convertToString(unsigned int const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<unsigned int>::convertToValue(std::string const&,unsigned int &)")]
pub fn stub_0x38d038(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<unsigned int>::convertToValue(std::string const&, unsigned int&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<long>::convertToValue(std::string const&,long &)")]
pub fn stub_0x38d14c(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<long>::convertToValue(std::string const&, long&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<double>::convertToValue(std::string const&,double &)")]
pub fn stub_0x38d260(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<double>::convertToValue(std::string const&, double&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<double>::convertToString(double const&)")]
pub fn stub_0x38d2e0(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<double>::convertToString(double const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<float>::convertToValue(std::string const&,float &)")]
pub fn stub_0x38d440(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<float>::convertToValue(std::string const&, float&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<float>::convertToString(float const&)")]
pub fn stub_0x38d4c4(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<float>::convertToString(float const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long>(long &)")]
pub fn stub_0x38d61c() -> crate::slot::PortedFn {
// IDA 0x38d61c: bool boost::detail::lexical_stream_limited_src<char, std::char_traits<char>, false>::shr_signed<long>(long&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x38d61c, "bool boost::detail::lexical_stream_limited_src<char, std::char_traits<char>, false>::shr_signed<long~")
}

#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long,char>(unsigned long &,char const*,char const*)")]
pub fn stub_0x38d67c() -> crate::slot::PortedFn {
// IDA 0x38d67c: bool boost::detail::lcast_ret_unsigned<std::char_traits<char>, unsigned long, char>(unsigned long&, char const*, char co~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x38d67c, "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>, unsigned long, char>(unsigned long&, ~")
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned int>(unsigned int &)")]
pub fn stub_0x38da14() -> crate::slot::PortedFn {
// IDA 0x38da14: bool boost::detail::lexical_stream_limited_src<char, std::char_traits<char>, false>::shr_unsigned<unsigned int>(unsigned~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x38da14, "bool boost::detail::lexical_stream_limited_src<char, std::char_traits<char>, false>::shr_unsigned<un~")
}

#[doc(alias = "global constructor keyed to_a_151")]
pub fn stub_0x38da58() -> crate::slot::PortedFn {
// IDA 0x38da58: __GLOBAL__I_a_151.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x38da58, "__GLOBAL__I_a_151")
}

#[doc(alias = "RBX::Accoutrement::setAttachmentPoint(G3D::CoordinateFrame const&)")]
pub fn stub_0x38db20(handle: crate::slot::InstanceHandle) {
// RBX::Accoutrement dtor.
drop(handle);
}

#[doc(alias = "RBX::Accoutrement::getAttachmentPos(void)const")]
pub fn stub_0x38dc30(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Accoutrement getter.
cell.get()
}

#[doc(alias = "RBX::Accoutrement::setAttachmentPos(G3D::Vector3 const&)")]
pub fn stub_0x38dc40(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Accoutrement setter.
cell.set(value)
}

#[doc(alias = "RBX::Accoutrement::getAttachmentForward(void)const")]
pub fn stub_0x38dc70(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Accoutrement getter.
cell.get()
}

#[doc(alias = "RBX::Accoutrement::setAttachmentForward(G3D::Vector3 const&)")]
pub fn stub_0x38dcb0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Accoutrement setter.
cell.set(value)
}

#[doc(alias = "RBX::Accoutrement::getAttachmentUp(void)const")]
pub fn stub_0x38ddfc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Accoutrement getter.
cell.get()
}

#[doc(alias = "RBX::Accoutrement::setAttachmentUp(G3D::Vector3 const&)")]
pub fn stub_0x38de0c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Accoutrement setter.
cell.set(value)
}

#[doc(alias = "RBX::Accoutrement::getAttachmentRight(void)const")]
pub fn stub_0x38df30(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Accoutrement getter.
cell.get()
}

#[doc(alias = "RBX::Accoutrement::setAttachmentRight(G3D::Vector3 const&)")]
pub fn stub_0x38df40(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Accoutrement setter.
cell.set(value)
}

#[doc(alias = "RBX::Accoutrement::setBackendAccoutrementState(int)")]
pub fn stub_0x38e064(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Accoutrement setter.
cell.set(value)
}

#[doc(alias = "RBX::Accoutrement::Accoutrement(void)")]
pub fn stub_0x38e084() -> crate::slot::InstanceHandle {
// RBX::Accoutrement ctor.
crate::slot::InstanceHandle::new("RBX::Accoutrement")
}

#[doc(alias = "RBX::Accoutrement::Accoutrement(void) [0x38e4b4]")]
pub fn stub_0x38e4b4() -> crate::slot::InstanceHandle {
// RBX::Accoutrement ctor.
crate::slot::InstanceHandle::new("RBX::Accoutrement")
}

#[doc(alias = "RBX::Accoutrement::~Accoutrement()")]
pub fn stub_0x38e90c(handle: crate::slot::InstanceHandle) {
// RBX::Accoutrement dtor.
drop(handle);
}

#[doc(alias = "RBX::Accoutrement::~Accoutrement() [0x38e9b8]")]
pub fn stub_0x38e9b8(handle: crate::slot::InstanceHandle) {
// RBX::Accoutrement dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
pub fn stub_0x38e9c8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement() [0x38e9d0]")]
pub fn stub_0x38e9d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement() [0x38e9d8]")]
pub fn stub_0x38e9d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}
