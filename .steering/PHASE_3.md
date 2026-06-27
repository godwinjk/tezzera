# Phase 3 — Build Tooling, Navigation & Dev Experience

> Status: IN PROGRESS
> Started: 2026-06-27
> Target: MVP distributable — web + desktop + developer tooling

## Steps

### Step 1 — `tzr new <name>` project scaffolding ✅
- Scaffolds `Cargo.toml`, `src/main.rs`, `.gitignore` for a new TEZZERA app
- Template is a working counter app with TezzeraApp, use_atom, SkiaCanvas
- Invalid names (spaces, special chars) and existing directories rejected with descriptive errors
- Commits: `feat(cli): add tzr new <name> project scaffolding`

### Step 2 — WASM platform backend ✅
- `tezzera-platform/src/web.rs` — run_web<F> backed by web-sys CanvasRenderingContext2d
- BGRA→RGBA channel swap for browser compatibility (tiny-skia outputs BGRA)
- `#[cfg(target_arch = "wasm32")]` gating — zero cost on native builds
- Commits: `feat(platform): add WASM web backend with canvas renderer`

### Step 3 — `tzr build --target web` ✅
- Checks/installs wasm32-unknown-unknown target via rustup
- Runs `cargo build --target wasm32-unknown-unknown --release`
- Runs wasm-bindgen if available (graceful fallback to raw .wasm copy)
- Generates `dist/index.html` with purple-on-dark TEZZERA styling
- Commits: `feat(cli): add tzr build --target web with wasm32 compilation and dist generation`

### Step 4 — `tezzera-nav` navigation router ✅
- Route trait (Debug + Clone + PartialEq + Send + Sync + 'static)
- NavigationStack<R> backed by Atom<Vec<R>> — reactive, UI rebuilds on navigation
- Navigator<R>: push/pop/replace/reset_to/current/can_go_back/depth
- NavigationGuard trait + AllowAllGuard + BlockWhenGuard
- KeepAliveRegistry (D030) — navigated-away screens stay in memory until reset
- RouteChange trace events on every navigation action
- 29 tests, all passing
- Commits: 3 commits (scaffold → Route/Stack/History → Navigator/Guard)

### Step 5 — `tzr package` desktop bundling ✅
- macOS: `.app` bundle with Info.plist, CFBundleIdentifier, NSHighResolutionCapable
- Linux: standalone binary + `.deb` tree (dpkg-deb if available)
- Windows: versioned `.exe` copy with sign reminder
- Reads name/version from Cargo.toml if not specified via flags
- Commits: `feat(cli): add tzr package command for macOS .app, Linux .deb, Windows .exe`

### Step 6 — `tzr dev` + web dev server ✅
- `tzr dev` → `cargo run` (desktop dev mode)
- `tzr dev --target web` → WASM build + pure-Rust HTTP server on :3000
- HTTP server: TcpListener, path→file mapping, MIME types, anti-traversal
- --port flag for custom port
- Commits: `feat(cli): add tzr dev command — desktop run and WASM local server`

### Step 7 — `tezzera-hot-reload` file watcher ✅
- Polling-based watcher (std only, no external notify crate)
- 200ms poll interval, 100ms debounce window
- Skips `target/` and hidden dirs
- RebuildRunner: listens for ChangeEvent, runs cargo build on each
- Supports Desktop + Web rebuild targets
- Commits: 3 commits (scaffold → FileWatcher/Debouncer → RebuildRunner)

### Step 8 — Wire hot-reload into `tzr dev --watch`
- `tzr dev --watch` flag: starts FileWatcher on `src/`, pipes events to RebuildRunner
- For web: restarts the serve loop after rebuild  
- For desktop: prints "rebuilt — restart app to see changes" (full hot-swap is Phase 4)
- Status: TODO

### Step 9 — Dev tools (tezzera-devtools crate)
- Trace viewer: reads TezzeraTrace events from a channel, renders ASCII summary
- Component inspector: walks the layout tree and prints the box model
- Time-travel: snapshot Atom values at each frame, step back/forward
- Status: TODO

## Exit Criteria

- [ ] `tzr new <name>` creates a runnable app skeleton
- [ ] `tzr build --target web` produces a working `dist/` with index.html + .wasm
- [ ] `tzr build --target desktop` produces a release binary
- [ ] `tzr package` creates `.app` / `.deb` / `.exe` on each platform
- [ ] `tzr dev` launches the desktop app
- [ ] `tzr dev --target web` serves at http://localhost:3000
- [ ] `tezzera-nav` Navigator push/pop round-trip tested end-to-end
- [ ] `tezzera-hot-reload` FileWatcher detects `.rs` changes within 500ms
- [ ] `tzr dev --watch` triggers rebuild on source change
- [ ] All workspace tests pass with zero warnings

## Approved dependencies

- `wasm-bindgen` 0.2 — JS/WASM bridge (already gated under `cfg(target_arch = "wasm32")`)
- `web-sys` 0.3 — browser DOM/Canvas API (same gating)
- `js-sys` 0.3 — JS primitives
- `console_error_panic_hook` 0.1 — WASM panic messages
- No external file-watch crates — watcher is polling-based with std only
- No Node.js / Python required for web dev server — pure Rust TcpListener

## DO NOT

- DO NOT add GPU rendering (skia-safe) in Phase 3 — that is v1.0
- DO NOT add live DOM diffing or React-style reconciler — that is Phase 4
- DO NOT require npm/yarn/node for web builds — must work with only Rust toolchain
- DO NOT add time-travel debugger UI in Phase 3 — snapshot API only
