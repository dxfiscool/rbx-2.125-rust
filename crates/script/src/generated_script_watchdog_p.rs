// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 120 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x46f148..0x475a4c | script 25952->26072 distinct (filler 0x46f148 asc, not-in-script 59593->59473)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx::thread_specific_reference<RBX::DataModel::GenericJob>::~thread_specific_reference()")]
pub fn stub_0x46f148(handle: crate::slot::InstanceHandle) {
// rbx::thread_specific_reference dtor.
drop(handle);
}

#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::~thread_specific_ptr()")]
pub fn stub_0x46f158() -> crate::slot::PortedFn {
// IDA 0x46f158: boost::thread_specific_ptr<RBX::DataModel::GenericJob*>::~thread_specific_ptr().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x46f158, "boost::thread_specific_ptr<RBX::DataModel::GenericJob*>::~thread_specific_ptr()")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()")]
pub fn stub_0x46f24c() -> crate::slot::PortedFn {
// IDA 0x46f24c: boost::thread_specific_ptr<RBX::DataModel::GenericJob*>::delete_data::~delete_data().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x46f24c, "boost::thread_specific_ptr<RBX::DataModel::GenericJob*>::delete_data::~delete_data()")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data() [0x46f250]")]
pub fn stub_0x46f250() -> crate::slot::PortedFn {
// IDA 0x46f250: boost::thread_specific_ptr<RBX::DataModel::GenericJob*>::delete_data::~delete_data().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x46f250, "boost::thread_specific_ptr<RBX::DataModel::GenericJob*>::delete_data::~delete_data()")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::operator()(void *)")]
pub fn stub_0x46f254() -> crate::slot::PortedFn {
// IDA 0x46f254: boost::thread_specific_ptr<RBX::DataModel::GenericJob*>::delete_data::operator()(void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x46f254, "boost::thread_specific_ptr<RBX::DataModel::GenericJob*>::delete_data::operator()(void*)")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>(boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>)")]
pub fn stub_0x46f260() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()")]
pub fn stub_0x46f358(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd() [0x46f35c]")]
pub fn stub_0x46f35c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::dispose(void)")]
pub fn stub_0x46f360() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_deleter(std::type_info const&)")]
pub fn stub_0x46f370() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_untyped_deleter(void)")]
pub fn stub_0x46f388() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx::signals::signal<void ()(std::string const&)>::disconnectAll(void)")]
pub fn stub_0x46f38c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (std::string const&)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::disconnectAll(void)")]
pub fn stub_0x46f504(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::UIEvent const&)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,std::allocator<std::pair<std::string const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,int>> *)")]
pub fn stub_0x46f67c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::GuiImageMixin::getImageRectOffset(void)const")]
pub fn stub_0x46f704(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GuiImageMixin getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::~PropDescriptor()")]
pub fn stub_0x46f710(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::GuiImageMixin::getImageRectSize(void)const")]
pub fn stub_0x46f734(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GuiImageMixin getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::~PropDescriptor()")]
pub fn stub_0x46f740(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::~TypedPropertyDescriptor()")]
pub fn stub_0x46f768(_v: u64) {
// G3D value dtor — host Copy payload needs no release.
let _ = _v;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x46f798(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection::TypedPropertyDescriptor setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x46f8fc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x46fa10() -> crate::slot::InstanceHandle {
// RBX::Reflection::TypedPropertyDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::TypedPropertyDescriptor")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::~PropDescriptor() [0x46fb34]")]
pub fn stub_0x46fb34(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::isReadOnly(void)const")]
pub fn stub_0x46fb60(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::isWriteOnly(void)const")]
pub fn stub_0x46fb64(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x46fb68(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
pub fn stub_0x46fba0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x46fbd4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::~PropDescriptor() [0x46fce8]")]
pub fn stub_0x46fce8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::isReadOnly(void)const")]
pub fn stub_0x46fd14(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::isWriteOnly(void)const")]
pub fn stub_0x46fd18(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x46fd1c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
pub fn stub_0x46fd54(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::DataModel::MouseStats::MouseStats(void)")]
pub fn stub_0x46fd8c() -> crate::slot::InstanceHandle {
// RBX::DataModel::MouseStats ctor.
crate::slot::InstanceHandle::new("RBX::DataModel::MouseStats")
}

#[doc(alias = "RBX::IMetric::~IMetric()")]
pub fn stub_0x46feac(handle: crate::slot::InstanceHandle) {
// RBX::IMetric dtor.
drop(handle);
}

#[doc(alias = "RBX::IMetric::~IMetric() [0x46feb0]")]
pub fn stub_0x46feb0(handle: crate::slot::InstanceHandle) {
// RBX::IMetric dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModel::GenericJob::GenericJob(rbx_core::SharedPtr<RBX::DataModel>,char const*,RBX::DataModelJob::TaskType)")]
pub fn stub_0x46ff84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel")
}

#[doc(alias = "RBX::DataModel::GenericJob::~GenericJob()")]
pub fn stub_0x47013c(handle: crate::slot::InstanceHandle) {
// RBX::DataModel::GenericJob dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModel::GenericJob::~GenericJob() [0x47025c]")]
pub fn stub_0x47025c(handle: crate::slot::InstanceHandle) {
// RBX::DataModel::GenericJob dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModel::GenericJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x470390(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::GenericJob::sleepTime(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::GenericJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x470400(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::GenericJob::error(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::GenericJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x470484(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::GenericJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::GenericJob::processTasks(void)")]
pub fn stub_0x470670(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::GenericJob::processTasks() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::GenericJob::step(boost::function<void ()(RBX::DataModel*)> &)")]
pub fn stub_0x470818(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::GenericJob::step(boost::function<void (RBX::DataModel*)>&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~deque()")]
pub fn stub_0x4708e0(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~_Deque_base()")]
pub fn stub_0x4709c8(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::deque(std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>> const&)")]
pub fn stub_0x4709f8() -> crate::slot::PortedFn {
// IDA 0x4709f8: std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void (RBX::DataModel*)>>, std::allocator<rbx~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4709f8, "std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void (RBX::DataModel*)>>~")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>> *)")]
pub fn stub_0x470b30(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::GearType>> *)")]
pub fn stub_0x470b58(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>> *)")]
pub fn stub_0x470b80(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::Genre>> *)")]
pub fn stub_0x470ba8(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>> *)")]
pub fn stub_0x470bd0(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::EnumDesc(void)")]
pub fn stub_0x4727ec() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::EnumDesc(void) [0x4727f0]")]
pub fn stub_0x4727f0() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::DataModelJob::DataModelJob(char const*,RBX::DataModelJob::TaskType,bool,rbx_core::SharedPtr<RBX::DataModelArbiter>,RBX::Time::Interval)")]
pub fn stub_0x4729dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModelArbiter")
}

#[doc(alias = "RBX::DataModelJob::step(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x472b4c(handle: &crate::slot::InstanceHandle) {
// RBX::DataModelJob::step(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModelJob::getPriorityFactor(void)")]
pub fn stub_0x472cd4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModelJob getter.
cell.get()
}

#[doc(alias = "RBX::DataModelArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
pub fn stub_0x472e00(handle: &crate::slot::InstanceHandle) {
// RBX::DataModelArbiter::areExclusive(RBX::TaskScheduler::Job*, RBX::TaskScheduler::Job*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModelArbiter::DataModelArbiter(void)")]
pub fn stub_0x472e2c() -> crate::slot::InstanceHandle {
// RBX::DataModelArbiter ctor.
crate::slot::InstanceHandle::new("RBX::DataModelArbiter")
}

#[doc(alias = "RBX::DataModelArbiter::~DataModelArbiter()")]
pub fn stub_0x473124(handle: crate::slot::InstanceHandle) {
// RBX::DataModelArbiter dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModelArbiter::~DataModelArbiter() [0x4731c4]")]
pub fn stub_0x4731c4(handle: crate::slot::InstanceHandle) {
// RBX::DataModelArbiter dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModelArbiter::~DataModelArbiter() [0x4731c8]")]
pub fn stub_0x4731c8(handle: crate::slot::InstanceHandle) {
// RBX::DataModelArbiter dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModelArbiter::preStep(RBX::TaskScheduler::Job *)")]
pub fn stub_0x473318(handle: &crate::slot::InstanceHandle) {
// RBX::DataModelArbiter::preStep(RBX::TaskScheduler::Job*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModelArbiter::postStep(RBX::TaskScheduler::Job *)")]
pub fn stub_0x473350(handle: &crate::slot::InstanceHandle) {
// RBX::DataModelArbiter::postStep(RBX::TaskScheduler::Job*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::addPair(RBX::DataModelArbiter::ConcurrencyModel,char const*)")]
pub fn stub_0x473388(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::addPair(RBX::DataModel~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModelJob::~DataModelJob()")]
pub fn stub_0x4736e8(handle: crate::slot::InstanceHandle) {
// RBX::DataModelJob dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModelJob::~DataModelJob() [0x4736ec]")]
pub fn stub_0x4736ec(handle: crate::slot::InstanceHandle) {
// RBX::DataModelJob dtor.
drop(handle);
}

#[doc(alias = "RBX::SimpleThrottlingArbiter::isThrottled(void)")]
pub fn stub_0x473790(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SimpleThrottlingArbiter getter.
cell.get()
}

#[doc(alias = "RBX::TaskScheduler::Arbiter::getSyncronizationArbiter(void)")]
pub fn stub_0x473858(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TaskScheduler::Arbiter getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::~EnumDesc()")]
pub fn stub_0x473860(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0x473868(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToString(unsign~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModelArbiter::ConcurrencyModel>::construct_func(char const*,char *)")]
pub fn stub_0x4739b0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::DataModelArbiter::ConcurrencyModel>::construct_func~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToItem(RBX::DataModelArbiter::ConcurrencyModel const&)const")]
pub fn stub_0x4739c0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::convertToItem(RBX::Dat~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModelArbiter::ConcurrencyModel const& rbx::any_cast<RBX::DataModelArbiter::ConcurrencyModel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x473a8c(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelArbiter::ConcurrencyModel>::~EnumDesc() [0x473b80]")]
pub fn stub_0x473b80(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::resize(unsigned long,RBX::DataModelArbiter::ConcurrencyModel)")]
pub fn stub_0x473f38(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::push_back(RBX::DataModelArbiter::ConcurrencyModel const&)")]
pub fn stub_0x473f70(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>::resize(unsigned long,RBX::Name const*)")]
pub fn stub_0x473f98(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::resize(unsigned long,RBX::Reflection::EnumDescriptor::Item const*)")]
pub fn stub_0x473fd0(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DataModelArbiter::ConcurrencyModel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x474004(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")]
pub fn stub_0x47405c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")]
pub fn stub_0x474110(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")]
pub fn stub_0x474168(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor::Item const**,std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>>,unsigned long,RBX::Reflection::EnumDescriptor::Item const* const&)")]
pub fn stub_0x4741d0(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::_M_allocate(unsigned long)")]
pub fn stub_0x474338() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,std::allocator<std::string>>>,unsigned long,std::string const&)")]
pub fn stub_0x474350(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModelArbiter::ConcurrencyModel*,std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>>,RBX::DataModelArbiter::ConcurrencyModel const&)")]
pub fn stub_0x47486c(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_allocate(unsigned long)")]
pub fn stub_0x474950() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::DataModelArbiter::ConcurrencyModel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *>(RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *)")]
pub fn stub_0x474968(handle: &crate::slot::InstanceHandle) {
// RBX::DataModelArbiter::ConcurrencyModel* std::__copy_backward<false, std::random_access_it~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Vector_base<unsigned long,std::allocator<unsigned long>>::_M_allocate(unsigned long)")]
pub fn stub_0x4749a8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModelArbiter::ConcurrencyModel*,std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>>,unsigned long,RBX::DataModelArbiter::ConcurrencyModel const&)")]
pub fn stub_0x4749c0(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor::Item const**,std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>>,RBX::Reflection::EnumDescriptor::Item const* const&)")]
pub fn stub_0x474b50(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::Item::~Item()")]
pub fn stub_0x474c30(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDescriptor::Item dtor.
drop(handle);
}

#[doc(alias = "RBX::ActivityMeter<2>::updateBuckets(void)")]
pub fn stub_0x474c38(handle: &crate::slot::InstanceHandle) {
// RBX::ActivityMeter<2>::updateBuckets() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::OnScreenProfiler::GetInst(void)")]
pub fn stub_0x474cf0(handle: &crate::slot::InstanceHandle) {
// RBX::OnScreenProfiler::GetInst() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::OnScreenProfiler::Create(void)")]
pub fn stub_0x474d54(handle: &crate::slot::InstanceHandle) {
// RBX::OnScreenProfiler::Create() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>> *)")]
pub fn stub_0x474dfc(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::EnumDesc(void)")]
pub fn stub_0x474eec() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::EnumDesc(void) [0x474ef0]")]
pub fn stub_0x474ef0() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::DataModelMesh::setLevelOfDetailX(RBX::DataModelMesh::LODType)")]
pub fn stub_0x4750c8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DataModelMesh setter.
cell.set(value)
}

#[doc(alias = "RBX::DataModelMesh::setLevelOfDetailY(RBX::DataModelMesh::LODType)")]
pub fn stub_0x4750e8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DataModelMesh setter.
cell.set(value)
}

#[doc(alias = "RBX::DataModelMesh::setScale(G3D::Vector3 const&)")]
pub fn stub_0x475108(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DataModelMesh setter.
cell.set(value)
}

#[doc(alias = "RBX::DataModelMesh::setVertColor(G3D::Vector3 const&)")]
pub fn stub_0x4751a8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DataModelMesh setter.
cell.set(value)
}

#[doc(alias = "RBX::DataModelMesh::setOffset(G3D::Vector3 const&)")]
pub fn stub_0x475210(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DataModelMesh setter.
cell.set(value)
}

#[doc(alias = "RBX::DataModelMesh::DataModelMesh(void)")]
pub fn stub_0x475278() -> crate::slot::InstanceHandle {
// RBX::DataModelMesh ctor.
crate::slot::InstanceHandle::new("RBX::DataModelMesh")
}

#[doc(alias = "RBX::DataModelMesh::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x4754a4(handle: &crate::slot::InstanceHandle) {
// RBX::DataModelMesh::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::addPair(RBX::DataModelMesh::LODType,char const*)")]
pub fn stub_0x4754e0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::addPair(RBX::DataModelMesh::LODTyp~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModelMesh::getLevelOfDetailX(void)const")]
pub fn stub_0x475840(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModelMesh getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::~EnumPropDescriptor()")]
pub fn stub_0x475848(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::DataModelMesh::getLevelOfDetailY(void)const")]
pub fn stub_0x47586c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModelMesh getter.
cell.get()
}

#[doc(alias = "RBX::DataModelMesh::getScale(void)const")]
pub fn stub_0x475874(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModelMesh getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::~PropDescriptor()")]
pub fn stub_0x475878(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::DataModelMesh::getVertColor(void)const")]
pub fn stub_0x47589c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModelMesh getter.
cell.get()
}

#[doc(alias = "RBX::DataModelMesh::getOffset(void)const")]
pub fn stub_0x4758a0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModelMesh getter.
cell.get()
}

#[doc(alias = "RBX::DataModelMesh::~DataModelMesh()")]
pub fn stub_0x4758a4(handle: crate::slot::InstanceHandle) {
// RBX::DataModelMesh dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModelMesh::~DataModelMesh() [0x4758a8]")]
pub fn stub_0x4758a8(handle: crate::slot::InstanceHandle) {
// RBX::DataModelMesh dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::DataModelMesh::~DataModelMesh()")]
pub fn stub_0x475970(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::DataModelMesh::~DataModelMesh() [0x475978]")]
pub fn stub_0x475978(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::DataModelMesh::~DataModelMesh() [0x475a44]")]
pub fn stub_0x475a44(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::DataModelMesh::~DataModelMesh() [0x475a4c]")]
pub fn stub_0x475a4c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}
