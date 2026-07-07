# Momo
Momo is a fast JS/TS runtime, package manager, bundler, and test runner

# To-Do
## Package Manager
- [ ] **Workspaces & Monorepo Support**: we need native support for managing multi-package repositories (like `pnpm-workspace.yaml` or yarn workspaces) with intelligent topological execution, we'll be including the toml as well since it's better, with support to pnpm-workspace.yaml and npm. If multiple found in the environment, we can ask the user to pick one when they start
- [ ] **Strict Dependency Isolation (PnP or Hard Links)**: (Optional) we would also need strict dependency trees using symlinks/hardlinks (like pnpm's `.pnpm` store) or Plug'n'Play (PnP) to prevent phantom dependencies and save disk space.
- [ ] **Offline Mode & Zero-Install**: this is the ability to install packages completely from a local cache without network access (ask the user that, if the library is found, also ask them which verison by listing out the versions that are there), and support for committing dependencies to git (zero install).
- [ ] **Overrides & Resolutions**: should allow users to force specific versions of transitive dependencies deep in the tree to resolve conflicts or security vulnerabilities (also show the vulnerable versions and warn them to upgrade to a newer version).
- [ ] **Lifecycle Scripts Sandboxing**: sandbox and execute `preinstall`, `install`, and `postinstall` scripts securely, ideally with a prompt or a sandboxed environment to prevent malicious code execution.
- [ ] **Lockfile Conflict Resolution**: auto resolve git merge conflicts on the lockfile (similar to yarn's lockfile resolution).
- [ ] **Peer Dependency Strictness**: automatically warn or resolve peer dependency mismatches with clear, actionable diagnostics.

## JS/TS Runtime
- [ ] **Native TypeScript & JSX/TSX Execution**: execute `.ts`, `.tsx`, and `.jsx` files out of the box without requiring a separate build step or configuration like bun does
- [ ] **High Node.js Compatibility**: implement the full suite of Node.js built-in modules (`fs`, `path`, `crypto`, `http`, `events`, `stream`, etc.) to ensure existing ecosystem packages run flawlessly (using deno's src code would probably be helpful).
- [ ] **Web Standard APIs (WinterCG)**: (Optional/Plugin) Full compliance with Web APIs like `fetch`, `WebSocket`, `WebStreams`, `WebCrypto`, and `Request/Response`.
- [ ] **Built-in SQLite (or fast KV)**: (Optional/Plugin) Provide an ultra-fast, built-in SQLite driver (e.g., `momo:sqlite`) implemented in Rust natively for immediate persistence.
- [ ] **FFI (Foreign Function Interface)**: (Low priority) Allow low overhead calling of Rust, C, or Zig libraries directly from JavaScript/TypeScript.
- [ ] **Hot Module Replacement (HMR)**: watchdog live reload for the runtime, when a file changes, patch it in memory without restarting the whole process.
- [ ] **Macro System**: Support for executing JavaScript functions at bundle-time and inlining the result (like bun macros)

## Bundler
- [ ] **Fast Multi-Entry Bundler**: (low priority) capable of taking multiple entry points and rapidly producing code-split chunks.
- [ ] **Advanced Tree-Shaking**: aggressive dead code elimination, including analyzing module side-effects (`sideEffects: false`).
- [ ] **Minifier**: High-performance built-in JS/CSS minification.
- [ ] **CSS Extraction & Bundling**: (low priority) native understanding of CSS imports, producing bundled CSS assets alongside JS chunks.
- [ ] **Plugin API**: an extensible plugin system compatible with (or similar to) esbuild or Rollup, allowing the community to extend the bundler.
- [ ] **Targeting & Transpilation**: down compile modern syntax to older ECMAScript targets (e.g., `es2015`) automatically (oxc can help)

## Test Runner
- [ ] **Snapshot Testing**: support for capturing and comparing UI/Data snapshots.
- [ ] **Built-in Mocking & Spying APIs**: Provide `vi.mock`, `jest.spyOn`, and fake timers natively so users don't need third-party mocking libraries.
- [ ] **DOM Environment Support**: Integration with `jsdom` or `happy-dom` to support testing React/Vue/Svelte components.
- [ ] **Intelligent Watch Mode**: (partially done) Rerun only the tests that depend on the files that actually changed, utilizing an internal module dependency graph.
- [ ] **Code Coverage**: built in coverage reporting (SWC coverage based).
- [ ] **Worker Thread Isolation**: run tests in isolated threads or processes to prevent global state leakage.

## Developer Experience (DX) & Tooling
- [ ] **Built-in Task Runner**: alternative to `npm run` that executes scripts directly without Node.js startup overhead.
- [ ] **Native `.env` Loading**: automatically parse and inject environment variables from `.env`, `.env.local`, etc., on startup.
- [ ] **Interactive REPL**: A TypeScript-aware REP Loop for quick testing.
- [ ] **Binary Updater**: a command  `momo upgrade` would seamlessly fetch and install the latest CLI binary.
- [ ] **Telemetry & Profiling**: (low priority) Built in CPU and memory profiling tools (`--inspect` / DevTools integration) to help developers debug memory leaks or slow code.