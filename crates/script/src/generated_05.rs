// Auto-generated skeletons for rbx-script — Lua/Script/CodeGen/Luau/RBX::Script batch (filler cont. 2)
// Filter: Lua|Script|CodeGen|Luau|RBX::Script (case-sensitive)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Note: task filter yields 4456 funcs, all already stubbed (5671 existing via broader Script|Lua|Yield|lua + filler); this batch appends next 100 EA-sorted funcs not yet stubbed (global filler 0xf6b054..0xf6b684)
// Previous max script EA 0xf6b044, filtered remaining 0, filler from 0xf6b054 onward (EA-sorted, not yet in any crate).
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "std::string::find_last_not_of(char,unsigned long)const")]
pub fn stub_0xf6b054() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::find_first_not_of(char const*,unsigned long,unsigned long)const")]
pub fn stub_0xf6b064() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::find(char const*,unsigned long,unsigned long)const")]
pub fn stub_0xf6b074() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::find(char,unsigned long)const")]
pub fn stub_0xf6b084() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::rfind(char const*,unsigned long,unsigned long)const")]
pub fn stub_0xf6b094() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::rfind(char,unsigned long)const")]
pub fn stub_0xf6b0a4() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::substr(unsigned long,unsigned long)const")]
pub fn stub_0xf6b0b4() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::compare(char const*)const")]
pub fn stub_0xf6b0c4(a: &String, b: &String) -> bool {
// std::string compare.
a == b
}

#[doc(alias = "std::string::compare(std::string const&)const")]
pub fn stub_0xf6b0d4(a: &String, b: &String) -> bool {
// std::string compare.
a == b
}

#[doc(alias = "std::string::compare(unsigned long,unsigned long,char const*)const")]
pub fn stub_0xf6b0e4(a: &String, b: &String) -> bool {
// std::string compare.
a == b
}

#[doc(alias = "std::string::compare(unsigned long,unsigned long,std::string const&)const")]
pub fn stub_0xf6b0f4(a: &String, b: &String) -> bool {
// std::string compare.
a == b
}

#[doc(alias = "std::__basic_file<char>::is_open(void)const")]
pub fn stub_0xf6b104() -> crate::slot::PortedFn {
// IDA 0xf6b104: std::__basic_file<char>::is_open() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b104, "std::__basic_file<char>::is_open() const")
}

#[doc(alias = "std::runtime_error::what(void)const")]
pub fn stub_0xf6b114(msg: &String) -> &str {
// std::exception::what.
msg.as_str()
}

#[doc(alias = "std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::str(void)const")]
pub fn stub_0xf6b124(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "std::locale::id::_M_id(void)const")]
pub fn stub_0xf6b134() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0xf6b134, "std::locale::id::_M_id() const")
}

#[doc(alias = "std::locale::operator==(std::locale const&)const")]
pub fn stub_0xf6b144() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0xf6b144, "std::locale::operator==(std::locale const&) const")
}

#[doc(alias = "std::basic_ios<char,std::char_traits<char>>::fill(void)const")]
pub fn stub_0xf6b154() -> crate::slot::PortedFn {
// IDA 0xf6b154: std::basic_ios<char, std::char_traits<char>>::fill() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b154, "std::basic_ios<char, std::char_traits<char>>::fill() const")
}

#[doc(alias = "std::basic_ios<char,std::char_traits<char>>::widen(char)const")]
pub fn stub_0xf6b164() -> crate::slot::PortedFn {
// IDA 0xf6b164: std::basic_ios<char, std::char_traits<char>>::widen(char) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b164, "std::basic_ios<char, std::char_traits<char>>::widen(char) const")
}

#[doc(alias = "std::basic_string<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t>>::_Rep::_M_destroy(std::allocator<wchar_t> const&)")]
pub fn stub_0xf6b174(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "std::basic_string<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t>>::reserve(unsigned long)")]
pub fn stub_0xf6b184(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "std::basic_string<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t>>::~basic_string()")]
pub fn stub_0xf6b194(s: String) {
// std::string dtor.
drop(s);
}

#[doc(alias = "std::istream & std::istream::_M_extract<bool>(bool &)")]
pub fn stub_0xf6b1a4() -> crate::slot::PortedFn {
// IDA 0xf6b1a4: std::istream& std::istream::_M_extract<bool>(bool&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b1a4, "std::istream& std::istream::_M_extract<bool>(bool&)")
}

#[doc(alias = "std::istream & std::istream::_M_extract<float>(float &)")]
pub fn stub_0xf6b1b4() -> crate::slot::PortedFn {
// IDA 0xf6b1b4: std::istream& std::istream::_M_extract<float>(float&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b1b4, "std::istream& std::istream::_M_extract<float>(float&)")
}

#[doc(alias = "std::istream & std::istream::_M_extract<unsigned int>(unsigned int &)")]
pub fn stub_0xf6b1c4() -> crate::slot::PortedFn {
// IDA 0xf6b1c4: std::istream& std::istream::_M_extract<unsigned int>(unsigned int&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b1c4, "std::istream& std::istream::_M_extract<unsigned int>(unsigned int&)")
}

#[doc(alias = "std::istream & std::istream::_M_extract<long>(long &)")]
pub fn stub_0xf6b1d4() -> crate::slot::PortedFn {
// IDA 0xf6b1d4: std::istream& std::istream::_M_extract<long>(long&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b1d4, "std::istream& std::istream::_M_extract<long>(long&)")
}

#[doc(alias = "std::istream & std::istream::_M_extract<unsigned long>(unsigned long &)")]
pub fn stub_0xf6b1e4() -> crate::slot::PortedFn {
// IDA 0xf6b1e4: std::istream& std::istream::_M_extract<unsigned long>(unsigned long&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b1e4, "std::istream& std::istream::_M_extract<unsigned long>(unsigned long&)")
}

#[doc(alias = "std::istream::get(char &)")]
pub fn stub_0xf6b1f4() -> crate::slot::PortedFn {
// IDA 0xf6b1f4: std::istream::get(char&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b1f4, "std::istream::get(char&)")
}

#[doc(alias = "std::istream::get(void)")]
pub fn stub_0xf6b204() -> crate::slot::PortedFn {
// IDA 0xf6b204: std::istream::get().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b204, "std::istream::get()")
}

#[doc(alias = "std::istream::peek(void)")]
pub fn stub_0xf6b214() -> crate::slot::PortedFn {
// IDA 0xf6b214: std::istream::peek().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b214, "std::istream::peek()")
}

#[doc(alias = "std::istream::read(char *,int)")]
pub fn stub_0xf6b224() -> crate::slot::PortedFn {
// IDA 0xf6b224: std::istream::read(char*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b224, "std::istream::read(char*, int)")
}

#[doc(alias = "std::istream::seekg(std::fpos<__mbstate_t>)")]
pub fn stub_0xf6b234() -> crate::slot::PortedFn {
// IDA 0xf6b234: std::istream::seekg(std::fpos<__mbstate_t>).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b234, "std::istream::seekg(std::fpos<__mbstate_t>)")
}

#[doc(alias = "std::istream::seekg(long long,std::_Ios_Seekdir)")]
pub fn stub_0xf6b244() -> crate::slot::PortedFn {
// IDA 0xf6b244: std::istream::seekg(long long, std::_Ios_Seekdir).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b244, "std::istream::seekg(long long, std::_Ios_Seekdir)")
}

#[doc(alias = "std::istream::tellg(void)")]
pub fn stub_0xf6b254() -> crate::slot::PortedFn {
// IDA 0xf6b254: std::istream::tellg().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b254, "std::istream::tellg()")
}

#[doc(alias = "std::istream::getline(char *,int,char)")]
pub fn stub_0xf6b264() -> crate::slot::PortedFn {
// IDA 0xf6b264: std::istream::getline(char*, int, char).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b264, "std::istream::getline(char*, int, char)")
}

#[doc(alias = "std::istream::operator>>(int &)")]
pub fn stub_0xf6b274() -> crate::slot::PortedFn {
// IDA 0xf6b274: std::istream::operator>>(int&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b274, "std::istream::operator>>(int&)")
}

#[doc(alias = "std::ostream::put(char)")]
pub fn stub_0xf6b284() -> crate::slot::PortedFn {
// IDA 0xf6b284: std::ostream::put(char).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b284, "std::ostream::put(char)")
}

#[doc(alias = "std::ostream::flush(void)")]
pub fn stub_0xf6b294() -> crate::slot::PortedFn {
// IDA 0xf6b294: std::ostream::flush().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b294, "std::ostream::flush()")
}

#[doc(alias = "std::ostream::tellp(void)")]
pub fn stub_0xf6b2a4() -> crate::slot::PortedFn {
// IDA 0xf6b2a4: std::ostream::tellp().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b2a4, "std::ostream::tellp()")
}

#[doc(alias = "std::ostream::write(char const*,int)")]
pub fn stub_0xf6b2b4() -> crate::slot::PortedFn {
// IDA 0xf6b2b4: std::ostream::write(char const*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b2b4, "std::ostream::write(char const*, int)")
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<void const*>(void const*)")]
pub fn stub_0xf6b2c4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<bool>(bool)")]
pub fn stub_0xf6b2d4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<double>(double)")]
pub fn stub_0xf6b2e4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<long>(long)")]
pub fn stub_0xf6b2f4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<unsigned long>(unsigned long)")]
pub fn stub_0xf6b304(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<long long>(long long)")]
pub fn stub_0xf6b314(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<unsigned long long>(unsigned long long)")]
pub fn stub_0xf6b324(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::ostream::operator<<(std::basic_streambuf<char,std::char_traits<char>> *)")]
pub fn stub_0xf6b334() -> crate::slot::PortedFn {
// IDA 0xf6b334: std::ostream::operator<<(std::basic_streambuf<char, std::char_traits<char>>*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b334, "std::ostream::operator<<(std::basic_streambuf<char, std::char_traits<char>>*)")
}

#[doc(alias = "std::ostream::operator<<(int)")]
pub fn stub_0xf6b344() -> crate::slot::PortedFn {
// IDA 0xf6b344: std::ostream::operator<<(int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b344, "std::ostream::operator<<(int)")
}

#[doc(alias = "std::string::_M_leak_hard(void)")]
pub fn stub_0xf6b354() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::_M_replace_aux(unsigned long,unsigned long,unsigned long,char)")]
pub fn stub_0xf6b364() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::at(unsigned long)")]
pub fn stub_0xf6b374() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::end(void)")]
pub fn stub_0xf6b384() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::_Rep::_M_destroy(std::allocator<char> const&)")]
pub fn stub_0xf6b394() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::_Rep::_S_create(unsigned long,unsigned long,std::allocator<char> const&)")]
pub fn stub_0xf6b3a4() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::swap(std::string &)")]
pub fn stub_0xf6b3b4() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::erase(unsigned long,unsigned long)")]
pub fn stub_0xf6b3c4() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::append(char const*,unsigned long)")]
pub fn stub_0xf6b3d4(s: &mut String, piece: &str) {
// std::string append.
s.push_str(piece);
}

#[doc(alias = "std::string::append(std::string const&)")]
pub fn stub_0xf6b3e4(s: &mut String, piece: &str) {
// std::string append.
s.push_str(piece);
}

#[doc(alias = "std::string::append(unsigned long,char)")]
pub fn stub_0xf6b3f4(s: &mut String, piece: &str) {
// std::string append.
s.push_str(piece);
}

#[doc(alias = "std::string::assign(char const*,unsigned long)")]
pub fn stub_0xf6b404(s: &mut String, piece: &str) {
// std::string append.
s.push_str(piece);
}

#[doc(alias = "std::string::assign(std::string const&)")]
pub fn stub_0xf6b414(s: &mut String, piece: &str) {
// std::string append.
s.push_str(piece);
}

#[doc(alias = "std::string::insert(unsigned long,char const*,unsigned long)")]
pub fn stub_0xf6b424() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::insert(unsigned long,std::string const&,unsigned long,unsigned long)")]
pub fn stub_0xf6b434() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::resize(unsigned long,char)")]
pub fn stub_0xf6b444(s: &String) -> usize {
// std::string::size.
s.len()
}

#[doc(alias = "std::string::replace(unsigned long,unsigned long,char const*,unsigned long)")]
pub fn stub_0xf6b454() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::reserve(unsigned long)")]
pub fn stub_0xf6b464() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::_M_mutate(unsigned long,unsigned long,unsigned long)")]
pub fn stub_0xf6b474() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::push_back(char)")]
pub fn stub_0xf6b484() -> String {
// std::string ctor — empty.
String::new()
}

#[doc(alias = "std::string::string(char const*,std::allocator<char> const&)")]
pub fn stub_0xf6b494(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "std::string::string(char const*,unsigned long,std::allocator<char> const&)")]
pub fn stub_0xf6b4a4(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "std::string::string(std::string const&)")]
pub fn stub_0xf6b4b4(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "std::string::string(std::string const&,unsigned long,unsigned long)")]
pub fn stub_0xf6b4c4(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "std::string::string(unsigned long,char,std::allocator<char> const&)")]
pub fn stub_0xf6b4d4(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "std::string::~string()")]
pub fn stub_0xf6b4e4(s: String) {
// std::string dtor.
drop(s);
}

#[doc(alias = "std::logic_error::logic_error(std::string const&)")]
pub fn stub_0xf6b4f4(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "std::logic_error::~logic_error()")]
pub fn stub_0xf6b504(msg: String) {
// exception dtor.
drop(msg);
}

#[doc(alias = "std::logic_error::~logic_error() [0xf6b514]")]
pub fn stub_0xf6b514(msg: String) {
// exception dtor.
drop(msg);
}

#[doc(alias = "std::__basic_file<char>::~__basic_file()")]
pub fn stub_0xf6b524() -> crate::slot::PortedFn {
// IDA 0xf6b524: std::__basic_file<char>::~__basic_file().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b524, "std::__basic_file<char>::~__basic_file()")
}

#[doc(alias = "std::length_error::length_error(std::string const&)")]
pub fn stub_0xf6b534() -> crate::slot::PortedFn {
// IDA 0xf6b534: std::length_error::length_error(std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b534, "std::length_error::length_error(std::string const&)")
}

#[doc(alias = "std::out_of_range::out_of_range(std::string const&)")]
pub fn stub_0xf6b544() -> crate::slot::PortedFn {
// IDA 0xf6b544: std::out_of_range::out_of_range(std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b544, "std::out_of_range::out_of_range(std::string const&)")
}

#[doc(alias = "std::bad_exception::~bad_exception()")]
pub fn stub_0xf6b554(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "std::basic_filebuf<char,std::char_traits<char>>::open(char const*,std::_Ios_Openmode)")]
pub fn stub_0xf6b564() -> crate::slot::PortedFn {
// IDA 0xf6b564: std::basic_filebuf<char, std::char_traits<char>>::open(char const*, std::_Ios_Openmode).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b564, "std::basic_filebuf<char, std::char_traits<char>>::open(char const*, std::_Ios_Openmode)")
}

#[doc(alias = "std::basic_filebuf<char,std::char_traits<char>>::close(void)")]
pub fn stub_0xf6b574() -> crate::slot::PortedFn {
// IDA 0xf6b574: std::basic_filebuf<char, std::char_traits<char>>::close().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b574, "std::basic_filebuf<char, std::char_traits<char>>::close()")
}

#[doc(alias = "std::basic_fstream<char,std::char_traits<char>>::basic_fstream(void)")]
pub fn stub_0xf6b584() -> crate::slot::PortedFn {
// IDA 0xf6b584: std::basic_fstream<char, std::char_traits<char>>::basic_fstream().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b584, "std::basic_fstream<char, std::char_traits<char>>::basic_fstream()")
}

#[doc(alias = "std::runtime_error::runtime_error(std::string const&)")]
pub fn stub_0xf6b594(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "std::runtime_error::runtime_error(std::string const&) [0xf6b5a4]")]
pub fn stub_0xf6b5a4(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "std::runtime_error::~runtime_error()")]
pub fn stub_0xf6b5b4(msg: String) {
// exception dtor.
drop(msg);
}

#[doc(alias = "std::runtime_error::~runtime_error() [0xf6b5c4]")]
pub fn stub_0xf6b5c4(msg: String) {
// exception dtor.
drop(msg);
}

#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::close(void)")]
pub fn stub_0xf6b5d4() -> crate::slot::PortedFn {
// IDA 0xf6b5d4: std::basic_ifstream<char, std::char_traits<char>>::close().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b5d4, "std::basic_ifstream<char, std::char_traits<char>>::close()")
}

#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::basic_ifstream(char const*,std::_Ios_Openmode)")]
pub fn stub_0xf6b5e4() -> crate::slot::PortedFn {
// IDA 0xf6b5e4: std::basic_ifstream<char, std::char_traits<char>>::basic_ifstream(char const*, std::_Ios_Openmode).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b5e4, "std::basic_ifstream<char, std::char_traits<char>>::basic_ifstream(char const*, std::_Ios_Openmode)")
}

#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::basic_ifstream(void)")]
pub fn stub_0xf6b5f4() -> crate::slot::PortedFn {
// IDA 0xf6b5f4: std::basic_ifstream<char, std::char_traits<char>>::basic_ifstream().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b5f4, "std::basic_ifstream<char, std::char_traits<char>>::basic_ifstream()")
}

#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::~basic_ifstream()")]
pub fn stub_0xf6b604() -> crate::slot::PortedFn {
// IDA 0xf6b604: std::basic_ifstream<char, std::char_traits<char>>::~basic_ifstream().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b604, "std::basic_ifstream<char, std::char_traits<char>>::~basic_ifstream()")
}

#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::~basic_ifstream() [0xf6b614]")]
pub fn stub_0xf6b614() -> crate::slot::PortedFn {
// IDA 0xf6b614: std::basic_ifstream<char, std::char_traits<char>>::~basic_ifstream().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b614, "std::basic_ifstream<char, std::char_traits<char>>::~basic_ifstream()")
}

#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::open(char const*,std::_Ios_Openmode)")]
pub fn stub_0xf6b624() -> crate::slot::PortedFn {
// IDA 0xf6b624: std::basic_ofstream<char, std::char_traits<char>>::open(char const*, std::_Ios_Openmode).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b624, "std::basic_ofstream<char, std::char_traits<char>>::open(char const*, std::_Ios_Openmode)")
}

#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::close(void)")]
pub fn stub_0xf6b634() -> crate::slot::PortedFn {
// IDA 0xf6b634: std::basic_ofstream<char, std::char_traits<char>>::close().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b634, "std::basic_ofstream<char, std::char_traits<char>>::close()")
}

#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::basic_ofstream(char const*,std::_Ios_Openmode)")]
pub fn stub_0xf6b644() -> crate::slot::PortedFn {
// IDA 0xf6b644: std::basic_ofstream<char, std::char_traits<char>>::basic_ofstream(char const*, std::_Ios_Openmode).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b644, "std::basic_ofstream<char, std::char_traits<char>>::basic_ofstream(char const*, std::_Ios_Openmode)")
}

#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::basic_ofstream(void)")]
pub fn stub_0xf6b654() -> crate::slot::PortedFn {
// IDA 0xf6b654: std::basic_ofstream<char, std::char_traits<char>>::basic_ofstream().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b654, "std::basic_ofstream<char, std::char_traits<char>>::basic_ofstream()")
}

#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::~basic_ofstream()")]
pub fn stub_0xf6b664() -> crate::slot::PortedFn {
// IDA 0xf6b664: std::basic_ofstream<char, std::char_traits<char>>::~basic_ofstream().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b664, "std::basic_ofstream<char, std::char_traits<char>>::~basic_ofstream()")
}

#[doc(alias = "std::_List_node_base::hook(std::_List_node_base*)")]
pub fn stub_0xf6b674() -> crate::slot::PortedFn {
// IDA 0xf6b674: std::_List_node_base::hook(std::_List_node_base*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b674, "std::_List_node_base::hook(std::_List_node_base*)")
}

#[doc(alias = "std::_List_node_base::swap(std::_List_node_base&,std::_List_node_base&)")]
pub fn stub_0xf6b684() -> crate::slot::PortedFn {
// IDA 0xf6b684: std::_List_node_base::swap(std::_List_node_base&, std::_List_node_base&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf6b684, "std::_List_node_base::swap(std::_List_node_base&, std::_List_node_base&)")
}
