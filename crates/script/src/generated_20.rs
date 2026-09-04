// Auto-generated skeletons for rbx-script -- Lua|Script batch (EA-sorted earliest gap)
// Filter: Lua|Script (case-sensitive, Script or Lua substring in demangled)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x267ec..0x2746bc | filtered 4456, compiled_before 2401, compiled_inter 691, remaining_before 3765 -> remaining_after 3665
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x267ec — -[PlaceLauncher injectJoinScript:]
// type: void __cdecl(PlaceLauncher *self, SEL, id)
#[doc(alias = "-[PlaceLauncher injectJoinScript:]")]
// IDA 0x267ec: UTF8String of the script (0x2681c), shared game from
// self->rbxView (0x2682a..0x26866), bind joinGameWithJoinScript(script, game)
// (0x2687e) then thread_wrapper dispatch on the "In..." worker thread
// (0x2688a). MODEL: UIKit (NSString/objc_msgSend) and the worker thread are
// not modeled; records the script bytes and flags the dispatch.
pub fn stub_0x267ec(launcher: &mut PlaceLauncher, script: &[u8]) {
    launcher.join_script = script.to_vec();
    launcher.join_dispatched = true;
}

// 0x29280 — -[PlaceLauncher startGameWithJoinScript:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, id, id, char)
#[doc(alias = "-[PlaceLauncher startGameWithJoinScript:controller:presentGameAutomatically:]")]
// IDA 0x29280: nil self returns 0 (0x292ce); otherwise
// setupPreloadedGameWithNonGameController:isApp: (0x292f4) must yield a game
// (0x292fc) before the same joinGameWithJoinScript bind+dispatch as 0x267ec
// (0x29314..0x29340). MODEL: preloading always succeeds here, so the result
// is always 1 when self is present; the nil-self early-out is kept.
pub fn stub_0x29280(launcher: Option<&mut PlaceLauncher>, script: &[u8]) -> bool {
    match launcher {
        None => false,
        Some(l) => {
            stub_0x267ec(l, script);
            true
        }
    }
}

// 0x29ccc — -[PlaceLauncher teleport:withAuthentication:withScript:]
// type: void __cdecl(PlaceLauncher *self, SEL, id, id, id)
#[doc(alias = "-[PlaceLauncher teleport:withAuthentication:withScript:]")]
// IDA 0x29ccc: resolves MainViewController sharedInstance, builds the
// SecurePlayerGame teleport context from place/auth/script ids and issues
// the teleport through the game controller (block completion at 0x2a99c).
// MODEL: UIKit/controller plumbing not modeled; records the script bytes and
// flags the dispatch, same observable effect as 0x267ec.
pub fn stub_0x29ccc(launcher: &mut PlaceLauncher, script: &[u8]) {
    launcher.join_script = script.to_vec();
    launcher.join_dispatched = true;
}

// 0x2a8c8 — ___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke
#[doc(alias = "___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke")]
// IDA 0x2a8c8: centers a 1x1 loading frame in the controller view: reads the
// view frame (0x2a904/0x2a940), halves width/height via vmul_f32(..., 0.5)
// (0x2a910/0x2a948), then setFrame:(x, y, 1.0, 1.0) (0x2a97a/0x2a984). A nil
// view centers at (0, 0) (0x2a920..0x2a922). Pure rect math, exact.
pub fn stub_0x2a8c8(view_frame: Option<(f32, f32, f32, f32)>) -> (f32, f32, f32, f32) {
    match view_frame {
        None => (0.0, 0.0, 1.0, 1.0),
        Some((_, _, w, h)) => (w * 0.5, h * 0.5, 1.0, 1.0),
    }
}

// 0x2a99c — ___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke246
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke246")]
// IDA 0x2a99c: async teleport-completion block: retains the shared game
// (0x2a9c8..0x2a9f6), swaps the game window on success and releases the
// join context. MODEL: window/retain plumbing not modeled; marks the
// teleport complete on the launcher.
pub fn stub_0x2a99c(launcher: &mut PlaceLauncher) {
    launcher.teleport_complete = true;
}

// 0x25f838 — __ZNK3RBX10Reflection15EventDescriptor12isScriptableEv
// type: int __fastcall(RBX::Reflection::EventDescriptor *this)
#[doc(alias = "RBX::Reflection::EventDescriptor::isScriptable(void)const")]
// IDA 0x25f838: returns 1 (0x25f83a). Events are always script-visible.
pub fn stub_0x25f838() -> bool {
    true
}

// 0x26a6c0 — __ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EEC2INS_9ContentIdEEET_
// type: RBX::BaseScript *__fastcall(RBX::BaseScript *, int *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EEC2INS_9ContentIdEEET_")]
pub fn stub_0x26a6c0() -> ! {
    todo!("0x26a6c0 __ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EEC2INS_9ContentIdEEET_")
}

// 0x26a88c — __ZN3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x26a88c (thunk): tail-calls RBX::BaseScript::~BaseScript. MODEL: the
// instance has no modeled fields, so destruction is a no-op drop marker.
pub fn stub_0x26a88c(_obj: &mut StarterScriptCore) {}

// 0x26a890 — __ZN3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x26a890: dtor (0x26a8e0) + operator delete (0x26a8e6). MODEL: consuming
// the Box runs the drop glue and frees the allocation, as delete does.
pub fn stub_0x26a890(_obj: Box<StarterScriptCore>) {}

// 0x26a930 — __ZThn32_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x26a930: SUBS R0, #0x20 then B.W RBX::BaseScript::~BaseScript — the
// +32 secondary-base adjust for the D1 dtor above. Same no-op drop marker.
pub fn stub_0x26a930(_obj: &mut StarterScriptCore) {}

// 0x26a938 — __ZThn32_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x26a938: +32 this-adjust then the D0 dtor+delete above. MODEL: same
// consuming-Box shape as 0x26a890.
pub fn stub_0x26a938(_obj: Box<StarterScriptCore>) {}

// 0x26a9dc — __ZThn36_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x26a9dc: +36 this-adjust then the D1 dtor above. Same no-op marker.
pub fn stub_0x26a9dc(_obj: &mut StarterScriptCore) {}

// 0x26a9e4 — __ZThn36_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_13StarterScriptENS_10CoreScriptELZNS_14sStarterScriptEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x26a9e4: +36 this-adjust then the D0 dtor+delete above. MODEL: same
// consuming-Box shape as 0x26a890.
pub fn stub_0x26a9e4(_obj: Box<StarterScriptCore>) {}

// 0x26aa88 — __ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x26aa88() -> ! {
    todo!("0x26aa88 __ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x26aba4 — __ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x26aba4: Described<StarterScript,...> D1 — same BaseScript teardown as
// 0x26a88c (D1 thunks share the base dtor). Same no-op drop marker.
pub fn stub_0x26aba4(_obj: &mut StarterScriptCore) {}

// 0x26aba8 — __ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x26aba8: D0 dtor + operator delete, same shape as 0x26a890. MODEL:
// consuming Box drops and frees.
pub fn stub_0x26aba8(_obj: Box<StarterScriptCore>) {}

// 0x26ac48 — __ZThn32_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x26ac48: +32 this-adjust then the D1 dtor above (same Thn32 pattern as
// 0x26a930). Same no-op marker.
pub fn stub_0x26ac48(_obj: &mut StarterScriptCore) {}

// 0x26ac50 — __ZThn32_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x26ac50: +32 this-adjust then the D0 dtor+delete above. MODEL: same
// consuming-Box shape as 0x26a890.
pub fn stub_0x26ac50(_obj: Box<StarterScriptCore>) {}

// 0x26acf4 — __ZThn36_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x26acf4() -> ! {
    todo!("0x26acf4 __ZThn36_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x26acfc — __ZThn36_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x26acfc() -> ! {
    todo!("0x26acfc __ZThn36_N3RBX10Reflection9DescribedINS_13StarterScriptELZNS_14sStarterScriptEENS_17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x26aff8 — __ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x26aff8() -> ! {
    todo!("0x26aff8 __ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x26affc — __ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x26affc() -> ! {
    todo!("0x26affc __ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x26b09c — __ZThn32_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x26b09c() -> ! {
    todo!("0x26b09c __ZThn32_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x26b0a4 — __ZThn32_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x26b0a4() -> ! {
    todo!("0x26b0a4 __ZThn32_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x26b148 — __ZThn36_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x26b148() -> ! {
    todo!("0x26b148 __ZThn36_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x26b150 — __ZThn36_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x26b150() -> ! {
    todo!("0x26b150 __ZThn36_N3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x26b55c — __ZNK3RBX3Lua12LuaArguments9getObjectEiRN5boost10shared_ptrINS_10Reflection13DescribedBaseEEE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::LuaArguments::getObject(int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &)const")]
pub fn stub_0x26b55c() -> ! {
    todo!("0x26b55c RBX::Lua::LuaArguments::getObject(int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)const")
}

// 0x26b6e4 — __ZNK3RBX3Lua12LuaArguments7getEnumEiRKNS_10Reflection14EnumDescriptorERi
// type: bool __fastcall(RBX::Lua::LuaArguments *this, int, const RBX::Reflection::EnumDescriptor *, int *)
#[doc(alias = "RBX::Lua::LuaArguments::getEnum(int,RBX::Reflection::EnumDescriptor const&,int &)const")]
pub fn stub_0x26b6e4() -> ! {
    todo!("0x26b6e4 RBX::Lua::LuaArguments::getEnum(int,RBX::Reflection::EnumDescriptor const&,int &)const")
}

// 0x26b788 — __ZN3RBX3Lua12LuaArguments3getEP9lua_StateiRNS_10Reflection7VariantEb
// type: int __fastcall(struct _Unwind_Exception *, int, int, int)
#[doc(alias = "RBX::Lua::LuaArguments::get(lua_State *,int,RBX::Reflection::Variant &,bool)")]
pub fn stub_0x26b788() -> ! {
    todo!("0x26b788 RBX::Lua::LuaArguments::get(lua_State *,int,RBX::Reflection::Variant &,bool)")
}

// 0x26c138 — __ZN3RBX3Lua12LuaArguments4pushERKNS_10Reflection7VariantEP9lua_State
// type: int()
#[doc(alias = "RBX::Lua::LuaArguments::push(RBX::Reflection::Variant const&,lua_State *)")]
pub fn stub_0x26c138() -> ! {
    todo!("0x26c138 RBX::Lua::LuaArguments::push(RBX::Reflection::Variant const&,lua_State *)")
}

// 0x26c38c — __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrIN5boost10shared_ptrINS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &)")]
pub fn stub_0x26c38c() -> ! {
    todo!("0x26c38c bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<boost::shared_ptr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)")
}

// 0x26c830 — __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
pub fn stub_0x26c830() -> ! {
    todo!("0x26c830 bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")
}

// 0x26c92c — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26c92c: lua_touserdata (0x26c93e); when non-null and the slot
// metatable rawequals Bridge::className[0] (0x26c950..0x26c988), writes
// Type::getSingleton<CoordinateFrame> + placement copy (0x26c994..0x26c99c)
// and returns 1; else 0. Same shape for 0x26c9a8..0x26d070, class tag only.
pub fn stub_0x26c92c(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_cframe(idx) {
        Some(v) => {
            *out = BridgeVal::CFrame(v);
            true
        }
        None => false,
    }
}

// 0x26c9a8 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::Region3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26c9a8: same metatable-check shape as 0x26c92c (0x26c9ba..0x26ca1c)
// with the Region3 class tag and singleton.
pub fn stub_0x26c9a8(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_region3(idx) {
        Some(v) => {
            *out = BridgeVal::Region3(v);
            true
        }
        None => false,
    }
}

// 0x26ca24 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::Region3int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26ca24: same metatable-check shape as 0x26c92c (0x26ca36..0x26ca98)
// with the Region3int16 class tag and singleton.
pub fn stub_0x26ca24(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_region3i16(idx) {
        Some(v) => {
            *out = BridgeVal::Region3i16(v);
            true
        }
        None => false,
    }
}

// 0x26caa0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26caa0: same metatable-check shape as 0x26c92c (0x26cab2..0x26cb14)
// with the Vector3int16 class tag and singleton.
pub fn stub_0x26caa0(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_vec3i16(idx) {
        Some(v) => {
            *out = BridgeVal::Vec3i16(v);
            true
        }
        None => false,
    }
}

// 0x26cb1c — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26cb1c: same metatable-check shape as 0x26c92c (0x26cb2e..0x26cb90)
// with the Vector2int16 class tag and singleton.
pub fn stub_0x26cb1c(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_vec2i16(idx) {
        Some(v) => {
            *out = BridgeVal::Vec2i16(v);
            true
        }
        None => false,
    }
}

// 0x26cb98 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26cb98: same metatable-check shape as 0x26c92c (0x26cbaa..0x26cc0c)
// with the Vector3 class tag and singleton.
pub fn stub_0x26cb98(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_vec3(idx) {
        Some(v) => {
            *out = BridgeVal::Vec3(v);
            true
        }
        None => false,
    }
}

// 0x26cc14 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26cc14: same metatable-check shape as 0x26c92c (0x26cc26..0x26cc88)
// with the Vector2 class tag and singleton.
pub fn stub_0x26cc14(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_vec2(idx) {
        Some(v) => {
            *out = BridgeVal::Vec2(v);
            true
        }
        None => false,
    }
}

// 0x26cc90 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::RbxRay,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26cc90: same metatable-check shape as 0x26c92c (0x26cca2..0x26cd04)
// with the RbxRay class tag and singleton.
pub fn stub_0x26cc90(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_ray(idx) {
        Some(v) => {
            *out = BridgeVal::Ray(v);
            true
        }
        None => false,
    }
}

// 0x26cd0c — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Color3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26cd0c: same metatable-check shape as 0x26c92c (0x26cd1e..0x26cd80)
// with the Color3 class tag and singleton.
pub fn stub_0x26cd0c(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_color3(idx) {
        Some(v) => {
            *out = BridgeVal::Color3(v);
            true
        }
        None => false,
    }
}

// 0x26cd88 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::BrickColor,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26cd88: same metatable-check shape as 0x26c92c (0x26cd9a..0x26cdfc)
// with the BrickColor class tag and singleton.
pub fn stub_0x26cd88(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_brick(idx) {
        Some(v) => {
            *out = BridgeVal::Brick(v);
            true
        }
        None => false,
    }
}

// 0x26ce04 — __ZN3RBX3Lua6BridgeINS_4UDimELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::UDim,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26ce04: same metatable-check shape as 0x26c92c (0x26ce16..0x26ce78)
// with the UDim class tag and singleton.
pub fn stub_0x26ce04(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_udim(idx) {
        Some(v) => {
            *out = BridgeVal::UDim(v);
            true
        }
        None => false,
    }
}

// 0x26ce80 — __ZN3RBX3Lua6BridgeINS_5UDim2ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::UDim2,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26ce80: same metatable-check shape as 0x26c92c (0x26ce92..0x26cef4)
// with the UDim2 class tag and singleton.
pub fn stub_0x26ce80(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_udim2(idx) {
        Some(v) => {
            *out = BridgeVal::UDim2(v);
            true
        }
        None => false,
    }
}

// 0x26cefc — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::Faces,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26cefc: same metatable-check shape as 0x26c92c (0x26cf0e..0x26cf70)
// with the Faces class tag and singleton.
pub fn stub_0x26cefc(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_faces(idx) {
        Some(v) => {
            *out = BridgeVal::Faces(v);
            true
        }
        None => false,
    }
}

// 0x26cf78 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::Axes,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26cf78: same metatable-check shape as 0x26c92c (0x26cf8a..0x26cfec)
// with the Axes class tag and singleton.
pub fn stub_0x26cf78(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_axes(idx) {
        Some(v) => {
            *out = BridgeVal::Axes(v);
            true
        }
        None => false,
    }
}

// 0x26cff4 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::CellID,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26cff4: same metatable-check shape as 0x26c92c (0x26d006..0x26d068)
// with the CellID class tag and singleton.
pub fn stub_0x26cff4(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_cell(idx) {
        Some(v) => {
            *out = BridgeVal::Cell(v);
            true
        }
        None => false,
    }
}

// 0x26d070 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::InputObject,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// IDA 0x26d070: same metatable-check shape as 0x26c92c (0x26d082..0x26d0e4)
// with the InputObject class tag and singleton.
pub fn stub_0x26d070(l: &BridgeState, idx: i32, out: &mut BridgeVal) -> bool {
    match l.get_input(idx) {
        Some(v) => {
            *out = BridgeVal::Input(v);
            true
        }
        None => false,
    }
}

// 0x26d0ec — __ZN3RBX16withVariantValueIiNS_3Lua14ArgumentPusherEEET_RKNS_10Reflection7VariantET0_
// type: int __fastcall(char ****, int)
#[doc(alias = "int RBX::withVariantValue<int,RBX::Lua::ArgumentPusher>(RBX::Reflection::Variant const&,RBX::Lua::ArgumentPusher)")]
pub fn stub_0x26d0ec() -> ! {
    todo!("0x26d0ec int RBX::withVariantValue<int,RBX::Lua::ArgumentPusher>(RBX::Reflection::Variant const&,RBX::Lua::ArgumentPusher)")
}

// 0x26dc38 — __ZNK3RBX3Lua12LuaArguments10getVariantEiRNS_10Reflection7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::LuaArguments::getVariant(int,RBX::Reflection::Variant &)const")]
pub fn stub_0x26dc38() -> ! {
    todo!("0x26dc38 RBX::Lua::LuaArguments::getVariant(int,RBX::Reflection::Variant &)const")
}

// 0x26ddb4 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEE
// type: int __fastcall(_DWORD *, _DWORD *)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")]
pub fn stub_0x26ddb4() -> ! {
    todo!("0x26ddb4 RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")
}

// 0x26dddc — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEE
// type: int __fastcall(int *, int *)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_0x26dddc() -> ! {
    todo!("0x26dddc RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")
}

// 0x26dea0 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsNS_10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEE
// type: int __fastcall(int *, _DWORD *)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_0x26dea0() -> ! {
    todo!("0x26dea0 RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")
}

// 0x26df2c — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKNS_10Reflection5TupleEEE
// type: int __fastcall(int *, char ******)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_0x26df2c() -> ! {
    todo!("0x26df2c RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0x26df60 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEE
// type: int __fastcall(int *, const shared_count *)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
pub fn stub_0x26df60() -> ! {
    todo!("0x26df60 RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)")
}

// 0x26e030 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEE
// type: int __fastcall(int *, const shared_count *)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")]
pub fn stub_0x26e030() -> ! {
    todo!("0x26e030 RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")
}

// 0x26e100 — __ZN3RBX3Lua14ArgumentPusherclINS_6CellIDEEEiRKT_PN5boost10disable_ifINS7_13is_arithmeticIS4_EEvE4typeE
// type: int __fastcall(int *, int)
#[doc(alias = "int RBX::Lua::ArgumentPusher::operator()<RBX::CellID>(RBX::CellID const&,boost::disable_if<boost::is_arithmetic<RBX::CellID>,void>::type *)")]
pub fn stub_0x26e100() -> ! {
    todo!("0x26e100 int RBX::Lua::ArgumentPusher::operator()<RBX::CellID>(RBX::CellID const&,boost::disable_if<boost::is_arithmetic<RBX::CellID>,void>::type *)")
}

// 0x26eb44 — __ZN3rbx8any_castIRKN5boost10shared_ptrINS1_8functionIFvNS2_IKN3RBX10Reflection5TupleEEENS3_IFvPNS4_3Lua12IAsyncResultEEEEEEEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const& rbx::any_cast<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26eb44() -> ! {
    todo!("0x26eb44 boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const& rbx::any_cast<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26f1d4 — __ZN3RBX3Lua12LuaArguments9pushArrayIN9__gnu_cxx17__normal_iteratorIPKNS_10Reflection7VariantESt6vectorIS6_SaIS6_EEEEEEiT_SD_P9lua_State
// type: int __fastcall(char ****, char ****, int)
#[doc(alias = "int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,lua_State *)")]
pub fn stub_0x26f1d4() -> ! {
    todo!("0x26f1d4 int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,lua_State *)")
}

// 0x26fa78 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
pub fn stub_0x26fa78() -> ! {
    todo!("0x26fa78 bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")
}

// 0x26ff94 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS3_INS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &)")]
pub fn stub_0x26ff94() -> ! {
    todo!("0x26ff94 bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<boost::shared_ptr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)")
}

// 0x270008 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE8getValueIS6_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::getValue<RBX::Reflection::EnumDescriptor::Item const*>(lua_State *,unsigned int,RBX::Reflection::EnumDescriptor::Item const* &)")]
// IDA 0x270008: same metatable-check shape as 0x26c92c (0x27001a..0x270070),
// but the out param takes the Item const* word itself (`*a3 = *v6` at
// 0x27006c), not a Variant. MODEL: the pointer identity is the EnumItemPtr.
pub fn stub_0x270008(l: &BridgeState, idx: i32, out: &mut EnumItemPtr) -> bool {
    match l.get_enum_item(idx) {
        Some(v) => {
            *out = v;
            true
        }
        None => false,
    }
}

// ── IMPL batch (25 stubs 0x272940..0x2735bc) ────────────────────────────────
// Vector2int16::on_newindex tail, the full Vector2 bridge and the BrickColor
// bridge. Grounded from IDA decompile + disasm over MCP; lane order verified
// in disasm (VADD at 0x272b22, VSUB at 0x272b7a with arg1-arg2 direction,
// VMUL at 0x272be4). The BrickColor value ops (closest/parse/random/name/
// color3, IDA 0x3043c4..0x304568) are datamodel-owned; the MODEL shims below
// implement the grounded algorithms against a still-empty palette table,
// i.e. exactly the originals' empty-map fallbacks (closest → 194 at
// 0x3044e2, parse → 194 at 0x304420).

// ── G3D value model ────────────────────────────────────────────────────────
// G3D::Vector2 is two floats (x@0, y@4 — VLDR [R0] / [R0,#4] in on_add, IDA
// 0x272b0e..0x272b14); G3D::Color3 is three floats (LDR [R0] / [R0,#4] /
// [R0,#8] in newBrickColor, IDA 0x2731d8..0x2731e2).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2int16 {
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

// RBX::BrickColor is one i32 palette number (4-byte LDR/STR of the value at
// IDA 0x273110 and 0x2731b2; vector<BrickColor>::at + pushNewObject take it
// by value at 0x273294..0x27329e).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrickColor(pub i32);

// `Bridge<..., true>::className[0]` strings read at the call sites.
pub const VECTOR2_CLASS: &str = "Vector2"; // IDA 0x272afc "Vector2"
pub const BRICKCOLOR_CLASS: &str = "BrickColor"; // IDA 0x27309c..0x2730ba luaL_register
pub const COLOR3_CLASS: &str = "Color3"; // IDA 0x2731d2 "Color3"

// ── G3D/RBX value types for Bridge<T,true>::getValue ────────────────────────
// Layouts from the userdata payloads copied by placement_any<T> at IDA
// 0x26c99c..0x26d0e0: G3D vectors are packed f32/i16 lanes, Region3 is a
// min/max vector pair, RbxRay is origin+direction, UDim is scale+offset,
// Faces/Axes are NormalId bitmasks, CellID is four i32 lanes (16-byte copy
// at IDA 0x26e12a..0x26e130: two double lanes + dword + shared ref).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3int16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Region3 {
    pub min: Vector3,
    pub max: Vector3,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Region3int16 {
    pub min: Vector3int16,
    pub max: Vector3int16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RbxRay {
    pub origin: Vector3,
    pub direction: Vector3,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UDim {
    pub scale: f32,
    pub offset: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UDim2 {
    pub x: UDim,
    pub y: UDim,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Faces(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Axes(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellID {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}
// MODEL: InputObject userdata holds a shared Instance ref; only the handle
// identity is modeled, not the referent.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputObject(pub u64);
// MODEL: the enum-item bridge copies the Item const* word (IDA 0x27006c
// `*a3 = *v6`); only the pointer identity is modeled.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumItemPtr(pub u32);
// Marker for DescribedNonCreatable<StarterScript,...> destruction (IDA
// 0x26a88c..0x26ac50): no fields modeled, drop glue only.
#[derive(Debug, Default)]
pub struct StarterScriptCore;
// Minimal PlaceLauncher: the join/teleport script plus dispatch flags. UIKit
// (NSString, objc_msgSend, view controllers) and the worker thread behind
// thread_wrapper are not modeled (MODEL).
#[derive(Debug, Default)]
pub struct PlaceLauncher {
    pub join_script: Vec<u8>,
    pub join_dispatched: bool,
    pub teleport_complete: bool,
}
pub const REGION3_CLASS: &str = "Region3";
pub const REGION3INT16_CLASS: &str = "Region3int16";
pub const VECTOR3INT16_CLASS: &str = "Vector3int16";
pub const VECTOR2INT16_CLASS: &str = "Vector2int16";
pub const RBXRAY_CLASS: &str = "RbxRay";
pub const UDIM_CLASS: &str = "UDim";
pub const UDIM2_CLASS: &str = "UDim2";
pub const FACES_CLASS: &str = "Faces";
pub const AXES_CLASS: &str = "Axes";
pub const CELLID_CLASS: &str = "CellID";
pub const INPUTOBJECT_CLASS: &str = "InputObject";

// ── Minimal Lua-stack façade ───────────────────────────────────────────────
// Same convention as generated_79.rs: positional args, userdata slots,
// number/string results, class-library registration. Type mismatches panic,
// standing in for the original's lua_error longjmp out of luaL_checkudata.
#[derive(Clone, Debug, PartialEq)]
pub enum BridgeVal {
    Nil,
    Num(f64),
    Bool(bool),
    Str(Vec<u8>),
    Vec2(Vector2),
    Vec3(Vector3),
    CFrame(CoordinateFrame),
    Color3(Color3),
    Brick(BrickColor),
    Region3(Region3),
    Region3i16(Region3int16),
    Vec3i16(Vector3int16),
    Vec2i16(Vector2int16),
    Ray(RbxRay),
    UDim(UDim),
    UDim2(UDim2),
    Faces(Faces),
    Axes(Axes),
    Cell(CellID),
    Input(InputObject),
    EnumItem(EnumItemPtr),
    Closure(&'static str),
    Table { readonly: bool },
}

#[derive(Clone, Debug, Default)]
pub struct BridgeState {
    stack: Vec<BridgeVal>,
    /// className values passed to luaL_register (IDA 0x272aca / 0x2730ba).
    pub registered_libs: Vec<&'static str>,
}

impl BridgeState {
    pub fn new() -> Self {
        BridgeState { stack: Vec::new(), registered_libs: Vec::new() }
    }
    // IDA lua_gettop: stack height (BL at 0x272a1e, 0x2730e8, 0x27322a).
    pub fn gettop(&self) -> i32 {
        self.stack.len() as i32
    }
    // IDA lua_pushnumber: appends a double (BL at 0x272e6c).
    pub fn push_number(&mut self, v: f64) {
        self.stack.push(BridgeVal::Num(v));
    }
    // IDA lua_pushinteger: Lua 5.1 numbers are double; widens (0x2734c4).
    pub fn push_integer(&mut self, v: i32) {
        self.push_number(v as f64);
    }
    // IDA lua_pushlstring (BL at 0x2734b4).
    pub fn push_str(&mut self, s: &[u8]) {
        self.stack.push(BridgeVal::Str(s.to_vec()));
    }
    // IDA lua_pushcclosure(L, f, 0) (0x272e5c): records which C fn closed.
    pub fn push_closure(&mut self, name: &'static str) {
        self.stack.push(BridgeVal::Closure(name));
    }
    pub fn push_vec2(&mut self, v: Vector2) {
        self.stack.push(BridgeVal::Vec2(v));
    }
    pub fn push_color3(&mut self, v: Color3) {
        self.stack.push(BridgeVal::Color3(v));
    }
    pub fn push_vec3(&mut self, v: Vector3) {
        self.stack.push(BridgeVal::Vec3(v));
    }
    pub fn push_cframe(&mut self, v: CoordinateFrame) {
        self.stack.push(BridgeVal::CFrame(v));
    }
    pub fn push_brick(&mut self, v: BrickColor) {
        self.stack.push(BridgeVal::Brick(v));
    }
    // Bridge<T,true>::getValue userdata readers (IDA 0x26c92c..0x26d070,
    // 0x270008): each checks the slot type — standing in for the
    // lua_touserdata + metatable rawequal sequence — and copies the payload
    // on match, else None (false) without raising. Variant out is the
    // BridgeVal itself.
    pub fn get_vec3(&self, idx: i32) -> Option<Vector3> {
        match self.slot(idx) {
            BridgeVal::Vec3(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_color3(&self, idx: i32) -> Option<Color3> {
        match self.slot(idx) {
            BridgeVal::Color3(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_brick(&self, idx: i32) -> Option<BrickColor> {
        match self.slot(idx) {
            BridgeVal::Brick(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_region3(&self, idx: i32) -> Option<Region3> {
        match self.slot(idx) {
            BridgeVal::Region3(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_region3i16(&self, idx: i32) -> Option<Region3int16> {
        match self.slot(idx) {
            BridgeVal::Region3i16(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_vec3i16(&self, idx: i32) -> Option<Vector3int16> {
        match self.slot(idx) {
            BridgeVal::Vec3i16(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_vec2i16(&self, idx: i32) -> Option<Vector2int16> {
        match self.slot(idx) {
            BridgeVal::Vec2i16(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_ray(&self, idx: i32) -> Option<RbxRay> {
        match self.slot(idx) {
            BridgeVal::Ray(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_udim(&self, idx: i32) -> Option<UDim> {
        match self.slot(idx) {
            BridgeVal::UDim(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_udim2(&self, idx: i32) -> Option<UDim2> {
        match self.slot(idx) {
            BridgeVal::UDim2(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_faces(&self, idx: i32) -> Option<Faces> {
        match self.slot(idx) {
            BridgeVal::Faces(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_axes(&self, idx: i32) -> Option<Axes> {
        match self.slot(idx) {
            BridgeVal::Axes(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_cell(&self, idx: i32) -> Option<CellID> {
        match self.slot(idx) {
            BridgeVal::Cell(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_input(&self, idx: i32) -> Option<InputObject> {
        match self.slot(idx) {
            BridgeVal::Input(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_enum_item(&self, idx: i32) -> Option<EnumItemPtr> {
        match self.slot(idx) {
            BridgeVal::EnumItem(v) => Some(*v),
            _ => None,
        }
    }
    fn slot(&self, idx: i32) -> &BridgeVal {
        // Callers here only use 1-based indices, as in the originals.
        &self.stack[(idx - 1) as usize]
    }
    // IDA luaL_checkudata(L, idx, className): the userdata, or a lua_error
    // raise on mismatch. The raise is a panic here.
    pub fn check_vec2(&self, idx: i32) -> Vector2 {
        match self.slot(idx) {
            BridgeVal::Vec2(v) => *v,
            _ => panic!("lua: Vector2 expected (bad argument)"),
        }
    }
    pub fn check_vec3(&self, idx: i32) -> Vector3 {
        match self.slot(idx) {
            BridgeVal::Vec3(v) => *v,
            _ => panic!("lua: Vector3 expected (bad argument)"),
        }
    }
    pub fn check_cframe(&self, idx: i32) -> CoordinateFrame {
        match self.slot(idx) {
            BridgeVal::CFrame(v) => *v,
            _ => panic!("lua: CoordinateFrame expected (bad argument)"),
        }
    }
    pub fn check_color3(&self, idx: i32) -> Color3 {
        match self.slot(idx) {
            BridgeVal::Color3(v) => *v,
            _ => panic!("lua: Color3 expected (bad argument)"),
        }
    }
    // Bridge<CoordinateFrame>::getValue (BLX at 0x273e06): copies the
    // userdata on match, else false without raising (callers fall back to
    // the Vector3 path).
    pub fn get_cframe(&self, idx: i32) -> Option<CoordinateFrame> {
        match self.slot(idx) {
            BridgeVal::CFrame(v) => Some(*v),
            _ => None,
        }
    }
    // IDA luaL_checknumber (BL at 0x2739f4, 0x273b58): double view of the
    // slot with a lua_error raise for non-numbers. The raise is a panic
    // here; numeric strings coerce via lua_tonumber, as with is_number.
    pub fn check_number(&self, idx: i32) -> f32 {
        match self.slot(idx) {
            BridgeVal::Num(v) => *v as f32,
            BridgeVal::Str(s) => match lua_strtod(s) {
                Some(v) => v as f32,
                None => panic!("lua: number expected (bad argument)"),
            },
            _ => panic!("lua: number expected (bad argument)"),
        }
    }
    // Bridge<T>::getValue(L, idx, out): copies the userdata when the slot
    // holds T, else returns false WITHOUT raising (callers fall back to the
    // scalar paths). BLX at 0x272bba / 0x272bcc / 0x272c8a / 0x272c9c.
    pub fn get_vec2(&self, idx: i32) -> Option<Vector2> {
        match self.slot(idx) {
            BridgeVal::Vec2(v) => Some(*v),
            _ => None,
        }
    }
    // IDA lua_isnumber (BL at 0x2730fa): true for numbers and numeric
    // strings (Lua 5.1 coerces via lua_tonumber).
    pub fn is_number(&self, idx: i32) -> bool {
        match self.slot(idx) {
            BridgeVal::Num(_) => true,
            BridgeVal::Str(s) => lua_strtod(s).is_some(),
            _ => false,
        }
    }
    // IDA lua_isstring (BL at 0x27316a): true for strings AND numbers.
    // Numbers never reach it here (isnumber is checked first, 0x2730fa
    // before 0x27316a), so only the Str arm matters in practice.
    pub fn is_string(&self, idx: i32) -> bool {
        matches!(self.slot(idx), BridgeVal::Str(_) | BridgeVal::Num(_))
    }
    // IDA lua_tointeger (BL at 0x273104, 0x27326a): truncates toward zero;
    // non-convertible slots read as 0.
    pub fn to_integer(&self, idx: i32) -> i32 {
        match self.slot(idx) {
            // BUG: original casts (C UB on overflow/NaN); Rust `as`
            // saturates instead. Same value for all in-range inputs.
            BridgeVal::Num(v) => *v as i32,
            BridgeVal::Str(s) => lua_strtod(s).unwrap_or(0.0) as i32,
            _ => 0,
        }
    }
    // IDA RBX::Lua::lua_tofloat(L, idx) (BL at 0x272a4c, 0x273146, ...):
    // float view of the slot; 0.0 for non-numbers.
    pub fn to_float(&self, idx: i32) -> f32 {
        match self.slot(idx) {
            BridgeVal::Num(v) => *v as f32,
            BridgeVal::Str(s) => lua_strtod(s).unwrap_or(0.0) as f32,
            _ => 0.0,
        }
    }
    // IDA lua_tolstring(L, idx, NULL) (BL at 0x273176).
    pub fn to_bytes(&self, idx: i32) -> Vec<u8> {
        match self.slot(idx) {
            BridgeVal::Str(s) => s.clone(),
            // MODEL: Lua coerces numbers via LUA_NUMBER_FMT ("%.14g");
            // Rust {} matches it for integers and short decimals.
            BridgeVal::Num(v) => format!("{v}").into_bytes(),
            _ => Vec::new(),
        }
    }
    // luaL_register(L, className, classLibrary) + lua_setreadonly(L, -1, 1)
    // + lua_settop(L, -2) (IDA 0x272aca..0x272ad6, 0x2730ba..0x2730c6). The
    // classLibrary static only contributes entry addresses, so registration
    // records the class name; the pushed table is popped by the settop.
    pub fn register_class(&mut self, name: &'static str) {
        self.stack.push(BridgeVal::Table { readonly: false });
        if let Some(BridgeVal::Table { readonly }) = self.stack.last_mut() {
            *readonly = true;
        }
        self.stack.pop();
        self.registered_libs.push(name);
    }
}

// Lua 5.1 string→number coercion (lua_tonumber → strtod with a full-string
// check): ASCII whitespace trimmed, then one float literal. Hex float
// literals and inf/nan spellings differ from strtod; unmodeled (MODEL).
fn lua_strtod(s: &[u8]) -> Option<f64> {
    let t = std::str::from_utf8(s).ok()?;
    t.trim().parse::<f64>().ok()
}

// ── RBX::BrickColor value model ───────────────────────────────────────────
// The palette table (RBX::BrickColor::BrickMap singleton, IDA dword_131EBB8 /
// dword_131EBBC, 28-byte entries) is owned by the datamodel batch at
// 0x3043c4..0x304568, so BRICK_PALETTE is still empty here. The helpers take
// the table as a parameter; the public shims pass the (empty) static, which
// reproduces exactly the originals' empty-map fallbacks.
pub struct PaletteEntry {
    pub number: i32,
    pub rgb: [f32; 3],
    pub name: &'static str,
}

pub static BRICK_PALETTE: &[PaletteEntry] = &[];

// RBX::BrickColor::colorPalette (IDA 0x3043c4) + vector size: entry count.
pub fn brick_palette_len() -> i32 {
    BRICK_PALETTE.len() as i32
}

// std::vector<RBX::BrickColor>::at (IDA 0x273294): the number stored at idx.
pub fn brick_palette_at(idx: i32) -> Option<BrickColor> {
    BRICK_PALETTE.get(idx as usize).map(|e| BrickColor(e.number))
}

// RBX::BrickColor::BrickColor(int) (thunk IDA 0x304568): stores the number.
pub fn brick_color_from_number(n: i32) -> BrickColor {
    BrickColor(n)
}

// RBX::BrickColor::parse(char const*) (IDA 0x3043fc): linear scan comparing
// the entry names (std::string::compare at 0x304438), first match wins,
// default 194 when nothing matches (0x304456).
fn parse_in_table(table: &[PaletteEntry], name: &[u8]) -> BrickColor {
    for e in table {
        if e.name.as_bytes() == name {
            return BrickColor(e.number);
        }
    }
    BrickColor(194)
}

pub fn brick_color_parse(name: &[u8]) -> BrickColor {
    parse_in_table(BRICK_PALETTE, name)
}

// RBX::BrickColor::closest(G3D::Color4) (IDA 0x3044c4): L1 distance
// |dr|+|dg|+|db| over the map (VABS/VADD lane triple at 0x304502..0x304536;
// alpha is never read), strict-less-than keeps the first of ties
// (0x304542), exact 0.0 breaks early (0x304552), default 194 (0x3044de).
fn closest_in_table(table: &[PaletteEntry], r: f32, g: f32, b: f32) -> BrickColor {
    let mut best = BrickColor(194);
    let mut best_dist = 10000.0f32; // IDA 0x3044ee
    for e in table {
        let dist = (e.rgb[0] - r).abs() + (e.rgb[1] - g).abs() + (e.rgb[2] - b).abs();
        if dist < best_dist {
            best_dist = dist;
            best = BrickColor(e.number);
            if dist == 0.0 {
                break;
            }
        }
    }
    best
}

pub fn brick_color_closest_rgb(r: f32, g: f32, b: f32) -> BrickColor {
    closest_in_table(BRICK_PALETTE, r, g, b)
}

// RBX::BrickColor::closest(G3D::Color3) (IDA 0x3044a0): repacks r/g/b and
// tail-calls closest(Color4) (BL at 0x3044c2).
pub fn brick_color_closest_c3(c: Color3) -> BrickColor {
    brick_color_closest_rgb(c.r, c.g, c.b)
}

// RBX::BrickColor::random (IDA 0x304468): iRandom over the palette vector.
// MODEL: the table is empty so any index is out of range; fall back to gray
// (194 — also newBrickColor()'s default at IDA 0x27311a).
pub fn brick_color_random() -> BrickColor {
    BrickColor(194)
}

// RBX::BrickColor::name / color3 lookups by number (IDA 0x2734a6 / 0x2734d6).
// MODEL fallbacks while the table is empty: name yields the decimal number
// (informative, deterministic), rgb yields neutral gray. Both flip to table
// data when BRICK_PALETTE lands.
fn name_in_table(table: &[PaletteEntry], bc: BrickColor) -> Vec<u8> {
    for e in table {
        if e.number == bc.0 {
            return e.name.as_bytes().to_vec();
        }
    }
    bc.0.to_string().into_bytes()
}

pub fn brick_color_name(bc: BrickColor) -> Vec<u8> {
    name_in_table(BRICK_PALETTE, bc)
}

fn rgb_in_table(table: &[PaletteEntry], bc: BrickColor) -> [f32; 3] {
    for e in table {
        if e.number == bc.0 {
            return e.rgb;
        }
    }
    [0.5, 0.5, 0.5]
}

pub fn brick_color_rgb(bc: BrickColor) -> [f32; 3] {
    rgb_in_table(BRICK_PALETTE, bc)
}

// ── IMPL batch 2 (13 stubs 0x273674..0x2746bc) ─────────────────────────────
// The CoordinateFrame bridge: constructors (new/fromEulerAnglesXYZ/
// fromAxisAngle), CFrame±Vector3, CFrame*CFrame/Vector3, inverse and the
// toWorldSpace/toObjectSpace/pointTo*/vectorTo* variadics. Grounded from IDA
// decompile + disasm over MCP, including the G3D callees: lookAt pair
// (0xc3ccb8/0xc3ccdc, columns (Y, cross(Y,Z), -Z) decoded from disasm),
// fromAxisAngle pair (0x27797c normalize + 0xc4015c Rodrigues — the decomp
// drops x/z lanes, disasm shows standard Rodrigues), Matrix3(Quat)
// (0xc3f348), fromEulerAnglesXYZ = Rx*Ry*Rz (0xc403a8, all 9 lanes verified
// in disasm), CFrame::operator* (0x5e1350) + Matrix3::operator* (0xc3f5d4),
// CFrame ctor = identity + zero (0xc3c1e4), identity static (0xc3eff8),
// unitize (0xc41cc0).

// ── G3D::CoordinateFrame model ─────────────────────────────────────────────
// G3D::CoordinateFrame is a row-major 3x3 rotation (m[0..8], rows at +0 /
// +12 / +24 — row-loop VLDR in on_mul, IDA 0x273e78..0x273ea8) plus a
// translation Vector3 at +36..+47 (IDA 0x273eb2..0x273ec6, 0xc3cd46..0xc3cd5a).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3 {
    /// Row-major: row r is m[3*r..3*r+3].
    pub m: [f32; 9],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoordinateFrame {
    pub rotation: Matrix3,
    pub translation: Vector3,
}

// G3D::Quat is four floats (x@0, y@4, z@8, w@12 — VLDR triple + w in
// Matrix3::Matrix3(Quat), IDA 0xc3f352..0xc3f366).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub const VECTOR3_CLASS: &str = "Vector3"; // IDA 0x273e4c "Vector3"
pub const COORDFRAME_CLASS: &str = "CoordinateFrame"; // IDA 0x273dcc className ref

// G3D::Matrix3::identity (IDA 0xc3eff8: static 1,0,0,0,1,0,0,0,1).
pub fn matrix3_identity() -> Matrix3 {
    Matrix3 { m: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0] }
}

// G3D::CoordinateFrame::CoordinateFrame() (IDA 0xc3c1e4): identity + zero.
pub fn cframe_identity() -> CoordinateFrame {
    CoordinateFrame {
        rotation: matrix3_identity(),
        translation: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
    }
}

// G3D::Matrix3::operator* (IDA 0xc3f5d4): standard row×col, summed as
// ((a0*b0 + a1*b3) + a2*b6) per lane (VADD triple at 0xc3f614).
pub fn matrix3_mul(a: &Matrix3, b: &Matrix3) -> Matrix3 {
    let mut m = [0.0f32; 9];
    for r in 0..3 {
        for c in 0..3 {
            m[3 * r + c] = (a.m[3 * r] * b.m[c] + a.m[3 * r + 1] * b.m[3 + c])
                + a.m[3 * r + 2] * b.m[6 + c];
        }
    }
    Matrix3 { m }
}

// G3D::CoordinateFrame::operator* (IDA 0x5e1350): R = R1*R2, T = R1*T2 + T1
// with T lanes summed as T1 + ((R0*Tx + R1*Ty) + R2*Tz) (0x5e1440..0x5e1448).
pub fn cframe_mul(a: &CoordinateFrame, b: &CoordinateFrame) -> CoordinateFrame {
    let r = matrix3_mul(&a.rotation, &b.rotation);
    let t = Vector3 {
        x: a.translation.x
            + ((a.rotation.m[0] * b.translation.x + a.rotation.m[1] * b.translation.y)
                + a.rotation.m[2] * b.translation.z),
        y: a.translation.y
            + ((a.rotation.m[3] * b.translation.x + a.rotation.m[4] * b.translation.y)
                + a.rotation.m[5] * b.translation.z),
        z: a.translation.z
            + ((a.rotation.m[6] * b.translation.x + a.rotation.m[7] * b.translation.y)
                + a.rotation.m[8] * b.translation.z),
    };
    CoordinateFrame { rotation: r, translation: t }
}

// Row-dot helper: ((R0*x + R1*y) + R2*z) per row, as in the on_mul /
// pointToWorldSpace lane triples (IDA 0x273e9c, 0x2744c6).
fn mat_vec(m: &Matrix3, v: &Vector3) -> Vector3 {
    Vector3 {
        x: (m.m[0] * v.x + m.m[1] * v.y) + m.m[2] * v.z,
        y: (m.m[3] * v.x + m.m[4] * v.y) + m.m[5] * v.z,
        z: (m.m[6] * v.x + m.m[7] * v.y) + m.m[8] * v.z,
    }
}

// Point transform R*v + T with T added outside the dots, as in
// pointToWorldSpace (IDA 0x2744c6..0x274516: T.x + ((R0*x + R1*y) + R2*z)).
fn cframe_point(cf: &CoordinateFrame, v: &Vector3) -> Vector3 {
    let r = mat_vec(&cf.rotation, v);
    Vector3 {
        x: cf.translation.x + r.x,
        y: cf.translation.y + r.y,
        z: cf.translation.z + r.z,
    }
}

// Transposed-matrix times vector, as in pointToObjectSpace (IDA
// 0x274682..0x2746aa): x' = (R[0]*dx + R[3]*dy) + R[6]*dz, etc.
fn matrix3_transpose_vec(m: &Matrix3, v: &Vector3) -> Vector3 {
    Vector3 {
        x: (m.m[0] * v.x + m.m[3] * v.y) + m.m[6] * v.z,
        y: (m.m[1] * v.x + m.m[4] * v.y) + m.m[7] * v.z,
        z: (m.m[2] * v.x + m.m[5] * v.y) + m.m[8] * v.z,
    }
}

pub fn matrix3_transpose(a: &Matrix3) -> Matrix3 {
    Matrix3 {
        m: [
            a.m[0], a.m[3], a.m[6],
            a.m[1], a.m[4], a.m[7],
            a.m[2], a.m[5], a.m[8],
        ],
    }
}

fn matrix3_neg(a: &Matrix3) -> Matrix3 {
    let mut m = [0.0f32; 9];
    for i in 0..9 {
        m[i] = -a.m[i];
    }
    Matrix3 { m }
}

// Rigid inverse (on_inverse inline at IDA 0x273f6e..0x273fee, identical math
// in on_toObjectSpace at 0x274298..0x274312): transpose() at 0x273f78,
// unary minus at 0x273f9e, then -(Rᵀ)*T row dots. Negation distributes over
// the dots bit-exactly, so negate-then-dot matches the original order.
pub fn cframe_inverse(cf: &CoordinateFrame) -> CoordinateFrame {
    let rt = matrix3_transpose(&cf.rotation);
    let t = mat_vec(&matrix3_neg(&rt), &cf.translation);
    CoordinateFrame { rotation: rt, translation: t }
}

fn vec3_add(a: &Vector3, b: &Vector3) -> Vector3 {
    Vector3 { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z }
}

fn vec3_sub(a: &Vector3, b: &Vector3) -> Vector3 {
    Vector3 { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z }
}

fn vec3_cross(a: &Vector3, b: &Vector3) -> Vector3 {
    Vector3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

fn vec3_dot(a: &Vector3, b: &Vector3) -> f32 {
    (a.x * b.x + a.y * b.y) + a.z * b.z
}

// G3D::Vector3::unitize(tolerance) (IDA 0xc41cc0): normalizes in place via
// VDIV 1/len (0xc41cfa) when len > tol, returning len; otherwise leaves the
// vector untouched and returns 0.
pub fn vec3_unitize(v: &mut Vector3, tol: f32) -> f32 {
    let len = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if len > tol {
        let s = 1.0 / len;
        v.x *= s;
        v.y *= s;
        v.z *= s;
        len
    } else {
        0.0
    }
}

// G3D::Matrix3::Matrix3(Quat) (IDA 0xc3f348): normalizes q (1/sqrt, no
// guard), then the standard quat→matrix rows with x2 = x+x doubling
// (VADD at 0xc3f39a/0xc3f39e).
pub fn matrix3_from_quat(q: &Quat) -> Matrix3 {
    let inv = 1.0 / (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt();
    let (x, y, z, w) = (q.x * inv, q.y * inv, q.z * inv, q.w * inv);
    let (x2, y2, z2) = (x + x, y + y, z + z);
    Matrix3 {
        m: [
            1.0 - y * y2 - z * z2,
            y * x2 - w * z2,
            z * x2 + w * y2,
            y * x2 + w * z2,
            1.0 - x * x2 - z * z2,
            z * y2 - w * x2,
            z * x2 - w * y2,
            z * y2 + w * x2,
            1.0 - x * x2 - y * y2,
        ],
    }
}

// G3D::Matrix3::fromEulerAnglesXYZ(x, y, z) (IDA 0xc403a8): R = Rx*Ry*Rz
// with float sinf/cosf per angle — all 9 lanes verified in disasm
// (R00 = cy*cz at 0xc40418..0xc40466 through R22 = cx*cy at 0xc4050c).
// BUG: host sinf/cosf may differ ~1ulp from the device libm; same formula.
pub fn matrix3_from_euler_xyz(x: f32, y: f32, z: f32) -> Matrix3 {
    let (sx, cx) = (x.sin(), x.cos());
    let (sy, cy) = (y.sin(), y.cos());
    let (sz, cz) = (z.sin(), z.cos());
    Matrix3 {
        m: [
            cy * cz,
            -cy * sz,
            sy,
            cx * sz + sx * sy * cz,
            cx * cz - sx * sy * sz,
            -sx * cy,
            sx * sz - cx * sy * cz,
            cx * sy * sz + sx * cz,
            cx * cy,
        ],
    }
}

// G3D::Matrix3::fromAxisAngle(axis, angle) (IDA 0x27797c + 0xc4015c):
// normalizes the axis (1/sqrt, no zero guard), then standard Rodrigues with
// cos/sin evaluated in double and narrowed to float (VCVT.F32.F64 at
// 0xc40196/0xc401a8 — the decompile drops x/z lanes, disasm shows standard
// Rodrigues; BUG: host double cos/sin may differ ~1ulp from device libm).
pub fn matrix3_from_axis_angle(axis: &Vector3, angle: f32) -> Matrix3 {
    let inv = 1.0 / (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
    let (x, y, z) = (axis.x * inv, axis.y * inv, axis.z * inv);
    let c = (angle as f64).cos() as f32;
    let s = (angle as f64).sin() as f32;
    let t = 1.0 - c;
    Matrix3 {
        m: [
            c + t * x * x,
            t * x * y - s * z,
            t * x * z + s * y,
            t * x * y + s * z,
            c + t * y * y,
            t * y * z - s * x,
            t * x * z - s * y,
            t * y * z + s * x,
            c + t * z * z,
        ],
    }
}

// G3D::CoordinateFrame::lookAt(target, up) rotation (IDA 0xc3ccdc, decoded
// from disasm 0xc3cd1a..0xc3cf2e): up is normalized (VDIV, no guard), Z =
// normalize(target - eye); |up·Z| > 0.99 falls back to unitX then unitY
// (0xc3cdb6..0xc3cf8); X = unitize(up - Z*(up·Z), 1e-6); Y = unitize(Z×X,
// 1e-6); columns become (Y, cross(Y,Z), -Z) via setColumn (0xc3cf1a..0xc3cf4a).
// Translation is untouched (no stores to +36..+47 in the disasm).
pub fn cframe_look_at_rotation(eye: &Vector3, target: &Vector3, up: &Vector3) -> Matrix3 {
    let mut up = *up;
    let inv = 1.0 / (up.x * up.x + up.y * up.y + up.z * up.z).sqrt();
    up.x *= inv;
    up.y *= inv;
    up.z *= inv;
    let mut z = vec3_sub(target, eye);
    let zinv = 1.0 / (z.x * z.x + z.y * z.y + z.z * z.z).sqrt();
    z.x *= zinv;
    z.y *= zinv;
    z.z *= zinv;
    if vec3_dot(&up, &z).abs() > 0.99 {
        up = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
        if vec3_dot(&up, &z).abs() > 0.99 {
            up = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
        }
    }
    let d = vec3_dot(&up, &z);
    let mut x = Vector3 { x: up.x - z.x * d, y: up.y - z.y * d, z: up.z - z.z * d };
    vec3_unitize(&mut x, 1e-6);
    let mut y = vec3_cross(&z, &x);
    vec3_unitize(&mut y, 1e-6);
    let x2 = vec3_cross(&y, &z);
    Matrix3 {
        m: [
            y.x, x2.x, -z.x,
            y.y, x2.y, -z.y,
            y.z, x2.z, -z.z,
        ],
    }
}

// 0x272940 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(G3D::Vector2int16&,char const*,lua_State *)")]
// IDA 0x272940: immutable-vector throw ("%s cannot be assigned to", 0x2729a4),
// same shape as 0x2723d0.
pub fn stub_0x272940(_l: &mut BridgeState, key: &str) -> ! {
    panic!("{key} cannot be assigned to");
}

// 0x2729f8 — __ZN3RBX3Lua13Vector2Bridge10newVector2EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::newVector2(lua_State *)")]
// IDA 0x2729f8: gettop n (0x272a1e); reads min(n,2) lua_tofloat args
// (0x272a48..0x272a58 loop); zero-fills lanes count..2 via memset
// (0x272a88: dst = SP+count*4, len = 4*(2-count)); pushNewObject<float*>
// (0x272a90); returns 1. ___stack_chk_guard prologue at 0x272a04..0x272a16.
pub fn stub_0x2729f8(l: &mut BridgeState) -> i32 {
    let n = l.gettop();
    let mut v = [0.0f32; 2];
    if n >= 1 {
        let count = n.min(2);
        for i in 0..count {
            v[i as usize] = l.to_float(i + 1);
        }
    }
    l.push_vec2(Vector2 { x: v[0], y: v[1] });
    1
}

// 0x272aac — __ZN3RBX3Lua13Vector2Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::registerClassLibrary(lua_State *)")]
// IDA 0x272aac: luaL_register(L, "Vector2", classLibrary) (0x272aca),
// lua_setreadonly(L, -1, 1) (0x272ad6), tail settop(L, -2); 0 results.
pub fn stub_0x272aac(l: &mut BridgeState) -> i32 {
    l.register_class(VECTOR2_CLASS);
    0
}

// 0x272ae8 — __ZN3RBX3Lua13Vector2Bridge6on_addEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::on_add(lua_State *)")]
// IDA 0x272ae8: checkudata args 1-2 (0x272afe/0x272b0a); VLDR x@0/y@4
// (0x272b0e..0x272b1e); VADD per lane (0x272b22/0x272b26);
// pushNewObject<Vector2> by value (0x272b36); returns 1.
pub fn stub_0x272ae8(l: &mut BridgeState) -> i32 {
    let a = l.check_vec2(1);
    let b = l.check_vec2(2);
    l.push_vec2(Vector2 { x: a.x + b.x, y: a.y + b.y });
    1
}

// 0x272b40 — __ZN3RBX3Lua13Vector2Bridge6on_subEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::on_sub(lua_State *)")]
// IDA 0x272b40: same shape as on_add but arg1 - arg2: VSUB D1,D3,D1 gives
// S2 = arg1.x - arg2.x (0x272b7a), VSUB D0,D2,D0 gives S0 = arg1.y - arg2.y
// (0x272b7e); returns 1.
pub fn stub_0x272b40(l: &mut BridgeState) -> i32 {
    let a = l.check_vec2(1);
    let b = l.check_vec2(2);
    l.push_vec2(Vector2 { x: a.x - b.x, y: a.y - b.y });
    1
}

// 0x272b98 — __ZN3RBX3Lua13Vector2Bridge6on_mulEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::on_mul(lua_State *)")]
// IDA 0x272b98: vec*vec when both getValue succeed (VMUL lane-wise at
// 0x272be4/0x272be8); vec*scalar via lua_tofloat(2) (0x272c30..0x272c44);
// scalar*vec after checkudata(arg 2) with lua_tofloat(arg 1)
// (0x272bfc..0x272c26). Pushes the product; returns 1.
pub fn stub_0x272b98(l: &mut BridgeState) -> i32 {
    if let Some(a) = l.get_vec2(1) {
        if let Some(b) = l.get_vec2(2) {
            l.push_vec2(Vector2 { x: a.x * b.x, y: a.y * b.y });
        } else {
            let s = l.to_float(2);
            l.push_vec2(Vector2 { x: a.x * s, y: a.y * s });
        }
    } else {
        let b = l.check_vec2(2);
        let s = l.to_float(1);
        l.push_vec2(Vector2 { x: s * b.x, y: s * b.y });
    }
    1
}

// 0x272c6c — __ZN3RBX3Lua13Vector2Bridge6on_divEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::on_div(lua_State *)")]
// IDA 0x272c6c: vec/vec lane-wise float div (0x272cae/0x272cb2); vec/scalar
// divides by lua_tofloat(2) via G3D::Vector2::operator/ (0x272cfe/0x272d08);
// scalar/vec divides lua_tofloat(1) by each lane of checkudata(arg 2)
// (0x272cce..0x272cec). IEEE div-by-zero (inf/NaN) matches the VDIVS result.
pub fn stub_0x272c6c(l: &mut BridgeState) -> i32 {
    if let Some(a) = l.get_vec2(1) {
        if let Some(b) = l.get_vec2(2) {
            l.push_vec2(Vector2 { x: a.x / b.x, y: a.y / b.y });
        } else {
            let s = l.to_float(2);
            l.push_vec2(Vector2 { x: a.x / s, y: a.y / s });
        }
    } else {
        let b = l.check_vec2(2);
        let s = l.to_float(1);
        l.push_vec2(Vector2 { x: s / b.x, y: s / b.y });
    }
    1
}

// 0x272d28 — __ZN3RBX3Lua13Vector2Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::on_unm(lua_State *)")]
// IDA 0x272d28: unary minus per lane (VNEG at 0x272d58/0x272d5c),
// pushNewObject; returns 1.
pub fn stub_0x272d28(l: &mut BridgeState) -> i32 {
    let a = l.check_vec2(1);
    l.push_vec2(Vector2 { x: -a.x, y: -a.y });
    1
}

// 0x272d70 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int32 *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_index(G3D::Vector2 const&,char const*,lua_State *)")]
// IDA 0x272d70: strcmp dispatch x/X -> x (0x272dde), y/Y -> y (0x272e04),
// "unit" -> v*(1/sqrt(x*x+y*y)) (0x272e1e..0x272edc, no zero guard — IEEE
// inf/NaN preserved), "magnitude" -> sqrt(x*x+y*y) (0x272e32..0x272efa),
// "lerp" -> pushcclosure(lerpVector2) (0x272e5c), else throws "%s is not a
// valid member" (0x272f0c..0x272f44). lua_pushnumber + return 1.
pub fn stub_0x272d70(obj: &Vector2, key: &str, l: &mut BridgeState) -> i32 {
    if key == "x" || key == "X" {
        l.push_number(obj.x as f64);
    } else if key == "y" || key == "Y" {
        l.push_number(obj.y as f64);
    } else if key == "unit" {
        let inv = 1.0 / (obj.x * obj.x + obj.y * obj.y).sqrt();
        l.push_vec2(Vector2 { x: obj.x * inv, y: obj.y * inv });
    } else if key == "magnitude" {
        l.push_number((obj.x * obj.x + obj.y * obj.y).sqrt() as f64);
    } else if key == "lerp" {
        l.push_closure("lerpVector2");
    } else {
        panic!("{key} is not a valid member");
    }
    1
}

// 0x272f6c — __ZN3RBX3LuaL11lerpVector2EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::lerpVector2(lua_State *)")]
// IDA 0x272f6c: checkudata args 1-2 (0x272f88/0x272f92), t = lua_tofloat(3)
// narrowed to float (0x272f98..0x272fa0), a+t*(b-a) per lane in float
// (VMUL/VSUB/VADD at 0x272fcc/0x272fd0); returns 1.
pub fn stub_0x272f6c(l: &mut BridgeState) -> i32 {
    let a = l.check_vec2(1);
    let b = l.check_vec2(2);
    let t = l.to_float(3);
    l.push_vec2(Vector2 { x: a.x + t * (b.x - a.x), y: a.y + t * (b.y - a.y) });
    1
}

// 0x272fe4 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(G3D::Vector2&,char const*,lua_State *)")]
// IDA 0x272fe4: same immutable-vector throw as 0x272940 ("%s cannot be
// assigned to", 0x273048).
pub fn stub_0x272fe4(_l: &mut BridgeState, key: &str) -> ! {
    panic!("{key} cannot be assigned to");
}

// 0x27309c — __ZN3RBX3Lua16BrickColorBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::BrickColorBridge::registerClassLibrary(lua_State *)")]
// IDA 0x27309c: luaL_register(L, "BrickColor", classLibrary) (0x2730ba),
// lua_setreadonly(L, -1, 1) (0x2730c6), tail settop(L, -2); 0 results.
pub fn stub_0x27309c(l: &mut BridgeState) -> i32 {
    l.register_class(BRICKCOLOR_CLASS);
    0
}

// 0x2730d8 — __ZN3RBX3Lua16BrickColorBridge13newBrickColorEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::BrickColorBridge::newBrickColor(lua_State *)")]
// IDA 0x2730d8: n = gettop (0x2730e8), count = min(n,3) (0x2730ea..0x2730f0).
// count == 1: number -> BrickColor(tointeger) (0x2730fa..0x273110), string ->
// BrickColor::parse (0x27316a..0x273182), else Color3 userdata ->
// BrickColor::closest(Color3) (0x2731d4..0x2731ec). Otherwise: count == 0
// pushes gray (194) AND falls through (0x273114..0x27311a, verified in
// disasm — CBNZ skips only when count != 0); r/g/b default to 0,0,0 and a
// to 1.0 (0x273124..0x27312c); count >= 1 reads min(count,3) lua_tofloat
// args (0x273130..0x273152); BrickColor::closest(Color4) (0x2731ae; alpha
// never read — only 3 lanes feed the L1 metric at 0x304502..0x304536).
// Pushes the closest match; returns 1 (so the zero-arg call leaves the
// extra gray below the returned value, as in the original).
pub fn stub_0x2730d8(l: &mut BridgeState) -> i32 {
    let n = l.gettop();
    let count = n.min(3);
    if count == 1 {
        let v = if l.is_number(1) {
            brick_color_from_number(l.to_integer(1))
        } else if l.is_string(1) {
            brick_color_parse(&l.to_bytes(1))
        } else {
            brick_color_closest_c3(l.check_color3(1))
        };
        l.push_brick(v);
    } else {
        if count == 0 {
            l.push_brick(BrickColor(194));
        }
        let mut rgba = [0.0f32, 0.0, 0.0, 1.0];
        if count >= 1 {
            for i in 0..count.min(3) {
                rgba[i as usize] = l.to_float(i + 1);
            }
        }
        l.push_brick(brick_color_closest_rgb(rgba[0], rgba[1], rgba[2]));
    }
    1
}

// 0x2731f0 — __ZN3RBX3Lua16BrickColorBridge16randomBrickColorEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::BrickColorBridge::randomBrickColor(lua_State *)")]
// IDA 0x2731f0: BrickColor::random (0x2731fa), pushNewObject (0x273202);
// returns 1. Randomness source is MODEL (brick_color_random).
pub fn stub_0x2731f0(l: &mut BridgeState) -> i32 {
    l.push_brick(brick_color_random());
    1
}

// 0x27320c — __ZN3RBX3Lua16BrickColorBridge17paletteBrickColorEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::BrickColorBridge::paletteBrickColor(lua_State *)")]
// IDA 0x27320c: idx = 0; when gettop >= 1, idx = lua_tointeger(1) (0x27326a)
// and idx < 0 throws (0x273270..0x2732c0); size = palette bytes >> 2
// (0x27327e..0x273282) and idx >= size throws (0x273284..0x273288); pushes
// palette.at(idx) (0x273294..0x27329e); returns 1. Message: "palette index
// out of bounds (%d)" (0x2732d8).
pub fn stub_0x27320c(l: &mut BridgeState) -> i32 {
    let mut idx = 0;
    if l.gettop() >= 1 {
        idx = l.to_integer(1);
        if idx < 0 {
            panic!("palette index out of bounds ({idx})");
        }
    }
    if idx >= brick_palette_len() {
        panic!("palette index out of bounds ({idx})");
    }
    l.push_brick(brick_palette_at(idx).expect("palette bounds checked above"));
    1
}

// 0x273330 — __ZN3RBX3LuaL9pushWhiteEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushWhite(lua_State *)")]
// IDA 0x273330: pushNewObject<BrickColor>(L, 1) (0x273336); returns 1.
pub fn stub_0x273330(l: &mut BridgeState) -> i32 {
    l.push_brick(BrickColor(1));
    1
}

// 0x273340 — __ZN3RBX3LuaL8pushGrayEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushGray(lua_State *)")]
// IDA 0x273340: pushNewObject<BrickColor>(L, 194) (0x273346); returns 1.
pub fn stub_0x273340(l: &mut BridgeState) -> i32 {
    l.push_brick(BrickColor(194));
    1
}

// 0x273350 — __ZN3RBX3LuaL12pushDarkGrayEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushDarkGray(lua_State *)")]
// IDA 0x273350: pushNewObject<BrickColor>(L, 199) (0x273356); returns 1.
pub fn stub_0x273350(l: &mut BridgeState) -> i32 {
    l.push_brick(BrickColor(199));
    1
}

// 0x273360 — __ZN3RBX3LuaL9pushBlackEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushBlack(lua_State *)")]
// IDA 0x273360: pushNewObject<BrickColor>(L, 26) (0x273366); returns 1.
pub fn stub_0x273360(l: &mut BridgeState) -> i32 {
    l.push_brick(BrickColor(26));
    1
}

// 0x273370 — __ZN3RBX3LuaL7pushRedEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushRed(lua_State *)")]
// IDA 0x273370: pushNewObject<BrickColor>(L, 21) (0x273376); returns 1.
pub fn stub_0x273370(l: &mut BridgeState) -> i32 {
    l.push_brick(BrickColor(21));
    1
}

// 0x273380 — __ZN3RBX3LuaL10pushYellowEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushYellow(lua_State *)")]
// IDA 0x273380: pushNewObject<BrickColor>(L, 24) (0x273386); returns 1.
pub fn stub_0x273380(l: &mut BridgeState) -> i32 {
    l.push_brick(BrickColor(24));
    1
}

// 0x273390 — __ZN3RBX3LuaL9pushGreenEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushGreen(lua_State *)")]
// IDA 0x273390: pushNewObject<BrickColor>(L, 28) (0x273396); returns 1.
pub fn stub_0x273390(l: &mut BridgeState) -> i32 {
    l.push_brick(BrickColor(28));
    1
}

// 0x2733a0 — __ZN3RBX3LuaL8pushBlueEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushBlue(lua_State *)")]
// IDA 0x2733a0: pushNewObject<BrickColor>(L, 23) (0x2733a6); returns 1.
pub fn stub_0x2733a0(l: &mut BridgeState) -> i32 {
    l.push_brick(BrickColor(23));
    1
}

// 0x2733b0 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(RBX::BrickColor *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_index(RBX::BrickColor const&,char const*,lua_State *)")]
// IDA 0x2733b0: strcmp dispatch "number"/"Number" -> pushinteger(number)
// (0x27341e..0x2734c8), "Color" -> push Color3(color3()) (0x273438..0x2734e4),
// "r"/"g"/"b" -> pushnumber of the color3() lane (0x27344c..0x27352c),
// "name"/"Name" -> pushlstring(name()) (0x273496..0x27354c), else throws
// "%s is not a valid member" (0x27355e..0x273596). Returns 1.
pub fn stub_0x2733b0(obj: &BrickColor, key: &str, l: &mut BridgeState) -> i32 {
    if key == "number" || key == "Number" {
        l.push_integer(obj.0);
    } else if key == "Color" {
        let rgb = brick_color_rgb(*obj);
        l.push_color3(Color3 { r: rgb[0], g: rgb[1], b: rgb[2] });
    } else if key == "r" {
        l.push_number(brick_color_rgb(*obj)[0] as f64);
    } else if key == "g" {
        l.push_number(brick_color_rgb(*obj)[1] as f64);
    } else if key == "b" {
        l.push_number(brick_color_rgb(*obj)[2] as f64);
    } else if key == "name" || key == "Name" {
        l.push_str(&brick_color_name(*obj));
    } else {
        panic!("{key} is not a valid member");
    }
    1
}

// 0x2735bc — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_newindex(RBX::BrickColor&,char const*,lua_State *)")]
// IDA 0x2735bc: same immutable-value throw as the vector bridges ("%s
// cannot be assigned to", 0x273620).
pub fn stub_0x2735bc(_l: &mut BridgeState, key: &str) -> ! {
    panic!("{key} cannot be assigned to");
}

// 0x273674 — __ZN3RBX3Lua21CoordinateFrameBridge18newCoordinateFrameEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::newCoordinateFrame(lua_State *)")]
// IDA 0x273674: identity CFrame (0x273696), then a gettop switch (0x2736ce):
// 0 args -> identity; 1 Vector3 arg -> translation (0x273708..0x273718);
// 2 Vector3 args -> translation = arg 1, rotation = lookAt(arg 2) with
// unitY up (0x27372e..0x273762); 3 floats -> translation (0x273766..0x27378a);
// 7 floats -> translation + Matrix3(Quat(args 4-7)) (0x273790..0x27384a);
// 12 floats -> translation + row-major rotation (0x27384e..0x2738c6);
// else throws "Invalid number of arguments: %d" (0x27391a..0x27398c).
// Pushes the frame; returns 1.
pub fn stub_0x273674(l: &mut BridgeState) -> i32 {
    let n = l.gettop();
    let mut cf = cframe_identity();
    match n {
        0 => {}
        1 => {
            cf.translation = l.check_vec3(1);
        }
        2 => {
            cf.translation = l.check_vec3(1);
            let target = l.check_vec3(2);
            cf.rotation = cframe_look_at_rotation(
                &cf.translation,
                &target,
                &Vector3 { x: 0.0, y: 1.0, z: 0.0 },
            );
        }
        3 => {
            cf.translation =
                Vector3 { x: l.to_float(1), y: l.to_float(2), z: l.to_float(3) };
        }
        7 => {
            cf.translation =
                Vector3 { x: l.to_float(1), y: l.to_float(2), z: l.to_float(3) };
            let q = Quat {
                x: l.to_float(4),
                y: l.to_float(5),
                z: l.to_float(6),
                w: l.to_float(7),
            };
            cf.rotation = matrix3_from_quat(&q);
        }
        12 => {
            cf.translation =
                Vector3 { x: l.to_float(1), y: l.to_float(2), z: l.to_float(3) };
            let mut m = [0.0f32; 9];
            for i in 0..9 {
                m[i] = l.to_float(4 + i as i32);
            }
            cf.rotation = Matrix3 { m };
        }
        _ => panic!("Invalid number of arguments: {n}"),
    }
    l.push_cframe(cf);
    1
}

// 0x27399c — __ZN3RBX3Lua21CoordinateFrameBridge18fromEulerAnglesXYZEP9lua_State
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::fromEulerAnglesXYZ(lua_State *)")]
// IDA 0x27399c: luaL_checknumber args 1-3 (0x2739f4..0x273a22),
// Matrix3::fromEulerAnglesXYZ (0x273a4a), zero translation; returns 1.
pub fn stub_0x27399c(l: &mut BridgeState) -> i32 {
    let mut cf = cframe_identity();
    cf.rotation = matrix3_from_euler_xyz(
        l.check_number(1),
        l.check_number(2),
        l.check_number(3),
    );
    l.push_cframe(cf);
    1
}

// 0x273ad8 — __ZN3RBX3Lua21CoordinateFrameBridge13fromAxisAngleEP9lua_State
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::fromAxisAngle(lua_State *)")]
// IDA 0x273ad8: checkudata Vector3 arg 1 (0x273b42), luaL_checknumber arg 2
// (0x273b58), Matrix3::fromAxisAngle (0x273b64), zero translation; returns 1.
pub fn stub_0x273ad8(l: &mut BridgeState) -> i32 {
    let axis = l.check_vec3(1);
    let angle = l.check_number(2);
    let mut cf = cframe_identity();
    cf.rotation = matrix3_from_axis_angle(&axis, angle);
    l.push_cframe(cf);
    1
}

// 0x273bf0 — __ZN3RBX3Lua21CoordinateFrameBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::registerClassLibrary(lua_State *)")]
// IDA 0x273bf0: luaL_register(L, "CoordinateFrame", classLibrary)
// (0x273c0e), lua_setreadonly(L, -1, 1) (0x273c1a), tail settop(L, -2);
// 0 results.
pub fn stub_0x273bf0(l: &mut BridgeState) -> i32 {
    l.register_class(COORDFRAME_CLASS);
    0
}

// 0x273c2c — __ZN3RBX3Lua21CoordinateFrameBridge6on_addEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_add(lua_State *)")]
// IDA 0x273c2c: checkudata CFrame arg 1 (0x273c5a) + Vector3 arg 2
// (0x273c6c); copies the rotation, adds the translation lanes (VADD at
// 0x273ca4..0x273cac); returns 1.
pub fn stub_0x273c2c(l: &mut BridgeState) -> i32 {
    let a = l.check_cframe(1);
    let b = l.check_vec3(2);
    l.push_cframe(CoordinateFrame {
        rotation: a.rotation,
        translation: vec3_add(&a.translation, &b),
    });
    1
}

// 0x273ce0 — __ZN3RBX3Lua21CoordinateFrameBridge6on_subEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_sub(lua_State *)")]
// IDA 0x273ce0: same shape as on_add but T - v (VSUB at 0x273d58..0x273d60);
// returns 1.
pub fn stub_0x273ce0(l: &mut BridgeState) -> i32 {
    let a = l.check_cframe(1);
    let b = l.check_vec3(2);
    l.push_cframe(CoordinateFrame {
        rotation: a.rotation,
        translation: vec3_sub(&a.translation, &b),
    });
    1
}

// 0x273d94 — __ZN3RBX3Lua21CoordinateFrameBridge6on_mulEP9lua_State
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_mul(lua_State *)")]
// IDA 0x273d94: checkudata CFrame arg 1 (0x273dcc); CFrame arg 2 via
// getValue composes with operator* (0x273e18); else checkudata Vector3 arg
// 2 (0x273e56) point-transforms with w = 1: R*v + T (row dots at
// 0x273e78..0x273ea8, translation added at 0x273eb2..0x273ece, Vector4 xyz
// path at 0x273eec..0x273efa — verified in disasm, unlike the rotate-only
// vectorToWorldSpace loop). Returns 1.
pub fn stub_0x273d94(l: &mut BridgeState) -> i32 {
    let a = l.check_cframe(1);
    if let Some(b) = l.get_cframe(2) {
        l.push_cframe(cframe_mul(&a, &b));
    } else {
        l.push_vec3(cframe_point(&a, &l.check_vec3(2)));
    }
    1
}

// 0x273f48 — __ZN3RBX3Lua21CoordinateFrameBridge10on_inverseEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_inverse(lua_State *)")]
// IDA 0x273f48: checkudata CFrame arg 1 (0x273f6a); rigid inverse via
// transpose (0x273f78) + unary minus (0x273f9e) + -(Rᵀ)*T dots
// (0x273fbe..0x273fee); returns 1.
pub fn stub_0x273f48(l: &mut BridgeState) -> i32 {
    l.push_cframe(cframe_inverse(&l.check_cframe(1)));
    1
}

// 0x274024 — __ZN3RBX3Lua21CoordinateFrameBridge15on_toWorldSpaceEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_toWorldSpace(lua_State *)")]
// IDA 0x274024: checkudata CFrame arg 1 (0x274044); 1 arg -> pushes a copy
// (0x2740c2..0x2740da), return 1; k >= 1 extra CFrame args -> pushes
// self*arg per arg (operator* at 0x27408e), returns n-1; 0 args returns -1
// (0x2740e2 quirk, no pushes).
pub fn stub_0x274024(l: &mut BridgeState) -> i32 {
    let a = l.check_cframe(1);
    let n = l.gettop();
    if n == 1 {
        l.push_cframe(a);
        1
    } else if n - 1 >= 1 {
        for i in 2..=n {
            l.push_cframe(cframe_mul(&a, &l.check_cframe(i)));
        }
        n - 1
    } else {
        n - 1
    }
}

// 0x2740e4 — __ZN3RBX3Lua21CoordinateFrameBridge16on_toObjectSpaceEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_toObjectSpace(lua_State *)")]
// IDA 0x2740e4: toWorldSpace shape with inverse(self): 1 arg -> pushes the
// rigid inverse (transpose + -(Rᵀ)*T inline at 0x274298..0x274312, same math
// as on_inverse), return 1; extra CFrame args -> pushes inverse*arg per arg
// (0x274238), returns n-1; 0 args returns -1 (0x274352 quirk).
pub fn stub_0x2740e4(l: &mut BridgeState) -> i32 {
    let inv = cframe_inverse(&l.check_cframe(1));
    let n = l.gettop();
    if n == 1 {
        l.push_cframe(inv);
        1
    } else if n - 1 >= 1 {
        for i in 2..=n {
            l.push_cframe(cframe_mul(&inv, &l.check_cframe(i)));
        }
        n - 1
    } else {
        n - 1
    }
}

// 0x274394 — __ZN3RBX3Lua21CoordinateFrameBridge20on_pointToWorldSpaceEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_pointToWorldSpace(lua_State *)")]
// IDA 0x274394: checkudata CFrame arg 1 (0x2743b2); 1 arg -> transforms the
// zero vector, i.e. pushes R*0+T (0x274480..0x27451a), return 1; extra
// Vector3 args -> pushes R*v+T per arg (0x2743e0..0x274476), returns n-1;
// 0 args returns -1 (0x274524 quirk).
pub fn stub_0x274394(l: &mut BridgeState) -> i32 {
    let a = l.check_cframe(1);
    let n = l.gettop();
    if n == 1 {
        l.push_vec3(cframe_point(&a, &Vector3 { x: 0.0, y: 0.0, z: 0.0 }));
        1
    } else if n - 1 >= 1 {
        for i in 2..=n {
            l.push_vec3(cframe_point(&a, &l.check_vec3(i)));
        }
        n - 1
    } else {
        n - 1
    }
}

// 0x274528 — __ZN3RBX3Lua21CoordinateFrameBridge21on_pointToObjectSpaceEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_pointToObjectSpace(lua_State *)")]
// IDA 0x274528: pointToWorldSpace shape with Rᵀ*(v-T): d = v-T per lane
// (VSUB at 0x274624..0x27463e), pushed lanes use the transposed rows
// (0x274682..0x2746aa); single-arg path transforms zero, i.e. Rᵀ*(0-T)
// with 0.0-T per lane (0x274618..0x2746aa); returns n-1 (-1 with 0 args).
pub fn stub_0x274528(l: &mut BridgeState) -> i32 {
    let a = l.check_cframe(1);
    let n = l.gettop();
    let unpoint = |v: &Vector3| {
        let d = Vector3 {
            x: v.x - a.translation.x,
            y: v.y - a.translation.y,
            z: v.z - a.translation.z,
        };
        matrix3_transpose_vec(&a.rotation, &d)
    };
    if n == 1 {
        l.push_vec3(unpoint(&Vector3 { x: 0.0, y: 0.0, z: 0.0 }));
        1
    } else if n - 1 >= 1 {
        for i in 2..=n {
            l.push_vec3(unpoint(&l.check_vec3(i)));
        }
        n - 1
    } else {
        n - 1
    }
}

// 0x2746bc — __ZN3RBX3Lua21CoordinateFrameBridge21on_vectorToWorldSpaceEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_vectorToWorldSpace(lua_State *)")]
// IDA 0x2746bc: pointToWorldSpace shape WITHOUT the translation: pushes
// R*v row dots per arg (0x27472e..0x27475e loop, no T add); single-arg path
// pushes R*0 (0x274774..0x2747cc); returns n-1 (-1 with 0 args, 0x2747de).
pub fn stub_0x2746bc(l: &mut BridgeState) -> i32 {
    let a = l.check_cframe(1);
    let n = l.gettop();
    if n == 1 {
        l.push_vec3(mat_vec(&a.rotation, &Vector3 { x: 0.0, y: 0.0, z: 0.0 }));
        1
    } else if n - 1 >= 1 {
        for i in 2..=n {
            l.push_vec3(mat_vec(&a.rotation, &l.check_vec3(i)));
        }
        n - 1
    } else {
        n - 1
    }
}

#[cfg(test)]
mod vector2_brickcolor_bridge_tests {
    use super::*;

    fn vec2(x: f32, y: f32) -> BridgeVal {
        BridgeVal::Vec2(Vector2 { x, y })
    }
    fn num(v: f64) -> BridgeVal {
        BridgeVal::Num(v)
    }
    fn state(vals: Vec<BridgeVal>) -> BridgeState {
        let mut l = BridgeState::new();
        for v in vals {
            l.stack.push(v);
        }
        l
    }

    #[test]
    fn new_vector2_reads_min_of_n_and_2_and_zero_fills() {
        let mut l = state(vec![]);
        assert_eq!(stub_0x2729f8(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(0.0, 0.0)));
        let mut l = state(vec![num(1.5)]);
        assert_eq!(stub_0x2729f8(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(1.5, 0.0)));
        let mut l = state(vec![num(1.5), num(-2.25)]);
        assert_eq!(stub_0x2729f8(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(1.5, -2.25)));
        // Extra args are ignored (count clamps at 2, IDA 0x272a38).
        let mut l = state(vec![num(1.0), num(2.0), num(3.0), num(4.0)]);
        assert_eq!(stub_0x2729f8(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(1.0, 2.0)));
    }

    #[test]
    fn register_class_library_returns_0_and_records() {
        let mut l = BridgeState::new();
        assert_eq!(stub_0x272aac(&mut l), 0);
        assert_eq!(l.registered_libs, vec!["Vector2"]);
        // The pushed table is popped by settop(L, -2): no stack growth.
        assert_eq!(l.gettop(), 0);
        let mut l = BridgeState::new();
        assert_eq!(stub_0x27309c(&mut l), 0);
        assert_eq!(l.registered_libs, vec!["BrickColor"]);
        assert_eq!(l.gettop(), 0);
    }

    #[test]
    fn add_sub_unm_match_f32_lane_math() {
        let mut l = state(vec![vec2(1.0, 2.0), vec2(4.0, -1.0)]);
        assert_eq!(stub_0x272ae8(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(5.0, 1.0)));
        let mut l = state(vec![vec2(1.0, 2.0), vec2(4.0, -1.0)]);
        assert_eq!(stub_0x272b40(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(-3.0, 3.0)));
        let mut l = state(vec![vec2(1.5, -2.0)]);
        assert_eq!(stub_0x272d28(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(-1.5, 2.0)));
    }

    #[test]
    fn mul_covers_vec_vec_and_both_scalar_orders() {
        let mut l = state(vec![vec2(2.0, 3.0), vec2(4.0, -0.5)]);
        assert_eq!(stub_0x272b98(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(8.0, -1.5)));
        // vec*scalar: getValue(arg 2) fails, lua_tofloat(2) (0x272c30).
        let mut l = state(vec![vec2(2.0, 3.0), num(4.0)]);
        assert_eq!(stub_0x272b98(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(8.0, 12.0)));
        // scalar*vec: getValue(arg 1) fails, checkudata(arg 2) (0x272bfc).
        let mut l = state(vec![num(4.0), vec2(2.0, 3.0)]);
        assert_eq!(stub_0x272b98(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(8.0, 12.0)));
    }

    #[test]
    fn div_covers_vec_vec_and_both_scalar_orders() {
        let mut l = state(vec![vec2(8.0, 3.0), vec2(4.0, -0.5)]);
        assert_eq!(stub_0x272c6c(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(2.0, -6.0)));
        let mut l = state(vec![vec2(8.0, 3.0), num(4.0)]);
        assert_eq!(stub_0x272c6c(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(2.0, 0.75)));
        let mut l = state(vec![num(8.0), vec2(4.0, -0.5)]);
        assert_eq!(stub_0x272c6c(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(2.0, -16.0)));
    }

    #[test]
    fn div_by_zero_follows_ieee_like_vdivs() {
        let mut l = state(vec![vec2(1.0, 0.0), vec2(0.0, 0.0)]);
        assert_eq!(stub_0x272c6c(&mut l), 1);
        // NaN != NaN, so compare lanes by predicate: 1/0 = inf, 0/0 = NaN.
        match l.stack.last() {
            Some(BridgeVal::Vec2(v)) => {
                assert!(v.x.is_infinite() && v.x > 0.0);
                assert!(v.y.is_nan());
            }
            other => panic!("expected Vec2, got {other:?}"),
        }
    }

    #[test]
    fn index_serves_xy_unit_magnitude_lerp() {
        let v = Vector2 { x: 3.0, y: 4.0 };
        let mut l = BridgeState::new();
        assert_eq!(stub_0x272d70(&v, "x", &mut l), 1);
        assert_eq!(stub_0x272d70(&v, "Y", &mut l), 1);
        assert_eq!(
            l.stack.as_slice(),
            &[BridgeVal::Num(3.0), BridgeVal::Num(4.0)]
        );
        // unit = v*(1/sqrt(25)) with the original's op order (0x272eb8..0x272ed0).
        let inv = 1.0f32 / 25.0f32.sqrt();
        let mut l = BridgeState::new();
        assert_eq!(stub_0x272d70(&v, "unit", &mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&vec2(3.0 * inv, 4.0 * inv))
        );
        let mut l = BridgeState::new();
        assert_eq!(stub_0x272d70(&v, "magnitude", &mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Num(5.0)));
        let mut l = BridgeState::new();
        assert_eq!(stub_0x272d70(&v, "lerp", &mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Closure("lerpVector2")));
    }

    #[test]
    #[should_panic(expected = "not a valid member")]
    fn index_rejects_unknown_member() {
        let v = Vector2 { x: 1.0, y: 2.0 };
        stub_0x272d70(&v, "z", &mut BridgeState::new());
    }

    #[test]
    fn lerp_is_a_plus_t_times_b_minus_a_in_float() {
        let mut l = state(vec![vec2(1.0, 2.0), vec2(3.0, 6.0), num(0.5)]);
        assert_eq!(stub_0x272f6c(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2(2.0, 4.0)));
    }

    #[test]
    #[should_panic(expected = "cannot be assigned to")]
    fn vector2int16_newindex_throws() {
        stub_0x272940(&mut BridgeState::new(), "x");
    }

    #[test]
    #[should_panic(expected = "cannot be assigned to")]
    fn vector2_newindex_throws() {
        stub_0x272fe4(&mut BridgeState::new(), "y");
    }

    #[test]
    #[should_panic(expected = "cannot be assigned to")]
    fn brickcolor_newindex_throws() {
        stub_0x2735bc(&mut BridgeState::new(), "number");
    }

    #[test]
    fn push_color_helpers_push_grounded_numbers() {
        let cases = [
            (stub_0x273330 as fn(&mut BridgeState) -> i32, 1),
            (stub_0x273340, 194),
            (stub_0x273350, 199),
            (stub_0x273360, 26),
            (stub_0x273370, 21),
            (stub_0x273380, 24),
            (stub_0x273390, 28),
            (stub_0x2733a0, 23),
        ];
        for (f, number) in cases {
            let mut l = BridgeState::new();
            assert_eq!(f(&mut l), 1);
            assert_eq!(l.stack.last(), Some(&BridgeVal::Brick(BrickColor(number))));
        }
    }

    #[test]
    fn new_brickcolor_dispatches_on_single_arg_kind() {
        // Number -> BrickColor(tointeger) (0x2730fa..0x273110).
        let mut l = state(vec![num(21.0)]);
        assert_eq!(stub_0x2730d8(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Brick(BrickColor(21))));
        // String -> parse (MODEL: empty table pins 194).
        let mut l = state(vec![BridgeVal::Str(b"White".to_vec())]);
        assert_eq!(stub_0x2730d8(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Brick(BrickColor(194))));
        // Color3 userdata -> closest (MODEL: empty table pins 194).
        let mut l = state(vec![BridgeVal::Color3(Color3 { r: 1.0, g: 1.0, b: 1.0 })]);
        assert_eq!(stub_0x2730d8(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Brick(BrickColor(194))));
        // r/g/b floats -> closest (MODEL: empty table pins 194, one push).
        let mut l = state(vec![num(1.0), num(0.0), num(0.0)]);
        assert_eq!(stub_0x2730d8(&mut l), 1);
        assert_eq!(l.gettop(), 4);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Brick(BrickColor(194))));
    }

    #[test]
    fn new_brickcolor_without_args_pushes_twice_like_original() {
        // IDA 0x273114..0x27311a falls through after pushing gray (194), so
        // the closest() result lands on top; return 1 keeps the top.
        let mut l = BridgeState::new();
        assert_eq!(stub_0x2730d8(&mut l), 1);
        assert_eq!(
            l.stack.as_slice(),
            &[
                BridgeVal::Brick(BrickColor(194)),
                BridgeVal::Brick(BrickColor(194)),
            ]
        );
    }

    #[test]
    fn palette_rejects_out_of_bounds_with_original_message() {
        // MODEL-pinned: the table is empty, so even index 0 is out of range.
        assert_eq!(brick_palette_len(), 0);
    }

    #[test]
    #[should_panic(expected = "palette index out of bounds (0)")]
    fn palette_without_args_throws_while_table_empty() {
        stub_0x27320c(&mut BridgeState::new());
    }

    #[test]
    #[should_panic(expected = "palette index out of bounds (-3)")]
    fn palette_rejects_negative_index() {
        let mut l = state(vec![num(-3.0)]);
        stub_0x27320c(&mut l);
    }

    #[test]
    fn brick_index_serves_number_and_rejects_unknown() {
        let bc = BrickColor(21);
        let mut l = BridgeState::new();
        assert_eq!(stub_0x2733b0(&bc, "number", &mut l), 1);
        assert_eq!(stub_0x2733b0(&bc, "Name", &mut l), 1);
        assert_eq!(
            l.stack.as_slice(),
            &[
                BridgeVal::Num(21.0),
                // MODEL-pinned: empty table falls back to the decimal number.
                BridgeVal::Str(b"21".to_vec()),
            ]
        );
    }

    #[test]
    #[should_panic(expected = "not a valid member")]
    fn brick_index_rejects_unknown_member() {
        stub_0x2733b0(&BrickColor(1), "foo", &mut BridgeState::new());
    }

    #[test]
    fn closest_uses_l1_with_first_of_ties_and_zero_early_out() {
        // Grounded algorithm (IDA 0x3044c4) against a synthetic table.
        let table = [
            PaletteEntry { number: 1, rgb: [0.0, 0.0, 0.0], name: "Black" },
            PaletteEntry { number: 2, rgb: [1.0, 1.0, 1.0], name: "White" },
            PaletteEntry { number: 3, rgb: [1.0, 0.0, 0.0], name: "Red" },
        ];
        assert_eq!(closest_in_table(&table, 0.9, 0.9, 0.9), BrickColor(2));
        assert_eq!(closest_in_table(&table, 1.0, 0.0, 0.0), BrickColor(3));
        // Equidistant (0.5 L1 to both ends): strict < keeps the first.
        assert_eq!(closest_in_table(&table, 0.5, 0.5, 0.5), BrickColor(1));
        assert_eq!(closest_in_table(&[], 0.0, 0.0, 0.0), BrickColor(194));
        assert_eq!(parse_in_table(&table, b"Red"), BrickColor(3));
        assert_eq!(parse_in_table(&table, b"Missing"), BrickColor(194));
    }

    #[test]
    fn random_falls_back_to_gray_while_table_empty() {
        let mut l = BridgeState::new();
        assert_eq!(stub_0x2731f0(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Brick(BrickColor(194))));
    }

    #[test]
    fn lua_string_number_coercion_matches_strtod_shape() {
        assert_eq!(lua_strtod(b"  12.5 "), Some(12.5));
        assert_eq!(lua_strtod(b"abc"), None);
        assert_eq!(lua_strtod(b"12abc"), None);
        let l = state(vec![BridgeVal::Str(b"21".to_vec())]);
        assert!(l.is_number(1));
        assert_eq!(l.to_integer(1), 21);
    }
}

#[cfg(test)]
mod coordinate_frame_bridge_tests {
    use super::*;

    fn vec3(x: f32, y: f32, z: f32) -> BridgeVal {
        BridgeVal::Vec3(Vector3 { x, y, z })
    }
    fn cf(rot: [f32; 9], t: (f32, f32, f32)) -> BridgeVal {
        BridgeVal::CFrame(CoordinateFrame {
            rotation: Matrix3 { m: rot },
            translation: Vector3 { x: t.0, y: t.1, z: t.2 },
        })
    }
    fn state(vals: Vec<BridgeVal>) -> BridgeState {
        let mut l = BridgeState::new();
        for v in vals {
            l.stack.push(v);
        }
        l
    }
    fn identity() -> CoordinateFrame {
        cframe_identity()
    }

    #[test]
    fn new_without_args_is_identity() {
        let mut l = state(vec![]);
        assert_eq!(stub_0x273674(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::CFrame(identity())));
    }

    #[test]
    fn new_with_vector3_sets_translation_only() {
        let mut l = state(vec![vec3(1.0, 2.0, 3.0)]);
        assert_eq!(stub_0x273674(&mut l), 1);
        let mut want = identity();
        want.translation = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(l.stack.last(), Some(&BridgeVal::CFrame(want)));
    }

    #[test]
    fn new_with_3_and_12_floats() {
        let mut l = state(vec![
            BridgeVal::Num(1.0),
            BridgeVal::Num(2.0),
            BridgeVal::Num(3.0),
        ]);
        assert_eq!(stub_0x273674(&mut l), 1);
        let mut want = identity();
        want.translation = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(l.stack.last(), Some(&BridgeVal::CFrame(want)));
        // 12 floats: translation + row-major rotation passthrough.
        let mut args = vec![BridgeVal::Num(5.0), BridgeVal::Num(6.0), BridgeVal::Num(7.0)];
        for i in 0..9 {
            args.push(BridgeVal::Num(i as f64));
        }
        let mut l = state(args);
        assert_eq!(stub_0x273674(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&cf(
                [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
                (5.0, 6.0, 7.0)
            ))
        );
    }

    #[test]
    fn new_with_quaternion_identity_stays_identity() {
        // Identity quat (0,0,0,1) normalizes to itself; matrix is exact.
        let mut l = state(vec![
            BridgeVal::Num(9.0),
            BridgeVal::Num(9.0),
            BridgeVal::Num(9.0),
            BridgeVal::Num(0.0),
            BridgeVal::Num(0.0),
            BridgeVal::Num(0.0),
            BridgeVal::Num(1.0),
        ]);
        assert_eq!(stub_0x273674(&mut l), 1);
        let mut want = identity();
        want.translation = Vector3 { x: 9.0, y: 9.0, z: 9.0 };
        assert_eq!(l.stack.last(), Some(&BridgeVal::CFrame(want)));
    }

    #[test]
    fn new_with_eye_and_target_aims_columns_y_x2_neg_z() {
        // eye = origin, target = +Z: z = (0,0,1); up = Y; dot = 0;
        // x = Y; y = Z×X = (-1,0,0); x2 = Y×Z = (0,1,0);
        // columns (Y, X2, -Z) -> rows [-1,0,0, 0,1,0, 0,0,-1].
        let mut l = state(vec![vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0)]);
        assert_eq!(stub_0x273674(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&cf(
                [-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0],
                (0.0, 0.0, 0.0)
            ))
        );
    }

    #[test]
    fn new_with_straight_up_target_takes_degenerate_up_path() {
        // target = +Y (parallel to up): |dot| = 1 > 0.99 -> up falls back to
        // X; z = (0,1,0); x = X; y = Z×X = (0,0,-1); x2 = Y×Z = (1,0,0);
        // columns (Y, X2, -Z) -> rows [0,1,0, 0,0,-1, -1,0,0].
        let mut l = state(vec![vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0)]);
        assert_eq!(stub_0x273674(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&cf(
                [0.0, 1.0, 0.0, 0.0, 0.0, -1.0, -1.0, 0.0, 0.0],
                (0.0, 0.0, 0.0)
            ))
        );
    }

    #[test]
    #[should_panic(expected = "Invalid number of arguments: 5")]
    fn new_with_bad_count_throws_original_message() {
        let mut l = state(vec![
            BridgeVal::Num(1.0),
            BridgeVal::Num(2.0),
            BridgeVal::Num(3.0),
            BridgeVal::Num(4.0),
            BridgeVal::Num(5.0),
        ]);
        stub_0x273674(&mut l);
    }

    #[test]
    fn euler_zero_is_identity_and_rejects_non_numbers() {
        let mut l = state(vec![
            BridgeVal::Num(0.0),
            BridgeVal::Num(0.0),
            BridgeVal::Num(0.0),
        ]);
        assert_eq!(stub_0x27399c(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::CFrame(identity())));
    }

    #[test]
    #[should_panic(expected = "number expected")]
    fn euler_checknumber_throws_on_non_number() {
        let mut l = state(vec![
            BridgeVal::Vec3(Vector3 { x: 0.0, y: 0.0, z: 0.0 }),
            BridgeVal::Num(0.0),
            BridgeVal::Num(0.0),
        ]);
        stub_0x27399c(&mut l);
    }

    #[test]
    fn axis_angle_zero_is_identity() {
        // angle 0: c = 1, s = 0, t = 0 -> exact identity for any axis.
        let mut l = state(vec![vec3(0.0, 1.0, 0.0), BridgeVal::Num(0.0)]);
        assert_eq!(stub_0x273ad8(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::CFrame(identity())));
    }

    #[test]
    fn add_sub_keep_rotation_and_shift_translation() {
        let a = cf(
            [0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            (1.0, 2.0, 3.0),
        );
        let mut l = state(vec![a.clone(), vec3(10.0, -20.0, 30.0)]);
        assert_eq!(stub_0x273c2c(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&cf(
                [0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
                (11.0, -18.0, 33.0)
            ))
        );
        let mut l = state(vec![a, vec3(10.0, -20.0, 30.0)]);
        assert_eq!(stub_0x273ce0(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&cf(
                [0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
                (-9.0, 22.0, -27.0)
            ))
        );
    }

    #[test]
    fn mul_composes_frames_and_point_transforms_with_translation() {
        // B = translated identity; A has a 90°-about-Z rotation (exact ints).
        let a = cf(
            [0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            (1.0, 2.0, 3.0),
        );
        let b = cf(
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            (4.0, 5.0, 6.0),
        );
        let mut l = state(vec![a, b]);
        assert_eq!(stub_0x273d94(&mut l), 1);
        // R = A.R; T = A.R*B.T + A.T = (-5+1, 4+2, 6+3).
        assert_eq!(
            l.stack.last(),
            Some(&cf(
                [0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
                (-4.0, 6.0, 9.0)
            ))
        );
        // CFrame * Vector3 includes the translation (w = 1 path, 0x273eb2).
        let mut l = state(vec![
            cf(
                [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
                (1.0, 2.0, 3.0),
            ),
            vec3(10.0, 20.0, 30.0),
        ]);
        assert_eq!(stub_0x273d94(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&BridgeVal::Vec3(Vector3 { x: 11.0, y: 22.0, z: 33.0 }))
        );
    }

    #[test]
    fn inverse_negates_translated_identity() {
        let mut l = state(vec![cf(
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            (1.0, 2.0, 3.0),
        )]);
        assert_eq!(stub_0x273f48(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&cf(
                [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
                (-1.0, -2.0, -3.0)
            ))
        );
        // 180°-about-Z (exact ints) is its own inverse rotation.
        let mut l = state(vec![cf(
            [-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0],
            (1.0, 0.0, 0.0),
        )]);
        assert_eq!(stub_0x273f48(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&cf(
                [-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0],
                (1.0, 0.0, 0.0)
            ))
        );
    }

    #[test]
    fn to_world_copies_self_and_composes_args() {
        let a = cf(
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            (1.0, 0.0, 0.0),
        );
        let b = cf(
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            (0.0, 2.0, 0.0),
        );
        let mut l = state(vec![a.clone()]);
        assert_eq!(stub_0x274024(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&a));
        let mut l = state(vec![a.clone(), b]);
        assert_eq!(stub_0x274024(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&cf(
                [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
                (1.0, 2.0, 0.0)
            ))
        );
        // Zero args: checkudata(1) raises before gettop is even read
        // (0x274044 precedes 0x27404c), so the `return -1` tail (0x2740e2)
        // is unreachable in practice; the model keeps the branch for shape
        // fidelity.
    }

    #[test]
    #[should_panic]
    fn to_world_zero_args_raises_like_checkudata() {
        stub_0x274024(&mut BridgeState::new());
    }

    #[test]
    fn to_object_uses_inverse() {
        let a = cf(
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            (1.0, 0.0, 0.0),
        );
        let mut l = state(vec![a]);
        assert_eq!(stub_0x2740e4(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&cf(
                [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
                (-1.0, -0.0, -0.0)
            ))
        );
    }

    #[test]
    fn point_to_world_adds_translation() {
        let a = cf(
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            (5.0, 6.0, 7.0),
        );
        // Single-arg path transforms zero -> the translation.
        let mut l = state(vec![a.clone()]);
        assert_eq!(stub_0x274394(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&BridgeVal::Vec3(Vector3 { x: 5.0, y: 6.0, z: 7.0 }))
        );
        let mut l = state(vec![a, vec3(1.0, 2.0, 3.0)]);
        assert_eq!(stub_0x274394(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&BridgeVal::Vec3(Vector3 { x: 6.0, y: 8.0, z: 10.0 }))
        );
    }

    #[test]
    fn point_to_object_subtracts_then_unrotates() {
        let a = cf(
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            (1.0, 2.0, 3.0),
        );
        let mut l = state(vec![a, vec3(4.0, 6.0, 8.0)]);
        assert_eq!(stub_0x274528(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&BridgeVal::Vec3(Vector3 { x: 3.0, y: 4.0, z: 5.0 }))
        );
    }

    #[test]
    fn vector_to_world_ignores_translation() {
        let a = cf(
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            (1.0, 2.0, 3.0),
        );
        let mut l = state(vec![a.clone(), vec3(4.0, 6.0, 8.0)]);
        assert_eq!(stub_0x2746bc(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&BridgeVal::Vec3(Vector3 { x: 4.0, y: 6.0, z: 8.0 }))
        );
        // Single-arg path rotates zero -> exact +0 lanes (no T added).
        let mut l = state(vec![a]);
        assert_eq!(stub_0x2746bc(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&BridgeVal::Vec3(Vector3 { x: 0.0, y: 0.0, z: 0.0 }))
        );
    }

    #[test]
    fn register_coordinate_frame_library() {
        let mut l = BridgeState::new();
        assert_eq!(stub_0x273bf0(&mut l), 0);
        assert_eq!(l.registered_libs, vec!["CoordinateFrame"]);
        assert_eq!(l.gettop(), 0);
    }

    #[test]
    fn look_at_math_matches_disasm_columns() {
        // Direct helper check: eye origin, target +Z, up +Y gives columns
        // (Y, X2, -Z) = [(-1,0,0), (0,1,0), (0,0,-1)] row-major.
        let r = cframe_look_at_rotation(
            &Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            &Vector3 { x: 0.0, y: 0.0, z: 1.0 },
            &Vector3 { x: 0.0, y: 1.0, z: 0.0 },
        );
        assert_eq!(
            r.m,
            [-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0]
        );
        // unitize degenerate branch: zero vector stays, returns 0.
        let mut z = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        assert_eq!(vec3_unitize(&mut z, 1e-6), 0.0);
        assert_eq!(z, Vector3 { x: 0.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn euler_and_quat_helpers_follow_grounded_formulas() {
        // Rx(90°)·Ry(0)·Rz(0): rows [1,0,0],[0,c,-s],[0,s,c] with f32 trig.
        let r = matrix3_from_euler_xyz(std::f32::consts::FRAC_PI_2, 0.0, 0.0);
        let (s, c) = (
            std::f32::consts::FRAC_PI_2.sin(),
            std::f32::consts::FRAC_PI_2.cos(),
        );
        assert_eq!(r.m, [1.0, 0.0, 0.0, 0.0, c, -s, 0.0, s, c]);
        // 90° about +Z via axis-angle: rows [c,-s,0],[s,c,0],[0,0,1].
        let q = matrix3_from_axis_angle(
            &Vector3 { x: 0.0, y: 0.0, z: 1.0 },
            std::f32::consts::FRAC_PI_2,
        );
        let c2 = (std::f32::consts::FRAC_PI_2 as f64).cos() as f32;
        let s2 = (std::f32::consts::FRAC_PI_2 as f64).sin() as f32;
        let t2 = 1.0 - c2;
        assert_eq!(
            q.m,
            [
                c2 + t2 * 0.0,
                t2 * 0.0 - s2 * 1.0,
                t2 * 0.0 + s2 * 0.0,
                t2 * 0.0 + s2 * 1.0,
                c2 + t2 * 0.0,
                t2 * 0.0 - s2 * 0.0,
                t2 * 0.0 - s2 * 0.0,
                t2 * 0.0 + s2 * 0.0,
                c2 + t2 * 1.0,
            ]
        );
    }

    #[test]
    fn cframe_mul_matches_grounded_compose() {
        let a = CoordinateFrame {
            rotation: Matrix3 { m: [2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0] },
            translation: Vector3 { x: 1.0, y: 1.0, z: 1.0 },
        };
        let b = CoordinateFrame {
            rotation: Matrix3 { m: [5.0, 0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0, 7.0] },
            translation: Vector3 { x: 1.0, y: 2.0, z: 3.0 },
        };
        // R = diag(10, 18, 28); T = R1*T2 + T1 = (2*1+1, 3*2+1, 4*3+1).
        let c = cframe_mul(&a, &b);
        assert_eq!(
            c,
            CoordinateFrame {
                rotation: Matrix3 {
                    m: [10.0, 0.0, 0.0, 0.0, 18.0, 0.0, 0.0, 0.0, 28.0]
                },
                translation: Vector3 { x: 3.0, y: 7.0, z: 13.0 },
            }
        );
    }
}

#[cfg(test)]
mod launcher_getvalue_batch_tests {
    use super::*;
    fn state(vals: Vec<BridgeVal>) -> BridgeState {
        let mut l = BridgeState::new();
        for v in vals {
            l.stack.push(v);
        }
        l
    }
    fn out() -> BridgeVal {
        BridgeVal::Nil
    }
    #[test]
    fn event_descriptor_is_always_scriptable() {
        assert!(stub_0x25f838());
    }
    #[test]
    fn starter_script_dtors_are_drop_markers() {
        stub_0x26a88c(&mut StarterScriptCore);
        stub_0x26a930(&mut StarterScriptCore);
        stub_0x26a9dc(&mut StarterScriptCore);
        stub_0x26aba4(&mut StarterScriptCore);
        stub_0x26ac48(&mut StarterScriptCore);
        stub_0x26a890(Box::new(StarterScriptCore));
        stub_0x26a938(Box::new(StarterScriptCore));
        stub_0x26a9e4(Box::new(StarterScriptCore));
        stub_0x26aba8(Box::new(StarterScriptCore));
        stub_0x26ac50(Box::new(StarterScriptCore));
    }
    #[test]
    fn inject_records_script_and_flags_dispatch() {
        let mut p = PlaceLauncher::default();
        stub_0x267ec(&mut p, b"game:Join()");
        assert_eq!(p.join_script, b"game:Join()");
        assert!(p.join_dispatched);
        assert!(!p.teleport_complete);
    }
    #[test]
    fn start_game_needs_self_and_dispatches() {
        let mut p = PlaceLauncher::default();
        assert!(stub_0x29280(Some(&mut p), b"join"));
        assert_eq!(p.join_script, b"join");
        assert!(p.join_dispatched);
        assert!(!stub_0x29280(None, b"join"));
    }
    #[test]
    fn teleport_dispatch_and_completion_block() {
        let mut p = PlaceLauncher::default();
        stub_0x29ccc(&mut p, b"teleport");
        assert_eq!(p.join_script, b"teleport");
        assert!(p.join_dispatched);
        stub_0x2a99c(&mut p);
        assert!(p.teleport_complete);
    }
    #[test]
    fn loading_frame_is_centered_1x1() {
        assert_eq!(stub_0x2a8c8(Some((0.0, 0.0, 320.0, 480.0))), (160.0, 240.0, 1.0, 1.0));
        assert_eq!(stub_0x2a8c8(None), (0.0, 0.0, 1.0, 1.0));
    }
    #[test]
    fn getvalue_copies_on_tag_match_and_fails_silently() {
        let cf = cframe_identity();
        let l = state(vec![BridgeVal::CFrame(cf)]);
        let mut o = out();
        assert!(stub_0x26c92c(&l, 1, &mut o));
        assert_eq!(o, BridgeVal::CFrame(cf));
        let mut o = out();
        assert!(!stub_0x26c9a8(&l, 1, &mut o));
        assert_eq!(o, BridgeVal::Nil);
    }
    #[test]
    fn getvalue_covers_every_value_bridge() {
        let r3 = Region3 {
            min: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            max: Vector3 { x: 1.0, y: 2.0, z: 3.0 },
        };
        let r3i = Region3int16 {
            min: Vector3int16 { x: 0, y: 0, z: 0 },
            max: Vector3int16 { x: 1, y: 2, z: 3 },
        };
        let v3i = Vector3int16 { x: 4, y: 5, z: 6 };
        let v2i = Vector2int16 { x: 7, y: 8 };
        let v3 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vector2 { x: 9.0, y: 10.0 };
        let ray = RbxRay { origin: v3, direction: v3 };
        let c3 = Color3 { r: 1.0, g: 0.5, b: 0.25 };
        let bc = BrickColor(21);
        let ud = UDim { scale: 0.5, offset: 3 };
        let ud2 = UDim2 { x: ud, y: ud };
        let cases: Vec<(BridgeVal, fn(&BridgeState, i32, &mut BridgeVal) -> bool)> = vec![
            (BridgeVal::Region3(r3), stub_0x26c9a8),
            (BridgeVal::Region3i16(r3i), stub_0x26ca24),
            (BridgeVal::Vec3i16(v3i), stub_0x26caa0),
            (BridgeVal::Vec2i16(v2i), stub_0x26cb1c),
            (BridgeVal::Vec3(v3), stub_0x26cb98),
            (BridgeVal::Vec2(v2), stub_0x26cc14),
            (BridgeVal::Ray(ray), stub_0x26cc90),
            (BridgeVal::Color3(c3), stub_0x26cd0c),
            (BridgeVal::Brick(bc), stub_0x26cd88),
            (BridgeVal::UDim(ud), stub_0x26ce04),
            (BridgeVal::UDim2(ud2), stub_0x26ce80),
            (BridgeVal::Faces(Faces(7)), stub_0x26cefc),
            (BridgeVal::Axes(Axes(3)), stub_0x26cf78),
            (
                BridgeVal::Cell(CellID { x: 1, y: 2, z: 3, w: 4 }),
                stub_0x26cff4,
            ),
            (BridgeVal::Input(InputObject(42)), stub_0x26d070),
        ];
        for (val, f) in cases {
            let l = state(vec![val.clone()]);
            let mut o = out();
            assert!(f(&l, 1, &mut o), "getValue failed for {val:?}");
            assert_eq!(o, val);
            let other = state(vec![BridgeVal::Num(1.0)]);
            let mut o = out();
            assert!(!f(&other, 1, &mut o));
            assert_eq!(o, BridgeVal::Nil);
        }
    }
    #[test]
    fn enum_item_getvalue_copies_the_item_word() {
        let l = state(vec![BridgeVal::EnumItem(EnumItemPtr(0x1a2b3c))]);
        let mut o = EnumItemPtr(0);
        assert!(stub_0x270008(&l, 1, &mut o));
        assert_eq!(o, EnumItemPtr(0x1a2b3c));
        let other = state(vec![BridgeVal::Num(2.0)]);
        let mut o = EnumItemPtr(0);
        assert!(!stub_0x270008(&other, 1, &mut o));
        assert_eq!(o, EnumItemPtr(0));
    }
}
