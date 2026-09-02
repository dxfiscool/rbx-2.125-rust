//! core shard HP — 4 core stubs EA-sorted, 0xf6b3f4..0xf6b424 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 4 after HO 0xf6b3e4 (21914->21918 covered, 0 remaining).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 4 uncovered (0xf6b3f4..0xf6b424, 21914->21918 covered, 0 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::string::append(unsigned long,char)")]
// 0xf6b3f4 — __ZNSs6appendEmc
pub fn stub_0xf6b3f4() -> ! {
    todo!("0xf6b3f4 __ZNSs6appendEmc")
}

#[doc(alias = "std::string::assign(char const*,unsigned long)")]
// 0xf6b404 — __ZNSs6assignEPKcm
pub fn stub_0xf6b404() -> ! {
    todo!("0xf6b404 __ZNSs6assignEPKcm")
}

#[doc(alias = "std::string::assign(std::string const&)")]
// 0xf6b414 — __ZNSs6assignERKSs
pub fn stub_0xf6b414() -> ! {
    todo!("0xf6b414 __ZNSs6assignERKSs")
}

#[doc(alias = "std::string::insert(unsigned long,char const*,unsigned long)")]
// 0xf6b424 — __ZNSs6insertEmPKcm
pub fn stub_0xf6b424() -> ! {
    todo!("0xf6b424 __ZNSs6insertEmPKcm")
}

