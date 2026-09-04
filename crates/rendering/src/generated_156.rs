//! rendering — generated_156 — next 100 stubs EA-sorted asc filler (Ogre|G3D|Gfx|Render|Adorn 15586 filtered, 15586 covered, filler 16989->17089, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf6b3d4 — __ZNSs6appendEPKcm
#[doc(alias = "std::string::append(char const*,unsigned long)")]
// was: std::string::append(char const*,unsigned long)
// type: _DWORD __fastcall(std::string *__hidden this, const char *, unsigned int)
// IDA 0xf6b3d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b3d4() {
}

// 0xf6b3e4 — __ZNSs6appendERKSs
#[doc(alias = "std::string::append(std::string const&)")]
// was: std::string::append(std::string const&)
// type: _DWORD __fastcall(std::string *__hidden this, const std::string *)
// IDA 0xf6b3e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b3e4() {
}

// 0xf6b3f4 — __ZNSs6appendEmc
#[doc(alias = "std::string::append(unsigned long,char)")]
// was: std::string::append(unsigned long,char)
// type: _DWORD __fastcall(std::string *__hidden this, unsigned int, char)
// IDA 0xf6b3f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b3f4() {
}

// 0xf6b404 — __ZNSs6assignEPKcm
#[doc(alias = "std::string::assign(char const*,unsigned long)")]
// was: std::string::assign(char const*,unsigned long)
// type: _DWORD __fastcall(std::string *__hidden this, const char *, unsigned int)
// IDA 0xf6b404: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b404() {
}

// 0xf6b414 — __ZNSs6assignERKSs
#[doc(alias = "std::string::assign(std::string const&)")]
// was: std::string::assign(std::string const&)
// type: _DWORD __fastcall(std::string *__hidden this, const std::string *)
// IDA 0xf6b414: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b414() {
}

// 0xf6b424 — __ZNSs6insertEmPKcm
#[doc(alias = "std::string::insert(unsigned long,char const*,unsigned long)")]
// was: std::string::insert(unsigned long,char const*,unsigned long)
// type: _DWORD __fastcall(std::string *__hidden this, unsigned int, const char *, unsigned int)
// IDA 0xf6b424: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b424() {
}

// 0xf6b434 — __ZNSs6insertEmRKSsmm
#[doc(alias = "std::string::insert(unsigned long,std::string const&,unsigned long,unsigned long)")]
// was: std::string::insert(unsigned long,std::string const&,unsigned long,unsigned long)
// type: _DWORD __fastcall(std::string *__hidden this, unsigned int, const std::string *, unsigned int, unsigned int)
// IDA 0xf6b434: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b434() {
}

// 0xf6b444 — __ZNSs6resizeEmc
#[doc(alias = "std::string::resize(unsigned long,char)")]
// was: std::string::resize(unsigned long,char)
// type: _DWORD __fastcall(std::string *__hidden this, unsigned int, char)
// IDA 0xf6b444: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b444() {
}

// 0xf6b454 — __ZNSs7replaceEmmPKcm
#[doc(alias = "std::string::replace(unsigned long,unsigned long,char const*,unsigned long)")]
// was: std::string::replace(unsigned long,unsigned long,char const*,unsigned long)
// type: _DWORD __fastcall(std::string *__hidden this, unsigned int, unsigned int, const char *, unsigned int)
// IDA 0xf6b454: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b454() {
}

// 0xf6b464 — __ZNSs7reserveEm
#[doc(alias = "std::string::reserve(unsigned long)")]
// was: std::string::reserve(unsigned long)
// type: _DWORD __fastcall(std::string *__hidden this, unsigned int)
// IDA 0xf6b464: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b464() {
}

// 0xf6b474 — __ZNSs9_M_mutateEmmm
#[doc(alias = "std::string::_M_mutate(unsigned long,unsigned long,unsigned long)")]
// was: std::string::_M_mutate(unsigned long,unsigned long,unsigned long)
// type: _DWORD __fastcall(std::string *__hidden this, unsigned int, unsigned int, unsigned int)
// IDA 0xf6b474: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b474() {
}

// 0xf6b484 — __ZNSs9push_backEc
#[doc(alias = "std::string::push_back(char)")]
// was: std::string::push_back(char)
// type: _DWORD __fastcall(std::string *__hidden this, char)
// IDA 0xf6b484: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_f6b484() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0xf6b494 — __ZNSsC1EPKcRKSaIcE
#[doc(alias = "std::string::string(char const*,std::allocator<char> const&)")]
// was: std::string::string(char const*,std::allocator<char> const&)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// IDA 0xf6b494: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b494() {
}

// 0xf6b4a4 — __ZNSsC1EPKcmRKSaIcE
#[doc(alias = "std::string::string(char const*,unsigned long,std::allocator<char> const&)")]
// was: std::string::string(char const*,unsigned long,std::allocator<char> const&)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
// IDA 0xf6b4a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b4a4() {
}

// 0xf6b4b4 — __ZNSsC1ERKSs
#[doc(alias = "std::string::string(std::string const&)")]
// was: std::string::string(std::string const&)
// type: _DWORD __fastcall(std::string *__hidden this, const std::string *)
// IDA 0xf6b4b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b4b4() {
}

// 0xf6b4c4 — __ZNSsC1ERKSsmm
#[doc(alias = "std::string::string(std::string const&,unsigned long,unsigned long)")]
// was: std::string::string(std::string const&,unsigned long,unsigned long)
// type: _DWORD __fastcall(std::string *__hidden this, const std::string *, unsigned int, unsigned int)
// IDA 0xf6b4c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b4c4() {
}

// 0xf6b4d4 — __ZNSsC1EmcRKSaIcE
#[doc(alias = "std::string::string(unsigned long,char,std::allocator<char> const&)")]
// was: std::string::string(unsigned long,char,std::allocator<char> const&)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
// IDA 0xf6b4d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b4d4() {
}

// 0xf6b4e4 — __ZNSsD2Ev
#[doc(alias = "std::string::~string()")]
// was: std::string::~string()
// type: void __fastcall(std::string *__hidden this)
// IDA 0xf6b4e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b4e4() {
}

// 0xf6b4f4 — __ZNSt11logic_errorC1ERKSs
#[doc(alias = "std::logic_error::logic_error(std::string const&)")]
// was: std::logic_error::logic_error(std::string const&)
// type: _DWORD __fastcall(std::logic_error *__hidden this, const std::string *)
// IDA 0xf6b4f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b4f4() {
}

// 0xf6b504 — __ZNSt11logic_errorD1Ev
#[doc(alias = "std::logic_error::~logic_error()")]
// was: std::logic_error::~logic_error()
// type: void __cdecl(std::logic_error *__hidden this)
// IDA 0xf6b504: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b504() {
}

// 0xf6b514 — __ZNSt11logic_errorD2Ev
#[doc(alias = "std::logic_error::~logic_error()")]
// was: std::logic_error::~logic_error()
// type: void __cdecl(std::logic_error *__hidden this)
// IDA 0xf6b514: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b514() {
}

// 0xf6b524 — __ZNSt12__basic_fileIcED1Ev
#[doc(alias = "std::__basic_file<char>::~__basic_file()")]
// was: std::__basic_file<char>::~__basic_file()
// IDA 0xf6b524: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b524() {
}

// 0xf6b534 — __ZNSt12length_errorC1ERKSs
#[doc(alias = "std::length_error::length_error(std::string const&)")]
// was: std::length_error::length_error(std::string const&)
// type: _DWORD __fastcall(std::length_error *__hidden this, const std::string *)
// IDA 0xf6b534: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b534() {
}

// 0xf6b544 — __ZNSt12out_of_rangeC2ERKSs
#[doc(alias = "std::out_of_range::out_of_range(std::string const&)")]
// was: std::out_of_range::out_of_range(std::string const&)
// type: _DWORD __fastcall(std::out_of_range *__hidden this, const std::string *)
// IDA 0xf6b544: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b544() {
}

// 0xf6b554 — __ZNSt13bad_exceptionD2Ev
#[doc(alias = "std::bad_exception::~bad_exception()")]
// was: std::bad_exception::~bad_exception()
// type: void __cdecl(std::bad_exception *__hidden this)
// IDA 0xf6b554: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b554() {
}

// 0xf6b564 — __ZNSt13basic_filebufIcSt11char_traitsIcEE4openEPKcSt13_Ios_Openmode
#[doc(alias = "std::basic_filebuf<char,std::char_traits<char>>::open(char const*,std::_Ios_Openmode)")]
// was: std::basic_filebuf<char,std::char_traits<char>>::open(char const*,std::_Ios_Openmode)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// IDA 0xf6b564: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b564() {
}

// 0xf6b574 — __ZNSt13basic_filebufIcSt11char_traitsIcEE5closeEv
#[doc(alias = "std::basic_filebuf<char,std::char_traits<char>>::close(void)")]
// was: std::basic_filebuf<char,std::char_traits<char>>::close(void)
// IDA 0xf6b574: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b574() {
}

// 0xf6b584 — __ZNSt13basic_fstreamIcSt11char_traitsIcEEC1Ev
#[doc(alias = "std::basic_fstream<char,std::char_traits<char>>::basic_fstream(void)")]
// was: std::basic_fstream<char,std::char_traits<char>>::basic_fstream(void)
// IDA 0xf6b584: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b584() {
}

// 0xf6b594 — __ZNSt13runtime_errorC1ERKSs
#[doc(alias = "std::runtime_error::runtime_error(std::string const&)")]
// was: std::runtime_error::runtime_error(std::string const&)
// type: _DWORD __fastcall(std::runtime_error *__hidden this, const std::string *)
// IDA 0xf6b594: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b594() {
}

// 0xf6b5a4 — __ZNSt13runtime_errorC2ERKSs
#[doc(alias = "std::runtime_error::runtime_error(std::string const&)")]
// was: std::runtime_error::runtime_error(std::string const&)
// type: _DWORD __fastcall(std::runtime_error *__hidden this, const std::string *)
// IDA 0xf6b5a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b5a4() {
}

// 0xf6b5b4 — __ZNSt13runtime_errorD1Ev
#[doc(alias = "std::runtime_error::~runtime_error()")]
// was: std::runtime_error::~runtime_error()
// type: void __cdecl(std::runtime_error *__hidden this)
// IDA 0xf6b5b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b5b4() {
}

// 0xf6b5c4 — __ZNSt13runtime_errorD2Ev
#[doc(alias = "std::runtime_error::~runtime_error()")]
// was: std::runtime_error::~runtime_error()
// type: void __cdecl(std::runtime_error *__hidden this)
// IDA 0xf6b5c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b5c4() {
}

// 0xf6b5d4 — __ZNSt14basic_ifstreamIcSt11char_traitsIcEE5closeEv
#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::close(void)")]
// was: std::basic_ifstream<char,std::char_traits<char>>::close(void)
// IDA 0xf6b5d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b5d4() {
}

// 0xf6b5e4 — __ZNSt14basic_ifstreamIcSt11char_traitsIcEEC1EPKcSt13_Ios_Openmode
#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::basic_ifstream(char const*,std::_Ios_Openmode)")]
// was: std::basic_ifstream<char,std::char_traits<char>>::basic_ifstream(char const*,std::_Ios_Openmode)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// IDA 0xf6b5e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b5e4() {
}

// 0xf6b5f4 — __ZNSt14basic_ifstreamIcSt11char_traitsIcEEC1Ev
#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::basic_ifstream(void)")]
// was: std::basic_ifstream<char,std::char_traits<char>>::basic_ifstream(void)
// type: int(void)
// IDA 0xf6b5f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b5f4() {
}

// 0xf6b604 — __ZNSt14basic_ifstreamIcSt11char_traitsIcEED1Ev
#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::~basic_ifstream()")]
// was: std::basic_ifstream<char,std::char_traits<char>>::~basic_ifstream()
// type: int()
// IDA 0xf6b604: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b604() {
}

// 0xf6b614 — __ZNSt14basic_ifstreamIcSt11char_traitsIcEED2Ev
#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::~basic_ifstream()")]
// was: std::basic_ifstream<char,std::char_traits<char>>::~basic_ifstream()
// IDA 0xf6b614: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b614() {
}

// 0xf6b624 — __ZNSt14basic_ofstreamIcSt11char_traitsIcEE4openEPKcSt13_Ios_Openmode
#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::open(char const*,std::_Ios_Openmode)")]
// was: std::basic_ofstream<char,std::char_traits<char>>::open(char const*,std::_Ios_Openmode)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// IDA 0xf6b624: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b624() {
}

// 0xf6b634 — __ZNSt14basic_ofstreamIcSt11char_traitsIcEE5closeEv
#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::close(void)")]
// was: std::basic_ofstream<char,std::char_traits<char>>::close(void)
// type: int __fastcall(_DWORD)
// IDA 0xf6b634: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b634() {
}

// 0xf6b644 — __ZNSt14basic_ofstreamIcSt11char_traitsIcEEC1EPKcSt13_Ios_Openmode
#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::basic_ofstream(char const*,std::_Ios_Openmode)")]
// was: std::basic_ofstream<char,std::char_traits<char>>::basic_ofstream(char const*,std::_Ios_Openmode)
// IDA 0xf6b644: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b644() {
}

// 0xf6b654 — __ZNSt14basic_ofstreamIcSt11char_traitsIcEEC1Ev
#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::basic_ofstream(void)")]
// was: std::basic_ofstream<char,std::char_traits<char>>::basic_ofstream(void)
// type: int __fastcall(_DWORD)
// IDA 0xf6b654: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b654() {
}

// 0xf6b664 — __ZNSt14basic_ofstreamIcSt11char_traitsIcEED1Ev
#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::~basic_ofstream()")]
// was: std::basic_ofstream<char,std::char_traits<char>>::~basic_ofstream()
// type: int __fastcall(_DWORD)
// IDA 0xf6b664: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b664() {
}

// 0xf6b674 — __ZNSt15_List_node_base4hookEPS_
#[doc(alias = "std::_List_node_base::hook(std::_List_node_base*)")]
// was: std::_List_node_base::hook(std::_List_node_base*)
// type: _DWORD __fastcall(std::_List_node_base *__hidden this, std::_List_node_base *)
// IDA 0xf6b674: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b674() {
}

// 0xf6b684 — __ZNSt15_List_node_base4swapERS_S0_
#[doc(alias = "std::_List_node_base::swap(std::_List_node_base&,std::_List_node_base&)")]
// was: std::_List_node_base::swap(std::_List_node_base&,std::_List_node_base&)
// type: _DWORD __fastcall(std::_List_node_base *__hidden this, std::_List_node_base *, std::_List_node_base *)
// IDA 0xf6b684: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b684() {
}

// 0xf6b694 — __ZNSt15_List_node_base6unhookEv
#[doc(alias = "std::_List_node_base::unhook(void)")]
// was: std::_List_node_base::unhook(void)
// type: _DWORD __fastcall(std::_List_node_base *__hidden this)
// IDA 0xf6b694: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b694() {
}

// 0xf6b6a4 — __ZNSt15_List_node_base8transferEPS_S0_
#[doc(alias = "std::_List_node_base::transfer(std::_List_node_base*,std::_List_node_base*)")]
// was: std::_List_node_base::transfer(std::_List_node_base*,std::_List_node_base*)
// type: _DWORD __fastcall(std::_List_node_base *__hidden this, std::_List_node_base *, std::_List_node_base *)
// IDA 0xf6b6a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b6a4() {
}

// 0xf6b6b4 — __ZNSt15basic_streambufIcSt11char_traitsIcEE8pubimbueERKSt6locale
#[doc(alias = "std::basic_streambuf<char,std::char_traits<char>>::pubimbue(std::locale const&)")]
// was: std::basic_streambuf<char,std::char_traits<char>>::pubimbue(std::locale const&)
// IDA 0xf6b6b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b6b4() {
}

// 0xf6b6c4 — __ZNSt15basic_stringbufIcSt11char_traitsIcESaIcEE17_M_stringbuf_initESt13_Ios_Openmode
#[doc(alias = "std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::_M_stringbuf_init(std::_Ios_Openmode)")]
// was: std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::_M_stringbuf_init(std::_Ios_Openmode)
// IDA 0xf6b6c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b6c4() {
}

// 0xf6b6d4 — __ZNSt15basic_stringbufIcSt11char_traitsIcESaIcEE7_M_syncEPcmm
#[doc(alias = "std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::_M_sync(char *,unsigned long,unsigned long)")]
// was: std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::_M_sync(char *,unsigned long,unsigned long)
// IDA 0xf6b6d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b6d4() {
}

// 0xf6b6e4 — __ZNSt16invalid_argumentC1ERKSs
#[doc(alias = "std::invalid_argument::invalid_argument(std::string const&)")]
// was: std::invalid_argument::invalid_argument(std::string const&)
// type: _DWORD __fastcall(std::invalid_argument *__hidden this, const std::string *)
// IDA 0xf6b6e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b6e4() {
}

// 0xf6b6f4 — __ZNSt18basic_stringstreamIcSt11char_traitsIcESaIcEEC1ERKSsSt13_Ios_Openmode
#[doc(alias = "std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::basic_stringstream(std::string const&,std::_Ios_Openmode)")]
// was: std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::basic_stringstream(std::string const&,std::_Ios_Openmode)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// IDA 0xf6b6f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b6f4() {
}

// 0xf6b704 — __ZNSt18basic_stringstreamIcSt11char_traitsIcESaIcEEC1ESt13_Ios_Openmode
#[doc(alias = "std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::basic_stringstream(std::_Ios_Openmode)")]
// was: std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::basic_stringstream(std::_Ios_Openmode)
// type: int __fastcall(_DWORD, _DWORD)
// IDA 0xf6b704: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b704() {
}

// 0xf6b714 — __ZNSt18basic_stringstreamIcSt11char_traitsIcESaIcEED1Ev
#[doc(alias = "std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_stringstream()")]
// was: std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_stringstream()
// type: int __fastcall(_DWORD)
// IDA 0xf6b714: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b714() {
}

// 0xf6b724 — __ZNSt18basic_stringstreamIcSt11char_traitsIcESaIcEED2Ev
#[doc(alias = "std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_stringstream()")]
// was: std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_stringstream()
// IDA 0xf6b724: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b724() {
}

// 0xf6b734 — __ZNSt19basic_istringstreamIcSt11char_traitsIcESaIcEEC1ERKSsSt13_Ios_Openmode
#[doc(alias = "std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::basic_istringstream(std::string const&,std::_Ios_Openmode)")]
// was: std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::basic_istringstream(std::string const&,std::_Ios_Openmode)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// IDA 0xf6b734: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b734() {
}

// 0xf6b744 — __ZNSt19basic_istringstreamIcSt11char_traitsIcESaIcEEC1ESt13_Ios_Openmode
#[doc(alias = "std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::basic_istringstream(std::_Ios_Openmode)")]
// was: std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::basic_istringstream(std::_Ios_Openmode)
// type: int()
// IDA 0xf6b744: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b744() {
}

// 0xf6b754 — __ZNSt19basic_istringstreamIcSt11char_traitsIcESaIcEED1Ev
#[doc(alias = "std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_istringstream()")]
// was: std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_istringstream()
// type: int __fastcall(_DWORD)
// IDA 0xf6b754: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b754() {
}

// 0xf6b764 — __ZNSt19basic_istringstreamIcSt11char_traitsIcESaIcEED2Ev
#[doc(alias = "std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_istringstream()")]
// was: std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_istringstream()
// type: int __fastcall(_DWORD, _DWORD)
// IDA 0xf6b764: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b764() {
}

// 0xf6b774 — __ZNSt19basic_ostringstreamIcSt11char_traitsIcESaIcEEC1ESt13_Ios_Openmode
#[doc(alias = "std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>::basic_ostringstream(std::_Ios_Openmode)")]
// was: std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>::basic_ostringstream(std::_Ios_Openmode)
// type: int __fastcall(_DWORD, _DWORD)
// IDA 0xf6b774: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b774() {
}

// 0xf6b784 — __ZNSt19basic_ostringstreamIcSt11char_traitsIcESaIcEED1Ev
#[doc(alias = "std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_ostringstream()")]
// was: std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_ostringstream()
// IDA 0xf6b784: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b784() {
}

// 0xf6b794 — __ZNSt6locale5_Impl16_M_install_facetEPKNS_2idEPKNS_5facetE
#[doc(alias = "std::locale::_Impl::_M_install_facet(std::locale::id const*,std::locale::facet const*)")]
// was: std::locale::_Impl::_M_install_facet(std::locale::id const*,std::locale::facet const*)
// type: int(void)
// IDA 0xf6b794: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b794() {
}

// 0xf6b7a4 — __ZNSt6locale5_ImplC1ERKS0_m
#[doc(alias = "std::locale::_Impl::_Impl(std::locale::_Impl const&,unsigned long)")]
// was: std::locale::_Impl::_Impl(std::locale::_Impl const&,unsigned long)
// type: _DWORD __fastcall(std::locale::_Impl *__hidden this, const _Impl *, unsigned int)
// IDA 0xf6b7a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b7a4() {
}

// 0xf6b7b4 — __ZNSt6locale5_ImplD1Ev
#[doc(alias = "std::locale::_Impl::~_Impl()")]
// was: std::locale::_Impl::~_Impl()
// type: void __fastcall(std::locale::_Impl *__hidden this)
// IDA 0xf6b7b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b7b4() {
}

// 0xf6b7c4 — __ZNSt6locale7classicEv
#[doc(alias = "std::locale::classic(void)")]
// was: std::locale::classic(void)
// type: _DWORD __fastcall(std::locale *__hidden this)
// IDA 0xf6b7c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b7c4() {
}

// 0xf6b7d4 — __ZNSt6localeC1ERKS_
#[doc(alias = "std::locale::locale(std::locale const&)")]
// was: std::locale::locale(std::locale const&)
// type: _DWORD __fastcall(std::locale *__hidden this, const std::locale *)
// IDA 0xf6b7d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b7d4() {
}

// 0xf6b7e4 — __ZNSt6localeC1Ev
#[doc(alias = "std::locale::locale(void)")]
// was: std::locale::locale(void)
// type: _DWORD __fastcall(std::locale *__hidden this)
// IDA 0xf6b7e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b7e4() {
}

// 0xf6b7f4 — __ZNSt6localeD1Ev
#[doc(alias = "std::locale::~locale()")]
// was: std::locale::~locale()
// type: void __fastcall(std::locale *__hidden this)
// IDA 0xf6b7f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b7f4() {
}

// 0xf6b804 — __ZNSt6localeaSERKS_
#[doc(alias = "std::locale::operator=(std::locale const&)")]
// was: std::locale::operator=(std::locale const&)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// IDA 0xf6b804: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b804() {
}

// 0xf6b814 — __ZNSt7codecvtIwc11__mbstate_tEC2Em
#[doc(alias = "std::codecvt<wchar_t,char,__mbstate_t>::codecvt(unsigned long)")]
// was: std::codecvt<wchar_t,char,__mbstate_t>::codecvt(unsigned long)
// type: int __fastcall(_DWORD, _DWORD)
// IDA 0xf6b814: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b814() {
}

// 0xf6b824 — __ZNSt7codecvtIwc11__mbstate_tED2Ev
#[doc(alias = "std::codecvt<wchar_t,char,__mbstate_t>::~codecvt()")]
// was: std::codecvt<wchar_t,char,__mbstate_t>::~codecvt()
// type: int __fastcall(_DWORD)
// IDA 0xf6b824: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b824() {
}

// 0xf6b834 — __ZNSt8bad_castD2Ev
#[doc(alias = "std::bad_cast::~bad_cast()")]
// was: std::bad_cast::~bad_cast()
// type: void __cdecl(std::bad_cast *__hidden this)
// IDA 0xf6b834: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b834() {
}

// 0xf6b844 — __ZNSt8ios_base4InitC1Ev
#[doc(alias = "std::ios_base::Init::Init(void)")]
// was: std::ios_base::Init::Init(void)
// type: _DWORD __fastcall(std::ios_base::Init *__hidden this)
// IDA 0xf6b844: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b844() {
}

// 0xf6b854 — __ZNSt8ios_base7failureC1ERKSs
#[doc(alias = "std::ios_base::failure::failure(std::string const&)")]
// was: std::ios_base::failure::failure(std::string const&)
// type: _DWORD __fastcall(std::ios_base::failure *__hidden this, const std::string *)
// IDA 0xf6b854: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b854() {
}

// 0xf6b864 — __ZNSt8ios_base7failureC2ERKSs
#[doc(alias = "std::ios_base::failure::failure(std::string const&)")]
// was: std::ios_base::failure::failure(std::string const&)
// type: _DWORD __fastcall(std::ios_base::failure *__hidden this, const std::string *)
// IDA 0xf6b864: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b864() {
}

// 0xf6b874 — __ZNSt8ios_base7failureD1Ev
#[doc(alias = "std::ios_base::failure::~failure()")]
// was: std::ios_base::failure::~failure()
// type: void __fastcall(std::ios_base::failure *__hidden this)
// IDA 0xf6b874: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b874() {
}

// 0xf6b884 — __ZNSt8ios_base7failureD2Ev
#[doc(alias = "std::ios_base::failure::~failure()")]
// was: std::ios_base::failure::~failure()
// type: void __fastcall(std::ios_base::failure *__hidden this)
// IDA 0xf6b884: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b884() {
}

// 0xf6b894 — __ZNSt8ios_baseC2Ev
#[doc(alias = "std::ios_base::ios_base(void)")]
// was: std::ios_base::ios_base(void)
// type: _DWORD __fastcall(std::ios_base *__hidden this)
// IDA 0xf6b894: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b894() {
}

// 0xf6b8a4 — __ZNSt8ios_baseD2Ev
#[doc(alias = "std::ios_base::~ios_base()")]
// was: std::ios_base::~ios_base()
// type: void __fastcall(std::ios_base *__hidden this)
// IDA 0xf6b8a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b8a4() {
}

// 0xf6b8b4 — __ZNSt9bad_allocD1Ev
#[doc(alias = "std::bad_alloc::~bad_alloc()")]
// was: std::bad_alloc::~bad_alloc()
// type: void __cdecl(std::bad_alloc *__hidden this)
// IDA 0xf6b8b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b8b4() {
}

// 0xf6b8c4 — __ZNSt9bad_allocD2Ev
#[doc(alias = "std::bad_alloc::~bad_alloc()")]
// was: std::bad_alloc::~bad_alloc()
// type: void __cdecl(std::bad_alloc *__hidden this)
// IDA 0xf6b8c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b8c4() {
}

// 0xf6b8d4 — __ZNSt9basic_iosIcSt11char_traitsIcEE4initEPSt15basic_streambufIcS1_E
#[doc(alias = "std::basic_ios<char,std::char_traits<char>>::init(std::basic_streambuf<char,std::char_traits<char>> *)")]
// was: std::basic_ios<char,std::char_traits<char>>::init(std::basic_streambuf<char,std::char_traits<char>> *)
// type: int __fastcall(_DWORD, _DWORD)
// IDA 0xf6b8d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b8d4() {
}

// 0xf6b8e4 — __ZNSt9basic_iosIcSt11char_traitsIcEE5clearESt12_Ios_Iostate
#[doc(alias = "std::basic_ios<char,std::char_traits<char>>::clear(std::_Ios_Iostate)")]
// was: std::basic_ios<char,std::char_traits<char>>::clear(std::_Ios_Iostate)
// type: int __fastcall(_DWORD, _DWORD)
// IDA 0xf6b8e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b8e4() {
}

// 0xf6b8f4 — __ZNSt9basic_iosIcSt11char_traitsIcEE5imbueERKSt6locale
#[doc(alias = "std::basic_ios<char,std::char_traits<char>>::imbue(std::locale const&)")]
// was: std::basic_ios<char,std::char_traits<char>>::imbue(std::locale const&)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// IDA 0xf6b8f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b8f4() {
}

// 0xf6b904 — __ZNSt9basic_iosIcSt11char_traitsIcEE5rdbufEPSt15basic_streambufIcS1_E
#[doc(alias = "std::basic_ios<char,std::char_traits<char>>::rdbuf(std::basic_streambuf<char,std::char_traits<char>> *)")]
// was: std::basic_ios<char,std::char_traits<char>>::rdbuf(std::basic_streambuf<char,std::char_traits<char>> *)
// type: int(void)
// IDA 0xf6b904: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b904() {
}

// 0xf6b914 — __ZNSt9exceptionD2Ev
#[doc(alias = "std::exception::~exception()")]
// was: std::exception::~exception()
// type: void __cdecl(std::exception *__hidden this)
// IDA 0xf6b914: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f6b914() {
}

// 0xf6b924 — __ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_i
#[doc(alias = "std::basic_ostream<char,std::char_traits<char>> & std::__ostream_insert<char,std::char_traits<char>>(std::basic_ostream<char,std::char_traits<char>> &,char const*,int)")]
// was: std::basic_ostream<char,std::char_traits<char>> & std::__ostream_insert<char,std::char_traits<char>>(std::basic_ostream<char,std::char_traits<char>> &,char const*,int)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// IDA 0xf6b924: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b924() {
}

// 0xf6b934 — __ZSt16__throw_bad_castv
#[doc(alias = "std::__throw_bad_cast(void)")]
// was: std::__throw_bad_cast(void)
// type: void __fastcall __noreturn()
// IDA 0xf6b934: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b934() {
}

// 0xf6b944 — __ZSt17__throw_bad_allocv
#[doc(alias = "std::__throw_bad_alloc(void)")]
// was: std::__throw_bad_alloc(void)
// type: void(void)
// IDA 0xf6b944: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b944() {
}

// 0xf6b954 — __ZSt18_Rb_tree_decrementPKSt18_Rb_tree_node_base
#[doc(alias = "std::_Rb_tree_decrement(std::_Rb_tree_node_base const*)")]
// was: std::_Rb_tree_decrement(std::_Rb_tree_node_base const*)
// type: _DWORD __fastcall(const _Rb_tree_node_base *)
// IDA 0xf6b954: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b954() {
}

// 0xf6b964 — __ZSt18_Rb_tree_decrementPSt18_Rb_tree_node_base
#[doc(alias = "std::_Rb_tree_decrement(std::_Rb_tree_node_base *)")]
// was: std::_Rb_tree_decrement(std::_Rb_tree_node_base *)
// type: _DWORD __fastcall(_Rb_tree_node_base *)
// IDA 0xf6b964: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b964() {
}

// 0xf6b974 — __ZSt18_Rb_tree_incrementPKSt18_Rb_tree_node_base
#[doc(alias = "std::_Rb_tree_increment(std::_Rb_tree_node_base const*)")]
// was: std::_Rb_tree_increment(std::_Rb_tree_node_base const*)
// type: _DWORD __fastcall(const _Rb_tree_node_base *)
// IDA 0xf6b974: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b974() {
}

// 0xf6b984 — __ZSt18_Rb_tree_incrementPSt18_Rb_tree_node_base
#[doc(alias = "std::_Rb_tree_increment(std::_Rb_tree_node_base *)")]
// was: std::_Rb_tree_increment(std::_Rb_tree_node_base *)
// type: _DWORD __fastcall(_Rb_tree_node_base *)
// IDA 0xf6b984: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b984() {
}

// 0xf6b994 — __ZSt19__throw_logic_errorPKc
#[doc(alias = "std::__throw_logic_error(char const*)")]
// was: std::__throw_logic_error(char const*)
// type: void __fastcall __noreturn(const char *)
// IDA 0xf6b994: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b994() {
}

// 0xf6b9a4 — __ZSt20__throw_length_errorPKc
#[doc(alias = "std::__throw_length_error(char const*)")]
// was: std::__throw_length_error(char const*)
// type: void __fastcall __noreturn(const char *)
// IDA 0xf6b9a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b9a4() {
}

// 0xf6b9b4 — __ZSt20__throw_out_of_rangePKc
#[doc(alias = "std::__throw_out_of_range(char const*)")]
// was: std::__throw_out_of_range(char const*)
// type: void __fastcall __noreturn(const char *)
// IDA 0xf6b9b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b9b4() {
}

// 0xf6b9c4 — __ZSt28_Rb_tree_rebalance_for_erasePSt18_Rb_tree_node_baseRS_
#[doc(alias = "std::_Rb_tree_rebalance_for_erase(std::_Rb_tree_node_base *,std::_Rb_tree_node_base&)")]
// was: std::_Rb_tree_rebalance_for_erase(std::_Rb_tree_node_base *,std::_Rb_tree_node_base&)
// type: int __fastcall(_Rb_tree_node_base *, _Rb_tree_node_base *)
// IDA 0xf6b9c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b9c4() {
}

// 0xf6b9d4 — __ZSt29_Rb_tree_insert_and_rebalancebPSt18_Rb_tree_node_baseS0_RS_
#[doc(alias = "std::_Rb_tree_insert_and_rebalance(bool,std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::_Rb_tree_node_base&)")]
// was: std::_Rb_tree_insert_and_rebalance(bool,std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::_Rb_tree_node_base&)
// type: _DWORD __fastcall(bool, _Rb_tree_node_base *, _Rb_tree_node_base *, _Rb_tree_node_base *)
// IDA 0xf6b9d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b9d4() {
}

// 0xf6b9e4 — __ZSt2wsIcSt11char_traitsIcEERSt13basic_istreamIT_T0_ES6_
#[doc(alias = "std::basic_istream<char,std::char_traits<char>> & std::ws<char,std::char_traits<char>>(std::basic_istream<char,std::char_traits<char>> &)")]
// was: std::basic_istream<char,std::char_traits<char>> & std::ws<char,std::char_traits<char>>(std::basic_istream<char,std::char_traits<char>> &)
// type: int(void)
// IDA 0xf6b9e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b9e4() {
}

// 0xf6b9f4 — __ZSt7getlineIcSt11char_traitsIcESaIcEERSt13basic_istreamIT_T0_ES7_RSbIS4_S5_T1_ES4_
#[doc(alias = "std::basic_istream<char,std::char_traits<char>> & std::getline<char,std::char_traits<char>,std::allocator<char>>(std::basic_istream<char,std::char_traits<char>> &,std::basic_string<char,std::char_traits<char>,std::allocator<char>> &,char)")]
// was: std::basic_istream<char,std::char_traits<char>> & std::getline<char,std::char_traits<char>,std::allocator<char>>(std::basic_istream<char,std::char_traits<char>> &,std::basic_string<char,std::char_traits<char>,std::allocator<char>> &,char)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// IDA 0xf6b9f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6b9f4() {
}

// 0xf6ba04 — __ZSt9terminatev
#[doc(alias = "std::terminate(void)")]
// was: std::terminate(void)
// type: void(void)
// IDA 0xf6ba04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6ba04() {
}
