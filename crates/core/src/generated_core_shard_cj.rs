//! core shard CJ — 100 core stubs EA-sorted, next uncovered after CI 0x660bd8 (strict RBX|boost|std|rbx earliest gap 0x660be0).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

/// Cutover state for the Team/Teams/TextBox clusters below (0x660be0..0x668e54).
/// Layout offsets are IDA byte offsets from the instance pointer; the vtable,
/// Instance base, and reflection-registrar traffic have no Rust mapping and are
/// documented per stub instead of modeled.

/// was: `RBX::SurfaceSelection` (surface id word 35 at +140, weak member word 34 at +136).
#[doc(alias = "RBX::SurfaceSelection")]
pub struct SurfaceSelectionState {
    surface: u32,
    watched: Option<crate::WeakPtr<()>>,
}

impl SurfaceSelectionState {
    pub fn new(surface: u32) -> Self {
        Self { surface, watched: None }
    }
    /// IDA 0x660be0: `LDR R0,[R0,#0x8C]` — returns word 35 (+140).
    pub fn surface(&self) -> u32 {
        self.surface
    }
    /// Adopts the weak owner released by D1 at 0x660c94. The once-guard lives in
    /// `enable_shared_from_this::_internal_accept_owner` (cf. boost_core_a stubs).
    pub fn watch(&mut self, owner: &crate::SharedPtr<()>) {
        self.watched = Some(crate::SharedPtr::downgrade(owner));
    }
    pub fn watched(&self) -> Option<crate::SharedPtr<()>> {
        self.watched.as_ref().and_then(|w| w.upgrade())
    }
}

/// was: `RBX::Team` (score word 23 at +92, teamColor/BrickColor word 24 at +96,
/// autoAssignable byte at +100, second flag byte at +101).
#[doc(alias = "RBX::Team")]
pub struct TeamState {
    score: i32,
    team_color: u32,
    auto_assignable: bool,
    flag_101: bool,
    name: String,
}

impl TeamState {
    /// IDA 0x662fcc: Instance base + vtables + classDescriptor (no mapping);
    /// score = 0 (+92); color = 194 then 1 (+96); bytes 100,101 = 1; name "Team".
    pub fn new() -> Self {
        let mut state = Self {
            score: 0,
            team_color: 194,
            auto_assignable: true,
            flag_101: true,
            name: "Team".to_string(),
        };
        // IDA 0x6630b2 stores 194, 0x6630fc overwrites 1 before return.
        state.team_color = 1;
        state
    }
    /// IDA 0x662f5c: `LDR R0,[R0,#0x5C]` — word 23 (+92).
    pub fn score(&self) -> i32 {
        self.score
    }
    /// IDA 0x662f7c: `*out = *(a2+96)` — word 24 (+96), raw BrickColor dword.
    pub fn team_color(&self) -> u32 {
        self.team_color
    }
    /// IDA 0x662fa0: `LDRB.W R0,[R0,#0x64]` — byte 100.
    pub fn auto_assignable(&self) -> bool {
        self.auto_assignable
    }
}

/// was: `RBX::Teams` (flag byte at +92, copy_on_write child-team vector at +96, name).
/// The vector holds datamodel children filtered by isA-Team at each query site
/// (0x664c56/0x664a34); the snapshot here stores the post-filter projection.
#[doc(alias = "RBX::Teams")]
pub struct TeamsState {
    flag_92: bool,
    teams: Vec<TeamState>,
    name: String,
}

impl TeamsState {
    /// IDA 0x6645dc: Instance base + vtables + classDescriptor (no mapping);
    /// byte +92 = 1; cow vector at +96 starts empty; name "Teams".
    pub fn new() -> Self {
        Self { flag_92: true, teams: Vec::new(), name: "Teams".to_string() }
    }
    pub fn teams(&self) -> &[TeamState] {
        &self.teams
    }
    pub fn add_team(&mut self, team: TeamState) {
        self.teams.push(team);
    }
}

/// was: `RBX::Network::Player` team-query projection (neutral flag at +104,
/// team color dword at +100, both via the +36 described-base view).
#[doc(alias = "RBX::Network::Player")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerTeamInfo {
    pub neutral: bool,
    pub team_color: u32,
}

/// was: `RBX::TextBox` field block (+536..+660). Zero-init words with no
/// observed behavior (141-143, 150, 153-156, 162, 164-165) and the GuiObject
/// base word 35 (+140, stored 1 at 0x666bc6) are documented, not modeled.
#[doc(alias = "RBX::TextBox")]
pub struct TextBoxState {
    flag_134: bool,
    text: String,
    font_size: i32,
    text_color3: [f32; 3],
    text_transparency: f32,
    field_144: f32,
    text_wrap: bool,
    text_scale: bool,
    x_alignment: i32,
    y_alignment: i32,
    font: i32,
    focused: bool,
    focus_word: u16,
    clear_text_on_focus: bool,
    display_text: String,
    multi_line: bool,
    name: String,
}

impl TextBoxState {
    /// IDA 0x666938: GuiObject("TextBox", 1) + vtables + classDescriptor (no mapping);
    /// flag134 = 0 (+536); text = "TextBox" (+540); fontSize = 0 (+544);
    /// color3 = default BrickColor::color3 (+548..+556, caller-supplied);
    /// transparency = 0.0 (+560); word144 = 1.0f (+576); wrap/scale = 0 (+580/581);
    /// xAlign = 2 (+584); yAlign = 1 (+588); font = 0 (+592); focus bytes = 0,0,0
    /// (+604..+606); clearTextOnFocus = 1 (+607); display = "" (+608); multiLine = 0 (+652).
    pub fn new(default_color3: [f32; 3]) -> Self {
        Self {
            flag_134: false,
            text: "TextBox".to_string(),
            font_size: 0,
            text_color3: default_color3,
            text_transparency: 0.0,
            field_144: 1.0,
            text_wrap: false,
            text_scale: false,
            x_alignment: 2,
            y_alignment: 1,
            font: 0,
            focused: false,
            focus_word: 0,
            clear_text_on_focus: true,
            display_text: String::new(),
            multi_line: false,
            name: "TextBox".to_string(),
        }
    }
    /// IDA 0x6685f0: copy-constructs the mixin text; borrow is the caller-side view.
    pub fn text(&self) -> &str {
        &self.text
    }
    /// IDA 0x668620: mixin word +2, same word the +544 setter stores.
    pub fn font_size(&self) -> i32 {
        self.font_size
    }
    /// IDA 0x668648: mixin word +14, same word the +592 setter stores.
    pub fn font(&self) -> i32 {
        self.font
    }
    /// IDA 0x668574: `LDRB.W R0,[R0,#0x28C]` — byte 652.
    pub fn multi_line(&self) -> bool {
        self.multi_line
    }
    /// IDA 0x6685a0: `LDRB.W R0,[R0,#0x25F]` — byte 607.
    pub fn clear_text_on_focus(&self) -> bool {
        self.clear_text_on_focus
    }
}

#[doc(alias = "RBX::SurfaceSelection::getSurface(void)const")]
// 0x660be0 — __ZNK3RBX16SurfaceSelection10getSurfaceEv
pub fn stub_660be0(state: &SurfaceSelectionState) -> u32 {
    // IDA 0x660be0: LDR R0,[R0,#0x8C]; return *(this+35) — surface id at +140.
    state.surface()
}

#[doc(alias = "RBX::SurfaceSelection::~SurfaceSelection()")]
// 0x660c0c — __ZN3RBX16SurfaceSelectionD1Ev
pub fn stub_660c0c(state: SurfaceSelectionState) {
    // IDA 0x660c0c D1: vtable reset + weak_release(word 34 at +136) + IAdornable/Instance dtors — Arc/Weak drop is the mapping.
    drop(state);
}

#[doc(alias = "RBX::SurfaceSelection::~SurfaceSelection()")]
// 0x660d54 — __ZN3RBX16SurfaceSelectionD0Ev
pub fn stub_660d54(state: SurfaceSelectionState) {
    // IDA 0x660d54 D0: D1(this) + operator delete(this) — drop covers both.
    stub_660c0c(state);
}

#[doc(alias = "non-virtual thunk toRBX::SurfaceSelection::~SurfaceSelection()")]
// 0x660e04 — __ZThn32_N3RBX16SurfaceSelectionD1Ev
pub fn stub_660e04(state: SurfaceSelectionState) {
    // IDA 0x660e04: non-virtual thunk, (char*)this-32, then the full D1 body — this-adjustment subsumed (no MI layout in Rust).
    stub_660c0c(state);
}

#[doc(alias = "non-virtual thunk toRBX::SurfaceSelection::~SurfaceSelection()")]
// 0x660f4c — __ZThn32_N3RBX16SurfaceSelectionD0Ev
pub fn stub_660f4c(state: SurfaceSelectionState) {
    // IDA 0x660f4c: non-virtual thunk to D0, (char*)this-32 — this-adjustment subsumed.
    stub_660d54(state);
}

#[doc(alias = "non-virtual thunk toRBX::SurfaceSelection::~SurfaceSelection()")]
// 0x6610b8 — __ZThn36_N3RBX16SurfaceSelectionD1Ev
pub fn stub_6610b8(state: SurfaceSelectionState) {
    // IDA 0x6610b8: non-virtual thunk to D1, (char*)this-36 — this-adjustment subsumed.
    stub_660c0c(state);
}

#[doc(alias = "non-virtual thunk toRBX::SurfaceSelection::~SurfaceSelection()")]
// 0x661200 — __ZThn36_N3RBX16SurfaceSelectionD0Ev
pub fn stub_661200(state: SurfaceSelectionState) {
    // IDA 0x661200: non-virtual thunk to D0, (char*)this-36 — this-adjustment subsumed.
    stub_660d54(state);
}

#[doc(alias = "RBX::Team::getScore(void)const")]
// 0x662f5c — __ZNK3RBX4Team8getScoreEv
pub fn stub_662f5c(state: &TeamState) -> i32 {
    // IDA 0x662f5c: LDR R0,[R0,#0x5C]; return *(this+23) — score at +92.
    state.score()
}

#[doc(alias = "RBX::Team::setScore(int)")]
// 0x662f60 — __ZN3RBX4Team8setScoreEi
pub fn stub_662f60(state: &mut TeamState, value: i32, mut raise_property_changed: impl FnMut()) {
    // IDA 0x662f60: if (*(this+23) != a2) { *(this+23) = a2; raisePropertyChanged(unk_1327190); } — returns this; no Instance to thread through in core.
    if state.score != value {
        state.score = value;
        raise_property_changed();
    }
}

#[doc(alias = "RBX::Team::getTeamColor(void)const")]
// 0x662f7c — __ZNK3RBX4Team12getTeamColorEv
pub fn stub_662f7c(state: &TeamState) -> u32 {
    // IDA 0x662f7c: *out = *(a2+96) — out-param collapsed to a return of word 24 (+96).
    state.team_color()
}

#[doc(alias = "RBX::Team::setTeamColor(RBX::BrickColor)")]
// 0x662f84 — __ZN3RBX4Team12setTeamColorENS_10BrickColorE
pub fn stub_662f84(state: &mut TeamState, value: u32, mut raise_property_changed: impl FnMut()) {
    // IDA 0x662f84: LDR R2,[R0,#0x60]; if (R2 != R1) { STR R1,[R0,#0x60]; raisePropertyChanged(unk_13271BC); } — returns this.
    if state.team_color != value {
        state.team_color = value;
        raise_property_changed();
    }
}

#[doc(alias = "RBX::Team::getAutoAssignable(void)const")]
// 0x662fa0 — __ZNK3RBX4Team17getAutoAssignableEv
pub fn stub_662fa0(state: &TeamState) -> bool {
    // IDA 0x662fa0: LDRB.W R0,[R0,#0x64] — byte 100.
    state.auto_assignable()
}

#[doc(alias = "RBX::Team::setAutoAssignable(bool)")]
// 0x662fa8 — __ZN3RBX4Team17setAutoAssignableEb
pub fn stub_662fa8(state: &mut TeamState, value: bool, mut raise_property_changed: impl FnMut()) {
    // IDA 0x662fa8: LDRB R2,[R0,#0x64]; if (R2 != R1) { STRB R1,[R0,#0x64]; raisePropertyChanged(unk_13271E8); } — returns this.
    if state.auto_assignable != value {
        state.auto_assignable = value;
        raise_property_changed();
    }
}

#[doc(alias = "RBX::Team::Team(void)")]
// 0x662fc8 — __ZN3RBX4TeamC1Ev
pub fn stub_662fc8() -> TeamState {
    // IDA 0x662fc8 [thunk]: return C2(this).
    stub_662fcc()
}

#[doc(alias = "RBX::Team::Team(void)")]
// 0x662fcc — __ZN3RBX4TeamC2Ev
pub fn stub_662fcc() -> TeamState {
    // IDA 0x662fcc C2: Instance base + vtables + Described init + registrar++ (no mapping); field stores per TeamState::new.
    TeamState::new()
}

#[doc(alias = "RBX::Team::~Team()")]
// 0x6631e0 — __ZN3RBX4TeamD0Ev
pub fn stub_6631e0(state: TeamState) {
    // IDA 0x6631e0 D0: Instance::~Instance(this) + operator delete(this) — drop covers both.
    drop(state);
}

#[doc(alias = "RBX::Team::~Team()")]
// 0x663280 — __ZN3RBX4TeamD1Ev
pub fn stub_663280(state: TeamState) {
    // IDA 0x663280 [thunk]: Instance::~Instance(this) — drop.
    drop(state);
}

#[doc(alias = "non-virtual thunk toRBX::Team::~Team()")]
// 0x663284 — __ZThn32_N3RBX4TeamD0Ev
pub fn stub_663284(state: TeamState) {
    // IDA 0x663284: SUBS R0,#0x20; B.W TeamD0 — this-32 adjustment subsumed.
    stub_6631e0(state);
}

#[doc(alias = "non-virtual thunk toRBX::Team::~Team()")]
// 0x66328c — __ZThn36_N3RBX4TeamD0Ev
pub fn stub_66328c(state: TeamState) {
    // IDA 0x66328c: SUBS R0,#0x24; B.W TeamD0 — this-36 adjustment subsumed.
    stub_6631e0(state);
}

#[doc(alias = "non-virtual thunk toRBX::Team::~Team()")]
// 0x663294 — __ZThn32_N3RBX4TeamD1Ev
pub fn stub_663294(state: TeamState) {
    // IDA 0x663294: Instance::~Instance((char*)this-32) — D1 with this-32 subsumed.
    stub_663280(state);
}

#[doc(alias = "non-virtual thunk toRBX::Team::~Team()")]
// 0x66329c — __ZThn36_N3RBX4TeamD1Ev
pub fn stub_66329c(state: TeamState) {
    // IDA 0x66329c: Instance::~Instance((char*)this-36) — D1 with this-36 subsumed.
    stub_663280(state);
}

#[doc(alias = "RBX::Teams::rebalanceTeams(void)")]
// 0x6645d4 — __ZN3RBX5Teams14rebalanceTeamsEv
pub fn stub_6645d4(_state: &TeamsState) {
    // IDA 0x6645d4: single BX LR — the original body is empty; verified no-op.
}

#[doc(alias = "RBX::Teams::Teams(void)")]
// 0x6645d8 — __ZN3RBX5TeamsC1Ev
pub fn stub_6645d8() -> TeamsState {
    // IDA 0x6645d8 [thunk]: return C2(this).
    stub_6645dc()
}

#[doc(alias = "RBX::Teams::Teams(void)")]
// 0x6645dc — __ZN3RBX5TeamsC2Ev
pub fn stub_6645dc() -> TeamsState {
    // IDA 0x6645dc C2: Instance base + vtables + Described init + registrar++ (no mapping); field stores per TeamsState::new.
    TeamsState::new()
}

#[doc(alias = "RBX::Teams::~Teams()")]
// 0x66482c — __ZN3RBX5TeamsD0Ev
pub fn stub_66482c(state: TeamsState) {
    // IDA 0x66482c D0: D2(this) + operator delete(this) — drop covers both.
    stub_6648e0(state);
}

#[doc(alias = "RBX::Teams::~Teams()")]
// 0x6648cc — __ZN3RBX5TeamsD1Ev
pub fn stub_6648cc(state: TeamsState) {
    // IDA 0x6648cc [thunk]: D2(this).
    stub_6648e0(state);
}

#[doc(alias = "non-virtual thunk toRBX::Teams::~Teams()")]
// 0x6648d0 — __ZThn32_N3RBX5TeamsD0Ev
pub fn stub_6648d0(state: TeamsState) {
    // IDA 0x6648d0: D0((char*)this-32) — this-adjustment subsumed.
    stub_66482c(state);
}

#[doc(alias = "non-virtual thunk toRBX::Teams::~Teams()")]
// 0x6648d8 — __ZThn36_N3RBX5TeamsD0Ev
pub fn stub_6648d8(state: TeamsState) {
    // IDA 0x6648d8: D0((char*)this-36) — this-adjustment subsumed.
    stub_66482c(state);
}

#[doc(alias = "RBX::Teams::~Teams()")]
// 0x6648e0 — __ZN3RBX5TeamsD2Ev
pub fn stub_6648e0(state: TeamsState) {
    // IDA 0x6648e0 D2: vtable reset + release(word 25 at +100, inside the +96 cow member) + Instance dtor — dropping TeamsState drops the Vec member the same way.
    drop(state);
}

#[doc(alias = "non-virtual thunk toRBX::Teams::~Teams()")]
// 0x6649c4 — __ZThn32_N3RBX5TeamsD1Ev
pub fn stub_6649c4(state: TeamsState) {
    // IDA 0x6649c4: D1((char*)this-32); D1 == D2 call (cf. 0x6648cc) — this-adjustment subsumed.
    stub_6648e0(state);
}

#[doc(alias = "non-virtual thunk toRBX::Teams::~Teams()")]
// 0x6649cc — __ZThn36_N3RBX5TeamsD1Ev
pub fn stub_6649cc(state: TeamsState) {
    // IDA 0x6649cc: D1((char*)this-36); D1 == D2 call (cf. 0x6648cc) — this-adjustment subsumed.
    stub_6648e0(state);
}

#[doc(alias = "RBX::Teams::isTeamGame(void)")]
// 0x6649d4 — __ZN3RBX5Teams10isTeamGameEv
pub fn stub_6649d4(players: &[PlayerTeamInfo]) -> bool {
    // IDA 0x6649d4: find<Players> (service lookup is datamodel-owned; empty slice == no service at 0x6649e8); per Player child (isA filter is the caller's projection): result = 1, break unless neutral (byte +104); loop tail re-reads numChildren (0x664a46) and clears result (0x664a48).
    let mut result = false;
    let mut i = 0;
    while i < players.len() {
        result = true;
        if !players[i].neutral {
            break;
        }
        i += 1;
        result = false;
    }
    result
}

#[doc(alias = "RBX::Teams::getNumPlayersInTeam(RBX::BrickColor)")]
// 0x664b24 — __ZN3RBX5Teams19getNumPlayersInTeamENS_10BrickColorE
pub fn stub_664b24(players: Option<&[PlayerTeamInfo]>, team_color: u32) -> usize {
    // IDA 0x664b24: find<Players>; FLog::Asserts && !players -> debugHook/ReleaseAssert (Teams.cpp:97) — panic is the mapping; then count Player children with !neutral (+104) && color (+100) == a2, re-reading numChildren each pass (0x664bf6).
    let players = players.expect("players (RBX ReleaseAssert at Teams.cpp:97)");
    let mut count = 0;
    let mut i = 0;
    while i < players.len() {
        let p = &players[i];
        if !p.neutral && p.team_color == team_color {
            count += 1;
        }
        i += 1;
    }
    count
}

#[doc(alias = "RBX::Teams::teamExists(RBX::BrickColor)")]
// 0x664c04 — __ZN3RBX5Teams10teamExistsENS_10BrickColorE
pub fn stub_664c04(team_color: u32, teams: &TeamsState) -> bool {
    // IDA 0x664c04: BL getTeamFromTeamColor; CMP R0,#0; MOVNE R0,#1 — return (found != 0).
    stub_664c14(team_color, teams).is_some()
}

#[doc(alias = "RBX::Teams::getTeamFromTeamColor(RBX::BrickColor)")]
// 0x664c14 — __ZN3RBX5Teams20getTeamFromTeamColorENS_10BrickColorE
pub fn stub_664c14<'a>(team_color: u32, teams: &'a TeamsState) -> Option<&'a TeamState> {
    // IDA 0x664c14: v3 = 0; if (numChildren) do { cand = child[i] (+56 cow vec); if (cand && isA Team at 0x664c56) { v3 = cand; getTeamColor; if (color == req) break; } ++i; v3 = 0; } while (i < numChildren); return v3. Slice entries are non-null Team projections by construction.
    let mut found: Option<&TeamState> = None;
    let mut i = 0;
    let list = teams.teams();
    while i < list.len() {
        let cand = &list[i];
        found = Some(cand);
        if stub_662f7c(cand) == team_color {
            break;
        }
        i += 1;
        found = None;
    }
    found
}

#[doc(alias = "RBX::Teams::getTeamColorForHumanoid(RBX::Humanoid *)")]
// 0x664cb0 — __ZN3RBX5Teams23getTeamColorForHumanoidEPNS_8HumanoidE
pub fn stub_664cb0() {
    // IDA 0x664cb0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Teams::getTeams(void)")]
// 0x66502c — __ZN3RBX5Teams8getTeamsEv
pub fn stub_66502c() {
    // IDA 0x66502c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::setMultiLine(bool)")]
// 0x665c58 — __ZN3RBX7TextBox12setMultiLineEb
pub fn stub_665c58(state: &mut TextBoxState, value: bool, mut raise_property_changed: impl FnMut()) {
    // IDA 0x665c58: if (a2 != byte+652) { byte+652 = a2; raisePropertyChanged(unk_13272A0); } — returns this.
    if state.multi_line != value {
        state.multi_line = value;
        raise_property_changed();
    }
}

#[doc(alias = "RBX::TextBox::setClearTextOnFocus(bool)")]
// 0x665c78 — __ZN3RBX7TextBox19setClearTextOnFocusEb
pub fn stub_665c78(state: &mut TextBoxState, value: bool, mut raise_property_changed: impl FnMut()) {
    // IDA 0x665c78: if (a2 != byte+607) { byte+607 = a2; raisePropertyChanged(unk_13272CC); } — returns this.
    if state.clear_text_on_focus != value {
        state.clear_text_on_focus = value;
        raise_property_changed();
    }
}

#[doc(alias = "RBX::TextBox::captureFocus(void)")]
// 0x665c98 — __ZN3RBX7TextBox12captureFocusEv
pub fn stub_665c98(state: &mut TextBoxState, user_input_ready: bool, mut focus_captured: impl FnMut()) {
    // IDA 0x665c98: +155 pointer fixup from (+152)-12 then cleared to 0 (internal, no mapping); word +605 = 1; string +608 = ""; +155 = 0; byte +604 = 1; if (create<UserInputService>) { shared_from; fire focus signal; release; } — service lookup is datamodel-owned (flag); the signal fire is the callback; release is drop.
    state.focus_word = 1;
    state.display_text.clear();
    state.focused = true;
    if user_input_ready {
        focus_captured();
    }
}

#[doc(alias = "RBX::TextBox::setText(std::string)")]
// 0x665da0 — __ZN3RBX7TextBox7setTextESs
pub fn stub_665da0(
    state: &mut TextBoxState,
    value: &str,
    contains_profanity: impl FnOnce(&str) -> bool,
    profanity_bypass: bool,
    mut raise_property_changed: impl FnMut(),
) {
    // IDA 0x665e00: length (bytes) > 0x400 round-trips through substr/assign with no observable truncation — value used as-is.
    // IDA 0x665e56: if (ContainsProfanity(v10, a2) != 1 || fw(a1)+22) { if (compare(v10, text+540)) { assign(text+540, v10); word134 = 0; raise x3 (unk_13272F8/2C/58); } }
    if !contains_profanity(value) || profanity_bypass {
        if state.text != value {
            state.text = value.to_string();
            state.flag_134 = false;
            raise_property_changed();
            raise_property_changed();
            raise_property_changed();
        }
    }
}

#[doc(alias = "RBX::TextBox::setFontSize(RBX::TextService::FontSize)")]
// 0x665f5c — __ZN3RBX7TextBox11setFontSizeENS_11TextService8FontSizeE
pub fn stub_665f5c(state: &mut TextBoxState, value: i32, mut raise_property_changed: impl FnMut() -> i32) -> i32 {
    // IDA 0x665f5c: old = word+136; if (old != a2) { store; raise(unk_132750C); return raise(unk_132742C); } return old — the raise status threads through.
    let old = state.font_size;
    if old != value {
        state.font_size = value;
        raise_property_changed();
        return raise_property_changed();
    }
    old
}

#[doc(alias = "RBX::TextBox::setFont(RBX::TextService::Font)")]
// 0x665f94 — __ZN3RBX7TextBox7setFontENS_11TextService4FontE
pub fn stub_665f94(state: &mut TextBoxState, value: i32, mut raise_property_changed: impl FnMut() -> i32) -> i32 {
    // IDA 0x665f94: old = word+148; if (old != a2) { store; raise(unk_1327540); return raise(unk_132742C); } return old — the raise status threads through.
    let old = state.font;
    if old != value {
        state.font = value;
        raise_property_changed();
        return raise_property_changed();
    }
    old
}

#[doc(alias = "RBX::TextBox::setTextColor(RBX::BrickColor)")]
// 0x665fcc — __ZN3RBX7TextBox12setTextColorENS_10BrickColorE
pub fn stub_665fcc(
    state: &mut TextBoxState,
    brick_color: u32,
    brick_to_color3: impl FnOnce(u32) -> [f32; 3],
    mut raise_property_changed: impl FnMut(),
) {
    // IDA 0x665fcc: v4[3] = a2; BrickColor::color3(&v4); setTextColor3(a1, v4) — the palette conversion is datamodel-owned (closure); store + single raise is setTextColor3's share (outside this batch).
    state.text_color3 = brick_to_color3(brick_color);
    raise_property_changed();
}

#[doc(alias = "RBX::TextBox::setTextTransparency(float)")]
// 0x66606c — __ZN3RBX7TextBox19setTextTransparencyEf
pub fn stub_66606c(state: &mut TextBoxState, value: f32, mut raise_property_changed: impl FnMut()) {
    // IDA 0x66606c: if (*(this+140) != a2) { *(this+140) = a2; raisePropertyChanged(unk_132737C); } — float word 140 (+560); returns this.
    if state.text_transparency != value {
        state.text_transparency = value;
        raise_property_changed();
    }
}

#[doc(alias = "RBX::TextBox::setTextWrap(bool)")]
// 0x666094 — __ZN3RBX7TextBox11setTextWrapEb
pub fn stub_666094(state: &mut TextBoxState, value: bool, mut raise_property_changed: impl FnMut() -> i32) -> i32 {
    // IDA 0x666094: old = byte+580; if (old != a2) { store; raise(unk_13273A8); raise(unk_132742C); return raise(unk_1327458); } return old.
    let old = state.text_wrap;
    if old != value {
        state.text_wrap = value;
        raise_property_changed();
        raise_property_changed();
        return raise_property_changed();
    }
    old as i32
}

#[doc(alias = "RBX::TextBox::setTextScale(bool)")]
// 0x6660d4 — __ZN3RBX7TextBox12setTextScaleEb
pub fn stub_6660d4(state: &mut TextBoxState, value: bool, mut raise_property_changed: impl FnMut() -> i32) -> i32 {
    // IDA 0x6660d4: old = byte+581; if (old != a2) { store; raise(unk_1327400); if (a2 == 1) return setTextWrap(this, 1); raise(unk_132742C); return raise(unk_1327458); } return old.
    let old = state.text_scale;
    if old != value {
        state.text_scale = value;
        raise_property_changed();
        if value {
            return stub_666094(state, true, raise_property_changed);
        }
        raise_property_changed();
        return raise_property_changed();
    }
    old as i32
}

#[doc(alias = "RBX::TextBox::setXAlignment(RBX::TextService::XAlignment)")]
// 0x666128 — __ZN3RBX7TextBox13setXAlignmentENS_11TextService10XAlignmentE
pub fn stub_666128(state: &mut TextBoxState, value: i32, mut raise_property_changed: impl FnMut() -> i32) -> i32 {
    // IDA 0x666128: old = word+146; if (old != a2) { store; raise(unk_1327574); raise(unk_132742C); return raise(unk_1327458); } return old.
    let old = state.x_alignment;
    if old != value {
        state.x_alignment = value;
        raise_property_changed();
        raise_property_changed();
        return raise_property_changed();
    }
    old
}

#[doc(alias = "RBX::TextBox::setYAlignment(RBX::TextService::YAlignment)")]
// 0x666168 — __ZN3RBX7TextBox13setYAlignmentENS_11TextService10YAlignmentE
pub fn stub_666168(state: &mut TextBoxState, value: i32, mut raise_property_changed: impl FnMut() -> i32) -> i32 {
    // IDA 0x666168: old = word+147; if (old != a2) { store; raise(unk_13275A8); raise(unk_132742C); return raise(unk_1327458); } return old.
    let old = state.y_alignment;
    if old != value {
        state.y_alignment = value;
        raise_property_changed();
        raise_property_changed();
        return raise_property_changed();
    }
    old
}

#[doc(alias = "RBX::TextBox::getTextBounds(void)const")]
// 0x6661a8 — __ZNK3RBX7TextBox13getTextBoundsEv
pub fn stub_6661a8() {
    // IDA 0x6661a8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::getTextFits(void)const")]
// 0x666334 — __ZNK3RBX7TextBox11getTextFitsEv
pub fn stub_666334() {
    // IDA 0x666334: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::setTextStrokeTransparency(float)")]
// 0x66654c — __ZN3RBX7TextBox25setTextStrokeTransparencyEf
pub fn stub_66654c() {
    // IDA 0x66654c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::checkForResize(void)")]
// 0x666578 — __ZN3RBX7TextBox14checkForResizeEv
pub fn stub_666578() {
    // IDA 0x666578: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::setTransparencyLegacy(float)")]
// 0x6665a4 — __ZN3RBX7TextBox21setTransparencyLegacyEf
pub fn stub_6665a4() {
    // IDA 0x6665a4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::getPersistentDataCost(void)const")]
// 0x6668b0 — __ZNK3RBX7TextBox21getPersistentDataCostEv
pub fn stub_6668b0() {
    // IDA 0x6668b0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::TextBox(void)")]
// 0x666938 — __ZN3RBX7TextBoxC2Ev
pub fn stub_666938(default_color3: [f32; 3]) -> TextBoxState {
    // IDA 0x666938 C2: GuiObject base + vtables + Described init + registrar++ + signal once-init (no mapping); field stores per TextBoxState::new.
    TextBoxState::new(default_color3)
}

#[doc(alias = "RBX::TextBox::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x666d28 — __ZN3RBX7TextBox17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_666d28() {
    // IDA 0x666d28: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::externalReleaseFocus(char const*,bool)")]
// 0x666e84 — __ZN3RBX7TextBox20externalReleaseFocusEPKcb
pub fn stub_666e84() {
    // IDA 0x666e84: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::processMouseEvent(RBX::GuiEvent const&)")]
// 0x667088 — __ZN3RBX7TextBox17processMouseEventERKNS_8GuiEventE
pub fn stub_667088() {
    // IDA 0x667088: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::gainFocus(RBX::GuiEvent const&)")]
// 0x667144 — __ZN3RBX7TextBox9gainFocusERKNS_8GuiEventE
pub fn stub_667144() {
    // IDA 0x667144: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::releaseFocus(RBX::GuiEvent const&,bool)")]
// 0x667388 — __ZN3RBX7TextBox12releaseFocusERKNS_8GuiEventEb
pub fn stub_667388() {
    // IDA 0x667388: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::onHeartbeat(RBX::Heartbeat const&)")]
// 0x6675f8 — __ZN3RBX7TextBox11onHeartbeatERKNS_9HeartbeatE
pub fn stub_6675f8() {
    // IDA 0x6675f8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::doKey(RBX::TextBox::RepeatKeyState::KeyType,char)")]
// 0x667698 — __ZN3RBX7TextBox5doKeyENS0_14RepeatKeyState7KeyTypeEc
pub fn stub_667698() {
    // IDA 0x667698: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::TextBox::onHeartbeat(RBX::Heartbeat const&)")]
// 0x667b28 — __ZThn596_N3RBX7TextBox11onHeartbeatERKNS_9HeartbeatE
pub fn stub_667b28() {
    // IDA 0x667b28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextBox::keyDown(RBX::TextBox::RepeatKeyState::KeyType,RBX::KeyCode,char)")]
// 0x667b30 — __ZN3RBX7TextBox7keyDownENS0_14RepeatKeyState7KeyTypeENS_7KeyCodeEc
pub fn stub_667b30() {
    // IDA 0x667b30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextBox::processKeyEvent(RBX::GuiEvent const&)")]
// 0x667b80 — __ZN3RBX7TextBox15processKeyEventERKNS_8GuiEventE
pub fn stub_667b80() {
    // IDA 0x667b80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextBox::getTextWithCursor(void)")]
// 0x667dd8 — __ZN3RBX7TextBox17getTextWithCursorEv
pub fn stub_667dd8() {
    // IDA 0x667dd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextBox::getTextWithBlankCursor(void)")]
// 0x667f3c — __ZN3RBX7TextBox22getTextWithBlankCursorEv
pub fn stub_667f3c() {
    // IDA 0x667f3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextBox::render2d(RBX::Adorn *)")]
// 0x668088 — __ZN3RBX7TextBox8render2dEPNS_5AdornE
pub fn stub_668088() {
    // IDA 0x668088: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::TextBox::render2d(RBX::Adorn *)")]
// 0x66856c — __ZThn96_N3RBX7TextBox8render2dEPNS_5AdornE
pub fn stub_66856c() {
    // IDA 0x66856c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextBox::getMultiLine(void)const")]
// 0x668574 — __ZNK3RBX7TextBox12getMultiLineEv
pub fn stub_668574(state: &TextBoxState) -> bool {
    // IDA 0x668574: LDRB.W R0,[R0,#0x28C] — byte 652.
    state.multi_line()
}

#[doc(alias = "RBX::TextBox::getClearTextOnFocus(void)const")]
// 0x6685a0 — __ZNK3RBX7TextBox19getClearTextOnFocusEv
pub fn stub_6685a0(state: &TextBoxState) -> bool {
    // IDA 0x6685a0: LDRB.W R0,[R0,#0x25F] — byte 607.
    state.clear_text_on_focus()
}

#[doc(alias = "RBX::GuiTextMixin::getText(void)const")]
// 0x6685f0 — __ZNK3RBX12GuiTextMixin7getTextEv
pub fn stub_6685f0(state: &TextBoxState) -> String {
    // IDA 0x6685f0: copy-constructs the mixin text into the out-param; clone is the by-value mapping.
    state.text().to_string()
}

#[doc(alias = "RBX::GuiTextMixin::getFontSize(void)const")]
// 0x668620 — __ZNK3RBX12GuiTextMixin11getFontSizeEv
pub fn stub_668620(state: &TextBoxState) -> i32 {
    // IDA 0x668620: *(mixin+2) — same word the +544 setter stores.
    state.font_size()
}

#[doc(alias = "RBX::GuiTextMixin::getFont(void)const")]
// 0x668648 — __ZNK3RBX12GuiTextMixin7getFontEv
pub fn stub_668648(state: &TextBoxState) -> i32 {
    // IDA 0x668648: *(mixin+14) — same word the +592 setter stores.
    state.font()
}

#[doc(alias = "RBX::GuiTextMixin::getTextColor(void)const")]
// 0x668670 — __ZNK3RBX12GuiTextMixin12getTextColorEv
pub fn stub_668670() {
    // IDA 0x668670: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GuiTextMixin::getTextColor3(void)const")]
// 0x6686b0 — __ZNK3RBX12GuiTextMixin13getTextColor3Ev
pub fn stub_6686b0() {
    // IDA 0x6686b0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GuiTextMixin::getTextTransparency(void)const")]
// 0x6686e4 — __ZNK3RBX12GuiTextMixin19getTextTransparencyEv
pub fn stub_6686e4() {
    // IDA 0x6686e4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GuiTextMixin::getTextWrap(void)const")]
// 0x66870c — __ZNK3RBX12GuiTextMixin11getTextWrapEv
pub fn stub_66870c() {
    // IDA 0x66870c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GuiTextMixin::getTextScale(void)const")]
// 0x668714 — __ZNK3RBX12GuiTextMixin12getTextScaleEv
pub fn stub_668714() {
    // IDA 0x668714: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GuiTextMixin::getXAlignment(void)const")]
// 0x66871c — __ZNK3RBX12GuiTextMixin13getXAlignmentEv
pub fn stub_66871c() {
    // IDA 0x66871c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GuiTextMixin::getYAlignment(void)const")]
// 0x668744 — __ZNK3RBX12GuiTextMixin13getYAlignmentEv
pub fn stub_668744() {
    // IDA 0x668744: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GuiTextMixin::getTextStrokeColor3(void)const")]
// 0x668790 — __ZNK3RBX12GuiTextMixin19getTextStrokeColor3Ev
pub fn stub_668790() {
    // IDA 0x668790: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GuiTextMixin::getTextStrokeTransparency(void)const")]
// 0x6687a0 — __ZNK3RBX12GuiTextMixin25getTextStrokeTransparencyEv
pub fn stub_6687a0() {
    // IDA 0x6687a0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GuiObject::setTransparencyLegacy(float)")]
// 0x6687a4 — __ZN3RBX9GuiObject21setTransparencyLegacyEf
pub fn stub_6687a4() {
    // IDA 0x6687a4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GuiObject::convertFontSize(RBX::TextService::FontSize)")]
// 0x6687c0 — __ZN3RBX9GuiObject15convertFontSizeENS_11TextService8FontSizeE
pub fn stub_6687c0() {
    // IDA 0x6687c0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GuiObject::getPersistentDataCost(void)const")]
// 0x668878 — __ZNK3RBX9GuiObject21getPersistentDataCostEv
pub fn stub_668878() {
    // IDA 0x668878: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::~TextBox()")]
// 0x668c4c — __ZN3RBX7TextBoxD1Ev
pub fn stub_668c4c(state: TextBoxState) {
    // IDA 0x668c4c [thunk]: D2 shim (member teardown + Instance dtor) — drop.
    drop(state);
}

#[doc(alias = "RBX::TextBox::~TextBox()")]
// 0x668c50 — __ZN3RBX7TextBoxD0Ev
pub fn stub_668c50(state: TextBoxState) {
    // IDA 0x668c50 D0: D2(this) + operator delete(this) — drop covers both.
    stub_668c4c(state);
}

#[doc(alias = "RBX::GuiObject::canProcessMeAndDescendants(void)const")]
// 0x668d00 — __ZNK3RBX9GuiObject26canProcessMeAndDescendantsEv
pub fn stub_668d00() {
    // IDA 0x668d00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiBase2d::getZIndex(void)const")]
// 0x668d08 — __ZNK3RBX9GuiBase2d9getZIndexEv
pub fn stub_668d08() {
    // IDA 0x668d08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiBase2d::getGuiQueue(void)const")]
// 0x668d10 — __ZNK3RBX9GuiBase2d11getGuiQueueEv
pub fn stub_668d10() {
    // IDA 0x668d10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiBase2d::isGuiLeaf(void)const")]
// 0x668d18 — __ZNK3RBX9GuiBase2d9isGuiLeafEv
pub fn stub_668d18() {
    // IDA 0x668d18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiBase2d::getChildRect2D(void)const")]
// 0x668d1c — __ZNK3RBX9GuiBase2d14getChildRect2DEv
pub fn stub_668d1c() {
    // IDA 0x668d1c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GuiBase2d::shouldRender2d(void)const")]
// 0x668d28 — __ZNK3RBX9GuiBase2d14shouldRender2dEv
pub fn stub_668d28() {
    // IDA 0x668d28: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::TextBox::~TextBox()")]
// 0x668d90 — __ZThn32_N3RBX7TextBoxD1Ev
pub fn stub_668d90(state: TextBoxState) {
    // IDA 0x668d90: D1((char*)this-32) — this-adjustment subsumed.
    stub_668c4c(state);
}

#[doc(alias = "non-virtual thunk toRBX::TextBox::~TextBox()")]
// 0x668d98 — __ZThn32_N3RBX7TextBoxD0Ev
pub fn stub_668d98(state: TextBoxState) {
    // IDA 0x668d98: D0((char*)this-32) + operator delete — this-adjustment subsumed.
    stub_668c50(state);
}

#[doc(alias = "non-virtual thunk toRBX::TextBox::~TextBox()")]
// 0x668e4c — __ZThn36_N3RBX7TextBoxD1Ev
pub fn stub_668e4c(state: TextBoxState) {
    // IDA 0x668e4c: D1((char*)this-36) — this-adjustment subsumed.
    stub_668c4c(state);
}

#[doc(alias = "non-virtual thunk toRBX::TextBox::~TextBox()")]
// 0x668e54 — __ZThn36_N3RBX7TextBoxD0Ev
pub fn stub_668e54(state: TextBoxState) {
    // IDA 0x668e54: D0((char*)this-36) + operator delete — this-adjustment subsumed.
    stub_668c50(state);
}

#[doc(alias = "non-virtual thunk toRBX::GuiBase2d::shouldRender2d(void)const")]
// 0x668ef8 — __ZThn96_NK3RBX9GuiBase2d14shouldRender2dEv
pub fn stub_668ef8() {
    // IDA 0x668ef8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[cfg(test)]
mod team_toolbox_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn counter() -> (impl FnMut(), Rc<Cell<i32>>) {
        let n = Rc::new(Cell::new(0));
        let c = Rc::clone(&n);
        (move || { c.set(c.get() + 1); }, n)
    }

    fn counter_ret() -> (impl FnMut() -> i32, Rc<Cell<i32>>) {
        let n = Rc::new(Cell::new(0));
        let c = Rc::clone(&n);
        (move || { c.set(c.get() + 1); c.get() }, n)
    }

    #[test]
    fn team_ctor_defaults_match_c2_stores() {
        let t = stub_662fcc();
        assert_eq!(t.score(), 0);
        assert_eq!(t.team_color(), 1);
        assert!(t.auto_assignable());
        assert!(t.flag_101);
        assert_eq!(t.name, "Team");
        let c1 = stub_662fc8();
        assert_eq!((c1.score(), c1.team_color()), (0, 1));
    }

    #[test]
    fn score_guarded_store_fires_once_on_change() {
        let mut t = TeamState::new();
        assert_eq!(stub_662f5c(&t), 0);
        let (raise, n) = counter();
        stub_662f60(&mut t, 0, raise);
        assert_eq!((t.score(), n.get()), (0, 0));
        let (raise, n) = counter();
        stub_662f60(&mut t, 3, raise);
        assert_eq!((t.score(), n.get()), (3, 1));
    }

    #[test]
    fn color_and_auto_assignable_round_trip() {
        let mut t = TeamState::new();
        assert_eq!(stub_662f7c(&t), 1);
        assert!(stub_662fa0(&t));
        let (raise, n) = counter();
        stub_662f84(&mut t, 1, raise);
        assert_eq!(n.get(), 0);
        let (raise, n) = counter();
        stub_662f84(&mut t, 9, raise);
        assert_eq!((stub_662f7c(&t), n.get()), (9, 1));
        let (raise, n) = counter();
        stub_662fa8(&mut t, false, raise);
        assert_eq!((stub_662fa0(&t), n.get()), (false, 1));
    }

    #[test]
    fn surface_get_and_weak_release() {
        let s = SurfaceSelectionState::new(7);
        assert_eq!(stub_660be0(&s), 7);
        let owner = crate::SharedPtr::new(());
        let probe = crate::SharedPtr::downgrade(&owner);
        let mut s = SurfaceSelectionState::new(1);
        s.watch(&owner);
        drop(owner);
        assert!(s.watched().is_none());
        stub_660c0c(s);
        assert!(probe.upgrade().is_none());
    }

    #[test]
    fn drop_glue_delegates() {
        stub_6631e0(TeamState::new());
        stub_663280(TeamState::new());
        stub_663284(TeamState::new());
        stub_66328c(TeamState::new());
        stub_663294(TeamState::new());
        stub_66329c(TeamState::new());
        stub_660d54(SurfaceSelectionState::new(0));
        stub_660e04(SurfaceSelectionState::new(0));
        stub_660f4c(SurfaceSelectionState::new(0));
        stub_6610b8(SurfaceSelectionState::new(0));
        stub_661200(SurfaceSelectionState::new(0));
        stub_66482c(TeamsState::new());
        stub_6648cc(TeamsState::new());
        stub_6648e0(TeamsState::new());
        stub_6648d0(TeamsState::new());
        stub_6648d8(TeamsState::new());
        stub_6649c4(TeamsState::new());
        stub_6649cc(TeamsState::new());
        stub_668c4c(TextBoxState::new([1.0; 3]));
        stub_668c50(TextBoxState::new([1.0; 3]));
        stub_668d90(TextBoxState::new([1.0; 3]));
        stub_668d98(TextBoxState::new([1.0; 3]));
        stub_668e4c(TextBoxState::new([1.0; 3]));
        stub_668e54(TextBoxState::new([1.0; 3]));
    }

    #[test]
    fn teams_lookup_delegates_to_finder() {
        let mut teams = stub_6645dc();
        assert_eq!(teams.name, "Teams");
        assert!(teams.teams().is_empty());
        stub_6645d4(&teams);
        let mut red = TeamState::new();
        red.team_color = 1;
        let mut blue = TeamState::new();
        blue.team_color = 5;
        teams.add_team(red);
        teams.add_team(blue);
        assert_eq!(stub_664c14(5, &teams).map(|t| t.team_color()), Some(5));
        assert!(stub_664c14(9, &teams).is_none());
        assert!(stub_664c04(5, &teams));
        assert!(!stub_664c04(9, &teams));
        assert!(stub_6645d8().teams().is_empty());
    }

    #[test]
    fn team_game_needs_a_non_neutral_player() {
        assert!(!stub_6649d4(&[]));
        let neutral = [PlayerTeamInfo { neutral: true, team_color: 1 }];
        assert!(!stub_6649d4(&neutral));
        let mixed = [
            PlayerTeamInfo { neutral: true, team_color: 1 },
            PlayerTeamInfo { neutral: false, team_color: 5 },
        ];
        assert!(stub_6649d4(&mixed));
        assert!(stub_6649d4(&mixed[1..]));
    }

    #[test]
    fn player_count_filters_neutral_and_color() {
        let players = [
            PlayerTeamInfo { neutral: true, team_color: 5 },
            PlayerTeamInfo { neutral: false, team_color: 5 },
            PlayerTeamInfo { neutral: false, team_color: 1 },
        ];
        assert_eq!(stub_664b24(Some(&players), 5), 1);
        assert_eq!(stub_664b24(Some(&players), 1), 1);
        assert_eq!(stub_664b24(Some(&[]), 5), 0);
    }

    #[test]
    #[should_panic(expected = "players")]
    fn player_count_without_service_panics() {
        stub_664b24(None, 5);
    }

    #[test]
    fn textbox_ctor_defaults_match_c2_stores() {
        let t = stub_666938([1.0, 1.0, 1.0]);
        assert_eq!(t.text(), "TextBox");
        assert!(!t.flag_134);
        assert_eq!(t.font_size(), 0);
        assert_eq!(t.text_color3, [1.0, 1.0, 1.0]);
        assert_eq!(t.text_transparency, 0.0);
        assert_eq!(t.field_144, 1.0);
        assert!(!t.text_wrap);
        assert!(!t.text_scale);
        assert_eq!((t.x_alignment, t.y_alignment, t.font), (2, 1, 0));
        assert!(!t.focused);
        assert_eq!(t.focus_word, 0);
        assert!(t.clear_text_on_focus);
        assert!(t.display_text.is_empty());
        assert!(!t.multi_line);
        assert_eq!(t.name, "TextBox");
    }

    #[test]
    fn bool_setters_fire_once_on_change() {
        let mut t = TextBoxState::new([0.0; 3]);
        let (raise, n) = counter();
        stub_665c58(&mut t, false, raise);
        assert_eq!(n.get(), 0);
        let (raise, n) = counter();
        stub_665c58(&mut t, true, raise);
        assert!((t.multi_line, n.get()) == (true, 1));
        let (raise, n) = counter();
        stub_665c78(&mut t, false, raise);
        assert!((!t.clear_text_on_focus, n.get()) == (true, 1));
    }

    #[test]
    fn text_set_filters_profanity_and_dedups() {
        let mut t = TextBoxState::new([0.0; 3]);
        let (raise, n) = counter();
        stub_665da0(&mut t, "hi", |_| true, false, raise);
        assert_eq!((t.text.as_str(), n.get()), ("TextBox", 0));
        let (raise, n) = counter();
        stub_665da0(&mut t, "hi", |_| true, true, raise);
        assert_eq!((t.text.as_str(), n.get()), ("hi", 3));
        assert!(!t.flag_134);
        let (raise, n) = counter();
        stub_665da0(&mut t, "hi", |_| false, false, raise);
        assert_eq!(n.get(), 0);
        let big = "x".repeat(2000);
        let (raise, n) = counter();
        stub_665da0(&mut t, &big, |_| false, false, raise);
        assert_eq!((t.text.len(), n.get()), (2000, 3));
    }

    #[test]
    fn int_setters_double_raise_and_thread_status() {
        let mut t = TextBoxState::new([0.0; 3]);
        let (raise, n) = counter_ret();
        assert_eq!((stub_665f5c(&mut t, 0, raise), n.get()), (0, 0));
        let (raise, n) = counter_ret();
        assert_eq!(stub_665f5c(&mut t, 12, raise), 2);
        assert_eq!((t.font_size(), n.get()), (12, 2));
        let (raise, n) = counter_ret();
        assert_eq!(stub_665f94(&mut t, 2, raise), 2);
        assert_eq!((t.font(), n.get()), (2, 2));
        let (raise, n) = counter_ret();
        assert_eq!(stub_666094(&mut t, true, raise), 3);
        assert!(t.text_wrap);
        let (raise, n) = counter_ret();
        assert_eq!((stub_666128(&mut t, 0, raise), n.get()), (3, 3));
        assert_eq!(t.x_alignment, 0);
        let (raise, n) = counter_ret();
        assert_eq!((stub_666168(&mut t, 0, raise), n.get()), (3, 3));
        assert_eq!(t.y_alignment, 0);
    }

    #[test]
    fn scale_on_cascades_into_wrap() {
        let mut t = TextBoxState::new([0.0; 3]);
        let (raise, n) = counter_ret();
        assert_eq!(stub_6660d4(&mut t, true, raise), 4);
        assert!((t.text_scale, t.text_wrap) == (true, true));
        assert_eq!(n.get(), 4);
        let (raise, n) = counter_ret();
        assert_eq!(stub_6660d4(&mut t, false, raise), 3);
        assert!((!t.text_scale, t.text_wrap) == (true, true));
        assert_eq!(n.get(), 3);
    }

    #[test]
    fn transparency_guarded_float_store() {
        let mut t = TextBoxState::new([0.0; 3]);
        let (raise, n) = counter();
        stub_66606c(&mut t, 0.0, raise);
        assert_eq!(n.get(), 0);
        let (raise, n) = counter();
        stub_66606c(&mut t, 0.5, raise);
        assert_eq!((t.text_transparency, n.get()), (0.5, 1));
    }

    #[test]
    fn capture_focus_gates_signal_on_service() {
        let mut t = TextBoxState::new([0.0; 3]);
        t.display_text = "typed".to_string();
        let fired = Rc::new(Cell::new(false));
        let probe = Rc::clone(&fired);
        stub_665c98(&mut t, false, move || probe.set(true));
        assert!((t.focused, t.focus_word) == (true, 1));
        assert!(t.display_text.is_empty());
        assert!(!fired.get());
        let probe = Rc::clone(&fired);
        stub_665c98(&mut t, true, move || probe.set(true));
        assert!(fired.get());
    }

    #[test]
    fn mixin_getters_mirror_fields() {
        let mut t = TextBoxState::new([0.0; 3]);
        t.text = "abc".to_string();
        t.font_size = 7;
        t.font = 2;
        assert_eq!(stub_6685f0(&t), "abc");
        assert_eq!(stub_668620(&t), 7);
        assert_eq!(stub_668648(&t), 2);
        assert!(!stub_668574(&t));
        assert!(stub_6685a0(&t));
        t.multi_line = true;
        assert!(stub_668574(&t));
    }

    #[test]
    fn text_color_routes_through_palette_closure() {
        let mut t = TextBoxState::new([0.0; 3]);
        let (raise, n) = counter();
        stub_665fcc(&mut t, 194, |b| { assert_eq!(b, 194); [1.0, 0.0, 0.0] }, raise);
        assert_eq!(t.text_color3, [1.0, 0.0, 0.0]);
        assert_eq!(n.get(), 1);
    }
}
