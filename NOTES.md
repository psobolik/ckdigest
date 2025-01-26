# Scaffold
```terminal
⫸ cargo create-tauri-app
✔ Project name · ckhash
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm, bun)
✔ Choose your package manager · pnpm
✔ Choose your UI template · Vanilla
✔ Choose your UI flavor · TypeScript

Template created! To get started run:
  cd ckhash
  pnpm install
  pnpm tauri dev
```
# Development
```
pnpm tauri dev
```
Runs `pnpm dev` to run the front end with Vite, 
builds the Tauri executable into `target/debug`, and then runs it.
# Release
```
pnpm tauri build
```
Runs `tsc` to compile the TypeScript, 
`pnpm build` to build the front end with Vite, and builds the Tauri executable into `target/release`.
Tauri also creates installers for the app into subdirectories of `target/release/bundle`.  

On Windows, the executable is `.\src-tauri\target\release\svg-symbol-tool.exe`, and the installers are 
* `.\src-tauri\target\release\bundle\msi\Verify Digest_0.1.1_x64_en-US.msi`
* `.\src-tauri\target\release\bundle\nsis\Verify Digest_0.1.1_x64-setup.exe`

On Linux, the executable is `./src-tauri/target/release/verify-digest`, and the installers are 
* `./src-tauri/target/release/bundle/deb/verify-digest_0.1.1_amd64.deb`
* `./src-tauri/target/release/bundle/rpm/verify-digest-0.1.1-1.x86_64.rpm`
* `.src-tauri/target/release/bundle/appimage/verify-digest_0.1.1_amd64.AppImage`

# Version
* `./src-tauri/Cargo.toml`
* `./src-tauri/tauri.conf.json`
* `package.json`