//! Auto-generated skeletons for rbx-network — RakNet + RBX::Network (auto-generated, do not edit manually)
//! Filter: RakNet|RBX::Network (case-insensitive) -> 4479 funcs, 1740 already stubbed (2739 remaining before batch); EA-sorted ascending earliest gap.
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x96025c..0x97133c | existing 12290 -> 12390 total (pool 1740->1840, remaining 2639, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x96025c — __ZN3RBX7Network11deserializeINS_9ContentIdEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX7Network11deserializeINS_9ContentIdEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE")]
#[doc(alias = "void RBX::Network::deserialize<RBX::ContentId>(RBX::Reflection::Property &,RakNet::BitStream &)")]
pub fn stub_96025c() -> ! {
    todo!("0x96025c void RBX::Network::deserialize<RBX::ContentId>(RBX::Reflection::Property &,RakNet::BitStream &)")
}

// 0x960380 — __ZN3RBX7Network25deserializeStringPropertyERNS_10Reflection8PropertyERN6RakNet9BitStreamE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX7Network25deserializeStringPropertyERNS_10Reflection8PropertyERN6RakNet9BitStreamE")]
#[doc(alias = "RBX::Network::deserializeStringProperty(RBX::Reflection::Property &,RakNet::BitStream &)")]
pub fn stub_960380() -> ! {
    todo!("0x960380 RBX::Network::deserializeStringProperty(RBX::Reflection::Property &,RakNet::BitStream &)")
}

// 0x9604a4 — __ZN3RBX7Network12IdSerializerC2Ev
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this)
#[doc(alias = "__ZN3RBX7Network12IdSerializerC2Ev")]
#[doc(alias = "RBX::Network::IdSerializer::IdSerializer(void)")]
pub fn stub_9604a4() -> ! {
    todo!("0x9604a4 RBX::Network::IdSerializer::IdSerializer(void)")
}

// 0x960624 — __ZN3RBX7Network12IdSerializer18setMaxGuidIndexBitEi
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this, int)
#[doc(alias = "__ZN3RBX7Network12IdSerializer18setMaxGuidIndexBitEi")]
#[doc(alias = "RBX::Network::IdSerializer::setMaxGuidIndexBit(int)")]
pub fn stub_960624() -> ! {
    todo!("0x960624 RBX::Network::IdSerializer::setMaxGuidIndexBit(int)")
}

// 0x960634 — __ZN3RBX7Network12IdSerializer14trySerializeIdERN6RakNet9BitStreamEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this, RakNet::BitStream *, const RBX::Instance *)
#[doc(alias = "__ZN3RBX7Network12IdSerializer14trySerializeIdERN6RakNet9BitStreamEPKNS_8InstanceE")]
#[doc(alias = "RBX::Network::IdSerializer::trySerializeId(RakNet::BitStream &,RBX::Instance const*)")]
pub fn stub_960634() -> ! {
    todo!("0x960634 RBX::Network::IdSerializer::trySerializeId(RakNet::BitStream &,RBX::Instance const*)")
}

// 0x96068c — __ZN3RBX7Network12IdSerializer11serializeIdERN6RakNet9BitStreamEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this, RakNet::BitStream *, const RBX::Instance *)
#[doc(alias = "__ZN3RBX7Network12IdSerializer11serializeIdERN6RakNet9BitStreamEPKNS_8InstanceE")]
#[doc(alias = "RBX::Network::IdSerializer::serializeId(RakNet::BitStream &,RBX::Instance const*)")]
pub fn stub_96068c() -> ! {
    todo!("0x96068c RBX::Network::IdSerializer::serializeId(RakNet::BitStream &,RBX::Instance const*)")
}

// 0x9606d8 — __ZN3RBX7Network12IdSerializer14canSerializeIdEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZN3RBX7Network12IdSerializer14canSerializeIdEPKNS_8InstanceE")]
#[doc(alias = "RBX::Network::IdSerializer::canSerializeId(RBX::Instance const*)")]
pub fn stub_9606d8() -> ! {
    todo!("0x9606d8 RBX::Network::IdSerializer::canSerializeId(RBX::Instance const*)")
}

// 0x96075c — __ZN3RBX7Network12IdSerializer17onServiceProviderEPNS_15ServiceProviderES3_
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "__ZN3RBX7Network12IdSerializer17onServiceProviderEPNS_15ServiceProviderES3_")]
#[doc(alias = "RBX::Network::IdSerializer::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_96075c() -> ! {
    todo!("0x96075c RBX::Network::IdSerializer::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x960784 — __ZN3RBX7Network12IdSerializer9extractIdEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZN3RBX7Network12IdSerializer9extractIdEPKNS_8InstanceE")]
#[doc(alias = "RBX::Network::IdSerializer::extractId(RBX::Instance const*)")]
pub fn stub_960784() -> ! {
    todo!("0x960784 RBX::Network::IdSerializer::extractId(RBX::Instance const*)")
}

// 0x9607ac — __ZN3RBX7Network12IdSerializer6sendIdERN6RakNet9BitStreamERKNS1_2IdE
// type: int __fastcall(int, RakNet::BitStream *this)
#[doc(alias = "__ZN3RBX7Network12IdSerializer6sendIdERN6RakNet9BitStreamERKNS1_2IdE")]
#[doc(alias = "RBX::Network::IdSerializer::sendId(RakNet::BitStream &,RBX::Network::IdSerializer::Id const&)")]
pub fn stub_9607ac() -> ! {
    todo!("0x9607ac RBX::Network::IdSerializer::sendId(RakNet::BitStream &,RBX::Network::IdSerializer::Id const&)")
}

// 0x9607ec — __ZN3RBX7Network12IdSerializer11serializeIdERN6RakNet9BitStreamERKNS_4Guid4DataE
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this, RakNet::BitStream *, const RBX::Guid::Data *)
#[doc(alias = "__ZN3RBX7Network12IdSerializer11serializeIdERN6RakNet9BitStreamERKNS_4Guid4DataE")]
#[doc(alias = "RBX::Network::IdSerializer::serializeId(RakNet::BitStream &,RBX::Guid::Data const&)")]
pub fn stub_9607ec() -> ! {
    todo!("0x9607ec RBX::Network::IdSerializer::serializeId(RakNet::BitStream &,RBX::Guid::Data const&)")
}

// 0x960814 — __ZN3RBX7Network12IdSerializer28serializeIdWithoutDictionaryERN6RakNet9BitStreamEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this, RakNet::BitStream *, const RBX::Instance *)
#[doc(alias = "__ZN3RBX7Network12IdSerializer28serializeIdWithoutDictionaryERN6RakNet9BitStreamEPKNS_8InstanceE")]
#[doc(alias = "RBX::Network::IdSerializer::serializeIdWithoutDictionary(RakNet::BitStream &,RBX::Instance const*)")]
pub fn stub_960814() -> ! {
    todo!("0x960814 RBX::Network::IdSerializer::serializeIdWithoutDictionary(RakNet::BitStream &,RBX::Instance const*)")
}

// 0x960a20 — __ZN3RBX7Network12IdSerializer13deserializeIdERN6RakNet9BitStreamERNS_4Guid4DataE
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this, RakNet::BitStream *, RBX::Guid::Data *)
#[doc(alias = "__ZN3RBX7Network12IdSerializer13deserializeIdERN6RakNet9BitStreamERNS_4Guid4DataE")]
#[doc(alias = "RBX::Network::IdSerializer::deserializeId(RakNet::BitStream &,RBX::Guid::Data &)")]
pub fn stub_960a20() -> ! {
    todo!("0x960a20 RBX::Network::IdSerializer::deserializeId(RakNet::BitStream &,RBX::Guid::Data &)")
}

// 0x960c8c — __ZN3RBX7Network12IdSerializer30deserializeIdWithoutDictionaryERN6RakNet9BitStreamERNS_4Guid4DataE
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this, RakNet::BitStream *, RBX::Guid::Data *)
#[doc(alias = "__ZN3RBX7Network12IdSerializer30deserializeIdWithoutDictionaryERN6RakNet9BitStreamERNS_4Guid4DataE")]
#[doc(alias = "RBX::Network::IdSerializer::deserializeIdWithoutDictionary(RakNet::BitStream &,RBX::Guid::Data &)")]
pub fn stub_960c8c() -> ! {
    todo!("0x960c8c RBX::Network::IdSerializer::deserializeIdWithoutDictionary(RakNet::BitStream &,RBX::Guid::Data &)")
}

// 0x960f0c — __ZN3RBX7Network12IdSerializer11setRefValueERNS1_8WaitItemEPNS_8InstanceE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZN3RBX7Network12IdSerializer11setRefValueERNS1_8WaitItemEPNS_8InstanceE")]
#[doc(alias = "RBX::Network::IdSerializer::setRefValue(RBX::Network::IdSerializer::WaitItem &,RBX::Instance *)")]
pub fn stub_960f0c() -> ! {
    todo!("0x960f0c RBX::Network::IdSerializer::setRefValue(RBX::Network::IdSerializer::WaitItem &,RBX::Instance *)")
}

// 0x960f28 — __ZN3RBX7Network12IdSerializer24resolvePendingReferencesEPNS_8InstanceENS_4Guid4DataE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZN3RBX7Network12IdSerializer24resolvePendingReferencesEPNS_8InstanceENS_4Guid4DataE")]
#[doc(alias = "RBX::Network::IdSerializer::resolvePendingReferences(RBX::Instance *,RBX::Guid::Data)")]
pub fn stub_960f28() -> ! {
    todo!("0x960f28 RBX::Network::IdSerializer::resolvePendingReferences(RBX::Instance *,RBX::Guid::Data)")
}

// 0x961094 — __ZN3RBX7Network12IdSerializer20serializeInstanceRefEPKNS_8InstanceERN6RakNet9BitStreamE
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this, const RBX::Instance *, RakNet::BitStream *)
#[doc(alias = "__ZN3RBX7Network12IdSerializer20serializeInstanceRefEPKNS_8InstanceERN6RakNet9BitStreamE")]
#[doc(alias = "RBX::Network::IdSerializer::serializeInstanceRef(RBX::Instance const*,RakNet::BitStream &)")]
pub fn stub_961094() -> ! {
    todo!("0x961094 RBX::Network::IdSerializer::serializeInstanceRef(RBX::Instance const*,RakNet::BitStream &)")
}

// 0x9610e0 — __ZN3RBX7Network12IdSerializer22deserializeInstanceRefERN6RakNet9BitStreamERPNS_8InstanceERNS_4Guid4DataE
// type: _DWORD __fastcall(RBX::Network::IdSerializer *__hidden this, RakNet::BitStream *, RBX::Instance **, RBX::Guid::Data *)
#[doc(alias = "__ZN3RBX7Network12IdSerializer22deserializeInstanceRefERN6RakNet9BitStreamERPNS_8InstanceERNS_4Guid4DataE")]
#[doc(alias = "RBX::Network::IdSerializer::deserializeInstanceRef(RakNet::BitStream &,RBX::Instance *&,RBX::Guid::Data &)")]
pub fn stub_9610e0() -> ! {
    todo!("0x9610e0 RBX::Network::IdSerializer::deserializeInstanceRef(RakNet::BitStream &,RBX::Instance *&,RBX::Guid::Data &)")
}

// 0x961178 — __ZN3RBX7Network12IdSerializer13addPendingRefEPKNS_10Reflection21RefPropertyDescriptorEN5boost10shared_ptrINS_8InstanceEEENS_4Guid4DataE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX7Network12IdSerializer13addPendingRefEPKNS_10Reflection21RefPropertyDescriptorEN5boost10shared_ptrINS_8InstanceEEENS_4Guid4DataE")]
#[doc(alias = "RBX::Network::IdSerializer::addPendingRef(RBX::Reflection::RefPropertyDescriptor const*,boost::shared_ptr<RBX::Instance>,RBX::Guid::Data)")]
pub fn stub_961178() -> ! {
    todo!("0x961178 RBX::Network::IdSerializer::addPendingRef(RBX::Reflection::RefPropertyDescriptor const*,boost::shared_ptr<RBX::Instance>,RBX::Guid::Data)")
}

// 0x961480 — __ZNK3RBX7Network16DescriptorSenderINS_10Reflection15ClassDescriptorEE9teachNameEPKS3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZNK3RBX7Network16DescriptorSenderINS_10Reflection15ClassDescriptorEE9teachNameEPKS3_")]
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::ClassDescriptor>::teachName(RBX::Reflection::ClassDescriptor const*)const")]
pub fn stub_961480() -> ! {
    todo!("0x961480 RBX::Network::DescriptorSender<RBX::Reflection::ClassDescriptor>::teachName(RBX::Reflection::ClassDescriptor const*)const")
}

// 0x961490 — __ZN3RBX7Network18DescriptorReceiverINS_10Reflection15ClassDescriptorEE9learnNameESsi
// type: int __fastcall(int, RBX::Name *this, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX7Network18DescriptorReceiverINS_10Reflection15ClassDescriptorEE9learnNameESsi")]
#[doc(alias = "RBX::Network::DescriptorReceiver<RBX::Reflection::ClassDescriptor>::learnName(std::string,int)")]
pub fn stub_961490() -> ! {
    todo!("0x961490 RBX::Network::DescriptorReceiver<RBX::Reflection::ClassDescriptor>::learnName(std::string,int)")
}

// 0x961700 — __ZNK3RBX7Network16DescriptorSenderINS_10Reflection15EventDescriptorEE9teachNameEPKS3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZNK3RBX7Network16DescriptorSenderINS_10Reflection15EventDescriptorEE9teachNameEPKS3_")]
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::EventDescriptor>::teachName(RBX::Reflection::EventDescriptor const*)const")]
pub fn stub_961700() -> ! {
    todo!("0x961700 RBX::Network::DescriptorSender<RBX::Reflection::EventDescriptor>::teachName(RBX::Reflection::EventDescriptor const*)const")
}

// 0x9618c4 — __ZN3RBX7Network18DescriptorReceiverINS_10Reflection15EventDescriptorEE9learnNameESsi
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZN3RBX7Network18DescriptorReceiverINS_10Reflection15EventDescriptorEE9learnNameESsi")]
#[doc(alias = "RBX::Network::DescriptorReceiver<RBX::Reflection::EventDescriptor>::learnName(std::string,int)")]
pub fn stub_9618c4() -> ! {
    todo!("0x9618c4 RBX::Network::DescriptorReceiver<RBX::Reflection::EventDescriptor>::learnName(std::string,int)")
}

// 0x961ca4 — __ZNK3RBX7Network16DescriptorSenderINS_10Reflection18PropertyDescriptorEE9teachNameEPKS3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZNK3RBX7Network16DescriptorSenderINS_10Reflection18PropertyDescriptorEE9teachNameEPKS3_")]
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::PropertyDescriptor>::teachName(RBX::Reflection::PropertyDescriptor const*)const")]
pub fn stub_961ca4() -> ! {
    todo!("0x961ca4 RBX::Network::DescriptorSender<RBX::Reflection::PropertyDescriptor>::teachName(RBX::Reflection::PropertyDescriptor const*)const")
}

// 0x961e68 — __ZN3RBX7Network18DescriptorReceiverINS_10Reflection18PropertyDescriptorEE9learnNameESsi
// type: int __fastcall(void *, int, int)
#[doc(alias = "__ZN3RBX7Network18DescriptorReceiverINS_10Reflection18PropertyDescriptorEE9learnNameESsi")]
#[doc(alias = "RBX::Network::DescriptorReceiver<RBX::Reflection::PropertyDescriptor>::learnName(std::string,int)")]
pub fn stub_961e68() -> ! {
    todo!("0x961e68 RBX::Network::DescriptorReceiver<RBX::Reflection::PropertyDescriptor>::learnName(std::string,int)")
}

// 0x96208c — __ZNK3RBX7Network16DescriptorSenderINS_10Reflection4TypeEE9teachNameEPKS3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZNK3RBX7Network16DescriptorSenderINS_10Reflection4TypeEE9teachNameEPKS3_")]
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::Type>::teachName(RBX::Reflection::Type const*)const")]
pub fn stub_96208c() -> ! {
    todo!("0x96208c RBX::Network::DescriptorSender<RBX::Reflection::Type>::teachName(RBX::Reflection::Type const*)const")
}

// 0x96209c — __ZN3RBX7Network18DescriptorReceiverINS_10Reflection4TypeEE9learnNameESsi
// type: int __fastcall(int, RBX::Name *this, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX7Network18DescriptorReceiverINS_10Reflection4TypeEE9learnNameESsi")]
#[doc(alias = "RBX::Network::DescriptorReceiver<RBX::Reflection::Type>::learnName(std::string,int)")]
pub fn stub_96209c() -> ! {
    todo!("0x96209c RBX::Network::DescriptorReceiver<RBX::Reflection::Type>::learnName(std::string,int)")
}

// 0x962300 — __ZN3RBX7Network16DescriptorSenderINS_10Reflection15ClassDescriptorEEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3RBX7Network16DescriptorSenderINS_10Reflection15ClassDescriptorEEC2Ev")]
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::ClassDescriptor>::DescriptorSender(void)")]
pub fn stub_962300() -> ! {
    todo!("0x962300 RBX::Network::DescriptorSender<RBX::Reflection::ClassDescriptor>::DescriptorSender(void)")
}

// 0x962464 — __ZN3RBX7Network16DescriptorSenderINS_10Reflection18PropertyDescriptorEEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3RBX7Network16DescriptorSenderINS_10Reflection18PropertyDescriptorEEC2Ev")]
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::PropertyDescriptor>::DescriptorSender(void)")]
pub fn stub_962464() -> ! {
    todo!("0x962464 RBX::Network::DescriptorSender<RBX::Reflection::PropertyDescriptor>::DescriptorSender(void)")
}

// 0x962694 — __ZN3RBX7Network16DescriptorSenderINS_10Reflection15EventDescriptorEEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3RBX7Network16DescriptorSenderINS_10Reflection15EventDescriptorEEC2Ev")]
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::EventDescriptor>::DescriptorSender(void)")]
pub fn stub_962694() -> ! {
    todo!("0x962694 RBX::Network::DescriptorSender<RBX::Reflection::EventDescriptor>::DescriptorSender(void)")
}

// 0x9628c4 — __ZN3RBX7Network16DescriptorSenderINS_10Reflection4TypeEEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3RBX7Network16DescriptorSenderINS_10Reflection4TypeEEC2Ev")]
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::Type>::DescriptorSender(void)")]
pub fn stub_9628c4() -> ! {
    todo!("0x9628c4 RBX::Network::DescriptorSender<RBX::Reflection::Type>::DescriptorSender(void)")
}

// 0x962a24 — __ZN6RakNet9BitStream5WriteIiEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN6RakNet9BitStream5WriteIiEEvRKT_")]
#[doc(alias = "void RakNet::BitStream::Write<int>(int const&)")]
pub fn stub_962a24() -> ! {
    todo!("0x962a24 void RakNet::BitStream::Write<int>(int const&)")
}

// 0x962b38 — __ZN6RakNet9BitStream4ReadIlEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN6RakNet9BitStream4ReadIlEEbRT_")]
#[doc(alias = "bool RakNet::BitStream::Read<long>(long &)")]
pub fn stub_962b38() -> ! {
    todo!("0x962b38 bool RakNet::BitStream::Read<long>(long &)")
}

// 0x962c60 — __ZN6RakNet9BitStream5WriteIyEEvRKT_
// type: int __fastcall(RakNet::BitStream *, RakNet::BitStream *, int, unsigned int)
#[doc(alias = "__ZN6RakNet9BitStream5WriteIyEEvRKT_")]
#[doc(alias = "void RakNet::BitStream::Write<unsigned long long>(unsigned long long const&)")]
pub fn stub_962c60() -> ! {
    todo!("0x962c60 void RakNet::BitStream::Write<unsigned long long>(unsigned long long const&)")
}

// 0x962d98 — __ZN6RakNet9BitStream4ReadIyEEbRT_
#[doc(alias = "__ZN6RakNet9BitStream4ReadIyEEbRT_")]
#[doc(alias = "bool RakNet::BitStream::Read<unsigned long long>(unsigned long long &)")]
pub fn stub_962d98() -> ! {
    todo!("0x962d98 bool RakNet::BitStream::Read<unsigned long long>(unsigned long long &)")
}

// 0x962ee4 — __ZN6RakNet9BitStream5WriteIlEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN6RakNet9BitStream5WriteIlEEvRKT_")]
#[doc(alias = "void RakNet::BitStream::Write<long>(long const&)")]
pub fn stub_962ee4() -> ! {
    todo!("0x962ee4 void RakNet::BitStream::Write<long>(long const&)")
}

// 0x962ff8 — __ZN6RakNet9BitStream4ReadIiEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN6RakNet9BitStream4ReadIiEEbRT_")]
#[doc(alias = "bool RakNet::BitStream::Read<int>(int &)")]
pub fn stub_962ff8() -> ! {
    todo!("0x962ff8 bool RakNet::BitStream::Read<int>(int &)")
}

// 0x963120 — __ZN6RakNet9BitStream5WriteIjEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN6RakNet9BitStream5WriteIjEEvRKT_")]
#[doc(alias = "void RakNet::BitStream::Write<unsigned int>(unsigned int const&)")]
pub fn stub_963120() -> ! {
    todo!("0x963120 void RakNet::BitStream::Write<unsigned int>(unsigned int const&)")
}

// 0x963234 — __ZN6RakNet9BitStream4ReadIjEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN6RakNet9BitStream4ReadIjEEbRT_")]
#[doc(alias = "bool RakNet::BitStream::Read<unsigned int>(unsigned int &)")]
pub fn stub_963234() -> ! {
    todo!("0x963234 bool RakNet::BitStream::Read<unsigned int>(unsigned int &)")
}

// 0x96335c — __ZN6RakNet9BitStream5WriteImEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN6RakNet9BitStream5WriteImEEvRKT_")]
#[doc(alias = "void RakNet::BitStream::Write<unsigned long>(unsigned long const&)")]
pub fn stub_96335c() -> ! {
    todo!("0x96335c void RakNet::BitStream::Write<unsigned long>(unsigned long const&)")
}

// 0x963470 — __ZN6RakNet9BitStream4ReadImEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN6RakNet9BitStream4ReadImEEbRT_")]
#[doc(alias = "bool RakNet::BitStream::Read<unsigned long>(unsigned long &)")]
pub fn stub_963470() -> ! {
    todo!("0x963470 bool RakNet::BitStream::Read<unsigned long>(unsigned long &)")
}

// 0x963598 — __ZN6RakNet9BitStream5WriteIdEEvRKT_
#[doc(alias = "__ZN6RakNet9BitStream5WriteIdEEvRKT_")]
#[doc(alias = "void RakNet::BitStream::Write<double>(double const&)")]
pub fn stub_963598() -> ! {
    todo!("0x963598 void RakNet::BitStream::Write<double>(double const&)")
}

// 0x9636d0 — __ZN6RakNet9BitStream4ReadIdEEbRT_
#[doc(alias = "__ZN6RakNet9BitStream4ReadIdEEbRT_")]
#[doc(alias = "bool RakNet::BitStream::Read<double>(double &)")]
pub fn stub_9636d0() -> ! {
    todo!("0x9636d0 bool RakNet::BitStream::Read<double>(double &)")
}

// 0x96381c — __ZN6RakNet9BitStream5WriteIsEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN6RakNet9BitStream5WriteIsEEvRKT_")]
#[doc(alias = "void RakNet::BitStream::Write<short>(short const&)")]
pub fn stub_96381c() -> ! {
    todo!("0x96381c void RakNet::BitStream::Write<short>(short const&)")
}

// 0x963930 — __ZN6RakNet9BitStream4ReadIsEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN6RakNet9BitStream4ReadIsEEbRT_")]
#[doc(alias = "bool RakNet::BitStream::Read<short>(short &)")]
pub fn stub_963930() -> ! {
    todo!("0x963930 bool RakNet::BitStream::Read<short>(short &)")
}

// 0x9645f8 — __ZNSt3mapIN3RBX4Guid4DataESt6vectorINS0_7Network12IdSerializer8WaitItemESaIS6_EESt4lessIS2_ESaISt4pairIKS2_S8_EEEixERSC_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, void *, int, int, void *, void *, int, int, int, int)
#[doc(alias = "__ZNSt3mapIN3RBX4Guid4DataESt6vectorINS0_7Network12IdSerializer8WaitItemESaIS6_EESt4lessIS2_ESaISt4pairIKS2_S8_EEEixERSC_")]
#[doc(alias = "std::map<RBX::Guid::Data,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::operator[](RBX::Guid::Data const&)")]
pub fn stub_9645f8() -> ! {
    todo!("0x9645f8 std::map<RBX::Guid::Data,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::operator[](RBX::Guid::Data const&)")
}

// 0x9652ac — __ZNSt6vectorIN3RBX7Network12IdSerializer8WaitItemESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_ // was: boost
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, boost::detail::shared_count *, int, int, int, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZNSt6vectorIN3RBX7Network12IdSerializer8WaitItemESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
#[doc(alias = "std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::IdSerializer::WaitItem*,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,RBX::Network::IdSerializer::WaitItem const&)")]
pub fn stub_9652ac() -> ! {
    todo!("0x9652ac std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::IdSerializer::WaitItem*,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,RBX::Network::IdSerializer::WaitItem const&)")
}

// 0x965a2c — __ZNSt6vectorIN3RBX7Network12IdSerializer8WaitItemESaIS3_EEC2ERKS5_
#[doc(alias = "__ZNSt6vectorIN3RBX7Network12IdSerializer8WaitItemESaIS3_EEC2ERKS5_")]
#[doc(alias = "std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>::vector(std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>> const&)")]
pub fn stub_965a2c() -> ! {
    todo!("0x965a2c std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>::vector(std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>> const&)")
}

// 0x965c54 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")]
pub fn stub_965c54() -> ! {
    todo!("0x965c54 std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>> const&)")
}

// 0x965d3c — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSB_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSB_")]
pub fn stub_965d3c() -> ! {
    todo!("0x965d3c std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>> const&)")
}

// 0x965e7c — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE16_M_insert_uniqueERKSB_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE16_M_insert_uniqueERKSB_")]
pub fn stub_965e7c() -> ! {
    todo!("0x965e7c std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_insert_unique(std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>> const&)")
}

// 0x965f98 — __ZN3RBX7Network16SenderDictionaryIPKNS_4NameEE7trySendERN6RakNet9BitStreamES4_
// type: int __fastcall(int, RakNet::BitStream *this)
#[doc(alias = "__ZN3RBX7Network16SenderDictionaryIPKNS_4NameEE7trySendERN6RakNet9BitStreamES4_")]
#[doc(alias = "RBX::Network::SenderDictionary<RBX::Name const*>::trySend(RakNet::BitStream &,RBX::Name const*)")]
pub fn stub_965f98() -> ! {
    todo!("0x965f98 RBX::Network::SenderDictionary<RBX::Name const*>::trySend(RakNet::BitStream &,RBX::Name const*)")
}

// 0x966d78 — __ZN3RBX7Network6Client13playerConnectEiSsiii
#[doc(alias = "__ZN3RBX7Network6Client13playerConnectEiSsiii")]
#[doc(alias = "RBX::Network::Client::playerConnect(int,std::string,int,int,int)")]
pub fn stub_966d78() -> ! {
    todo!("0x966d78 RBX::Network::Client::playerConnect(int,std::string,int,int,int)")
}

// 0x96765c — __ZN3RBX7Network6Client10disconnectEi
// type: _DWORD __fastcall(RBX::Network::Client *__hidden this, int)
#[doc(alias = "__ZN3RBX7Network6Client10disconnectEi")]
#[doc(alias = "RBX::Network::Client::disconnect(int)")]
pub fn stub_96765c() -> ! {
    todo!("0x96765c RBX::Network::Client::disconnect(int)")
}

// 0x967744 — __ZN3RBX7Network6ClientC2Ev
// type: _DWORD __fastcall(RBX::Network::Client *__hidden this)
#[doc(alias = "__ZN3RBX7Network6ClientC2Ev")]
#[doc(alias = "RBX::Network::Client::Client(void)")]
pub fn stub_967744() -> ! {
    todo!("0x967744 RBX::Network::Client::Client(void)")
}

// 0x967e28 — __ZN3RBX7Network6ClientD0Ev
// type: void __fastcall(RBX::Network::Client *__hidden this)
#[doc(alias = "__ZN3RBX7Network6ClientD0Ev")]
#[doc(alias = "RBX::Network::Client::~Client()")]
pub fn stub_967e28() -> ! {
    todo!("0x967e28 RBX::Network::Client::~Client()")
}

// 0x967ec8 — __ZN3RBX7Network6ClientD1Ev
// type: void __fastcall(RBX::Network::Client *__hidden this)
#[doc(alias = "__ZN3RBX7Network6ClientD1Ev")]
#[doc(alias = "RBX::Network::Client::~Client()")]
pub fn stub_967ec8() -> ! {
    todo!("0x967ec8 RBX::Network::Client::~Client()")
}

// 0x967ed4 — __ZThn32_N3RBX7Network6ClientD0Ev
// type: void __fastcall(RBX::Network::Client *__hidden this)
#[doc(alias = "__ZThn32_N3RBX7Network6ClientD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Network::Client::~Client()")]
pub fn stub_967ed4() -> ! {
    todo!("0x967ed4 non-virtual thunk toRBX::Network::Client::~Client()")
}

// 0x967f78 — __ZThn36_N3RBX7Network6ClientD0Ev
// type: void __fastcall(RBX::Network::Client *__hidden this)
#[doc(alias = "__ZThn36_N3RBX7Network6ClientD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Network::Client::~Client()")]
pub fn stub_967f78() -> ! {
    todo!("0x967f78 non-virtual thunk toRBX::Network::Client::~Client()")
}

// 0x96801c — __ZThn92_N3RBX7Network6ClientD0Ev
// type: void __fastcall(RBX::Network::Client *this, int, int, const void *)
#[doc(alias = "__ZThn92_N3RBX7Network6ClientD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::Network::Client::~Client()")]
pub fn stub_96801c() -> ! {
    todo!("0x96801c non-virtual thunk toRBX::Network::Client::~Client()")
}

// 0x9680c0 — __ZN3RBX7Network6ClientD2Ev
// type: void __fastcall(RBX::Network::Client *this, int, int, const void *)
#[doc(alias = "__ZN3RBX7Network6ClientD2Ev")]
#[doc(alias = "RBX::Network::Client::~Client()")]
pub fn stub_9680c0() -> ! {
    todo!("0x9680c0 RBX::Network::Client::~Client()")
}

// 0x9686ac — __ZThn32_N3RBX7Network6ClientD1Ev
// type: void __fastcall(RBX::Network::Client *this, int, int, const void *)
#[doc(alias = "__ZThn32_N3RBX7Network6ClientD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Network::Client::~Client()")]
pub fn stub_9686ac() -> ! {
    todo!("0x9686ac non-virtual thunk toRBX::Network::Client::~Client()")
}

// 0x9686b8 — __ZThn36_N3RBX7Network6ClientD1Ev
// type: void __fastcall(RBX::Network::Client *this, int, int, const void *)
#[doc(alias = "__ZThn36_N3RBX7Network6ClientD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Network::Client::~Client()")]
pub fn stub_9686b8() -> ! {
    todo!("0x9686b8 non-virtual thunk toRBX::Network::Client::~Client()")
}

// 0x9686c4 — __ZThn92_N3RBX7Network6ClientD1Ev
// type: void __fastcall(RBX::Network::Client *this, int, int, const void *)
#[doc(alias = "__ZThn92_N3RBX7Network6ClientD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::Network::Client::~Client()")]
pub fn stub_9686c4() -> ! {
    todo!("0x9686c4 non-virtual thunk toRBX::Network::Client::~Client()")
}

// 0x9686d0 — __ZN3RBX7Network6Client10findClientEPKNS_8InstanceEb
// type: int __fastcall(RBX::Network::Client *this, int, bool, const void *)
#[doc(alias = "__ZN3RBX7Network6Client10findClientEPKNS_8InstanceEb")]
#[doc(alias = "RBX::Network::Client::findClient(RBX::Instance const*,bool)")]
pub fn stub_9686d0() -> ! {
    todo!("0x9686d0 RBX::Network::Client::findClient(RBX::Instance const*,bool)")
}

// 0x9688a4 — __ZN3RBX7Network6Client15clientIsPresentEPKNS_8InstanceEb
// type: bool __fastcall(RBX::Network::Client *this, const RBX::Instance *, bool, const void *)
#[doc(alias = "__ZN3RBX7Network6Client15clientIsPresentEPKNS_8InstanceEb")]
#[doc(alias = "RBX::Network::Client::clientIsPresent(RBX::Instance const*,bool)")]
pub fn stub_9688a4() -> ! {
    todo!("0x9688a4 RBX::Network::Client::clientIsPresent(RBX::Instance const*,bool)")
}

// 0x9688b4 — __ZN3RBX7Network6Client27physicsOutBandwidthExceededEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::Client *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZN3RBX7Network6Client27physicsOutBandwidthExceededEPKNS_8InstanceE")]
#[doc(alias = "RBX::Network::Client::physicsOutBandwidthExceeded(RBX::Instance const*)")]
pub fn stub_9688b4() -> ! {
    todo!("0x9688b4 RBX::Network::Client::physicsOutBandwidthExceeded(RBX::Instance const*)")
}

// 0x9688d8 — __ZN3RBX7Network6Client22getNetworkBufferHealthEPKNS_8InstanceE
// type: int __fastcall(RBX::Network::Client *this, const RBX::Instance *, bool, const void *)
#[doc(alias = "__ZN3RBX7Network6Client22getNetworkBufferHealthEPKNS_8InstanceE")]
#[doc(alias = "RBX::Network::Client::getNetworkBufferHealth(RBX::Instance const*)")]
pub fn stub_9688d8() -> ! {
    todo!("0x9688d8 RBX::Network::Client::getNetworkBufferHealth(RBX::Instance const*)")
}

// 0x968910 — __ZN3RBX7Network6Client25findLocalSimulatorAddressEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::Client *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZN3RBX7Network6Client25findLocalSimulatorAddressEPKNS_8InstanceE")]
#[doc(alias = "RBX::Network::Client::findLocalSimulatorAddress(RBX::Instance const*)")]
pub fn stub_968910() -> ! {
    todo!("0x968910 RBX::Network::Client::findLocalSimulatorAddress(RBX::Instance const*)")
}

// 0x968970 — __ZN3RBX7Network6Client17onServiceProviderEPNS_15ServiceProviderES3_
// type: void __fastcall(RBX::Network::Client *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "__ZN3RBX7Network6Client17onServiceProviderEPNS_15ServiceProviderES3_")]
#[doc(alias = "RBX::Network::Client::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_968970() -> ! {
    todo!("0x968970 RBX::Network::Client::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x968c18 — __ZN3RBX7Network6Client10sendTicketEv
// type: int __fastcall(__guard **this)
#[doc(alias = "__ZN3RBX7Network6Client10sendTicketEv")]
#[doc(alias = "RBX::Network::Client::sendTicket(void)")]
pub fn stub_968c18() -> ! {
    todo!("0x968c18 RBX::Network::Client::sendTicket(void)")
}

// 0x968fb0 — __ZN3RBX7Network6Client25OnFailedConnectionAttemptEPN6RakNet6PacketENS2_33PI2_FailedConnectionAttemptReasonE
// type: void __fastcall(int, unsigned __int8 **)
#[doc(alias = "__ZN3RBX7Network6Client25OnFailedConnectionAttemptEPN6RakNet6PacketENS2_33PI2_FailedConnectionAttemptReasonE")]
#[doc(alias = "RBX::Network::Client::OnFailedConnectionAttempt(RakNet::Packet *,RakNet::PI2_FailedConnectionAttemptReason)")]
pub fn stub_968fb0() -> ! {
    todo!("0x968fb0 RBX::Network::Client::OnFailedConnectionAttempt(RakNet::Packet *,RakNet::PI2_FailedConnectionAttemptReason)")
}

// 0x9694b4 — __ZThn92_N3RBX7Network6Client25OnFailedConnectionAttemptEPN6RakNet6PacketENS2_33PI2_FailedConnectionAttemptReasonE
// type: void __fastcall(int, unsigned __int8 **)
#[doc(alias = "__ZThn92_N3RBX7Network6Client25OnFailedConnectionAttemptEPN6RakNet6PacketENS2_33PI2_FailedConnectionAttemptReasonE")]
#[doc(alias = "non-virtual thunk toRBX::Network::Client::OnFailedConnectionAttempt(RakNet::Packet *,RakNet::PI2_FailedConnectionAttemptReason)")]
pub fn stub_9694b4() -> ! {
    todo!("0x9694b4 non-virtual thunk toRBX::Network::Client::OnFailedConnectionAttempt(RakNet::Packet *,RakNet::PI2_FailedConnectionAttemptReason)")
}

// 0x9694c0 — __ZNK3RBX7Network6Client21sendPreferedSpawnNameEv
// type: int __fastcall(RBX::Network::ConcurrentRakPeer **this)
#[doc(alias = "__ZNK3RBX7Network6Client21sendPreferedSpawnNameEv")]
#[doc(alias = "RBX::Network::Client::sendPreferedSpawnName(void)const")]
pub fn stub_9694c0() -> ! {
    todo!("0x9694c0 RBX::Network::Client::sendPreferedSpawnName(void)const")
}

// 0x969704 — __ZN3RBX7Network6Client9OnReceiveEPN6RakNet6PacketE
// type: int __fastcall(int, int)
#[doc(alias = "__ZN3RBX7Network6Client9OnReceiveEPN6RakNet6PacketE")]
#[doc(alias = "RBX::Network::Client::OnReceive(RakNet::Packet *)")]
pub fn stub_969704() -> ! {
    todo!("0x969704 RBX::Network::Client::OnReceive(RakNet::Packet *)")
}

// 0x96c474 — __ZThn92_N3RBX7Network6Client9OnReceiveEPN6RakNet6PacketE
// type: int __fastcall(int, int)
#[doc(alias = "__ZThn92_N3RBX7Network6Client9OnReceiveEPN6RakNet6PacketE")]
#[doc(alias = "non-virtual thunk toRBX::Network::Client::OnReceive(RakNet::Packet *)")]
pub fn stub_96c474() -> ! {
    todo!("0x96c474 non-virtual thunk toRBX::Network::Client::OnReceive(RakNet::Packet *)")
}

// 0x96c484 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFN5boost10shared_ptrINS_8InstanceEEEiSsiiiELi5EED1Ev // was: boost::shared_ptr
// type: int()
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFN5boost10shared_ptrINS_8InstanceEEEiSsiiiELi5EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Client,boost::shared_ptr<RBX::Instance> ()(int,std::string,int,int,int),5>::~BoundFuncDesc()")]
pub fn stub_96c484() -> ! {
    todo!("0x96c484 RBX::Reflection::BoundFuncDesc<RBX::Network::Client,boost::shared_ptr<RBX::Instance> ()(int,std::string,int,int,int),5>::~BoundFuncDesc()")
}

// 0x96c490 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFviELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFviELi1EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Client,void ()(int),1>::~BoundFuncDesc()")]
pub fn stub_96c490() -> ! {
    todo!("0x96c490 RBX::Reflection::BoundFuncDesc<RBX::Network::Client,void ()(int),1>::~BoundFuncDesc()")
}

// 0x96c4f8 — __ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev // was: boost::shared_ptr
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Client::*>::~EventDesc()")]
pub fn stub_96c4f8() -> ! {
    todo!("0x96c4f8 RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Client::*>::~EventDesc()")
}

// 0x96c540 — __ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Client::*>::~EventDesc()")]
pub fn stub_96c540() -> ! {
    todo!("0x96c540 RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Client::*>::~EventDesc()")
}

// 0x96c588 — __ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::~EventDesc()")]
pub fn stub_96c588() -> ! {
    todo!("0x96c588 RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::~EventDesc()")
}

// 0x96c5d0 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_7Network16ClientReplicatorEEEPKT_v
// type: int __fastcall(int, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX8Instance25findConstFirstChildOfTypeINS_7Network16ClientReplicatorEEEPKT_v")]
#[doc(alias = "RBX::Network::ClientReplicator const* RBX::Instance::findConstFirstChildOfType<RBX::Network::ClientReplicator>(void)const")]
pub fn stub_96c5d0() -> ! {
    todo!("0x96c5d0 RBX::Network::ClientReplicator const* RBX::Instance::findConstFirstChildOfType<RBX::Network::ClientReplicator>(void)const")
}

// 0x96ca10 — __ZN3RBX7Network6Client10disconnectEv
// type: int __fastcall(RBX::Network::Client *this)
#[doc(alias = "__ZN3RBX7Network6Client10disconnectEv")]
#[doc(alias = "RBX::Network::Client::disconnect(void)")]
pub fn stub_96ca10() -> ! {
    todo!("0x96ca10 RBX::Network::Client::disconnect(void)")
}

// 0x96d260 — __ZN6RakNet16PluginInterface29OnReceiveEPNS_6PacketE
// type: int()
#[doc(alias = "__ZN6RakNet16PluginInterface29OnReceiveEPNS_6PacketE")]
#[doc(alias = "RakNet::PluginInterface2::OnReceive(RakNet::Packet *)")]
pub fn stub_96d260() -> ! {
    todo!("0x96d260 RakNet::PluginInterface2::OnReceive(RakNet::Packet *)")
}

// 0x96fd88 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network16ClientReplicatorES7_EEvPKNS_10shared_ptrIT_EEPT0_ // was: boost::shared_ptr
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network16ClientReplicatorES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::ClientReplicator,RBX::Network::ClientReplicator>(boost::shared_ptr<RBX::Network::ClientReplicator> const*,RBX::Network::ClientReplicator *)const")]
pub fn stub_96fd88() -> ! {
    todo!("0x96fd88 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::ClientReplicator,RBX::Network::ClientReplicator>(boost::shared_ptr<RBX::Network::ClientReplicator> const*,RBX::Network::ClientReplicator *)const")
}

// 0x970044 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ClientReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev // was: boost
// type: void()
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ClientReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ClientReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_970044() -> ! {
    todo!("0x970044 boost::detail::sp_counted_impl_pd<RBX::Network::ClientReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x970048 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ClientReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev // was: boost
// type: void __fastcall(void *)
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ClientReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ClientReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_970048() -> ! {
    todo!("0x970048 boost::detail::sp_counted_impl_pd<RBX::Network::ClientReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x970054 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ClientReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv // was: boost
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ClientReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ClientReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_970054() -> ! {
    todo!("0x970054 boost::detail::sp_counted_impl_pd<RBX::Network::ClientReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x970070 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ClientReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info // was: boost
// type: int __fastcall(int, int)
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ClientReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ClientReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_970070() -> ! {
    todo!("0x970070 boost::detail::sp_counted_impl_pd<RBX::Network::ClientReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x970088 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ClientReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv // was: boost
// type: int __fastcall(int)
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ClientReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ClientReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_970088() -> ! {
    todo!("0x970088 boost::detail::sp_counted_impl_pd<RBX::Network::ClientReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x970304 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX7Network6ClientEEENS6_5list1INS6_5valueIPSC_EEEEEEED1Ev // was: boost
// type: int __fastcall(int)
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX7Network6ClientEEENS6_5list1INS6_5valueIPSC_EEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Client>,boost::_bi::list1<boost::_bi::value<RBX::Network::Client*>>>>::~callable_slot()")]
pub fn stub_970304() -> ! {
    todo!("0x970304 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Client>,boost::_bi::list1<boost::_bi::value<RBX::Network::Client*>>>>::~callable_slot()")
}

// 0x970360 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX7Network6ClientEEENS6_5list1INS6_5valueIPSC_EEEEEEED0Ev // was: boost
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX7Network6ClientEEENS6_5list1INS6_5valueIPSC_EEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Client>,boost::_bi::list1<boost::_bi::value<RBX::Network::Client*>>>>::~callable_slot()")]
pub fn stub_970360() -> ! {
    todo!("0x970360 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Client>,boost::_bi::list1<boost::_bi::value<RBX::Network::Client*>>>>::~callable_slot()")
}

// 0x970468 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX7Network6ClientEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv // was: boost
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX7Network6ClientEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Client>,boost::_bi::list1<boost::_bi::value<RBX::Network::Client*>>>,0,void ()(void)>::call(void)")]
pub fn stub_970468() -> ! {
    todo!("0x970468 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Client>,boost::_bi::list1<boost::_bi::value<RBX::Network::Client*>>>,0,void ()(void)>::call(void)")
}

// 0x970484 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX7Network6ClientEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv // was: boost
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX7Network6ClientEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Client>,boost::_bi::list1<boost::_bi::value<RBX::Network::Client*>>>,0,void ()(void)>::call(void)")]
pub fn stub_970484() -> ! {
    todo!("0x970484 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Client>,boost::_bi::list1<boost::_bi::value<RBX::Network::Client*>>>,0,void ()(void)>::call(void)")
}

// 0x9709b0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network6ClientES7_EEvPKNS_10shared_ptrIT_EEPT0_ // was: boost::shared_ptr
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network6ClientES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Client,RBX::Network::Client>(boost::shared_ptr<RBX::Network::Client> const*,RBX::Network::Client *)const")]
pub fn stub_9709b0() -> ! {
    todo!("0x9709b0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Client,RBX::Network::Client>(boost::shared_ptr<RBX::Network::Client> const*,RBX::Network::Client *)const")
}

// 0x970c70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev // was: boost
// type: void __fastcall(void *)
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_970c70() -> ! {
    todo!("0x970c70 boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x970c80 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info // was: boost
// type: int __fastcall(int, int)
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_970c80() -> ! {
    todo!("0x970c80 boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x970c98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv // was: boost
// type: int __fastcall(int)
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_970c98() -> ! {
    todo!("0x970c98 boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x970f5c — __ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_SB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_SB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_970f5c() -> ! {
    todo!("0x970f5c RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x97133c — __ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_ED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::~EventDesc()")]
pub fn stub_97133c() -> ! {
    todo!("0x97133c RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::~EventDesc()")
}