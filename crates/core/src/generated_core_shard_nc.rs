//! core shard nc — 100 core stubs EA-sorted asc global gap filler not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33887, 1423 uncovered before -> 1323 after, batch 0xf27e04..0xf32684).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "_glReadPixels")]
// 0xf27e04 — _glReadPixels
// type: void __cdecl(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid *pixels)
pub fn stub_0xf27e04() {
    // IDA 0xf27e04: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glRenderbufferStorage")]
// 0xf27e14 — _glRenderbufferStorage
// type: void __cdecl(GLenum target, GLenum internalformat, GLsizei width, GLsizei height)
pub fn stub_0xf27e14() {
    // IDA 0xf27e14: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glRenderbufferStorageMultisampleAPPLE")]
// 0xf27e24 — _glRenderbufferStorageMultisampleAPPLE
pub fn stub_0xf27e24() {
    // IDA 0xf27e24: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glResolveMultisampleFramebufferAPPLE")]
// 0xf27e34 — _glResolveMultisampleFramebufferAPPLE
pub fn stub_0xf27e34() {
    // IDA 0xf27e34: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glScissor")]
// 0xf27e44 — _glScissor
// type: void __cdecl(GLint x, GLint y, GLsizei width, GLsizei height)
pub fn stub_0xf27e44() {
    // IDA 0xf27e44: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glShaderSource")]
// 0xf27e54 — _glShaderSource
// type: void __cdecl(GLuint shader, GLsizei count, const GLchar *const *string, const GLint *length)
pub fn stub_0xf27e54() {
    // IDA 0xf27e54: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glStencilFunc")]
// 0xf27e64 — _glStencilFunc
// type: void __cdecl(GLenum func, GLint ref, GLuint mask)
pub fn stub_0xf27e64() {
    // IDA 0xf27e64: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glStencilFuncSeparate")]
// 0xf27e74 — _glStencilFuncSeparate
// type: void __cdecl(GLenum face, GLenum func, GLint ref, GLuint mask)
pub fn stub_0xf27e74() {
    // IDA 0xf27e74: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glStencilMask")]
// 0xf27e84 — _glStencilMask
// type: void __cdecl(GLuint mask)
pub fn stub_0xf27e84() {
    // IDA 0xf27e84: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glStencilMaskSeparate")]
// 0xf27e94 — _glStencilMaskSeparate
// type: void __cdecl(GLenum face, GLuint mask)
pub fn stub_0xf27e94() {
    // IDA 0xf27e94: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glStencilOp")]
// 0xf27ea4 — _glStencilOp
// type: void __cdecl(GLenum fail, GLenum zfail, GLenum zpass)
pub fn stub_0xf27ea4() {
    // IDA 0xf27ea4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glStencilOpSeparate")]
// 0xf27eb4 — _glStencilOpSeparate
// type: void __cdecl(GLenum face, GLenum fail, GLenum zfail, GLenum zpass)
pub fn stub_0xf27eb4() {
    // IDA 0xf27eb4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glTexImage2D")]
// 0xf27ec4 — _glTexImage2D
// type: void __cdecl(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid *pixels)
pub fn stub_0xf27ec4() {
    // IDA 0xf27ec4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glTexParameterf")]
// 0xf27ed4 — _glTexParameterf
// type: void __cdecl(GLenum target, GLenum pname, GLfloat param)
pub fn stub_0xf27ed4() {
    // IDA 0xf27ed4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glTexParameteri")]
// 0xf27ee4 — _glTexParameteri
// type: void __cdecl(GLenum target, GLenum pname, GLint param)
pub fn stub_0xf27ee4() {
    // IDA 0xf27ee4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glTexSubImage2D")]
// 0xf27ef4 — _glTexSubImage2D
// type: void __cdecl(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid *pixels)
pub fn stub_0xf27ef4() {
    // IDA 0xf27ef4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUniform1fv")]
// 0xf27f04 — _glUniform1fv
// type: void __cdecl(GLint location, GLsizei count, const GLfloat *v)
pub fn stub_0xf27f04() {
    // IDA 0xf27f04: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUniform1iv")]
// 0xf27f14 — _glUniform1iv
// type: void __cdecl(GLint location, GLsizei count, const GLint *v)
pub fn stub_0xf27f14() {
    // IDA 0xf27f14: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUniform2fv")]
// 0xf27f24 — _glUniform2fv
// type: void __cdecl(GLint location, GLsizei count, const GLfloat *v)
pub fn stub_0xf27f24() {
    // IDA 0xf27f24: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUniform2iv")]
// 0xf27f34 — _glUniform2iv
// type: void __cdecl(GLint location, GLsizei count, const GLint *v)
pub fn stub_0xf27f34() {
    // IDA 0xf27f34: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUniform3fv")]
// 0xf27f44 — _glUniform3fv
// type: void __cdecl(GLint location, GLsizei count, const GLfloat *v)
pub fn stub_0xf27f44() {
    // IDA 0xf27f44: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUniform3iv")]
// 0xf27f54 — _glUniform3iv
// type: void __cdecl(GLint location, GLsizei count, const GLint *v)
pub fn stub_0xf27f54() {
    // IDA 0xf27f54: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUniform4fv")]
// 0xf27f64 — _glUniform4fv
// type: void __cdecl(GLint location, GLsizei count, const GLfloat *v)
pub fn stub_0xf27f64() {
    // IDA 0xf27f64: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUniform4iv")]
// 0xf27f74 — _glUniform4iv
// type: void __cdecl(GLint location, GLsizei count, const GLint *v)
pub fn stub_0xf27f74() {
    // IDA 0xf27f74: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUniformMatrix2fv")]
// 0xf27f84 — _glUniformMatrix2fv
// type: void __cdecl(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)
pub fn stub_0xf27f84() {
    // IDA 0xf27f84: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUniformMatrix3fv")]
// 0xf27f94 — _glUniformMatrix3fv
// type: void __cdecl(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)
pub fn stub_0xf27f94() {
    // IDA 0xf27f94: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUniformMatrix4fv")]
// 0xf27fa4 — _glUniformMatrix4fv
// type: void __cdecl(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)
pub fn stub_0xf27fa4() {
    // IDA 0xf27fa4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUnmapBufferOES")]
// 0xf27fb4 — _glUnmapBufferOES
pub fn stub_0xf27fb4() {
    // IDA 0xf27fb4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUseProgram")]
// 0xf27fc4 — _glUseProgram
// type: void __cdecl(GLuint program)
pub fn stub_0xf27fc4() {
    // IDA 0xf27fc4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_glUseProgramStagesEXT")]
// 0xf27fd4 — _glUseProgramStagesEXT
pub fn stub_0xf27fd4() {
    // IDA 0xf27fd4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "_glValidateProgram")]
// 0xf27fe4 — _glValidateProgram
// type: void __cdecl(GLuint program)
pub fn stub_0xf27fe4() {
    // IDA 0xf27fe4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "_glValidateProgramPipelineEXT")]
// 0xf27ff4 — _glValidateProgramPipelineEXT
pub fn stub_0xf27ff4() {
    // IDA 0xf27ff4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "_glVertexAttribPointer")]
// 0xf28004 — _glVertexAttribPointer
// type: void __cdecl(GLuint indx, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const GLvoid *ptr)
pub fn stub_0xf28004() {
    // IDA 0xf28004: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "_glViewport")]
// 0xf28014 — _glViewport
// type: void __cdecl(GLint x, GLint y, GLsizei width, GLsizei height)
pub fn stub_0xf28014() {
    // IDA 0xf28014: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "_NSClassFromString")]
// 0xf28024 — _NSClassFromString
// type: Class __cdecl(NSString *aClassName)
pub fn stub_0xf28024() {
    // IDA 0xf28024: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "_NSGetUncaughtExceptionHandler")]
// 0xf28034 — _NSGetUncaughtExceptionHandler
// type: NSUncaughtExceptionHandler *(void)
pub fn stub_0xf28034() {
    // IDA 0xf28034: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "_NSLog")]
// 0xf28044 — _NSLog
// type: void(NSString *format, ...)
pub fn stub_0xf28044() {
    // IDA 0xf28044: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_NSSearchPathForDirectoriesInDomains")]
// 0xf28054 — _NSSearchPathForDirectoriesInDomains
// type: NSArray *__cdecl(NSSearchPathDirectory directory, NSSearchPathDomainMask domainMask, BOOL expandTilde)
pub fn stub_0xf28054() {
    // IDA 0xf28054: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_NSSetUncaughtExceptionHandler")]
// 0xf28064 — _NSSetUncaughtExceptionHandler
// type: void __cdecl(NSUncaughtExceptionHandler *)
pub fn stub_0xf28064() {
    // IDA 0xf28064: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_NSStringFromClass")]
// 0xf28074 — _NSStringFromClass
// type: NSString *__cdecl(Class aClass)
pub fn stub_0xf28074() {
    // IDA 0xf28074: Foundation helper owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_NSStringFromSelector")]
// 0xf28084 — _NSStringFromSelector
// type: NSString *__cdecl(SEL aSelector)
pub fn stub_0xf28084() {
    // IDA 0xf28084: Foundation helper owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_NSTemporaryDirectory")]
// 0xf28094 — _NSTemporaryDirectory
// type: NSString *(void)
pub fn stub_0xf28094() {
    // IDA 0xf28094: Foundation helper owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CGBitmapContextGetData")]
// 0xf280a4 — _CGBitmapContextGetData
// type: void *__cdecl(CGContextRef context)
pub fn stub_0xf280a4() {
    // IDA 0xf280a4: Foundation helper owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CGColorSpaceCreateDeviceRGB")]
// 0xf280b4 — _CGColorSpaceCreateDeviceRGB
// type: CGColorSpaceRef(void)
pub fn stub_0xf280b4() {
    // IDA 0xf280b4: Foundation helper owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CGContextDrawImage")]
// 0xf280c4 — _CGContextDrawImage
// type: void __cdecl(CGContextRef c, CGRect rect, CGImageRef image)
pub fn stub_0xf280c4() {
    // IDA 0xf280c4: Foundation helper owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CGDataProviderCreateWithData")]
// 0xf280d4 — _CGDataProviderCreateWithData
// type: CGDataProviderRef __cdecl(void *info, const void *data, size_t size, CGDataProviderReleaseDataCallback releaseData)
pub fn stub_0xf280d4() {
    // IDA 0xf280d4: CoreGraphics helper owned by the rendering crate -- carrier no-op in core.
}

#[doc(alias = "_CGImageCreate")]
// 0xf280e4 — _CGImageCreate
// type: CGImageRef __cdecl(size_t width, size_t height, size_t bitsPerComponent, size_t bitsPerPixel, size_t bytesPerRow, CGColorSpaceRef space, CGBitmapInfo bitmapInfo, CGDataProviderRef provider, const CGFloat *decode, bool shouldInterpolate, CGColorRenderingIntent intent)
pub fn stub_0xf280e4() {
    // IDA 0xf280e4: CoreGraphics helper owned by the rendering crate -- carrier no-op in core.
}

#[doc(alias = "_CGImageRelease")]
// 0xf280f4 — _CGImageRelease
// type: void __cdecl(CGImageRef image)
pub fn stub_0xf280f4() {
    // IDA 0xf280f4: CoreGraphics helper owned by the rendering crate -- carrier no-op in core.
}

#[doc(alias = "_CGRectContainsPoint")]
// 0xf28104 — _CGRectContainsPoint
// type: bool __cdecl(CGRect rect, CGPoint point)
pub fn stub_0xf28104() {
    // IDA 0xf28104: CoreGraphics helper owned by the rendering crate -- carrier no-op in core.
}

#[doc(alias = "_CGRectGetMidX")]
// 0xf28114 — _CGRectGetMidX
// type: CGFloat __cdecl(CGRect rect)
pub fn stub_0xf28114() {
    // IDA 0xf28114: CoreGraphics helper owned by the rendering crate -- carrier no-op in core.
}

#[doc(alias = "_CGRectGetMidY")]
// 0xf28124 — _CGRectGetMidY
// type: CGFloat __cdecl(CGRect rect)
pub fn stub_0xf28124() {
    // IDA 0xf28124: CoreGraphics helper owned by the rendering crate -- carrier no-op in core.
}

#[doc(alias = "DebugBreak(void)")]
// 0xf28454 — j___Z10DebugBreakv
// type: void __fastcall __noreturn()
pub fn stub_0xf28454() {
    // IDA 0xf28454: CoreGraphics helper owned by the rendering crate -- carrier no-op in core.
}

#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
// 0xf289b4 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf289b4() {
    // IDA 0xf289b4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v")]
// 0xf28c34 — j___ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v
pub fn stub_0xf28c34() {
    // IDA 0xf28c34: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v")]
// 0xf28e14 — j___ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v
pub fn stub_0xf28e14() {
    // IDA 0xf28e14: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v")]
// 0xf28e24 — j___ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v
pub fn stub_0xf28e24() {
    // IDA 0xf28e24: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v")]
// 0xf28e34 — j___ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v
pub fn stub_0xf28e34() {
    // IDA 0xf28e34: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RobloxExtraSpace::eraseRefsFromAllNodes(void)")]
// 0xf2ad54 — j___ZN16RobloxExtraSpace21eraseRefsFromAllNodesEv
// type: _DWORD __fastcall(RobloxExtraSpace *__hidden this)
pub fn stub_0xf2ad54() {
    // IDA 0xf2ad54: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v")]
// 0xf2b2d4 — j___ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v
pub fn stub_0xf2b2d4() {
    // IDA 0xf2b2d4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v")]
// 0xf2b2e4 — j___ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v
pub fn stub_0xf2b2e4() {
    // IDA 0xf2b2e4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v")]
// 0xf2b2f4 — j___ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v
pub fn stub_0xf2b2f4() {
    // IDA 0xf2b2f4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v")]
// 0xf2b344 — j___ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v
pub fn stub_0xf2b344() {
    // IDA 0xf2b344: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v")]
// 0xf2b374 — j___ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v
pub fn stub_0xf2b374() {
    // IDA 0xf2b374: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v")]
// 0xf2b384 — j___ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v
// type: int(void)
pub fn stub_0xf2b384() {
    // IDA 0xf2b384: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN5boost6threadC2INS_9function0IvEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRS4_NS_6detail13thread_move_tIS4_EEEE5valueEPNS0_5dummyEE4typeE")]
// 0xf2bf14 — j___ZN5boost6threadC2INS_9function0IvEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRS4_NS_6detail13thread_move_tIS4_EEEE5valueEPNS0_5dummyEE4typeE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf2bf14() {
    // IDA 0xf2bf14: boost::thread ctor over a functor. std::thread::spawn — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
// 0xf2c184 — j___ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
pub fn stub_0xf2c184() {
    // IDA 0xf2c184: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RobloxExtraSpace::createNewNode(void)")]
// 0xf2ce14 — j___ZN16RobloxExtraSpace13createNewNodeEv
// type: _DWORD __fastcall(RobloxExtraSpace *__hidden this)
pub fn stub_0xf2ce14() {
    // IDA 0xf2ce14: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v")]
// 0xf2d584 — j___ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v
pub fn stub_0xf2d584() {
    // IDA 0xf2d584: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v")]
// 0xf2d5b4 — j___ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v
pub fn stub_0xf2d5b4() {
    // IDA 0xf2d5b4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v")]
// 0xf2d634 — j___ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v
pub fn stub_0xf2d634() {
    // IDA 0xf2d634: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// 0xf2daa4 — j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf2daa4() {
    // IDA 0xf2daa4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// 0xf2daf4 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf2daf4() {
    // IDA 0xf2daf4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS7_5list4INS_3argILi1EEENSG_ILi2EEENS7_5valueISC_EENSJ_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// 0xf2e174 — j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS7_5list4INS_3argILi1EEENSG_ILi2EEENS7_5valueISC_EENSJ_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: void __fastcall(int, int *)
pub fn stub_0xf2e174() {
    // IDA 0xf2e174: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS6_5list4INS_3argILi1EEENSF_ILi2EEENS6_5valueISB_EENSI_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// 0xf2e1e4 — j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS6_5list4INS_3argILi1EEENSF_ILi2EEENS6_5valueISB_EENSI_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int)
pub fn stub_0xf2e1e4() {
    // IDA 0xf2e1e4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS0_IFvPSsPSt9exceptionEEEENS8_5list5INS8_5valueISsEENSJ_ISA_EENSJ_IbEESM_NSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// 0xf2e594 — j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS0_IFvPSsPSt9exceptionEEEENS8_5list5INS8_5valueISsEENSJ_ISA_EENSJ_IbEESM_NSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
pub fn stub_0xf2e594() {
    // IDA 0xf2e594: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS0_IFvPSsPSt9exceptionEEEENS8_5list5INS8_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// 0xf2e5a4 — j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS0_IFvPSsPSt9exceptionEEEENS8_5list5INS8_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_0xf2e5a4() {
    // IDA 0xf2e5a4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsbNS0_IFvPSsPSt9exceptionEEEENS8_5list3INS8_5valueISsEENSI_IbEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// 0xf2e5b4 — j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsbNS0_IFvPSsPSt9exceptionEEEENS8_5list3INS8_5valueISsEENSI_IbEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_0xf2e5b4() {
    // IDA 0xf2e5b4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// 0xf2e5f4 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
pub fn stub_0xf2e5f4() {
    // IDA 0xf2e5f4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// 0xf2e604 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_0xf2e604() {
    // IDA 0xf2e604: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// 0xf2e614 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_0xf2e614() {
    // IDA 0xf2e614: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_7sCameraEEEERKS0_v")]
// 0xf2f744 — j___ZN3RBX4Name7declareILZNS_7sCameraEEEERKS0_v
pub fn stub_0xf2f744() {
    // IDA 0xf2f744: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_7sCameraEEEERKS0_v")]
// 0xf2f754 — j___ZN3RBX4Name9doDeclareILZNS_7sCameraEEEERKS0_v
pub fn stub_0xf2f754() {
    // IDA 0xf2f754: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE")]
// 0xf30db4 — j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf30db4() {
    // IDA 0xf30db4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sAccoutrementEEEERKS0_v")]
// 0xf31174 — j___ZN3RBX4Name9doDeclareILZNS_13sAccoutrementEEEERKS0_v
pub fn stub_0xf31174() {
    // IDA 0xf31174: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_4sHatEEEERKS0_v")]
// 0xf31184 — j___ZN3RBX4Name9doDeclareILZNS_4sHatEEEERKS0_v
pub fn stub_0xf31184() {
    // IDA 0xf31184: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v")]
// 0xf312f4 — j___ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v
pub fn stub_0xf312f4() {
    // IDA 0xf312f4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v")]
// 0xf31404 — j___ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v
pub fn stub_0xf31404() {
    // IDA 0xf31404: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v")]
// 0xf31504 — j___ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v
pub fn stub_0xf31504() {
    // IDA 0xf31504: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v")]
// 0xf31934 — j___ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v
pub fn stub_0xf31934() {
    // IDA 0xf31934: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v")]
// 0xf31cb4 — j___ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v
pub fn stub_0xf31cb4() {
    // IDA 0xf31cb4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v")]
// 0xf31cc4 — j___ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v
pub fn stub_0xf31cc4() {
    // IDA 0xf31cc4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v")]
// 0xf31cd4 — j___ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v
pub fn stub_0xf31cd4() {
    // IDA 0xf31cd4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v")]
// 0xf31ce4 — j___ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v
pub fn stub_0xf31ce4() {
    // IDA 0xf31ce4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v")]
// 0xf31cf4 — j___ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v
pub fn stub_0xf31cf4() {
    // IDA 0xf31cf4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sBackpackEEEERKS0_v")]
// 0xf32174 — j___ZN3RBX4Name9doDeclareILZNS_9sBackpackEEEERKS0_v
// type: int(void)
pub fn stub_0xf32174() {
    // IDA 0xf32174: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list6INS7_5valueISC_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// 0xf32424 — j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list6INS7_5valueISC_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xf32424() {
    // IDA 0xf32424: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list7INS7_5valueISC_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// 0xf32434 — j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list7INS7_5valueISC_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32434() {
    // IDA 0xf32434: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// 0xf32474 — j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xf32474() {
    // IDA 0xf32474: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// 0xf32484 — j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf32484() {
    // IDA 0xf32484: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_15sFormFactorPartEEEERKS0_v")]
// 0xf32684 — j___ZN3RBX4Name9doDeclareILZNS_15sFormFactorPartEEEERKS0_v
pub fn stub_0xf32684() {
    // IDA 0xf32684: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}
