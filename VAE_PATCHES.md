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
- then syncs submodules recursively at the commits that revision records.

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
