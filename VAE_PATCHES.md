# Wild Surmise `surge-rs` fork — VAE patch set

This is a fork of [surge-synthesizer/surge-rs](https://github.com/surge-synthesizer/surge-rs)
maintained for the VAE Surge instrument. It exists so the bindings can be
**pinned** and **build on macOS**.

Upstream base: `7bfeafc76d1c57860a177e9e076bed7ec764009a`

## Pinned Surge XT revision

`surge-sys/build.rs` clones Surge XT at build time. Upstream clones the
**default branch at depth 1**, which is neither reproducible nor able to
check out a pinned revision/submodule SHAs. This fork:

- clones full history (no `depth(1)`);
- checks out a pinned commit and detaches HEAD;
- syncs submodules via the **git CLI** (`git submodule update --init
  --recursive`). Upstream used `git2`, which cannot fetch submodule commits
  that aren't reachable from the submodule's default branch (e.g.
  `libs/sst/sst-effects` @ `5f2bd1a8`), so a clean checkout fails.

Pinned commit: `45568c7d60be52aa2a4f27dd7b7a074f56bd00ba`

To bump, edit `SURGE_XT_COMMIT` in `surge-sys/build.rs` and rebuild.

## macOS link patches

1. **`surge-common-binary`** — Surge's cmrc embedded-resource archive is
   referenced by `SurgeStorage.cpp` in all build modes, but upstream links it
   only under the `in-surge-tree` feature, giving an undefined
   `cmrc::surge_common_binary::get_filesystem()`. Now linked unconditionally.
2. **Objective-C frameworks** — `sst-plugininfra`'s `paths_macos.mm` needs the
   ObjC runtime and Foundation/AppKit, which upstream never links
   (`_OBJC_CLASS_$_NSFileManager`, `_objc_msgSend`). Now linked on macOS.

## Parameter / state bridge patches (Phase 2F)

Built so the plugin can expose Surge's **whole** parameter surface and round-trip
its full patch state.

1. **`getParameterMetaFull` (`surge-sys/cpp/plumber.{h,cpp}` + `src/hell_ffi.rs`).**
   Surge's own `SurgeSynthesizer::getParameterMeta` deliberately reports
   `fmin`/`fmax` as `0.0`/`1.0` (the real fields are commented out in
   `SurgeSynthesizer.cpp`), which cannot drive discrete host params. This
   fork-owned plumber function reads the real `Parameter::val_min`/`val_max`
   (per `valtype`), the normalized default, and adds the stable-key fields
   `ctrlgroup`, `ctrlgroup_entry`, `scene`. The Rust glue exposes it as
   `SurgeSynthesizer::get_parameter_meta(&mut SurgeId) -> ParameterMeta`.
2. **`SurgeSynthesizer::save_patch()` (Rust glue).** Wraps the bindgen-exposed
   `SurgeSynthesizer::saveRaw(void**)` (defined in
   `src/common/SurgeSynthesizerIO.cpp`) into `-> Vec<u8>`. Takes `&self` so it
   fits Truce's `PluginLogic::save_state(&DspState)` (the host serializes state
   with the audio thread stopped; `saveRaw` only touches Surge's patch buffer).
   The bytes are copied out; Surge owns the allocation (freed on the next save /
   on drop), so callers must not free. This is the counterpart of the existing
   `load_raw` and carries the full patch stream (mod routings, wavetables,
   step-seq, both scenes).

No Surge XT source is modified: `surge-sys/sbmod/surge` is cloned by `build.rs`
and is not part of this repo.

## Usage

Pin by revision (cargo does not track branches for reproducibility):

```toml
surge-rs = { git = "ssh://git@github.com/Wild-Surmise/surge-rs.git", rev = "<commit>" }
```

## Licence

Surge XT and this fork are **GPL-3.0-or-later**. Any distributed binary that
links this code is a combined work and must satisfy GPL obligations
(corresponding source, notices, no additional restrictions). This is not legal
advice; obtain review before proprietary branding/distribution.
