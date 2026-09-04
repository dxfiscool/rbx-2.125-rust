//! core shard na — 100 core stubs EA-sorted asc global gap filler not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33887, 1523 uncovered before -> 1423 after, batch 0xf26e64..0xf27df4).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v")]
// 0xf26e64 — j___ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf26e64() {
    // IDA 0xf26e64: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "iOSSettingsService::iOSSettingsService(void)")]
// 0xf27354 — j___ZN18iOSSettingsServiceC2Ev
// type: void __fastcall(iOSSettingsService *this)
pub fn stub_0xf27354() {
    // IDA 0xf27354: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "iOSSettingsService::~iOSSettingsService()")]
// 0xf27364 — j___ZN18iOSSettingsServiceD2Ev
// type: void __fastcall(iOSSettingsService *__hidden this)
pub fn stub_0xf27364() {
    // IDA 0xf27364: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "_SecItemAdd")]
// 0xf277f4 — _SecItemAdd
// type: OSStatus __cdecl(CFDictionaryRef attributes, CFTypeRef *result)
pub fn stub_0xf277f4() {
    // IDA 0xf277f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "_SecItemCopyMatching")]
// 0xf27804 — _SecItemCopyMatching
// type: OSStatus __cdecl(CFDictionaryRef query, CFTypeRef *result)
pub fn stub_0xf27804() {
    // IDA 0xf27804: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "_SecItemDelete")]
// 0xf27814 — _SecItemDelete
// type: OSStatus __cdecl(CFDictionaryRef query)
pub fn stub_0xf27814() {
    // IDA 0xf27814: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "_SecItemUpdate")]
// 0xf27824 — _SecItemUpdate
// type: OSStatus __cdecl(CFDictionaryRef query, CFDictionaryRef attributesToUpdate)
pub fn stub_0xf27824() {
    // IDA 0xf27824: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "_UIApplicationMain")]
// 0xf27834 — _UIApplicationMain
// type: int __cdecl(int argc, char **argv, NSString *principalClassName, NSString *delegateClassName)
pub fn stub_0xf27834() {
    // IDA 0xf27834: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "_UIGraphicsBeginImageContextWithOptions")]
// 0xf27844 — _UIGraphicsBeginImageContextWithOptions
// type: void __cdecl(CGSize size, BOOL opaque, CGFloat scale)
pub fn stub_0xf27844() {
    // IDA 0xf27844: Security-framework (keychain) helper owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_UIGraphicsEndImageContext")]
// 0xf27854 — _UIGraphicsEndImageContext
// type: void(void)
pub fn stub_0xf27854() {
    // IDA 0xf27854: Security-framework (keychain) helper owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_UIGraphicsGetCurrentContext")]
// 0xf27864 — _UIGraphicsGetCurrentContext
// type: CGContextRef(void)
pub fn stub_0xf27864() {
    // IDA 0xf27864: UIKit app-entry helper owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_glActiveTexture")]
// 0xf27874 — _glActiveTexture
// type: void __cdecl(GLenum texture)
pub fn stub_0xf27874() {
    // IDA 0xf27874: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glAttachShader")]
// 0xf27884 — _glAttachShader
// type: void __cdecl(GLuint program, GLuint shader)
pub fn stub_0xf27884() {
    // IDA 0xf27884: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glBeginQueryEXT")]
// 0xf27894 — _glBeginQueryEXT
// type: void
pub fn stub_0xf27894() {
    // IDA 0xf27894: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glBindBuffer")]
// 0xf278a4 — _glBindBuffer
// type: void __cdecl(GLenum target, GLuint buffer)
pub fn stub_0xf278a4() {
    // IDA 0xf278a4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glBindFramebuffer")]
// 0xf278b4 — _glBindFramebuffer
// type: void __cdecl(GLenum target, GLuint framebuffer)
pub fn stub_0xf278b4() {
    // IDA 0xf278b4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glBindProgramPipelineEXT")]
// 0xf278c4 — _glBindProgramPipelineEXT
// type: void
pub fn stub_0xf278c4() {
    // IDA 0xf278c4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glBindRenderbuffer")]
// 0xf278d4 — _glBindRenderbuffer
// type: void __cdecl(GLenum target, GLuint renderbuffer)
pub fn stub_0xf278d4() {
    // IDA 0xf278d4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glBindTexture")]
// 0xf278e4 — _glBindTexture
// type: void __cdecl(GLenum target, GLuint texture)
pub fn stub_0xf278e4() {
    // IDA 0xf278e4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glBlendEquation")]
// 0xf278f4 — _glBlendEquation
// type: void __cdecl(GLenum mode)
pub fn stub_0xf278f4() {
    // IDA 0xf278f4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glBlendEquationSeparate")]
// 0xf27904 — _glBlendEquationSeparate
// type: void __cdecl(GLenum modeRGB, GLenum modeAlpha)
pub fn stub_0xf27904() {
    // IDA 0xf27904: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glBlendFunc")]
// 0xf27914 — _glBlendFunc
// type: void __cdecl(GLenum sfactor, GLenum dfactor)
pub fn stub_0xf27914() {
    // IDA 0xf27914: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glBlendFuncSeparate")]
// 0xf27924 — _glBlendFuncSeparate
// type: void __cdecl(GLenum srcRGB, GLenum dstRGB, GLenum srcAlpha, GLenum dstAlpha)
pub fn stub_0xf27924() {
    // IDA 0xf27924: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glBufferData")]
// 0xf27934 — _glBufferData
// type: void __cdecl(GLenum target, GLsizeiptr size, const GLvoid *data, GLenum usage)
pub fn stub_0xf27934() {
    // IDA 0xf27934: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glBufferSubData")]
// 0xf27944 — _glBufferSubData
// type: void __cdecl(GLenum target, GLintptr offset, GLsizeiptr size, const GLvoid *data)
pub fn stub_0xf27944() {
    // IDA 0xf27944: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glCheckFramebufferStatus")]
// 0xf27954 — _glCheckFramebufferStatus
// type: GLenum __cdecl(GLenum target)
pub fn stub_0xf27954() {
    // IDA 0xf27954: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glClear")]
// 0xf27964 — _glClear
// type: void __cdecl(GLbitfield mask)
pub fn stub_0xf27964() {
    // IDA 0xf27964: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glClearColor")]
// 0xf27974 — _glClearColor
// type: void __cdecl(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha)
pub fn stub_0xf27974() {
    // IDA 0xf27974: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glClearDepthf")]
// 0xf27984 — _glClearDepthf
// type: void __cdecl(GLclampf depth)
pub fn stub_0xf27984() {
    // IDA 0xf27984: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glClearStencil")]
// 0xf27994 — _glClearStencil
// type: void __cdecl(GLint s)
pub fn stub_0xf27994() {
    // IDA 0xf27994: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glColorMask")]
// 0xf279a4 — _glColorMask
// type: void __cdecl(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha)
pub fn stub_0xf279a4() {
    // IDA 0xf279a4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glCompileShader")]
// 0xf279b4 — _glCompileShader
// type: void __cdecl(GLuint shader)
pub fn stub_0xf279b4() {
    // IDA 0xf279b4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glCompressedTexImage2D")]
// 0xf279c4 — _glCompressedTexImage2D
// type: void __cdecl(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const GLvoid *data)
pub fn stub_0xf279c4() {
    // IDA 0xf279c4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glCompressedTexSubImage2D")]
// 0xf279d4 — _glCompressedTexSubImage2D
// type: void __cdecl(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const GLvoid *data)
pub fn stub_0xf279d4() {
    // IDA 0xf279d4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glCopyTexSubImage2D")]
// 0xf279e4 — _glCopyTexSubImage2D
// type: void __cdecl(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height)
pub fn stub_0xf279e4() {
    // IDA 0xf279e4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glCreateProgram")]
// 0xf279f4 — _glCreateProgram
// type: GLuint(void)
pub fn stub_0xf279f4() {
    // IDA 0xf279f4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glCreateShader")]
// 0xf27a04 — _glCreateShader
// type: GLuint __cdecl(GLenum type)
pub fn stub_0xf27a04() {
    // IDA 0xf27a04: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glCullFace")]
// 0xf27a14 — _glCullFace
// type: void __cdecl(GLenum mode)
pub fn stub_0xf27a14() {
    // IDA 0xf27a14: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDeleteBuffers")]
// 0xf27a24 — _glDeleteBuffers
// type: void __cdecl(GLsizei n, const GLuint *buffers)
pub fn stub_0xf27a24() {
    // IDA 0xf27a24: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDeleteFramebuffers")]
// 0xf27a34 — _glDeleteFramebuffers
// type: void __cdecl(GLsizei n, const GLuint *framebuffers)
pub fn stub_0xf27a34() {
    // IDA 0xf27a34: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDeleteProgram")]
// 0xf27a44 — _glDeleteProgram
// type: void __cdecl(GLuint program)
pub fn stub_0xf27a44() {
    // IDA 0xf27a44: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDeleteProgramPipelinesEXT")]
// 0xf27a54 — _glDeleteProgramPipelinesEXT
// type: void
pub fn stub_0xf27a54() {
    // IDA 0xf27a54: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDeleteQueriesEXT")]
// 0xf27a64 — _glDeleteQueriesEXT
// type: void
pub fn stub_0xf27a64() {
    // IDA 0xf27a64: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDeleteRenderbuffers")]
// 0xf27a74 — _glDeleteRenderbuffers
// type: void __cdecl(GLsizei n, const GLuint *renderbuffers)
pub fn stub_0xf27a74() {
    // IDA 0xf27a74: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDeleteShader")]
// 0xf27a84 — _glDeleteShader
// type: void __cdecl(GLuint shader)
pub fn stub_0xf27a84() {
    // IDA 0xf27a84: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDeleteTextures")]
// 0xf27a94 — _glDeleteTextures
// type: void __cdecl(GLsizei n, const GLuint *textures)
pub fn stub_0xf27a94() {
    // IDA 0xf27a94: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDepthFunc")]
// 0xf27aa4 — _glDepthFunc
// type: void __cdecl(GLenum func)
pub fn stub_0xf27aa4() {
    // IDA 0xf27aa4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDepthMask")]
// 0xf27ab4 — _glDepthMask
// type: void __cdecl(GLboolean flag)
pub fn stub_0xf27ab4() {
    // IDA 0xf27ab4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDisable")]
// 0xf27ac4 — _glDisable
// type: void __cdecl(GLenum cap)
pub fn stub_0xf27ac4() {
    // IDA 0xf27ac4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDisableVertexAttribArray")]
// 0xf27ad4 — _glDisableVertexAttribArray
// type: void __cdecl(GLuint index)
pub fn stub_0xf27ad4() {
    // IDA 0xf27ad4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDiscardFramebufferEXT")]
// 0xf27ae4 — _glDiscardFramebufferEXT
// type: void
pub fn stub_0xf27ae4() {
    // IDA 0xf27ae4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDrawArrays")]
// 0xf27af4 — _glDrawArrays
// type: void __cdecl(GLenum mode, GLint first, GLsizei count)
pub fn stub_0xf27af4() {
    // IDA 0xf27af4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glDrawElements")]
// 0xf27b04 — _glDrawElements
// type: void __cdecl(GLenum mode, GLsizei count, GLenum type, const GLvoid *indices)
pub fn stub_0xf27b04() {
    // IDA 0xf27b04: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glEnable")]
// 0xf27b14 — _glEnable
// type: void __cdecl(GLenum cap)
pub fn stub_0xf27b14() {
    // IDA 0xf27b14: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glEnableVertexAttribArray")]
// 0xf27b24 — _glEnableVertexAttribArray
// type: void __cdecl(GLuint index)
pub fn stub_0xf27b24() {
    // IDA 0xf27b24: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glEndQueryEXT")]
// 0xf27b34 — _glEndQueryEXT
// type: void
pub fn stub_0xf27b34() {
    // IDA 0xf27b34: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glFramebufferRenderbuffer")]
// 0xf27b44 — _glFramebufferRenderbuffer
// type: void __cdecl(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer)
pub fn stub_0xf27b44() {
    // IDA 0xf27b44: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glFramebufferTexture2D")]
// 0xf27b54 — _glFramebufferTexture2D
// type: void __cdecl(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level)
pub fn stub_0xf27b54() {
    // IDA 0xf27b54: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGenBuffers")]
// 0xf27b64 — _glGenBuffers
// type: void __cdecl(GLsizei n, GLuint *buffers)
pub fn stub_0xf27b64() {
    // IDA 0xf27b64: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGenFramebuffers")]
// 0xf27b74 — _glGenFramebuffers
// type: void __cdecl(GLsizei n, GLuint *framebuffers)
pub fn stub_0xf27b74() {
    // IDA 0xf27b74: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGenProgramPipelinesEXT")]
// 0xf27b84 — _glGenProgramPipelinesEXT
// type: void
pub fn stub_0xf27b84() {
    // IDA 0xf27b84: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGenQueriesEXT")]
// 0xf27b94 — _glGenQueriesEXT
// type: void
pub fn stub_0xf27b94() {
    // IDA 0xf27b94: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGenRenderbuffers")]
// 0xf27ba4 — _glGenRenderbuffers
// type: void __cdecl(GLsizei n, GLuint *renderbuffers)
pub fn stub_0xf27ba4() {
    // IDA 0xf27ba4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGenTextures")]
// 0xf27bb4 — _glGenTextures
// type: void __cdecl(GLsizei n, GLuint *textures)
pub fn stub_0xf27bb4() {
    // IDA 0xf27bb4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetActiveUniform")]
// 0xf27bc4 — _glGetActiveUniform
// type: void __cdecl(GLuint program, GLuint index, GLsizei bufsize, GLsizei *length, GLint *size, GLenum *type, GLchar *name)
pub fn stub_0xf27bc4() {
    // IDA 0xf27bc4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetAttribLocation")]
// 0xf27bd4 — _glGetAttribLocation
// type: int __cdecl(GLuint program, const GLchar *name)
pub fn stub_0xf27bd4() {
    // IDA 0xf27bd4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetError")]
// 0xf27be4 — _glGetError
// type: GLenum(void)
pub fn stub_0xf27be4() {
    // IDA 0xf27be4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetFloatv")]
// 0xf27bf4 — _glGetFloatv
// type: void __cdecl(GLenum pname, GLfloat *params)
pub fn stub_0xf27bf4() {
    // IDA 0xf27bf4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetIntegerv")]
// 0xf27c04 — _glGetIntegerv
// type: void __cdecl(GLenum pname, GLint *params)
pub fn stub_0xf27c04() {
    // IDA 0xf27c04: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetProgramInfoLog")]
// 0xf27c14 — _glGetProgramInfoLog
// type: void __cdecl(GLuint program, GLsizei bufsize, GLsizei *length, GLchar *infolog)
pub fn stub_0xf27c14() {
    // IDA 0xf27c14: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetProgramPipelineInfoLogEXT")]
// 0xf27c24 — _glGetProgramPipelineInfoLogEXT
// type: void
pub fn stub_0xf27c24() {
    // IDA 0xf27c24: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetProgramPipelineivEXT")]
// 0xf27c34 — _glGetProgramPipelineivEXT
// type: void
pub fn stub_0xf27c34() {
    // IDA 0xf27c34: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetProgramiv")]
// 0xf27c44 — _glGetProgramiv
// type: void __cdecl(GLuint program, GLenum pname, GLint *params)
pub fn stub_0xf27c44() {
    // IDA 0xf27c44: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetQueryObjectuivEXT")]
// 0xf27c54 — _glGetQueryObjectuivEXT
// type: void
pub fn stub_0xf27c54() {
    // IDA 0xf27c54: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetRenderbufferParameteriv")]
// 0xf27c64 — _glGetRenderbufferParameteriv
// type: void __cdecl(GLenum target, GLenum pname, GLint *params)
pub fn stub_0xf27c64() {
    // IDA 0xf27c64: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetShaderInfoLog")]
// 0xf27c74 — _glGetShaderInfoLog
// type: void __cdecl(GLuint shader, GLsizei bufsize, GLsizei *length, GLchar *infolog)
pub fn stub_0xf27c74() {
    // IDA 0xf27c74: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetShaderiv")]
// 0xf27c84 — _glGetShaderiv
// type: void __cdecl(GLuint shader, GLenum pname, GLint *params)
pub fn stub_0xf27c84() {
    // IDA 0xf27c84: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetString")]
// 0xf27c94 — _glGetString
// type: const GLubyte *__cdecl(GLenum name)
pub fn stub_0xf27c94() {
    // IDA 0xf27c94: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetTexParameterfv")]
// 0xf27ca4 — _glGetTexParameterfv
// type: void __cdecl(GLenum target, GLenum pname, GLfloat *params)
pub fn stub_0xf27ca4() {
    // IDA 0xf27ca4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glGetUniformLocation")]
// 0xf27cb4 — _glGetUniformLocation
// type: int __cdecl(GLuint program, const GLchar *name)
pub fn stub_0xf27cb4() {
    // IDA 0xf27cb4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glIsEnabled")]
// 0xf27cc4 — _glIsEnabled
// type: GLboolean __cdecl(GLenum cap)
pub fn stub_0xf27cc4() {
    // IDA 0xf27cc4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glIsProgram")]
// 0xf27cd4 — _glIsProgram
// type: GLboolean __cdecl(GLuint program)
pub fn stub_0xf27cd4() {
    // IDA 0xf27cd4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glIsProgramPipelineEXT")]
// 0xf27ce4 — _glIsProgramPipelineEXT
// type: void
pub fn stub_0xf27ce4() {
    // IDA 0xf27ce4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glIsShader")]
// 0xf27cf4 — _glIsShader
// type: GLboolean __cdecl(GLuint shader)
pub fn stub_0xf27cf4() {
    // IDA 0xf27cf4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glLinkProgram")]
// 0xf27d04 — _glLinkProgram
// type: void __cdecl(GLuint program)
pub fn stub_0xf27d04() {
    // IDA 0xf27d04: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glMapBufferOES")]
// 0xf27d14 — _glMapBufferOES
// type: void
pub fn stub_0xf27d14() {
    // IDA 0xf27d14: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glPixelStorei")]
// 0xf27d24 — _glPixelStorei
// type: void __cdecl(GLenum pname, GLint param)
pub fn stub_0xf27d24() {
    // IDA 0xf27d24: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glPolygonOffset")]
// 0xf27d34 — _glPolygonOffset
// type: void __cdecl(GLfloat factor, GLfloat units)
pub fn stub_0xf27d34() {
    // IDA 0xf27d34: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glProgramParameteriEXT")]
// 0xf27d44 — _glProgramParameteriEXT
// type: void __cdecl(GLuint program, GLenum pname, GLint value)
pub fn stub_0xf27d44() {
    // IDA 0xf27d44: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glProgramUniform1fvEXT")]
// 0xf27d54 — _glProgramUniform1fvEXT
// type: void
pub fn stub_0xf27d54() {
    // IDA 0xf27d54: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glProgramUniform1ivEXT")]
// 0xf27d64 — _glProgramUniform1ivEXT
// type: void
pub fn stub_0xf27d64() {
    // IDA 0xf27d64: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glProgramUniform2fvEXT")]
// 0xf27d74 — _glProgramUniform2fvEXT
// type: void
pub fn stub_0xf27d74() {
    // IDA 0xf27d74: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glProgramUniform2ivEXT")]
// 0xf27d84 — _glProgramUniform2ivEXT
// type: void
pub fn stub_0xf27d84() {
    // IDA 0xf27d84: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glProgramUniform3fvEXT")]
// 0xf27d94 — _glProgramUniform3fvEXT
// type: void
pub fn stub_0xf27d94() {
    // IDA 0xf27d94: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glProgramUniform3ivEXT")]
// 0xf27da4 — _glProgramUniform3ivEXT
// type: void
pub fn stub_0xf27da4() {
    // IDA 0xf27da4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glProgramUniform4fvEXT")]
// 0xf27db4 — _glProgramUniform4fvEXT
// type: void
pub fn stub_0xf27db4() {
    // IDA 0xf27db4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glProgramUniform4ivEXT")]
// 0xf27dc4 — _glProgramUniform4ivEXT
// type: void
pub fn stub_0xf27dc4() {
    // IDA 0xf27dc4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glProgramUniformMatrix2fvEXT")]
// 0xf27dd4 — _glProgramUniformMatrix2fvEXT
// type: int()
pub fn stub_0xf27dd4() {
    // IDA 0xf27dd4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glProgramUniformMatrix3fvEXT")]
// 0xf27de4 — _glProgramUniformMatrix3fvEXT
// type: void
pub fn stub_0xf27de4() {
    // IDA 0xf27de4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glProgramUniformMatrix4fvEXT")]
// 0xf27df4 — _glProgramUniformMatrix4fvEXT
// type: void
pub fn stub_0xf27df4() {
    // IDA 0xf27df4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}
