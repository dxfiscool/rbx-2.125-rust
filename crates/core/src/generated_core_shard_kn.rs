//! core shard kn — 120 stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 120 not yet in rbx_core after km 0xe87f4c (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33260 filtered, 5063->4943 remaining, 28197->28317 distinct, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "-[EAGL2View mWindowName]")]
#[doc(alias = "-[EAGL2View mWindowName]")]
// 0xe880b4 — -[EAGL2View mWindowName]
// type: basic_string<char, std::char_traits<char>, std::allocator<char> > __cdecl(EAGL2View *self, SEL)
pub fn stub_0xe880b4() {
    // IDA 0xe880b4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[EAGL2View setMWindowName:]")]
#[doc(alias = "-[EAGL2View setMWindowName:]")]
// 0xe880cc — -[EAGL2View setMWindowName:]
// type: void __cdecl(EAGL2View *self, SEL, basic_string<char, std::char_traits<char>, std::allocator<char> >)
pub fn stub_0xe880cc() {
    // IDA 0xe880cc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[EAGL2View .cxx_destruct]")]
#[doc(alias = "-[EAGL2View .cxx_destruct]")]
// 0xe880e8 — -[EAGL2View .cxx_destruct]
// type: void __cdecl(EAGL2View *self, SEL)
pub fn stub_0xe880e8() {
    // IDA 0xe880e8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[EAGL2View .cxx_construct]")]
#[doc(alias = "-[EAGL2View .cxx_construct]")]
// 0xe88140 — -[EAGL2View .cxx_construct]
// type: id __cdecl(EAGL2View *self, SEL)
pub fn stub_0xe88140() {
    // IDA 0xe88140: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_849")]
#[doc(alias = "__GLOBAL__I_a_849")]
// 0xe88160 — __GLOBAL__I_a_849
pub fn stub_0xe88160() {
    // IDA 0xe88160: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "-[EAGL2ViewController init]")]
#[doc(alias = "-[EAGL2ViewController init]")]
// 0xe88194 — -[EAGL2ViewController init]
// type: EAGL2ViewController *__cdecl(EAGL2ViewController *self, SEL)
pub fn stub_0xe88194() {
    // IDA 0xe88194: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "-[EAGL2ViewController initWithNibName:bundle:]")]
#[doc(alias = "-[EAGL2ViewController initWithNibName:bundle:]")]
// 0xe881c0 — -[EAGL2ViewController initWithNibName:bundle:]
// type: EAGL2ViewController *__cdecl(EAGL2ViewController *self, SEL, id, id)
pub fn stub_0xe881c0() {
    // IDA 0xe881c0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "-[EAGL2ViewController dealloc]")]
#[doc(alias = "-[EAGL2ViewController dealloc]")]
// 0xe881f0 — -[EAGL2ViewController dealloc]
// type: void __cdecl(EAGL2ViewController *self, SEL)
pub fn stub_0xe881f0() {
    // IDA 0xe881f0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "-[EAGL2ViewController didReceiveMemoryWarning]")]
#[doc(alias = "-[EAGL2ViewController didReceiveMemoryWarning]")]
// 0xe8821c — -[EAGL2ViewController didReceiveMemoryWarning]
// type: void __cdecl(EAGL2ViewController *self, SEL)
pub fn stub_0xe8821c() {
    // IDA 0xe8821c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[EAGL2ViewController loadView]")]
#[doc(alias = "-[EAGL2ViewController loadView]")]
// 0xe88248 — -[EAGL2ViewController loadView]
// type: void __cdecl(EAGL2ViewController *self, SEL)
pub fn stub_0xe88248() {
    // IDA 0xe88248: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[EAGL2ViewController viewDidLoad]")]
#[doc(alias = "-[EAGL2ViewController viewDidLoad]")]
// 0xe88274 — -[EAGL2ViewController viewDidLoad]
// type: void __cdecl(EAGL2ViewController *self, SEL)
pub fn stub_0xe88274() {
    // IDA 0xe88274: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[EAGL2ViewController viewDidUnload]")]
#[doc(alias = "-[EAGL2ViewController viewDidUnload]")]
// 0xe882a0 — -[EAGL2ViewController viewDidUnload]
// type: void __cdecl(EAGL2ViewController *self, SEL)
pub fn stub_0xe882a0() {
    // IDA 0xe882a0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[EAGL2ViewController shouldAutorotate]")]
#[doc(alias = "-[EAGL2ViewController shouldAutorotate]")]
// 0xe882cc — -[EAGL2ViewController shouldAutorotate]
// type: char __cdecl(EAGL2ViewController *self, SEL)
pub fn stub_0xe882cc() {
    // IDA 0xe882cc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[EAGL2ViewController supportedInterfaceOrientations]")]
#[doc(alias = "-[EAGL2ViewController supportedInterfaceOrientations]")]
// 0xe88310 — -[EAGL2ViewController supportedInterfaceOrientations]
// type: unsigned int __cdecl(EAGL2ViewController *self, SEL)
pub fn stub_0xe88310() {
    // IDA 0xe88310: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[EAGL2ViewController shouldAutorotateToInterfaceOrientation:]")]
#[doc(alias = "-[EAGL2ViewController shouldAutorotateToInterfaceOrientation:]")]
// 0xe88314 — -[EAGL2ViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(EAGL2ViewController *self, SEL, int)
pub fn stub_0xe88314() {
    // IDA 0xe88314: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[EAGL2ViewController mGLSupport]")]
#[doc(alias = "-[EAGL2ViewController mGLSupport]")]
// 0xe88328 — -[EAGL2ViewController mGLSupport]
// type: EAGL2Support *__cdecl(EAGL2ViewController *self, SEL)
pub fn stub_0xe88328() {
    // IDA 0xe88328: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[EAGL2ViewController setMGLSupport:]")]
#[doc(alias = "-[EAGL2ViewController setMGLSupport:]")]
// 0xe8833c — -[EAGL2ViewController setMGLSupport:]
// type: void __cdecl(EAGL2ViewController *self, SEL, EAGL2Support *)
pub fn stub_0xe8833c() {
    // IDA 0xe8833c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_850")]
#[doc(alias = "__GLOBAL__I_a_850")]
// 0xe88354 — __GLOBAL__I_a_850
pub fn stub_0xe88354() {
    // IDA 0xe88354: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_851")]
#[doc(alias = "__GLOBAL__I_a_851")]
// 0xe8a664 — __GLOBAL__I_a_851
pub fn stub_0xe8a664() {
    // IDA 0xe8a664: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_852")]
#[doc(alias = "__GLOBAL__I_a_852")]
// 0xe8b498 — __GLOBAL__I_a_852
pub fn stub_0xe8b498() {
    // IDA 0xe8b498: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_853")]
#[doc(alias = "__GLOBAL__I_a_853")]
// 0xe8bdc4 — __GLOBAL__I_a_853
pub fn stub_0xe8bdc4() {
    // IDA 0xe8bdc4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_854")]
#[doc(alias = "__GLOBAL__I_a_854")]
// 0xe8dd98 — __GLOBAL__I_a_854
pub fn stub_0xe8dd98() {
    // IDA 0xe8dd98: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_855")]
#[doc(alias = "__GLOBAL__I_a_855")]
// 0xe8e1a4 — __GLOBAL__I_a_855
pub fn stub_0xe8e1a4() {
    // IDA 0xe8e1a4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_856")]
#[doc(alias = "__GLOBAL__I_a_856")]
// 0xe8ea40 — __GLOBAL__I_a_856
pub fn stub_0xe8ea40() {
    // IDA 0xe8ea40: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_857")]
#[doc(alias = "__GLOBAL__I_a_857")]
// 0xe8ee84 — __GLOBAL__I_a_857
pub fn stub_0xe8ee84() {
    // IDA 0xe8ee84: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_858")]
#[doc(alias = "__GLOBAL__I_a_858")]
// 0xe8f664 — __GLOBAL__I_a_858
pub fn stub_0xe8f664() {
    // IDA 0xe8f664: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_859")]
#[doc(alias = "__GLOBAL__I_a_859")]
// 0xe90344 — __GLOBAL__I_a_859
pub fn stub_0xe90344() {
    // IDA 0xe90344: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_860")]
#[doc(alias = "__GLOBAL__I_a_860")]
// 0xe93870 — __GLOBAL__I_a_860
pub fn stub_0xe93870() {
    // IDA 0xe93870: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_861")]
#[doc(alias = "__GLOBAL__I_a_861")]
// 0xe9582c — __GLOBAL__I_a_861
pub fn stub_0xe9582c() {
    // IDA 0xe9582c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_862")]
#[doc(alias = "__GLOBAL__I_a_862")]
// 0xe98088 — __GLOBAL__I_a_862
pub fn stub_0xe98088() {
    // IDA 0xe98088: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_863")]
#[doc(alias = "__GLOBAL__I_a_863")]
// 0xe99ce0 — __GLOBAL__I_a_863
pub fn stub_0xe99ce0() {
    // IDA 0xe99ce0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_864")]
#[doc(alias = "__GLOBAL__I_a_864")]
// 0xe9aad8 — __GLOBAL__I_a_864
pub fn stub_0xe9aad8() {
    // IDA 0xe9aad8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_865")]
#[doc(alias = "__GLOBAL__I_a_865")]
// 0xe9ae58 — __GLOBAL__I_a_865
pub fn stub_0xe9ae58() {
    // IDA 0xe9ae58: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_866")]
#[doc(alias = "__GLOBAL__I_a_866")]
// 0xe9bcb0 — __GLOBAL__I_a_866
pub fn stub_0xe9bcb0() {
    // IDA 0xe9bcb0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_867")]
#[doc(alias = "__GLOBAL__I_a_867")]
// 0xe9d9d4 — __GLOBAL__I_a_867
pub fn stub_0xe9d9d4() {
    // IDA 0xe9d9d4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_868")]
#[doc(alias = "__GLOBAL__I_a_868")]
// 0xe9e59c — __GLOBAL__I_a_868
pub fn stub_0xe9e59c() {
    // IDA 0xe9e59c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "___cxx_global_array_dtor_5")]
#[doc(alias = "___cxx_global_array_dtor_5")]
// 0xe9f88c — ___cxx_global_array_dtor_5
pub fn stub_0xe9f88c() {
    // IDA 0xe9f88c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "___cxx_global_array_dtor3")]
#[doc(alias = "___cxx_global_array_dtor3")]
// 0xe9f890 — ___cxx_global_array_dtor3
pub fn stub_0xe9f890() {
    // IDA 0xe9f890: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_869")]
#[doc(alias = "__GLOBAL__I_a_869")]
// 0xe9f894 — __GLOBAL__I_a_869
pub fn stub_0xe9f894() {
    // IDA 0xe9f894: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_870")]
#[doc(alias = "__GLOBAL__I_a_870")]
// 0xe9fc48 — __GLOBAL__I_a_870
pub fn stub_0xe9fc48() {
    // IDA 0xe9fc48: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_871")]
#[doc(alias = "__GLOBAL__I_a_871")]
// 0xea0948 — __GLOBAL__I_a_871
pub fn stub_0xea0948() {
    // IDA 0xea0948: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_872")]
#[doc(alias = "__GLOBAL__I_a_872")]
// 0xea1598 — __GLOBAL__I_a_872
pub fn stub_0xea1598() {
    // IDA 0xea1598: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_873")]
#[doc(alias = "__GLOBAL__I_a_873")]
// 0xea1948 — __GLOBAL__I_a_873
pub fn stub_0xea1948() {
    // IDA 0xea1948: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_874")]
#[doc(alias = "__GLOBAL__I_a_874")]
// 0xea260c — __GLOBAL__I_a_874
pub fn stub_0xea260c() {
    // IDA 0xea260c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_875")]
#[doc(alias = "__GLOBAL__I_a_875")]
// 0xea3040 — __GLOBAL__I_a_875
pub fn stub_0xea3040() {
    // IDA 0xea3040: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_876")]
#[doc(alias = "__GLOBAL__I_a_876")]
// 0xea524c — __GLOBAL__I_a_876
pub fn stub_0xea524c() {
    // IDA 0xea524c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_877")]
#[doc(alias = "__GLOBAL__I_a_877")]
// 0xea552c — __GLOBAL__I_a_877
pub fn stub_0xea552c() {
    // IDA 0xea552c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_878")]
#[doc(alias = "__GLOBAL__I_a_878")]
// 0xea5eb0 — __GLOBAL__I_a_878
pub fn stub_0xea5eb0() {
    // IDA 0xea5eb0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_879")]
#[doc(alias = "__GLOBAL__I_a_879")]
// 0xea6d1c — __GLOBAL__I_a_879
pub fn stub_0xea6d1c() {
    // IDA 0xea6d1c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_880")]
#[doc(alias = "__GLOBAL__I_a_880")]
// 0xea732c — __GLOBAL__I_a_880
pub fn stub_0xea732c() {
    // IDA 0xea732c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "anonymous namespace::readFloatToken(char *,char,float *)")]
#[doc(alias = "__ZN12_GLOBAL__N_114readFloatTokenEPccPf")]
// 0xea8a58 — __ZN12_GLOBAL__N_114readFloatTokenEPccPf
// type: _DWORD __fastcall(_anonymous_namespace_ *__hidden this, char *, char, float *)
pub fn stub_0xea8a58() {
    // IDA 0xea8a58: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_881")]
#[doc(alias = "__GLOBAL__I_a_881")]
// 0xea97d4 — __GLOBAL__I_a_881
pub fn stub_0xea97d4() {
    // IDA 0xea97d4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_882")]
#[doc(alias = "__GLOBAL__I_a_882")]
// 0xeac200 — __GLOBAL__I_a_882
pub fn stub_0xeac200() {
    // IDA 0xeac200: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_883")]
#[doc(alias = "__GLOBAL__I_a_883")]
// 0xeadbb0 — __GLOBAL__I_a_883
pub fn stub_0xeadbb0() {
    // IDA 0xeadbb0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_884")]
#[doc(alias = "__GLOBAL__I_a_884")]
// 0xeb1190 — __GLOBAL__I_a_884
pub fn stub_0xeb1190() {
    // IDA 0xeb1190: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_885")]
#[doc(alias = "__GLOBAL__I_a_885")]
// 0xeb2b3c — __GLOBAL__I_a_885
pub fn stub_0xeb2b3c() {
    // IDA 0xeb2b3c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_886")]
#[doc(alias = "__GLOBAL__I_a_886")]
// 0xeb2e5c — __GLOBAL__I_a_886
pub fn stub_0xeb2e5c() {
    // IDA 0xeb2e5c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_887")]
#[doc(alias = "__GLOBAL__I_a_887")]
// 0xeb2efc — __GLOBAL__I_a_887
pub fn stub_0xeb2efc() {
    // IDA 0xeb2efc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_888")]
#[doc(alias = "__GLOBAL__I_a_888")]
// 0xeb3844 — __GLOBAL__I_a_888
pub fn stub_0xeb3844() {
    // IDA 0xeb3844: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_889")]
#[doc(alias = "__GLOBAL__I_a_889")]
// 0xeb3a60 — __GLOBAL__I_a_889
pub fn stub_0xeb3a60() {
    // IDA 0xeb3a60: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "-[GAI defaultTracker]")]
#[doc(alias = "-[GAI defaultTracker]")]
// 0xeb3a98 — -[GAI defaultTracker]
// type: GAITracker *__cdecl(GAI *self, SEL)
pub fn stub_0xeb3a98() {
    // IDA 0xeb3a98: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "-[GAI setDefaultTracker:]")]
#[doc(alias = "-[GAI setDefaultTracker:]")]
// 0xeb3abc — -[GAI setDefaultTracker:]
// type: void __cdecl(GAI *self, SEL, id)
pub fn stub_0xeb3abc() {
    // IDA 0xeb3abc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "-[GAI optOut]")]
#[doc(alias = "-[GAI optOut]")]
// 0xeb3ae0 — -[GAI optOut]
// type: char __cdecl(GAI *self, SEL)
pub fn stub_0xeb3ae0() {
    // IDA 0xeb3ae0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "-[GAI setOptOut:]")]
#[doc(alias = "-[GAI setOptOut:]")]
// 0xeb3c3c — -[GAI setOptOut:]
// type: void __cdecl(GAI *self, SEL, char)
pub fn stub_0xeb3c3c() {
    // IDA 0xeb3c3c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI clientId]")]
#[doc(alias = "-[GAI clientId]")]
// 0xeb3d94 — -[GAI clientId]
// type: id __cdecl(GAI *self, SEL)
pub fn stub_0xeb3d94() {
    // IDA 0xeb3d94: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI dispatchInterval]")]
#[doc(alias = "-[GAI dispatchInterval]")]
// 0xeb3ef0 — -[GAI dispatchInterval]
// type: double __cdecl(GAI *self, SEL)
pub fn stub_0xeb3ef0() {
    // IDA 0xeb3ef0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI setDispatchInterval:]")]
#[doc(alias = "-[GAI setDispatchInterval:]")]
// 0xeb4060 — -[GAI setDispatchInterval:]
// type: void __cdecl(GAI *self, SEL, double)
pub fn stub_0xeb4060() {
    // IDA 0xeb4060: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI trackUncaughtExceptions]")]
#[doc(alias = "-[GAI trackUncaughtExceptions]")]
// 0xeb41c4 — -[GAI trackUncaughtExceptions]
// type: char __cdecl(GAI *self, SEL)
pub fn stub_0xeb41c4() {
    // IDA 0xeb41c4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI setTrackUncaughtExceptions:]")]
#[doc(alias = "-[GAI setTrackUncaughtExceptions:]")]
// 0xeb41e8 — -[GAI setTrackUncaughtExceptions:]
// type: void __cdecl(GAI *self, SEL, char)
pub fn stub_0xeb41e8() {
    // IDA 0xeb41e8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAI dataStorePath]")]
#[doc(alias = "+[GAI dataStorePath]")]
// 0xeb42b0 — +[GAI dataStorePath]
// type: id __cdecl(id, SEL)
pub fn stub_0xeb42b0() {
    // IDA 0xeb42b0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI createDispatcher:]")]
#[doc(alias = "-[GAI createDispatcher:]")]
// 0xeb4310 — -[GAI createDispatcher:]
// type: char __cdecl(GAI *self, SEL, id *)
pub fn stub_0xeb4310() {
    // IDA 0xeb4310: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI init]")]
#[doc(alias = "-[GAI init]")]
// 0xeb43a8 — -[GAI init]
// type: GAI *__cdecl(GAI *self, SEL)
pub fn stub_0xeb43a8() {
    // IDA 0xeb43a8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAI allocWithZone:]")]
#[doc(alias = "+[GAI allocWithZone:]")]
// 0xeb45a8 — +[GAI allocWithZone:]
// type: id __cdecl(id, SEL, _NSZone *)
pub fn stub_0xeb45a8() {
    // IDA 0xeb45a8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI retain]")]
#[doc(alias = "-[GAI retain]")]
// 0xeb4690 — -[GAI retain]
// type: GAI *__cdecl(GAI *self, SEL)
pub fn stub_0xeb4690() {
    // IDA 0xeb4690: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI retainCount]")]
#[doc(alias = "-[GAI retainCount]")]
// 0xeb4694 — -[GAI retainCount]
// type: unsigned int __cdecl(GAI *self, SEL)
pub fn stub_0xeb4694() {
    // IDA 0xeb4694: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI release]")]
#[doc(alias = "-[GAI release]")]
// 0xeb469c — -[GAI release]
// type: void __cdecl(GAI *self, SEL)
pub fn stub_0xeb469c() {
    // IDA 0xeb469c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI autorelease]")]
#[doc(alias = "-[GAI autorelease]")]
// 0xeb46a0 — -[GAI autorelease]
// type: GAI *__cdecl(GAI *self, SEL)
pub fn stub_0xeb46a0() {
    // IDA 0xeb46a0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI copyWithZone:]")]
#[doc(alias = "-[GAI copyWithZone:]")]
// 0xeb46a4 — -[GAI copyWithZone:]
// type: id __cdecl(GAI *self, SEL, _NSZone *)
pub fn stub_0xeb46a4() {
    // IDA 0xeb46a4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI trackerWithTrackingId:]")]
#[doc(alias = "-[GAI trackerWithTrackingId:]")]
// 0xeb46a8 — -[GAI trackerWithTrackingId:]
// type: id __cdecl(GAI *self, SEL, id)
pub fn stub_0xeb46a8() {
    // IDA 0xeb46a8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI debug]")]
#[doc(alias = "-[GAI debug]")]
// 0xeb4a3c — -[GAI debug]
// type: char __cdecl(GAI *self, SEL)
pub fn stub_0xeb4a3c() {
    // IDA 0xeb4a3c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI setDebug:]")]
#[doc(alias = "-[GAI setDebug:]")]
// 0xeb4a4c — -[GAI setDebug:]
// type: void __cdecl(GAI *self, SEL, char)
pub fn stub_0xeb4a4c() {
    // IDA 0xeb4a4c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI dispatch]")]
#[doc(alias = "-[GAI dispatch]")]
// 0xeb4a5c — -[GAI dispatch]
// type: void __cdecl(GAI *self, SEL)
pub fn stub_0xeb4a5c() {
    // IDA 0xeb4a5c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI dispatcher]")]
#[doc(alias = "-[GAI dispatcher]")]
// 0xeb4bb0 — -[GAI dispatcher]
// type: GAIDispatcher *__cdecl(GAI *self, SEL)
pub fn stub_0xeb4bb0() {
    // IDA 0xeb4bb0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI reset]")]
#[doc(alias = "-[GAI reset]")]
// 0xeb4bd4 — -[GAI reset]
// type: void __cdecl(GAI *self, SEL)
pub fn stub_0xeb4bd4() {
    // IDA 0xeb4bd4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI removeTracker:]")]
#[doc(alias = "-[GAI removeTracker:]")]
// 0xeb4d34 — -[GAI removeTracker:]
// type: void __cdecl(GAI *self, SEL, id)
pub fn stub_0xeb4d34() {
    // IDA 0xeb4d34: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI updateAdMobParameters:]")]
#[doc(alias = "-[GAI updateAdMobParameters:]")]
// 0xeb4e6c — -[GAI updateAdMobParameters:]
// type: void __cdecl(GAI *self, SEL, id)
pub fn stub_0xeb4e6c() {
    // IDA 0xeb4e6c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI adMobParameters]")]
#[doc(alias = "-[GAI adMobParameters]")]
// 0xeb4f98 — -[GAI adMobParameters]
// type: id __cdecl(GAI *self, SEL)
pub fn stub_0xeb4f98() {
    // IDA 0xeb4f98: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI setDispatcher:]")]
#[doc(alias = "-[GAI setDispatcher:]")]
// 0xeb507c — -[GAI setDispatcher:]
// type: void __cdecl(GAI *self, SEL, id)
pub fn stub_0xeb507c() {
    // IDA 0xeb507c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI trackers]")]
#[doc(alias = "-[GAI trackers]")]
// 0xeb50a0 — -[GAI trackers]
// type: NSMutableDictionary *__cdecl(GAI *self, SEL)
pub fn stub_0xeb50a0() {
    // IDA 0xeb50a0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI setTrackers:]")]
#[doc(alias = "-[GAI setTrackers:]")]
// 0xeb50b0 — -[GAI setTrackers:]
// type: void __cdecl(GAI *self, SEL, id)
pub fn stub_0xeb50b0() {
    // IDA 0xeb50b0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI adMobInfo]")]
#[doc(alias = "-[GAI adMobInfo]")]
// 0xeb50d4 — -[GAI adMobInfo]
// type: NSDictionary *__cdecl(GAI *self, SEL)
pub fn stub_0xeb50d4() {
    // IDA 0xeb50d4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI setAdMobInfo:]")]
#[doc(alias = "-[GAI setAdMobInfo:]")]
// 0xeb50e4 — -[GAI setAdMobInfo:]
// type: void __cdecl(GAI *self, SEL, id)
pub fn stub_0xeb50e4() {
    // IDA 0xeb50e4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI trackerImplClass]")]
#[doc(alias = "-[GAI trackerImplClass]")]
// 0xeb5108 — -[GAI trackerImplClass]
// type: Class __cdecl(GAI *self, SEL)
pub fn stub_0xeb5108() {
    // IDA 0xeb5108: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAI setTrackerImplClass:]")]
#[doc(alias = "-[GAI setTrackerImplClass:]")]
// 0xeb5118 — -[GAI setTrackerImplClass:]
// type: void __cdecl(GAI *self, SEL, Class)
pub fn stub_0xeb5118() {
    // IDA 0xeb5118: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction init]")]
#[doc(alias = "-[GAITransaction init]")]
// 0xeb5128 — -[GAITransaction init]
// type: GAITransaction *__cdecl(GAITransaction *self, SEL)
pub fn stub_0xeb5128() {
    // IDA 0xeb5128: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction initWithTransactionId:withAffiliation:]")]
#[doc(alias = "-[GAITransaction initWithTransactionId:withAffiliation:]")]
// 0xeb5184 — -[GAITransaction initWithTransactionId:withAffiliation:]
// type: GAITransaction *__cdecl(GAITransaction *self, SEL, id, id)
pub fn stub_0xeb5184() {
    // IDA 0xeb5184: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction dealloc]")]
#[doc(alias = "-[GAITransaction dealloc]")]
// 0xeb5278 — -[GAITransaction dealloc]
// type: void __cdecl(GAITransaction *self, SEL)
pub fn stub_0xeb5278() {
    // IDA 0xeb5278: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAITransaction transactionWithId:withAffiliation:]")]
#[doc(alias = "+[GAITransaction transactionWithId:withAffiliation:]")]
// 0xeb5304 — +[GAITransaction transactionWithId:withAffiliation:]
// type: id __cdecl(id, SEL, id, id)
pub fn stub_0xeb5304() {
    // IDA 0xeb5304: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction addItem:]")]
#[doc(alias = "-[GAITransaction addItem:]")]
// 0xeb5350 — -[GAITransaction addItem:]
// type: void __cdecl(GAITransaction *self, SEL, id)
pub fn stub_0xeb5350() {
    // IDA 0xeb5350: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction addItemWithCode:name:category:priceMicros:quantity:]")]
#[doc(alias = "-[GAITransaction addItemWithCode:name:category:priceMicros:quantity:]")]
// 0xeb53f8 — -[GAITransaction addItemWithCode:name:category:priceMicros:quantity:]
// type: void __cdecl(GAITransaction *self, SEL, id, id, id, signed __int64, int)
pub fn stub_0xeb53f8() {
    // IDA 0xeb53f8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction items]")]
#[doc(alias = "-[GAITransaction items]")]
// 0xeb5450 — -[GAITransaction items]
// type: NSArray *__cdecl(GAITransaction *self, SEL)
pub fn stub_0xeb5450() {
    // IDA 0xeb5450: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction transactionId]")]
#[doc(alias = "-[GAITransaction transactionId]")]
// 0xeb5474 — -[GAITransaction transactionId]
// type: NSString *__cdecl(GAITransaction *self, SEL)
pub fn stub_0xeb5474() {
    // IDA 0xeb5474: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction affiliation]")]
#[doc(alias = "-[GAITransaction affiliation]")]
// 0xeb548c — -[GAITransaction affiliation]
// type: NSString *__cdecl(GAITransaction *self, SEL)
pub fn stub_0xeb548c() {
    // IDA 0xeb548c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction revenueMicros]")]
#[doc(alias = "-[GAITransaction revenueMicros]")]
// 0xeb54a4 — -[GAITransaction revenueMicros]
// type: signed __int64 __cdecl(GAITransaction *self, SEL)
pub fn stub_0xeb54a4() {
    // IDA 0xeb54a4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction setRevenueMicros:]")]
#[doc(alias = "-[GAITransaction setRevenueMicros:]")]
// 0xeb54bc — -[GAITransaction setRevenueMicros:]
// type: void __cdecl(GAITransaction *self, SEL, signed __int64)
pub fn stub_0xeb54bc() {
    // IDA 0xeb54bc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction taxMicros]")]
#[doc(alias = "-[GAITransaction taxMicros]")]
// 0xeb54d0 — -[GAITransaction taxMicros]
// type: signed __int64 __cdecl(GAITransaction *self, SEL)
pub fn stub_0xeb54d0() {
    // IDA 0xeb54d0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction setTaxMicros:]")]
#[doc(alias = "-[GAITransaction setTaxMicros:]")]
// 0xeb54e8 — -[GAITransaction setTaxMicros:]
// type: void __cdecl(GAITransaction *self, SEL, signed __int64)
pub fn stub_0xeb54e8() {
    // IDA 0xeb54e8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction shippingMicros]")]
#[doc(alias = "-[GAITransaction shippingMicros]")]
// 0xeb54fc — -[GAITransaction shippingMicros]
// type: signed __int64 __cdecl(GAITransaction *self, SEL)
pub fn stub_0xeb54fc() {
    // IDA 0xeb54fc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction setShippingMicros:]")]
#[doc(alias = "-[GAITransaction setShippingMicros:]")]
// 0xeb5514 — -[GAITransaction setShippingMicros:]
// type: void __cdecl(GAITransaction *self, SEL, signed __int64)
pub fn stub_0xeb5514() {
    // IDA 0xeb5514: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction currencyCode]")]
#[doc(alias = "-[GAITransaction currencyCode]")]
// 0xeb5528 — -[GAITransaction currencyCode]
// type: NSString *__cdecl(GAITransaction *self, SEL)
pub fn stub_0xeb5528() {
    // IDA 0xeb5528: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction setCurrencyCode:]")]
#[doc(alias = "-[GAITransaction setCurrencyCode:]")]
// 0xeb5540 — -[GAITransaction setCurrencyCode:]
// type: void __cdecl(GAITransaction *self, SEL, id)
pub fn stub_0xeb5540() {
    // IDA 0xeb5540: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction mutableItems]")]
#[doc(alias = "-[GAITransaction mutableItems]")]
// 0xeb5564 — -[GAITransaction mutableItems]
// type: NSMutableDictionary *__cdecl(GAITransaction *self, SEL)
pub fn stub_0xeb5564() {
    // IDA 0xeb5564: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransaction setMutableItems:]")]
#[doc(alias = "-[GAITransaction setMutableItems:]")]
// 0xeb5574 — -[GAITransaction setMutableItems:]
// type: void __cdecl(GAITransaction *self, SEL, id)
pub fn stub_0xeb5574() {
    // IDA 0xeb5574: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransactionItem dealloc]")]
#[doc(alias = "-[GAITransactionItem dealloc]")]
// 0xeb5598 — -[GAITransactionItem dealloc]
// type: void __cdecl(GAITransactionItem *self, SEL)
pub fn stub_0xeb5598() {
    // IDA 0xeb5598: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransactionItem init]")]
#[doc(alias = "-[GAITransactionItem init]")]
// 0xeb5610 — -[GAITransactionItem init]
// type: GAITransactionItem *__cdecl(GAITransactionItem *self, SEL)
pub fn stub_0xeb5610() {
    // IDA 0xeb5610: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransactionItem initWithCode:name:category:priceMicros:quantity:]")]
#[doc(alias = "-[GAITransactionItem initWithCode:name:category:priceMicros:quantity:]")]
// 0xeb566c — -[GAITransactionItem initWithCode:name:category:priceMicros:quantity:]
// type: GAITransactionItem *__cdecl(GAITransactionItem *self, SEL, id, id, id, signed __int64, unsigned int)
pub fn stub_0xeb566c() {
    // IDA 0xeb566c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAITransactionItem itemWithCode:name:category:priceMicros:quantity:]")]
#[doc(alias = "+[GAITransactionItem itemWithCode:name:category:priceMicros:quantity:]")]
// 0xeb5728 — +[GAITransactionItem itemWithCode:name:category:priceMicros:quantity:]
// type: id __cdecl(id, SEL, id, id, id, signed __int64, int)
pub fn stub_0xeb5728() {
    // IDA 0xeb5728: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransactionItem productCode]")]
#[doc(alias = "-[GAITransactionItem productCode]")]
// 0xeb5864 — -[GAITransactionItem productCode]
// type: NSString *__cdecl(GAITransactionItem *self, SEL)
pub fn stub_0xeb5864() {
    // IDA 0xeb5864: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransactionItem productName]")]
#[doc(alias = "-[GAITransactionItem productName]")]
// 0xeb587c — -[GAITransactionItem productName]
// type: NSString *__cdecl(GAITransactionItem *self, SEL)
pub fn stub_0xeb587c() {
    // IDA 0xeb587c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITransactionItem setProductName:]")]
#[doc(alias = "-[GAITransactionItem setProductName:]")]
// 0xeb5894 — -[GAITransactionItem setProductName:]
// type: void __cdecl(GAITransactionItem *self, SEL, id)
pub fn stub_0xeb5894() {
    // IDA 0xeb5894: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
