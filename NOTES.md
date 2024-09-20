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

```
pnpm tauri dev
```
Runs `pnpm dev` to run the front end with Vite, 
builds the Tauri executable into target/debug, and then runs it.
```
pnpm tauri build
```
Runs `tsc` to compile the TypeScript, 
`pnpm build` to build the front end with Vite, and builds the Tauri executable into `target/release`.
Tauri also creates installers for the app into subdirectories of `target/release/bundle`.  

On Windows, the executable is `.\src-tauri\target\release\svg-symbol-tool.exe`, and the installers go in `.\src-tauri\target\release\bundle`.
