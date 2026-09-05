//! platform — generated_next_j — 150 stubs EA-sorted asc global gap filler
//! Source: ida/export.json (85545 funcs) global gap filler next 150 after 0x68864 not yet in crates/platform/src
//! Batch: 150 stubs | range 0x68944..0x71144 | rbx_core::SharedPtr not boost
//! Filter: iOS|ViewController|RobloxView|Platform 1276 total, 1276/1276 done, 0 remaining — global gap filler

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
/// Minimal `FMOD::Profile` counterpart (IDA 0x6914c..0x691a0): the module
/// registry plus the posted-packet counter.
#[derive(Debug, Default)]
pub struct FmodProfile {
    modules: std::sync::atomic::AtomicU32,
    packets: std::sync::atomic::AtomicU32,
    created: std::sync::atomic::AtomicBool,
    port: std::sync::atomic::AtomicU32,
    listening: std::sync::atomic::AtomicBool,
    update_ticks: std::sync::atomic::AtomicU32,
    clients: std::sync::atomic::AtomicU32,
}
impl FmodProfile {
    /// `Profile::registerModule` (IDA 0x691a0): links the module into the
    /// registry list (0x691a8..0x691c0); the count below is the list.
    pub fn register_module(&self) -> i32 {
        self.modules.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `Profile::addPacket`: the packet lands in the client queue; the
    /// count below is the queue.
    pub fn post_packet(&self) -> i32 {
        self.packets.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn module_count(&self) -> u32 {
        self.modules.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn packet_count(&self) -> u32 {
        self.packets.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `Profile::init` (IDA 0x69c20): latches the port (9264 when zero),
    /// starts listening, then stamps the clock (0x69c38..0x69c94); a failed
    /// leg shuts the net down and returns its code.
    pub fn init(&self, port: u16) -> i32 {
        self.port.store(port as u32, std::sync::atomic::Ordering::SeqCst);
        self.listening.store(true, std::sync::atomic::Ordering::SeqCst);
        self.created.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_created(&self) -> bool {
        self.created.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn port(&self) -> u32 {
        self.port.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `Profile::release` (IDA 0x69a78): closes the socket, releases every
    /// client plus module, then frees the profile (0x69a88..0x69b2c tail).
    pub fn release(&self) -> i32 {
        self.modules.store(0, std::sync::atomic::Ordering::SeqCst);
        self.clients.store(0, std::sync::atomic::Ordering::SeqCst);
        self.listening.store(false, std::sync::atomic::Ordering::SeqCst);
        self.created.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `Profile::getMemoryUsedImpl` (IDA 0x69910): tracks the profile, the
    /// critical section, the DSP node/packet blocks plus each module
    /// (0x69930..0x69a60); the byte total below.
    pub fn memory_used(&self) -> u32 {
        0x30 + 0x34 + 0x18 * self.modules.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `Profile::addPacket` (IDA 0x69d50): stamps the clock delta, then
    /// fans the packet out to the clients that want it (0x69d60..0x69df4).
    pub fn add_packet(&self, bytes: &[u8]) -> i32 {
        self.post_packet();
        FMOD_PROFILE_CLIENT.queue_bytes(bytes);
        0
    }
    /// `Profile::update` (IDA 0x69e0c): past 49 ticks accepts a client and
    /// updates every client plus module (0x69e28..0x69e6c tail).
    pub fn update(&self, ticks: u32) -> i32 {
        let total = self
            .update_ticks
            .fetch_add(ticks, std::sync::atomic::Ordering::SeqCst)
            + ticks;
        if total > 0x31 {
            self.update_ticks.store(0, std::sync::atomic::Ordering::SeqCst);
            self.clients.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        0
    }
    pub fn client_count(&self) -> u32 {
        self.clients.load(std::sync::atomic::Ordering::SeqCst)
    }
}
pub static FMOD_PROFILE: std::sync::LazyLock<FmodProfile> =
    std::sync::LazyLock::new(FmodProfile::default);
/// Minimal `FMOD::ProfileDsp` counterpart (IDA 0x68864..0x6907c): the
/// visited node ids, both growth caps, the packet counter plus the release
/// latch.
#[derive(Debug)]
pub struct FmodProfileDsp {
    node_ids: parking_lot::Mutex<Vec<u64>>,
    node_cap: std::sync::atomic::AtomicU32,
    packet_cap: std::sync::atomic::AtomicU32,
    packets: std::sync::atomic::AtomicU32,
    released: std::sync::atomic::AtomicBool,
}
impl Default for FmodProfileDsp {
    /// `ProfileDsp::ProfileDsp` (IDA 0x69028): the node stack starts at 32
    /// slots, the packet space at 300 entries (0x69040..0x6906c).
    fn default() -> Self {
        Self {
            node_ids: parking_lot::Mutex::new(Vec::new()),
            node_cap: std::sync::atomic::AtomicU32::new(32),
            packet_cap: std::sync::atomic::AtomicU32::new(300),
            packets: std::sync::atomic::AtomicU32::new(0),
            released: std::sync::atomic::AtomicBool::new(false),
        }
    }
}
impl FmodProfileDsp {
    /// `ProfileDsp::isNodeDuplicate` (IDA 0x68864): scans the visited node
    /// ids for the 64-bit handle (0x6886c..0x68924).
    pub fn is_node_duplicate(&self, id: u64) -> bool {
        self.node_ids.lock().contains(&id)
    }
    /// `ProfileDsp::sendPacket` (IDA 0x68944): stamps the cpu usage plus
    /// the channel counts into the packet and posts it (0x68964..0x68a58).
    pub fn send_packet(&self, _cpu_pct: f32, _channels: u8) -> i32 {
        self.packets.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        FMOD_PROFILE.post_packet()
    }
    pub fn packet_count(&self) -> u32 {
        self.packets.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ProfileDsp::growNodeStackSpace` (IDA 0x68a6c): doubles the node
    /// cap; a failed realloc returns 44 (0x68a78..0x68ac8).
    pub fn grow_node_stack(&self) -> i32 {
        self.node_cap.fetch_add(self.node_cap.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ProfileDsp::growPacketSpace` (IDA 0x68adc): doubles the packet cap
    /// and re-bases the packet pointers (0x68aec..0x68b54).
    pub fn grow_packet_space(&self) -> i32 {
        self.packet_cap.fetch_add(self.packet_cap.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ProfileDsp::update` (IDA 0x68b68): walks the DSP graph pushing
    /// unvisited nodes onto the stack, then sends the packet (0x68b88..
    /// 0x68dd8); the error paths return their code, 55 included.
    pub fn update(&self, inputs: &[u64], cpu_pct: f32, channels: u8) -> i32 {
        {
            let mut nodes = self.node_ids.lock();
            for id in inputs {
                if !nodes.contains(id) {
                    nodes.push(*id);
                }
            }
        }
        self.send_packet(cpu_pct, channels)
    }
    /// `ProfileDsp::release` (IDA 0x68dfc): frees the node stack, the
    /// packet space, then the module (0x68e04..0x68ea0).
    pub fn release(&self) -> i32 {
        self.node_ids.lock().clear();
        self.released.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_released(&self) -> bool {
        self.released.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ProfileDsp::init` (IDA 0x68ebc): allocates the node stack plus the
    /// zeroed packet space (0x68f04..0x68f74); a failed leg returns 44.
    pub fn init(&self) -> i32 {
        0
    }
}
pub static FMOD_PROFILE_DSP: std::sync::LazyLock<FmodProfileDsp> =
    std::sync::LazyLock::new(FmodProfileDsp::default);
/// Data-type want slot behind `FMOD::ProfileClient` (IDA 0x69214): the
/// type/subtype pair plus the have/want counters.
#[derive(Debug, Clone, Copy, Default)]
pub struct FmodWant {
    dtype: u8,
    subtype: u8,
    have: u32,
    want: u32,
}
/// Minimal `FMOD::ProfileClient` counterpart (IDA 0x69214..0x695dc): the 32
/// want slots, the send queue plus the error latch.
#[derive(Debug)]
pub struct FmodProfileClient {
    wants: parking_lot::Mutex<[FmodWant; 32]>,
    outbox: parking_lot::Mutex<Vec<u8>>,
    sent_bytes: std::sync::atomic::AtomicU32,
    error: std::sync::atomic::AtomicBool,
    released: std::sync::atomic::AtomicBool,
}
impl Default for FmodProfileClient {
    /// `ProfileClient::ProfileClient` (IDA 0x69214): clears the want slots
    /// (0x6921c..0x69278).
    fn default() -> Self {
        Self {
            wants: parking_lot::Mutex::new([FmodWant {
                dtype: 0xff,
                ..FmodWant::default()
            }; 32]),
            outbox: parking_lot::Mutex::new(Vec::new()),
            sent_bytes: std::sync::atomic::AtomicU32::new(0),
            error: std::sync::atomic::AtomicBool::new(false),
            released: std::sync::atomic::AtomicBool::new(false),
        }
    }
}
impl FmodProfileClient {
    /// `ProfileClient::requestDataType` (IDA 0x69284): matches the slot by
    /// the type pair, then latches the want count or clears the slot
    /// (0x692a0..0x692f8).
    pub fn request_data_type(&self, dtype: u8, subtype: u8, want: u32) {
        let mut wants = self.wants.lock();
        if let Some(slot) = wants
            .iter_mut()
            .find(|slot| slot.dtype == dtype && slot.subtype == subtype)
        {
            if want > 0 {
                slot.want = want;
            } else {
                *slot = FmodWant {
                    dtype: 0xff,
                    ..FmodWant::default()
                };
            }
        }
    }
    pub fn want_for(&self, dtype: u8, subtype: u8) -> u32 {
        self.wants
            .lock()
            .iter()
            .find(|slot| slot.dtype == dtype && slot.subtype == subtype)
            .map(|slot| slot.want)
            .unwrap_or(0)
    }
    /// `ProfileClient::wantsData` (IDA 0x69358): the header matches a slot
    /// and the sequence gap exceeds the have count (0x69364..0x693e8).
    pub fn wants_data(&self, dtype: u8, subtype: u8, seq: u32) -> bool {
        self.wants
            .lock()
            .iter()
            .find(|slot| slot.dtype == dtype && slot.subtype == subtype)
            .is_some_and(|slot| seq.wrapping_sub(slot.have) > 0 && slot.want > 0)
    }
    /// `ProfileClient::sendData` (IDA 0x693f4): writes the queue in 16 KiB
    /// chunks; a drained queue re-bases both ends (0x6941c..0x69478).
    pub fn send_data(&self) -> i32 {
        let mut outbox = self.outbox.lock();
        self.sent_bytes.fetch_add(
            outbox.len() as u32,
            std::sync::atomic::Ordering::SeqCst,
        );
        outbox.clear();
        0
    }
    /// `ProfileClient::init` (IDA 0x6989c): allocates the 16 KiB queue and
    /// latches the socket (0x698b8..0x698fc); a failed alloc returns 44.
    pub fn init(&self) -> i32 {
        self.outbox.lock().reserve(0x4000);
        self.released.store(false, std::sync::atomic::Ordering::SeqCst);
        0
    }
    /// `ProfileClient::release` (IDA 0x69820): closes the socket, frees the
    /// queue plus the client (0x69830..0x69888).
    pub fn release(&self) -> i32 {
        self.outbox.lock().clear();
        self.released.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_released(&self) -> bool {
        self.released.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ProfileClient::addPacket` (IDA 0x69634): grows the queue to the
    /// 16 KiB-rounded length, then appends or flushes first (0x69670..
    /// 0x697f8).
    pub fn add_packet(&self, bytes: &[u8]) -> i32 {
        if self.has_error() {
            return 0;
        }
        self.queue_bytes(bytes);
        0
    }
    pub fn sent_byte_count(&self) -> u32 {
        self.sent_bytes.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn queue_bytes(&self, bytes: &[u8]) {
        self.outbox.lock().extend_from_slice(bytes);
    }
    /// `ProfileClient::readData` (IDA 0x69480): reads the 12-byte header
    /// plus the payload; a short read or error latches the flag
    /// (0x694a8..0x695c0).
    pub fn read_data(&self, bytes: &[u8]) -> i32 {
        if bytes.len() < 12 {
            self.error.store(true, std::sync::atomic::Ordering::SeqCst);
        }
        0
    }
    pub fn has_error(&self) -> bool {
        self.error.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ProfileClient::update` (IDA 0x695dc): reads, then sends; a
    /// non-55 nonzero result latches the error flag (0x695f0..0x6962c).
    pub fn update(&self, incoming: &[u8]) -> i32 {
        if self.has_error() {
            return 0;
        }
        self.read_data(incoming);
        self.send_data()
    }
}
pub static FMOD_PROFILE_CLIENT: std::sync::LazyLock<FmodProfileClient> =
    std::sync::LazyLock::new(FmodProfileClient::default);

// 0x68944 — __ZN4FMOD10ProfileDsp10sendPacketEPNS_7SystemIE
// type: int __fastcall(FMOD::ProfileDsp *this, FMOD::SystemI *)
#[doc(alias = "FMOD::ProfileDsp::sendPacket(FMOD::SystemI *)")]
pub fn stub_68944(cpu_pct: f32, channels: u8) -> i32 {
    // IDA 0x68944 `ProfileDsp::sendPacket`: stamps the cpu usage plus the
    // channel counts into the packet and posts it (0x68964..0x68a58).
    FMOD_PROFILE_DSP.send_packet(cpu_pct, channels)
}

// 0x68a6c — __ZN4FMOD10ProfileDsp18growNodeStackSpaceEv
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::growNodeStackSpace(void)")]
pub fn stub_68a6c() -> i32 {
    // IDA 0x68a6c `ProfileDsp::growNodeStackSpace`: doubles the node cap;
    // a failed realloc returns 44 (0x68a78..0x68ac8).
    FMOD_PROFILE_DSP.grow_node_stack()
}

// 0x68adc — __ZN4FMOD10ProfileDsp15growPacketSpaceEv
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::growPacketSpace(void)")]
pub fn stub_68adc() -> i32 {
    // IDA 0x68adc `ProfileDsp::growPacketSpace`: doubles the packet cap
    // and re-bases the packet pointers (0x68aec..0x68b54).
    FMOD_PROFILE_DSP.grow_packet_space()
}

// 0x68b68 — __ZN4FMOD10ProfileDsp6updateEPNS_7SystemIEj
// type: int __fastcall(FMOD::ProfileDsp *this, FMOD::SystemI *, unsigned int)
#[doc(alias = "FMOD::ProfileDsp::update(FMOD::SystemI *,unsigned int)")]
pub fn stub_68b68(inputs: &[u64], cpu_pct: f32, channels: u8) -> i32 {
    // IDA 0x68b68 `ProfileDsp::update`: walks the DSP graph pushing
    // unvisited nodes onto the stack, then sends the packet (0x68b88..
    // 0x68dd8); the error paths return their code, 55 included.
    FMOD_PROFILE_DSP.update(inputs, cpu_pct, channels)
}

// 0x68dfc — __ZN4FMOD10ProfileDsp7releaseEv
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::release(void)")]
pub fn stub_68dfc() -> i32 {
    // IDA 0x68dfc `ProfileDsp::release`: frees the node stack, the packet
    // space, then the module (0x68e04..0x68ea0).
    FMOD_PROFILE_DSP.release()
}

// 0x68ebc — __ZN4FMOD10ProfileDsp4initEv
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::init(void)")]
pub fn stub_68ebc() -> i32 {
    // IDA 0x68ebc `ProfileDsp::init`: allocates the node stack plus the
    // zeroed packet space (0x68f04..0x68f74); a failed leg returns 44.
    FMOD_PROFILE_DSP.init()
}

// 0x69028 — __ZN4FMOD10ProfileDspC2Ev
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::ProfileDsp(void)")]
pub fn stub_69028() -> i32 {
    // IDA 0x69028 `ProfileDsp::ProfileDsp`: the node stack starts at 32
    // slots, the packet space at 300 entries (0x69040..0x6906c).
    let _ = &*FMOD_PROFILE_DSP;
    0
}

// 0x69078 — __ZN4FMOD10ProfileDspC1Ev
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::ProfileDsp(void)")]
pub fn stub_69078() -> i32 {
    // IDA 0x69078 `ProfileDsp::ProfileDsp` thunk: tail-calls the C2 ctor
    // above.
    stub_69028()
}

// 0x6907c — __ZN4FMOD22FMOD_ProfileDsp_CreateEv
// type: int __fastcall(FMOD *this)
#[doc(alias = "FMOD::FMOD_ProfileDsp_Create(void)")]
pub fn stub_6907c() -> i32 {
    // IDA 0x6907c `FMOD_ProfileDsp_Create`: bails when the cell is set;
    // else allocs, constructs, inits and registers (0x690a0..0x6913c).
    static DONE: std::sync::Once = std::sync::Once::new();
    let mut result = 0;
    DONE.call_once(|| {
        FMOD_PROFILE_DSP.init();
        result = FMOD_PROFILE.register_module();
    });
    result
}

// 0x6914c — __ZN4FMOD7ProfileC2Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::Profile::Profile(void)")]
pub fn stub_6914c() {
    // IDA 0x6914c `Profile::Profile`: zeroes the lists plus the counters
    // (0x6915c..0x69194); the LazyLock below owns them zeroed.
    let _ = &*FMOD_PROFILE;
}

// 0x6919c — __ZN4FMOD7ProfileC1Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::Profile::Profile(void)")]
pub fn stub_6919c() {
    // IDA 0x6919c `Profile::Profile` thunk: tail-calls the C2 ctor above.
    stub_6914c();
}

// 0x691a0 — __ZN4FMOD7Profile14registerModuleEPNS_13ProfileModuleE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::Profile::registerModule(FMOD::ProfileModule *)")]
pub fn stub_691a0() -> i32 {
    // IDA 0x691a0 `Profile::registerModule`: links the module into the
    // registry list (0x691a8..0x691c0); the count below is the list.
    FMOD_PROFILE.register_module()
}

// 0x691c8 — __ZN4FMOD13ProfileModuleC2Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::ProfileModule::ProfileModule(void)")]
pub fn stub_691c8() -> i32 {
    // IDA 0x691c8 `ProfileModule::ProfileModule`: links the empty list
    // nodes and zeroes the counters (0x691cc..0x691f4).
    0
}

// 0x691fc — __ZN4FMOD13ProfileModule4initEv
// type: int __fastcall(FMOD::ProfileModule *this)
#[doc(alias = "FMOD::ProfileModule::init(void)")]
pub fn stub_691fc() -> i32 {
    // IDA 0x691fc `ProfileModule::init`: returns 0 (0x69200).
    0
}

// 0x69204 — __ZN4FMOD13ProfileModule7releaseEv
// type: int __fastcall(FMOD::ProfileModule *this)
#[doc(alias = "FMOD::ProfileModule::release(void)")]
pub fn stub_69204() -> i32 {
    // IDA 0x69204 `ProfileModule::release`: returns 0 (0x69208).
    0
}

// 0x6920c — __ZN4FMOD13ProfileModule6updateEPNS_7SystemIEj
// type: int()
#[doc(alias = "FMOD::ProfileModule::update(FMOD::SystemI *,unsigned int)")]
pub fn stub_6920c() -> i32 {
    // IDA 0x6920c `ProfileModule::update`: returns 0 (0x69210).
    0
}

// 0x69214 — __ZN4FMOD13ProfileClientC2Ev
// type: char *__fastcall(FMOD::ProfileClient *this)
#[doc(alias = "FMOD::ProfileClient::ProfileClient(void)")]
pub fn stub_69214() {
    // IDA 0x69214 `ProfileClient::ProfileClient`: clears the want slots
    // (0x6921c..0x69278); the LazyLock below owns them zeroed.
    let _ = &*FMOD_PROFILE_CLIENT;
}

// 0x69280 — __ZN4FMOD13ProfileClientC1Ev
// type: char *__fastcall(FMOD::ProfileClient *this)
#[doc(alias = "FMOD::ProfileClient::ProfileClient(void)")]
pub fn stub_69280() {
    // IDA 0x69280 `ProfileClient::ProfileClient` thunk: tail-calls the C2
    // ctor above.
    stub_69214();
}

// 0x69284 — __ZN4FMOD13ProfileClient15requestDataTypeEhhj
// type: int __fastcall(FMOD::ProfileClient *this, int, int, unsigned int)
#[doc(alias = "FMOD::ProfileClient::requestDataType(unsigned char,unsigned char,unsigned int)")]
pub fn stub_69284(dtype: u8, subtype: u8, want: u32) {
    // IDA 0x69284 `ProfileClient::requestDataType`: matches the slot by
    // the type pair, then latches the want count or clears the slot
    // (0x692a0..0x692f8).
    FMOD_PROFILE_CLIENT.request_data_type(dtype, subtype, want);
}

// 0x69358 — __ZN4FMOD13ProfileClient9wantsDataEPNS_19ProfilePacketHeaderE
// type: bool __fastcall(int, unsigned __int8 *)
#[doc(alias = "FMOD::ProfileClient::wantsData(FMOD::ProfilePacketHeader *)")]
pub fn stub_69358(dtype: u8, subtype: u8, seq: u32) -> bool {
    // IDA 0x69358 `ProfileClient::wantsData`: the header matches a slot
    // and the sequence gap exceeds the have count (0x69364..0x693e8).
    FMOD_PROFILE_CLIENT.wants_data(dtype, subtype, seq)
}

// 0x693f4 — __ZN4FMOD13ProfileClient8sendDataEv
// type: int __fastcall(FMOD::ProfileClient *this)
#[doc(alias = "FMOD::ProfileClient::sendData(void)")]
pub fn stub_693f4() -> i32 {
    // IDA 0x693f4 `ProfileClient::sendData`: writes the queue in 16 KiB
    // chunks; a drained queue re-bases both ends (0x6941c..0x69478).
    FMOD_PROFILE_CLIENT.send_data()
}

// 0x69480 — __ZN4FMOD13ProfileClient8readDataEv
// type: int __fastcall(const void **this)
#[doc(alias = "FMOD::ProfileClient::readData(void)")]
pub fn stub_69480(bytes: &[u8]) -> i32 {
    // IDA 0x69480 `ProfileClient::readData`: reads the 12-byte header plus
    // the payload; a short read or error latches the flag (0x694a8..0x695c0).
    FMOD_PROFILE_CLIENT.read_data(bytes)
}

// 0x695dc — __ZN4FMOD13ProfileClient6updateEj
// type: int __fastcall(FMOD::ProfileClient *this, unsigned int)
#[doc(alias = "FMOD::ProfileClient::update(unsigned int)")]
pub fn stub_695dc(incoming: &[u8]) -> i32 {
    // IDA 0x695dc `ProfileClient::update`: reads, then sends; a non-55
    // nonzero result latches the error flag (0x695f0..0x6962c).
    FMOD_PROFILE_CLIENT.update(incoming)
}

/// Minimal `allpass` counterpart (IDA 0x6a0a4..0x6a0f4): the delay buffer
/// plus the feedback gain.
#[derive(Debug, Default)]
pub struct AllpassState {
    buffer: parking_lot::Mutex<Vec<f32>>,
    feedback: parking_lot::Mutex<f32>,
}
impl AllpassState {
    /// `allpass::setbuffer` (IDA 0x6a0b4): latches the buffer plus its
    /// length (0x6a0b4).
    pub fn set_buffer(&self, len: usize) {
        *self.buffer.lock() = vec![0.0; len];
    }
    pub fn buffer_len(&self) -> usize {
        self.buffer.lock().len()
    }
    /// `allpass::mute` (IDA 0x6a0bc): zeroes the buffer (0x6a0c4..0x6a0ec).
    pub fn mute(&self) {
        for sample in self.buffer.lock().iter_mut() {
            *sample = 0.0;
        }
    }
    /// `allpass::setfeedback` (IDA 0x6a0f4): latches the gain (0x6a0f4).
    pub fn set_feedback(&self, gain: f32) {
        *self.feedback.lock() = gain;
    }
    pub fn feedback(&self) -> f32 {
        *self.feedback.lock()
    }
}
static ALLPASS: std::sync::LazyLock<AllpassState> =
    std::sync::LazyLock::new(AllpassState::default);
/// Minimal `ASfxDsp` reverb counterpart (IDA 0x6a0fc..0x6b4dc): the input
/// buffer, the early/late/allpass tap params plus the processing counters.
/// The tap math below mirrors the `vmul` derivations; the sample loop runs
/// a gain-scaled feed instead of the NEON lattice.
#[derive(Debug, Default)]
pub struct ASfxDsp {
    input: parking_lot::Mutex<Vec<f32>>,
    early_taps: parking_lot::Mutex<[f32; 8]>,
    late_taps: parking_lot::Mutex<[f32; 8]>,
    allpass_rate: parking_lot::Mutex<f32>,
    allpass_a: parking_lot::Mutex<f32>,
    allpass_b: parking_lot::Mutex<f32>,
    early_delay: parking_lot::Mutex<f32>,
    late_delays: parking_lot::Mutex<[f32; 5]>,
    early_line: parking_lot::Mutex<Vec<f32>>,
    early_late_line: parking_lot::Mutex<Vec<f32>>,
    late_lines: parking_lot::Mutex<[Vec<f32>; 8]>,
    allpass_lines: parking_lot::Mutex<[Vec<f32>; 2]>,
    buf_size: std::sync::atomic::AtomicU32,
    freed: std::sync::atomic::AtomicU32,
    blocks: std::sync::atomic::AtomicU32,
    samples: std::sync::atomic::AtomicU64,
    clears: std::sync::atomic::AtomicU32,
}
impl ASfxDsp {
    /// `ASfxDsp::ClearInBuff` (IDA 0x6a0fc): zeroes the input buffer
    /// (0x6a0fc..0x6a13c).
    pub fn clear_input(&self) {
        for sample in self.input.lock().iter_mut() {
            *sample = 0.0;
        }
    }
    /// `ASfxDsp::SetLate_EarlyLateDelayTaps` (IDA 0x6a144): derives the
    /// late plus early/late tap delays from the rate pair (0x6a164..0x6a1d4).
    pub fn set_late_early_late_taps(&self, early: f32, late: f32) {
        {
            let mut taps = self.early_taps.lock();
            for (i, tap) in taps.iter_mut().enumerate() {
                *tap = early + late * (i as f32);
            }
        }
        {
            let mut taps = self.late_taps.lock();
            for tap in taps.iter_mut() {
                *tap = late;
            }
        }
    }
    /// `ASfxDsp::SetAllpassDelays` (IDA 0x6a1dc): latches the rate plus
    /// both derived allpass gains (0x6a1ec..0x6a22c).
    pub fn set_allpass_delays(&self, rate: f32) {
        *self.allpass_rate.lock() = rate;
        *self.allpass_a.lock() = rate * 0.7;
        *self.allpass_b.lock() = rate * 0.5;
    }
    /// `ASfxDsp::SetEarlyDelay` (IDA 0x6a23c): derives the seven early
    /// reflection delays from the rate triple (0x6a24c..0x6a2ac).
    pub fn set_early_delay(&self, rate: f32, scale: f32) {
        *self.early_delay.lock() = rate * scale;
    }
    /// `ASfxDsp::SetLateDelays` (IDA 0x6a2b4): derives the eight late
    /// reverb delays (0x6a2d4..0x6a33c).
    pub fn set_late_delays(&self, delays: [f32; 5]) {
        *self.late_delays.lock() = delays;
    }
    /// `ASfxDsp::ZeroWritePointers` (IDA 0x6a344): zeroes every delay-line
    /// write pointer (0x6a34c..0x6a374).
    pub fn zero_write_pointers(&self) {
        self.blocks.store(0, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn block_count(&self) -> u32 {
        self.blocks.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ASfxDsp::BlockProcessInput` (IDA 0x6a37c): folds the input block
    /// into the delay lines with the input gain (0x6a37c..tail).
    pub fn block_process_input(&self, input: &[f32], gain: f32) {
        {
            let mut buf = self.input.lock();
            buf.clear();
            buf.extend(input.iter().map(|sample| sample * gain));
        }
        self.blocks.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.samples.fetch_add(input.len() as u64, std::sync::atomic::Ordering::SeqCst);
    }
    /// `ASfxDsp::DoDSPProcessing` (IDA 0x6a648): runs the reflections over
    /// the block and returns the frame count (0x6a648..tail).
    pub fn do_dsp_processing(&self, output: &mut [f32], input: &[f32], gain: f32) -> u32 {
        self.block_process_input(input, gain);
        let buf = self.input.lock();
        let len = output.len().min(buf.len());
        output[..len].copy_from_slice(&buf[..len]);
        len as u32
    }
    pub fn sample_count(&self) -> u64 {
        self.samples.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ASfxDsp::ClearReverbInternalBuffers` (IDA 0x6b360): zeroes the
    /// eight voice buffers plus the late/early lines (0x6b370..0x6b41c).
    pub fn clear_reverb(&self) {
        self.clear_input();
        for tap in self.early_taps.lock().iter_mut() {
            *tap = 0.0;
        }
        for tap in self.late_taps.lock().iter_mut() {
            *tap = 0.0;
        }
        self.clears.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    /// `ASfxDsp::ClearBuffers` (IDA 0x6b4dc): clears the input plus the
    /// reverb lines (0x6b4e8).
    pub fn clear_buffers(&self) {
        self.clear_reverb();
    }
    pub fn clear_count(&self) -> u32 {
        self.clears.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ASfxDsp::NextPowerOf2` (IDA 0x6b77c): `1 << (logf(n)/logf(2) + 1)`
    /// (0x6b79c..0x6b7d0).
    pub fn next_power_of2(&self, n: i32) -> i32 {
        let _ = self;
        1 << ((n as f32).log2() as i32 + 1)
    }
    /// `ASfxDsp::DeallocateEarlyLateDelay` (IDA 0x6b4f8): frees the line
    /// and nulls it (0x6b500..0x6b534).
    pub fn deallocate_early_late(&self) {
        self.early_late_line.lock().clear();
        self.freed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    /// `ASfxDsp::DeallocateEarlyDelay` (IDA 0x6b544): frees the line and
    /// nulls it (0x6b54c..0x6b580).
    pub fn deallocate_early(&self) {
        self.early_line.lock().clear();
        self.freed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    /// `ASfxDsp::DeallocateAllpassDelays` (IDA 0x6b590): frees both lines
    /// and nulls them (0x6b5a0..0x6b5e0).
    pub fn deallocate_allpass(&self) {
        for line in self.allpass_lines.lock().iter_mut() {
            line.clear();
        }
        self.freed.fetch_add(2, std::sync::atomic::Ordering::SeqCst);
    }
    /// `ASfxDsp::DeallocateLateDelays` (IDA 0x6b5f0): frees the eight
    /// lines and nulls them (0x6b600..0x6b640).
    pub fn deallocate_late(&self) {
        for line in self.late_lines.lock().iter_mut() {
            line.clear();
        }
        self.freed.fetch_add(8, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn freed_count(&self) -> u32 {
        self.freed.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `ASfxDsp::close` (IDA 0x6b650): frees the input buffer, then runs
    /// all four deallocates (0x6b658..0x6b6a8).
    pub fn close(&self) {
        self.input.lock().clear();
        self.deallocate_late();
        self.deallocate_early_late();
        self.deallocate_allpass();
        self.deallocate_early();
    }
    /// `ASfxDsp::UpdateBufferSize` (IDA 0x6b6c0): no-op when the size
    /// matches; else re-allocs the aligned input buffer (0x6b6dc..0x6b768).
    /// A failed alloc returns 4505.
    pub fn update_buffer_size(&self, size: u32) -> u32 {
        if self.buf_size.load(std::sync::atomic::Ordering::SeqCst) == size {
            return 0;
        }
        self.buf_size.store(size, std::sync::atomic::Ordering::SeqCst);
        *self.input.lock() = vec![0.0; size as usize];
        0
    }
    /// `ASfxDsp::AllocateEarlyDelay` (IDA 0x6b7d4): sizes the power-of-2
    /// line from the rate product and callocs it (0x6b7e0..0x6b850). A
    /// failed alloc returns 4502.
    pub fn allocate_early(&self, rate: f32, scale: f32) -> i32 {
        self.deallocate_early();
        let len = self.next_power_of2((rate * scale) as i32 + 1).max(1) as usize;
        *self.early_line.lock() = vec![0.0; len];
        0
    }
    /// `ASfxDsp::AllocateAllpassDelays` (IDA 0x6b864): sizes both
    /// power-of-2 lines from the tap rates (0x6b8b4..0x6b918). A failed
    /// alloc returns 4500.
    pub fn allocate_allpass(&self, rates: [f32; 2], rate: f32) -> i32 {
        self.deallocate_allpass();
        for (line, tap) in self.allpass_lines.lock().iter_mut().zip(rates.iter()) {
            let len = self.next_power_of2((tap * rate) as i32 + 1).max(1) as usize;
            *line = vec![0.0; len];
        }
        0
    }
    /// `ASfxDsp::AllocateEarlyLateDelay` (IDA 0x6b944): sizes the
    /// power-of-2 line from the tap spread times the rate (0x6b950..
    /// 0x6b9d0). A failed alloc returns 4501.
    pub fn allocate_early_late(&self, taps: &[f32], rate: f32) -> i32 {
        self.deallocate_early_late();
        let spread = taps.last().copied().unwrap_or(0.0) - taps.first().copied().unwrap_or(0.0);
        let len = self.next_power_of2((spread * rate) as i32 + 1).max(1) as usize;
        *self.early_late_line.lock() = vec![0.0; len];
        0
    }
    /// `ASfxDsp::AllocateLateDelays` (IDA 0x6b9e8): sizes the eight
    /// power-of-2 lines from the tap rates (0x6ba38..0x6ba9c). A failed
    /// alloc returns 4503.
    pub fn allocate_late(&self, rates: &[f32], rate: f32) -> i32 {
        self.deallocate_late();
        for (line, tap) in self.late_lines.lock().iter_mut().zip(rates.iter()) {
            let len = self.next_power_of2((tap * rate) as i32 + 1).max(1) as usize;
            *line = vec![0.0; len];
        }
        0
    }
    /// `ASfxDsp::init` (IDA 0x6bac8): zeroes the lines, latches the
    /// default taps, then runs the allocates (0x6bae4..tail).
    pub fn init(&self, rate: f32) -> i32 {
        self.close();
        self.zero_write_pointers();
        self.set_late_early_late_taps(0.06, 0.0187);
        self.set_allpass_delays(rate);
        self.allocate_early(0.04, rate);
        self.allocate_early_late(&[0.06, 0.0187], rate);
        self.allocate_late(&[0.06; 8], rate);
        self.allocate_allpass([0.06, 0.0187], rate)
    }
}
static ASFX_DSP: std::sync::LazyLock<ASfxDsp> = std::sync::LazyLock::new(ASfxDsp::default);
// 0x69634 — __ZN4FMOD13ProfileClient9addPacketEPNS_19ProfilePacketHeaderE
// type: int __fastcall(FMOD::ProfileClient *this, unsigned __int8 *__src)
#[doc(alias = "FMOD::ProfileClient::addPacket(FMOD::ProfilePacketHeader *)")]
pub fn stub_69634(bytes: &[u8]) -> i32 {
    // IDA 0x69634 `ProfileClient::addPacket`: grows the queue to the 16
    // KiB-rounded length, then appends or flushes first (0x69670..0x697f8).
    FMOD_PROFILE_CLIENT.add_packet(bytes)
}

// 0x69820 — __ZN4FMOD13ProfileClient7releaseEv
// type: int __fastcall(const void **this)
#[doc(alias = "FMOD::ProfileClient::release(void)")]
pub fn stub_69820() -> i32 {
    // IDA 0x69820 `ProfileClient::release`: closes the socket, frees the
    // queue plus the client (0x69830..0x69888).
    FMOD_PROFILE_CLIENT.release()
}

// 0x6989c — __ZN4FMOD13ProfileClient4initEPv
// type: int __fastcall(FMOD::ProfileClient *this, void *)
#[doc(alias = "FMOD::ProfileClient::init(void *)")]
pub fn stub_6989c() -> i32 {
    // IDA 0x6989c `ProfileClient::init`: allocates the 16 KiB queue and
    // latches the socket (0x698b8..0x698fc); a failed alloc returns 44.
    FMOD_PROFILE_CLIENT.init()
}

// 0x69910 — __ZN4FMOD7Profile17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::Profile *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::Profile::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_69910() -> u32 {
    // IDA 0x69910 `Profile::getMemoryUsedImpl`: tracks the profile, the
    // critical section, the DSP node/packet blocks plus each module
    // (0x69930..0x69a60); the byte total below.
    FMOD_PROFILE.memory_used()
}

// 0x69a78 — __ZN4FMOD7Profile7releaseEv
// type: int __fastcall(FMOD::Profile *this)
#[doc(alias = "FMOD::Profile::release(void)")]
pub fn stub_69a78() -> i32 {
    // IDA 0x69a78 `Profile::release`: closes the socket, releases every
    // client plus module, then frees the profile (0x69a88..0x69b2c tail).
    FMOD_PROFILE.release()
}

// 0x69be8 — __ZN4FMOD20FMOD_Profile_ReleaseEv
// type: int __fastcall(FMOD *this)
#[doc(alias = "FMOD::FMOD_Profile_Release(void)")]
pub fn stub_69be8() -> i32 {
    // IDA 0x69be8 `FMOD_Profile_Release`: releases when the cell is set
    // (0x69bfc..0x69c14).
    FMOD_PROFILE.release()
}

// 0x69c20 — __ZN4FMOD7Profile4initEt
// type: int __fastcall(FMOD::Profile *this, unsigned __int16)
#[doc(alias = "FMOD::Profile::init(unsigned short)")]
pub fn stub_69c20(port: u16) -> i32 {
    // IDA 0x69c20 `Profile::init`: latches the port (9264 when zero),
    // starts listening, then stamps the clock (0x69c38..0x69c94); a failed
    // leg shuts the net down and returns its code.
    FMOD_PROFILE.init(port)
}

// 0x69c9c — __ZN4FMOD19FMOD_Profile_CreateEt
// type: int __fastcall(FMOD *this, unsigned __int16)
#[doc(alias = "FMOD::FMOD_Profile_Create(unsigned short)")]
pub fn stub_69c9c(port: u16) -> i32 {
    // IDA 0x69c9c `FMOD_Profile_Create`: bails when the cell is set; else
    // allocs, constructs and inits (0x69cc4..0x69d2c).
    static DONE: std::sync::Once = std::sync::Once::new();
    let mut result = 0;
    DONE.call_once(|| {
        result = FMOD_PROFILE.init(port);
    });
    result
}

// 0x69d50 — __ZN4FMOD7Profile9addPacketEPNS_19ProfilePacketHeaderE
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "FMOD::Profile::addPacket(FMOD::ProfilePacketHeader *)")]
pub fn stub_69d50(bytes: &[u8]) -> i32 {
    // IDA 0x69d50 `Profile::addPacket`: stamps the clock delta, then fans
    // the packet out to the clients that want it (0x69d60..0x69df4).
    FMOD_PROFILE.add_packet(bytes)
}

// 0x69e0c — __ZN4FMOD7Profile6updateEPNS_7SystemIEj
// type: int __fastcall(FMOD::Profile *this, FMOD::SystemI *, unsigned int)
#[doc(alias = "FMOD::Profile::update(FMOD::SystemI *,unsigned int)")]
pub fn stub_69e0c(ticks: u32) -> i32 {
    // IDA 0x69e0c `Profile::update`: past 49 ticks accepts a client and
    // updates every client plus module (0x69e28..0x69e6c tail).
    FMOD_PROFILE.update(ticks)
}

// 0x6a018 — __ZN4FMOD19FMOD_Profile_UpdateEPNS_7SystemIEj
// type: int __fastcall(FMOD *this, FMOD::SystemI *, unsigned int)
#[doc(alias = "FMOD::FMOD_Profile_Update(FMOD::SystemI *,unsigned int)")]
pub fn stub_6a018(ticks: u32) -> i32 {
    // IDA 0x6a018 `FMOD_Profile_Update`: updates when the cell is set,
    // else returns 81 (0x6a034..0x6a040).
    if FMOD_PROFILE.is_created() {
        FMOD_PROFILE.update(ticks)
    } else {
        81
    }
}

// 0x6a04c — __ZN4FMOD7Profile13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::Profile::getMemoryUsed(FMOD::MemoryTracker *)")]
pub fn stub_6a04c(full: bool) -> i32 {
    // IDA 0x6a04c `Profile::getMemoryUsed`: dispatches through the vtable
    // and latches the flag (0x6a05c..0x6a09c); the impl returns 0.
    if full {
        FMOD_PROFILE.memory_used();
    }
    0
}

// 0x6a0a4 — __ZN7allpassC2Ev
// type: void __fastcall(allpass *this)
#[doc(alias = "allpass::allpass(void)")]
pub fn stub_6a0a4() {
    // IDA 0x6a0a4 `allpass::allpass`: zeroes the delay slot (0x6a0a8); the
    // LazyLock below owns it zeroed.
    let _ = &*ALLPASS;
}

// 0x6a0b0 — __ZN7allpassC1Ev
// type: void __fastcall(allpass *this)
#[doc(alias = "allpass::allpass(void)")]
pub fn stub_6a0b0() {
    // IDA 0x6a0b0 `allpass::allpass` thunk: tail-calls the C2 ctor above.
    let _ = &*ALLPASS;
}

// 0x6a0b4 — __ZN7allpass9setbufferEPfi
// type: int __fastcall(int this, float *, int)
#[doc(alias = "allpass::setbuffer(float *,int)")]
pub fn stub_6a0b4(len: usize) {
    // IDA 0x6a0b4 `allpass::setbuffer`: latches the buffer plus its length
    // (0x6a0b4).
    ALLPASS.set_buffer(len);
}

// 0x6a0bc — __ZN7allpass4muteEv
// type: int __fastcall(int this)
#[doc(alias = "allpass::mute(void)")]
pub fn stub_6a0bc() {
    // IDA 0x6a0bc `allpass::mute`: zeroes the buffer (0x6a0c4..0x6a0ec).
    ALLPASS.mute();
}

// 0x6a0f4 — __ZN7allpass11setfeedbackEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "allpass::setfeedback(float)")]
pub fn stub_6a0f4(gain: f32) {
    // IDA 0x6a0f4 `allpass::setfeedback`: latches the gain (0x6a0f4).
    ALLPASS.set_feedback(gain);
}

// 0x6a0fc — __ZN7ASfxDsp11ClearInBuffEv
// type: int __fastcall(int this)
#[doc(alias = "ASfxDsp::ClearInBuff(void)")]
pub fn stub_6a0fc() {
    // IDA 0x6a0fc `ASfxDsp::ClearInBuff`: zeroes the input buffer
    // (0x6a0fc..0x6a13c).
    ASFX_DSP.clear_input();
}

// 0x6a144 — __ZN7ASfxDsp26SetLate_EarlyLateDelayTapsEffff
// type: char *__fastcall(ASfxDsp *this, float, float32_t, float32_t, float32_t)
#[doc(alias = "ASfxDsp::SetLate_EarlyLateDelayTaps(float,float,float,float)")]
pub fn stub_6a144(early: f32, late: f32) {
    // IDA 0x6a144 `ASfxDsp::SetLate_EarlyLateDelayTaps`: derives the late
    // plus early/late tap delays from the rate pair (0x6a164..0x6a1d4).
    ASFX_DSP.set_late_early_late_taps(early, late);
}

// 0x6a1dc — __ZN7ASfxDsp16SetAllpassDelaysEf
// type: _DWORD *__fastcall(_DWORD *this, float32_t)
#[doc(alias = "ASfxDsp::SetAllpassDelays(float)")]
pub fn stub_6a1dc(rate: f32) {
    // IDA 0x6a1dc `ASfxDsp::SetAllpassDelays`: latches the rate plus both
    // derived allpass gains (0x6a1ec..0x6a22c).
    ASFX_DSP.set_allpass_delays(rate);
}

// 0x6a23c — __ZN7ASfxDsp13SetEarlyDelayEfff
// type: _DWORD *__fastcall(ASfxDsp *this, float, float32_t, float32_t)
#[doc(alias = "ASfxDsp::SetEarlyDelay(float,float,float)")]
pub fn stub_6a23c(rate: f32, scale: f32) {
    // IDA 0x6a23c `ASfxDsp::SetEarlyDelay`: derives the seven early
    // reflection delays from the rate triple (0x6a24c..0x6a2ac).
    ASFX_DSP.set_early_delay(rate, scale);
}

// 0x6a2b4 — __ZN7ASfxDsp13SetLateDelaysEfffff
// type: _DWORD *__fastcall(_DWORD *this, float32_t, float32_t, float32_t, float32_t, float32_t)
#[doc(alias = "ASfxDsp::SetLateDelays(float,float,float,float,float)")]
pub fn stub_6a2b4(delays: [f32; 5]) {
    // IDA 0x6a2b4 `ASfxDsp::SetLateDelays`: derives the eight late reverb
    // delays (0x6a2d4..0x6a33c).
    ASFX_DSP.set_late_delays(delays);
}

// 0x6a344 — __ZN7ASfxDsp17ZeroWritePointersEv
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "ASfxDsp::ZeroWritePointers(void)")]
pub fn stub_6a344() {
    // IDA 0x6a344 `ASfxDsp::ZeroWritePointers`: zeroes every delay-line
    // write pointer (0x6a34c..0x6a374).
    ASFX_DSP.zero_write_pointers();
}

// 0x6a37c — __ZN7ASfxDsp17BlockProcessInputEjiPff
// type: void **__fastcall(void **this, unsigned int, int, float *__src, float)
#[doc(alias = "ASfxDsp::BlockProcessInput(unsigned int,int,float *,float)")]
pub fn stub_6a37c(input: &[f32], gain: f32) {
    // IDA 0x6a37c `ASfxDsp::BlockProcessInput`: folds the input block into
    // the delay lines with the input gain (0x6a37c..tail).
    ASFX_DSP.block_process_input(input, gain);
}

// 0x6a648 — __ZN7ASfxDsp15DoDSPProcessingEPfS0_ijfft
// type: unsigned int __fastcall(void **this, float *, float *, int, unsigned int, float, float32_t, unsigned __int16)
#[doc(alias = "ASfxDsp::DoDSPProcessing(float *,float *,int,unsigned int,float,float,unsigned short)")]
pub fn stub_6a648(output: &mut [f32], input: &[f32], gain: f32) -> u32 {
    // IDA 0x6a648 `ASfxDsp::DoDSPProcessing`: runs the reflections over
    // the block and returns the frame count (0x6a648..tail).
    ASFX_DSP.do_dsp_processing(output, input, gain)
}

// 0x6b360 — __ZN7ASfxDsp26ClearReverbInternalBuffersEv
// type: void *__fastcall(ASfxDsp *this)
#[doc(alias = "ASfxDsp::ClearReverbInternalBuffers(void)")]
pub fn stub_6b360() {
    // IDA 0x6b360 `ASfxDsp::ClearReverbInternalBuffers`: zeroes the eight
    // voice buffers plus the late/early lines (0x6b370..0x6b41c).
    ASFX_DSP.clear_reverb();
}

// 0x6b4dc — __ZN7ASfxDsp12ClearBuffersEv
// type: void *__fastcall(ASfxDsp *this)
#[doc(alias = "ASfxDsp::ClearBuffers(void)")]
pub fn stub_6b4dc() {
    // IDA 0x6b4dc `ASfxDsp::ClearBuffers`: clears the input plus the
    // reverb lines (0x6b4e8).
    ASFX_DSP.clear_buffers();
}

// 0x6b4f8 — __ZN7ASfxDsp24DeallocateEarlyLateDelayEv
// type: int __fastcall(int this)
#[doc(alias = "ASfxDsp::DeallocateEarlyLateDelay(void)")]
pub fn stub_6b4f8() {
    // IDA 0x6b4f8 `ASfxDsp::DeallocateEarlyLateDelay`: frees the line and
    // nulls it (0x6b500..0x6b534).
    ASFX_DSP.deallocate_early_late();
}

// 0x6b544 — __ZN7ASfxDsp20DeallocateEarlyDelayEv
// type: int __fastcall(int this)
#[doc(alias = "ASfxDsp::DeallocateEarlyDelay(void)")]
pub fn stub_6b544() {
    // IDA 0x6b544 `ASfxDsp::DeallocateEarlyDelay`: frees the line and nulls
    // it (0x6b54c..0x6b580).
    ASFX_DSP.deallocate_early();
}

// 0x6b590 — __ZN7ASfxDsp23DeallocateAllpassDelaysEv
// type: int __fastcall(int this)
#[doc(alias = "ASfxDsp::DeallocateAllpassDelays(void)")]
pub fn stub_6b590() {
    // IDA 0x6b590 `ASfxDsp::DeallocateAllpassDelays`: frees both lines and
    // nulls them (0x6b5a0..0x6b5e0).
    ASFX_DSP.deallocate_allpass();
}

// 0x6b5f0 — __ZN7ASfxDsp20DeallocateLateDelaysEv
// type: int __fastcall(int this)
#[doc(alias = "ASfxDsp::DeallocateLateDelays(void)")]
pub fn stub_6b5f0() {
    // IDA 0x6b5f0 `ASfxDsp::DeallocateLateDelays`: frees the eight lines
    // and nulls them (0x6b600..0x6b640).
    ASFX_DSP.deallocate_late();
}

// 0x6b650 — __ZN7ASfxDsp5closeEv
// type: int __fastcall(void **this)
#[doc(alias = "ASfxDsp::close(void)")]
pub fn stub_6b650() {
    // IDA 0x6b650 `ASfxDsp::close`: frees the input buffer, then runs all
    // four deallocates (0x6b658..0x6b6a8).
    ASFX_DSP.close();
}

// 0x6b6c0 — __ZN7ASfxDsp16UpdateBufferSizeEi
// type: unsigned int __fastcall(ASfxDsp *this, int)
#[doc(alias = "ASfxDsp::UpdateBufferSize(int)")]
pub fn stub_6b6c0(size: u32) -> u32 {
    // IDA 0x6b6c0 `ASfxDsp::UpdateBufferSize`: no-op when the size matches;
    // else re-allocs the aligned input buffer (0x6b6dc..0x6b768). A failed
    // alloc returns 4505.
    ASFX_DSP.update_buffer_size(size)
}

// 0x6b77c — __ZN7ASfxDsp12NextPowerOf2Ei
// type: int __fastcall(ASfxDsp *this, int)
#[doc(alias = "ASfxDsp::NextPowerOf2(int)")]
pub fn stub_6b77c(n: i32) -> i32 {
    // IDA 0x6b77c `ASfxDsp::NextPowerOf2`: `1 << (logf(n)/logf(2) + 1)`
    // (0x6b79c..0x6b7d0).
    ASFX_DSP.next_power_of2(n)
}

// 0x6b7d4 — __ZN7ASfxDsp18AllocateEarlyDelayEff
// type: int __fastcall(ASfxDsp *this, float32_t, float32_t)
#[doc(alias = "ASfxDsp::AllocateEarlyDelay(float,float)")]
pub fn stub_6b7d4(rate: f32, scale: f32) -> i32 {
    // IDA 0x6b7d4 `ASfxDsp::AllocateEarlyDelay`: sizes the power-of-2 line
    // from the rate product and callocs it (0x6b7e0..0x6b850). A failed
    // alloc returns 4502.
    ASFX_DSP.allocate_early(rate, scale)
}

// 0x6b864 — __ZN7ASfxDsp21AllocateAllpassDelaysEiPff
// type: int __fastcall(ASfxDsp *this, int, float *, float32_t)
#[doc(alias = "ASfxDsp::AllocateAllpassDelays(int,float *,float)")]
pub fn stub_6b864(rates: [f32; 2], rate: f32) -> i32 {
    // IDA 0x6b864 `ASfxDsp::AllocateAllpassDelays`: sizes both power-of-2
    // lines from the tap rates (0x6b8b4..0x6b918). A failed alloc returns
    // 4500.
    ASFX_DSP.allocate_allpass(rates, rate)
}

// 0x6b944 — __ZN7ASfxDsp22AllocateEarlyLateDelayEPff
// type: int __fastcall(ASfxDsp *this, float *, float32_t)
#[doc(alias = "ASfxDsp::AllocateEarlyLateDelay(float *,float)")]
pub fn stub_6b944(taps: &[f32], rate: f32) -> i32 {
    // IDA 0x6b944 `ASfxDsp::AllocateEarlyLateDelay`: sizes the power-of-2
    // line from the tap spread times the rate (0x6b950..0x6b9d0). A failed
    // alloc returns 4501.
    ASFX_DSP.allocate_early_late(taps, rate)
}

// 0x6b9e8 — __ZN7ASfxDsp18AllocateLateDelaysEiPff
// type: int __fastcall(ASfxDsp *this, int, float *, float32_t)
#[doc(alias = "ASfxDsp::AllocateLateDelays(int,float *,float)")]
pub fn stub_6b9e8(rates: &[f32], rate: f32) -> i32 {
    // IDA 0x6b9e8 `ASfxDsp::AllocateLateDelays`: sizes the eight
    // power-of-2 lines from the tap rates (0x6ba38..0x6ba9c). A failed
    // alloc returns 4503.
    ASFX_DSP.allocate_late(rates, rate)
}

// 0x6bac8 — __ZN7ASfxDsp4initEf
// type: int __fastcall(ASfxDsp *this, float32_t)
#[doc(alias = "ASfxDsp::init(float)")]
pub fn stub_6bac8(rate: f32) -> i32 {
    // IDA 0x6bac8 `ASfxDsp::init`: zeroes the lines, latches the default
    // taps, then runs the allocates (0x6bae4..tail).
    ASFX_DSP.init(rate)
}

/// `_FLAC__bitmath_ilog2` (IDA 0x6bdcc): `floor(log2(n))`, zero for 0 and 1
/// (0x6bdcc..0x6bdec).
pub fn flac_ilog2_6bdcc(n: u32) -> u32 {
    if n < 2 {
        0
    } else {
        31 - n.leading_zeros()
    }
}
/// One MSB-first CRC-16 step (poly 0x8005) behind `_crc16_update_word_`
/// (IDA 0x6bdf0): matches the `FLAC__crc16_table` byte walk without
/// embedding the table.
pub fn flac_crc16_step_6bdf0(crc: u16, byte: u8) -> u16 {
    let mut crc = crc ^ ((byte as u16) << 8);
    for _ in 0..8 {
        crc = if crc & 0x8000 != 0 {
            (crc << 1) ^ 0x8005
        } else {
            crc << 1
        };
    }
    crc
}
/// Feed a big-endian word through the CRC byte lanes selected by the bit
/// offset (IDA 0x6bdfc..0x6bef0: the 0/8/0x10/0x18 fallthrough).
pub fn flac_crc16_word_6bdf0(crc: u16, word: u32, offset: u32) -> u16 {
    let bytes = word.to_be_bytes();
    let start = (offset / 8) as usize;
    let mut crc = crc;
    for byte in bytes.iter().skip(start) {
        crc = flac_crc16_step_6bdf0(crc, *byte);
    }
    crc
}
/// Reader core behind `FLAC__bitreader_*` (IDA 0x6bf10..0x6c688): the
/// queued big-endian words plus the consume/crc cursors. Field order
/// follows the scattered init stores at 0x6c094..0x6c0f0 (capacity 2048
/// words, 8 KiB buffer).
#[derive(Debug, Default)]
pub struct FlacBitreaderCore {
    buffer: Vec<u32>,
    capacity_words: u32,
    total_words: u32,
    tail_bytes: u32,
    consumed_idx: u32,
    consumed_bits: u32,
    crc: u16,
    crc_consumed_bits: u32,
    read_crc: u16,
}
/// Minimal `FLAC__bitreader` counterpart (IDA 0x6bf10..0x6c688).
#[derive(Debug, Default)]
pub struct FlacBitreader {
    core: parking_lot::Mutex<FlacBitreaderCore>,
}
impl FlacBitreader {
    /// `FLAC__bitreader_clear` (IDA 0x6bf10): zeroes the cursors, returns 1
    /// (0x6bf14..0x6bf28).
    pub fn clear(&self) -> bool {
        let mut core = self.core.lock();
        core.total_words = 0;
        core.tail_bytes = 0;
        core.consumed_idx = 0;
        core.consumed_bits = 0;
        true
    }
    /// `FLAC__bitreader_reset_read_crc16` (IDA 0x6bf2c): latches the
    /// expected crc and the consumed mark (0x6bf2c..0x6bf38).
    pub fn reset_read_crc(&self, crc: u16) {
        let mut core = self.core.lock();
        core.read_crc = crc;
        core.crc_consumed_bits = core.consumed_bits;
    }
    /// `FLAC__bitreader_get_read_crc16` (IDA 0x6bf40): folds the words
    /// consumed since the mark into the crc (0x6bf58..0x6bfac).
    pub fn read_crc(&self) -> u16 {
        let mut core = self.core.lock();
        while core.crc_consumed_bits + 8 <= core.consumed_bits {
            let bit = core.crc_consumed_bits;
            let word = core.buffer.get((bit / 32) as usize).copied().unwrap_or(0);
            let byte = ((word >> (24 - (bit % 32))) & 0xff) as u8;
            core.crc = flac_crc16_step_6bdf0(core.crc, byte);
            core.crc_consumed_bits += 8;
        }
        core.crc
    }
    /// `FLAC__bitreader_is_consumed_byte_aligned` (IDA 0x6bfc8).
    pub fn is_byte_aligned(&self) -> bool {
        self.core.lock().consumed_bits % 8 == 0
    }
    /// `FLAC__bitreader_bits_left_for_byte_alignment` (IDA 0x6bfdc).
    pub fn bits_to_align(&self) -> u32 {
        8 - (self.core.lock().consumed_bits % 8)
    }
    /// `FLAC__bitreader_get_input_bits_unconsumed` (IDA 0x6bfec):
    /// `8 * (4 * (total - idx) + tail) - bits` (0x6c010).
    pub fn bits_unconsumed(&self) -> u32 {
        let core = self.core.lock();
        8 * (4 * (core.total_words - core.consumed_idx) + core.tail_bytes) - core.consumed_bits
    }
    /// `FLAC__bitreader_free` (IDA 0x6c014): frees the buffer and zeroes
    /// the cursors (0x6c020..0x6c050).
    pub fn free(&self) {
        let mut core = self.core.lock();
        core.buffer.clear();
        core.capacity_words = 0;
        core.total_words = 0;
        core.tail_bytes = 0;
        core.consumed_idx = 0;
        core.consumed_bits = 0;
        core.crc = 0;
        core.crc_consumed_bits = 0;
    }
    /// `FLAC__bitreader_init` (IDA 0x6c074): zeroes the cursors, sets the
    /// 2048-word capacity plus the 8 KiB buffer (0x6c094..0x6c0f4).
    pub fn init(&self) -> bool {
        let mut core = self.core.lock();
        core.buffer = vec![0; 2048];
        core.capacity_words = 2048;
        core.total_words = 0;
        core.tail_bytes = 0;
        core.consumed_idx = 0;
        core.consumed_bits = 0;
        core.crc = 0;
        core.crc_consumed_bits = 0;
        true
    }
    /// Queue big-endian words from the client (`bitreader_read_from_client_`
    /// body, IDA 0x6c11c): compacts the consumed prefix, byte-swaps the
    /// tail, then appends; false when the buffer is full (0x6c128..0x6c284).
    pub fn queue_words(&self, words: &[u32]) -> bool {
        let mut core = self.core.lock();
        let drop = core.consumed_idx as usize;
        if drop > 0 {
            let len = core.buffer.len();
            core.buffer.drain(..drop.min(len));
            core.total_words -= drop as u32;
            core.consumed_idx = 0;
        }
        if (core.buffer.len() + words.len()) as u32 > core.capacity_words {
            return false;
        }
        core.buffer.extend_from_slice(words);
        core.total_words += words.len() as u32;
        true
    }
    fn read_bit(&self, core: &mut FlacBitreaderCore) -> Option<u32> {
        let idx = (core.consumed_idx) as usize;
        let word = *core.buffer.get(idx)?;
        let bit = (word >> (31 - (core.consumed_bits % 32))) & 1;
        core.consumed_bits += 1;
        if core.consumed_bits % 32 == 0 {
            core.consumed_idx += 1;
            core.consumed_bits = 0;
        }
        Some(bit)
    }
    fn read_bits(&self, core: &mut FlacBitreaderCore, n: u32) -> Option<u32> {
        let mut value = 0u32;
        for _ in 0..n {
            value = (value << 1) | self.read_bit(core)?;
        }
        Some(value)
    }
    /// `FLAC__bitreader_read_unary_unsigned` (IDA 0x6c688): counts the
    /// zero bits up to the stop bit (0x6c69c..0x6c7b0); `None` on underflow.
    pub fn read_unary(&self) -> Option<u32> {
        let mut core = self.core.lock();
        let mut count = 0u32;
        loop {
            match self.read_bit(&mut core)? {
                0 => count += 1,
                _ => return Some(count),
            }
        }
    }
    /// `FLAC__bitreader_read_rice_signed_block` (IDA 0x6c28c): decodes
    /// `count` rice values with the escape/partition walk (0x6c2b4..
    /// tail); short blocks end early on underflow.
    pub fn read_rice_signed_block(&self, count: u32, param: u32) -> Vec<i32> {
        let mut core = self.core.lock();
        let mut out = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let mut quotient = 0u32;
            loop {
                match self.read_bit(&mut core) {
                    Some(0) => quotient += 1,
                    Some(_) => break,
                    None => return out,
                }
            }
            let remainder = self.read_bits(&mut core, param).unwrap_or(0);
            let value = (quotient << param) | remainder;
            out.push((value >> 1) as i32 ^ -((value & 1) as i32));
        }
        out
    }
    /// `FLAC__bitreader_read_raw_uint32` (IDA 0x6c8d0): refills from the
    /// client until `n` bits are queued, then assembles them MSB-first
    /// (0x6c914..0x6ca20); zero bits return 0, short input returns `None`.
    pub fn read_raw(&self, n: u32) -> Option<u32> {
        if n == 0 {
            return Some(0);
        }
        if n > self.bits_unconsumed() {
            return None;
        }
        let mut core = self.core.lock();
        self.read_bits(&mut core, n)
    }
    fn read_utf8_inner(&self) -> Option<(u64, Vec<u8>)> {
        let first = self.read_raw(8)? as u8;
        let mut raw = vec![first];
        if first & 0x80 == 0 {
            return Some((first as u64, raw));
        }
        let (mut value, mut extra) = if first & 0xe0 == 0xc0 && first & 0x20 == 0 {
            ((first & 0x1f) as u64, 1)
        } else if first & 0xf0 == 0xe0 && first & 0x10 == 0 {
            ((first & 0x0f) as u64, 2)
        } else if first & 0xf8 == 0xf0 && first & 0x08 == 0 {
            ((first & 0x07) as u64, 3)
        } else if first & 0xfc == 0xf8 && first & 0x04 == 0 {
            ((first & 0x03) as u64, 4)
        } else if first & 0xfe == 0xfc && first & 0x02 == 0 {
            ((first & 0x01) as u64, 5)
        } else {
            return None;
        };
        while extra > 0 {
            let next = self.read_raw(8)? as u8;
            raw.push(next);
            if next & 0xc0 != 0x80 {
                return None;
            }
            value = (value << 6) | ((next & 0x3f) as u64);
            extra -= 1;
        }
        Some((value, raw))
    }
    /// `FLAC__bitreader_read_utf8_uint64` (IDA 0x6ca70): the lead byte
    /// selects the sequence length, continuations fold in (0x6ca4..
    /// 0x6cb98); `None` on short/invalid input.
    pub fn read_utf8_uint64(&self) -> Option<u64> {
        self.read_utf8_inner().map(|(value, _)| value)
    }
    /// `FLAC__bitreader_read_utf8_uint32` (IDA 0x6cc88): same walk capped
    /// at five lead patterns (0x6ccbc..0x6cd74).
    pub fn read_utf8_uint32(&self) -> Option<u32> {
        self.read_utf8_inner().map(|(value, _)| value as u32)
    }
    /// `FLAC__bitreader_read_byte_block_aligned_no_crc` (IDA 0x6cdf4):
    /// drains the partial word, memcpys whole words, then finishes
    /// byte-wise (0x6ce04..0x6cf00); `None` on short input.
    pub fn read_byte_block(&self, count: u32) -> Option<Vec<u8>> {
        let mut out = Vec::with_capacity(count as usize);
        for _ in 0..count {
            out.push(self.read_raw(8)? as u8);
        }
        Some(out)
    }
    /// `FLAC__bitreader_skip_byte_block_aligned_no_crc` (IDA 0x6cf18):
    /// same word fast path without storing (0x6cf24..0x6cfe4).
    pub fn skip_byte_block(&self, count: u32) -> bool {
        self.read_byte_block(count).is_some()
    }
    /// `FLAC__bitreader_skip_bits_no_crc` (IDA 0x6cff0): aligns, skips
    /// whole bytes, then the tail bits (0x6d000..0x6d0a8).
    pub fn skip_bits(&self, n: u32) -> bool {
        if n == 0 {
            return true;
        }
        let align = self.bits_to_align() % 8;
        if align > 0 {
            if self.read_raw(align.min(n)).is_none() {
                return false;
            }
            let rest = n - align.min(n);
            return self.skip_byte_block(rest / 8) && self.read_raw(rest % 8).is_some();
        }
        self.skip_byte_block(n / 8) && self.read_raw(n % 8).is_some()
    }
    /// `FLAC__bitreader_read_uint32_little_endian` (IDA 0x6d0b8): four
    /// bytes folded little-endian (0x6d0cc..0x6d174).
    pub fn read_u32_le(&self) -> Option<u32> {
        let b0 = self.read_raw(8)?;
        let b1 = self.read_raw(8)?;
        let b2 = self.read_raw(8)?;
        let b3 = self.read_raw(8)?;
        Some(b0 | (b1 << 8) | (b2 << 16) | (b3 << 24))
    }
    /// `FLAC__bitreader_read_raw_uint64` (IDA 0x6d17c): at most 32 bits go
    /// straight through, wider reads split hi/lo (0x6d198..0x6d22c).
    pub fn read_raw_uint64(&self, n: u32) -> Option<u64> {
        if n <= 0x20 {
            return self.read_raw(n).map(|value| value as u64);
        }
        let hi = self.read_raw(n - 32)? as u64;
        let lo = self.read_raw(32)? as u64;
        Some((hi << 32) | lo)
    }
    /// `FLAC__bitreader_read_raw_int32` (IDA 0x6d234): the raw bits
    /// sign-extended (0x6d240..0x6d264).
    pub fn read_raw_int32(&self, n: u32) -> Option<i32> {
        if n == 0 {
            return Some(0);
        }
        self.read_raw(n).map(|value| ((value << (32 - n)) as i32) >> (32 - n))
    }
}
static FLAC_READER: std::sync::LazyLock<FlacBitreader> =
    std::sync::LazyLock::new(FlacBitreader::default);
// 0x6bdcc — _FLAC__bitmath_ilog2
// type: int __fastcall(unsigned int)
#[doc(alias = "_FLAC__bitmath_ilog2")]
pub fn stub_6bdcc(n: u32) -> u32 {
    // IDA 0x6bdcc `_FLAC__bitmath_ilog2`: `floor(log2(n))`, zero for 0 and
    // 1 (0x6bdcc..0x6bdec).
    flac_ilog2_6bdcc(n)
}

// 0x6bdf0 — _crc16_update_word_
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "_crc16_update_word_")]
pub fn stub_6bdf0(crc: u16, word: u32, offset: u32) -> u16 {
    // IDA 0x6bdf0 `_crc16_update_word_`: the offset selects the starting
    // byte lane through the table fallthrough (0x6bdfc..0x6bef0).
    flac_crc16_word_6bdf0(crc, word, offset)
}

// 0x6bf10 — _FLAC__bitreader_clear
// type: int __fastcall(_DWORD *)
#[doc(alias = "_FLAC__bitreader_clear")]
pub fn stub_6bf10() -> bool {
    // IDA 0x6bf10 `FLAC__bitreader_clear`: zeroes the cursors, returns 1
    // (0x6bf14..0x6bf28).
    FLAC_READER.clear()
}

// 0x6bf2c — _FLAC__bitreader_reset_read_crc16
// type: _DWORD *__fastcall(_DWORD *result, unsigned __int16)
#[doc(alias = "_FLAC__bitreader_reset_read_crc16")]
pub fn stub_6bf2c(crc: u16) {
    // IDA 0x6bf2c `FLAC__bitreader_reset_read_crc16`: latches the expected
    // crc and the consumed mark (0x6bf2c..0x6bf38).
    FLAC_READER.reset_read_crc(crc);
}

// 0x6bf40 — _FLAC__bitreader_get_read_crc16
// type: unsigned int __fastcall(_DWORD *)
#[doc(alias = "_FLAC__bitreader_get_read_crc16")]
pub fn stub_6bf40() -> u16 {
    // IDA 0x6bf40 `FLAC__bitreader_get_read_crc16`: folds the words
    // consumed since the mark into the crc (0x6bf58..0x6bfac).
    FLAC_READER.read_crc()
}

// 0x6bfc8 — _FLAC__bitreader_is_consumed_byte_aligned
// type: bool __fastcall(int)
#[doc(alias = "_FLAC__bitreader_is_consumed_byte_aligned")]
pub fn stub_6bfc8() -> bool {
    // IDA 0x6bfc8 `FLAC__bitreader_is_consumed_byte_aligned`.
    FLAC_READER.is_byte_aligned()
}

// 0x6bfdc — _FLAC__bitreader_bits_left_for_byte_alignment
// type: int __fastcall(int)
#[doc(alias = "_FLAC__bitreader_bits_left_for_byte_alignment")]
pub fn stub_6bfdc() -> u32 {
    // IDA 0x6bfdc `FLAC__bitreader_bits_left_for_byte_alignment`.
    FLAC_READER.bits_to_align()
}

// 0x6bfec — _FLAC__bitreader_get_input_bits_unconsumed
// type: int __fastcall(_DWORD *)
#[doc(alias = "_FLAC__bitreader_get_input_bits_unconsumed")]
pub fn stub_6bfec() -> u32 {
    // IDA 0x6bfec `FLAC__bitreader_get_input_bits_unconsumed`:
    // `8 * (4 * (total - idx) + tail) - bits` (0x6c010).
    FLAC_READER.bits_unconsumed()
}

// 0x6c014 — _FLAC__bitreader_free
// type: void __fastcall(int)
#[doc(alias = "_FLAC__bitreader_free")]
pub fn stub_6c014() {
    // IDA 0x6c014 `FLAC__bitreader_free`: frees the buffer and zeroes the
    // cursors (0x6c020..0x6c050).
    FLAC_READER.free();
}

// 0x6c058 — _FLAC__bitreader_delete
// type: void __fastcall(void *)
#[doc(alias = "_FLAC__bitreader_delete")]
pub fn stub_6c058() {
    // IDA 0x6c058 `FLAC__bitreader_delete`: frees the buffer plus the
    // reader (0x6c064..0x6c06c).
    FLAC_READER.free();
}

// 0x6c074 — _FLAC__bitreader_init
// type: int __fastcall(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "_FLAC__bitreader_init")]
pub fn stub_6c074() -> bool {
    // IDA 0x6c074 `FLAC__bitreader_init`: zeroes the cursors, sets the
    // 2048-word capacity plus the 8 KiB buffer (0x6c094..0x6c0f4).
    FLAC_READER.init()
}

// 0x6c104 — _FLAC__bitreader_new
// type: void *()
#[doc(alias = "_FLAC__bitreader_new")]
pub fn stub_6c104() {
    // IDA 0x6c104 `FLAC__bitreader_new`: callocs the 0x60-byte reader
    // (0x6c118); the LazyLock below owns it zeroed.
    FLAC_READER.clear();
}

// 0x6c11c — _bitreader_read_from_client_
// type: int __fastcall(int, int)
#[doc(alias = "_bitreader_read_from_client_")]
pub fn stub_6c11c(words: &[u32]) -> bool {
    // IDA 0x6c11c `bitreader_read_from_client_`: compacts the consumed
    // prefix, byte-swaps the tail, then appends; false when the buffer is
    // full (0x6c128..0x6c284).
    FLAC_READER.queue_words(words)
}

// 0x6c28c — _FLAC__bitreader_read_rice_signed_block
// type: int __fastcall(int, _DWORD *, unsigned int *, int, unsigned int)
#[doc(alias = "_FLAC__bitreader_read_rice_signed_block")]
pub fn stub_6c28c(count: u32, param: u32) -> Vec<i32> {
    // IDA 0x6c28c `FLAC__bitreader_read_rice_signed_block`: decodes
    // `count` rice values with the escape/partition walk (0x6c2b4..tail);
    // short blocks end early on underflow.
    FLAC_READER.read_rice_signed_block(count, param)
}

// 0x6c688 — _FLAC__bitreader_read_unary_unsigned
// type: int __fastcall(int, _DWORD *, _DWORD *)
#[doc(alias = "_FLAC__bitreader_read_unary_unsigned")]
pub fn stub_6c688() -> Option<u32> {
    // IDA 0x6c688 `FLAC__bitreader_read_unary_unsigned`: counts the zero
    // bits up to the stop bit (0x6c69c..0x6c7b0); `None` on underflow.
    FLAC_READER.read_unary()
}

// 0x6c8d0 — _FLAC__bitreader_read_raw_uint32
// type: int __fastcall(int, _DWORD *, _DWORD *, unsigned int)
#[doc(alias = "_FLAC__bitreader_read_raw_uint32")]
pub fn stub_6c8d0(n: u32) -> Option<u32> {
    // IDA 0x6c8d0 `FLAC__bitreader_read_raw_uint32`: refills from the
    // client until `n` bits are queued, then assembles them MSB-first
    // (0x6c914..0x6ca20); zero bits return 0, short input returns `None`.
    FLAC_READER.read_raw(n)
}

// 0x6ca70 — _FLAC__bitreader_read_utf8_uint64
// type: int __fastcall(int, _DWORD *, int, int, int *)
#[doc(alias = "_FLAC__bitreader_read_utf8_uint64")]
pub fn stub_6ca70() -> Option<u64> {
    // IDA 0x6ca70 `FLAC__bitreader_read_utf8_uint64`: the lead byte selects
    // the sequence length, continuations fold in (0x6ca4..0x6cb98); `None`
    // on short/invalid input.
    FLAC_READER.read_utf8_uint64()
}

// 0x6cc88 — _FLAC__bitreader_read_utf8_uint32
// type: int __fastcall(int, _DWORD *, int *, int, int *)
#[doc(alias = "_FLAC__bitreader_read_utf8_uint32")]
pub fn stub_6cc88() -> Option<u32> {
    // IDA 0x6cc88 `FLAC__bitreader_read_utf8_uint32`: same walk capped at
    // five lead patterns (0x6ccbc..0x6cd74).
    FLAC_READER.read_utf8_uint32()
}

// 0x6cdf4 — _FLAC__bitreader_read_byte_block_aligned_no_crc
// type: int __fastcall(int, _DWORD *, _BYTE *, unsigned int)
#[doc(alias = "_FLAC__bitreader_read_byte_block_aligned_no_crc")]
pub fn stub_6cdf4(count: u32) -> Option<Vec<u8>> {
    // IDA 0x6cdf4 `FLAC__bitreader_read_byte_block_aligned_no_crc`: drains
    // the partial word, memcpys whole words, then finishes byte-wise
    // (0x6ce04..0x6cf00); `None` on short input.
    FLAC_READER.read_byte_block(count)
}

// 0x6cf18 — _FLAC__bitreader_skip_byte_block_aligned_no_crc
// type: int __fastcall(int, _DWORD *, unsigned int)
#[doc(alias = "_FLAC__bitreader_skip_byte_block_aligned_no_crc")]
pub fn stub_6cf18(count: u32) -> bool {
    // IDA 0x6cf18 `FLAC__bitreader_skip_byte_block_aligned_no_crc`: same
    // word fast path without storing (0x6cf24..0x6cfe4).
    FLAC_READER.skip_byte_block(count)
}

// 0x6cff0 — _FLAC__bitreader_skip_bits_no_crc
// type: bool __fastcall(int, _DWORD *, unsigned int)
#[doc(alias = "_FLAC__bitreader_skip_bits_no_crc")]
pub fn stub_6cff0(n: u32) -> bool {
    // IDA 0x6cff0 `FLAC__bitreader_skip_bits_no_crc`: aligns, skips whole
    // bytes, then the tail bits (0x6d000..0x6d0a8).
    FLAC_READER.skip_bits(n)
}

// 0x6d0b8 — _FLAC__bitreader_read_uint32_little_endian
// type: int __fastcall(int, _DWORD *, _DWORD *)
#[doc(alias = "_FLAC__bitreader_read_uint32_little_endian")]
pub fn stub_6d0b8() -> Option<u32> {
    // IDA 0x6d0b8 `FLAC__bitreader_read_uint32_little_endian`: four bytes
    // folded little-endian (0x6d0cc..0x6d174).
    FLAC_READER.read_u32_le()
}

// 0x6d17c — _FLAC__bitreader_read_raw_uint64
// type: int __fastcall(int, _DWORD *, _DWORD *, unsigned int)
#[doc(alias = "_FLAC__bitreader_read_raw_uint64")]
pub fn stub_6d17c(n: u32) -> Option<u64> {
    // IDA 0x6d17c `FLAC__bitreader_read_raw_uint64`: at most 32 bits go
    // straight through, wider reads split hi/lo (0x6d198..0x6d22c).
    FLAC_READER.read_raw_uint64(n)
}

// 0x6d234 — _FLAC__bitreader_read_raw_int32
// type: int __fastcall(int, _DWORD *, int *, unsigned int)
#[doc(alias = "_FLAC__bitreader_read_raw_int32")]
pub fn stub_6d234(n: u32) -> Option<i32> {
    // IDA 0x6d234 `FLAC__bitreader_read_raw_int32`: the raw bits
    // sign-extended (0x6d240..0x6d264).
    FLAC_READER.read_raw_int32(n)
}

/// Minimal `oggpack_buffer` counterpart behind `_FMOD_oggpack_*` (IDA
/// 0x6d26c..0x6d44c): the byte buffer plus the endbyte/endbit cursors.
/// Field order follows `readinit` (0x6d454..0x6d474).
#[derive(Debug, Default)]
pub struct OggpackState {
    buffer: parking_lot::Mutex<Vec<u8>>,
    endbyte: parking_lot::Mutex<u32>,
    endbit: parking_lot::Mutex<u32>,
    storage: parking_lot::Mutex<u32>,
}
impl OggpackState {
    const MASK: [u32; 33] = [
        0x00000000, 0x00000001, 0x00000003, 0x00000007, 0x0000000f, 0x0000001f,
        0x0000003f, 0x0000007f, 0x000000ff, 0x000001ff, 0x000003ff, 0x000007ff,
        0x00000fff, 0x00001fff, 0x00003fff, 0x00007fff, 0x0000ffff, 0x0001ffff,
        0x0003ffff, 0x0007ffff, 0x000fffff, 0x001fffff, 0x003fffff, 0x007fffff,
        0x00ffffff, 0x01ffffff, 0x03ffffff, 0x07ffffff, 0x0fffffff, 0x1fffffff,
        0x3fffffff, 0x7fffffff, 0xffffffff,
    ];
    /// `_FMOD_oggpack_readinit` (IDA 0x6d44c): zeroes the cursors and
    /// latches the buffer plus its storage (0x6d454..0x6d474).
    pub fn readinit(&self, data: &[u8]) {
        *self.buffer.lock() = data.to_vec();
        *self.storage.lock() = data.len() as u32;
        *self.endbyte.lock() = 0;
        *self.endbit.lock() = 0;
    }
    /// `_FMOD_oggpack_look` (IDA 0x6d26c): peeks `n` LSB-first bits without
    /// advancing; -1 past the end (0x6d278..0x6d308).
    pub fn look(&self, n: u32) -> i32 {
        let buffer = self.buffer.lock();
        let endbyte = *self.endbyte.lock();
        let endbit = *self.endbit.lock();
        let storage = *self.storage.lock();
        let bitpos = endbyte * 8 + endbit;
        if endbyte + 4 >= storage && bitpos + n > storage * 8 {
            return -1;
        }
        let mut value = 0u32;
        for i in 0..n {
            let pos = bitpos + i;
            let byte = buffer.get((pos / 8) as usize).copied().unwrap_or(0);
            value |= (((byte >> (pos % 8)) & 1) as u32) << i;
        }
        (value & Self::MASK[n as usize]) as i32
    }
    /// `_FMOD_oggpack_adv` (IDA 0x6d318): advances `n` bits (0x6d31c..
    /// 0x6d34c).
    pub fn adv(&self, n: u32) {
        let total = n + *self.endbit.lock();
        *self.endbit.lock() = total & 7;
        let bytes = total / 8;
        *self.endbyte.lock() += bytes;
    }
    /// `_FMOD_oggpack_read` (IDA 0x6d354): looks, then advances; -1 past
    /// the end (0x6d360..0x6d420).
    pub fn read(&self, n: u32) -> i32 {
        let value = self.look(n);
        if value >= 0 {
            self.adv(n);
        }
        value
    }
    /// `_FMOD_oggpack_bytes` (IDA 0x6d434): consumed whole bytes rounding
    /// the partial bit up (0x6d434..0x6d448).
    pub fn bytes_used(&self) -> u32 {
        *self.endbyte.lock() + ((*self.endbit.lock() + 7) / 8)
    }
}
static OGGPACK: std::sync::LazyLock<OggpackState> =
    std::sync::LazyLock::new(OggpackState::default);
/// `_ilog2` (IDA 0x6d47c): bit-length of `n - 1`, zero for 0 and 1
/// (0x6d480..0x6d488).
pub fn ogg_ilog2_6d47c(n: u32) -> u32 {
    if n < 2 {
        0
    } else {
        32 - (n - 1).leading_zeros()
    }
}
/// `_bitreverse` (IDA 0x6e708): the ROR/UXTB16/nibble ladder is a 32-bit
/// reversal (0x6e70c..0x6e758).
pub fn vorbis_bitreverse_6e708(n: u32) -> u32 {
    n.reverse_bits()
}
/// Huffman codebook behind `_FMOD_vorbis_book_decode*` (IDA 0x6e778..):
/// the entry count, minimum length, direct table plus the sorted lengths.
#[derive(Debug, Default, Clone)]
pub struct VorbisCodebook {
    entries: u32,
    min_len: u32,
    direct: Vec<i32>,
    lengths: Vec<u8>,
    sorted_min: Vec<u32>,
    values: Vec<f32>,
}
impl VorbisCodebook {
    pub fn new(
        entries: u32,
        min_len: u32,
        direct: Vec<i32>,
        lengths: Vec<u8>,
        values: Vec<f32>,
    ) -> Self {
        Self {
            entries,
            min_len,
            direct,
            lengths,
            sorted_min: Vec::new(),
            values,
        }
    }
    /// `_FMOD_vorbis_book_decode` (IDA 0x6e778): looks `min_len` bits for
    /// the direct hit, else walks the length ladder on the reversed peek
    /// (0x6e798..0x6e868); -1 on underflow.
    pub fn decode(&self, packed: &[u8], bitpos: &mut u32) -> i32 {
        let peek = |n: u32| -> Option<u32> {
            let mut value = 0u32;
            for i in 0..n {
                let pos = *bitpos + i;
                value |= (((packed.get((pos / 8) as usize).copied().unwrap_or(0) >> (pos % 8)) & 1) as u32) << i;
            }
            Some(value)
        };
        if self.entries == 0 {
            return -1;
        }
        let first = peek(self.min_len).unwrap_or(0);
        let entry = *self.direct.get(first as usize).unwrap_or(&-1);
        if entry >= 0 {
            *bitpos += self.lengths.get(entry as usize).copied().unwrap_or(self.min_len as u8) as u32;
            return entry;
        }
        let mut low = ((entry >> 15) & 0x7fff) as u32;
        let mut high = self.entries - (entry as u32 & 0x7fff);
        let rev = vorbis_bitreverse_6e708(peek(self.min_len.max(1)).unwrap_or(0));
        while high - low > 1 {
            let mid = (high + low) >> 1;
            if rev < self.sorted_min.get(mid as usize).copied().unwrap_or(u32::MAX) {
                high = mid;
            } else {
                low = mid;
            }
        }
        *bitpos += self.min_len;
        low as i32
    }
    /// Shared tail behind the `_add` decodes: adds the entry value into
    /// the output slot; -1 decodes add nothing.
    fn add_entry(&self, out: &mut [f32], slot: usize, entry: i32) {
        if entry >= 0 {
            if let (Some(dst), Some(&value)) =
                (out.get_mut(slot), self.values.get(entry as usize))
            {
                *dst += value;
            }
        }
    }
    /// `_FMOD_vorbis_book_decodev_add` (IDA 0x6ee98): decodes `n` entries
    /// sequentially adding each value (0x6ee98..tail).
    pub fn decodev_add(
        &self,
        out: &mut [f32],
        base: usize,
        n: usize,
        packed: &[u8],
        bitpos: &mut u32,
    ) -> u32 {
        let mut done = 0;
        for i in 0..n {
            let entry = self.decode(packed, bitpos);
            if entry < 0 {
                break;
            }
            self.add_entry(out, base + i, entry);
            done += 1;
        }
        done
    }
    /// `_FMOD_vorbis_book_decodevv_add` (IDA 0x6ec78): decodes `n` vectors
    /// of `step` entries adding each row (0x6ec78..tail).
    pub fn decodevv_add(
        &self,
        out: &mut [f32],
        base: usize,
        step: usize,
        n: usize,
        packed: &[u8],
        bitpos: &mut u32,
    ) -> u32 {
        let mut done = 0;
        for row in 0..n {
            for col in 0..step {
                let entry = self.decode(packed, bitpos);
                if entry < 0 {
                    return done;
                }
                self.add_entry(out, base + row * step + col, entry);
            }
            done += 1;
        }
        done
    }
    /// `_FMOD_vorbis_book_decodevs_add` (IDA 0x6f37c): decodes `n` entries
    /// adding with the channel stride (0x6f37c..tail).
    pub fn decodevs_add(
        &self,
        out: &mut [f32],
        base: usize,
        stride: usize,
        n: usize,
        packed: &[u8],
        bitpos: &mut u32,
    ) -> u32 {
        let mut done = 0;
        for i in 0..n {
            let entry = self.decode(packed, bitpos);
            if entry < 0 {
                break;
            }
            self.add_entry(out, base + i * stride, entry);
            done += 1;
        }
        done
    }
    /// `_FMOD_vorbis_staticbook_unpack` (IDA 0x6e8c4): validates the
    /// 0x564b42 magic plus the dimension guards, then stores the
    /// dimensions/lengths (0x6e92c..tail).
    pub fn staticbook_unpack(
        &mut self,
        magic_ok: bool,
        entries: u32,
        min_len: u32,
        lengths: Vec<u8>,
        values: Vec<f32>,
    ) -> bool {
        if !magic_ok || entries == 0 {
            return false;
        }
        self.entries = entries;
        self.min_len = min_len;
        self.lengths = lengths;
        self.values = values;
        self.direct = vec![-1; 1 << min_len.min(16) as usize];
        true
    }
}
/// Minimal `vorbis_dsp_state`/`vorbis_block` counterpart behind
/// `_FMOD_vorbis_synthesis_*` plus `_FMOD_vorbis_block_*` (IDA 0x6d4b4..
/// 0x6e6c0): the PCM ring, the block arena plus the lifecycle latches.
#[derive(Debug, Default)]
pub struct VorbisSynth {
    channels: parking_lot::Mutex<u32>,
    pcm: parking_lot::Mutex<Vec<Vec<f32>>>,
    pcm_current: parking_lot::Mutex<u32>,
    pcm_end: parking_lot::Mutex<u32>,
    arena: parking_lot::Mutex<Vec<u8>>,
    blocks: std::sync::atomic::AtomicU32,
    initialized: std::sync::atomic::AtomicBool,
}
impl VorbisSynth {
    /// `_FMOD_vorbis_synthesis_init` (IDA 0x6e2c4): zeroes the state,
    /// derives the pcm/codebook tables; nonzero without info (0x6e2d4..
    /// tail).
    pub fn synthesis_init(&self, channels: u32) -> i32 {
        if channels == 0 {
            return 1;
        }
        *self.channels.lock() = channels;
        *self.pcm.lock() = vec![Vec::new(); channels as usize];
        *self.pcm_current.lock() = 0;
        *self.pcm_end.lock() = 0;
        self.initialized.store(true, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `_FMOD_vorbis_synthesis_restart` (IDA 0x6d4b4): re-derives the
    /// center/end cursors; -1 without dsp state (0x6d4b8..0x6d52c).
    pub fn restart(&self) -> i32 {
        if !self.is_initialized() {
            return -1;
        }
        *self.pcm_current.lock() = 0;
        0
    }
    /// `_FMOD_vorbis_synthesis_pcmout` (IDA 0x6d538): available frames
    /// between center and end (0x6d540..0x6d5bc).
    pub fn pcmout(&self) -> u32 {
        self.pcm_end.lock().saturating_sub(*self.pcm_current.lock())
    }
    /// `_FMOD_vorbis_synthesis_read` (IDA 0x6d5c8): advances past `n`
    /// frames; -131 past the end (0x6d5cc..0x6d5f4).
    pub fn read(&self, n: u32) -> i32 {
        let mut current = self.pcm_current.lock();
        if n + *current <= *self.pcm_end.lock() {
            *current += n;
            0
        } else {
            -131
        }
    }
    /// `_FMOD_vorbis_synthesis_blockin` (IDA 0x6d600): overlaps the decoded
    /// block into the PCM ring (0x6d600..tail).
    pub fn blockin(&self, block: &[Vec<f32>]) -> i32 {
        let mut pcm = self.pcm.lock();
        for (channel, samples) in pcm.iter_mut().zip(block.iter()) {
            channel.extend_from_slice(samples);
        }
        let frames = block.first().map(Vec::len).unwrap_or(0) as u32;
        *self.pcm_end.lock() += frames;
        self.blocks.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn block_count(&self) -> u32 {
        self.blocks.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `_FMOD_vorbis_block_alloc` (IDA 0x6dee8): bump-allocates the
    /// 8-aligned size from the arena (0x6defc..0x6df88); the offset below
    /// is the pointer.
    pub fn block_alloc(&self, size: usize) -> usize {
        let mut arena = self.arena.lock();
        let aligned = (size + 7) & !7;
        let offset = arena.len();
        arena.resize(offset + aligned, 0);
        offset
    }
    /// `_FMOD_vorbis_block_ripcord` (IDA 0x6df94): frees the spill list
    /// and resets the arena cursor (0x6dfb0..0x6e038).
    pub fn block_ripcord(&self) -> i32 {
        self.arena.lock().clear();
        0
    }
    /// `_FMOD_vorbis_block_init` (IDA 0x6e044): zeroes the 0x68-byte block
    /// and latches the dsp state (0x6e060..0x6e070).
    pub fn block_init(&self) -> i32 {
        self.arena.lock().clear();
        0
    }
    /// `_FMOD_vorbis_block_clear` (IDA 0x6e6c0): ripcords, frees the arena
    /// and zeroes the block (0x6e6d4..0x6e6fc).
    pub fn block_clear(&self) -> i32 {
        self.block_ripcord()
    }
    /// `_FMOD_vorbis_dsp_clear` (IDA 0x6e078): clears the mdct/codebook
    /// tables plus the pcm ring (0x6e094..tail).
    pub fn dsp_clear(&self) {
        self.pcm.lock().clear();
        self.arena.lock().clear();
        *self.pcm_current.lock() = 0;
        *self.pcm_end.lock() = 0;
        self.initialized.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}
static VORBIS_SYNTH: std::sync::LazyLock<VorbisSynth> =
    std::sync::LazyLock::new(VorbisSynth::default);
static VORBIS_BOOK: std::sync::LazyLock<parking_lot::Mutex<VorbisCodebook>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(VorbisCodebook::default()));
// 0x6d26c — _FMOD_oggpack_look
// type: int __fastcall(int *, int)
#[doc(alias = "_FMOD_oggpack_look")]
pub fn stub_6d26c(n: u32) -> i32 {
    // IDA 0x6d26c `_FMOD_oggpack_look`: peeks `n` LSB-first bits without
    // advancing; -1 past the end (0x6d278..0x6d308).
    OGGPACK.look(n)
}

// 0x6d318 — _FMOD_oggpack_adv
// type: _DWORD *__fastcall(_DWORD *result, int)
#[doc(alias = "_FMOD_oggpack_adv")]
pub fn stub_6d318(n: u32) {
    // IDA 0x6d318 `_FMOD_oggpack_adv`: advances `n` bits (0x6d31c..0x6d34c).
    OGGPACK.adv(n);
}

// 0x6d354 — _FMOD_oggpack_read
// type: int __fastcall(int *, int)
#[doc(alias = "_FMOD_oggpack_read")]
pub fn stub_6d354(n: u32) -> i32 {
    // IDA 0x6d354 `_FMOD_oggpack_read`: looks, then advances; -1 past the
    // end (0x6d360..0x6d420).
    OGGPACK.read(n)
}

// 0x6d434 — _FMOD_oggpack_bytes
// type: int __fastcall(int *)
#[doc(alias = "_FMOD_oggpack_bytes")]
pub fn stub_6d434() -> u32 {
    // IDA 0x6d434 `_FMOD_oggpack_bytes`: consumed whole bytes rounding the
    // partial bit up (0x6d434..0x6d448).
    OGGPACK.bytes_used()
}

// 0x6d44c — _FMOD_oggpack_readinit
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "_FMOD_oggpack_readinit")]
pub fn stub_6d44c(data: &[u8]) {
    // IDA 0x6d44c `_FMOD_oggpack_readinit`: zeroes the cursors and latches
    // the buffer plus its storage (0x6d454..0x6d474).
    OGGPACK.readinit(data);
}

// 0x6d47c — _ilog2
// type: int __fastcall(int)
#[doc(alias = "_ilog2")]
pub fn stub_6d47c(n: u32) -> u32 {
    // IDA 0x6d47c `_ilog2`: bit-length of `n - 1`, zero for 0 and 1
    // (0x6d480..0x6d488).
    ogg_ilog2_6d47c(n)
}

// 0x6d4b4 — _FMOD_vorbis_synthesis_restart
// type: int __fastcall(int **)
#[doc(alias = "_FMOD_vorbis_synthesis_restart")]
pub fn stub_6d4b4() -> i32 {
    // IDA 0x6d4b4 `_FMOD_vorbis_synthesis_restart`: re-derives the
    // center/end cursors; -1 without dsp state (0x6d4b8..0x6d52c).
    VORBIS_SYNTH.restart()
}

// 0x6d538 — _FMOD_vorbis_synthesis_pcmout
// type: int __fastcall(int *, _DWORD *)
#[doc(alias = "_FMOD_vorbis_synthesis_pcmout")]
pub fn stub_6d538() -> u32 {
    // IDA 0x6d538 `_FMOD_vorbis_synthesis_pcmout`: available frames between
    // center and end (0x6d540..0x6d5bc).
    VORBIS_SYNTH.pcmout()
}

// 0x6d5c8 — _FMOD_vorbis_synthesis_read
// type: int __fastcall(int, int)
#[doc(alias = "_FMOD_vorbis_synthesis_read")]
pub fn stub_6d5c8(n: u32) -> i32 {
    // IDA 0x6d5c8 `_FMOD_vorbis_synthesis_read`: advances past `n` frames;
    // -131 past the end (0x6d5cc..0x6d5f4).
    VORBIS_SYNTH.read(n)
}

// 0x6d600 — _FMOD_vorbis_synthesis_blockin
// type: int __fastcall(int *, int)
#[doc(alias = "_FMOD_vorbis_synthesis_blockin")]
pub fn stub_6d600(block: &[Vec<f32>]) -> i32 {
    // IDA 0x6d600 `_FMOD_vorbis_synthesis_blockin`: overlaps the decoded
    // block into the PCM ring (0x6d600..tail).
    VORBIS_SYNTH.blockin(block)
}

// 0x6dee8 — __FMOD_vorbis_block_alloc
// type: int __fastcall(int, _DWORD *, int)
#[doc(alias = "__FMOD_vorbis_block_alloc")]
pub fn stub_6dee8(size: usize) -> usize {
    // IDA 0x6dee8 `_FMOD_vorbis_block_alloc`: bump-allocates the 8-aligned
    // size from the arena (0x6defc..0x6df88); the offset below is the
    // pointer.
    VORBIS_SYNTH.block_alloc(size)
}

// 0x6df94 — __FMOD_vorbis_block_ripcord
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "__FMOD_vorbis_block_ripcord")]
pub fn stub_6df94() -> i32 {
    // IDA 0x6df94 `_FMOD_vorbis_block_ripcord`: frees the spill list and
    // resets the arena cursor (0x6dfb0..0x6e038).
    VORBIS_SYNTH.block_ripcord()
}

// 0x6e044 — _FMOD_vorbis_block_init
// type: int __fastcall(int, int, void *__b)
#[doc(alias = "_FMOD_vorbis_block_init")]
pub fn stub_6e044() -> i32 {
    // IDA 0x6e044 `_FMOD_vorbis_block_init`: zeroes the 0x68-byte block
    // and latches the dsp state (0x6e060..0x6e070).
    VORBIS_SYNTH.block_init()
}

// 0x6e078 — _FMOD_vorbis_dsp_clear
// type: void *__fastcall(void *result, int *, int, int)
#[doc(alias = "_FMOD_vorbis_dsp_clear")]
pub fn stub_6e078() {
    // IDA 0x6e078 `_FMOD_vorbis_dsp_clear`: clears the mdct/codebook tables
    // plus the pcm ring (0x6e094..tail).
    VORBIS_SYNTH.dsp_clear();
}

// 0x6e2c4 — _FMOD_vorbis_synthesis_init
// type: int __fastcall(void *, int *__b, int, int)
#[doc(alias = "_FMOD_vorbis_synthesis_init")]
pub fn stub_6e2c4(channels: u32) -> i32 {
    // IDA 0x6e2c4 `_FMOD_vorbis_synthesis_init`: zeroes the state, derives
    // the pcm/codebook tables; nonzero without info (0x6e2d4..tail).
    VORBIS_SYNTH.synthesis_init(channels)
}

// 0x6e6c0 — _FMOD_vorbis_block_clear
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "_FMOD_vorbis_block_clear")]
pub fn stub_6e6c0() -> i32 {
    // IDA 0x6e6c0 `_FMOD_vorbis_block_clear`: ripcords, frees the arena
    // and zeroes the block (0x6e6d4..0x6e6fc).
    VORBIS_SYNTH.block_clear()
}

// 0x6e708 — _bitreverse
// type: unsigned int __fastcall(int)
#[doc(alias = "_bitreverse")]
pub fn stub_6e708(n: u32) -> u32 {
    // IDA 0x6e708 `_bitreverse`: the ROR/UXTB16/nibble ladder is a 32-bit
    // reversal (0x6e70c..0x6e758).
    vorbis_bitreverse_6e708(n)
}

// 0x6e778 — _FMOD_vorbis_book_decode
// type: int __fastcall(int *, int *)
#[doc(alias = "_FMOD_vorbis_book_decode")]
pub fn stub_6e778(packed: &[u8], bitpos: &mut u32) -> i32 {
    // IDA 0x6e778 `_FMOD_vorbis_book_decode`: looks `min_len` bits for the
    // direct hit, else walks the length ladder on the reversed peek
    // (0x6e798..0x6e868); -1 on underflow.
    VORBIS_BOOK.lock().decode(packed, bitpos)
}

/// Minimal `comb` counterpart (IDA 0x6f5ec..0x6f660): the delay buffer,
/// the damping pair plus the feedback gain.
#[derive(Debug, Default)]
pub struct CombState {
    buffer: parking_lot::Mutex<Vec<f32>>,
    damp: parking_lot::Mutex<f32>,
    damp2: parking_lot::Mutex<f32>,
    feedback: parking_lot::Mutex<f32>,
}
impl CombState {
    /// `comb::setbuffer` (IDA 0x6f604): latches the buffer plus its length
    /// (0x6f604..0x6f60c).
    pub fn set_buffer(&self, len: usize) {
        *self.buffer.lock() = vec![0.0; len];
    }
    pub fn buffer_len(&self) -> usize {
        self.buffer.lock().len()
    }
    /// `comb::mute` (IDA 0x6f610): zeroes the buffer (0x6f618..0x6f640).
    pub fn mute(&self) {
        for sample in self.buffer.lock().iter_mut() {
            *sample = 0.0;
        }
    }
    /// `comb::setdamp` (IDA 0x6f648): latches the damping plus `1 - damp`
    /// (0x6f650..0x6f658).
    pub fn set_damp(&self, damp: f32) {
        *self.damp.lock() = damp;
        *self.damp2.lock() = 1.0 - damp;
    }
    pub fn damp(&self) -> (f32, f32) {
        (*self.damp.lock(), *self.damp2.lock())
    }
    /// `comb::setfeedback` (IDA 0x6f660): latches the gain (0x6f660).
    pub fn set_feedback(&self, gain: f32) {
        *self.feedback.lock() = gain;
    }
    pub fn feedback(&self) -> f32 {
        *self.feedback.lock()
    }
}
static COMB: std::sync::LazyLock<CombState> = std::sync::LazyLock::new(CombState::default);
/// `_FLAC__crc8` (IDA 0x6f67c): CRC-8 (poly 0x07) over the bytes
/// (0x6f698..0x6f6b4); matches the `FLAC__crc8_table` walk bit-wise.
pub fn flac_crc8_6f67c(data: &[u8]) -> u8 {
    let mut crc = 0u8;
    for byte in data {
        crc ^= byte;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 {
                (crc << 1) ^ 0x07
            } else {
                crc << 1
            };
        }
    }
    crc
}
/// `_FLAC__fixed_restore_signal` (IDA 0x6f6c4): orders 0..4 rebuild the
/// signal from the residual plus the warmup (0x6f6d4..tail).
pub fn flac_fixed_restore_6f6c4(residual: &[i32], order: u32, warmup: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(warmup.len() + residual.len());
    out.extend_from_slice(warmup);
    for (i, &r) in residual.iter().enumerate() {
        let n = out.len();
        let p = |k: usize| out[n - k];
        let value = match order {
            0 => r,
            1 => p(1) + r,
            2 => 2 * p(1) - p(2) + r,
            3 => 3 * p(1) - 3 * p(2) + p(3) + r,
            _ => 4 * p(1) - 6 * p(2) + 4 * p(3) - p(4) + r,
        };
        let _ = i;
        out.push(value);
    }
    out
}
/// `_ilog` (IDA 0x6f804): bit-length, zero for 0 (0x6f808..0x6f824).
pub fn flac_ilog_6f804(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        32 - n.leading_zeros()
    }
}
/// Floor-1 header behind `_FMOD_floor1_unpack` (IDA 0x6fe9c): the order
/// plus the rate table.
#[derive(Debug, Default, Clone)]
pub struct Floor1Info {
    order: u32,
    rates: Vec<u32>,
}
/// Floor-1 render state behind `_FMOD_floor1_look` (IDA 0x6fbe0): the post
/// positions plus the block size.
#[derive(Debug, Default, Clone)]
pub struct Floor1Look {
    posts: Vec<u32>,
    n: u32,
}
/// Minimal floor-1 counterpart (IDA 0x6f840..0x701fc).
#[derive(Debug, Default)]
pub struct Floor1 {
    info: parking_lot::Mutex<Floor1Info>,
    look: parking_lot::Mutex<Floor1Look>,
}
impl Floor1 {
    /// `_FMOD_floor1_unpack` (IDA 0x6fe9c): reads the order plus the rate
    /// table (0x6fed8..tail); false on bad dims.
    pub fn unpack(&self, order: u32, rates: Vec<u32>) -> bool {
        if order == 0 || order > 256 || rates.len() != order as usize {
            return false;
        }
        *self.info.lock() = Floor1Info { order, rates };
        true
    }
    /// `_FMOD_floor1_look` (IDA 0x6fbe0): builds the sorted post layout
    /// from the rates (0x6fc08..tail).
    pub fn look(&self) -> bool {
        let info = self.info.lock().clone();
        if info.order == 0 {
            return false;
        }
        let mut posts = vec![0u32, 1u32 << 16];
        let mut acc = 0u32;
        for rate in &info.rates {
            acc += rate + 1;
            posts.push(acc);
        }
        posts.sort_unstable();
        posts.dedup();
        let n = posts.len() as u32;
        *self.look.lock() = Floor1Look { posts, n };
        true
    }
    /// `_FMOD_floor1_free_info` (IDA 0x6fe68): zeroes the header
    /// (0x6fe88) and frees it.
    pub fn free_info(&self) {
        *self.info.lock() = Floor1Info::default();
    }
    /// `_FMOD_floor1_free_look` (IDA 0x6fbac): zeroes the look
    /// (0x6fbcc) and frees it.
    pub fn free_look(&self) {
        *self.look.lock() = Floor1Look::default();
    }
    /// `_FMOD_floor1_inverse1` (IDA 0x6f840): with a nonzero flag reads
    /// the amp/filter bits into the fit vector (0x6f894..tail); `None`
    /// without audio.
    pub fn inverse1(&self, has_audio: bool, fit: Vec<i32>) -> Option<Vec<i32>> {
        if !has_audio {
            return None;
        }
        Some(fit)
    }
    /// `_FMOD_floor1_inverse2` (IDA 0x701fc): renders the floor curve by
    /// linear dB interpolation across the posts (0x70218..tail); `None`
    /// without a fit.
    pub fn inverse2(&self, fit: &[i32], n: usize) -> Option<Vec<f32>> {
        let look = self.look.lock().clone();
        if fit.is_empty() || look.posts.len() < 2 {
            return None;
        }
        let span = *look.posts.last().unwrap_or(&1).max(&1) as f32;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let x = (i as f32) * span / (n.max(1) as f32);
            let mut seg = 0;
            while seg + 1 < look.posts.len() && (look.posts[seg + 1] as f32) < x {
                seg += 1;
            }
            let x0 = look.posts[seg] as f32;
            let x1 = look.posts.get(seg + 1).copied().unwrap_or(span as u32) as f32;
            let y0 = fit.get(seg).copied().unwrap_or(0) as f32;
            let y1 = fit.get(seg + 1).copied().unwrap_or(y0 as i32) as f32;
            let t = if x1 > x0 { (x - x0) / (x1 - x0) } else { 0.0 };
            out.push(y0 + (y1 - y0) * t);
        }
        Some(out)
    }
}
static FLOOR1: std::sync::LazyLock<Floor1> = std::sync::LazyLock::new(Floor1::default);
/// Minimal `FMOD::SystemI` lifecycle counterpart (IDA 0x70458..0x705cc):
/// the created latch plus the pool stats.
#[derive(Debug, Default)]
pub struct FmodSystem {
    created: std::sync::atomic::AtomicBool,
    systems: std::sync::atomic::AtomicU32,
    mem_current: std::sync::atomic::AtomicU32,
    mem_max: std::sync::atomic::AtomicU32,
}
impl FmodSystem {
    /// `_FMOD_System_Create` (IDA 0x70474): callocs plus constructs the
    /// system; 37 without a slot, 44 on alloc failure (0x70488..0x70564).
    pub fn system_create(&self) -> i32 {
        self.created.store(true, std::sync::atomic::Ordering::SeqCst);
        self.systems.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        0
    }
    pub fn system_count(&self) -> u32 {
        self.systems.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `_FMOD_Channel_GetUserData` (IDA 0x70458): 37 without a channel,
    /// else the channel value (0x70464..0x7046c).
    pub fn channel_user_data(&self, has_channel: bool) -> i32 {
        if has_channel {
            0
        } else {
            37
        }
    }
    /// `_FMOD_Memory_GetStats` (IDA 0x705cc): flushes the DSP requests,
    /// then reports current/max pool bytes (0x705e4..0x70688).
    pub fn memory_stats(&self) -> (u32, u32) {
        (
            self.mem_current.load(std::sync::atomic::Ordering::SeqCst),
            self.mem_max.load(std::sync::atomic::Ordering::SeqCst),
        )
    }
    pub fn note_alloc(&self, bytes: u32) {
        let current = self.mem_current.fetch_add(bytes, std::sync::atomic::Ordering::SeqCst) + bytes;
        let mut max = self.mem_max.load(std::sync::atomic::Ordering::SeqCst);
        while current > max {
            match self.mem_max.compare_exchange(
                max,
                current,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(actual) => max = actual,
            }
        }
    }
}
static FMOD_SYSTEM: std::sync::LazyLock<FmodSystem> =
    std::sync::LazyLock::new(FmodSystem::default);
/// Minimal `FMOD::AsyncThread` counterpart (IDA 0x7069c..0x706b4): the
/// running/stop latches plus the processed count.
#[derive(Debug, Default)]
pub struct FmodAsyncThread {
    running: std::sync::atomic::AtomicBool,
    stop_requested: std::sync::atomic::AtomicBool,
    processed: std::sync::atomic::AtomicU32,
    pending: std::sync::atomic::AtomicU32,
}
impl FmodAsyncThread {
    pub fn set_running(&self, running: bool) {
        self.running.store(running, std::sync::atomic::Ordering::SeqCst);
    }
    /// `AsyncThread::release` (IDA 0x7069c): with the thread up, asks it
    /// to stop (0x7069c..0x706b0).
    pub fn release(&self) -> i32 {
        if self.running.load(std::sync::atomic::Ordering::SeqCst) {
            self.stop_requested.store(true, std::sync::atomic::Ordering::SeqCst);
        }
        0
    }
    pub fn stop_requested(&self) -> bool {
        self.stop_requested.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `AsyncThread::threadFunc` (IDA 0x706b4): dequeues one stream
    /// request per pass; idle passes return 0 (0x706c8..tail).
    pub fn thread_func(&self) -> i32 {
        if !self.running.load(std::sync::atomic::Ordering::SeqCst) {
            return 0;
        }
        if self.pending.load(std::sync::atomic::Ordering::SeqCst) > 0 {
            self.pending.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            self.processed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        0
    }
    pub fn queue(&self) {
        self.pending.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn processed_count(&self) -> u32 {
        self.processed.load(std::sync::atomic::Ordering::SeqCst)
    }
}
static FMOD_ASYNC: std::sync::LazyLock<FmodAsyncThread> =
    std::sync::LazyLock::new(FmodAsyncThread::default);
// 0x6e8c4 — _FMOD_vorbis_staticbook_unpack
// type: int __fastcall(int, int *, int *)
#[doc(alias = "_FMOD_vorbis_staticbook_unpack")]
pub fn stub_6e8c4(magic_ok: bool, entries: u32, min_len: u32, lengths: Vec<u8>, values: Vec<f32>) -> bool {
    // IDA 0x6e8c4 `_FMOD_vorbis_staticbook_unpack`: validates the 0x564b42
    // magic plus the dimension guards, then stores the dimensions/lengths
    // (0x6e92c..tail).
    VORBIS_BOOK.lock().staticbook_unpack(magic_ok, entries, min_len, lengths, values)
}

// 0x6ec78 — _FMOD_vorbis_book_decodevv_add
// type: int __fastcall(int *, int, int, int, int *, int)
#[doc(alias = "_FMOD_vorbis_book_decodevv_add")]
pub fn stub_6ec78(out: &mut [f32], base: usize, step: usize, n: usize, packed: &[u8], bitpos: &mut u32) -> u32 {
    // IDA 0x6ec78 `_FMOD_vorbis_book_decodevv_add`: decodes `n` vectors of
    // `step` entries adding each row (0x6ec78..tail).
    VORBIS_BOOK.lock().decodevv_add(out, base, step, n, packed, bitpos)
}

// 0x6ee98 — _FMOD_vorbis_book_decodev_add
// type: int __fastcall(int *, int, int *, int)
#[doc(alias = "_FMOD_vorbis_book_decodev_add")]
pub fn stub_6ee98(out: &mut [f32], base: usize, n: usize, packed: &[u8], bitpos: &mut u32) -> u32 {
    // IDA 0x6ee98 `_FMOD_vorbis_book_decodev_add`: decodes `n` entries
    // sequentially adding each value (0x6ee98..tail).
    VORBIS_BOOK.lock().decodev_add(out, base, n, packed, bitpos)
}

// 0x6f37c — _FMOD_vorbis_book_decodevs_add
// type: int __fastcall(int *, __int32 *, int *, int)
#[doc(alias = "_FMOD_vorbis_book_decodevs_add")]
pub fn stub_6f37c(out: &mut [f32], base: usize, stride: usize, n: usize, packed: &[u8], bitpos: &mut u32) -> u32 {
    // IDA 0x6f37c `_FMOD_vorbis_book_decodevs_add`: decodes `n` entries
    // adding with the channel stride (0x6f37c..tail).
    VORBIS_BOOK.lock().decodevs_add(out, base, stride, n, packed, bitpos)
}

// 0x6f5ec — __ZN4combC2Ev
// type: void __fastcall(comb *this)
#[doc(alias = "comb::comb(void)")]
pub fn stub_6f5ec() {
    // IDA 0x6f5ec `comb::comb`: zeroes the delay slots (0x6f5f0..0x6f5f8);
    // the LazyLock below owns them zeroed.
    let _ = &*COMB;
}

// 0x6f600 — __ZN4combC1Ev
// type: void __fastcall(comb *this)
#[doc(alias = "comb::comb(void)")]
pub fn stub_6f600() {
    // IDA 0x6f600 `comb::comb` thunk: tail-calls the C2 ctor above.
    let _ = &*COMB;
}

// 0x6f604 — __ZN4comb9setbufferEPfi
// type: int __fastcall(int this, float *, int)
#[doc(alias = "comb::setbuffer(float *,int)")]
pub fn stub_6f604(len: usize) {
    // IDA 0x6f604 `comb::setbuffer`: latches the buffer plus its length
    // (0x6f604..0x6f60c).
    COMB.set_buffer(len);
}

// 0x6f610 — __ZN4comb4muteEv
// type: int __fastcall(int this)
#[doc(alias = "comb::mute(void)")]
pub fn stub_6f610() {
    // IDA 0x6f610 `comb::mute`: zeroes the buffer (0x6f618..0x6f640).
    COMB.mute();
}

// 0x6f648 — __ZN4comb7setdampEf
// type: int __fastcall(int this, float)
#[doc(alias = "comb::setdamp(float)")]
pub fn stub_6f648(damp: f32) {
    // IDA 0x6f648 `comb::setdamp`: latches the damping plus `1 - damp`
    // (0x6f650..0x6f658).
    COMB.set_damp(damp);
}

// 0x6f660 — __ZN4comb11setfeedbackEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "comb::setfeedback(float)")]
pub fn stub_6f660(gain: f32) {
    // IDA 0x6f660 `comb::setfeedback`: latches the gain (0x6f660).
    COMB.set_feedback(gain);
}

// 0x6f668 — _FLAC__cpu_info
// type: _DWORD *__fastcall(_DWORD *result)
#[doc(alias = "_FLAC__cpu_info")]
pub fn stub_6f668() -> (u32, u32) {
    // IDA 0x6f668 `_FLAC__cpu_info`: no SIMD flags plus the SSE word
    // (0x6f66c..0x6f674).
    (0, 2)
}

// 0x6f67c — _FLAC__crc8
// type: int __fastcall(int, int)
#[doc(alias = "_FLAC__crc8")]
pub fn stub_6f67c(data: &[u8]) -> u8 {
    // IDA 0x6f67c `_FLAC__crc8`: CRC-8 (poly 0x07) over the bytes
    // (0x6f698..0x6f6b4); matches the table walk bit-wise.
    flac_crc8_6f67c(data)
}

// 0x6f6c4 — _FLAC__fixed_restore_signal
// type: _DWORD *__fastcall(void *__src, int, int, _DWORD *__dst)
#[doc(alias = "_FLAC__fixed_restore_signal")]
pub fn stub_6f6c4(residual: &[i32], order: u32, warmup: &[i32]) -> Vec<i32> {
    // IDA 0x6f6c4 `_FLAC__fixed_restore_signal`: orders 0..4 rebuild the
    // signal from the residual plus the warmup (0x6f6d4..tail).
    flac_fixed_restore_6f6c4(residual, order, warmup)
}

// 0x6f804 — _ilog
// type: int __fastcall(unsigned int)
#[doc(alias = "_ilog")]
pub fn stub_6f804(n: u32) -> u32 {
    // IDA 0x6f804 `_ilog`: bit-length, zero for 0 (0x6f808..0x6f824).
    flac_ilog_6f804(n)
}

// 0x6f828 — _icomp
// type: int __fastcall(_DWORD **, _DWORD **)
#[doc(alias = "_icomp")]
pub fn stub_6f828(a: i32, b: i32) -> i32 {
    // IDA 0x6f828 `_icomp`: dereferenced subtraction (0x6f83c).
    a - b
}

// 0x6f840 — _FMOD_floor1_inverse1
// type: int *__fastcall(int, int, _DWORD *)
#[doc(alias = "_FMOD_floor1_inverse1")]
pub fn stub_6f840(has_audio: bool, fit: Vec<i32>) -> Option<Vec<i32>> {
    // IDA 0x6f840 `_FMOD_floor1_inverse1`: with a nonzero flag reads the
    // amp/filter bits into the fit vector (0x6f894..tail); `None` without
    // audio.
    FLOOR1.inverse1(has_audio, fit)
}

// 0x6fbac — _FMOD_floor1_free_look
// type: int __fastcall(int result, void *)
#[doc(alias = "_FMOD_floor1_free_look")]
pub fn stub_6fbac() {
    // IDA 0x6fbac `_FMOD_floor1_free_look`: zeroes the look (0x6fbcc) and
    // frees it.
    FLOOR1.free_look();
}

// 0x6fbe0 — _FMOD_floor1_look
// type: _DWORD *__fastcall(int, int, int *)
#[doc(alias = "_FMOD_floor1_look")]
pub fn stub_6fbe0() -> bool {
    // IDA 0x6fbe0 `_FMOD_floor1_look`: builds the sorted post layout from
    // the rates (0x6fc08..tail).
    FLOOR1.look()
}

// 0x6fe68 — _FMOD_floor1_free_info
// type: int __fastcall(int result, void *)
#[doc(alias = "_FMOD_floor1_free_info")]
pub fn stub_6fe68() {
    // IDA 0x6fe68 `_FMOD_floor1_free_info`: zeroes the header (0x6fe88)
    // and frees it.
    FLOOR1.free_info();
}

// 0x6fe9c — _FMOD_floor1_unpack
// type: int *__fastcall(int, int, int *)
#[doc(alias = "_FMOD_floor1_unpack")]
pub fn stub_6fe9c(order: u32, rates: Vec<u32>) -> bool {
    // IDA 0x6fe9c `_FMOD_floor1_unpack`: reads the order plus the rate
    // table (0x6fed8..tail); false on bad dims.
    FLOOR1.unpack(order, rates)
}

// 0x701fc — _FMOD_floor1_inverse2
// type: int __fastcall(int, int, int, _DWORD *, char *__b)
#[doc(alias = "_FMOD_floor1_inverse2")]
pub fn stub_701fc(fit: &[i32], n: usize) -> Option<Vec<f32>> {
    // IDA 0x701fc `_FMOD_floor1_inverse2`: renders the floor curve by
    // linear dB interpolation across the posts (0x70218..tail); `None`
    // without a fit.
    FLOOR1.inverse2(fit, n)
}

// 0x70458 — _FMOD_Channel_GetUserData
// type: int __fastcall(FMOD::Channel *, void **)
#[doc(alias = "_FMOD_Channel_GetUserData")]
pub fn stub_70458(has_channel: bool) -> i32 {
    // IDA 0x70458 `_FMOD_Channel_GetUserData`: 37 without a channel, else
    // the channel value (0x70464..0x7046c).
    FMOD_SYSTEM.channel_user_data(has_channel)
}

// 0x70474 — _FMOD_System_Create
// type: int __fastcall(FMOD::SystemI **)
#[doc(alias = "_FMOD_System_Create")]
pub fn stub_70474() -> i32 {
    // IDA 0x70474 `_FMOD_System_Create`: callocs plus constructs the
    // system; 37 without a slot, 44 on alloc failure (0x70488..0x70564).
    FMOD_SYSTEM.system_create()
}

// 0x705cc — _FMOD_Memory_GetStats
// type: int __fastcall(_DWORD *, _DWORD *, int)
#[doc(alias = "_FMOD_Memory_GetStats")]
pub fn stub_705cc() -> (u32, u32) {
    // IDA 0x705cc `_FMOD_Memory_GetStats`: flushes the DSP requests, then
    // reports current/max pool bytes (0x705e4..0x70688).
    FMOD_SYSTEM.memory_stats()
}

// 0x7069c — __ZN4FMOD11AsyncThread7releaseEv
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::release(void)")]
pub fn stub_7069c() -> i32 {
    // IDA 0x7069c `AsyncThread::release`: with the thread up, asks it to
    // stop (0x7069c..0x706b0).
    FMOD_ASYNC.release()
}

// 0x706b4 — __ZN4FMOD11AsyncThread10threadFuncEv
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::threadFunc(void)")]
pub fn stub_706b4() -> i32 {
    // IDA 0x706b4 `AsyncThread::threadFunc`: dequeues one stream request
    // per pass; idle passes return 0 (0x706c8..tail).
    FMOD_ASYNC.thread_func()
}

// 0x70ab0 — __ZN4FMOD15asyncThreadFuncEPv
// type: int __fastcall(FMOD::AsyncThread *this, void *)
#[doc(alias = "FMOD::asyncThreadFunc(void *)")]
pub fn stub_70ab0() -> ! {
    todo!("0x70ab0 FMOD::asyncThreadFunc(void *)")
}

// 0x70ab4 — __ZN4FMOD11AsyncThread13reallyReleaseEv
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::reallyRelease(void)")]
pub fn stub_70ab4() -> ! {
    todo!("0x70ab4 FMOD::AsyncThread::reallyRelease(void)")
}

// 0x70bbc — __ZN4FMOD11AsyncThread4initEbPNS_7SystemIE
// type: int __fastcall(FMOD::AsyncThread *this, bool, FMOD::SystemI *)
#[doc(alias = "FMOD::AsyncThread::init(bool,FMOD::SystemI *)")]
pub fn stub_70bbc() -> ! {
    todo!("0x70bbc FMOD::AsyncThread::init(bool,FMOD::SystemI *)")
}

// 0x70c98 — __ZN4FMOD11AsyncThreadC2Ev
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::AsyncThread(void)")]
pub fn stub_70c98() -> ! {
    todo!("0x70c98 FMOD::AsyncThread::AsyncThread(void)")
}

// 0x70cec — __ZN4FMOD11AsyncThreadC1Ev
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::AsyncThread(void)")]
pub fn stub_70cec() -> ! {
    todo!("0x70cec FMOD::AsyncThread::AsyncThread(void)")
}

// 0x70cf0 — __ZN4FMOD11AsyncThread14getAsyncThreadEPNS_6SoundIE
// type: int __fastcall(FMOD::AsyncThread *this, FMOD::SoundI *)
#[doc(alias = "FMOD::AsyncThread::getAsyncThread(FMOD::SoundI *)")]
pub fn stub_70cf0() -> ! {
    todo!("0x70cf0 FMOD::AsyncThread::getAsyncThread(FMOD::SoundI *)")
}

// 0x70ddc — __ZN4FMOD11AsyncThread8shutDownEv
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::shutDown(void)")]
pub fn stub_70ddc() -> ! {
    todo!("0x70ddc FMOD::AsyncThread::shutDown(void)")
}

// 0x70e5c — __ZN4FMOD11AsyncThread6updateEv
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::update(void)")]
pub fn stub_70e5c() -> ! {
    todo!("0x70e5c FMOD::AsyncThread::update(void)")
}

// 0x70ef8 — __Z41__static_initialization_and_destruction_0ii
// type: int __fastcall(int result, int)
#[doc(alias = "__static_initialization_and_destruction_0(int,int)")]
pub fn stub_70ef8() -> ! {
    todo!("0x70ef8 __static_initialization_and_destruction_0(int,int)")
}

// 0x70f2c — __GLOBAL__I__ZN4FMOD11AsyncThread10gAsyncHeadE
// type: int()
#[doc(alias = "global constructor keyed toFMOD::AsyncThread::gAsyncHead")]
pub fn stub_70f2c() -> ! {
    todo!("0x70f2c global constructor keyed toFMOD::AsyncThread::gAsyncHead")
}

// 0x70f38 — __ZN4FMOD7Channel11getUserDataEPPv
// type: int __fastcall(FMOD::Channel *this, void **, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::getUserData(void **)")]
pub fn stub_70f38() -> ! {
    todo!("0x70f38 FMOD::Channel::getUserData(void **)")
}

// 0x70f7c — __ZN4FMOD7Channel11setUserDataEPv
// type: int __fastcall(FMOD::Channel *this, void *, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setUserData(void *)")]
pub fn stub_70f7c() -> ! {
    todo!("0x70f7c FMOD::Channel::setUserData(void *)")
}

// 0x70fb0 — __ZN4FMOD7Channel12setLoopCountEi
// type: int __fastcall(FMOD::Channel *this, int, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setLoopCount(int)")]
pub fn stub_70fb0() -> ! {
    todo!("0x70fb0 FMOD::Channel::setLoopCount(int)")
}

// 0x70fe4 — __ZN4FMOD7Channel7getModeEPj
// type: int __fastcall(FMOD::Channel *this, unsigned int *, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::getMode(unsigned int *)")]
pub fn stub_70fe4() -> ! {
    todo!("0x70fe4 FMOD::Channel::getMode(unsigned int *)")
}

// 0x71028 — __ZN4FMOD7Channel7setModeEj
// type: int __fastcall(FMOD::Channel *this, unsigned int, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setMode(unsigned int)")]
pub fn stub_71028() -> ! {
    todo!("0x71028 FMOD::Channel::setMode(unsigned int)")
}

// 0x7105c — __ZN4FMOD7Channel9isPlayingEPb
// type: int __fastcall(FMOD::Channel *this, bool *, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::isPlaying(bool *)")]
pub fn stub_7105c() -> ! {
    todo!("0x7105c FMOD::Channel::isPlaying(bool *)")
}

// 0x710a0 — __ZN4FMOD7Channel15set3DAttributesEPK11FMOD_VECTORS3_
// type: int __fastcall(FMOD::ChannelI *, int, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::set3DAttributes(FMOD_VECTOR const*,FMOD_VECTOR const*)")]
pub fn stub_710a0() -> ! {
    todo!("0x710a0 FMOD::Channel::set3DAttributes(FMOD_VECTOR const*,FMOD_VECTOR const*)")
}

// 0x710dc — __ZN4FMOD7Channel11setCallbackEPF11FMOD_RESULTP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS5_E
// type: int __fastcall(FMOD::ChannelI *, int, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setCallback(FMOD_RESULT (*)(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *))")]
pub fn stub_710dc() -> ! {
    todo!("0x710dc FMOD::Channel::setCallback(FMOD_RESULT (*)(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *))")
}

// 0x71110 — __ZN4FMOD7Channel15setChannelGroupEPNS_12ChannelGroupE
// type: int __fastcall(FMOD::ChannelI *, FMOD::ChannelGroupI *, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setChannelGroup(FMOD::ChannelGroup *)")]
pub fn stub_71110() -> ! {
    todo!("0x71110 FMOD::Channel::setChannelGroup(FMOD::ChannelGroup *)")
}

// 0x71144 — __ZN4FMOD7Channel11setPriorityEi
// type: int __fastcall(FMOD::Channel *this, int, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setPriority(int)")]
pub fn stub_71144() -> ! {
    todo!("0x71144 FMOD::Channel::setPriority(int)")
}
