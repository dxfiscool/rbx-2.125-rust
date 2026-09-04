// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0xa47358..0xaa1e28 | total filtered 10215, remaining 3428 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::{SharedPtr, WeakPtr};
use rbx_core::shared_ptr::{ControlBlockPd, CreatableInstanceDeleter};

use parking_lot::Mutex;
use rbx_core::signal::Signal;
use crate::generated_05::{ATTR_REFERENT, CreatorRole, EventDescPayload, FunctorOp, GenericSlotWrapper, GuidData, GuidIter, GuidTree, Instance, ReferenceBinder, SignatureItem, Variant, XmlElement, instance_is_a};
use crate::instance::{ModelInstance, Players};
use crate::data_model::DataModel;
use crate::generated_86::stub_6ffc98;
use crate::generated_10::stub_0x701240;
/// `RBX::Network::Players::PlayerChatType` (IDA `0xa4a600`): the tag word
/// carried by `Variant::ChatType`; enumerant values are not yet resolved.
pub type PlayerChatType = u32;
/// Argument tuple of the `Players` 4-arg chat signal (IDA `0xa4ad68`):
/// `(PlayerChatType, speaker, message, recipient)`.
pub type PlayersChatArgs = (PlayerChatType, SharedPtr<Instance>, String, SharedPtr<Instance>);
/// Rust model of `rbx::placement_any<RBX::Region3>` in the
/// `shared_ptr<vector<shared_ptr<Instance>>>` state produced by
/// `operator=` (IDA `0xa47358`). The holder-tag word plus the retained
/// payload collapse into the `Option`; the `Region3` state is unmodeled.
pub struct Region3InstanceList {
    pub instances: Option<SharedPtr<Vec<SharedPtr<Instance>>>>,
}
/// Bound `Players` 3-arg member used by `BoundFuncDesc<Players, void
/// (Instance, string, string), 3>` (IDA `0xa4752c`): the receiver plus the
/// retained instance and the two message strings.
pub type PlayersChatMethod = fn(&Players, &SharedPtr<Instance>, &str, &str);
/// Rust model of that `BoundFuncDesc` (IDA `0xa4752c`): the member pointer
/// plus the three declared argument names and their reflected types.
pub struct PlayersChatFuncDesc {
    pub method: PlayersChatMethod,
    pub arg_names: [String; 3],
    pub signature: Vec<SignatureItem>,
}
/// Bound `Players` 2-arg member used by `BoundFuncDesc<Players, void
/// (string, shared_ptr<Instance>), 2>` (IDA `0xa4dcc0`): the receiver plus
/// the message string and the retained instance.
pub type PlayersStrInstMethod = fn(&Players, &str, &SharedPtr<Instance>);
/// Rust model of that `BoundFuncDesc` (IDA `0xa4dcc0`): the member pointer
/// plus the two declared argument names and their reflected types.
pub struct PlayersStrInstFuncDesc {
    pub method: PlayersStrInstMethod,
    pub arg_names: [String; 2],
    pub signature: Vec<SignatureItem>,
}
/// Bound `Players` 1-arg member used by `BoundFuncDesc<Players,
/// shared_ptr<Instance> (int), 1>` (IDA `0xa4eedc`): the receiver plus the
/// integer key, returning a retained instance.
pub type PlayersIntRetMethod = fn(&Players, i32) -> SharedPtr<Instance>;
/// Rust model of that `BoundFuncDesc` (IDA `0xa4eedc`): the member pointer
/// plus the declared argument name and its reflected type.
pub struct PlayersIntRetFuncDesc {
    pub method: PlayersIntRetMethod,
    pub arg_names: [String; 1],
    pub signature: Vec<SignatureItem>,
}
/// Getter behind `RefPropDescriptor<Players, Instance>` (IDA `0xa4f4a0`):
/// the raw `Instance*` return collapses into an optional retained instance
/// (`None` is the null return at IDA `0xa50018`).
pub type PlayersRefGetter = fn(&Players) -> Option<SharedPtr<Instance>>;
/// Rust model of that `RefPropDescriptor` (IDA `0xa4f4a0`): the name and
/// category words, the getter, the trailing int word, and the
/// attributes/permissions words. The `+44` `GetImpl` layer (IDA `0xa503b8`)
/// collapses away: get-only behavior is in the accessors below.
pub struct PlayersRefPropDesc {
    pub name: String,
    pub category: String,
    pub getter: PlayersRefGetter,
    pub flags: i32,
    pub attributes: u32,
    pub permissions: u32,
}
/// Bound `Players` 1-arg member used by `BoundFuncDesc<Players,
/// shared_ptr<Instance> (shared_ptr<Instance>), 1>` (IDA `0xa53b04`): the
/// receiver plus the retained instance, returning a retained instance.
pub type PlayersInstRetMethod = fn(&Players, &SharedPtr<Instance>) -> SharedPtr<Instance>;
/// Rust model of that `BoundFuncDesc`: the member pointer plus the declared
/// argument name and its reflected type.
pub struct PlayersInstRetFuncDesc {
    pub method: PlayersInstRetMethod,
    pub arg_names: [String; 1],
    pub signature: Vec<SignatureItem>,
}
/// Rust model of `RBX::Reflection::Type<RBX::Instance *>` (IDA `0xa50508`):
/// the descriptor category and the declared tag. The `typeinfo` word and
/// the `addToAllTypes` registry link are compiler/global state that
/// collapses away here.
pub struct InstancePtrType {
    pub category: String,
    pub tag: String,
}
/// Rust model of `RBX::FriendService::FriendStatus` (IDA `0xa8572c`): the
/// friendship relation tag; enumerant values are not yet resolved.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct FriendStatus(pub u32);
/// Rust model of `RBX::AsyncHttpQueue::RequestResult` (IDA `0xa91498`): `1`
/// is success (the only discriminant consulted here); other values are
/// unmodeled failures.
pub type HttpRequestResult = i32;
/// Rust model of `RBX::Network::Player` (IDA `0xa80f18`): the
/// friend-status-changed member signal plus the `+116` data-loaded flag
/// behind `saveInstance` (IDA `0xa83044`). Shared ownership mirrors the
/// C++ `enable_shared_from_this` (`weak_from<Player>`, IDA `0xa851a8`).
pub struct Player {
    pub friend_status_changed: Signal<(SharedPtr<Instance>, FriendStatus)>,
    pub data_loaded: bool,
    /// Character model behind `RefPropDescriptor<Player, ModelInstance>`
    /// (IDA `0xac0518` `assignIDREF`): the retained character; `None` is the
    /// null ref.
    pub character: Mutex<Option<SharedPtr<ModelInstance>>>,
}
/// Bound `Player` 1-arg member used by `BoundFuncDesc<Player, void
/// (shared_ptr<Instance>), 1>` (IDA `0xa965a8`): the receiver plus the
/// retained instance.
pub type PlayerInstMethod = fn(&Player, &SharedPtr<Instance>);
/// Rust model of that `BoundFuncDesc`: the member pointer plus the declared
/// argument name and its reflected type.
pub struct PlayerInstFuncDesc {
    pub method: PlayerInstMethod,
    pub arg_names: [String; 1],
    pub signature: Vec<SignatureItem>,
}
/// Bound `Player` 0-arg member used by `BoundFuncDesc<Player,
/// shared_ptr<Instance> (void), 0>` (IDA `0xa965fc`): the receiver,
/// returning a retained instance.
pub type PlayerVoidRetMethod = fn(&Player) -> SharedPtr<Instance>;
/// Rust model of that `BoundFuncDesc`: the member pointer plus an empty
/// signature.
pub struct PlayerVoidRetFuncDesc {
    pub method: PlayerVoidRetMethod,
    pub arg_names: [String; 0],
    pub signature: Vec<SignatureItem>,
}
/// Bound `Player` 1-arg member used by `BoundFuncDesc<Player,
/// shared_ptr<Instance> (string), 1>` (IDA `0xa969b0`): the receiver plus
/// the string key, returning a retained instance.
pub type PlayerStrRetMethod = fn(&Player, &str) -> SharedPtr<Instance>;
/// Rust model of that `BoundFuncDesc`: the member pointer plus the declared
/// argument name and its reflected type.
pub struct PlayerStrRetFuncDesc {
    pub method: PlayerStrRetMethod,
    pub arg_names: [String; 1],
    pub signature: Vec<SignatureItem>,
}
/// Bound `Player` 2-arg member used by `BoundFuncDesc<Player, void
/// (string, shared_ptr<Instance>), 2>` (IDA `0xa96a58`): the receiver plus
/// the message string and the retained instance.
pub type PlayerStrInstMethod = fn(&Player, &str, &SharedPtr<Instance>);
/// Rust model of that `BoundFuncDesc`: the member pointer plus the two
/// declared argument names and their reflected types.
pub struct PlayerStrInstFuncDesc {
    pub method: PlayerStrInstMethod,
    pub arg_names: [String; 2],
    pub signature: Vec<SignatureItem>,
}
/// Bound `Player` 1-arg member used by `BoundFuncDesc<Player, FriendStatus
/// (shared_ptr<Instance>), 1>` (IDA `0xa96cc4`): the receiver plus the
/// retained instance, returning the friendship tag.
pub type PlayerInstStatusMethod = fn(&Player, &SharedPtr<Instance>) -> FriendStatus;
/// Rust model of that `BoundFuncDesc`: the member pointer plus the declared
/// argument name and its reflected type.
pub struct PlayerInstStatusFuncDesc {
    pub method: PlayerInstStatusMethod,
    pub arg_names: [String; 1],
    pub signature: Vec<SignatureItem>,
}
/// Rust model of `boost::_bi::bind_t<void, void (*)(weak<Player>,
/// weak<Instance>, bool), list_av_3<weak<Player>, arg<1>, bool>>` (IDA
/// `0xa98f78`): the retained player weak (the `weak_count` copy at bind
/// time) plus the late-bound appearance weak and the bool value.
#[derive(Clone)]
pub struct PlayerAppearanceBind {
    pub player: WeakPtr<Player>,
    pub flag: bool,
}
/// Rust model of `boost::_bi::bind_t<void, void (*)(string*, exception*,
/// weak<Player>, weak<DataModel>), list_av_4<arg<1>, arg<2>, weak<Player>,
/// weak<DataModel>>>` (IDA `0xa99284`): the two retained weaks; the string
/// and exception args stay late-bound.
#[derive(Clone)]
pub struct PlayerDataModelBind {
    pub player: WeakPtr<Player>,
    pub data_model: WeakPtr<DataModel>,
}
/// Rust model of `boost::_bi::bind_t<void, void (*)(ModelInstance const&,
/// Instance const&), list_av_2<ModelInstance, arg<1>>>` (IDA `0xa99cb4`):
/// the retained model plus the late-bound child.
#[derive(Clone)]
pub struct ModelChildBind {
    pub model: SharedPtr<ModelInstance>,
}
/// Rust model of `boost::_bi::bind_t<void, void (*)(ModelInstance,
/// RequestResult, vector), list_av_3<ModelInstance, arg<1>, arg<2>>>` (IDA
/// `0xa9a314`): the retained model plus the late-bound result and vector.
#[derive(Clone)]
pub struct ModelLoadBind {
    pub model: SharedPtr<ModelInstance>,
}
/// Rust model of `RBX::PlayerMouse` (IDA `0xaa1e28`): the player-mouse leaf;
/// members land with the input batch.
pub struct PlayerMouse {
    _opaque: (),
}
/// Rust model of an `rbx::signals::signal<void ()(PlayerChatType,
/// SharedPtr<Instance>, string, SharedPtr<Instance>)>::slot` link holding a
/// wrapper bind (IDA `0xa4c674` insert): same intrusive-`next` discipline as
/// `TripleSlotNode`, but the callback is the chat wrapper function.
pub struct Chat4SlotNode {
    pub next: Option<SharedPtr<Chat4SlotNode>>,
    pub func: Chat4WrapperFunction,
}
/// Process-wide mutex behind the chat-signal slot guards (IDA `0xa4c71e`,
/// `0xa4ad8e`); per-instantiation twin of `TRIPLE_SLOT_STATIC_MUTEX`.
static CHAT4_SLOT_STATIC_MUTEX: Mutex<()> = Mutex::new(());
/// Rust model of `boost::_bi::bind_t<void, mf4 execute4 on
/// GenericSlotWrapper>` (IDA `0xa4b0f4`): retained wrapper (the
/// `shared_count` copy at bind time) plus late-bound chat args.
#[derive(Clone)]
pub struct Chat4WrapperBind {
    pub target: SharedPtr<GenericSlotWrapper>,
}
/// Rust model of `boost::function4<void, PlayerChatType,
/// SharedPtr<Instance>, string, SharedPtr<Instance>>` holding the `execute4`
/// bind (IDA `0xa4bd28`): nullability of the retained wrapper is the vtable
/// word. Twin of `TripleWrapperFunction`.
#[derive(Clone, Default)]
pub struct Chat4WrapperFunction {
    pub target: Option<SharedPtr<GenericSlotWrapper>>,
}
/// Mangled type name `strcmp`ed by the `functor_manager::manager`
/// check-type path (disasm `0xa4c616`-`0xa4c620`).
pub const CHAT4_BIND_TYPE_NAME: &str =
    "N5boost3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS4_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS4_8InstanceEEERKSsSG_EENS0_5list5INS0_5valueINSC_IS6_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEENSO_ILi4EEEEEEE";

// 0xa47358 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKSt6vectorINS6_INS1_8InstanceEEESaIS9_EEEEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&)
pub fn stub_a47358<'a>(this: &'a mut Region3InstanceList, src: &SharedPtr<Vec<SharedPtr<Instance>>>) -> &'a mut Region3InstanceList {
    // IDA 0xa47358: `typed_holder<shared_ptr<vector>>::singleton` once-init
    // (disasm 0xa4737a-0xa473b8), then the same-type fast path (disasm
    // 0xa473d0-0xa474c6: `shared_count` retain of the incoming payload plus
    // release of the held one) versus destroy + copy-construct + tag install
    // (disasm 0xa473da-0xa4751e). Both collapse into the retained `Arc` clone.
    this.instances = Some(src.clone());
    this
}

// 0xa4752c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EEC2EMS3_FvS7_SsSsEPKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::BoundFuncDesc(void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::BoundFuncDesc(void (RBX::Network::Players::*)(boost::shared_ptr<RBX::Instance>,std::string,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_a4752c(this: *mut PlayersChatFuncDesc, method: PlayersChatMethod, first: &str, second: &str, third: &str) {
    // IDA 0xa4752c: `Players::classDescriptor` once-init (disasm
    // 0xa475a8-0xa4760e), base `FunctionDescriptor` init (disasm 0xa47632),
    // member-pointer words (disasm 0xa4764c-0xa4765e), then three
    // `addArgument` calls for `shared_ptr<Instance>` (disasm 0xa476b6-0xa476c6)
    // and two `std::string` args (disasm 0xa476d4-0xa4770e). The descriptor
    // temps are compiler-managed here.
    // SAFETY: `this` must point to valid uninitialized `PlayersChatFuncDesc` storage.
    unsafe {
        core::ptr::write(
            this,
            PlayersChatFuncDesc {
                method,
                arg_names: [first.to_string(), second.to_string(), third.to_string()],
                signature: vec![
                    SignatureItem { type_name: "SharedPtr<Instance>" },
                    SignatureItem { type_name: "string" },
                    SignatureItem { type_name: "string" },
                ],
            },
        );
    }
}

// 0xa47920 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()
pub fn stub_a47920(this: *mut PlayersChatFuncDesc) {
    // IDA 0xa47920: D0 — D1 body (disasm 0xa47970) plus `operator delete`
    // (disasm 0xa47976); the box reclaim runs the field drops and frees
    // together.
    // SAFETY: `this` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(this));
    }
}

// 0xa479c0 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_a479c0(desc: &PlayersChatFuncDesc, players: &Players, args: &[Variant]) {
    // IDA 0xa479c0: `source ? source - 36 : 0` member-signal select (disasm
    // 0xa47a14-0xa47a22), three `ArgHelper::getArg` extracts
    // (`shared_ptr<Instance>`, `string`, `string`; disasm 0xa47a2e-0xa47a56),
    // then `Call3Helper::call` (disasm 0xa47a70); the trailing string and
    // `shared_ptr` releases are `Drop`-managed here.
    assert!(args.len() == 3, "0xa479c0: Arguments must hold 3 values");
    let inst = match &args[0] {
        Variant::Instance(i) => i,
        _ => panic!("0xa479c0: any_cast<SharedPtr<Instance>> failed"),
    };
    let first = match &args[1] {
        Variant::Text(s) => s,
        _ => panic!("0xa479c0: any_cast<string> failed"),
    };
    let second = match &args[2] {
        Variant::Text(s) => s,
        _ => panic!("0xa479c0: any_cast<string> failed"),
    };
    stub_a47d30(players, desc.method, inst, first, second);
}

// 0xa47d30 — __ZN3RBX10Reflection11Call3HelperINS_7Network7PlayersEMS3_FvN5boost10shared_ptrINS_8InstanceEEESsSsES7_SsSsvE4callEPS3_S9_RNS0_7VariantERKS7_RKSsSH_
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Network::Players,void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),rbx_core::SharedPtr<RBX::Instance>,std::string,std::string,void>::call(RBX::Network::Players*,void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,std::string const&)")]
// was: RBX::Reflection::Call3Helper<RBX::Network::Players,void (RBX::Network::Players::*)(boost::shared_ptr<RBX::Instance>,std::string,std::string),boost::shared_ptr<RBX::Instance>,std::string,std::string,void>::call(RBX::Network::Players*,void (RBX::Network::Players::*)(boost::shared_ptr<RBX::Instance>,std::string,std::string),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,std::string const&,std::string const&)
pub fn stub_a47d30(players: &Players, method: PlayersChatMethod, inst: &SharedPtr<Instance>, first: &str, second: &str) {
    // IDA 0xa47d30: member-pointer adjust for the `a3` tag bit (disasm
    // 0xa47d8c-0xa47d92), retained `shared_ptr` copy of the instance arg
    // (disasm 0xa47d94-0xa47dea), local `string` copies of both message args
    // (disasm 0xa47df4-0xa47e00), the member call (disasm 0xa47e12), then the
    // mirrored string and `shared_ptr` releases. Clones plus the call plus
    // `Drop` are the same sequence.
    let inst = inst.clone();
    let first = first.to_string();
    let second = second.to_string();
    method(players, &inst, &first, &second);
}

// 0xa49b0c — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_EC2ESD_PKcSG_SG_SG_SG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_a49b0c(this: *mut EventDescPayload, name: &str, permissions: u32, attributes: u32) {
    // IDA 0xa49b0c: `Players::classDescriptor` once-init (disasm
    // 0xa49b9a-0xa49c00), base `EventDescriptor` init (disasm 0xa49c24), then
    // four signature items — `PlayerChatType` (disasm 0xa49c50-0xa49cb2),
    // `shared_ptr<Instance>` (disasm 0xa49cc6-0xa49d24), `string` (disasm
    // 0xa49d38-0xa49d96), `shared_ptr<Instance>` (disasm 0xa49daa-0xa49e00) —
    // each a `Type::getSingleton` + `Item::Item` + list `hook`. The member
    // signal offset (`+40`, disasm 0xa49c3a) collapses into the payload-side
    // list, as in the `0x707b28` twin.
    // SAFETY: `this` must point to valid uninitialized `EventDescPayload` storage.
    unsafe {
        core::ptr::write(
            this,
            EventDescPayload {
                name: name.to_string(),
                permissions,
                attributes,
                items: vec![
                    SignatureItem { type_name: "PlayerChatType" },
                    SignatureItem { type_name: "SharedPtr<Instance>" },
                    SignatureItem { type_name: "string" },
                    SignatureItem { type_name: "SharedPtr<Instance>" },
                ],
                connections: Mutex::new(Vec::new()),
                single: Signal::new(),
                triple: Signal::new(),
                triple_isi: Signal::new(),
                pair_if: Signal::new(),
            },
        );
    }
}

// 0xa4a0a0 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()
pub fn stub_a4a0a0(this: *mut EventDescPayload) {
    // IDA 0xa4a0a0: D0 — vtable reset (disasm 0xa4a0d8) + signature-list
    // `_M_clear` loop running each item disposer (disasm 0xa4a0e2-0xa4a12c)
    // plus `operator delete` (disasm 0xa4a130); the box reclaim runs the
    // field drops and frees together. Same shape as 0x707d18.
    // SAFETY: `this` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(this));
    }
}

// 0xa4a17c — __ZNK3RBX10Reflection13EventDescImplILi4ENS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_E14connectGenericEPNS0_11EventSourceENS6_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<4,RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_a4a17c(desc: *const EventDescPayload, slot: &SharedPtr<GenericSlotWrapper>) {
    // IDA 0xa4a17c: retain the wrapper (the `shared_count` copy at disasm
    // 0xa4a1a0-0xa4a22a), `bind(execute4-mf4, wrapper, _1.._4)` (disasm
    // 0xa4a238), wrap in `function4` (disasm 0xa4a244), `EventDescBase::connect`
    // (disasm 0xa4a252), then the two mirrored releases. Collapses to a
    // retained clone + push onto the payload-side list; twin of 0x707dcc.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    unsafe {
        (*desc).connections.lock().push(slot.clone());
    }
}

// 0xa4a600 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISI_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<4,RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_a4a600(desc: *const EventDescPayload, args: &[Variant]) {
    // IDA 0xa4a600: `ReleaseAssert(args.size() == 4)` (Event.h:413, disasm
    // 0xa4a63e-0xa4a6b6), four `any_cast`s — `PlayerChatType` (disasm
    // 0xa4a6d2-0xa4a70c, `bad_placement_any_cast` on mismatch), the two
    // `shared_ptr<Instance>`s and the `string` — then
    // `signal_with_args<4>::operator()` fans out to each connected slot's
    // `callable::call`. Twin of 0x707f20 with the extra tag arg.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    assert!(args.len() == 4, "0xa4a600: args.size() == 4");
    let chat_type = match &args[0] {
        Variant::ChatType(t) => *t,
        _ => panic!("0xa4a600: any_cast<PlayerChatType> failed"),
    };
    let (speaker, message, recipient) = match (&args[1], &args[2], &args[3]) {
        (Variant::Instance(a), Variant::Text(b), Variant::Instance(c)) => (a, b, c),
        _ => panic!("0xa4a600: any_cast<(Instance, string, Instance)> failed"),
    };
    unsafe {
        let slots = (*desc).connections.lock().clone();
        for slot in slots.iter() {
            if let Some(cb) = slot.on_player_chat {
                cb(chat_type, speaker, message, recipient);
            }
        }
    }
}

// 0xa4ad50 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_a4ad50(desc: *const EventDescPayload) {
    // IDA 0xa4ad50: `source ? source - 36 : 0` (disasm 0xa4ad52-0xa4ad58)
    // selects the member signal at `*(a1 + 40) + v2`, then
    // `signal::disconnectAll` (disasm 0xa4ad66); the addressing collapses
    // into the payload-side list, so this clears the connections. Twin of
    // 0x706754.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    unsafe {
        (*desc).connections.lock().clear();
    }
}

// 0xa4ad68 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::disconnectAll(void)
pub fn stub_a4ad68(sig: *mut Signal<PlayersChatArgs>) {
    // IDA 0xa4ad68: loop over the slot list (disasm 0xa4ad8e): lock the
    // signal mutex (disasm 0xa4adc2-0xa4add0), unlink up to 10 slots per pass
    // (disasm 0xa4ade2-0xa4adf4), splice the remainder back (disasm
    // 0xa4ae20), release the unlinked head (disasm 0xa4ae24-0xa4ae64), repeat
    // until empty. `Signal::disconnect_all` holds the same lock and drops
    // the same slot list. Twin of 0x2b8f4c.
    // SAFETY: `sig` must point to a valid `Signal`.
    unsafe {
        (*sig).disconnect_all();
    }
}

// 0xa4af20 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_E7connectEPNS0_11EventSourceERKNS5_8functionIS9_EE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)const")]
// was: RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> const&)const
pub fn stub_a4af20(desc: *const EventDescPayload, func: &Chat4WrapperFunction) -> Option<SharedPtr<GenericSlotWrapper>> {
    // IDA 0xa4af20: null function returns a null connection (disasm
    // 0xa4af70, 0xa4afec); else `malloc` the `callable` slot (disasm
    // 0xa4af7c-0xa4afc2), copy the functor into it (disasm 0xa4afc4-0xa4b01e),
    // `signal::insert` into the member signal (disasm 0xa4b02c), publish the
    // connection (disasm 0xa4b032-0xa4b038). Collapses to retaining the
    // bound wrapper and pushing it onto the payload-side list; the returned
    // clone is the connection keep-alive.
    // SAFETY: `desc` must point to a valid `EventDescPayload`.
    let target = func.target.clone()?;
    unsafe {
        (*desc).connections.lock().push(target.clone());
    }
    Some(target)
}

// 0xa4b0f4 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS1_8InstanceEEERKSsSD_NS9_IS3_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISO_T0_T1_T2_T3_T4_EENSM_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSR_FSO_SS_ST_SU_SV_ESY_SZ_S10_S11_S12_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list_av_5<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)
pub fn stub_a4b0f4<'a>(out: &'a mut Chat4WrapperBind, target: &SharedPtr<GenericSlotWrapper>) -> &'a mut Chat4WrapperBind {
    // IDA 0xa4b0f4: retain the wrapper (the `shared_count` copy at disasm
    // 0xa4b118-0xa4b19a), build the `list5<value<wrapper>, _1.._4>` (disasm
    // 0xa4b1a2), store the `mf4` + list words (disasm 0xa4b1ac-0xa4b484),
    // then release the temporary (disasm 0xa4b486-0xa4b530). Collapses to
    // the retained clone. Twin of 0x2b91b8.
    *out = Chat4WrapperBind { target: target.clone() };
    out
}
// 0xa4b560 — __ZN3RBX10Reflection18GenericSlotWrapper8execute4INS_7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS9_EEvRKT_RKT0_RKT1_RKT2_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute4<RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>(RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute4<RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>(RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_a4b560(wrapper: &SharedPtr<GenericSlotWrapper>, chat_type: PlayerChatType, speaker: &SharedPtr<Instance>, message: &str, recipient: &SharedPtr<Instance>) {
    // IDA 0xa4b560: build the 4-`Variant` vector (disasm 0xa4b5d6) —
    // `PlayerChatType` tag (disasm 0xa4b602-0xa4b698), retained speaker
    // (disasm 0xa4b69c-0xa4b6b4), message string (disasm 0xa4b6ce-0xa4b778),
    // retained recipient (disasm 0xa4b78a-0xa4b798) — call the wrapper's
    // script callback (disasm 0xa4b7a8), then run the per-item disposers
    // (disasm 0xa4b7aa-0xa4b7d8). The callback is the native `on_player_chat`
    // stand-in; the vector temps are `Drop`-managed here.
    if let Some(cb) = wrapper.on_player_chat {
        cb(chat_type, speaker, message, recipient);
    }
}

// 0xa4bd28 — __ZN5boost9function4IvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS1_8InstanceEEESsS7_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS1_10Reflection18GenericSlotWrapperERKS4_RKS7_RKSsSJ_EENSA_5list5INSA_5valueINS5_ISF_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEEEvT_
#[doc(alias = "void boost::function4<void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
// was: void boost::function4<void,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)
pub fn stub_a4bd28(dst: &mut Chat4WrapperFunction, src: &Chat4WrapperBind) {
    // IDA 0xa4bd28: `function4::assign_to<bind_t>` spills the bind functor
    // (disasm 0xa4bd58-0xa4be42) and heap-installs it through
    // `basic_vtable4::assign_to` (disasm 0xa4be4e); the retained wrapper
    // clone is that same copy. Twin of 0x2b9658.
    dst.target = Some(src.target.clone());
}

// 0xa4c1a0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS7_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS7_8InstanceEEERKSsSJ_EENS3_5list5INS3_5valueINSF_IS9_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEE6manageERKNS1_15function_bufferERSZ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_a4c1a0(src: &Chat4WrapperBind, dst: &mut Chat4WrapperBind, op: FunctorOp) -> bool {
    // IDA 0xa4c1a0: `functor_manager::manage` dispatches on `op` into
    // `manager` (same shape as the 0x2b9750 prologue); discriminants mirror
    // the `0x705780` family (0 clone, 1 move, 2 destroy, 3 check-type, 4
    // get-type). Twin of 0x2b9750.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            *dst = src.clone();
            true
        }
        FunctorOp::Destroy => false,
        FunctorOp::CheckType => {
            *dst = src.clone();
            true
        }
        FunctorOp::GetType => true,
    }
}
// was: boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_a4c1c4(bind: &Chat4WrapperBind, chat_type: PlayerChatType, speaker: &SharedPtr<Instance>, message: &str, recipient: &SharedPtr<Instance>) {
    // IDA 0xa4c1c4: unwrap the buffer to the `bind_t` (disasm 0xa4c1cc-0xa4c1e6,
    // adjusting for the member-pointer tag bit) and tail-call the `mf4`
    // through `list5::operator()` (disasm 0xa4c1f4) — the `execute4` bind.
    // Twin of 0x2b976c.
    stub_a4b560(&bind.target, chat_type, speaker, message, recipient);
}

// 0xa4c1f8 — __ZNK5boost6detail8function13basic_vtable4IvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS3_8InstanceEEESsS9_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS3_10Reflection18GenericSlotWrapperERKS6_RKS9_RKSsSL_EENSC_5list5INSC_5valueINS7_ISH_EEEENS_3argILi1EEENST_ILi2EEENST_ILi3EEENST_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable4<void,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_a4c1f8(dst: &mut Chat4WrapperFunction, src: &Chat4WrapperBind) -> bool {
    // IDA 0xa4c1f8: `basic_vtable4::assign_to` with `function_obj_tag`:
    // retain the incoming wrapper (disasm 0xa4c226-0xa4c28e), heap-install
    // the 4-word bind copy (disasm 0xa4c294-0xa4c328), release the temporary
    // (disasm 0xa4c332-0xa4c3a8), return `true` (disasm 0xa4c3d8) — the
    // functor never needs the heap fallback to fail. Twin of 0x2b9874.
    stub_a4bd28(dst, src);
    true
}

// 0xa4c4e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS7_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS7_8InstanceEEERKSsSJ_EENS3_5list5INS3_5valueINSF_IS9_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSZ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_a4c4e0(src: &Chat4WrapperBind, dst: &mut Chat4WrapperFunction, op: FunctorOp, type_name: &str) -> bool {
    // IDA 0xa4c4e0: `functor_manager::manager` switch on `op` (disasm
    // 0xa4c544): 0 heap-clone the 4-word bind plus a `shared_count` retain
    // (disasm 0xa4c554-0xa4c5d2), 1 move the words and clear the source
    // (disasm 0xa4c5d8-0xa4c5dc), 2 destroy the count and free (disasm
    // 0xa4c5e2-0xa4c5f8), 3 conditional copy on `strcmp` against the bind
    // type name (disasm 0xa4c616-0xa4c620), default publish the typeinfo
    // (disasm 0xa4c53e-0xa4c540). The heap words collapse into the retained
    // clone; `CheckType` reports the match.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            dst.target = Some(src.target.clone());
            true
        }
        FunctorOp::Destroy => {
            dst.target = None;
            false
        }
        FunctorOp::CheckType => type_name == CHAT4_BIND_TYPE_NAME,
        FunctorOp::GetType => true,
    }
}

// 0xa4c674 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE6insertEPNSB_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_a4c674(head: *mut Option<SharedPtr<Chat4SlotNode>>, slot: *mut Chat4SlotNode) {
    // IDA 0xa4c674: `ReleaseAssert(item)` on the null slot (signal.h:290,
    // disasm 0xa4c6b2-0xa4c71e), mutex acquisition (disasm 0xa4c71e-0xa4c734),
    // then the intrusive link with the `next == head` assert (signal.h:310,
    // disasm 0xa4c746-0xa4c822) and list-head publish (disasm 0xa4c814). The
    // signal list itself is unmodeled, so the head is an explicit out-param;
    // same head-explicit collapse as the 0x2b57e0 twin.
    // SAFETY: `head` must be writable; `slot` must be a live box pointer with
    // no concurrent/shared mutation; the node must stay alive while linked.
    unsafe {
        assert!(!slot.is_null(), "0xa4c674: item");
        let guard = CHAT4_SLOT_STATIC_MUTEX.lock();
        let owned = SharedPtr::from_raw(slot);
        let linked = owned.clone();
        core::mem::forget(owned);
        (*slot).next = (*head).clone();
        *head = Some(linked);
        drop(guard);
    }
}

// 0xa4c934 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS4_8InstanceEEESsSA_EE4slotEEaSEPSD_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot*)
pub fn stub_a4c934(dst: *mut Option<SharedPtr<Chat4SlotNode>>, src: *mut Chat4SlotNode) {
    // IDA 0xa4c934: move-assign (`aSEPSD_`, pointer `src`): null skips the
    // add_ref (disasm 0xa4c944-0xa4c9a4, with the `strong < max - 10` assert),
    // store (disasm 0xa4c9ac), release-old with virtual delete + free
    // (disasm 0xa4c9b0-0xa4c9dc). The `Arc` clone is the add_ref and the
    // overwritten `Option` drop is the release.
    // SAFETY: `dst` must be writable; `src` must be null or a live
    // `SharedPtr`-owned node pointer whose ownership moves to `dst`.
    unsafe {
        if src.is_null() {
            *dst = None;
        } else {
            let owned = SharedPtr::from_raw(src);
            let cloned = owned.clone();
            core::mem::forget(owned);
            *dst = Some(cloned);
        }
    }
}

// 0xa4c9e8 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE13callable_slotINS6_8functionISA_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub fn stub_a4c9e8(slot: *mut Chat4SlotNode) {
    // IDA 0xa4c9e8: `callable_slot` D1 delegates to `callable` D1 (disasm
    // 0xa4c9f0) — vtable resets (compiler-managed) + function clear + link
    // release; storage kept. Same body as 0x2b5b30.
    // SAFETY: `slot` must point to a valid `Chat4SlotNode`.
    unsafe {
        (*slot).func = Chat4WrapperFunction::default();
        (*slot).next = None;
    }
}

// 0xa4c9f4 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE13callable_slotINS6_8functionISA_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub fn stub_a4c9f4(slot: *mut Chat4SlotNode) {
    // IDA 0xa4c9f4: `callable_slot` D0 — the D1 body plus `operator delete`;
    // the box reclaim runs the field drops and frees together. Same shape
    // as 0x2b5b5c.
    // SAFETY: `slot` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(slot));
    }
}

// 0xa4caa8 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::disconnect(void)
pub fn stub_a4caa8(slot: *mut Chat4SlotNode) {
    // IDA 0xa4caa8: no-op when the link word is null (disasm 0xa4cad2),
    // else once-init the slot mutex (disasm 0xa4cb14-0xa4cb82), lock it
    // (disasm 0xa4cb90), clear the link and `remove` from the list (disasm
    // 0xa4cb94-0xa4cba0), unlock (disasm 0xa4cba8-0xa4cbb2). Clearing the
    // callback and the successor is the unlink, and the static lock is the
    // same guard. Same shape as the 0x2b5c30 twin.
    // SAFETY: `slot` must point to a valid `Chat4SlotNode` with no
    // concurrent/shared mutation.
    unsafe {
        if (*slot).next.is_some() {
            let _guard = CHAT4_SLOT_STATIC_MUTEX.lock();
            (*slot).func = Chat4WrapperFunction::default();
            (*slot).next = None;
        }
    }
}

// 0xa4cc28 — __ZNK3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::connected(void)const
pub fn stub_a4cc28(slot: *const Chat4SlotNode) -> bool {
    // IDA 0xa4cc28: `*(a1 + 12) != 0` (disasm 0xa4cc30) — the intrusive link
    // word; same as 0x2b5d40.
    // SAFETY: `slot` must point to a valid `Chat4SlotNode`.
    unsafe { (*slot).next.is_some() }
}

// 0xa4cc34 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS3_8InstanceEEESsSA_EE4slotENS7_8functionISB_EELi4ESB_E4callES6_SA_SsSA_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::call(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_a4cc34(slot: &Chat4SlotNode, chat_type: PlayerChatType, speaker: &SharedPtr<Instance>, message: &str, recipient: &SharedPtr<Instance>) {
    // IDA 0xa4cc34: retain the speaker (disasm 0xa4cc58-0xa4ccda), copy the
    // message string (disasm 0xa4cce2), retain the recipient (disasm
    // 0xa4cce8-0xa4cd3e), `function4::operator()` on the slot functor
    // (disasm 0xa4cd50), then the mirrored releases (disasm 0xa4cd56-0xa4cece).
    // Clones plus the operator() dispatch plus `Drop` are the same sequence.
    stub_a4d148(&slot.func, chat_type, speaker, message, recipient);
}

// 0xa4d130 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS3_8InstanceEEESsSA_EE4slotENS7_8functionISB_EELi4ESB_E4callES6_SA_SsSA_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::call(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_a4d130(slot: &Chat4SlotNode, chat_type: PlayerChatType, speaker: &SharedPtr<Instance>, message: &str, recipient: &SharedPtr<Instance>) {
    // IDA 0xa4d130: non-virtual thunk — `a1 - 4` adjusts the `callable`
    // subobject back to the slot base (disasm 0xa4d140), then tail-calls
    // `callable::call`. The adjustment is a vtable-layout detail that
    // collapses away here; direct delegation keeps the two bodies identical.
    stub_a4cc34(slot, chat_type, speaker, message, recipient);
}

// 0xa4d148 — __ZNK5boost9function4IvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS1_8InstanceEEESsS7_EclES4_S7_SsS7_
#[doc(alias = "boost::function4<void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::function4<void,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)const
pub fn stub_a4d148(func: &Chat4WrapperFunction, chat_type: PlayerChatType, speaker: &SharedPtr<Instance>, message: &str, recipient: &SharedPtr<Instance>) {
    // IDA 0xa4d148: `bad_function_call` throw on the null vtable (disasm
    // 0xa4d19e-0xa4d4ae), else retain both `shared_ptr` args plus a `string`
    // copy (disasm 0xa4d1a6-0xa4d262), dispatch through the invoker vtable
    // (disasm 0xa4d276), then the mirrored releases. The empty check plus
    // the `execute4` dispatch plus `Drop` are the same sequence.
    let target = func.target.as_ref().expect("0xa4d148: call to empty boost::function");
    let speaker = speaker.clone();
    let message = message.to_string();
    let recipient = recipient.clone();
    stub_a4b560(target, chat_type, &speaker, &message, &recipient);
}

// 0xa4d734 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE6removeEPNSB_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_a4d734(slot: *mut Chat4SlotNode) {
    // IDA 0xa4d734: `ReleaseAssert(!intrusive_ptr_expired(item))`
    // (signal.h:261), `SignalPrints` log line, then unlink of the item from
    // the signal list (list walk with the `intrusive_ptr::operator=` splice).
    // The log collapses; the expired-assert becomes a linked-assert and the
    // unlink clears the successor. Twin of 0x7093f0 on `Chat4SlotNode`.
    // SAFETY: `slot` must point to a valid `Chat4SlotNode`.
    unsafe {
        debug_assert!((*slot).next.is_some(), "0xa4d734: intrusive_ptr_expired");
        (*slot).next = None;
    }
}

// 0xa4d820 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_init_mutex(void)
pub fn stub_a4d820() -> &'static Mutex<()> {
    // IDA 0xa4d820: guard-checked once-init (`__cxa_guard_acquire`) of the
    // function-local `safe_static_do_get_mutex::value`, `mutex::mutex` plus
    // `__cxa_atexit` registration. A `static` with `const` init is the same
    // once-init; the pthread object lives inside `Mutex`. Twin of 0x7094e4.
    &CHAT4_SLOT_STATIC_MUTEX
}

// 0xa4d904 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS3_8InstanceEEESsSA_EE4slotENS7_8functionISB_EELi4ESB_ED2Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_a4d904(slot: *mut Chat4SlotNode) {
    // IDA 0xa4d904: `callable` D2 — vtable resets (compiler-managed),
    // `function4::clear(a1 + 4)` running the bind manager destroy op plus
    // `a1[4] = 0` (disasm `~callable`), then `release(a1[2])` of the link;
    // storage kept. Same body as the 0x7095d4 twin on `Chat4SlotNode`.
    // SAFETY: `slot` must point to a valid `Chat4SlotNode`.
    unsafe {
        (*slot).func.target = None;
        (*slot).next = None;
    }
}

// 0xa4da9c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS3_8InstanceEEESsSA_EE4slotENS7_8functionISB_EELi4ESB_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_a4da9c(slot: *mut Chat4SlotNode) {
    // IDA 0xa4da9c: `callable` D1 — same vtable-reset + `function4::clear` +
    // link-release sequence as D2 (IDA 0xa4d904); storage kept.
    // SAFETY: `slot` must point to a valid `Chat4SlotNode`.
    stub_a4d904(slot);
}

// 0xa4daa8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS3_8InstanceEEESsSA_EE4slotENS7_8functionISB_EELi4ESB_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_a4daa8(slot: *mut Chat4SlotNode) {
    // IDA 0xa4daa8: `callable` D0 — the D1 body (`clear` + `release`) plus
    // `operator delete`; the box reclaim runs the field drops and frees
    // together. Same shape as 0x7096e4.
    // SAFETY: `slot` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(slot));
    }
}

// 0xa4db5c — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_a4db5c(slot: *mut Chat4SlotNode) {
    // IDA 0xa4db5c: `slot` D1 — vtable resets (compiler-managed) plus the
    // single `shared_ptr` release of the link word at `+8`; the callback
    // word was already cleared by the derived `callable` D1. Storage kept.
    // SAFETY: `slot` must point to a valid `Chat4SlotNode`.
    unsafe {
        (*slot).next = None;
    }
}

// 0xa4dbb8 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_a4dbb8(slot: *mut Chat4SlotNode) {
    // IDA 0xa4dbb8: `slot` D0 — the D1 body (link release) plus `operator
    // delete`; the box reclaim runs the field drops and frees together.
    // SAFETY: `slot` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(slot));
    }
}

// 0xa4dcc0 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS3_FvSsS7_EPKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::Network::Players::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::Network::Players::*)(std::string,boost::shared_ptr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_a4dcc0(this: *mut PlayersStrInstFuncDesc, method: PlayersStrInstMethod, first: &str, second: &str) {
    // IDA 0xa4dcc0: `Players::classDescriptor` once-init, base
    // `FunctionDescriptor` init, member-pointer words, then two
    // `addArgument` calls for the `std::string` (first) and
    // `shared_ptr<Instance>` (second) args. The descriptor temps are
    // compiler-managed here. Twin of 0xa4752c with (string, Instance) args.
    // SAFETY: `this` must point to valid uninitialized `PlayersStrInstFuncDesc` storage.
    unsafe {
        core::ptr::write(
            this,
            PlayersStrInstFuncDesc {
                method,
                arg_names: [first.to_string(), second.to_string()],
                signature: vec![
                    SignatureItem { type_name: "string" },
                    SignatureItem { type_name: "SharedPtr<Instance>" },
                ],
            },
        );
    }
}

// 0xa4e000 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
pub fn stub_a4e000(this: *mut PlayersStrInstFuncDesc) {
    // IDA 0xa4e000: D0 — D1 body plus `operator delete`; the box reclaim
    // runs the field drops and frees together. Same shape as 0xa47920.
    // SAFETY: `this` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(this));
    }
}

// 0xa4e0a0 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_a4e0a0(desc: &PlayersStrInstFuncDesc, players: &Players, args: &[Variant]) {
    // IDA 0xa4e0a0: `source ? source - 36 : 0` member-signal select, two
    // `ArgHelper::getArg` extracts (`std::string` then
    // `shared_ptr<Instance>`), then `Call2Helper::call`; the trailing
    // string and `shared_ptr` releases are `Drop`-managed here. Twin of
    // 0xa479c0 with (string, Instance) args.
    assert!(args.len() == 2, "0xa4e0a0: Arguments must hold 2 values");
    let message = match &args[0] {
        Variant::Text(s) => s,
        _ => panic!("0xa4e0a0: any_cast<string> failed"),
    };
    let inst = match &args[1] {
        Variant::Instance(i) => i,
        _ => panic!("0xa4e0a0: any_cast<SharedPtr<Instance>> failed"),
    };
    stub_a4e388(players, desc.method, message, inst);
}

// 0xa4e388 — __ZN3RBX10Reflection11Call2HelperINS_7Network7PlayersEMS3_FvSsN5boost10shared_ptrINS_8InstanceEEEESsS7_vE4callEPS3_S9_RNS0_7VariantERKSsRKS7_
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Network::Players,void (RBX::Network::Players::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),std::string,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Network::Players*,void (RBX::Network::Players::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call2Helper<RBX::Network::Players,void (RBX::Network::Players::*)(std::string,boost::shared_ptr<RBX::Instance>),std::string,boost::shared_ptr<RBX::Instance>,void>::call(RBX::Network::Players*,void (RBX::Network::Players::*)(std::string,boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_a4e388(players: &Players, method: PlayersStrInstMethod, message: &str, inst: &SharedPtr<Instance>) {
    // IDA 0xa4e388: member-pointer adjust for the `a3` tag bit, local
    // `string` copy of the message arg, retained `shared_ptr` copy of the
    // instance arg, the member call, then the mirrored `shared_ptr` and
    // string releases. Clones plus the call plus `Drop` are the same
    // sequence. Twin of 0xa47d30 with (string, Instance) args.
    let message = message.to_string();
    let inst = inst.clone();
    method(players, &message, &inst);
}

// 0xa4eedc — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEEiELi1EEC2EMS3_FS7_iEPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(int),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Network::Players::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_a4eedc(this: *mut PlayersIntRetFuncDesc, method: PlayersIntRetMethod, first: &str) {
    // IDA 0xa4eedc: `Players::classDescriptor` once-init, base
    // `FunctionDescriptor` init, member-pointer words, then one
    // `addArgument` call for the `int` arg. Twin of 0xa4752c/0xa4dcc0 with
    // a single int arg.
    // SAFETY: `this` must point to valid uninitialized `PlayersIntRetFuncDesc` storage.
    unsafe {
        core::ptr::write(
            this,
            PlayersIntRetFuncDesc {
                method,
                arg_names: [first.to_string()],
                signature: vec![SignatureItem { type_name: "int" }],
            },
        );
    }
}

// 0xa4f148 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEEiELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(int),1>::~BoundFuncDesc()
pub fn stub_a4f148(this: *mut PlayersIntRetFuncDesc) {
    // IDA 0xa4f148: D0 — D1 body plus `operator delete`; the box reclaim
    // runs the field drops and frees together. Same shape as 0xa47920.
    // SAFETY: `this` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(this));
    }
}

// 0xa4f244 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEEiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_a4f244(desc: &PlayersIntRetFuncDesc, players: &Players, args: &[Variant]) -> SharedPtr<Instance> {
    // IDA 0xa4f244: `source ? source - 36 : 0` member-signal select, one
    // `ArgHelper::getArg<int>` extract, then the member call; the returned
    // `shared_ptr<Instance>` lands in the `Arguments` return slot. Returning
    // it is the same value; the `shared_ptr` copies are `Drop`-managed here.
    assert!(args.len() == 1, "0xa4f244: Arguments must hold 1 value");
    let key = match &args[0] {
        Variant::Int(i) => *i,
        _ => panic!("0xa4f244: any_cast<int> failed"),
    };
    (desc.method)(players, key)
}

// 0xa4f4a0 — __ZN3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEEC2IMS3_KFPS4_vEiEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::Network::Players::*)(void)const,int>(char const*,char const*,RBX::Instance* (RBX::Network::Players::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_a4f4a0(this: *mut PlayersRefPropDesc, name: &str, category: &str, getter: PlayersRefGetter, flags: i32, attributes: u32, permissions: u32) {
    // IDA 0xa4f4a0: `Players::classDescriptor` once-init, base
    // `PropertyDescriptor` init with the name/category words, then the
    // `GetImpl<getter>` install at `+44` (the trailing int word rides along
    // as its template arg). The impl object collapses into the stored
    // function pointer here; the vtable words are compiler-managed.
    // SAFETY: `this` must point to valid uninitialized `PlayersRefPropDesc` storage.
    unsafe {
        core::ptr::write(
            this,
            PlayersRefPropDesc { name: name.to_string(), category: category.to_string(), getter, flags, attributes, permissions },
        );
    }
}

// 0xa4f6b4 — __ZN3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::~RefPropDescriptor()")]
pub fn stub_a4f6b4(this: *mut PlayersRefPropDesc) {
    // IDA 0xa4f6b4: D0 — two vtable resets (compiler-managed here), the
    // conditional `operator delete` of the heap word at `+11`, plus
    // `operator delete(this)`; the box reclaim frees the same allocation.
    // SAFETY: `this` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(this));
    }
}

// 0xa4f6e4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::isReadOnly(void)const")]
pub fn stub_a4f6e4(desc: &PlayersRefPropDesc) -> bool {
    // IDA 0xa4f6e4: delegates through the `+44` impl (`GetImpl::isReadOnly`,
    // IDA 0xa503b8), which returns `1` — the get-only impl has no setter.
    let _ = desc;
    stub_a503b8()
}

// 0xa4f6f4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::isWriteOnly(void)const")]
pub fn stub_a4f6f4(desc: &PlayersRefPropDesc) -> bool {
    // IDA 0xa4f6f4: delegates through the `+44` impl (`GetImpl::isWriteOnly`,
    // IDA 0xa503bc), which returns `0` — the getter always yields a value.
    let _ = desc;
    stub_a503bc()
}

// 0xa4f704 — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_a4f704(desc: &PlayersRefPropDesc, first: &Players, second: &Players) -> bool {
    // IDA 0xa4f704: `getRefValue` (IDA 0xa50018, i.e. `GetImpl::getValue` at
    // 0xa503c0 plus the `+36` DescribedBase adjust) on both sides, then the
    // raw pointer compare. `None` is the null return, equal only to `None`;
    // `SharedPtr::ptr_eq` is the word compare.
    match ((desc.getter)(first), (desc.getter)(second)) {
        (Some(a), Some(b)) => SharedPtr::ptr_eq(&a, &b),
        (None, None) => true,
        _ => false,
    }
}

// 0xa4f72c — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_a4f72c(desc: &PlayersRefPropDesc, players: &Players, out: &mut Variant) {
    // IDA 0xa4f72c: `getRefValue` plus `shared_from` retain into the
    // `Variant`. The retained clone is that same retain; a null return
    // assigns the empty `shared_ptr`, which collapses to `Variant::Null`.
    match (desc.getter)(players) {
        Some(inst) => *out = Variant::Instance(inst.clone()),
        None => *out = Variant::Null,
    }
}

// 0xa4fb04 — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_a4fb04(desc: &PlayersRefPropDesc, players: &Players, variant: &Variant) {
    // IDA 0xa4fb04: `Variant::get<shared_ptr<DescribedBase>>` any_cast, then
    // the virtual `setRefValue` (slot `+64` → IDA 0xa5002c); the temp
    // `shared_ptr` release is `Drop`-managed here.
    match variant {
        Variant::Instance(inst) => stub_a5002c(desc, players, inst),
        _ => panic!("0xa4fb04: any_cast<shared_ptr<DescribedBase>> failed"),
    }
}

// 0xa4fd20 — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_a4fd20(desc: &PlayersRefPropDesc, src: &Players, dst: &Players) {
    // IDA 0xa4fd20: `getValue` (`**(desc + 44) + 8`, IDA 0xa50018) on `src`,
    // then `setValue` (`**(desc + 44) + 12`, IDA 0xa500a8) on `dst` — which
    // throws for the get-only impl, so the get runs and the set panics.
    let value = stub_a50018(desc, src);
    stub_a500a8(desc, dst, value.as_ref());
}

// 0xa4fd4c — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_a4fd4c(desc: &PlayersRefPropDesc, players: &Players, _element: *mut XmlElement) -> Option<SharedPtr<Instance>> {
    // IDA 0xa4fd4c: `getRefValue` plus `InstanceHandle` construction, then
    // `XmlNameValuePair::clearValue(element + 12)` and boxing of the handle
    // into the value slot. The element value slot is unmodeled (`XmlElement`
    // carries tag/attrs/children only), so the clear+install collapses into
    // returning the retained handle for the caller to sink.
    // SAFETY: `_element` must point to a valid `XmlElement` (sink documented, unmodeled).
    let _ = _element;
    stub_a50018(desc, players)
}

// 0xa4fff4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_a4fff4(desc: &PlayersRefPropDesc, players: &Players, element: &XmlElement, binder: &mut ReferenceBinder) {
    // IDA 0xa4fff4: `element ? element + 12 : 0`, then the `IReferenceBinder`
    // virtual (slot `+4`) with the value, the object, and `desc + 40`. The
    // binder machinery is unmodeled, so the pending IDREF is recorded under
    // its `referent` name with a null target for later resolution.
    let _ = (desc, players);
    if let Some(attr) = element.find_attribute(ATTR_REFERENT) {
        binder.bind(attr.value.as_str(), std::ptr::null());
    }
}

// 0xa50018 — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_a50018(desc: &PlayersRefPropDesc, players: &Players) -> Option<SharedPtr<Instance>> {
    // IDA 0xa50018: `GetImpl::getValue` (IDA 0xa503c0) plus the `+36`
    // DescribedBase→Instance adjust on non-null; null stays null. The
    // adjust collapses; `None` is the null return.
    (desc.getter)(players)
}

// 0xa5002c — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_a5002c(desc: &PlayersRefPropDesc, players: &Players, value: &SharedPtr<Instance>) {
    // IDA 0xa5002c: `__dynamic_cast<Instance*>` of the value (always
    // succeeds for a `SharedPtr<Instance>`; a mismatch would throw
    // `std::bad_cast`), then `GetImpl::setValue` (IDA 0xa503e4), which
    // throws for the get-only impl.
    let _ = (desc, players, value);
    stub_a503e4()
}

// 0xa500a8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_a500a8(desc: &PlayersRefPropDesc, players: &Players, value: Option<&SharedPtr<Instance>>) {
    // IDA 0xa500a8: the `setRefValueUnsafe` path — same `GetImpl::setValue`
    // call as 0xa5002c without the `dynamic_cast` (null passes through to
    // the same throw).
    let _ = (desc, players, value);
    stub_a503e4()
}

// 0xa500c8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_a500c8(desc: &PlayersRefPropDesc, players: &Players, handle: Option<&SharedPtr<Instance>>) {
    // IDA 0xa500c8: retain the incoming handle, adjust to the
    // `DescribedBase` subobject, call `GetImpl::setValue` (same throw as
    // 0xa500a8), release on scope exit. The clone plus `Drop` is the
    // retain/release pair.
    let retained = handle.cloned();
    stub_a500a8(desc, players, retained.as_ref());
}

// 0xa50340 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk to RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_a50340(desc: &PlayersRefPropDesc, players: &Players, handle: Option<&SharedPtr<Instance>>) {
    // IDA 0xa50340: non-virtual thunk — `this - 40` adjusts back to the
    // `RefPropDescriptor` base, then tail-calls `assignIDREF` (IDA
    // 0xa500c8). The adjustment is a vtable-layout detail that collapses
    // away here.
    stub_a500c8(desc, players, handle);
}

// 0xa503b8 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network7PlayersEPNS_8InstanceEE7GetImplIMS3_KFS5_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::Network::Players::*)(void)const>::isReadOnly(void)const")]
pub fn stub_a503b8() -> bool {
    // IDA 0xa503b8: `GetImpl<Instance* (Players::*)() const>::isReadOnly` —
    // `return 1`; the get-only impl has no setter.
    true
}

// 0xa503bc — __ZNK3RBX10Reflection14PropDescriptorINS_7Network7PlayersEPNS_8InstanceEE7GetImplIMS3_KFS5_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::Network::Players::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_a503bc() -> bool {
    // IDA 0xa503bc: `GetImpl<Instance* (Players::*)() const>::isWriteOnly` —
    // `return 0`; the getter always yields a value.
    false
}

// 0xa503c0 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network7PlayersEPNS_8InstanceEE7GetImplIMS3_KFS5_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::Network::Players::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_a503c0(getter: PlayersRefGetter, players: &Players) -> Option<SharedPtr<Instance>> {
    // IDA 0xa503c0: `DescribedBase - 36` select, member-pointer tag-bit
    // adjust, then the `Instance* (Players::*)() const` call. Both adjusts
    // collapse; `None` is the null return.
    getter(players)
}

// 0xa503e4 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network7PlayersEPNS_8InstanceEE7GetImplIMS3_KFS5_vEE8setValueEPNS0_13DescribedBaseERKS5_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,RBX::Instance *>::GetImpl<RBX::Instance * (RBX::Network::Players::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")]
pub fn stub_a503e4() -> ! {
    // IDA 0xa503e4: throws `std::runtime_error("can't set value")` — the
    // get-only impl has no setter. Every `setRefValue*`/`assignIDREF` path
    // on this descriptor funnels here.
    panic!("0xa503e4: can't set value")
}

// 0xa50508 — __ZN3RBX10Reflection4TypeC2IPNS_8InstanceEEEPKcS6_PT_
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Instance *>(char const*,char const*,RBX::Instance * *)")]
pub fn stub_a50508(this: *mut InstancePtrType, category: &str, tag: &str) {
    // IDA 0xa50508: base `Descriptor::Descriptor` init, `typeinfo` install,
    // `Name::declare` of the tag, `ReleaseAssert(!tag.empty())` (Type.h:77),
    // then the `addToAllTypes` registry link. The typeinfo word and the
    // global registry are unmodeled; the tag assert stays. The trailing
    // `Instance **` tag-dispatch word collapses (overload resolution only).
    // SAFETY: `this` must point to valid uninitialized `InstancePtrType` storage.
    debug_assert!(!tag.is_empty(), "0xa50508: !this->tag.empty()");
    unsafe {
        core::ptr::write(this, InstancePtrType { category: category.to_string(), tag: tag.to_string() });
    }
}

// 0xa51ee0 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::_M_insert_unique(std::pair<RBX::Guid::Data const,RBX::Instance *> const&)")]
pub fn stub_a51ee0(tree: *mut GuidTree, key: &GuidData, value: *const Instance) -> (GuidIter, bool) {
    // IDA 0xa51ee0: descending walk from the root via
    // `Guid::Data::operator<` (see `guid_data_less`), `lower_bound` plus
    // `_Rb_tree_decrement` duplicate check, then node `operator new` plus
    // `_M_insert` rebalance on miss. The red-black words collapse into the
    // `HashMap`; the result is (iterator to the key, whether inserted).
    // SAFETY: `tree` must point to a valid `GuidTree`; `key` must be readable.
    unsafe {
        match (*tree).entry(*key) {
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(value);
                (Some(*key), true)
            }
            std::collections::hash_map::Entry::Occupied(_) => (Some(*key), false),
        }
    }
}

// 0xa53b04 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED2Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_a53b04(this: *mut PlayersInstRetFuncDesc) {
    // IDA 0xa53b04: `BoundFuncDesc<Players, shared_ptr<Instance>
    // (shared_ptr<Instance>), 1>` D2 — vtable resets (compiler-managed) plus
    // member drops (arg-name release, signature-list `_M_clear`); storage
    // kept. `drop_in_place` runs the same field drops without freeing.
    // SAFETY: `this` must point to a valid `PlayersInstRetFuncDesc` that is not used again.
    unsafe {
        core::ptr::drop_in_place(this);
    }
}

// 0xa53c54 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED2Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()
pub fn stub_a53c54(this: *mut PlayersChatFuncDesc) {
    // IDA 0xa53c54: `BoundFuncDesc<Players, void
    // (shared_ptr<Instance>, string, string), 3>` D2 — same vtable-reset +
    // member-drop sequence as 0xa53b04 on `PlayersChatFuncDesc`; storage kept.
    // SAFETY: `this` must point to a valid `PlayersChatFuncDesc` that is not used again.
    unsafe {
        core::ptr::drop_in_place(this);
    }
}

// 0xa53e38 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED2Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
pub fn stub_a53e38(this: *mut PlayersStrInstFuncDesc) {
    // IDA 0xa53e38: `BoundFuncDesc<Players, void (string,
    // shared_ptr<Instance>), 2>` D2 — same vtable-reset + member-drop
    // sequence as 0xa53b04 on `PlayersStrInstFuncDesc`; storage kept.
    // SAFETY: `this` must point to a valid `PlayersStrInstFuncDesc` that is not used again.
    unsafe {
        core::ptr::drop_in_place(this);
    }
}

// 0xa80f18 — __ZN3RBX7Network6Player17requestFriendshipEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Player::requestFriendship(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Player::requestFriendship(boost::shared_ptr<RBX::Instance>)
pub fn stub_a80f18(player: &SharedPtr<Instance>) {
    // IDA 0xa80f18: `fastSharedDynamicCast<Player, Instance>` — a non-`Player`
    // throws `runtime_error("RequestFriendship should be passed a Player")`;
    // else `signal_with_args<2,bool,int>::operator()` fires `(true, userId)`
    // (`+156`) and `RemoteEventDescImpl::replicateEvent` ships it. The cast
    // collapses into `instance_is_a`; the retain/release pair into the
    // clone/`Drop`. The signal fan-out and replication are unmodeled network
    // routing.
    // // BUG: the friendship request never reaches the server — the event
    // // fire plus `replicateEvent` are no-ops until the remote-event port lands.
    let raw: *const Instance = &**player;
    if !instance_is_a(raw, "Player") {
        panic!("0xa80f18: RequestFriendship should be passed a Player");
    }
    let _retained = player.clone();
}

// 0xa81364 — __ZN3RBX7Network6Player16revokeFriendshipEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Player::revokeFriendship(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Player::revokeFriendship(boost::shared_ptr<RBX::Instance>)
pub fn stub_a81364(player: &SharedPtr<Instance>) {
    // IDA 0xa81364: same shape as 0xa80f18 (`fastSharedDynamicCast<Player>`,
    // `(true, userId)` fire, `replicateEvent`) with
    // `runtime_error("RevokeFriendship should be passed a Player")` on a
    // non-`Player`.
    // // BUG: the revocation never reaches the server — same unmodeled
    // // fire/`replicateEvent` as 0xa80f18.
    let raw: *const Instance = &**player;
    if !instance_is_a(raw, "Player") {
        panic!("0xa81364: RevokeFriendship should be passed a Player");
    }
    let _retained = player.clone();
}

// 0xa83044 — __ZN3RBX7Network6Player12saveInstanceESsN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Player::saveInstance(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Player::saveInstance(std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_a83044(player: &Player, key: &str, inst: &SharedPtr<Instance>) {
    // IDA 0xa83044: `Players::backendProcessing` gate → `runtime_error("LocalScripts
    // cannot use SaveInstance")`; `+116` data-loaded gate → `runtime_error("Data for
    // player not yet loaded, wait for DataReady")`; `PersistentDataStore` write;
    // null write result → `runtime_error("Exceeded DataComplexity limit for Instance
    // key %s")`; `raisePropertyChanged`. The backend gate, the store write, and the
    // property-changed fire are unmodeled service state; the data-loaded gate is real.
    // // BUG: nothing is persisted and no property-changed event fires — the
    // // PersistentDataStore routing is unmodeled.
    if !player.data_loaded {
        panic!("0xa83044: Data for player not yet loaded, wait for DataReady");
    }
    let _retained = inst.clone();
    let _ = key;
}

// 0xa851a8 — __ZN3RBX7Network6Player29loadCharacterAppearanceScriptEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Player::loadCharacterAppearanceScript(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Player::loadCharacterAppearanceScript(boost::shared_ptr<RBX::Instance>)
pub fn stub_a851a8(player: &SharedPtr<Player>, inst: &SharedPtr<Instance>) {
    // IDA 0xa851a8: `weak_from<Player>` plus `weak_from<Instance>`, then
    // `setAppearanceParent(playerWeak, instWeak, false)` (IDA 0xa8d6b4). The
    // weak pair is the synchronous retain collapse below.
    stub_a8d6b4(&SharedPtr::downgrade(player), &SharedPtr::downgrade(inst), false);
}

// 0xa8572c — __ZN3RBX7Network6Player15getFriendStatusEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Player::getFriendStatus(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Player::getFriendStatus(boost::shared_ptr<RBX::Instance>)
pub fn stub_a8572c(player: &SharedPtr<Instance>, other: &SharedPtr<Instance>) -> FriendStatus {
    // IDA 0xa8572c: provider → `FriendService` lookup, both userIds, then
    // `FriendService::getFriendStatus`; a missing service yields 0. The
    // service lookup and the `+39`/`+156` userId fields are unmodeled — the
    // miss path is the conservative default.
    // // BUG: always reports 0 — FriendService routing unmodeled.
    let _ = (player, other);
    FriendStatus(0)
}

// 0xa87d44 — __ZN3RBX7Network6Player27physicsOutBandwidthExceededEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Player::physicsOutBandwidthExceeded(RBX::Instance const*)")]
pub fn stub_a87d44(player: &SharedPtr<Instance>, inst: &SharedPtr<Instance>) -> bool {
    // IDA 0xa87d44: tail-calls `Client::physicsOutBandwidthExceeded` (IDA
    // 0x9688b4) — `findClient(inst, true)`, then
    // `findConstFirstChildOfType<ClientReplicator>` and its flag word; null
    // at either step yields 0. The `Client`/replicator lookup is
    // network-crate state, unmodeled here — the miss path stays.
    // // BUG: always reports false — Client lookup unmodeled.
    let _ = (player, inst);
    false
}

// 0xa87d50 — __ZN3RBX7Network6Player22getNetworkBufferHealthEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Player::getNetworkBufferHealth(RBX::Instance const*)")]
pub fn stub_a87d50(player: &SharedPtr<Instance>, inst: &SharedPtr<Instance>) -> i32 {
    // IDA 0xa87d50: tail-calls `Client::getNetworkBufferHealth` with the same
    // `findClient` + `ClientReplicator` chain as 0xa87d44. Same unmodeled
    // lookup, same miss-path default.
    // // BUG: always reports 0 — Client lookup unmodeled.
    let _ = (player, inst);
    0
}

// 0xa8d6b4 — __ZN3RBX7Network6Player19setAppearanceParentEN5boost8weak_ptrIS1_EENS3_INS_8InstanceEEEb
#[doc(alias = "RBX::Network::Player::setAppearanceParent(rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::Instance>,bool)")]
// was: RBX::Network::Player::setAppearanceParent(boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::Instance>,bool)
pub fn stub_a8d6b4(player: &WeakPtr<Player>, appearance: &WeakPtr<Instance>, flag: bool) {
    // IDA 0xa8d6b4 (39KB): upgrades both weaks (expired → return), then the
    // appearance orchestration — Humanoid search, accessory/apparel
    // reparenting via `setParentInternal`, Humanoid wiring. The orchestration
    // touches Humanoid/character state unmodeled here.
    // // BUG: appearance is never reparented — the orchestration is unmodeled.
    if player.upgrade().is_none() || appearance.upgrade().is_none() {
        return;
    }
    let _ = flag;
}

// 0xa8e498 — __ZL23setAppearanceParentNullN5boost10shared_ptrIN3RBX8InstanceEEE
#[doc(alias = "setAppearanceParentNull(rbx_core::SharedPtr<RBX::Instance>)")]
// was: setAppearanceParentNull(boost::shared_ptr<RBX::Instance>)
pub fn stub_a8e498(inst: &SharedPtr<Instance>) {
    // IDA 0xa8e498: null `shared_ptr` returns at once (unspellable here —
    // only the live path is modeled); else `CharacterAppearance`
    // `classDescriptor` once-init + `isA` — on match,
    // `setParentInternal(inst, null, false)` unparents it.
    // Callers hold both trees via `SharedPtr`; `stub_6ffc98` takes raw pointers
    // but is a safe fn (its own contract, see `generated_86`).
    if instance_is_a(SharedPtr::as_ptr(inst), "CharacterAppearance") {
        let _ = stub_6ffc98(SharedPtr::as_ptr(inst) as *mut Instance, std::ptr::null(), false);
    }
}

// 0xa8e5f4 — __ZL29setAppearanceParentNullScriptN5boost10shared_ptrIN3RBX8InstanceEEE
#[doc(alias = "setAppearanceParentNullScript(rbx_core::SharedPtr<RBX::Instance>)")]
// was: setAppearanceParentNullScript(boost::shared_ptr<RBX::Instance>)
pub fn stub_a8e5f4(inst: &SharedPtr<Instance>) {
    // IDA 0xa8e5f4: null returns at once; `CharacterAppearance isA` →
    // unparent (same `LABEL_17` as 0xa8e498); else `__dynamic_cast<IEquipable*>`
    // plus the `Tool isA` chain — a `Tool` keeps its parent for the backpack
    // branch below, while the IEquipable/backpack-parent lookup deciding the
    // remaining arms is unmodeled service state.
    // // BUG: `Tool`s and non-Appearance equipables keep their parent — the
    // // IEquipable/backpack branch is unmodeled.
    // Same ownership contract as `stub_a8e498`.
    let raw = SharedPtr::as_ptr(inst);
    if instance_is_a(raw, "CharacterAppearance") {
        let _ = stub_6ffc98(raw as *mut Instance, std::ptr::null(), false);
    }
}

// 0xa90080 — __ZL24makeAccoutrementRequestsPSsPSt9exceptionN5boost8weak_ptrIN3RBX7Network6PlayerEEENS3_INS4_9DataModelEEE
#[doc(alias = "makeAccoutrementRequests(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>)")]
// was: makeAccoutrementRequests(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>)
pub fn stub_a90080(url: &str, player: &WeakPtr<Player>, data_model: &WeakPtr<DataModel>) {
    // IDA 0xa90080 (12KB): upgrades both weaks (expired → return), builds the
    // accoutrement URL request, enqueues it on the `AsyncHttpQueue` with a
    // `doMakeAccoutrementRequests` bind (IDA 0xa946c0); thrown errors funnel
    // to `StandardOut::printf("Make Accoutrement Request exception: %s")`.
    // The queue machinery is unmodeled.
    // // BUG: no request is ever enqueued — appearance fetch unmodeled.
    if player.upgrade().is_none() || data_model.upgrade().is_none() {
        return;
    }
    let _ = url;
}

// 0xa91484 — __ZL8addChildRKN5boost10shared_ptrIN3RBX13ModelInstanceEEERKNS0_INS1_8InstanceEEE
#[doc(alias = "addChild(rbx_core::SharedPtr<RBX::ModelInstance> const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: addChild(boost::shared_ptr<RBX::ModelInstance> const&,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_a91484(model: &SharedPtr<ModelInstance>, child: &SharedPtr<Instance>) -> bool {
    // IDA 0xa91484: `setParentInternal(child, model, false)`; the bool result
    // propagates (`setParentInternal` panics mirror the C++ throws). Callers
    // hold both trees via `SharedPtr`; `stub_6ffc98` is a safe fn.
    stub_6ffc98(SharedPtr::as_ptr(child) as *mut Instance, SharedPtr::as_ptr(model) as *const Instance, false)
}

// 0xa91498 — __ZL19CharacterLoadHelperN5boost10shared_ptrIN3RBX13ModelInstanceEEENS1_14AsyncHttpQueue13RequestResultENS0_ISt6vectorINS0_INS1_8InstanceEEESaIS8_EEEE
#[doc(alias = "CharacterLoadHelper(rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
// was: CharacterLoadHelper(boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)
pub fn stub_a91498(model: &SharedPtr<ModelInstance>, result: HttpRequestResult, instances: &SharedPtr<Vec<SharedPtr<Instance>>>) {
    // IDA 0xa91498: `RequestResult != 1` returns at once (the failure path
    // only drops the binds); on success the model is retained and `addChild`
    // (IDA 0xa91484) runs over the loaded vector via `for_each`+`bind`.
    // Clones plus `Drop` are the retains/releases.
    if result != 1 {
        return;
    }
    for child in instances.iter() {
        stub_a91484(model, child);
    }
}

// 0xa91a80 — __ZN3RBX7Network6Player19characterChildAddedEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Player::characterChildAdded(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Player::characterChildAdded(boost::shared_ptr<RBX::Instance>)
pub fn stub_a91a80(player: &Player, child: &SharedPtr<Instance>) {
    // IDA 0xa91a80: null child returns at once (unspellable — only the live
    // path is modeled); `Humanoid classDescriptor` once-init + `isA` — on
    // match, `shared_from<Humanoid>` plus `setupHumanoid(player, humanoid)`
    // and `connection::disconnect(+280)` (one-shot). The Humanoid wiring and
    // the connection live in unmodeled character state.
    // // BUG: Humanoid setup never runs — setupHumanoid/connection unmodeled.
    let _ = player;
    if !instance_is_a(SharedPtr::as_ptr(child), "Humanoid") {
        return;
    }
    let _retained = child.clone();
}

// 0xa91d80 — __Z22copyChildrenToBackpackPN3RBX8InstanceES1_
#[doc(alias = "copyChildrenToBackpack(RBX::Instance *,RBX::Instance *)")]
pub fn stub_a91d80(src: &SharedPtr<Instance>, dst: &SharedPtr<Instance>) {
    // IDA 0xa91d80: walks the children vector (`ReleaseAssert`s on the
    // slots, unspellable nulls aside), `clone`s each child
    // (`Instance::clone`, IDA 0x701240 with role 3) and `setParentInternal`s
    // the clone into the backpack (a2). `setParentInternal` panics mirror
    // the C++ throws.
    // SAFETY: both trees must outlive the call with caller-held ownership.
    let kids: Vec<SharedPtr<Instance>> = unsafe { (*SharedPtr::as_ptr(src)).children.clone() };
    for child in kids.iter() {
        let raw: *const Instance = &**child;
        if let Some(copy) = stub_0x701240(raw, CreatorRole(3)) {
            let _ = stub_6ffc98(SharedPtr::as_ptr(&copy) as *mut Instance, SharedPtr::as_ptr(dst) as *const Instance, false);
        }
    }
}

// 0xa921a8 — __ZNK3RBX7Network6Player15verifySetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Player::verifySetParent(RBX::Instance const*)const")]
pub fn stub_a921a8(player: &Player, new_parent: *const Instance) {
    // IDA 0xa921a8: null parent returns at once; else the new parent's
    // descriptor `isA Players` — mismatch throws `runtime_error("Parent of
    // Player can not be changed")`.
    // SAFETY: `new_parent` must be null or point to a valid `Instance`.
    let _ = player;
    if new_parent.is_null() {
        return;
    }
    if !instance_is_a(new_parent, "Players") {
        panic!("0xa921a8: Parent of Player can not be changed");
    }
}

// 0xa9233c — __ZN3RBX7Network6Player21onFriendStatusChangedEN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusE
#[doc(alias = "RBX::Network::Player::onFriendStatusChanged(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)")]
// was: RBX::Network::Player::onFriendStatusChanged(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)
pub fn stub_a9233c(player: &Player, inst: &SharedPtr<Instance>, status: FriendStatus) {
    // IDA 0xa9233c: retains the instance, fires the member
    // `signal_with_args<2,(shared_ptr<Instance>, FriendStatus)>` with the
    // pair, releases. Clone plus `fire` plus `Drop` is the same sequence.
    stub_a9aafc(&player.friend_status_changed, inst, status);
}

// 0xa946c0 — __ZL26doMakeAccoutrementRequestsSsN5boost8weak_ptrIN3RBX7Network6PlayerEEENS0_INS1_9DataModelEEE
#[doc(alias = "doMakeAccoutrementRequests(std::string,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>)")]
// was: doMakeAccoutrementRequests(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>)
pub fn stub_a946c0(url: &str, player: &WeakPtr<Player>, data_model: &WeakPtr<DataModel>) {
    // IDA 0xa946c0 (24KB): the `AsyncHttpQueue` completion behind 0xa90080 —
    // upgrades the weak pair (expired → return), parses the accoutrement XML
    // payload, creates/clones appearance instances and reparents them into
    // the character. Queue + XML + Humanoid state unmodeled.
    // // BUG: the completion does nothing — appearance assembly unmodeled.
    if player.upgrade().is_none() || data_model.upgrade().is_none() {
        return;
    }
    let _ = url;
}

// 0xa957f0 — __ZL16doLoadAppearanceN5boost8weak_ptrIN3RBX7Network6PlayerEEENS1_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS7_INS1_8InstanceEEESaISA_EEEESsbd
#[doc(alias = "doLoadAppearance(rbx_core::Weak<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double)")]
// was: doLoadAppearance(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double)
pub fn stub_a957f0(player: &WeakPtr<Player>, result: HttpRequestResult, instances: &SharedPtr<Vec<SharedPtr<Instance>>>, name: &str, flag: bool, timeout: f64) {
    // IDA 0xa957f0 (9KB): appearance-load completion — weak upgrade (expired
    // → return), result check, appearance application onto the character.
    // Queue + Humanoid state unmodeled.
    // // BUG: the completion does nothing — appearance application unmodeled.
    if player.upgrade().is_none() {
        return;
    }
    let _ = (result, instances, name, flag, timeout);
}

// 0xa965a8 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_a965a8(this: *mut PlayerInstFuncDesc) {
    // IDA 0xa965a8: `BoundFuncDesc<Player, void (shared_ptr<Instance>), 1>`
    // D2 — vtable resets (compiler-managed) plus member drops (arg-name
    // release, signature-list `_M_clear`); storage kept. `drop_in_place`
    // runs the same field drops without freeing.
    // SAFETY: `this` must point to a valid `PlayerInstFuncDesc` that is not used again.
    unsafe {
        core::ptr::drop_in_place(this);
    }
}

// 0xa965fc — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()
pub fn stub_a965fc(this: *mut PlayerVoidRetFuncDesc) {
    // IDA 0xa965fc: `BoundFuncDesc<Player, shared_ptr<Instance> (void), 0>`
    // D2 — same vtable-reset + member-drop sequence as 0xa965a8 on
    // `PlayerVoidRetFuncDesc`; storage kept.
    // SAFETY: `this` must point to a valid `PlayerVoidRetFuncDesc` that is not used again.
    unsafe {
        core::ptr::drop_in_place(this);
    }
}

// 0xa969b0 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()
pub fn stub_a969b0(this: *mut PlayerStrRetFuncDesc) {
    // IDA 0xa969b0: `BoundFuncDesc<Player, shared_ptr<Instance> (string), 1>`
    // D2 — same vtable-reset + member-drop sequence as 0xa965a8 on
    // `PlayerStrRetFuncDesc`; storage kept.
    // SAFETY: `this` must point to a valid `PlayerStrRetFuncDesc` that is not used again.
    unsafe {
        core::ptr::drop_in_place(this);
    }
}

// 0xa96a58 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
pub fn stub_a96a58(this: *mut PlayerStrInstFuncDesc) {
    // IDA 0xa96a58: `BoundFuncDesc<Player, void (string,
    // shared_ptr<Instance>), 2>` D2 — vtable resets (compiler-managed) plus
    // member drops (arg-name releases, signature-list `_M_clear`); storage
    // kept. `drop_in_place` runs the same field drops without freeing.
    // SAFETY: `this` must point to a valid `PlayerStrInstFuncDesc` that is not used again.
    unsafe {
        core::ptr::drop_in_place(this);
    }
}

// 0xa96cc4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFNS_13FriendService12FriendStatusEN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_a96cc4(this: *mut PlayerInstStatusFuncDesc) {
    // IDA 0xa96cc4: `BoundFuncDesc<Player, FriendStatus
    // (shared_ptr<Instance>), 1>` D2 — same vtable-reset + member-drop
    // sequence as 0xa96a58 on `PlayerInstStatusFuncDesc`; storage kept.
    // SAFETY: `this` must point to a valid `PlayerInstStatusFuncDesc` that is not used again.
    unsafe {
        core::ptr::drop_in_place(this);
    }
}

// 0xa96ec8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()
pub fn stub_a96ec8(this: *mut EventDescPayload) {
    // IDA 0xa96ec8: `EventDesc<Player, void (string, shared_ptr<Instance>)>`
    // D0 — the D1 body (vtable reset + signature-list `_M_clear`) plus
    // `operator delete`; the box reclaim runs the field drops and frees
    // together. Same shape as 0xa4a0a0.
    // SAFETY: `this` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(this));
    }
}

// 0xa96f10 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()
pub fn stub_a96f10(this: *mut EventDescPayload) {
    // IDA 0xa96f10: `EventDesc<Player, void (shared_ptr<Instance>)>` D0 —
    // same D1-plus-delete shape as 0xa96ec8.
    // SAFETY: `this` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(this));
    }
}

// 0xa96fa0 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::~EventDesc()
pub fn stub_a96fa0(this: *mut EventDescPayload) {
    // IDA 0xa96fa0: `EventDesc<Player, void (shared_ptr<Instance>,
    // FriendStatus)>` D0 — same D1-plus-delete shape as 0xa96ec8.
    // SAFETY: `this` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(this));
    }
}

// 0xa98f78 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network6PlayerEEENS1_INS2_8InstanceEEEbS5_NS_3argILi1EEEbEENS_3_bi6bind_tIT_PFSC_T0_T1_T2_ENSA_9list_av_3IT3_T4_T5_E4typeEEESH_SJ_SK_SL_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::Instance>,bool),boost::_bi::list_av_3<rbx_core::Weak<RBX::Network::Player>,boost::arg<1>,bool>::type> boost::bind<void,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::Instance>,bool,rbx_core::Weak<RBX::Network::Player>,boost::arg<1>,bool>(void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::Instance>,bool),rbx_core::Weak<RBX::Network::Player>,boost::arg<1>,bool)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::Instance>,bool),boost::_bi::list_av_3<boost::weak_ptr<RBX::Network::Player>,boost::arg<1>,bool>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::Instance>,bool,boost::weak_ptr<RBX::Network::Player>,boost::arg<1>,bool>(void (*)(boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::Instance>,bool),boost::weak_ptr<RBX::Network::Player>,boost::arg<1>,bool)
pub fn stub_a98f78(player: &WeakPtr<Player>, flag: bool) -> PlayerAppearanceBind {
    // IDA 0xa98f78: `boost::bind(setAppearanceParent, playerWeak, _1, flag)`
    // — the player weak is retained into the `bind_t` (the `weak_count`
    // copy) while the appearance arg stays late-bound and the bool is a
    // value. The retained clone is that same copy. Twin of 0xa4b0f4 with an
    // extra bool value word.
    PlayerAppearanceBind { player: player.clone(), flag }
}

// 0xa99284 — __ZN5boost4bindIvPSsPSt9exceptionNS_8weak_ptrIN3RBX7Network6PlayerEEENS4_INS5_9DataModelEEENS_3argILi1EEENSB_ILi2EEES8_SA_EENS_3_bi6bind_tIT_PFSG_T0_T1_T2_T3_ENSE_9list_av_4IT4_T5_T6_T7_E4typeEEESM_SO_SP_SQ_SR_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>>::type> boost::bind<void,std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>,boost::arg<1>,boost::arg<2>,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>>(void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::arg<1>,boost::arg<2>,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>)")]
// was: boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>>::type> boost::bind<void,std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>,boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>>(void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>)
pub fn stub_a99284(player: &WeakPtr<Player>, data_model: &WeakPtr<DataModel>) -> PlayerDataModelBind {
    // IDA 0xa99284: `boost::bind(makeAccoutrementRequests, _1, _2,
    // playerWeak, dmWeak)` — both weaks retained into the `bind_t` while the
    // string/exception args stay late-bound. Same collapse as 0xa98f78.
    PlayerDataModelBind { player: player.clone(), data_model: data_model.clone() }
}

// 0xa99cb4 — __ZN5boost4bindIvRKNS_10shared_ptrIN3RBX13ModelInstanceEEERKNS1_INS2_8InstanceEEES4_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_ENSD_9list_av_2IT2_T3_E4typeEEESJ_SL_SM_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::ModelInstance> const&,rbx_core::SharedPtr<RBX::Instance> const&),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::ModelInstance>,boost::arg<1>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::ModelInstance> const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::ModelInstance>,boost::arg<1>>(void (*)(rbx_core::SharedPtr<RBX::ModelInstance> const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::ModelInstance>,boost::arg<1>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::ModelInstance> const&,boost::shared_ptr<RBX::Instance> const&),boost::_bi::list_av_2<boost::shared_ptr<RBX::ModelInstance>,boost::arg<1>>::type> boost::bind<void,boost::shared_ptr<RBX::ModelInstance> const&,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::ModelInstance>,boost::arg<1>>(void (*)(boost::shared_ptr<RBX::ModelInstance> const&,boost::shared_ptr<RBX::Instance> const&),boost::shared_ptr<RBX::ModelInstance>,boost::arg<1>)
pub fn stub_a99cb4(model: &SharedPtr<ModelInstance>) -> ModelChildBind {
    // IDA 0xa99cb4: `boost::bind(addChild, model, _1)` — the model
    // `shared_ptr` is retained into the `bind_t` (the `shared_count` copy)
    // while the child stays late-bound. Twin of 0x70825c on `ModelInstance`.
    ModelChildBind { model: model.clone() }
}

// 0xa9a314 — __ZN5boost4bindIvNS_10shared_ptrIN3RBX13ModelInstanceEEENS2_14AsyncHttpQueue13RequestResultENS1_ISt6vectorINS1_INS2_8InstanceEEESaIS9_EEEES4_NS_3argILi1EEENSD_ILi2EEEEENS_3_bi6bind_tIT_PFSI_T0_T1_T2_ENSG_9list_av_3IT3_T4_T5_E4typeEEESN_SP_SQ_SR_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::ModelInstance>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::ModelInstance>,boost::arg<1>,boost::arg<2>>(void (*)(rbx_core::SharedPtr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>),rbx_core::SharedPtr<RBX::ModelInstance>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::_bi::list_av_3<boost::shared_ptr<RBX::ModelInstance>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::ModelInstance>,boost::arg<1>,boost::arg<2>>(void (*)(boost::shared_ptr<RBX::ModelInstance>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>),boost::shared_ptr<RBX::ModelInstance>,boost::arg<1>,boost::arg<2>)
pub fn stub_a9a314(model: &SharedPtr<ModelInstance>) -> ModelLoadBind {
    // IDA 0xa9a314: `boost::bind(CharacterLoadHelper, model, _1, _2)` — the
    // model retain with two late-bound args. Same collapse as 0xa99cb4.
    ModelLoadBind { model: model.clone() }
}

// 0xa9aafc — __ZN3rbx7signals16signal_with_argsILi2EFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEEEclES6_S8_
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>::operator()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)")]
// was: rbx::signals::signal_with_args<2,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>::operator()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)
pub fn stub_a9aafc(sig: &Signal<(SharedPtr<Instance>, FriendStatus)>, inst: &SharedPtr<Instance>, status: FriendStatus) {
    // IDA 0xa9aafc: `signal_with_args<2,(shared_ptr<Instance>,
    // FriendStatus)>::operator()` retains the instance arg (`shared_count`
    // copy), fans out to each connected slot's `callable::call`, releases on
    // scope exit. Clone plus `fire` plus `Drop` is the same sequence; the
    // per-slot retain/release pair rides on the fan-out.
    sig.fire((inst.clone(), status));
}

// 0xa9be60 — __ZNK3RBX7Network6Player11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Player::askAddChild(RBX::Instance const*)const")]
pub fn stub_a9be60(_player: &Player, _child: *const Instance) -> bool {
    // IDA 0xa9be60: `MOVS R0,#1` — a `Player` accepts any child.
    // SAFETY: `_child` must be null or point to a valid `Instance` (unchecked, as in C++).
    true
}

// 0xaa1e28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PlayerMouseENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerMouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_aa1e28(_block: *mut ControlBlockPd<PlayerMouse, CreatableInstanceDeleter>) {
    // IDA 0xaa1e28: `BX LR` — empty; the vtable reset is compiler-managed and
    // storage is released by the D0/owner path. Same shape as 0xf198.
}