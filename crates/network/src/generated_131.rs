//! Auto-generated skeletons for rbx-network — RakNet|RBX::Network|Replicator filtered EA-sorted asc
//! Filter: RakNet|RBX::Network|Replicator (case-insensitive) -> 4797 funcs, 2658 already stubbed (2139 remaining before batch)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x9be164..0x9cbe4c | existing 13690 -> 13790 total (filtered EA-sorted asc, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
// 0x9be164 — __ZN3RBX7Network15PhysicsReceiver12readVelocityERN6RakNet9BitStreamERNS_8VelocityE
// type: void __fastcall(RBX::Network::PhysicsReceiver *this, RakNet::BitStream *, RBX::Velocity *)
#[doc(alias = "RBX::Network::PhysicsReceiver::readVelocity(RakNet::BitStream &,RBX::Velocity &)")]
pub fn stub_9be164() -> ! {
    todo!("0x9be164 __ZN3RBX7Network15PhysicsReceiver12readVelocityERN6RakNet9BitStreamERNS_8VelocityE")
}

// 0x9be2ec — __ZN3RBX7Network15PhysicsReceiver17readCompactCFrameERN6RakNet9BitStreamERNS_13CompactCFrameE
// type: int __fastcall(RBX::Network::PhysicsReceiver *this, RakNet::BitStream *, RBX::CompactCFrame *)
#[doc(alias = "RBX::Network::PhysicsReceiver::readCompactCFrame(RakNet::BitStream &,RBX::CompactCFrame &)")]
pub fn stub_9be2ec() -> ! {
    todo!("0x9be2ec __ZN3RBX7Network15PhysicsReceiver17readCompactCFrameERN6RakNet9BitStreamERNS_13CompactCFrameE")
}

// 0x9be624 — __ZN3RBX7Network15PhysicsReceiver10setPhysicsERKNS_13MechanismItemERKNS_10RemoteTimeEj
// type: void __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::PhysicsReceiver::setPhysics(RBX::MechanismItem const&,RBX::RemoteTime const&,unsigned int)")]
pub fn stub_9be624() -> ! {
    todo!("0x9be624 __ZN3RBX7Network15PhysicsReceiver10setPhysicsERKNS_13MechanismItemERKNS_10RemoteTimeEj")
}

// 0x9bebb4 — __ZN3RBX7Network15PhysicsReceiver24okDistributedReceivePartERKN5boost10shared_ptrINS_12PartInstanceEEE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "RBX::Network::PhysicsReceiver::okDistributedReceivePart(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
pub fn stub_9bebb4() -> ! {
    todo!("0x9bebb4 __ZN3RBX7Network15PhysicsReceiver24okDistributedReceivePartERKN5boost10shared_ptrINS_12PartInstanceEEE")
}

// 0x9bebd8 — __ZN3RBX7Network15PhysicsReceiver15receiveRootPartERN5boost10shared_ptrINS_12PartInstanceEEERN6RakNet9BitStreamE
// type: int __fastcall(int, struct _Unwind_Exception *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::PhysicsReceiver::receiveRootPart(rbx_core::SharedPtr<RBX::PartInstance> &,RakNet::BitStream &)")]
pub fn stub_9bebd8() -> ! {
    todo!("0x9bebd8 __ZN3RBX7Network15PhysicsReceiver15receiveRootPartERN5boost10shared_ptrINS_12PartInstanceEEERN6RakNet9BitStreamE")
}

// 0x9bedec — __ZN3RBX7Network16CustomSerializer10readVectorERfS2_S2_RN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::CustomSerializer *this, float *, float *, float *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::CustomSerializer::readVector(float &,float &,float &,RakNet::BitStream &)")]
pub fn stub_9bedec(stream: &mut crate::bitstream::BitStream, out: &mut [f32; 3]) -> bool {
    // IDA 0x9bedec: `CustomSerializer::readVector` — compression-gated component read into `out`.
    crate::custom_serializer::read_vector(stream, out)
}

// 0x9bfa90 — __ZN3RBX7Network13PhysicsSender11sendTouchesE14PacketPriority
// type: void __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, char, int, char, int, int, int, int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Network::PhysicsSender::sendTouches(PacketPriority)")]
pub fn stub_9bfa90() -> ! {
    todo!("0x9bfa90 __ZN3RBX7Network13PhysicsSender11sendTouchesE14PacketPriority")
}

// 0x9c0908 — __ZN3RBX7Network13PhysicsSenderC2ERNS0_10ReplicatorE
// type: RBX::Network::PhysicsSender *__fastcall(RBX::Network::PhysicsSender *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::PhysicsSender::PhysicsSender(RBX::Network::Replicator &)")]
pub fn stub_9c0908() -> crate::physics::PhysicsSender {
    // IDA 0x9c0908: unordered-map buckets + G3D motor-angle array init.
    crate::physics::PhysicsSender::new()
}

// 0x9c0a9c — __ZN3RBX7Network13PhysicsSender11onTouchStepERKNS_9TouchPairE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Network::PhysicsSender::onTouchStep(RBX::TouchPair const&)")]
pub fn stub_9c0a9c(sender: &mut crate::physics::PhysicsSender, pair: crate::physics::TouchPair) -> bool {
    // IDA 0x9c0ab4: emplace into the unordered set at +20.
    sender.on_touch_step(pair)
}

// 0x9c0ab8 — __ZN3RBX7Network13PhysicsSender14connectTouchesEv
// type: void __fastcall(RBX::Network::PhysicsSender *this)
#[doc(alias = "RBX::Network::PhysicsSender::connectTouches(void)")]
pub fn stub_9c0ab8(sender: &mut crate::physics::PhysicsSender) {
    // IDA 0x9c0bd4..0x9c0c5e: insert the `onTouchStep` slot into the workspace touch signal.
    sender.connect_touches();
}

// 0x9c0dd4 — __ZN3RBX7Network13PhysicsSender5startEN5boost10shared_ptrIS1_EE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, void *, int, char, char, int, int, int, int)
#[doc(alias = "RBX::Network::PhysicsSender::start(rbx_core::SharedPtr<RBX::Network::PhysicsSender>)")]
pub fn stub_9c0dd4(sender: &mut crate::physics::PhysicsSender) {
    // IDA 0x9c0dd4: connectTouches + Job/TouchJob scheduler submission.
    sender.start();
}

// 0x9c1ea4 — __ZN3RBX7Network13PhysicsSenderD0Ev
// type: void __fastcall(RBX::Network::PhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::~PhysicsSender()")]
pub fn stub_9c1ea4(sender: crate::physics::PhysicsSender) {
    // IDA 0x9c1ef4..0x9c1efa: D2 then `operator delete`.
    drop(sender);
}

// 0x9c1f44 — __ZN3RBX7Network13PhysicsSenderD1Ev
// type: void __fastcall(RBX::Network::PhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::~PhysicsSender()")]
pub fn stub_9c1f44(sender: &mut crate::physics::PhysicsSender) {
    // IDA 0x9c1f48: tail-calls D2.
    sender.tear_down();
}

// 0x9c1f50 — __ZN3RBX7Network13PhysicsSenderD2Ev
// type: void __fastcall(RBX::Network::PhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::~PhysicsSender()")]
pub fn stub_9c1f50(sender: &mut crate::physics::PhysicsSender) {
    // IDA 0x9c2016..0x9c2248: scheduler removes, job resets, signal disconnect, touch-set teardown.
    sender.tear_down();
}

// 0x9c2504 — __ZN3RBX7Network13PhysicsSender33sendChildPrimitiveCoordinateFrameEPNS_9PrimitiveEPN6RakNet9BitStreamEPNS0_10ReplicatorE
// type: unsigned int __fastcall(RBX::Network::PhysicsSender *this, RBX::Primitive *, RakNet::BitStream *, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::PhysicsSender::sendChildPrimitiveCoordinateFrame(RBX::Primitive *,RakNet::BitStream *,RBX::Network::Replicator *)")]
pub fn stub_9c2504(
    sender: &crate::physics::PhysicsSender,
    stream: &mut crate::bitstream::BitStream,
    translation: [f32; 3],
    rotation: [f32; 4],
    streaming_enabled: bool,
    part_present: bool,
    replication_container: bool,
    try_serialize_id: &mut dyn FnMut(&mut crate::bitstream::BitStream) -> bool,
) -> bool {
    // IDA 0x9c2520..0x9c2562: ReleaseAssert(isStreamingEnabled) (:254).
    debug_assert!(streaming_enabled, "replicator->isStreamingEnabled() Client/Network/PhysicsSender.cpp line: 254");
    // IDA 0x9c2566..0x9c256e: fromPrimitive null check.
    if !part_present {
        return false;
    }
    // IDA 0x9c2574..0x9c257a: isReplicationContainer.
    if !replication_container {
        return false;
    }
    // IDA 0x9c2582..0x9c2588: trySerializeId gates the body.
    if !try_serialize_id(stream) {
        return false;
    }
    // IDA 0x9c2594..0x9c25a8: writeTranslation + tail writeRotation.
    crate::physics::write_translation(stream, translation, sender.translation_compression);
    crate::physics::write_rotation(stream, rotation, sender.translation_compression);
    true
}

// 0x9c25b8 — __ZN3RBX7Network13PhysicsSender20sendMechanismCFramesERN6RakNet9BitStreamEPKNS_12PartInstanceEb
// type: int __fastcall(RBX::Network::PhysicsSender *this, RakNet::BitStream *, const RBX::PartInstance *, int)
#[doc(alias = "RBX::Network::PhysicsSender::sendMechanismCFrames(RakNet::BitStream &,RBX::PartInstance const*,bool)")]
#[allow(clippy::too_many_arguments)]
pub fn stub_9c25b8(
    stream: &mut crate::bitstream::BitStream,
    streaming_enabled: bool,
    mechanism_present: bool,
    flag_set: bool,
    complex_moving: bool,
    moving: bool,
    replication_container: bool,
    try_serialize_id: &mut dyn FnMut(&mut crate::bitstream::BitStream) -> bool,
    translation: [f32; 3],
    rotation: [f32; 4],
    visit_children: &mut dyn FnMut(),
    serialize_null_id: &mut dyn FnMut(&mut crate::bitstream::BitStream) -> bool,
) -> bool {
    // IDA 0x9c25b8: mode select + gated CF write + trailing null-id.
    let mode = crate::physics::select_mechanism_mode(flag_set, complex_moving, moving);
    crate::physics::send_mechanism_cframes(stream, streaming_enabled, mechanism_present, mode, replication_container, try_serialize_id, translation, rotation, visit_children, serialize_null_id)
}

// 0x9c2758 — __ZN3RBX7Network13PhysicsSender13sendMechanismERN6RakNet9BitStreamEPKNS_12PartInstanceEb
// type: int __fastcall(RBX::Network::PhysicsSender *this, RakNet::BitStream *, RBX::Primitive **, int)
#[doc(alias = "RBX::Network::PhysicsSender::sendMechanism(RakNet::BitStream &,RBX::PartInstance const*,bool)")]
pub fn stub_9c2758(
    stream: &mut crate::bitstream::BitStream,
    assembly_present: bool,
    flag_set: bool,
    complex_moving: bool,
    moving: bool,
    motor_count: u8,
    write_root: &mut dyn FnMut(&mut crate::bitstream::BitStream),
    child_count: usize,
    visit_child: &mut dyn FnMut(usize, &mut crate::bitstream::BitStream),
) {
    // IDA 0x9c27da..0x9c27e6: the mode select lands in +16 for the virtual root write.
    let _mode = crate::physics::select_mechanism_mode(flag_set, complex_moving, moving);
    // IDA 0x9c2758: motor framing + virtual root write + child visits + trailing true.
    crate::physics::send_mechanism(stream, assembly_present, motor_count, write_root, child_count, visit_child)
}

// 0x9c28a8 — __ZN3RBX7Network13PhysicsSender17sendChildAssemblyEPN6RakNet9BitStreamEPKNS_8AssemblyE
// type: int __fastcall(RBX::Network::IdSerializer **this, RakNet::BitStream *, const RBX::Assembly *)
#[doc(alias = "RBX::Network::PhysicsSender::sendChildAssembly(RakNet::BitStream *,RBX::Assembly const*)")]
pub fn stub_9c28a8(
    part_present: bool,
    use_try_serialize_id: bool,
    can_serialize_id: bool,
    stream: &mut crate::bitstream::BitStream,
    serialize_id: &mut dyn FnMut(&mut crate::bitstream::BitStream),
    sender: &mut crate::physics::PhysicsSender,
    packet: &crate::physics::AssemblyPacket<'_>,
) -> bool {
    // IDA 0x9c28bc..0x9c290c: ReleaseAssert(part) (:328).
    debug_assert!(part_present, "part Client/Network/PhysicsSender.cpp line: 328");
    // IDA 0x9c2928..0x9c294c: try-serialize-id gate.
    if use_try_serialize_id && !can_serialize_id {
        return false;
    }
    // IDA 0x9c292e: presence bit for the child assembly.
    stream.write_bit(false);
    // IDA 0x9c2938: IdSerializer::serializeId.
    serialize_id(stream);
    // IDA 0x9c2946: virtual body write (ErrorComp override selected engine-side).
    sender.write_assembly(stream, packet);
    true
}

// 0x9c2950 — __ZN3RBX7Network13PhysicsSender13writeAssemblyERN6RakNet9BitStreamEPKNS_8AssemblyE
// type: unsigned int __fastcall(RBX::Network::PhysicsSender *this, RakNet::BitStream *, const RBX::Assembly *)
#[doc(alias = "RBX::Network::PhysicsSender::writeAssembly(RakNet::BitStream &,RBX::Assembly const*)")]
pub fn stub_9c2950(
    sender: &mut crate::physics::PhysicsSender,
    stream: &mut crate::bitstream::BitStream,
    packet: &crate::physics::AssemblyPacket<'_>,
) {
    // IDA 0x9c2962..0x9c29bc: assembly primitive/PV/+124 nibble arrive as `packet`.
    sender.write_assembly(stream, packet);
}

// 0x9c29c0 — __ZN3RBX7Network13PhysicsSender16writeMotorAnglesERN6RakNet9BitStreamEPKNS_8AssemblyE
// type: _DWORD __fastcall(RBX::Network::PhysicsSender *__hidden this, RakNet::BitStream *, const RBX::Assembly *)
#[doc(alias = "RBX::Network::PhysicsSender::writeMotorAngles(RakNet::BitStream &,RBX::Assembly const*)")]
pub fn stub_9c29c0(sender: &mut crate::physics::PhysicsSender, stream: &mut crate::bitstream::BitStream, physics: &[crate::physics::CompactCFrame]) {
    // IDA 0x9c2a1e: `Assembly::getPhysics` scratch slice arrives as `physics`.
    sender.write_motor_angles(stream, physics);
}

// 0x9c2aa4 — __ZN3RBX7Network13PhysicsSender13writeVelocityERN6RakNet9BitStreamERKNS_8VelocityE
// type: _DWORD __fastcall(RBX::Network::PhysicsSender *__hidden this, RakNet::BitStream *, const RBX::Velocity *)
#[doc(alias = "RBX::Network::PhysicsSender::writeVelocity(RakNet::BitStream &,RBX::Velocity const&)")]
pub fn stub_9c2aa4(sender: &crate::physics::PhysicsSender, stream: &mut crate::bitstream::BitStream, velocity: &crate::physics::Velocity) {
    sender.write_velocity(stream, velocity);
}

// 0x9c2b10 — __ZN3RBX7Network13PhysicsSender18writeCompactCFrameERN6RakNet9BitStreamERKNS_13CompactCFrameE
// type: unsigned int __fastcall(double this, const RBX::CompactCFrame *)
#[doc(alias = "RBX::Network::PhysicsSender::writeCompactCFrame(RakNet::BitStream &,RBX::CompactCFrame const&)")]
pub fn stub_9c2b10(sender: &crate::physics::PhysicsSender, stream: &mut crate::bitstream::BitStream, frame: &crate::physics::CompactCFrame) {
    sender.write_compact_cframe(stream, frame);
}

// 0x9c2d18 — __ZN3RBX7Network13PhysicsSender7canSendEPKNS_12PartInstanceEPKNS_8AssemblyERN6RakNet9BitStreamE
// type: bool __fastcall(RBX::Network::PhysicsSender *this, const RBX::PartInstance *, const RBX::Assembly *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::PhysicsSender::canSend(RBX::PartInstance const*,RBX::Assembly const*,RakNet::BitStream &)")]
pub fn stub_9c2d18(
    assembly_present: bool,
    part_present: bool,
    primitives_match: bool,
    stream_gate: Option<bool>,
    streaming_enabled: bool,
    serialize_pending: bool,
) -> bool {
    // IDA 0x9c2d18: membership assert, null/gate refusals, pending check.
    crate::physics::can_send(assembly_present, part_present, primitives_match, stream_gate, streaming_enabled, serialize_pending)
}

// 0x9c2dd4 — __ZN3RBX7Network13PhysicsSender15sendPhysicsDataERN6RakNet9BitStreamEPKNS_12PartInstanceEb
// type: int __fastcall(RBX::Network::PhysicsSender *this, RakNet::BitStream *, RBX::Primitive **, int)
#[doc(alias = "RBX::Network::PhysicsSender::sendPhysicsData(RakNet::BitStream &,RBX::PartInstance const*,bool)")]
#[allow(clippy::too_many_arguments)]
pub fn stub_9c2dd4(
    stream: &mut crate::bitstream::BitStream,
    part_present: bool,
    assembly_root: bool,
    sendable: bool,
    streaming_enabled: bool,
    in_streamed_regions: bool,
    try_serialize_id: &mut dyn FnMut(&mut crate::bitstream::BitStream) -> bool,
    serialize_null_id: &mut dyn FnMut(&mut crate::bitstream::BitStream),
    serialize_id: &mut dyn FnMut(&mut crate::bitstream::BitStream),
    use_try_serialize_id: bool,
    send_cframes: &mut dyn FnMut(&mut crate::bitstream::BitStream),
    send_mechanism_body: &mut dyn FnMut(&mut crate::bitstream::BitStream),
) -> bool {
    // IDA 0x9c2dd4: root/canSend gates, streaming vs direct branches.
    crate::physics::send_physics_data(stream, part_present, assembly_root, sendable, streaming_enabled, in_streamed_regions, try_serialize_id, serialize_null_id, serialize_id, use_try_serialize_id, send_cframes, send_mechanism_body)
}

// 0x9c2f6c — __ZN5boost10shared_ptrIN3RBX7Network13PhysicsSender3JobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::PhysicsSender::Job>::reset(void)")]
pub fn stub_9c2f6c(sender: &mut crate::physics::PhysicsSender) {
    // IDA 0x9c2f6c: release the Job slot.
    sender.reset_job_slot();
}

// 0x9c300c — __ZN5boost10shared_ptrIN3RBX7Network13PhysicsSender8TouchJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::PhysicsSender::TouchJob>::reset(void)")]
pub fn stub_9c300c(sender: &mut crate::physics::PhysicsSender) {
    // IDA 0x9c300c: release the TouchJob slot.
    sender.reset_job_slot();
}

// 0x9c30ac — __ZN3RBX7Network16CustomSerializer11writeVectorEbRKfS3_S3_RN6RakNet9BitStreamE
// type: unsigned int __fastcall(RBX::Network::CustomSerializer *this, __int32 *, float *, float *, RakNet::BitStream *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::CustomSerializer::writeVector(bool,float const&,float const&,float const&,RakNet::BitStream &)")]
pub fn stub_9c30ac(
    heavy: bool,
    x: f32,
    y: f32,
    z: f32,
    stream: &mut crate::bitstream::BitStream,
) {
    // IDA 0x9c30ac: `CustomSerializer::writeVector` — heavy/light component packing.
    crate::custom_serializer::write_vector(heavy, x, y, z, stream);
}

// 0x9c3488 — __ZN6RakNet9BitStream5WriteIfEEvRKT_
// type: void __fastcall(RakNet::BitStream *, unsigned __int8 *, int, unsigned int, __guard *, int, int, int, int)
#[doc(alias = "void RakNet::BitStream::Write<float>(float const&)")]
pub fn stub_9c3488(stream: &mut crate::bitstream::BitStream, value: f32) {
    // IDA 0x9c3488: `Write<float>` template — `ReverseBytes` + `WriteBits(..., 32, 1)`, big-endian.
    stream.write_f32(value);
}

// 0x9c35a0 — __ZN3RBX11IndexedTree23visitConstMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvNS3_4_mfi3mf2IvNS_7Network13PhysicsSenderEPN6RakNet9BitStreamEPKS2_EENS4_5list3INS4_5valueIPS9_EENSH_ISC_EENS3_3argILi1EEEEEEEEEvT0_
// type: int __fastcall(int, void (*)(void), int, int, int)
#[doc(alias = "void RBX::IndexedTree::visitConstMeAndChildren<RBX::Assembly,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::PhysicsSender,RakNet::BitStream *,RBX::Assembly const*>,boost::_bi::list3<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::_bi::value<RakNet::BitStream *>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::PhysicsSender,RakNet::BitStream *,RBX::Assembly const*>,boost::_bi::list3<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::_bi::value<RakNet::BitStream *>,boost::arg<1>>>)")]
pub fn stub_9c35a0(
    tree: &crate::physics::AssemblyTree,
    visit: &mut dyn FnMut(&crate::physics::AssemblyTree),
) {
    // IDA 0x9c35c6..0x9c3662: self bind then child assemblies; `indexOf` asserts live in the model.
    tree.visit_const_me_and_children(visit);
}

// 0x9c3664 — __ZN3RBX9Mechanism19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf3IvNS_7Network13PhysicsSenderEPNS_9PrimitiveEPN6RakNet9BitStreamEPNS7_10ReplicatorEEENS3_5list4INS3_5valueIPS8_EENS2_3argILi1EEENSI_ISD_EENSI_ISF_EEEEEEEEvT_PNS_8AssemblyE
// type: int __fastcall(int, int, int, int, int, int, RBX::Assembly *)
#[doc(alias = "void RBX::Mechanism::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::PhysicsSender,RBX::Primitive *,RakNet::BitStream *,RBX::Network::Replicator *>,boost::_bi::list4<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>,boost::_bi::value<RakNet::BitStream *>,boost::_bi::value<RBX::Network::Replicator *>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::PhysicsSender,RBX::Primitive *,RakNet::BitStream *,RBX::Network::Replicator *>,boost::_bi::list4<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>,boost::_bi::value<RakNet::BitStream *>,boost::_bi::value<RBX::Network::Replicator *>>>,RBX::Assembly *)")]
pub fn stub_9c3664(tree: &crate::physics::MechanismTree, visit: &mut dyn FnMut(u32)) {
    // IDA 0x9c367c..0x9c3774: assembly-primitive assert, visit, child-mechanism recurse.
    tree.visit_primitives(visit);
}

// 0x9c3778 — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf3IvNS_7Network13PhysicsSenderEPNS_9PrimitiveEPN6RakNet9BitStreamEPNS7_10ReplicatorEEENS3_5list4INS3_5valueIPS8_EENS2_3argILi1EEENSI_ISD_EENSI_ISF_EEEEEEEEvT_SA_
// type: int __fastcall(int, void (*)(void), int, int, int, int, int)
#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::PhysicsSender,RBX::Primitive *,RakNet::BitStream *,RBX::Network::Replicator *>,boost::_bi::list4<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>,boost::_bi::value<RakNet::BitStream *>,boost::_bi::value<RBX::Network::Replicator *>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::PhysicsSender,RBX::Primitive *,RakNet::BitStream *,RBX::Network::Replicator *>,boost::_bi::list4<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>,boost::_bi::value<RakNet::BitStream *>,boost::_bi::value<RBX::Network::Replicator *>>>,RBX::Primitive *)")]
pub fn stub_9c3778(node: &crate::physics::PrimitiveNode, visit: &mut dyn FnMut(u32)) {
    // IDA 0x9c3794..0x9c3850: primitive bind then non-root children (IDA 0x9c3824).
    node.visit_primitives(visit);
}

// 0x9c3854 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSender8TouchJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender::TouchJob,RBX::Network::PhysicsSender::TouchJob>(rbx_core::SharedPtr<RBX::Network::PhysicsSender::TouchJob> *,RBX::Network::PhysicsSender::TouchJob *,boost::detail::shared_count &)")]
pub fn stub_9c3854() -> ! {
    todo!("0x9c3854 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSender8TouchJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

// 0x9c3a04 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network13PhysicsSender8TouchJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PhysicsSender::TouchJob,RBX::Network::PhysicsSender::TouchJob>(rbx_core::SharedPtr<RBX::Network::PhysicsSender::TouchJob> const*,RBX::Network::PhysicsSender::TouchJob *)const")]
pub fn stub_9c3a04() -> ! {
    todo!("0x9c3a04 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network13PhysicsSender8TouchJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0x9c3cb0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::~sp_counted_impl_p()")]
pub fn stub_9c3cb0() -> ! {
    todo!("0x9c3cb0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEED1Ev")
}

// 0x9c3cb4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::~sp_counted_impl_p()")]
pub fn stub_9c3cb4() -> ! {
    todo!("0x9c3cb4 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEED0Ev")
}

// 0x9c3cc0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::dispose(void)")]
pub fn stub_9c3cc0() -> ! {
    todo!("0x9c3cc0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEE7disposeEv")
}

// 0x9c3cd4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::get_deleter(std::type_info const&)")]
pub fn stub_9c3cd4() -> ! {
    todo!("0x9c3cd4 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEE11get_deleterERKSt9type_info")
}

// 0x9c3cd8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::TouchJob>::get_untyped_deleter(void)")]
pub fn stub_9c3cd8() -> ! {
    todo!("0x9c3cd8 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender8TouchJobEE19get_untyped_deleterEv")
}

// 0x9c3cdc — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSender3JobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender::Job,RBX::Network::PhysicsSender::Job>(rbx_core::SharedPtr<RBX::Network::PhysicsSender::Job> *,RBX::Network::PhysicsSender::Job *,boost::detail::shared_count &)")]
pub fn stub_9c3cdc() -> ! {
    todo!("0x9c3cdc __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSender3JobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

// 0x9c3e8c — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network13PhysicsSender3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PhysicsSender::Job,RBX::Network::PhysicsSender::Job>(rbx_core::SharedPtr<RBX::Network::PhysicsSender::Job> const*,RBX::Network::PhysicsSender::Job *)const")]
pub fn stub_9c3e8c() -> ! {
    todo!("0x9c3e8c __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network13PhysicsSender3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0x9c4138 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::~sp_counted_impl_p()")]
pub fn stub_9c4138() -> ! {
    todo!("0x9c4138 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEED1Ev")
}

// 0x9c413c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::~sp_counted_impl_p()")]
pub fn stub_9c413c() -> ! {
    todo!("0x9c413c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEED0Ev")
}

// 0x9c4148 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::dispose(void)")]
pub fn stub_9c4148() -> ! {
    todo!("0x9c4148 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEE7disposeEv")
}

// 0x9c415c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::get_deleter(std::type_info const&)")]
pub fn stub_9c415c() -> ! {
    todo!("0x9c415c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEE11get_deleterERKSt9type_info")
}

// 0x9c4160 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsSender::Job>::get_untyped_deleter(void)")]
pub fn stub_9c4160() -> ! {
    todo!("0x9c4160 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network13PhysicsSender3JobEE19get_untyped_deleterEv")
}

// 0x9c469c — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_7Network13PhysicsSenderES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_9c469c() -> ! {
    todo!("0x9c469c __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_7Network13PhysicsSenderES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED1Ev")
}

// 0x9c46f8 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_7Network13PhysicsSenderES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_9c46f8() -> ! {
    todo!("0x9c46f8 __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_7Network13PhysicsSenderES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED0Ev")
}

// 0x9c4980 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9TouchPairEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_7Network13PhysicsSenderES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>,1,void ()(RBX::TouchPair const&)>::call(RBX::TouchPair const&)")]
pub fn stub_9c4980() -> ! {
    todo!("0x9c4980 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9TouchPairEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_7Network13PhysicsSenderES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")
}

// 0x9c499c — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9TouchPairEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_7Network13PhysicsSenderES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsSender,RBX::TouchPair const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsSender*>,boost::arg<1>>>,1,void ()(RBX::TouchPair const&)>::call(RBX::TouchPair const&)")]
pub fn stub_9c499c() -> ! {
    todo!("0x9c499c __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9TouchPairEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_7Network13PhysicsSenderES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")
}

// 0x9c56a4 — __ZN3RBX7Network13PhysicsSender8TouchJobC2EN5boost10shared_ptrIS1_EE
// type: RBX::TaskScheduler::Job *__fastcall(RBX::TaskScheduler::Job *, _DWORD *)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::TouchJob(rbx_core::SharedPtr<RBX::Network::PhysicsSender>)")]
pub fn stub_9c56a4() -> crate::physics::TouchJob {
    // IDA 0x9c56a4 (C2): `TouchJob` from the sender shared pointer; stateless.
    crate::physics::TouchJob
}

// 0x9c5830 — __ZN3RBX7Network13PhysicsSender8TouchJobD1Ev
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::~TouchJob()")]
pub fn stub_9c5830(_job: crate::physics::TouchJob) {
    // IDA 0x9c5830..0x9c5834 (D1): tail-calls D2 (IDA 0x9c5e38); stateless drop covers it.
}

// 0x9c583c — __ZN3RBX7Network13PhysicsSender8TouchJobD0Ev
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::~TouchJob()")]
pub fn stub_9c583c(job: crate::physics::TouchJob) {
    // IDA 0x9c583c..0x9c5892 (D0): D2 then `operator delete`; by-value drop covers both.
    crate::generated_netA_watchdog_52::stub_9c5e38(job);
}

// 0x9c58dc — __ZN3RBX7Network13PhysicsSender8TouchJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_9c58dc(
    elapsed: f64,
    rate_hz: f32,
    ctx: &crate::physics::SleepContext,
) -> f64 {
    // IDA 0x9c58ea..0x9c58f2: stats rate at +496 into `computeStandardSleepTime`.
    crate::physics::TouchJob::sleep_time(elapsed, rate_hz, ctx)
}

// 0x9c58fc — __ZN3RBX7Network13PhysicsSender8TouchJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_9c58fc(
    gate: &crate::physics::SendGate,
    replicator_present: bool,
    job_pending: bool,
    error: f64,
    rate_hz: f32,
) -> crate::physics::StandardError {
    // IDA 0x9c5958..0x9c59ca: gates select the zero shape or `computeStandardError`.
    crate::physics::TouchJob::error(gate, replicator_present, job_pending, error, rate_hz)
}

// 0x9c5bfc — __ZN3RBX7Network13PhysicsSender8TouchJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::PhysicsSender::TouchJob *this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_9c5bfc(stats_present: bool, job_present: bool, step: &mut dyn FnMut()) -> bool {
    // IDA 0x9c5bfc: stats/job gates around the +122 step.
    crate::physics::TouchJob::step_data_model_job(stats_present, job_present, step)
}

// 0x9c5e38 — __ZN3RBX7Network13PhysicsSender8TouchJobD2Ev
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::~TouchJob()")]
pub fn stub_9c5e38(_job: crate::physics::TouchJob) {
    // IDA 0x9c5e70..0x9c5f24 (D2): vtable reset, weak-ref/shared-count dtors, base Job dtor.
}

// 0x9c5fdc — __ZN3RBX7Network13PhysicsSender3JobC2EN5boost10shared_ptrIS1_EE
// type: RBX::TaskScheduler::Job *__fastcall(RBX::TaskScheduler::Job *, _DWORD *)
#[doc(alias = "RBX::Network::PhysicsSender::Job::Job(rbx_core::SharedPtr<RBX::Network::PhysicsSender>)")]
pub fn stub_9c5fdc() -> crate::physics::SendJob {
    // IDA 0x9c5fdc (C2): `Job` from the sender shared pointer; stateless.
    crate::physics::SendJob
}

// 0x9c6168 — __ZN3RBX7Network13PhysicsSender3JobD1Ev
// type: void __fastcall(RBX::Network::PhysicsSender::Job *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::Job::~Job()")]
pub fn stub_9c6168(_job: crate::physics::SendJob) {
    // IDA 0x9c6168..0x9c616c (D1): tail-calls D2 (IDA 0x9c6568); stateless drop covers it.
}

// 0x9c6174 — __ZN3RBX7Network13PhysicsSender3JobD0Ev
// type: void __fastcall(RBX::Network::PhysicsSender::Job *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::Job::~Job()")]
pub fn stub_9c6174(job: crate::physics::SendJob) {
    // IDA 0x9c6174..0x9c61ca (D0): D2 then `operator delete`; by-value drop covers both.
    crate::generated_netA_watchdog_57::stub_9c6568(job);
}

// 0x9c6214 — __ZN3RBX7Network13PhysicsSender3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::PhysicsSender::Job *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::PhysicsSender::Job::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_9c6214(
    elapsed: f64,
    rate_hz: f32,
    ctx: &crate::physics::SleepContext,
) -> f64 {
    // IDA 0x9c6222..0x9c622a: stats rate at +496 into `computeStandardSleepTime`.
    crate::physics::SendJob::sleep_time(elapsed, rate_hz, ctx)
}

// 0x9c6234 — __ZN3RBX7Network13PhysicsSender3Job5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::PhysicsSender::Job *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::PhysicsSender::Job::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_9c6234(
    gate: &crate::physics::SendGate,
    error: f64,
    rate_hz: f32,
) -> crate::physics::StandardError {
    // IDA 0x9c6248..0x9c627e: `canSendPacket` selects the zero shape or `computeStandardError`.
    crate::physics::SendJob::error(gate, error, rate_hz)
}

// 0x9c6288 — __ZN3RBX7Network13PhysicsSender3Job16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::PhysicsSender::Job *this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::Network::PhysicsSender::Job::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_9c6288(stats_present: bool, job_present: bool, step: &mut dyn FnMut()) -> bool {
    // IDA 0x9c6288: stats/job gates around the stats-sample step.
    crate::physics::SendJob::step_data_model_job(stats_present, job_present, step)
}

// 0x9c6568 — __ZN3RBX7Network13PhysicsSender3JobD2Ev
// type: void __fastcall(RBX::Network::PhysicsSender::Job *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::Job::~Job()")]
pub fn stub_9c6568(_job: crate::physics::SendJob) {
    // IDA 0x9c65a0..0x9c6654 (D2): vtable reset, weak-ref/shared-count dtors, base Job dtor.
}

// 0x9c6da4 — __ZN3RBX7Network6Server5startEii
// type: int __fastcall(RBX::Network::ConcurrentRakPeer **this, unsigned __int16, int, const void *)
#[doc(alias = "RBX::Network::Server::start(int,int)")]
pub fn stub_9c6da4(
    server: &mut crate::server::Server,
    startup: Result<u16, i32>,
) -> Result<u16, String> {
    // IDA 0x9c6e64..0x9c6e90: `Startup(128, ...)` failure throws, success stores the bound port.
    server.start(startup)
}

// 0x9c7234 — __ZN3RBX7Network6Server4stopEi
// type: int __fastcall(RBX::Network::ConcurrentRakPeer **this, char *, int, const void *)
#[doc(alias = "RBX::Network::Server::stop(int)")]
pub fn stub_9c7234(server: &mut crate::server::Server, block_duration_ms: i32) -> bool {
    // IDA 0x9c7274..0x9c72a2: unlock children, drop them, disconnect when active.
    server.stop(block_duration_ms)
}

// 0x9c72a8 — __ZN3RBX7Network6Server14getClientCountEv
// type: _DWORD __fastcall(RBX::Network::Server *__hidden this)
#[doc(alias = "RBX::Network::Server::getClientCount(void)")]
pub fn stub_9c72a8(server: &crate::server::Server) -> usize {
    server.client_count()
}

// 0x9c72d0 — __ZL16createReplicatorN6RakNet13SystemAddressEPN3RBX7Network6ServerEPNS1_15NetworkSettingsE
// type: void __fastcall(int *, int, int, int, pthread_mutex_t *, boost::detail::shared_count *, pthread_mutex_t *, int)
#[doc(alias = "createReplicator(RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *)")]
pub fn stub_9c72d0() -> ! {
    todo!("0x9c72d0 __ZL16createReplicatorN6RakNet13SystemAddressEPN3RBX7Network6ServerEPNS1_15NetworkSettingsE")
}

// 0x9c7444 — __ZN3RBX7Network6ServerC1Ev
// type: int __fastcall(RBX::Network::Server *this)
#[doc(alias = "RBX::Network::Server::Server(void)")]
pub fn stub_9c7444() -> crate::server::Server {
    // IDA 0x9c7444 (C1): Peer init; a fresh server has no players and no bound port.
    crate::server::Server::default()
}

// 0x9c7450 — __ZN3RBX7Network6ServerC2Ev
// type: RBX::Network::Peer *__fastcall(RBX::Network::Server *this)
#[doc(alias = "RBX::Network::Server::Server(void)")]
pub fn stub_9c7450() -> crate::server::Server {
    // IDA 0x9c7450 (C2): Peer init; a fresh server has no players and no bound port.
    crate::server::Server::default()
}

// 0x9c7e78 — __ZN3RBX7Network6ServerD0Ev
// type: void __fastcall(RBX::Network::Server *__hidden this)
#[doc(alias = "RBX::Network::Server::~Server()")]
pub fn stub_9c7e78() -> ! {
    todo!("0x9c7e78 __ZN3RBX7Network6ServerD0Ev")
}

// 0x9c7f18 — __ZN3RBX7Network6ServerD1Ev
// type: void __fastcall(RBX::Network::Server *__hidden this)
#[doc(alias = "RBX::Network::Server::~Server()")]
pub fn stub_9c7f18() -> ! {
    todo!("0x9c7f18 __ZN3RBX7Network6ServerD1Ev")
}

// 0x9c7f24 — __ZThn32_N3RBX7Network6ServerD0Ev
// type: void __fastcall(RBX::Network::Server *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Server::~Server()")]
pub fn stub_9c7f24() -> ! {
    todo!("0x9c7f24 __ZThn32_N3RBX7Network6ServerD0Ev")
}

// 0x9c7fc8 — __ZThn36_N3RBX7Network6ServerD0Ev
// type: void __fastcall(RBX::Network::Server *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Server::~Server()")]
pub fn stub_9c7fc8() -> ! {
    todo!("0x9c7fc8 __ZThn36_N3RBX7Network6ServerD0Ev")
}

// 0x9c806c — __ZThn92_N3RBX7Network6ServerD0Ev
// type: void __fastcall(RBX::Network::Server *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Server::~Server()")]
pub fn stub_9c806c() -> ! {
    todo!("0x9c806c __ZThn92_N3RBX7Network6ServerD0Ev")
}

// 0x9c8110 — __ZN3RBX7Network6ServerD2Ev
// type: void __fastcall(RBX::Network::Server *this, int, int, const void *)
#[doc(alias = "RBX::Network::Server::~Server()")]
pub fn stub_9c8110() -> ! {
    todo!("0x9c8110 __ZN3RBX7Network6ServerD2Ev")
}

// 0x9c87d4 — __ZThn32_N3RBX7Network6ServerD1Ev
// type: void __fastcall(RBX::Network::Server *this, int, int, const void *)
#[doc(alias = "non-virtual thunk toRBX::Network::Server::~Server()")]
pub fn stub_9c87d4() -> ! {
    todo!("0x9c87d4 __ZThn32_N3RBX7Network6ServerD1Ev")
}

// 0x9c87e0 — __ZThn36_N3RBX7Network6ServerD1Ev
// type: void __fastcall(RBX::Network::Server *this, int, int, const void *)
#[doc(alias = "non-virtual thunk toRBX::Network::Server::~Server()")]
pub fn stub_9c87e0() -> ! {
    todo!("0x9c87e0 __ZThn36_N3RBX7Network6ServerD1Ev")
}

// 0x9c87ec — __ZThn92_N3RBX7Network6ServerD1Ev
// type: void __fastcall(RBX::Network::Server *this, int, int, const void *)
#[doc(alias = "non-virtual thunk toRBX::Network::Server::~Server()")]
pub fn stub_9c87ec() -> ! {
    todo!("0x9c87ec __ZThn92_N3RBX7Network6ServerD1Ev")
}

// 0x9c87f8 — __ZN3RBX7Network6Server15serverIsPresentEPKNS_8InstanceEb
// type: bool __fastcall(RBX::Network::Server *this, int, bool, int (*)(const char *, ...))
#[doc(alias = "RBX::Network::Server::serverIsPresent(RBX::Instance const*,bool)")]
pub fn stub_9c87f8() -> ! {
    todo!("0x9c87f8 __ZN3RBX7Network6Server15serverIsPresentEPKNS_8InstanceEb")
}

// 0x9c8b20 — __ZN3RBX7Network6Server15onCreateRakPeerEv
// type: int __fastcall(RBX::Network::ConcurrentRakPeer **this)
#[doc(alias = "RBX::Network::Server::onCreateRakPeer(void)")]
pub fn stub_9c8b20() -> ! {
    todo!("0x9c8b20 __ZN3RBX7Network6Server15onCreateRakPeerEv")
}

// 0x9c8b88 — __ZN3RBX7Network6Server17onServiceProviderEPNS_15ServiceProviderES3_
// type: void __fastcall(struct _Unwind_Exception *this, RBX::ServiceProvider *, pthread_mutex_t *, int)
#[doc(alias = "RBX::Network::Server::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_9c8b88() -> ! {
    todo!("0x9c8b88 __ZN3RBX7Network6Server17onServiceProviderEPNS_15ServiceProviderES3_")
}

// 0x9c9b78 — __ZN3RBX7Network6Server11onItemAddedEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, int, int, int)
#[doc(alias = "RBX::Network::Server::onItemAdded(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_9c9b78() -> ! {
    todo!("0x9c9b78 __ZN3RBX7Network6Server11onItemAddedEN5boost10shared_ptrINS_8InstanceEEE")
}

// 0x9c9f74 — __ZNK3RBX7Network6Server11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::Server *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Server::askAddChild(RBX::Instance const*)const")]
pub fn stub_9c9f74() -> ! {
    todo!("0x9c9f74 __ZNK3RBX7Network6Server11askAddChildEPKNS_8InstanceE")
}

// 0x9c9fb0 — __ZN3RBX7Network6Server9OnReceiveEPN6RakNet6PacketE
// type: int __fastcall(RBX::Instance *, RakNet::SystemAddress *, int, const void *)
#[doc(alias = "RBX::Network::Server::OnReceive(RakNet::Packet *)")]
pub fn stub_9c9fb0() -> ! {
    todo!("0x9c9fb0 __ZN3RBX7Network6Server9OnReceiveEPN6RakNet6PacketE")
}

// 0x9cade8 — __ZThn92_N3RBX7Network6Server9OnReceiveEPN6RakNet6PacketE
// type: int __fastcall(int, RakNet::SystemAddress *, int, const void *)
#[doc(alias = "non-virtual thunk toRBX::Network::Server::OnReceive(RakNet::Packet *)")]
pub fn stub_9cade8() -> ! {
    todo!("0x9cade8 __ZThn92_N3RBX7Network6Server9OnReceiveEPN6RakNet6PacketE")
}

// 0x9cadf8 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6ServerEFviiELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Server,void ()(int,int),2>::~BoundFuncDesc()")]
pub fn stub_9cadf8() -> ! {
    todo!("0x9cadf8 __ZN3RBX10Reflection13BoundFuncDescINS_7Network6ServerEFviiELi2EED1Ev")
}

// 0x9cae6c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6ServerEFviELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Server,void ()(int),1>::~BoundFuncDesc()")]
pub fn stub_9cae6c() -> ! {
    todo!("0x9cae6c __ZN3RBX10Reflection13BoundFuncDescINS_7Network6ServerEFviELi1EED1Ev")
}

// 0x9caed4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6ServerEFivELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Server,int ()(void),0>::~BoundFuncDesc()")]
pub fn stub_9caed4() -> ! {
    todo!("0x9caed4 __ZN3RBX10Reflection13BoundFuncDescINS_7Network6ServerEFivELi0EED1Ev")
}

// 0x9caf1c — __ZNK3RBX7Network6Server7getPortEv
// type: int __fastcall(RBX::Network::Server *this)
#[doc(alias = "RBX::Network::Server::getPort(void)const")]
pub fn stub_9caf1c() -> ! {
    todo!("0x9caf1c __ZNK3RBX7Network6Server7getPortEv")
}

// 0x9caf24 — __ZN3RBX10Reflection14PropDescriptorINS_7Network6ServerEiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Server,int>::~PropDescriptor()")]
pub fn stub_9caf24() -> ! {
    todo!("0x9caf24 __ZN3RBX10Reflection14PropDescriptorINS_7Network6ServerEiED1Ev")
}

// 0x9caf48 — __ZN3RBX7Network6Server33setIsPlayerAuthenticationRequiredEb
// type: int __fastcall(int this, bool)
#[doc(alias = "RBX::Network::Server::setIsPlayerAuthenticationRequired(bool)")]
pub fn stub_9caf48() -> ! {
    todo!("0x9caf48 __ZN3RBX7Network6Server33setIsPlayerAuthenticationRequiredEb")
}

// 0x9caf50 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6ServerEFvbELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Server,void ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_9caf50() -> ! {
    todo!("0x9caf50 __ZN3RBX10Reflection13BoundFuncDescINS_7Network6ServerEFvbELi1EED1Ev")
}

// 0x9cafb8 — __ZN3RBX10Reflection9EventDescINS_7Network6ServerEFvN5boost10shared_ptrINS_8InstanceEEENS2_12FilterResultES7_SsEN3rbx6signalIS9_EEMS3_SC_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Server,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)> RBX::Network::Server::*>::~EventDesc()")]
pub fn stub_9cafb8() -> ! {
    todo!("0x9cafb8 __ZN3RBX10Reflection9EventDescINS_7Network6ServerEFvN5boost10shared_ptrINS_8InstanceEEENS2_12FilterResultES7_SsEN3rbx6signalIS9_EEMS3_SC_ED1Ev")
}

// 0x9cb000 — __ZN3RBX10Reflection9EventDescINS_7Network6ServerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Server,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Server::*>::~EventDesc()")]
pub fn stub_9cb000() -> ! {
    todo!("0x9cb000 __ZN3RBX10Reflection9EventDescINS_7Network6ServerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev")
}

// 0x9cb048 — __ZN5boost8functionIFNS_10shared_ptrIN3RBX7Network16ServerReplicatorEEEN6RakNet13SystemAddressEPNS3_6ServerEPNS2_15NetworkSettingsEEED1Ev
// type: int *__fastcall(int *)
#[doc(alias = "boost::function<rbx_core::SharedPtr<RBX::Network::ServerReplicator> ()(RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *)>::~function()")]
pub fn stub_9cb048() -> ! {
    todo!("0x9cb048 __ZN5boost8functionIFNS_10shared_ptrIN3RBX7Network16ServerReplicatorEEEN6RakNet13SystemAddressEPNS3_6ServerEPNS2_15NetworkSettingsEEED1Ev")
}

// 0x9cb40c — __ZN5boost10shared_ptrIN3RBX7Network7PlayersEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Players>::reset(void)")]
pub fn stub_9cb40c() -> ! {
    todo!("0x9cb40c __ZN5boost10shared_ptrIN3RBX7Network7PlayersEE5resetEv")
}

// 0x9cb4ac — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network6ServerENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS8_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>> const&)const")]
pub fn stub_9cb4ac() -> ! {
    todo!("0x9cb4ac __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network6ServerENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS8_EENS2_3argILi1EEEEEEEEEvRKT_")
}

// 0x9cb9f0 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15NetworkOwnerJobES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::NetworkOwnerJob,RBX::Network::NetworkOwnerJob>(rbx_core::SharedPtr<RBX::Network::NetworkOwnerJob> *,RBX::Network::NetworkOwnerJob *,boost::detail::shared_count &)")]
pub fn stub_9cb9f0() -> ! {
    todo!("0x9cb9f0 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15NetworkOwnerJobES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

// 0x9cbba0 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network15NetworkOwnerJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::NetworkOwnerJob,RBX::Network::NetworkOwnerJob>(rbx_core::SharedPtr<RBX::Network::NetworkOwnerJob> const*,RBX::Network::NetworkOwnerJob *)const")]
pub fn stub_9cbba0() -> ! {
    todo!("0x9cbba0 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network15NetworkOwnerJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0x9cbe4c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network15NetworkOwnerJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::NetworkOwnerJob>::~sp_counted_impl_p()")]
pub fn stub_9cbe4c() -> ! {
    todo!("0x9cbe4c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network15NetworkOwnerJobEED1Ev")
}
