//! rendering generated — next 100 stubs
//! Filter: Ogre|Gfx|Render|G3D (1416 remaining, 100 this batch) — 0xe4f70c..0xe5eb7c
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(
    non_snake_case,
    dead_code,
    unused_variables,
    unused_imports,
    clippy::all
)]

use rbx_core::SharedPtr;
use crate::generated_502::UnifiedHighLevelGpuProgram;

/// was: `Ogre::Resource::LoadingState` — observed through the
/// `UnifiedHighLevelGpuProgram::getLoadingState` forwarder (IDA `0xe4f7cc`:
/// null delegate returns 0; `isLoading` compares against 1 at `0xe4f7ae`).
#[doc(alias = "Ogre::Resource::LoadingState")]
pub mod loading_state {
    /// `LOADSTATE_UNLOADED` (IDA `0xe4f7e2`: null-delegate default).
    pub const UNLOADED: u32 = 0;
    /// `LOADSTATE_LOADING` (IDA `0xe4f7ae`/`0xe4f7bc`: `isLoading` vtable query).
    pub const LOADING: u32 = 1;
    /// `LOADSTATE_LOADED`.
    pub const LOADED: u32 = 2;
    /// `LOADSTATE_UNLOADING`.
    pub const UNLOADING: u32 = 3;
    /// `LOADSTATE_PREPARED`.
    pub const PREPARED: u32 = 4;
    /// `LOADSTATE_PREPARING`.
    pub const PREPARING: u32 = 5;
}

/// was: `Ogre::UnifiedHighLevelGpuProgramFactory` (IDA `0xe50004`: stores the
/// vtable at `off_1202478`, `0xe50010`). No data members; construction is the impl.
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgramFactory")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnifiedHighLevelGpuProgramFactory;

impl UnifiedHighLevelGpuProgramFactory {
    /// IDA `0xe50004`: vtable store, return `this`.
    pub fn new() -> Self {
        Self
    }
}

impl UnifiedHighLevelGpuProgram {
    /// IDA `0xe4f70c`: bound delegate → `HighLevelGpuProgram::reload` (vtable +68);
    /// null → `chooseDelegate` (`0xe4f71a`), retry, else return 0 (`0xe4f726`).
    /// A reload re-establishes the loaded latch on the delegate.
    pub fn reload(&mut self) {
        // IDA 0xe4f712..0xe4f716: bound → delegate reload.
        // IDA 0xe4f71a..0xe4f72c: choose, re-read +232, reload when bound now.
        self.choose_delegate();
        if let Some(delegate) = self.binding_delegate.as_mut() {
            delegate.loaded = true;
            delegate.loading_state = loading_state::LOADED;
        }
    }

    /// IDA `0xe4f730`: forward `isReloadable` (vtable +72); null delegate → 1 (`0xe4f746`).
    pub fn is_reloadable(&mut self) -> bool {
        self.choose_delegate();
        self.binding_delegate
            .as_ref()
            .map(|d| d.reloadable)
            .unwrap_or(true)
    }

    /// IDA `0xe4f758`: bound delegate → `unload` (vtable +80);
    /// null → `chooseDelegate` (`0xe4f766`), retry, else no-op (`0xe4f772`).
    pub fn unload(&mut self) {
        self.choose_delegate();
        if let Some(delegate) = self.binding_delegate.as_mut() {
            delegate.loaded = false;
            delegate.loading_state = loading_state::UNLOADED;
        }
    }

    /// IDA `0xe4f77c`: forward `isLoaded` (vtable +104); null delegate → 0 (`0xe4f792`).
    pub fn resource_is_loaded(&mut self) -> bool {
        self.choose_delegate();
        self.binding_delegate
            .as_ref()
            .map(|d| d.loaded)
            .unwrap_or(false)
    }

    /// IDA `0xe4f7a4`: forward `isLoading` (vtable +108); null delegate → 0 (`0xe4f7ba`).
    pub fn resource_is_loading(&mut self) -> bool {
        self.choose_delegate();
        self.binding_delegate
            .as_ref()
            .map(|d| d.loading_state == loading_state::LOADING)
            .unwrap_or(false)
    }

    /// IDA `0xe4f7cc`: forward `getLoadingState` (vtable +112); null delegate → 0 (`0xe4f7e2`).
    pub fn loading_state(&mut self) -> u32 {
        self.choose_delegate();
        self.binding_delegate
            .as_ref()
            .map(|d| d.loading_state)
            .unwrap_or(loading_state::UNLOADED)
    }

    /// IDA `0xe4f7f4`: forward `getSize` (vtable +84); null delegate → 0 (`0xe4f80a`).
    pub fn resource_size(&mut self) -> usize {
        self.choose_delegate();
        self.binding_delegate
            .as_ref()
            .map(|d| d.resource_size)
            .unwrap_or(0)
    }

    /// IDA `0xe4f840`: forward `isBackgroundLoaded` (vtable +116); null → 0 (`0xe4f856`).
    pub fn is_background_loaded(&mut self) -> bool {
        self.choose_delegate();
        self.binding_delegate
            .as_ref()
            .map(|d| d.background_loaded)
            .unwrap_or(false)
    }

    /// IDA `0xe4f868`: forward `setBackgroundLoaded` (vtable +120); null → no-op.
    pub fn set_background_loaded(&mut self, loaded: bool) {
        self.choose_delegate();
        if let Some(delegate) = self.binding_delegate.as_mut() {
            delegate.background_loaded = loaded;
        }
    }

    /// IDA `0xe4f890`: forward `escalateLoading` (vtable +124); null → no-op.
    /// `Resource::escalateLoading` finishes a background load synchronously.
    pub fn escalate_loading(&mut self) {
        self.choose_delegate();
        if let Some(delegate) = self.binding_delegate.as_mut() {
            if delegate.loading_state == loading_state::LOADING {
                delegate.loaded = true;
                delegate.loading_state = loading_state::LOADED;
            }
        }
    }

    /// IDA `0xe4f8b4`: forward `addListener` (vtable +128); null → `chooseDelegate`, no-op.
    /// Listener pointers are opaque; the delegate tracks how many are attached.
    pub fn add_listener(&mut self) {
        self.choose_delegate();
        if let Some(delegate) = self.binding_delegate.as_mut() {
            delegate.listeners = delegate.listeners.saturating_add(1);
        }
    }

    /// IDA `0xe4f8e0`: forward `removeListener` (vtable +132); null → `chooseDelegate`, no-op.
    pub fn remove_listener(&mut self) {
        self.choose_delegate();
        if let Some(delegate) = self.binding_delegate.as_mut() {
            delegate.listeners = delegate.listeners.saturating_sub(1);
        }
    }

    /// IDA `0xe4f90c`: throws `Ogre::UnimplementedException` (code 9,
    /// "This method should never get called!", `OgreUnifiedHighLevelGpuProgram.cpp:351`).
    pub fn create_low_level_impl(&mut self) -> ! {
        panic!(
            "Ogre::UnimplementedException at 0xe4f90c: This method should never get called!"
        );
    }

    /// IDA `0xe4fac0`: throws `Ogre::UnimplementedException` (code 9,
    /// "This method should never get called!", `OgreUnifiedHighLevelGpuProgram.cpp:358`).
    pub fn unload_high_level_impl(&mut self) -> ! {
        panic!(
            "Ogre::UnimplementedException at 0xe4fac0: This method should never get called!"
        );
    }

    /// IDA `0xe4fc74`: throws `Ogre::UnimplementedException` (code 9,
    /// "This method should never get called!", `OgreUnifiedHighLevelGpuProgram.cpp:365`).
    pub fn build_constant_definitions(&mut self) -> ! {
        panic!(
            "Ogre::UnimplementedException at 0xe4fc74: This method should never get called!"
        );
    }

    /// IDA `0xe4fe28`: throws `Ogre::UnimplementedException` (code 9,
    /// "This method should never get called!", `OgreUnifiedHighLevelGpuProgram.cpp:372`).
    pub fn load_from_source(&mut self) -> ! {
        panic!(
            "Ogre::UnimplementedException at 0xe4fe28: This method should never get called!"
        );
    }

    /// IDA `0xe4ffdc`: returns `Ogre::StringUtil::BLANK` (`0xe4fff0`).
    pub fn cmd_delegate_get() -> String {
        String::new()
    }

    /// IDA `0xe4fff4`: tail-calls `addDelegateProgram(target, name)` (`0xe50000`).
    pub fn cmd_delegate_set(&mut self, name: &str) {
        self.add_delegate_program(name);
    }
}

// 0xe4f70c — __ZN4Ogre26UnifiedHighLevelGpuProgram6reloadEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::reload(void)")]
// was: Ogre::UnifiedHighLevelGpuProgram::reload(void)
// IDA 0xe4f70c: bound delegate reloads (vtable +68); null → chooseDelegate, retry, else 0.
pub fn stub_e4f70c(program: &mut UnifiedHighLevelGpuProgram) {
    program.reload()
}

// 0xe4f730 — __ZNK4Ogre26UnifiedHighLevelGpuProgram12isReloadableEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::isReloadable(void)const")]
// was: Ogre::UnifiedHighLevelGpuProgram::isReloadable(void)const
// IDA 0xe4f730: forward isReloadable (vtable +72); null delegate → true.
pub fn stub_e4f730(program: &mut UnifiedHighLevelGpuProgram) -> bool {
    program.is_reloadable()
}
// 0xe4f758 — __ZN4Ogre26UnifiedHighLevelGpuProgram6unloadEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::unload(void)")]
// was: Ogre::UnifiedHighLevelGpuProgram::unload(void)
// IDA 0xe4f758: bound delegate unloads (vtable +80); null → chooseDelegate, retry, else no-op.
pub fn stub_e4f758(program: &mut UnifiedHighLevelGpuProgram) {
    program.unload()
}
// 0xe4f77c — __ZNK4Ogre26UnifiedHighLevelGpuProgram8isLoadedEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::isLoaded(void)const")]
// was: Ogre::UnifiedHighLevelGpuProgram::isLoaded(void)const
// IDA 0xe4f77c: forward isLoaded (vtable +104); null delegate → false.
pub fn stub_e4f77c(program: &mut UnifiedHighLevelGpuProgram) -> bool {
    program.resource_is_loaded()
}

// 0xe4f7a4 — __ZNK4Ogre26UnifiedHighLevelGpuProgram9isLoadingEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::isLoading(void)const")]
// was: Ogre::UnifiedHighLevelGpuProgram::isLoading(void)const
// IDA 0xe4f7a4: forward isLoading (vtable +108); null delegate → false.
pub fn stub_e4f7a4(program: &mut UnifiedHighLevelGpuProgram) -> bool {
    program.resource_is_loading()
}

// 0xe4f7cc — __ZNK4Ogre26UnifiedHighLevelGpuProgram15getLoadingStateEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::getLoadingState(void)const")]
// was: Ogre::UnifiedHighLevelGpuProgram::getLoadingState(void)const
// IDA 0xe4f7cc: forward getLoadingState (vtable +112); null delegate → UNLOADED (0).
pub fn stub_e4f7cc(program: &mut UnifiedHighLevelGpuProgram) -> u32 {
    program.loading_state()
}

// 0xe4f7f4 — __ZNK4Ogre26UnifiedHighLevelGpuProgram7getSizeEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::getSize(void)const")]
// was: Ogre::UnifiedHighLevelGpuProgram::getSize(void)const
// IDA 0xe4f7f4: forward getSize (vtable +84); null delegate → 0.
pub fn stub_e4f7f4(program: &mut UnifiedHighLevelGpuProgram) -> usize {
    program.resource_size()
}

// 0xe4f840 — __ZNK4Ogre26UnifiedHighLevelGpuProgram18isBackgroundLoadedEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::isBackgroundLoaded(void)const")]
// was: Ogre::UnifiedHighLevelGpuProgram::isBackgroundLoaded(void)const
// IDA 0xe4f840: forward isBackgroundLoaded (vtable +116); null delegate → false.
pub fn stub_e4f840(program: &mut UnifiedHighLevelGpuProgram) -> bool {
    program.is_background_loaded()
}

// 0xe4f868 — __ZN4Ogre26UnifiedHighLevelGpuProgram19setBackgroundLoadedEb
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::setBackgroundLoaded(bool)")]
// was: Ogre::UnifiedHighLevelGpuProgram::setBackgroundLoaded(bool)
// IDA 0xe4f868: forward setBackgroundLoaded (vtable +120); null delegate → no-op.
pub fn stub_e4f868(program: &mut UnifiedHighLevelGpuProgram, loaded: bool) {
    program.set_background_loaded(loaded)
}

// 0xe4f890 — __ZN4Ogre26UnifiedHighLevelGpuProgram15escalateLoadingEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::escalateLoading(void)")]
// IDA 0xe4f890: forward escalateLoading (vtable +124); null delegate → no-op.
pub fn stub_e4f890(program: &mut UnifiedHighLevelGpuProgram) {
    program.escalate_loading()
}

// 0xe4f8b4 — __ZN4Ogre26UnifiedHighLevelGpuProgram11addListenerEPNS_8Resource8ListenerE
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::addListener(Ogre::Resource::Listener *)")]
// IDA 0xe4f8b4: forward addListener (vtable +128); null → chooseDelegate, no-op.
pub fn stub_e4f8b4(program: &mut UnifiedHighLevelGpuProgram) {
    program.add_listener()
}

// 0xe4f8e0 — __ZN4Ogre26UnifiedHighLevelGpuProgram14removeListenerEPNS_8Resource8ListenerE
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::removeListener(Ogre::Resource::Listener *)")]
// IDA 0xe4f8e0: forward removeListener (vtable +132); null → chooseDelegate, no-op.
pub fn stub_e4f8e0(program: &mut UnifiedHighLevelGpuProgram) {
    program.remove_listener()
}

// 0xe4f90c — __ZN4Ogre26UnifiedHighLevelGpuProgram18createLowLevelImplEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::createLowLevelImpl(void)")]
// IDA 0xe4f90c: UnimplementedException("This method should never get called!", OgreUnifiedHighLevelGpuProgram.cpp:351).
pub fn stub_e4f90c(program: &mut UnifiedHighLevelGpuProgram) {
    program.create_low_level_impl()
}

// 0xe4fac0 — __ZN4Ogre26UnifiedHighLevelGpuProgram19unloadHighLevelImplEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::unloadHighLevelImpl(void)")]
// IDA 0xe4fac0: UnimplementedException("This method should never get called!", OgreUnifiedHighLevelGpuProgram.cpp:358).
pub fn stub_e4fac0(program: &mut UnifiedHighLevelGpuProgram) {
    program.unload_high_level_impl()
}

// 0xe4fc74 — __ZNK4Ogre26UnifiedHighLevelGpuProgram24buildConstantDefinitionsEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::buildConstantDefinitions(void)const")]
// IDA 0xe4fc74: UnimplementedException("This method should never get called!", OgreUnifiedHighLevelGpuProgram.cpp:365).
pub fn stub_e4fc74(program: &mut UnifiedHighLevelGpuProgram) {
    program.build_constant_definitions()
}

// 0xe4fe28 — __ZN4Ogre26UnifiedHighLevelGpuProgram14loadFromSourceEv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::loadFromSource(void)")]
// IDA 0xe4fe28: UnimplementedException("This method should never get called!", OgreUnifiedHighLevelGpuProgram.cpp:372).
pub fn stub_e4fe28(program: &mut UnifiedHighLevelGpuProgram) {
    program.load_from_source()
}

// 0xe4ffdc — __ZNK4Ogre26UnifiedHighLevelGpuProgram11CmdDelegate5doGetEPKv
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::CmdDelegate::doGet(void const*)const")]
// IDA 0xe4ffdc: copies Ogre::StringUtil::BLANK (0xe4fff0).
pub fn stub_e4ffdc() -> String {
    UnifiedHighLevelGpuProgram::cmd_delegate_get()
}

// 0xe4fff4 — __ZN4Ogre26UnifiedHighLevelGpuProgram11CmdDelegate5doSetEPvRKSs
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::CmdDelegate::doSet(void *,std::string const&)")]
// IDA 0xe4fff4: tail-calls addDelegateProgram(target, name) (0xe50000).
pub fn stub_e4fff4(program: &mut UnifiedHighLevelGpuProgram, name: &str) {
    program.cmd_delegate_set(name)
}

// 0xe50004 — __ZN4Ogre33UnifiedHighLevelGpuProgramFactoryC1Ev
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgramFactory::UnifiedHighLevelGpuProgramFactory(void)")]
// IDA 0xe50004: vtable store at off_1202478 (0xe50010), return this.
pub fn stub_e50004() -> UnifiedHighLevelGpuProgramFactory {
    UnifiedHighLevelGpuProgramFactory::new()
}

// 0xe50014 — __ZN4Ogre33UnifiedHighLevelGpuProgramFactoryD0Ev
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgramFactory::~UnifiedHighLevelGpuProgramFactory()")]
// was: Ogre::UnifiedHighLevelGpuProgramFactory::~UnifiedHighLevelGpuProgramFactory()
// IDA 0xe50014: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e50014() {
}

// 0xe59934 — __ZNSt10_List_baseIPN4Ogre9WorkQueue15ResponseHandlerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xe59934: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e59934() {
}

// 0xe59938 — __ZNSt10_List_baseIPN4Ogre9WorkQueue15ResponseHandlerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xe59938: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e59938() {
}

// 0xe59ad4 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE16_M_insert_uniqueERKSE_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xe59ad4: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59ad4() {
}

// 0xe59b40 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE9_M_insertEPSt18_Rb_tree_node_baseSM_RKSE_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xe59b40: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59b40() {
}

// 0xe59c78 — __ZNSt11_Deque_baseIPN4Ogre9WorkQueue8ResponseENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE11_Deque_implD1Ev
#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()")]
// was: std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()
// IDA 0xe59c78: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e59c78() {
}

// 0xe59c7c — __ZNSt11_Deque_baseIPN4Ogre9WorkQueue7RequestENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE11_Deque_implD1Ev
#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()")]
// was: std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()
// IDA 0xe59c7c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e59c7c() {
}

// 0xe59c80 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIPN4Ogre9WorkQueue15ResponseHandlerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessItENS7_ISD_SA_EEE13_Rb_tree_implISH_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xe59c80: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e59c80() {
}

// 0xe59c84 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIPN4Ogre9WorkQueue15ResponseHandlerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessItENS7_ISD_SA_EEE13_Rb_tree_implISH_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xe59c84: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59c84() {
}

// 0xe59c90 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE13_Rb_tree_implISI_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xe59c90: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e59c90() {
}

// 0xe59c94 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE13_Rb_tree_implISI_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xe59c94: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59c94() {
}

// 0xe59ca0 — __ZNSt11_Deque_baseIPN4Ogre9WorkQueue8ResponseENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_initialize_mapEm
#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_initialize_map(unsigned long)")]
// was: std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_initialize_map(unsigned long)
// IDA 0xe59ca0: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59ca0() {
}

// 0xe59e70 — __ZNSt11_Deque_baseIPN4Ogre9WorkQueue8ResponseENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE11_Deque_implD0Ev
#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()")]
// was: std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()
// IDA 0xe59e70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e59e70() {
}

// 0xe59e7c — __ZNSt11_Deque_baseIPN4Ogre9WorkQueue7RequestENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_initialize_mapEm
#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_initialize_map(unsigned long)")]
// was: std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_initialize_map(unsigned long)
// IDA 0xe59e7c: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59e7c() {
}

// 0xe5a04c — __ZNSt11_Deque_baseIPN4Ogre9WorkQueue7RequestENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE11_Deque_implD0Ev
#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()")]
// was: std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()
// IDA 0xe5a04c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5a04c() {
}

// 0xe5a328 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIPN4Ogre9WorkQueue15ResponseHandlerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessItENS7_ISD_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISD_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)
// IDA 0xe5a328: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5a328() {
}

// 0xe5a4a4 — __ZN4Ogre10ZipArchiveC1ERKSsS2_P15_zzip_plugin_io
#[doc(alias = "Ogre::ZipArchive::ZipArchive(std::string const&,std::string const&,_zzip_plugin_io *)")]
// was: Ogre::ZipArchive::ZipArchive(std::string const&,std::string const&,_zzip_plugin_io *)
// IDA 0xe5a4a4: 113 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5a4a4() {
}

// 0xe5a5e0 — __ZN4Ogre10ZipArchiveD0Ev
#[doc(alias = "Ogre::ZipArchive::~ZipArchive()")]
// was: Ogre::ZipArchive::~ZipArchive()
// IDA 0xe5a5e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5a5e0() {
}

// 0xe5a670 — __ZN4Ogre10ZipArchiveD1Ev
#[doc(alias = "Ogre::ZipArchive::~ZipArchive()")]
// was: Ogre::ZipArchive::~ZipArchive()
// IDA 0xe5a670: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5a670() {
}

// 0xe5a67c — __ZN4Ogre10ZipArchiveD2Ev
#[doc(alias = "Ogre::ZipArchive::~ZipArchive()")]
// was: Ogre::ZipArchive::~ZipArchive()
// IDA 0xe5a67c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5a67c() {
}

// 0xe5a8c4 — __ZN4Ogre10ZipArchive4loadEv
#[doc(alias = "Ogre::ZipArchive::load(void)")]
// was: Ogre::ZipArchive::load(void)
// IDA 0xe5a8c4: 408 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5a8c4() {
}

// 0xe5ad48 — __ZNK4Ogre10ZipArchive14checkZzipErrorEiRKSs
#[doc(alias = "Ogre::ZipArchive::checkZzipError(int,std::string const&)const")]
// was: Ogre::ZipArchive::checkZzipError(int,std::string const&)const
// IDA 0xe5ad48: 188 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5ad48() {
}

// 0xe5b238 — __ZN4Ogre10ZipArchive6unloadEv
#[doc(alias = "Ogre::ZipArchive::unload(void)")]
// was: Ogre::ZipArchive::unload(void)
// IDA 0xe5b238: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5b238() {
}

// 0xe5b274 — __ZNK4Ogre10ZipArchive4openERKSsb
#[doc(alias = "Ogre::ZipArchive::open(std::string const&,bool)const")]
// was: Ogre::ZipArchive::open(std::string const&,bool)const
// IDA 0xe5b274: 1069 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5b274() {
}

// 0xe5be80 — __ZNK4Ogre10ZipArchive6createERKSs
#[doc(alias = "Ogre::ZipArchive::create(std::string const&)const")]
// was: Ogre::ZipArchive::create(std::string const&)const
// IDA 0xe5be80: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5be80() {
}

// 0xe5c030 — __ZNK4Ogre10ZipArchive6removeERKSs
#[doc(alias = "Ogre::ZipArchive::remove(std::string const&)const")]
// was: Ogre::ZipArchive::remove(std::string const&)const
// IDA 0xe5c030: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e5c030() {
}

// 0xe5c034 — __ZN4Ogre10ZipArchive4listEbb
#[doc(alias = "Ogre::ZipArchive::list(bool,bool)")]
// was: Ogre::ZipArchive::list(bool,bool)
// IDA 0xe5c034: 186 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c034() {
}

// 0xe5c210 — __ZN4Ogre10ZipArchive12listFileInfoEbb
#[doc(alias = "Ogre::ZipArchive::listFileInfo(bool,bool)")]
// was: Ogre::ZipArchive::listFileInfo(bool,bool)
// IDA 0xe5c210: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c210() {
}

// 0xe5c2bc — __ZN4Ogre10ZipArchive4findERKSsbb
#[doc(alias = "Ogre::ZipArchive::find(std::string const&,bool,bool)")]
// was: Ogre::ZipArchive::find(std::string const&,bool,bool)
// IDA 0xe5c2bc: 228 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c2bc() {
}

// 0xe5c4fc — __ZNK4Ogre10ZipArchive12findFileInfoERKSsbb
#[doc(alias = "Ogre::ZipArchive::findFileInfo(std::string const&,bool,bool)const")]
// was: Ogre::ZipArchive::findFileInfo(std::string const&,bool,bool)const
// IDA 0xe5c4fc: 207 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c4fc() {
}

// 0xe5c70c — __ZN4Ogre10ZipArchive6existsERKSs
#[doc(alias = "Ogre::ZipArchive::exists(std::string const&)")]
// was: Ogre::ZipArchive::exists(std::string const&)
// IDA 0xe5c70c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c70c() {
}

// 0xe5c72c — __ZN4Ogre10ZipArchive15getModifiedTimeERKSs
#[doc(alias = "Ogre::ZipArchive::getModifiedTime(std::string const&)")]
// was: Ogre::ZipArchive::getModifiedTime(std::string const&)
// IDA 0xe5c72c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c72c() {
}

// 0xe5c748 — __ZN4Ogre13ZipDataStreamD0Ev
#[doc(alias = "Ogre::ZipDataStream::~ZipDataStream()")]
// was: Ogre::ZipDataStream::~ZipDataStream()
// IDA 0xe5c748: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5c748() {
}

// 0xe5c7d8 — __ZN4Ogre13ZipDataStreamD1Ev
#[doc(alias = "Ogre::ZipDataStream::~ZipDataStream()")]
// was: Ogre::ZipDataStream::~ZipDataStream()
// IDA 0xe5c7d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5c7d8() {
}

// 0xe5c7e4 — __ZN4Ogre13ZipDataStreamD2Ev
#[doc(alias = "Ogre::ZipDataStream::~ZipDataStream()")]
// was: Ogre::ZipDataStream::~ZipDataStream()
// IDA 0xe5c7e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5c7e4() {
}

// 0xe5c944 — __ZN4Ogre13ZipDataStream4readEPvm
#[doc(alias = "Ogre::ZipDataStream::read(void *,unsigned long)")]
// was: Ogre::ZipDataStream::read(void *,unsigned long)
// IDA 0xe5c944: 350 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c944() {
}

// 0xe5cd48 — __ZN4Ogre13ZipDataStream4skipEl
#[doc(alias = "Ogre::ZipDataStream::skip(long)")]
// was: Ogre::ZipDataStream::skip(long)
// IDA 0xe5cd48: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cd48() {
}

// 0xe5cd98 — __ZN4Ogre13ZipDataStream4seekEm
#[doc(alias = "Ogre::ZipDataStream::seek(unsigned long)")]
// was: Ogre::ZipDataStream::seek(unsigned long)
// IDA 0xe5cd98: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cd98() {
}

// 0xe5cdb4 — __ZNK4Ogre13ZipDataStream4tellEv
#[doc(alias = "Ogre::ZipDataStream::tell(void)const")]
// was: Ogre::ZipDataStream::tell(void)const
// IDA 0xe5cdb4: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cdb4() {
}

// 0xe5cdd4 — __ZNK4Ogre13ZipDataStream3eofEv
#[doc(alias = "Ogre::ZipDataStream::eof(void)const")]
// was: Ogre::ZipDataStream::eof(void)const
// IDA 0xe5cdd4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cdd4() {
}

// 0xe5cdf0 — __ZN4Ogre13ZipDataStream5closeEv
#[doc(alias = "Ogre::ZipDataStream::close(void)")]
// was: Ogre::ZipDataStream::close(void)
// IDA 0xe5cdf0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cdf0() {
}

// 0xe5ce0c — __ZNK4Ogre17ZipArchiveFactory7getTypeEv
#[doc(alias = "Ogre::ZipArchiveFactory::getType(void)const")]
// was: Ogre::ZipArchiveFactory::getType(void)const
// IDA 0xe5ce0c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5ce0c() {
}

// 0xe5cf00 — __ZN4Ogre30EmbeddedZipArchiveFactory_openEPKciz
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory_open(char const*,int,...)")]
// was: Ogre::EmbeddedZipArchiveFactory_open(char const*,int,...)
// IDA 0xe5cf00: 63 insns (SUB..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cf00() {
}

// 0xe5cfac — __ZN4Ogre31EmbeddedZipArchiveFactory_closeEi
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory_close(int)")]
// was: Ogre::EmbeddedZipArchiveFactory_close(int)
// IDA 0xe5cfac: 24 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cfac() {
}

// 0xe5cff4 — __ZN4Ogre30EmbeddedZipArchiveFactory_readEiPvm
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory_read(int,void *,unsigned long)")]
// was: Ogre::EmbeddedZipArchiveFactory_read(int,void *,unsigned long)
// IDA 0xe5cff4: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cff4() {
}

// 0xe5d068 — __ZN4Ogre31EmbeddedZipArchiveFactory_seeksEixi
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory_seeks(int,long long,int)")]
// was: Ogre::EmbeddedZipArchiveFactory_seeks(int,long long,int)
// IDA 0xe5d068: 36 insns (CMP.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d068() {
}

// 0xe5d0c4 — __ZN4Ogre34EmbeddedZipArchiveFactory_filesizeEi
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory_filesize(int)")]
// was: Ogre::EmbeddedZipArchiveFactory_filesize(int)
// IDA 0xe5d0c4: 15 insns (CMP.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d0c4() {
}

// 0xe5d0f4 — __ZN4Ogre31EmbeddedZipArchiveFactory_writeEiPKvm
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory_write(int,void const*,unsigned long)")]
// was: Ogre::EmbeddedZipArchiveFactory_write(int,void const*,unsigned long)
// IDA 0xe5d0f4: 2 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d0f4() {
}

// 0xe5d0fc — __ZN4Ogre25EmbeddedZipArchiveFactoryC1Ev
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory::EmbeddedZipArchiveFactory(void)")]
// was: Ogre::EmbeddedZipArchiveFactory::EmbeddedZipArchiveFactory(void)
// IDA 0xe5d0fc: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d0fc() {
}

// 0xe5d184 — __ZN4Ogre25EmbeddedZipArchiveFactoryD0Ev
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory::~EmbeddedZipArchiveFactory()")]
// was: Ogre::EmbeddedZipArchiveFactory::~EmbeddedZipArchiveFactory()
// IDA 0xe5d184: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5d184() {
}

// 0xe5d210 — __ZN4Ogre25EmbeddedZipArchiveFactoryD1Ev
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory::~EmbeddedZipArchiveFactory()")]
// was: Ogre::EmbeddedZipArchiveFactory::~EmbeddedZipArchiveFactory()
// IDA 0xe5d210: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e5d210() {
}

// 0xe5d214 — __ZNK4Ogre25EmbeddedZipArchiveFactory7getTypeEv
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory::getType(void)const")]
// was: Ogre::EmbeddedZipArchiveFactory::getType(void)const
// IDA 0xe5d214: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d214() {
}

// 0xe5d30c — __ZN4Ogre12STLAllocatorINS_8FileInfoENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xe5d30c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e5d30c() {
}

// 0xe5d310 — __ZNSt6vectorIN4Ogre8FileInfoENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
#[doc(alias = "std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::FileInfo const&)")]
// was: std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::FileInfo const&)
// IDA 0xe5d310: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_e5d310() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0xe5d4a4 — __ZN4Ogre9SharedPtrISt6vectorINS_8FileInfoENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xe5d4a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5d4a4() {
}

// 0xe5d554 — __ZN4Ogre9SharedPtrISt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev
#[doc(alias = "Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xe5d554: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5d554() {
}

// 0xe5d604 — __ZN4Ogre22InternalErrorExceptionD1Ev
#[doc(alias = "Ogre::InternalErrorException::~InternalErrorException()")]
// was: Ogre::InternalErrorException::~InternalErrorException()
// IDA 0xe5d604: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5d604() {
}

// 0xe5d610 — __ZNK4Ogre10ZipArchive15isCaseSensitiveEv
#[doc(alias = "Ogre::ZipArchive::isCaseSensitive(void)const")]
// was: Ogre::ZipArchive::isCaseSensitive(void)const
// IDA 0xe5d610: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d610() {
}

// 0xe5d614 — __ZNK4Ogre7Archive10isReadOnlyEv
#[doc(alias = "Ogre::Archive::isReadOnly(void)const")]
// was: Ogre::Archive::isReadOnly(void)const
// IDA 0xe5d614: 2 insns (LDRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d614() {
}

// 0xe5d618 — __ZNK4Ogre10DataStream10isReadableEv
#[doc(alias = "Ogre::DataStream::isReadable(void)const")]
// was: Ogre::DataStream::isReadable(void)const
// IDA 0xe5d618: 3 insns (LDRH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d618() {
}

// 0xe5d620 — __ZNK4Ogre10DataStream11isWriteableEv
#[doc(alias = "Ogre::DataStream::isWriteable(void)const")]
// was: Ogre::DataStream::isWriteable(void)const
// IDA 0xe5d620: 4 insns (LDRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d620() {
}

// 0xe5d62c — __ZN4Ogre10DataStream5writeEPKvm
#[doc(alias = "Ogre::DataStream::write(void const*,unsigned long)")]
// was: Ogre::DataStream::write(void const*,unsigned long)
// IDA 0xe5d62c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d62c() {
}

// 0xe5d84c — __ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xe5d84c: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d84c() {
}

// 0xe5d8f0 — __ZN4Ogre9SharedPtrISt6vectorINS_8FileInfoENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xe5d8f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5d8f0() {
}

// 0xe5d9a4 — __ZN4Ogre9SharedPtrISt6vectorINS_8FileInfoENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
// was: Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)
// IDA 0xe5d9a4: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d9a4() {
}

// 0xe5dac0 — __ZN4Ogre9SharedPtrISt6vectorINS_8FileInfoENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERS9_
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)")]
// was: Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)
// IDA 0xe5dac0: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5dac0() {
}

// 0xe5dadc — __ZN4Ogre9SharedPtrISt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev
#[doc(alias = "Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xe5dadc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5dadc() {
}

// 0xe5db90 — __ZN4Ogre9SharedPtrISt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
// was: Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)
// IDA 0xe5db90: 151 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5db90() {
}

// 0xe5dd2c — __ZN4Ogre9SharedPtrISt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERS8_
#[doc(alias = "Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)")]
// was: Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)
// IDA 0xe5dd2c: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5dd2c() {
}

// 0xe5dd48 — __ZN4Ogre12STLAllocatorINS_8FileInfoENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xe5dd48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5dd48() {
}

// 0xe5dd54 — __ZN4Ogre12STLAllocatorINS_8FileInfoENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS1_
#[doc(alias = "Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Ogre::FileInfo*)")]
// was: Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Ogre::FileInfo*)
// IDA 0xe5dd54: 66 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5dd54() {
}

// 0xe5de10 — __ZNSt6vectorIN4Ogre8FileInfoENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::FileInfo*,std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::FileInfo const&)")]
// was: std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::FileInfo*,std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::FileInfo const&)
// IDA 0xe5de10: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e5de10() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe5e4b8 — __ZSt22__uninitialized_copy_aIPN4Ogre8FileInfoES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
#[doc(alias = "Ogre::FileInfo * std::__uninitialized_copy_a<Ogre::FileInfo *,Ogre::FileInfo *,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::FileInfo *,Ogre::FileInfo *,Ogre::FileInfo *,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: Ogre::FileInfo * std::__uninitialized_copy_a<Ogre::FileInfo *,Ogre::FileInfo *,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::FileInfo *,Ogre::FileInfo *,Ogre::FileInfo *,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
// IDA 0xe5e4b8: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5e4b8() {
}

// 0xe5e6d8 — __ZNSt12_Vector_baseIN4Ogre8FileInfoENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe5e6d8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e5e6d8() {
}

// 0xe5e6dc — __ZNSt12_Vector_baseIN4Ogre8FileInfoENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe5e6dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5e6dc() {
}

// 0xe5e6e8 — __ZN4Ogre10DataStreamD1Ev
#[doc(alias = "Ogre::DataStream::~DataStream()")]
// was: Ogre::DataStream::~DataStream()
// IDA 0xe5e6e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5e6e8() {
}

// 0xe5e744 — __ZN4Ogre10DataStreamD0Ev
#[doc(alias = "Ogre::DataStream::~DataStream()")]
// was: Ogre::DataStream::~DataStream()
// IDA 0xe5e744: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5e744() {
}

// 0xe5e820 — __ZN4Ogre7ArchiveD1Ev
#[doc(alias = "Ogre::Archive::~Archive()")]
// was: Ogre::Archive::~Archive()
// IDA 0xe5e820: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5e820() {
}

// 0xe5e8b8 — __ZN4Ogre7ArchiveD0Ev
#[doc(alias = "Ogre::Archive::~Archive()")]
// was: Ogre::Archive::~Archive()
// IDA 0xe5e8b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5e8b8() {
}

// 0xe5e9cc — __ZNK4Ogre7Archive6createERKSs
#[doc(alias = "Ogre::Archive::create(std::string const&)const")]
// was: Ogre::Archive::create(std::string const&)const
// IDA 0xe5e9cc: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5e9cc() {
}

// 0xe5eb7c — __ZNK4Ogre7Archive6removeERKSs
#[doc(alias = "Ogre::Archive::remove(std::string const&)const")]
// was: Ogre::Archive::remove(std::string const&)const
// IDA 0xe5eb7c: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5eb7c() {
}

#[cfg(test)]
mod resource_lifecycle_tests {
    use super::*;
    use crate::generated_502::UnifiedHighLevelGpuProgram;

    fn program_with_delegate() -> UnifiedHighLevelGpuProgram {
        let mut program = UnifiedHighLevelGpuProgram::new();
        program.add_delegate_program("cg");
        program.choose_delegate();
        program
    }

    #[test]
    fn null_delegate_answers_match_ida_defaults() {
        let mut program = UnifiedHighLevelGpuProgram::new();
        // IDA 0xe4f746/0xe4f7e2/0xe4f80a/0xe4f792/0xe4f7ba/0xe4f856 null arms.
        assert!(stub_e4f730(&mut program));
        assert!(!stub_e4f77c(&mut program));
        assert!(!stub_e4f7a4(&mut program));
        assert_eq!(stub_e4f7cc(&mut program), loading_state::UNLOADED);
        assert_eq!(stub_e4f7f4(&mut program), 0);
        assert!(!stub_e4f840(&mut program));
        // Null-delegate mutators are no-ops (IDA 0xe4f884/0xe4f8aa/0xe4f8d0/0xe4f8fc).
        stub_e4f70c(&mut program);
        stub_e4f758(&mut program);
        stub_e4f868(&mut program, true);
        stub_e4f890(&mut program);
        stub_e4f8b4(&mut program);
        stub_e4f8e0(&mut program);
        assert!(!stub_e4f77c(&mut program));
    }

    #[test]
    fn reload_unload_drive_delegate_latches() {
        let mut program = program_with_delegate();
        stub_e4f758(&mut program);
        assert!(!stub_e4f77c(&mut program));
        assert_eq!(stub_e4f7cc(&mut program), loading_state::UNLOADED);
        stub_e4f70c(&mut program);
        assert!(stub_e4f77c(&mut program));
        assert_eq!(stub_e4f7cc(&mut program), loading_state::LOADED);
        assert!(!stub_e4f7a4(&mut program));
    }

    #[test]
    fn background_loaded_escalate_and_size_forward() {
        let mut program = program_with_delegate();
        assert!(!stub_e4f840(&mut program));
        stub_e4f868(&mut program, true);
        assert!(stub_e4f840(&mut program));
        program.binding_delegate.as_mut().unwrap().resource_size = 4096;
        assert_eq!(stub_e4f7f4(&mut program), 4096);
        // Escalate a background load: LOADING → loaded + LOADED.
        program.binding_delegate.as_mut().unwrap().loading_state = loading_state::LOADING;
        program.binding_delegate.as_mut().unwrap().loaded = false;
        assert!(stub_e4f7a4(&mut program));
        stub_e4f890(&mut program);
        assert!(stub_e4f77c(&mut program));
        assert_eq!(stub_e4f7cc(&mut program), loading_state::LOADED);
    }

    #[test]
    fn listeners_count_up_and_down() {
        let mut program = program_with_delegate();
        stub_e4f8b4(&mut program);
        stub_e4f8b4(&mut program);
        assert_eq!(program.binding_delegate.as_ref().unwrap().listeners, 2);
        stub_e4f8e0(&mut program);
        assert_eq!(program.binding_delegate.as_ref().unwrap().listeners, 1);
        stub_e4f8e0(&mut program);
        stub_e4f8e0(&mut program);
        assert_eq!(program.binding_delegate.as_ref().unwrap().listeners, 0);
    }

    #[test]
    fn cmd_delegate_round_trips_blank_and_name() {
        assert_eq!(stub_e4ffdc(), "");
        let mut program = UnifiedHighLevelGpuProgram::new();
        stub_e4fff4(&mut program, "hlsl");
        assert_eq!(program.delegate_names, vec!["hlsl".to_owned()]);
    }

    #[test]
    fn factory_constructs() {
        let factory = stub_e50004();
        assert_eq!(factory, UnifiedHighLevelGpuProgramFactory::new());
    }

    #[test]
    #[should_panic(expected = "This method should never get called!")]
    fn create_low_level_impl_throws() {
        stub_e4f90c(&mut UnifiedHighLevelGpuProgram::new());
    }

    #[test]
    #[should_panic(expected = "This method should never get called!")]
    fn unload_high_level_impl_throws() {
        stub_e4fac0(&mut UnifiedHighLevelGpuProgram::new());
    }

    #[test]
    #[should_panic(expected = "This method should never get called!")]
    fn build_constant_definitions_throws() {
        stub_e4fc74(&mut UnifiedHighLevelGpuProgram::new());
    }

    #[test]
    #[should_panic(expected = "This method should never get called!")]
    fn load_from_source_throws() {
        stub_e4fe28(&mut UnifiedHighLevelGpuProgram::new());
    }
}
