//! network generated_bg_7 — RakNet + RBX::Network + Replicator (auto-generated, do not edit manually)
//! Global gap filler bg_7 100 stubs 0x2684e8..0x26cb98 EA-sorted asc next 100 (RakNet|Network|Replicat|Socket|Upnp|HTTP 6232/6232 complete, 26099->26199 network distinct, rbx_core::SharedPtr not boost) [skeleton batch]

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::collections::{HashMap, HashSet};
/// `RBX::Lua::LuaArguments` value mirror for typed stack getters (IDA 0x26b464 et al.).
#[derive(Debug, Clone)]
pub enum LuaArg {
    Str(String),
    Num(f64),
    Bool(bool),
    Vec3([f32; 3]),
    Region3([f32; 3], [f32; 3]),
    Object(SharedPtr<()>),
    Enum(i32),
}
/// Lazily-initialized script `ClassDescriptor` mirror (cf. 0x26aae4 shape).
#[derive(Debug)]
pub struct ScriptClassDescriptor {
    pub name: &'static str,
    pub base: &'static str,
}

use std::sync::{Mutex, OnceLock};

use rbx_core::SharedPtr;

/// `RBX::Reflection::Type` constructor parts (IDA 0x26867c et al.):
/// Descriptor base init, typeinfo tag, `Name::lookup`, non-empty assert
/// (type.h:66) and `addToAllTypes`. Reflection tables stay engine-side.
#[derive(Debug, Clone)]
pub struct ReflectionTypeInit {
    pub tag: String,
}

/// `RBX::CoreScript::onServiceProvider` decision (IDA 0x268eec): the
/// ScriptContext lookup + `hasScript` assert outcome; context juggling
/// stays engine-side.
#[derive(Debug, PartialEq, Eq)]
pub enum ServiceProviderEffect {
    Passthrough,
    Reparented,
}

/// `RBX::CoreScript::requestCode` outcome (IDA 0x268ffc): cached
/// `ProtectedString` source or the `BaseScript::requestCode` fallback.
#[derive(Debug, PartialEq, Eq)]
pub enum RequestCodeSource {
    Cached,
    BaseFallback,
}

fn script_class_name(cell: &'static OnceLock<&'static str>, name: &'static str) -> &'static str {
    *cell.get_or_init(|| name)
}



// 0x2684e8 — __ZN3rbx8any_castIN5boost10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEENS4_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> * rbx::any_cast<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_2684e8(type_matches: bool, payload: usize) -> Option<usize> {
    // IDA 0x2684e8: returns payload + 1 on typeinfo match (cf. 0x2684ea..0x26851c), null otherwise (0x26853c).
    type_matches.then_some(payload + 1)
}


// 0x268540 — __ZN3RBX10Reflection5TTypeIvED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<void>::~TType()")]
pub fn stub_268540() {
    // IDA 0x268540: TType vtable reset + base destroy; static-type teardown.
    // was: RBX::Reflection::TType<...>::~TType.
}


// 0x268544 — __ZNSt6vectorIPKN3RBX10Reflection4TypeESaIS4_EED1Ev
// type: void **__fastcall(void **)
#[doc(alias = "std::vector<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>::~vector()")]
pub fn stub_268544(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0x268544: base destroy body.
    destroy_instance(this);
}


// 0x268558 — __ZNSt6vectorIPKN3RBX10Reflection4TypeESaIS4_EE9push_backERKS4_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>::push_back(RBX::Reflection::Type const* const&)")]
pub fn stub_268558(vec: &mut Vec<usize>, value: usize) {
    // IDA 0x268558: appends, growing via _M_insert_aux at capacity.
    // was: std::vector<...>::push_back.
    vec.push(value);
}


// 0x268584 — __ZNSt6vectorIPKN3RBX10Reflection4TypeESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::Type const**,std::vector<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>>,RBX::Reflection::Type const* const&)")]
pub fn stub_268584(vec: &mut Vec<usize>, pos: usize, value: usize) {
    // IDA 0x268584: doubling realloc insert with memmove.
    // was: std::vector<...>::_M_insert_aux.
    let pos = pos.min(vec.len());
    vec.insert(pos, value);
}


// 0x268664 — __ZNSt12_Vector_baseIPKN3RBX10Reflection4TypeESaIS4_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>::_M_allocate(unsigned long)")]
pub fn stub_268664(capacity: usize) -> Vec<usize> {
    // IDA 0x268664: raw allocate for capacity elements.
    // was: std::_Vector_base<...>::_M_allocate.
    Vec::with_capacity(capacity)
}


// 0x26867c — __ZN3RBX10Reflection4TypeC2IvEEPKcPT_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::Type::Type<void>(char const*,void *)")]
pub fn stub_26867c(tag: &str, lookup: &mut dyn FnMut(&str) -> usize, register: &mut dyn FnMut()) -> ReflectionTypeInit {
    // IDA 0x26867c: Descriptor base init, typeinfo tag, Name::lookup, assert !tag.empty() (type.h:66), addToAllTypes (cf. 0x26868a..0x268724).
    let found = lookup(tag);
    assert!(found != 0, "!this->tag.empty() type.h:66");
    register();
    ReflectionTypeInit { tag: "void".to_owned() }
}


// 0x268728 — __ZN3RBX10Reflection5TTypeIvED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TType<void>::~TType()")]
pub fn stub_268728(free: &mut dyn FnMut()) {
    // IDA 0x268728: TType teardown then operator delete.
    // was: RBX::Reflection::TType<...>::~TType D0.
    free();
}


// 0x26872c — __ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS6_EEEEEEEPKcPT_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::Type::Type<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(char const*,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> *)")]
pub fn stub_26872c(tag: &str, lookup: &mut dyn FnMut(&str) -> usize, register: &mut dyn FnMut()) -> ReflectionTypeInit {
    // IDA 0x26872c: Descriptor base init, typeinfo tag, Name::lookup, assert !tag.empty() (type.h:66), addToAllTypes (cf. 0x26868a..0x268724).
    let found = lookup(tag);
    assert!(found != 0, "!this->tag.empty() type.h:66");
    register();
    ReflectionTypeInit { tag: "Map".to_owned() }
}


// 0x2687d8 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TType<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::~TType()")]
pub fn stub_2687d8(free: &mut dyn FnMut()) {
    // IDA 0x2687d8: TType teardown then operator delete.
    // was: RBX::Reflection::TType<...>::~TType D0.
    free();
}


// 0x2687dc — __ZN5boost10shared_ptrIKNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS5_EEEEEC2ISE_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *)")]
pub fn stub_2687dc<T>(value: T) -> SharedPtr<T> {
    // IDA 0x2687dc: takes raw ownership + allocates the counted impl (counts 1,1).
    // was: boost::shared_ptr<...>::shared_ptr<T*> → SharedPtr (Arc).
    SharedPtr::new(value)
}


// 0x2688b0 — __ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEEPKcPT_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::Type::Type<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(char const*,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> *)")]
pub fn stub_2688b0(tag: &str, lookup: &mut dyn FnMut(&str) -> usize, register: &mut dyn FnMut()) -> ReflectionTypeInit {
    // IDA 0x2688b0: Descriptor base init, typeinfo tag, Name::lookup, assert !tag.empty() (type.h:66), addToAllTypes (cf. 0x26868a..0x268724).
    let found = lookup(tag);
    assert!(found != 0, "!this->tag.empty() type.h:66");
    register();
    ReflectionTypeInit { tag: "Dictionary".to_owned() }
}


// 0x26895c — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsNS0_7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEEEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TType<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::~TType()")]
pub fn stub_26895c(free: &mut dyn FnMut()) {
    // IDA 0x26895c: TType teardown then operator delete.
    // was: RBX::Reflection::TType<...>::~TType D0.
    free();
}


// 0x268960 — __ZN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS4_EEEC2IS6_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)")]
pub fn stub_268960<T>(value: T) -> SharedPtr<T> {
    // IDA 0x268960: takes raw ownership + allocates the counted impl (counts 1,1).
    // was: boost::shared_ptr<...>::shared_ptr<T*> → SharedPtr (Arc).
    SharedPtr::new(value)
}


// 0x268a34 — __ZN5boost6detail12shared_countC2ISt6vectorIN3RBX10Reflection7VariantESaIS6_EEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)")]
pub fn stub_268a34<T>(value: T) -> SharedPtr<T> {
    // IDA 0x268a34: operator new the counted impl with use/weak counts 1,1 (cf. 0x268a60..0x268aa8).
    // was: boost::detail::shared_count<...> ctor → SharedPtr (Arc).
    SharedPtr::new(value)
}


// 0x268b40 — __ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEEEPKcPT_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::Type::Type<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(char const*,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> *)")]
pub fn stub_268b40(tag: &str, lookup: &mut dyn FnMut(&str) -> usize, register: &mut dyn FnMut()) -> ReflectionTypeInit {
    // IDA 0x268b40: Descriptor base init, typeinfo tag, Name::lookup, assert !tag.empty() (type.h:66), addToAllTypes (cf. 0x26868a..0x268724).
    let found = lookup(tag);
    assert!(found != 0, "!this->tag.empty() type.h:66");
    register();
    ReflectionTypeInit { tag: "Array".to_owned() }
}


// 0x268bec — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS5_EEEEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TType<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::~TType()")]
pub fn stub_268bec(free: &mut dyn FnMut()) {
    // IDA 0x268bec: TType teardown then operator delete.
    // was: RBX::Reflection::TType<...>::~TType D0.
    free();
}


// 0x268cb8 — __ZN3RBX10CoreScriptC1ERKNS_9ContentIdE
// type: int __fastcall(RBX::CoreScript *this, const RBX::ContentId *)
#[doc(alias = "RBX::CoreScript::CoreScript(RBX::ContentId const&)")]
pub fn stub_268cb8(content: &str, init_base: &mut dyn FnMut(&str)) {
    // IDA 0x268cb8: C1 delegates to C2 (cf. 0x268cb8 / 0x269da0).
    init_base(content);
}


// 0x268cbc — __ZN3RBX10CoreScriptC2ERKNS_9ContentIdE
// type: RBX::BaseScript *__fastcall(RBX::CoreScript *this, __guard *)
#[doc(alias = "RBX::CoreScript::CoreScript(RBX::ContentId const&)")]
pub fn stub_268cbc(content: &str, init_base: &mut dyn FnMut(&str)) {
    // IDA 0x268cbc: BaseScript init + vtable install + source setup.
    init_base(content);
}


// 0x268eec — __ZN3RBX10CoreScript17onServiceProviderEPNS_15ServiceProviderES2_
// type: int __fastcall(RBX::CoreScript *this, RBX::ServiceProvider *, RBX::ServiceProvider *, int)
#[doc(alias = "RBX::CoreScript::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_268eec(
    slot_occupied: bool,
    find_context: &mut dyn FnMut() -> bool,
    has_script: &mut dyn FnMut() -> bool,
    remove_and_forward: &mut dyn FnMut(),
) -> ServiceProviderEffect {
    // IDA 0x268eec: with a provider and a clear slot: find ScriptContext, assert non-null ("sc", CoreScript.cpp:32, cf. 0x268f12..0x268f66), removeScript + assert hasScript (:34, 0x268f70..0x268fdc), then BaseScript::onServiceProvider (0x268fe4).
    if !slot_occupied {
        return ServiceProviderEffect::Passthrough;
    }
    assert!(find_context(), "sc CoreScript.cpp:32");
    assert!(has_script(), "sc->hasScript(this) CoreScript.cpp:34");
    remove_and_forward();
    ServiceProviderEffect::Reparented
}


// 0x268ffc — __ZN3RBX10CoreScript11requestCodeEPNS_25ScriptInformationProviderE
// type: int __fastcall(RBX::BaseScript *, RBX::Instance *, int)
#[doc(alias = "RBX::CoreScript::requestCode(RBX::ScriptInformationProvider *)")]
pub fn stub_268ffc(cached_source: Option<String>, fallback: &mut dyn FnMut() -> String) -> (RequestCodeSource, String) {
    // IDA 0x268ffc: uses the cached ProtectedString flyweight source when present (cf. 0x2691e8..0x2691f4), else BaseScript::requestCode (0x2692ea).
    match cached_source {
        Some(source) => (RequestCodeSource::Cached, source),
        None => (RequestCodeSource::BaseFallback, fallback()),
    }
}


// 0x26973c — __ZN3RBX10CoreScript19extraErrorReportingEP9lua_State
// type: int __fastcall(RBX::DataModel *, int)
#[doc(alias = "RBX::CoreScript::extraErrorReporting(lua_State *)")]
pub fn stub_26973c(source_is_self: bool, place_id: Option<i32>, write_log: &mut dyn FnMut(&str)) -> String {
    // IDA 0x26973c: asserts source.get() == this (CoreScript.cpp:95), appends "\nPlaceID: <id>" or "\nError finding PlaceID!" (cf. 0x2698c6..0x269918), writes <userdir>/logs/*_ln*.cse (0x269952..0x269a62).
    assert!(source_is_self, "source.get() == this CoreScript.cpp:95");
    let report = match place_id {
        Some(id) => format!("\nPlaceID: {id}"),
        None => "\nError finding PlaceID!".to_owned(),
    };
    write_log(&report);
    report
}


// 0x269da0 — __ZN3RBX13StarterScriptC1ERKNS_9ContentIdE
// type: int __fastcall(RBX::StarterScript *this, const RBX::ContentId *)
#[doc(alias = "RBX::StarterScript::StarterScript(RBX::ContentId const&)")]
pub fn stub_269da0(content: &str, init_base: &mut dyn FnMut(&str)) {
    // IDA 0x269da0: C1 delegates to C2 (cf. 0x268cb8 / 0x269da0).
    init_base(content);
}


// 0x269da4 — __ZN3RBX13StarterScriptC2ERKNS_9ContentIdE
// type: RBX::BaseScript *__fastcall(RBX::StarterScript *this, const RBX::ContentId *)
#[doc(alias = "RBX::StarterScript::StarterScript(RBX::ContentId const&)")]
pub fn stub_269da4(content: &str, init_base: &mut dyn FnMut(&str)) {
    // IDA 0x269da4: BaseScript init + vtable install + source setup.
    init_base(content);
}


// 0x26a060 — __ZN3RBX10CoreScriptD1Ev
// type: void __fastcall(RBX::CoreScript *__hidden this)
#[doc(alias = "RBX::CoreScript::~CoreScript()")]
pub fn stub_26a060(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0x26a060: base destroy body.
    destroy_instance(this);
}


// 0x26a064 — __ZN3RBX10CoreScriptD0Ev
// type: void __fastcall(RBX::CoreScript *__hidden this)
#[doc(alias = "RBX::CoreScript::~CoreScript()")]
pub fn stub_26a064(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x26a064: base destroy then operator delete.
    destroy_instance(this);
    free(this);
}


// 0x26a104 — __ZNK3RBX17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEE12getClassNameEv")]
pub fn stub_26a104() -> &'static str {
    // IDA 0x26a104: call_once declare + return the class name.
    static CELL: OnceLock<&'static str> = OnceLock::new();
    script_class_name(&CELL, "CoreScript")
}


// 0x26a12c — __ZThn32_N3RBX10CoreScriptD1Ev
// type: void __fastcall(RBX::CoreScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CoreScript::~CoreScript()")]
pub fn stub_26a12c(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x26a12c: this-32 adjust then tail-call the primary dtor.
    destroy_at(this - 32);
}


// 0x26a134 — __ZThn32_N3RBX10CoreScriptD0Ev
// type: void __fastcall(RBX::CoreScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CoreScript::~CoreScript()")]
pub fn stub_26a134(this: usize, destroy_at: &mut dyn FnMut(usize), free_at: &mut dyn FnMut(usize)) {
    // IDA 0x26a134: this-32 adjust then tail-call the primary dtor + delete.
    destroy_at(this - 32);
    free_at(this - 32);
}


// 0x26a1d8 — __ZThn32_NK3RBX17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEE12getClassNameEv")]
pub fn stub_26a1d8() -> &'static str {
    // IDA 0x26a1d8: Thn32 into the primary getClassName (0x26a104).
    stub_26a104()
}


// 0x26a200 — __ZThn36_N3RBX10CoreScriptD1Ev
// type: void __fastcall(RBX::CoreScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CoreScript::~CoreScript()")]
pub fn stub_26a200(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x26a200: this-36 adjust then tail-call the primary dtor.
    destroy_at(this - 36);
}


// 0x26a208 — __ZThn36_N3RBX10CoreScriptD0Ev
// type: void __fastcall(RBX::CoreScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CoreScript::~CoreScript()")]
pub fn stub_26a208(this: usize, destroy_at: &mut dyn FnMut(usize), free_at: &mut dyn FnMut(usize)) {
    // IDA 0x26a208: this-36 adjust then tail-call the primary dtor + delete.
    destroy_at(this - 36);
    free_at(this - 36);
}


// 0x26a2ac — __ZN3RBX13StarterScriptD1Ev
// type: void __fastcall(RBX::StarterScript *__hidden this)
#[doc(alias = "RBX::StarterScript::~StarterScript()")]
pub fn stub_26a2ac(this: usize, destroy_instance: &mut dyn FnMut(usize)) {
    // IDA 0x26a2ac: base destroy body.
    destroy_instance(this);
}


// 0x26a2b0 — __ZN3RBX13StarterScriptD0Ev
// type: void __fastcall(RBX::StarterScript *__hidden this)
#[doc(alias = "RBX::StarterScript::~StarterScript()")]
pub fn stub_26a2b0(this: usize, destroy_instance: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x26a2b0: base destroy then operator delete.
    destroy_instance(this);
    free(this);
}


// 0x26a350 — __ZNK3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEE12getClassNameEv")]
pub fn stub_26a350() -> &'static str {
    // IDA 0x26a350: call_once declare + return the class name.
    static CELL: OnceLock<&'static str> = OnceLock::new();
    script_class_name(&CELL, "StarterScript")
}


// 0x26a378 — __ZThn32_N3RBX13StarterScriptD1Ev
// type: void __fastcall(RBX::StarterScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::StarterScript::~StarterScript()")]
pub fn stub_26a378(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x26a378: this-32 adjust then tail-call the primary dtor.
    destroy_at(this - 32);
}


// 0x26a380 — __ZThn32_N3RBX13StarterScriptD0Ev
// type: void __fastcall(RBX::StarterScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::StarterScript::~StarterScript()")]
pub fn stub_26a380(this: usize, destroy_at: &mut dyn FnMut(usize), free_at: &mut dyn FnMut(usize)) {
    // IDA 0x26a380: this-32 adjust then tail-call the primary dtor + delete.
    destroy_at(this - 32);
    free_at(this - 32);
}


// 0x26a424 — __ZThn32_NK3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEE12getClassNameEv")]
pub fn stub_26a424() -> &'static str {
    // IDA 0x26a424: Thn32 into the primary getClassName (0x26a350).
    stub_26a350()
}


// 0x26a44c — __ZThn36_N3RBX13StarterScriptD1Ev
// type: void __fastcall(RBX::StarterScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::StarterScript::~StarterScript()")]
pub fn stub_26a44c(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x26a44c: this-36 adjust then tail-call the primary dtor.
    destroy_at(this - 36);
}


// 0x26a454 — __ZThn36_N3RBX13StarterScriptD0Ev
// type: void __fastcall(RBX::StarterScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::StarterScript::~StarterScript()")]
pub fn stub_26a454(this: usize, destroy_at: &mut dyn FnMut(usize), free_at: &mut dyn FnMut(usize)) {
    // IDA 0x26a454: this-36 adjust then tail-call the primary dtor + delete.
    destroy_at(this - 36);
    free_at(this - 36);
}


// 0x26a4f8 — __ZN3RBX4Name13callDoDeclareILZNS_14sStarterScriptEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sStarterScriptEEEEvv")]
pub fn stub_26a4f8() -> &'static str {
    // IDA 0x26a4f8: guard-checked declare of sStarterScript (cf. 0x26a558..0x26a582 shape).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    script_class_name(&CELL, "StarterScript")
}


// 0x26a4fc — __ZN3RBX4Name9doDeclareILZNS_14sStarterScriptEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sStarterScriptEEEERKS0_v")]
pub fn stub_26a4fc() -> &'static str {
    // IDA 0x26a4fc: guard-checked Name::declare + return (cf. 0x26a558..0x26a5b0).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    script_class_name(&CELL, "StarterScript")
}


// 0x26a5dc — __ZN3RBX4Name13callDoDeclareILZNS_11sCoreScriptEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sCoreScriptEEEEvv")]
pub fn stub_26a5dc() -> &'static str {
    // IDA 0x26a5dc: guard-checked declare of sCoreScript (cf. 0x26a558..0x26a582 shape).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    script_class_name(&CELL, "CoreScript")
}


// 0x26a5e0 — __ZN3RBX4Name9doDeclareILZNS_11sCoreScriptEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sCoreScriptEEEERKS0_v")]
pub fn stub_26a5e0() -> &'static str {
    // IDA 0x26a5e0: guard-checked Name::declare + return (cf. 0x26a558..0x26a5b0).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    script_class_name(&CELL, "CoreScript")
}


// 0x26a6c0 — __ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EEC2INS_9ContentIdEEET_
// type: RBX::BaseScript *__fastcall(RBX::BaseScript *, int *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EEC2INS_9ContentIdEEET_")]
pub fn stub_26a6c0(content: &str, init: &mut dyn FnMut(&str)) {
    // IDA 0x26a6c0: described + content-id init chain.
    init(content);
}


// 0x26a88c — __ZN3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_26a88c(this: usize, destroy: &mut dyn FnMut(usize)) {
    // IDA 0x26a88c: base destroy body.
    destroy(this);
}


// 0x26a890 — __ZN3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_26a890(this: usize, destroy: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x26a890: base destroy then operator delete.
    destroy(this);
    free(this);
}


// 0x26a930 — __ZThn32_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_26a930(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x26a930: this-32 adjust then tail-call the primary dtor.
    destroy_at(this - 32);
}


// 0x26a938 — __ZThn32_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_26a938(this: usize, destroy_at: &mut dyn FnMut(usize), free_at: &mut dyn FnMut(usize)) {
    // IDA 0x26a938: this-32 adjust then tail-call the primary dtor + delete.
    destroy_at(this - 32);
    free_at(this - 32);
}


// 0x26a9dc — __ZThn36_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_26a9dc(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x26a9dc: this-36 adjust then tail-call the primary dtor.
    destroy_at(this - 36);
}


// 0x26a9e4 — __ZThn36_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_26a9e4(this: usize, destroy_at: &mut dyn FnMut(usize), free_at: &mut dyn FnMut(usize)) {
    // IDA 0x26a9e4: this-36 adjust then tail-call the primary dtor + delete.
    destroy_at(this - 36);
    free_at(this - 36);
}


// 0x26aa88 — __ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_26aa88() -> &'static ScriptClassDescriptor {
    // IDA 0x26aa88: guard-checked once init of the StarterScript descriptor over its base (cf. 0x26aae4 shape).
    static CELL: OnceLock<ScriptClassDescriptor> = OnceLock::new();
    CELL.get_or_init(|| ScriptClassDescriptor { name: "StarterScript", base: "BaseScript" })
}


// 0x26aba4 — __ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::BaseScript *)
pub fn stub_26aba4(this: usize, destroy: &mut dyn FnMut(usize)) {
    // IDA 0x26aba4: base destroy body.
    destroy(this);
}


// 0x26aba8 — __ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_26aba8(this: usize, destroy: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x26aba8: base destroy then operator delete.
    destroy(this);
    free(this);
}


// 0x26ac48 — __ZThn32_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_26ac48(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x26ac48: this-32 adjust then tail-call the primary dtor.
    destroy_at(this - 32);
}


// 0x26ac50 — __ZThn32_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_26ac50(this: usize, destroy_at: &mut dyn FnMut(usize), free_at: &mut dyn FnMut(usize)) {
    // IDA 0x26ac50: this-32 adjust then tail-call the primary dtor + delete.
    destroy_at(this - 32);
    free_at(this - 32);
}


// 0x26acf4 — __ZThn36_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_26acf4(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x26acf4: this-36 adjust then tail-call the primary dtor.
    destroy_at(this - 36);
}


// 0x26acfc — __ZThn36_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_26acfc(this: usize, destroy_at: &mut dyn FnMut(usize), free_at: &mut dyn FnMut(usize)) {
    // IDA 0x26acfc: this-36 adjust then tail-call the primary dtor + delete.
    destroy_at(this - 36);
    free_at(this - 36);
}


// 0x26ada0 — __ZN3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED1Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED1Ev")]
pub fn stub_26ada0(this: usize, destroy: &mut dyn FnMut(usize)) {
    // IDA 0x26ada0: base destroy body.
    destroy(this);
}


// 0x26ada4 — __ZN3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED0Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED0Ev")]
pub fn stub_26ada4(this: usize, destroy: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x26ada4: base destroy then operator delete.
    destroy(this);
    free(this);
}


// 0x26ae44 — __ZThn32_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED1Ev")]
pub fn stub_26ae44(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x26ae44: this-32 adjust then tail-call the primary dtor.
    destroy_at(this - 32);
}


// 0x26ae4c — __ZThn32_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED0Ev")]
pub fn stub_26ae4c(this: usize, destroy_at: &mut dyn FnMut(usize), free_at: &mut dyn FnMut(usize)) {
    // IDA 0x26ae4c: this-32 adjust then tail-call the primary dtor + delete.
    destroy_at(this - 32);
    free_at(this - 32);
}


// 0x26aef0 — __ZThn36_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED1Ev")]
pub fn stub_26aef0(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x26aef0: this-36 adjust then tail-call the primary dtor.
    destroy_at(this - 36);
}


// 0x26aef8 — __ZThn36_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED0Ev")]
pub fn stub_26aef8(this: usize, destroy_at: &mut dyn FnMut(usize), free_at: &mut dyn FnMut(usize)) {
    // IDA 0x26aef8: this-36 adjust then tail-call the primary dtor + delete.
    destroy_at(this - 36);
    free_at(this - 36);
}


// 0x26af9c — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE10link_pointERKSC_RPNS1_22hashed_index_node_implISaIcEEEST_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::link_point(boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const&,boost::multi_index::detail::hashed_index_node_impl<std::allocator<char>> *&,boost::multi_index::detail::hashed_unique_tag)")]
pub fn stub_26af9c(set: &mut std::collections::HashSet<String>, key: &str) -> bool {
    // IDA 0x26af9c: links the interned node into the hashed index.
    // was: boost::multi_index hashed_index::link_point (ProtectedString flyweight).
    set.insert(key.to_owned())
}


// 0x26afd0 — __ZN5boost11multi_index6detail10auto_spaceImSaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES8_EEEEC2ERKSC_m
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "boost::multi_index::detail::auto_space<unsigned long,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::auto_space(std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&,unsigned long)")]
pub fn stub_26afd0(size: usize) -> Vec<usize> {
    // IDA 0x26afd0: allocates size slots, bad_alloc at >= 0x40000000 (cf. 0x26afda..0x26afe8).
    assert!(size < 0x4000_0000, "bad_alloc");
    vec![0; size]
}


// 0x26aff4 — __ZN3RBX10BaseScript19extraErrorReportingEP9lua_State
// type: void()
#[doc(alias = "RBX::BaseScript::extraErrorReporting(lua_State *)")]
pub fn stub_26aff4() {
    // IDA 0x26aff4: empty base implementation (0x26aff4).
}


// 0x26aff8 — __ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_26aff8(this: usize, destroy: &mut dyn FnMut(usize)) {
    // IDA 0x26aff8: base destroy body.
    destroy(this);
}


// 0x26affc — __ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_26affc(this: usize, destroy: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x26affc: base destroy then operator delete.
    destroy(this);
    free(this);
}


// 0x26b09c — __ZThn32_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_26b09c(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x26b09c: this-32 adjust then tail-call the primary dtor.
    destroy_at(this - 32);
}


// 0x26b0a4 — __ZThn32_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_26b0a4(this: usize, destroy_at: &mut dyn FnMut(usize), free_at: &mut dyn FnMut(usize)) {
    // IDA 0x26b0a4: this-32 adjust then tail-call the primary dtor + delete.
    destroy_at(this - 32);
    free_at(this - 32);
}


// 0x26b148 — __ZThn36_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_26b148(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x26b148: this-36 adjust then tail-call the primary dtor.
    destroy_at(this - 36);
}


// 0x26b150 — __ZThn36_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_26b150(this: usize, destroy_at: &mut dyn FnMut(usize), free_at: &mut dyn FnMut(usize)) {
    // IDA 0x26b150: this-36 adjust then tail-call the primary dtor + delete.
    destroy_at(this - 36);
    free_at(this - 36);
}


// 0x26b464 — __ZNK3RBX3Lua12LuaArguments9getStringEiRSs
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, std::string *)
#[doc(alias = "RBX::Lua::LuaArguments::getString(int,std::string &)const")]
pub fn stub_26b464(args: &[LuaArg], index: usize) -> Option<String> {
    // IDA 0x26b464: indexed slot within range and lua_type == string (4), then assign (cf. 0x26b474..0x26b4a6).
    match args.get(index) {
        Some(LuaArg::Str(s)) => Some(s.clone()),
        _ => None,
    }
}


// 0x26b4ac — __ZNK3RBX3Lua12LuaArguments15getVector3int16EiRN3G3D12Vector3int16E
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, G3D::Vector3int16 *)
#[doc(alias = "RBX::Lua::LuaArguments::getVector3int16(int,G3D::Vector3int16 &)const")]
pub fn stub_26b4ac(args: &[LuaArg], index: usize) -> Option<[f32; 3]> {
    // IDA 0x26b4ac: indexed slot within range and userdata/vector type, then copy.
    match args.get(index) {
        Some(LuaArg::Vec3(v)) => Some(*v),
        _ => None,
    }
}


// 0x26b4d8 — __ZNK3RBX3Lua12LuaArguments15getRegion3int16EiRNS_12Region3int16E
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, RBX::Region3int16 *)
#[doc(alias = "RBX::Lua::LuaArguments::getRegion3int16(int,RBX::Region3int16 &)const")]
pub fn stub_26b4d8(args: &[LuaArg], index: usize) -> Option<([f32; 3], [f32; 3])> {
    // IDA 0x26b4d8: indexed slot within range and Region3 userdata, then copy both corners.
    match args.get(index) {
        Some(LuaArg::Region3(lo, hi)) => Some((*lo, *hi)),
        _ => None,
    }
}


// 0x26b504 — __ZNK3RBX3Lua12LuaArguments10getVector3EiRN3G3D7Vector3E
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, G3D::Vector3 *)
#[doc(alias = "RBX::Lua::LuaArguments::getVector3(int,G3D::Vector3 &)const")]
pub fn stub_26b504(args: &[LuaArg], index: usize) -> Option<[f32; 3]> {
    // IDA 0x26b504: indexed slot within range and userdata/vector type, then copy.
    match args.get(index) {
        Some(LuaArg::Vec3(v)) => Some(*v),
        _ => None,
    }
}


// 0x26b530 — __ZNK3RBX3Lua12LuaArguments10getRegion3EiRNS_7Region3E
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, RBX::Region3 *)
#[doc(alias = "RBX::Lua::LuaArguments::getRegion3(int,RBX::Region3 &)const")]
pub fn stub_26b530(args: &[LuaArg], index: usize) -> Option<([f32; 3], [f32; 3])> {
    // IDA 0x26b530: indexed slot within range and Region3 userdata, then copy both corners.
    match args.get(index) {
        Some(LuaArg::Region3(lo, hi)) => Some((*lo, *hi)),
        _ => None,
    }
}


// 0x26b55c — __ZNK3RBX3Lua12LuaArguments9getObjectEiRN5boost10shared_ptrINS_10Reflection13DescribedBaseEEE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::LuaArguments::getObject(int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)const")]
pub fn stub_26b55c(args: &[LuaArg], index: usize) -> Option<SharedPtr<()>> {
    // IDA 0x26b55c: indexed slot within range and userdata/instance type, then retain.
    // was: boost::shared_ptr<DescribedBase> retained copy.
    match args.get(index) {
        Some(LuaArg::Object(o)) => Some(SharedPtr::clone(o)),
        _ => None,
    }
}


// 0x26b660 — __ZNK3RBX3Lua12LuaArguments9getDoubleEiRd
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, double *)
#[doc(alias = "RBX::Lua::LuaArguments::getDouble(int,double &)const")]
pub fn stub_26b660(args: &[LuaArg], index: usize) -> Option<f64> {
    // IDA 0x26b660: indexed slot within range and lua_type == number, then assign.
    match args.get(index) {
        Some(LuaArg::Num(n)) => Some(*n),
        _ => None,
    }
}


// 0x26b6a0 — __ZNK3RBX3Lua12LuaArguments7getBoolEiRb
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, bool *)
#[doc(alias = "RBX::Lua::LuaArguments::getBool(int,bool &)const")]
pub fn stub_26b6a0(args: &[LuaArg], index: usize) -> Option<bool> {
    // IDA 0x26b6a0: indexed slot within range and lua_type == boolean, then assign.
    match args.get(index) {
        Some(LuaArg::Bool(b)) => Some(*b),
        _ => None,
    }
}


// 0x26b6e4 — __ZNK3RBX3Lua12LuaArguments7getEnumEiRKNS_10Reflection14EnumDescriptorERi
// type: bool __fastcall(RBX::Lua::LuaArguments *this, int, const RBX::Reflection::EnumDescriptor *, int *)
#[doc(alias = "RBX::Lua::LuaArguments::getEnum(int,RBX::Reflection::EnumDescriptor const&,int &)const")]
pub fn stub_26b6e4(args: &[LuaArg], index: usize) -> Option<i32> {
    // IDA 0x26b6e4: indexed slot within range and enum userdata matching the descriptor, then assign.
    match args.get(index) {
        Some(LuaArg::Enum(v)) => Some(*v),
        _ => None,
    }
}


// 0x26b788 — __ZN3RBX3Lua12LuaArguments3getEP9lua_StateiRNS_10Reflection7VariantEb
// type: int __fastcall(struct _Unwind_Exception *, int, int, int)
#[doc(alias = "RBX::Lua::LuaArguments::get(lua_State *,int,RBX::Reflection::Variant &,bool)")]
pub fn stub_26b788(tag: u8, convert: &mut dyn FnMut(u8) -> Option<LuaArg>) -> Option<LuaArg> {
    // IDA 0x26b788: dispatches on the lua_type tag to the typed getter above.
    convert(tag)
}


// 0x26c138 — __ZN3RBX3Lua12LuaArguments4pushERKNS_10Reflection7VariantEP9lua_State
// type: int()
#[doc(alias = "RBX::Lua::LuaArguments::push(RBX::Reflection::Variant const&,lua_State *)")]
pub fn stub_26c138(push: &mut dyn FnMut()) {
    // IDA 0x26c138: withVariantValue<ArgumentPusher> thunk (cf. 0x26c138).
    push();
}


// 0x26c140 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8getValueIS3_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<G3D::Vector3int16>(lua_State *,unsigned int,G3D::Vector3int16 &)")]
pub fn stub_26c140(metatable_matches: bool, value: [i16; 3]) -> Option<[i16; 3]> {
    // IDA 0x26c140: userdata + metatable/rawequal check then 3xint16 copy (cf. 0x26c152..0x26c1b0).
    metatable_matches.then_some(value)
}


// 0x26c1b8 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE8getValueIS2_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::Region3int16,true>::getValue<RBX::Region3int16>(lua_State *,unsigned int,RBX::Region3int16 &)")]
pub fn stub_26c1b8(metatable_matches: bool, lo: [i16; 3], hi: [i16; 3]) -> Option<([i16; 3], [i16; 3])> {
    // IDA 0x26c1b8: userdata + metatable check then both-corner copy.
    metatable_matches.then_some((lo, hi))
}


// 0x26c230 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8getValueIS3_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<G3D::Vector3>(lua_State *,unsigned int,G3D::Vector3 &)")]
pub fn stub_26c230(metatable_matches: bool, value: [f32; 3]) -> Option<[f32; 3]> {
    // IDA 0x26c230: userdata + metatable check then 3xfloat copy.
    metatable_matches.then_some(value)
}


// 0x26c2ac — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE8getValueIS2_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::Region3,true>::getValue<RBX::Region3>(lua_State *,unsigned int,RBX::Region3 &)")]
pub fn stub_26c2ac(metatable_matches: bool, lo: [f32; 3], hi: [f32; 3]) -> Option<([f32; 3], [f32; 3])> {
    // IDA 0x26c2ac: userdata + metatable check then both-corner copy.
    metatable_matches.then_some((lo, hi))
}


// 0x26c350 — __ZN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEaSINS1_8InstanceEEERS4_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Reflection::DescribedBase>& boost::shared_ptr<RBX::Reflection::DescribedBase>::operator=<RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)")]
pub fn stub_26c350(slot: &mut SharedPtr<()>, value: SharedPtr<()>) {
    // IDA 0x26c350: retains the new counted impl, stores it, releases the old (cf. 0x26c358..0x26c380).
    // was: boost::shared_ptr<DescribedBase>::operator=<Instance> → Arc assign.
    *slot = value;
}


// 0x26c38c — __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrIN5boost10shared_ptrINS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<boost::shared_ptr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)")]
pub fn stub_26c38c(args: &[LuaArg], index: usize) -> Option<SharedPtr<()>> {
    // IDA 0x26c38c: userdata metatable check then shared_ptr retain.
    // was: boost::shared_ptr retained copy.
    match args.get(index) {
        Some(LuaArg::Object(o)) => Some(SharedPtr::clone(o)),
        _ => None,
    }
}


// 0x26c474 — __ZNK3RBX10Reflection4TypeneERKS1_
// type: bool __fastcall(int, int)
#[doc(alias = "RBX::Reflection::Type::operator!=(RBX::Reflection::Type const&)const")]
pub fn stub_26c474(this: usize, other: usize, same_type: bool) -> bool {
    // IDA 0x26c474: asserts (type != right.type) == (this != &right) (type.h:45, cf. 0x26c48e..0x26c4ee) then pointer inequality (0x26c4fe).
    assert!((!same_type) == (this != other), "(type!=right.type) == (this!=&right) type.h:45");
    this != other
}


// 0x26c500 — __ZN3rbx11make_sharedISt6vectorIN3RBX10Reflection7VariantESaIS4_EEiEEN5boost10shared_ptrIT_EERKT0_
// type: int __fastcall(_DWORD *, int *)
#[doc(alias = "boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> rbx::make_shared<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>,int>(int const&)")]
pub fn stub_26c500(capacity: i32) -> SharedPtr<Vec<usize>> {
    // IDA 0x26c500: single-shot counted alloc of the vector.
    // was: rbx::make_shared<vector<Variant>, int> → SharedPtr (Arc).
    let mut v = Vec::new();
    v.reserve(capacity.max(0) as usize);
    SharedPtr::new(v)
}


// 0x26c6a4 — __ZN5boost10shared_ptrINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS5_EEEEEaSERKSF_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::operator=(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> const&)")]
pub fn stub_26c6a4(slot: &mut SharedPtr<()>, value: SharedPtr<()>) {
    // IDA 0x26c6a4: retains the new counted impl, stores it, releases the old (cf. 0x26c358..0x26c380).
    // was: boost::shared_ptr<DescribedBase>::operator=<Instance> → Arc assign.
    *slot = value;
}


// 0x26c6dc — __ZN3rbx11make_sharedIKSt6vectorIN3RBX10Reflection7VariantESaIS4_EEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> rbx::make_shared<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>(void)")]
pub fn stub_26c6dc() -> SharedPtr<Vec<usize>> {
    // IDA 0x26c6dc: single-shot counted alloc of the empty vector.
    // was: rbx::make_shared<vector<Variant> const> → SharedPtr (Arc).
    SharedPtr::new(Vec::new())
}


// 0x26c830 — __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
pub fn stub_26c830(args: &[LuaArg], index: usize) -> Option<SharedPtr<()>> {
    // IDA 0x26c830: userdata metatable check then shared_ptr retain.
    // was: boost::shared_ptr retained copy.
    match args.get(index) {
        Some(LuaArg::Object(o)) => Some(SharedPtr::clone(o)),
        _ => None,
    }
}


// 0x26c92c — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
pub fn stub_26c92c(args: &[LuaArg], index: usize) -> Option<LuaArg> {
    // IDA 0x26c92c: metatable-checked userdata read, pushed as a Variant.
    args.get(index).cloned()
}


// 0x26c9a8 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::Region3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
pub fn stub_26c9a8(args: &[LuaArg], index: usize) -> Option<LuaArg> {
    // IDA 0x26c9a8: metatable-checked userdata read, pushed as a Variant.
    args.get(index).cloned()
}


// 0x26ca24 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::Region3int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
pub fn stub_26ca24(args: &[LuaArg], index: usize) -> Option<LuaArg> {
    // IDA 0x26ca24: metatable-checked userdata read, pushed as a Variant.
    args.get(index).cloned()
}


// 0x26caa0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
pub fn stub_26caa0(args: &[LuaArg], index: usize) -> Option<LuaArg> {
    // IDA 0x26caa0: metatable-checked userdata read, pushed as a Variant.
    args.get(index).cloned()
}


// 0x26cb1c — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
pub fn stub_26cb1c(args: &[LuaArg], index: usize) -> Option<LuaArg> {
    // IDA 0x26cb1c: metatable-checked userdata read, pushed as a Variant.
    args.get(index).cloned()
}

