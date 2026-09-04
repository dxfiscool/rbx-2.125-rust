//! network generated_20 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (4797 strict, 6004 with replica/remote expansion for BG5, 120 stubs here, 5199+120=5319 total, shard BG5, EA-sorted ascending earliest gap).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Boost types mapped: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> Weak, with // was: original.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;

/// `RBX::Reflection::RemoteEventCommon::Attributes` resolved state
/// (IDA 0x25f66c): functionality id, deprecated flag, member descriptor.
#[derive(Debug, Default)]
pub struct RemoteEventAttributes {
    pub deprecated: bool,
    pub functionality: u32,
    pub descriptor: usize,
}

/// `RBX::Reflection::EventDesc` constructor parts (IDA 0x39f984 et al.):
/// the signal member, descriptor names, permissions and attributes.
/// The vtable installs and signature-list builds stay engine-side.
#[derive(Debug, Clone)]
pub struct EventDescriptorInit<'a> {
    pub member: usize,
    pub names: &'a [&'a str],
    pub permissions: u32,
    pub attributes: u32,
}

/// `ChatService.Chatted` argument bundle
/// (`(shared_ptr<Instance>, string, ChatColor)`, IDA 0x3ed188).
#[derive(Debug, Clone)]
pub struct ChatEventArgs {
    pub speaker: SharedPtr<()>,
    pub text: String,
    pub color: u32,
}

/// `(G3D::Vector3::Axis, float, float)` argument bundle (IDA 0x3ad048).
#[derive(Debug, Clone, Copy)]
pub struct AxisFfArgs {
    pub axis: u32,
    pub a: f32,
    pub b: f32,
}



// 0x25f66c — __ZN3RBX10Reflection17RemoteEventCommon10Attributes10deprecatedENS1_13FunctionalityEPKNS0_16MemberDescriptorE
// type: int __fastcall(int result, int, int)
#[doc(alias = "__ZN3RBX10Reflection17RemoteEventCommon10Attributes10deprecatedENS1_13FunctionalityEPKNS0_16MemberDescriptorE")]
pub fn stub_25f66c(attrs: &mut RemoteEventAttributes, functionality: u32, descriptor: usize) -> &mut RemoteEventAttributes {
    // IDA 0x25f66c: *(result+8) = functionality (0x25f66e); *result = 1 (0x25f670); *(result+4) = descriptor (0x25f672); returns result (0x25f674).
    attrs.functionality = functionality;
    attrs.deprecated = true;
    attrs.descriptor = descriptor;
    attrs
}


// 0x25f678 — __ZN3RBX10Reflection11EventSource18processRemoteEventERKNS0_15EventDescriptorERKSt6vectorINS0_7VariantESaIS6_EERKNS_13SystemAddressE
// type: int __fastcall(int, int)
#[doc(alias = "__ZN3RBX10Reflection11EventSource18processRemoteEventERKNS0_15EventDescriptorERKSt6vectorINS0_7VariantESaIS6_EERKNS_13SystemAddressE")]
pub fn stub_25f678(dispatch: &mut dyn FnMut(usize, usize) -> i32, source: usize, descriptor: usize) -> i32 {
    // IDA 0x25f678: (*(desc.vtbl + 20))(desc, source).
    dispatch(descriptor, source)
}


// 0x39c124 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_39c124(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x39c124: resets the vtable then SignatureDescriptor list clear (cf. 0x39c13c..0x39c140).
    clear_signatures(this);
}


// 0x39c148 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_39c148(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x39c148: resets the vtable then SignatureDescriptor list clear (cf. 0x39c13c..0x39c140).
    clear_signatures(this);
}


// 0x39c16c — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_39c16c(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x39c16c: resets the vtable then SignatureDescriptor list clear (cf. 0x39c13c..0x39c140).
    clear_signatures(this);
}


// 0x39c190 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_39c190(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x39c190: resets the vtable then SignatureDescriptor list clear (cf. 0x39c13c..0x39c140).
    clear_signatures(this);
}


// 0x39c724 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss
// type: void __fastcall(int, int, std::string *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss")]
pub fn stub_39c724(signal_offset: usize, source: usize, value: &str, emit: &mut dyn FnMut(usize, String)) {
    // IDA 0x39c724: copies the string then signal_with_args<1>::operator()(source + *(this + 40)) (cf. 0x39c744..0x39c780).
    emit(source + signal_offset, value.to_owned());
}


// 0x39ec94 — __ZN3rbx13remote_signalIFvfffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvfffEEC2Ev")]
pub fn stub_39ec94(init_signal: &mut dyn FnMut()) {
    // IDA 0x39ec94: zero-inits the signal word then call_once's the signal mutex init (cf. 0x39ecc4..0x39ed34); the boost mutex stays engine-side.
    // was: rbx::remote_signal<...>::remote_signal.
    init_signal();
}


// 0x39ef68 — __ZN3rbx13remote_signalIFvffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvffEEC2Ev")]
pub fn stub_39ef68(init_signal: &mut dyn FnMut()) {
    // IDA 0x39ef68: zero-inits the signal word then call_once's the signal mutex init (cf. 0x39ecc4..0x39ed34); the boost mutex stays engine-side.
    // was: rbx::remote_signal<...>::remote_signal.
    init_signal();
}


// 0x39f0c4 — __ZN3rbx13remote_signalIFvffffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvffffEEC2Ev")]
pub fn stub_39f0c4(init_signal: &mut dyn FnMut()) {
    // IDA 0x39f0c4: zero-inits the signal word then call_once's the signal mutex init (cf. 0x39ecc4..0x39ed34); the boost mutex stays engine-side.
    // was: rbx::remote_signal<...>::remote_signal.
    init_signal();
}


// 0x39f594 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_39f594(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x39f594: D1 body then operator delete (cf. 0x39f5d2..0x39f5fe).
    clear_signatures(this);
    free(this);
}


// 0x39f648 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_39f648(slot: SharedPtr<()>, connect: &mut dyn FnMut(SharedPtr<()>)) {
    // IDA 0x39f648: retains the shared_ptr slot, binds GenericSlotWrapper::executeN and connects it to the signal (cf. 0x39f674..0x39f6c0); the boost bind/functor stays engine-side.
    // was: boost::shared_ptr<RBX::Reflection::GenericSlotWrapper> retained + bound.
    connect(slot);
}


// 0x39f7ac — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_39f7ac(scriptable_flags: u32) -> bool {
    // IDA 0x39f7ac: *(_DWORD *)(this + 48) & 1 (cf. 0x39f7b2).
    scriptable_flags & 1 == 1
}


// 0x39f7b4 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_39f7b4(broadcast_flags: u32) -> bool {
    // IDA 0x39f7b4: *(_DWORD *)(this + 44) & 1 (cf. 0x39f7ba).
    broadcast_flags & 1 == 1
}


// 0x39f7bc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_39f7bc(signal_offset: usize, source: Option<usize>, args: &[String], emit: &mut dyn FnMut(usize, &str)) {
    // IDA 0x39f7bc: asserts args.size() == 1 (Event.h:320, cf. 0x39f7f8..0x39f852); base = source ? source - 36 : 0; any_cast<string>; signal_with_args<1>::operator() (cf. 0x39f86c..0x39f89e).
    assert!(args.len() == 1, "args.size() == 1 Event.h:320");
    let base = source.map(|s| s - 36).unwrap_or(0);
    emit(base + signal_offset, &args[0]);
}


// 0x39f960 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_39f960(dispatch: &mut dyn FnMut(usize, usize, usize, u32) -> i32, source: usize, descriptor: usize, args: usize) -> i32 {
    // IDA 0x39f960: (*(source.vtbl + 12))(source, descriptor, args, 0) (cf. 0x39f960).
    dispatch(source, descriptor, args, 0)
}


// 0x39f970 — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_39f970(signal_offset: usize, source: Option<usize>, disconnect: &mut dyn FnMut(usize) -> i32) -> i32 {
    // IDA 0x39f970: base = source ? source - 36 : 0 then signal::disconnectAll(base + *(this + 40)) (cf. 0x39f970..0x39f976).
    let base = source.map(|s| s - 36).unwrap_or(0);
    disconnect(base + signal_offset)
}


// 0x39f984 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_39f984<'a>(member: usize, names: &'a [&'a str], permissions: u32, attributes: u32) -> EventDescriptorInit<'a> {
    // IDA 0x39f984: Described classDescriptor + EventDescriptor init, vtable install and signature build (cf. 0x39f9bc..0x39fa02); the reflection tables stay engine-side.
    EventDescriptorInit { member, names, permissions, attributes }
}


// 0x39fb08 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_39fb08(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x39fb08: resets the vtable then SignatureDescriptor list clear (cf. 0x39fb20..0x39fb24).
    clear_signatures(this);
}


// 0x39fb2c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_39fb2c(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x39fb2c: D1 body then operator delete (cf. 0x39fb6a..0x39fb96).
    clear_signatures(this);
    free(this);
}


// 0x39fbe0 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_39fbe0(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x39fbe0: D1 body then operator delete (cf. 0x39f5d2..0x39f5fe).
    clear_signatures(this);
    free(this);
}


// 0x39fc94 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_39fc94(slot: SharedPtr<()>, connect: &mut dyn FnMut(SharedPtr<()>)) {
    // IDA 0x39fc94: retains the shared_ptr slot, binds GenericSlotWrapper::executeN and connects it to the signal (cf. 0x39f674..0x39f6c0); the boost bind/functor stays engine-side.
    // was: boost::shared_ptr<RBX::Reflection::GenericSlotWrapper> retained + bound.
    connect(slot);
}


// 0x39fdf8 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_39fdf8(scriptable_flags: u32) -> bool {
    // IDA 0x39fdf8: *(_DWORD *)(this + 48) & 1 (cf. 0x39f7b2).
    scriptable_flags & 1 == 1
}


// 0x39fe00 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_39fe00(broadcast_flags: u32) -> bool {
    // IDA 0x39fe00: *(_DWORD *)(this + 44) & 1 (cf. 0x39f7ba).
    broadcast_flags & 1 == 1
}


// 0x39fe08 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_39fe08(signal_offset: usize, source: Option<usize>, args: &[f32], emit: &mut dyn FnMut(usize, &[f32])) {
    // IDA 0x39fe08: asserts args.size() == 3 (Event.h:322, cf. 0x3a1b84..0x3a1bbe); base = source ? source - 36 : 0; any_cast<float> x3; signal_with_args<3>::operator() (cf. 0x3a1bd0..0x3a1c28).
    assert!(args.len() == 3, "args.size() == 3 Event.h:322");
    let base = source.map(|s| s - 36).unwrap_or(0);
    emit(base + signal_offset, args);
}


// 0x39feb0 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_39feb0(dispatch: &mut dyn FnMut(usize, usize, usize, u32) -> i32, source: usize, descriptor: usize, args: usize) -> i32 {
    // IDA 0x39feb0: (*(source.vtbl + 12))(source, descriptor, args, 0) (cf. 0x39f960).
    dispatch(source, descriptor, args, 0)
}


// 0x39fec0 — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_39fec0(signal_offset: usize, source: Option<usize>, disconnect: &mut dyn FnMut(usize) -> i32) -> i32 {
    // IDA 0x39fec0: base = source ? source - 36 : 0 then signal::disconnectAll(base + *(this + 40)) (cf. 0x39f970..0x39f976).
    let base = source.map(|s| s - 36).unwrap_or(0);
    disconnect(base + signal_offset)
}


// 0x3a105c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3a105c<'a>(member: usize, names: &'a [&'a str], permissions: u32, attributes: u32) -> EventDescriptorInit<'a> {
    // IDA 0x3a105c: Described classDescriptor + EventDescriptor init, vtable install and signature build (cf. 0x39f9bc..0x39fa02); the reflection tables stay engine-side.
    EventDescriptorInit { member, names, permissions, attributes }
}


// 0x3a12b8 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_3a12b8(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3a12b8: resets the vtable then SignatureDescriptor list clear (cf. 0x39fb20..0x39fb24).
    clear_signatures(this);
}


// 0x3a12dc — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_3a12dc(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3a12dc: D1 body then operator delete (cf. 0x39fb6a..0x39fb96).
    clear_signatures(this);
    free(this);
}


// 0x3a1390 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_3a1390(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3a1390: D1 body then operator delete (cf. 0x39f5d2..0x39f5fe).
    clear_signatures(this);
    free(this);
}


// 0x3a1444 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_3a1444(slot: SharedPtr<()>, connect: &mut dyn FnMut(SharedPtr<()>)) {
    // IDA 0x3a1444: retains the shared_ptr slot, binds GenericSlotWrapper::executeN and connects it to the signal (cf. 0x39f674..0x39f6c0); the boost bind/functor stays engine-side.
    // was: boost::shared_ptr<RBX::Reflection::GenericSlotWrapper> retained + bound.
    connect(slot);
}


// 0x3a15a8 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_3a15a8(scriptable_flags: u32) -> bool {
    // IDA 0x3a15a8: *(_DWORD *)(this + 48) & 1 (cf. 0x39f7b2).
    scriptable_flags & 1 == 1
}


// 0x3a15b0 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_3a15b0(broadcast_flags: u32) -> bool {
    // IDA 0x3a15b0: *(_DWORD *)(this + 44) & 1 (cf. 0x39f7ba).
    broadcast_flags & 1 == 1
}


// 0x3a15b8 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_3a15b8(signal_offset: usize, source: Option<usize>, args: &[f32], emit: &mut dyn FnMut(usize, &[f32])) {
    // IDA 0x3a15b8: asserts args.size() == 2 (Event.h:321, cf. 0x3a1b84..0x3a1bbe); base = source ? source - 36 : 0; any_cast<float> x2; signal_with_args<2>::operator() (cf. 0x3a1bd0..0x3a1c28).
    assert!(args.len() == 2, "args.size() == 2 Event.h:321");
    let base = source.map(|s| s - 36).unwrap_or(0);
    emit(base + signal_offset, args);
}


// 0x3a1654 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_3a1654(dispatch: &mut dyn FnMut(usize, usize, usize, u32) -> i32, source: usize, descriptor: usize, args: usize) -> i32 {
    // IDA 0x3a1654: (*(source.vtbl + 12))(source, descriptor, args, 0) (cf. 0x39f960).
    dispatch(source, descriptor, args, 0)
}


// 0x3a1664 — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3a1664(signal_offset: usize, source: Option<usize>, disconnect: &mut dyn FnMut(usize) -> i32) -> i32 {
    // IDA 0x3a1664: base = source ? source - 36 : 0 then signal::disconnectAll(base + *(this + 40)) (cf. 0x39f970..0x39f976).
    let base = source.map(|s| s - 36).unwrap_or(0);
    disconnect(base + signal_offset)
}


// 0x3a1678 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3a1678<'a>(member: usize, names: &'a [&'a str], permissions: u32, attributes: u32) -> EventDescriptorInit<'a> {
    // IDA 0x3a1678: Described classDescriptor + EventDescriptor init, vtable install and signature build (cf. 0x39f9bc..0x39fa02); the reflection tables stay engine-side.
    EventDescriptorInit { member, names, permissions, attributes }
}


// 0x3a1868 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_3a1868(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3a1868: resets the vtable then SignatureDescriptor list clear (cf. 0x39fb20..0x39fb24).
    clear_signatures(this);
}


// 0x3a188c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_3a188c(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3a188c: D1 body then operator delete (cf. 0x39fb6a..0x39fb96).
    clear_signatures(this);
    free(this);
}


// 0x3a1940 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_3a1940(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3a1940: D1 body then operator delete (cf. 0x39f5d2..0x39f5fe).
    clear_signatures(this);
    free(this);
}


// 0x3a19f4 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_3a19f4(slot: SharedPtr<()>, connect: &mut dyn FnMut(SharedPtr<()>)) {
    // IDA 0x3a19f4: retains the shared_ptr slot, binds GenericSlotWrapper::executeN and connects it to the signal (cf. 0x39f674..0x39f6c0); the boost bind/functor stays engine-side.
    // was: boost::shared_ptr<RBX::Reflection::GenericSlotWrapper> retained + bound.
    connect(slot);
}


// 0x3a1b58 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_3a1b58(scriptable_flags: u32) -> bool {
    // IDA 0x3a1b58: *(_DWORD *)(this + 48) & 1 (cf. 0x39f7b2).
    scriptable_flags & 1 == 1
}


// 0x3a1b60 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_3a1b60(broadcast_flags: u32) -> bool {
    // IDA 0x3a1b60: *(_DWORD *)(this + 44) & 1 (cf. 0x39f7ba).
    broadcast_flags & 1 == 1
}


// 0x3a1b68 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_3a1b68(signal_offset: usize, source: Option<usize>, args: &[f32], emit: &mut dyn FnMut(usize, &[f32])) {
    // IDA 0x3a1b68: asserts args.size() == 4 (Event.h:413, cf. 0x3a1b84..0x3a1bbe); base = source ? source - 36 : 0; any_cast<float> x4; signal_with_args<4>::operator() (cf. 0x3a1bd0..0x3a1c28).
    assert!(args.len() == 4, "args.size() == 4 Event.h:413");
    let base = source.map(|s| s - 36).unwrap_or(0);
    emit(base + signal_offset, args);
}


// 0x3a1c2c — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_3a1c2c(dispatch: &mut dyn FnMut(usize, usize, usize, u32) -> i32, source: usize, descriptor: usize, args: usize) -> i32 {
    // IDA 0x3a1c2c: (*(source.vtbl + 12))(source, descriptor, args, 0) (cf. 0x39f960).
    dispatch(source, descriptor, args, 0)
}


// 0x3a1c3c — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3a1c3c(signal_offset: usize, source: Option<usize>, disconnect: &mut dyn FnMut(usize) -> i32) -> i32 {
    // IDA 0x3a1c3c: base = source ? source - 36 : 0 then signal::disconnectAll(base + *(this + 40)) (cf. 0x39f970..0x39f976).
    let base = source.map(|s| s - 36).unwrap_or(0);
    disconnect(base + signal_offset)
}


// 0x3a2e1c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3a2e1c<'a>(member: usize, names: &'a [&'a str], permissions: u32, attributes: u32) -> EventDescriptorInit<'a> {
    // IDA 0x3a2e1c: Described classDescriptor + EventDescriptor init, vtable install and signature build (cf. 0x39f9bc..0x39fa02); the reflection tables stay engine-side.
    EventDescriptorInit { member, names, permissions, attributes }
}


// 0x3a30e8 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_3a30e8(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3a30e8: resets the vtable then SignatureDescriptor list clear (cf. 0x39fb20..0x39fb24).
    clear_signatures(this);
}


// 0x3a310c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_3a310c(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3a310c: D1 body then operator delete (cf. 0x39fb6a..0x39fb96).
    clear_signatures(this);
    free(this);
}


// 0x3a31c0 — __ZN3rbx13remote_signalIFvffffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvffffEED2Ev")]
pub fn stub_3a31c0(disconnect: &mut dyn FnMut(), release_slots: &mut dyn FnMut()) {
    // IDA 0x3a31c0: disconnectAll on both inner signals then intrusive_ptr_release of the slots (cf. 0x3a3216..0x3a323c).
    // was: rbx::remote_signal<...>::~remote_signal.
    disconnect();
    release_slots();
}


// 0x3a330c — __ZN3rbx13remote_signalIFvfffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvfffEED2Ev")]
pub fn stub_3a330c(disconnect: &mut dyn FnMut(), release_slots: &mut dyn FnMut()) {
    // IDA 0x3a330c: disconnectAll on both inner signals then intrusive_ptr_release of the slots (cf. 0x3a3216..0x3a323c).
    // was: rbx::remote_signal<...>::~remote_signal.
    disconnect();
    release_slots();
}


// 0x3a3458 — __ZN3rbx13remote_signalIFvffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvffEED2Ev")]
pub fn stub_3a3458(disconnect: &mut dyn FnMut(), release_slots: &mut dyn FnMut()) {
    // IDA 0x3a3458: disconnectAll on both inner signals then intrusive_ptr_release of the slots (cf. 0x3a3216..0x3a323c).
    // was: rbx::remote_signal<...>::~remote_signal.
    disconnect();
    release_slots();
}


// 0x3a7f20 — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEED1Ev")]
pub fn stub_3a7f20(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3a7f20: resets the vtable then SignatureDescriptor list clear (cf. 0x39c13c..0x39c140).
    clear_signatures(this);
}


// 0x3a7f44 — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEED1Ev")]
pub fn stub_3a7f44(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3a7f44: resets the vtable then SignatureDescriptor list clear (cf. 0x39c13c..0x39c140).
    clear_signatures(this);
}


// 0x3a9ea0 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEEC2Ev")]
pub fn stub_3a9ea0(init_signal: &mut dyn FnMut()) {
    // IDA 0x3a9ea0: zero-inits the signal word then call_once's the signal mutex init (cf. 0x39ecc4..0x39ed34); the boost mutex stays engine-side.
    // was: rbx::remote_signal<...>::remote_signal.
    init_signal();
}


// 0x3aa174 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEEC2Ev")]
pub fn stub_3aa174(init_signal: &mut dyn FnMut()) {
    // IDA 0x3aa174: zero-inits the signal word then call_once's the signal mutex init (cf. 0x39ecc4..0x39ed34); the boost mutex stays engine-side.
    // was: rbx::remote_signal<...>::remote_signal.
    init_signal();
}


// 0x3aa53c — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE12getSignalPtrEPNS0_11EventSourceE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_3aa53c(signal_base: usize, source: Option<usize>) -> usize {
    // IDA 0x3aa53c: source ? *(this + 40) + source - 36 (0x3aa540..0x3aa546) : ReleaseAssert("0", Event.h:797) with null return (0x3aa550..0x3aa5a0).
    match source {
        Some(s) => signal_base + s - 36,
        None => {
            debug_assert!(false, "0 Event.h:797");
            0
        }
    }
}


// 0x3ab198 — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE12getSignalPtrEPNS0_11EventSourceE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_3ab198(signal_base: usize, source: Option<usize>) -> usize {
    // IDA 0x3ab198: source ? *(this + 40) + source - 36 (0x3aa540..0x3aa546) : ReleaseAssert("0", Event.h:797) with null return (0x3aa550..0x3aa5a0).
    match source {
        Some(s) => signal_base + s - 36,
        None => {
            debug_assert!(false, "0 Event.h:797");
            0
        }
    }
}


// 0x3ace20 — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEED0Ev")]
pub fn stub_3ace20(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3ace20: D1 body then operator delete (cf. 0x39f5d2..0x39f5fe).
    clear_signatures(this);
    free(this);
}


// 0x3aced4 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_3aced4(slot: SharedPtr<()>, connect: &mut dyn FnMut(SharedPtr<()>)) {
    // IDA 0x3aced4: retains the shared_ptr slot, binds GenericSlotWrapper::executeN and connects it to the signal (cf. 0x39f674..0x39f6c0); the boost bind/functor stays engine-side.
    // was: boost::shared_ptr<RBX::Reflection::GenericSlotWrapper> retained + bound.
    connect(slot);
}


// 0x3ad038 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE12isScriptableEv")]
pub fn stub_3ad038(scriptable_flags: u32) -> bool {
    // IDA 0x3ad038: *(_DWORD *)(this + 48) & 1 (cf. 0x39f7b2).
    scriptable_flags & 1 == 1
}


// 0x3ad040 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE11isBroadcastEv")]
pub fn stub_3ad040(broadcast_flags: u32) -> bool {
    // IDA 0x3ad040: *(_DWORD *)(this + 44) & 1 (cf. 0x39f7ba).
    broadcast_flags & 1 == 1
}


// 0x3ad048 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE")]
pub fn stub_3ad048(signal_offset: usize, source: Option<usize>, args: AxisFfArgs, emit: &mut dyn FnMut(usize, AxisFfArgs)) {
    // IDA 0x3ad048: asserts args.size() == 3 (Event.h:322); base = source ? source - 36 : 0; any_cast<Axis,float,float>; signal_with_args<3>::operator() (cf. 0x3ad048 shape).
    let base = source.map(|s| s - 36).unwrap_or(0);
    emit(base + signal_offset, args);
}


// 0x3ad0f0 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE")]
pub fn stub_3ad0f0(dispatch: &mut dyn FnMut(usize, usize, usize, u32) -> i32, source: usize, descriptor: usize, args: usize) -> i32 {
    // IDA 0x3ad0f0: (*(source.vtbl + 12))(source, descriptor, args, 0) (cf. 0x39f960).
    dispatch(source, descriptor, args, 0)
}


// 0x3ad100 — __ZNK3RBX10Reflection13EventDescBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3ad100(signal_offset: usize, source: Option<usize>, disconnect: &mut dyn FnMut(usize) -> i32) -> i32 {
    // IDA 0x3ad100: base = source ? source - 36 : 0 then signal::disconnectAll(base + *(this + 40)) (cf. 0x39f970..0x39f976).
    let base = source.map(|s| s - 36).unwrap_or(0);
    disconnect(base + signal_offset)
}


// 0x3ae298 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_EC2ESA_PKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_EC2ESA_PKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3ae298<'a>(member: usize, names: &'a [&'a str], permissions: u32, attributes: u32) -> EventDescriptorInit<'a> {
    // IDA 0x3ae298: Described classDescriptor + EventDescriptor init, vtable install and signature build (cf. 0x39f9bc..0x39fa02); the reflection tables stay engine-side.
    EventDescriptorInit { member, names, permissions, attributes }
}


// 0x3ae4f4 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_ED1Ev")]
pub fn stub_3ae4f4(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3ae4f4: resets the vtable then SignatureDescriptor list clear (cf. 0x39fb20..0x39fb24).
    clear_signatures(this);
}


// 0x3ae518 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_ED0Ev")]
pub fn stub_3ae518(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3ae518: D1 body then operator delete (cf. 0x39fb6a..0x39fb96).
    clear_signatures(this);
    free(this);
}


// 0x3ae5cc — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEED0Ev")]
pub fn stub_3ae5cc(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3ae5cc: D1 body then operator delete (cf. 0x39f5d2..0x39f5fe).
    clear_signatures(this);
    free(this);
}


// 0x3ae680 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_3ae680(slot: SharedPtr<()>, connect: &mut dyn FnMut(SharedPtr<()>)) {
    // IDA 0x3ae680: retains the shared_ptr slot, binds GenericSlotWrapper::executeN and connects it to the signal (cf. 0x39f674..0x39f6c0); the boost bind/functor stays engine-side.
    // was: boost::shared_ptr<RBX::Reflection::GenericSlotWrapper> retained + bound.
    connect(slot);
}


// 0x3ae7e4 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE12isScriptableEv")]
pub fn stub_3ae7e4(scriptable_flags: u32) -> bool {
    // IDA 0x3ae7e4: *(_DWORD *)(this + 48) & 1 (cf. 0x39f7b2).
    scriptable_flags & 1 == 1
}


// 0x3ae7ec — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE11isBroadcastEv")]
pub fn stub_3ae7ec(broadcast_flags: u32) -> bool {
    // IDA 0x3ae7ec: *(_DWORD *)(this + 44) & 1 (cf. 0x39f7ba).
    broadcast_flags & 1 == 1
}


// 0x3ae7f4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE")]
pub fn stub_3ae7f4(signal_offset: usize, source: Option<usize>, axis: u32, emit: &mut dyn FnMut(usize, u32)) {
    // IDA 0x3ae7f4: asserts args.size() == 1 (Event.h:320); base = source ? source - 36 : 0; any_cast<Axis>; signal_with_args<1>::operator().
    let base = source.map(|s| s - 36).unwrap_or(0);
    emit(base + signal_offset, axis);
}


// 0x3ae880 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE")]
pub fn stub_3ae880(dispatch: &mut dyn FnMut(usize, usize, usize, u32) -> i32, source: usize, descriptor: usize, args: usize) -> i32 {
    // IDA 0x3ae880: (*(source.vtbl + 12))(source, descriptor, args, 0) (cf. 0x39f960).
    dispatch(source, descriptor, args, 0)
}


// 0x3ae890 — __ZNK3RBX10Reflection13EventDescBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3ae890(signal_offset: usize, source: Option<usize>, disconnect: &mut dyn FnMut(usize) -> i32) -> i32 {
    // IDA 0x3ae890: base = source ? source - 36 : 0 then signal::disconnectAll(base + *(this + 40)) (cf. 0x39f970..0x39f976).
    let base = source.map(|s| s - 36).unwrap_or(0);
    disconnect(base + signal_offset)
}


// 0x3af9a8 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3af9a8<'a>(member: usize, names: &'a [&'a str], permissions: u32, attributes: u32) -> EventDescriptorInit<'a> {
    // IDA 0x3af9a8: Described classDescriptor + EventDescriptor init, vtable install and signature build (cf. 0x39f9bc..0x39fa02); the reflection tables stay engine-side.
    EventDescriptorInit { member, names, permissions, attributes }
}


// 0x3afb2c — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_ED1Ev")]
pub fn stub_3afb2c(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3afb2c: resets the vtable then SignatureDescriptor list clear (cf. 0x39fb20..0x39fb24).
    clear_signatures(this);
}


// 0x3afb50 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_ED0Ev")]
pub fn stub_3afb50(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3afb50: D1 body then operator delete (cf. 0x39fb6a..0x39fb96).
    clear_signatures(this);
    free(this);
}


// 0x3b0324 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEED2Ev")]
pub fn stub_3b0324(disconnect: &mut dyn FnMut(), release_slots: &mut dyn FnMut()) {
    // IDA 0x3b0324: disconnectAll on both inner signals then intrusive_ptr_release of the slots (cf. 0x3a3216..0x3a323c).
    // was: rbx::remote_signal<...>::~remote_signal.
    disconnect();
    release_slots();
}


// 0x3b0470 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEED2Ev")]
pub fn stub_3b0470(disconnect: &mut dyn FnMut(), release_slots: &mut dyn FnMut()) {
    // IDA 0x3b0470: disconnectAll on both inner signals then intrusive_ptr_release of the slots (cf. 0x3a3216..0x3a323c).
    // was: rbx::remote_signal<...>::~remote_signal.
    disconnect();
    release_slots();
}


// 0x3b5318 — __ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_3b5318(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3b5318: resets the vtable then SignatureDescriptor list clear (cf. 0x39c13c..0x39c140).
    clear_signatures(this);
}


// 0x3b611c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss
// type: void __fastcall(int, int, std::string *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss")]
pub fn stub_3b611c(signal_offset: usize, source: usize, value: &str, emit: &mut dyn FnMut(usize, String)) {
    // IDA 0x3b611c: copies the string then signal_with_args<1>::operator()(source + *(this + 40)) (cf. 0x39c744..0x39c780).
    emit(source + signal_offset, value.to_owned());
}


// 0x3b97b4 — __ZN3rbx13remote_signalIFvSsEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvSsEEC2Ev")]
pub fn stub_3b97b4(init_signal: &mut dyn FnMut()) {
    // IDA 0x3b97b4: zero-inits the signal word then call_once's the signal mutex init (cf. 0x39ecc4..0x39ed34); the boost mutex stays engine-side.
    // was: rbx::remote_signal<...>::remote_signal.
    init_signal();
}


// 0x3b9b0c — __ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_3b9b0c(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3b9b0c: D1 body then operator delete (cf. 0x39f5d2..0x39f5fe).
    clear_signatures(this);
    free(this);
}


// 0x3b9bc0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_3b9bc0(slot: SharedPtr<()>, connect: &mut dyn FnMut(SharedPtr<()>)) {
    // IDA 0x3b9bc0: retains the shared_ptr slot, binds GenericSlotWrapper::executeN and connects it to the signal (cf. 0x39f674..0x39f6c0); the boost bind/functor stays engine-side.
    // was: boost::shared_ptr<RBX::Reflection::GenericSlotWrapper> retained + bound.
    connect(slot);
}


// 0x3b9d24 — __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_3b9d24(scriptable_flags: u32) -> bool {
    // IDA 0x3b9d24: *(_DWORD *)(this + 48) & 1 (cf. 0x39f7b2).
    scriptable_flags & 1 == 1
}


// 0x3b9d2c — __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_3b9d2c(broadcast_flags: u32) -> bool {
    // IDA 0x3b9d2c: *(_DWORD *)(this + 44) & 1 (cf. 0x39f7ba).
    broadcast_flags & 1 == 1
}


// 0x3b9d34 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_3b9d34(signal_offset: usize, source: Option<usize>, args: &[String], emit: &mut dyn FnMut(usize, &str)) {
    // IDA 0x3b9d34: asserts args.size() == 1 (Event.h:320, cf. 0x39f7f8..0x39f852); base = source ? source - 36 : 0; any_cast<string>; signal_with_args<1>::operator() (cf. 0x39f86c..0x39f89e).
    assert!(args.len() == 1, "args.size() == 1 Event.h:320");
    let base = source.map(|s| s - 36).unwrap_or(0);
    emit(base + signal_offset, &args[0]);
}


// 0x3b9ed8 — __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_3b9ed8(dispatch: &mut dyn FnMut(usize, usize, usize, u32) -> i32, source: usize, descriptor: usize, args: usize) -> i32 {
    // IDA 0x3b9ed8: (*(source.vtbl + 12))(source, descriptor, args, 0) (cf. 0x39f960).
    dispatch(source, descriptor, args, 0)
}


// 0x3b9ee8 — __ZNK3RBX10Reflection13EventDescBaseINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3b9ee8(signal_offset: usize, source: Option<usize>, disconnect: &mut dyn FnMut(usize) -> i32) -> i32 {
    // IDA 0x3b9ee8: base = source ? source - 36 : 0 then signal::disconnectAll(base + *(this + 40)) (cf. 0x39f970..0x39f976).
    let base = source.map(|s| s - 36).unwrap_or(0);
    disconnect(base + signal_offset)
}


// 0x3b9efc — __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3b9efc<'a>(member: usize, names: &'a [&'a str], permissions: u32, attributes: u32) -> EventDescriptorInit<'a> {
    // IDA 0x3b9efc: Described classDescriptor + EventDescriptor init, vtable install and signature build (cf. 0x39f9bc..0x39fa02); the reflection tables stay engine-side.
    EventDescriptorInit { member, names, permissions, attributes }
}


// 0x3ba080 — __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_3ba080(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3ba080: resets the vtable then SignatureDescriptor list clear (cf. 0x39fb20..0x39fb24).
    clear_signatures(this);
}


// 0x3ba0a4 — __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_3ba0a4(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3ba0a4: D1 body then operator delete (cf. 0x39fb6a..0x39fb96).
    clear_signatures(this);
    free(this);
}


// 0x3bb204 — __ZN3rbx13remote_signalIFvSsEED2Ev
// type: _DWORD *__fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvSsEED2Ev")]
pub fn stub_3bb204(disconnect: &mut dyn FnMut(), release_slots: &mut dyn FnMut()) {
    // IDA 0x3bb204: disconnectAll on both inner signals then intrusive_ptr_release of the slots (cf. 0x3a3216..0x3a323c).
    // was: rbx::remote_signal<...>::~remote_signal.
    disconnect();
    release_slots();
}


// 0x3ec400 — __ZN3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEED1Ev")]
pub fn stub_3ec400(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3ec400: resets the vtable then SignatureDescriptor list clear (cf. 0x39c13c..0x39c140).
    clear_signatures(this);
}


// 0x3ed188 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPS2_S6_SsS7_
// type: void __fastcall(int, int, const shared_count *, const std::string *, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPS2_S6_SsS7_")]
pub fn stub_3ed188(signal_offset: usize, source: usize, args: ChatEventArgs, emit: &mut dyn FnMut(usize, ChatEventArgs)) {
    // IDA 0x3ed188: retains the speaker shared_ptr, copies the string, then signal_with_args<3>::operator()(source + *(this + 40)) (cf. 0x3ed188..0x3ed21a); releases afterwards (cf. 0xed224..0xed22c shape).
    // was: boost::shared_ptr<RBX::Instance> retained across the emit.
    emit(source + signal_offset, args);
}


// 0x3ed9c8 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS3_11ChatService9ChatColorEEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS3_11ChatService9ChatColorEEEC2Ev")]
pub fn stub_3ed9c8(init_signal: &mut dyn FnMut()) {
    // IDA 0x3ed9c8: zero-inits the signal word then call_once's the signal mutex init (cf. 0x39ecc4..0x39ed34); the boost mutex stays engine-side.
    // was: rbx::remote_signal<...>::remote_signal.
    init_signal();
}


// 0x3ee4d0 — __ZN3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEED0Ev")]
pub fn stub_3ee4d0(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3ee4d0: D1 body then operator delete (cf. 0x39f5d2..0x39f5fe).
    clear_signatures(this);
    free(this);
}


// 0x3ee584 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
pub fn stub_3ee584(slot: SharedPtr<()>, connect: &mut dyn FnMut(SharedPtr<()>)) {
    // IDA 0x3ee584: retains the shared_ptr slot, binds GenericSlotWrapper::executeN and connects it to the signal (cf. 0x39f674..0x39f6c0); the boost bind/functor stays engine-side.
    // was: boost::shared_ptr<RBX::Reflection::GenericSlotWrapper> retained + bound.
    connect(slot);
}


// 0x3ee6e8 — __ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE12isScriptableEv")]
pub fn stub_3ee6e8(scriptable_flags: u32) -> bool {
    // IDA 0x3ee6e8: *(_DWORD *)(this + 48) & 1 (cf. 0x39f7b2).
    scriptable_flags & 1 == 1
}


// 0x3ee6f0 — __ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE11isBroadcastEv")]
pub fn stub_3ee6f0(broadcast_flags: u32) -> bool {
    // IDA 0x3ee6f0: *(_DWORD *)(this + 44) & 1 (cf. 0x39f7ba).
    broadcast_flags & 1 == 1
}


// 0x3ee6f8 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE")]
pub fn stub_3ee6f8(signal_offset: usize, source: Option<usize>, args: ChatEventArgs, emit: &mut dyn FnMut(usize, ChatEventArgs)) {
    // IDA 0x3ee6f8: asserts args.size() == 3 (Event.h:322); base = source ? source - 36 : 0; any_cast<shared_ptr<Instance>,string,ChatColor>; signal_with_args<3>::operator().
    let base = source.map(|s| s - 36).unwrap_or(0);
    emit(base + signal_offset, args);
}


// 0x3ee910 — __ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
pub fn stub_3ee910(dispatch: &mut dyn FnMut(usize, usize, usize, u32) -> i32, source: usize, descriptor: usize, args: usize) -> i32 {
    // IDA 0x3ee910: (*(source.vtbl + 12))(source, descriptor, args, 0) (cf. 0x39f960).
    dispatch(source, descriptor, args, 0)
}


// 0x3ee920 — __ZNK3RBX10Reflection13EventDescBaseINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3ee920(signal_offset: usize, source: Option<usize>, disconnect: &mut dyn FnMut(usize) -> i32) -> i32 {
    // IDA 0x3ee920: base = source ? source - 36 : 0 then signal::disconnectAll(base + *(this + 40)) (cf. 0x39f970..0x39f976).
    let base = source.map(|s| s - 36).unwrap_or(0);
    disconnect(base + signal_offset)
}


// 0x3efce8 — __ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3efce8<'a>(member: usize, names: &'a [&'a str], permissions: u32, attributes: u32) -> EventDescriptorInit<'a> {
    // IDA 0x3efce8: Described classDescriptor + EventDescriptor init, vtable install and signature build (cf. 0x39f9bc..0x39fa02); the reflection tables stay engine-side.
    EventDescriptorInit { member, names, permissions, attributes }
}


// 0x3eff44 — __ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_ED1Ev")]
pub fn stub_3eff44(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3eff44: resets the vtable then SignatureDescriptor list clear (cf. 0x39fb20..0x39fb24).
    clear_signatures(this);
}


// 0x3eff68 — __ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_ED0Ev")]
pub fn stub_3eff68(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3eff68: D1 body then operator delete (cf. 0x39fb6a..0x39fb96).
    clear_signatures(this);
    free(this);
}


// 0x3f0948 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS3_11ChatService9ChatColorEEED2Ev
// type: _DWORD *__fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS3_11ChatService9ChatColorEEED2Ev")]
pub fn stub_3f0948(disconnect: &mut dyn FnMut(), release_slots: &mut dyn FnMut()) {
    // IDA 0x3f0948: disconnectAll on both inner signals then intrusive_ptr_release of the slots (cf. 0x3a3216..0x3a323c).
    // was: rbx::remote_signal<...>::~remote_signal.
    disconnect();
    release_slots();
}


// 0x3f17f8 — __ZN3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED1Ev")]
pub fn stub_3f17f8(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3f17f8: resets the vtable then SignatureDescriptor list clear (cf. 0x39c13c..0x39c140).
    clear_signatures(this);
}


// 0x3f1d14 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_
// type: void __fastcall(int, int, const shared_count *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_")]
pub fn stub_3f1d14(signal_offset: usize, source: usize, instance: SharedPtr<()>, emit: &mut dyn FnMut(usize, SharedPtr<()>)) {
    // IDA 0x3f1d14: retains the shared_ptr then signal_with_args<1>::operator()(source + *(this + 40)) (cf. 0x3f1d36..0x3f1d80); releases afterwards (0x3f1d86..0x3f1d8e).
    // was: boost::shared_ptr<RBX::Instance> retained across the emit.
    emit(source + signal_offset, instance);
}


// 0x3f2330 — __ZN3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED0Ev")]
pub fn stub_3f2330(this: usize, clear_signatures: &mut dyn FnMut(usize), free: &mut dyn FnMut(usize)) {
    // IDA 0x3f2330: D1 body then operator delete (cf. 0x39f5d2..0x39f5fe).
    clear_signatures(this);
    free(this);
}


// 0x3f23e4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
pub fn stub_3f23e4(slot: SharedPtr<()>, connect: &mut dyn FnMut(SharedPtr<()>)) {
    // IDA 0x3f23e4: retains the shared_ptr slot, binds GenericSlotWrapper::executeN and connects it to the signal (cf. 0x39f674..0x39f6c0); the boost bind/functor stays engine-side.
    // was: boost::shared_ptr<RBX::Reflection::GenericSlotWrapper> retained + bound.
    connect(slot);
}


// 0x3f2548 — __ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE12isScriptableEv")]
pub fn stub_3f2548(scriptable_flags: u32) -> bool {
    // IDA 0x3f2548: *(_DWORD *)(this + 48) & 1 (cf. 0x39f7b2).
    scriptable_flags & 1 == 1
}


// 0x3f2550 — __ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE11isBroadcastEv")]
pub fn stub_3f2550(broadcast_flags: u32) -> bool {
    // IDA 0x3f2550: *(_DWORD *)(this + 44) & 1 (cf. 0x39f7ba).
    broadcast_flags & 1 == 1
}


// 0x3f2558 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
pub fn stub_3f2558(signal_offset: usize, source: Option<usize>, instance: SharedPtr<()>, emit: &mut dyn FnMut(usize, SharedPtr<()>)) {
    // IDA 0x3f2558: asserts args.size() == 1 (Event.h:320); base = source ? source - 36 : 0; any_cast<shared_ptr<Instance>; signal_with_args<1>::operator().
    let base = source.map(|s| s - 36).unwrap_or(0);
    emit(base + signal_offset, instance);
}


// 0x3f26b8 — __ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE")]
pub fn stub_3f26b8(dispatch: &mut dyn FnMut(usize, usize, usize, u32) -> i32, source: usize, descriptor: usize, args: usize) -> i32 {
    // IDA 0x3f26b8: (*(source.vtbl + 12))(source, descriptor, args, 0) (cf. 0x39f960).
    dispatch(source, descriptor, args, 0)
}


// 0x3f26c8 — __ZNK3RBX10Reflection13EventDescBaseINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_3f26c8(signal_offset: usize, source: Option<usize>, disconnect: &mut dyn FnMut(usize) -> i32) -> i32 {
    // IDA 0x3f26c8: base = source ? source - 36 : 0 then signal::disconnectAll(base + *(this + 40)) (cf. 0x39f970..0x39f976).
    let base = source.map(|s| s - 36).unwrap_or(0);
    disconnect(base + signal_offset)
}


// 0x3f26dc — __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_3f26dc<'a>(member: usize, names: &'a [&'a str], permissions: u32, attributes: u32) -> EventDescriptorInit<'a> {
    // IDA 0x3f26dc: Described classDescriptor + EventDescriptor init, vtable install and signature build (cf. 0x39f9bc..0x39fa02); the reflection tables stay engine-side.
    EventDescriptorInit { member, names, permissions, attributes }
}


// 0x3f2860 — __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev")]
pub fn stub_3f2860(this: usize, clear_signatures: &mut dyn FnMut(usize)) {
    // IDA 0x3f2860: resets the vtable then SignatureDescriptor list clear (cf. 0x39fb20..0x39fb24).
    clear_signatures(this);
}

