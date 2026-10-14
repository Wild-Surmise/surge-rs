include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::ffi;

/// VAE: mirror of the fork's `SurgeParamMeta` in `cpp/plumber.h`. Surge's own
/// `parametermeta` reports fmin/fmax as 0/1; this carries the real engine range
/// plus the stable-key triple `(ctrlgroup, ctrlgroup_entry, scene)`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SurgeParamMeta {
    pub fmin: f32,
    pub fmax: f32,
    pub fdefault: f32,
    pub valtype: i32,
    pub ctrlgroup: i32,
    pub ctrlgroup_entry: i32,
    pub scene: i32,
    pub flags: u32,
    pub hide: bool,
    pub expert: bool,
    pub meta: bool,
}

unsafe extern "C" {
    pub fn create_engine(sr: f32) -> *mut SurgeSynthesizer;
    pub fn create_patch() -> *mut SurgePatch;
    pub fn destroy_engine(surge: *mut SurgeSynthesizer);
    pub fn destroy_patch(patch: *mut SurgePatch);
    pub fn destroy_parameter(p: *mut Parameter);

    pub fn getNumInputs(surge: *mut SurgeSynthesizer) -> i32;   // TODO: convert to const?
    pub fn getNumOutputs(surge: *mut SurgeSynthesizer) -> i32;  // update: no, but look into it more.
    pub fn getBlockSize(surge: *mut SurgeSynthesizer) -> i32;   // as in, it didn't work.

    pub fn getSynthSideId(id: *const SurgeSynthesizer_ID) -> i32;

    pub fn fromSynthSideId(surge: *const SurgeSynthesizer, i: i32 , q: *mut SurgeSynthesizer_ID) -> bool;
    pub fn idForParameter(surge: *const SurgeSynthesizer, p: *const Parameter) -> SurgeSynthesizer_ID;
    pub fn getParameterDisplay(surge: *const SurgeSynthesizer, index: *mut SurgeSynthesizer_ID, text: *mut ffi::c_char);
    pub fn getParameterDisplayAlt(surge: *const SurgeSynthesizer, index: *mut SurgeSynthesizer_ID, text: *mut ffi::c_char);
    pub fn getParameterName(surge: *const SurgeSynthesizer, index: *mut SurgeSynthesizer_ID, text: *mut ffi::c_char);
    pub fn getParameterNameExtendedByFXGroup(surge: *const SurgeSynthesizer, index: *mut SurgeSynthesizer_ID, text: *mut ffi::c_char);
    pub fn getParameterAccessibleName(surge: *const SurgeSynthesizer, index: *mut SurgeSynthesizer_ID, text: *mut ffi::c_char);
    pub fn getParameterMeta(surge: *const SurgeSynthesizer, index: *mut SurgeSynthesizer_ID, pm: *mut parametermeta);
    pub fn getParameterMetaFull(surge: *const SurgeSynthesizer, index: *mut SurgeSynthesizer_ID, out: *mut SurgeParamMeta);
    pub fn getParameter01(surge: *const SurgeSynthesizer, index: *mut SurgeSynthesizer_ID) -> f32;
    pub fn setParameter01(surge: *mut SurgeSynthesizer, index: *mut SurgeSynthesizer_ID, value: f32, external: bool, force_integer: bool) -> bool;

    pub fn normalizedToValue(surge: *const SurgeSynthesizer, index: *mut SurgeSynthesizer_ID, value: f32) -> f32;
    pub fn valueToNormalized(surge: *const SurgeSynthesizer, index: *mut SurgeSynthesizer_ID, value: f32) -> f32;
    pub fn sendParameterAutomation(surge: *mut SurgeSynthesizer, index: *mut SurgeSynthesizer_ID, value: f32) -> f32;
}
