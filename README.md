# Tauri Bevy Svelte App

Experimenting with the integration of the Bevy game engine and a Tauri app using Svelte views.

- **Tauri**
- **Bevy**
- **GitHub action** for cross-platform builds
- **Svelte**
- **Vite**
- **TypeScript**
- **`svelte-preprocess`** with Sass installed by default
- **Hot module replacement**
- **ESLint**
- **Prettier**

## Dev instructions

### Get started

1. Install Node.js
2. Install Yarn
3. Install Rust
   . Follow the [Tauri setup guide](https://tauri.studio/en/docs/get-started/intro)
4. Run `yarn` to install dependencies
5. From the `src-tauri` folder, run `cargo check` to install dependencies

### Commands

- `yarn dev`: Start app in dev mode
- `yarn build`: Build
- `yarn lint`: Lint
- `yarn format`: Format

### Release new version

1. Update `CHANGELOG.md`
2. Bump the version number in `src-tauri/Cargo.toml`
3. Run `cargo check` to update `Cargo.lock`
4. Create a git tag in the format `v#.#.#`
5. Add release notes to the generated GitHub release and publish it
