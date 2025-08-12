# deep-journal
Like to use LLMs, but don't want to give your chat data up? Use this. Everything runs on your machine and stores your chat data to a local SQLite database file (data/journal.db). Installable via a lightweight installer.

Technologies:
- LLM Model: deepseek-r1:1.5b
  - The app can easily be modified to select more model options from dropdown menu
- Frontend: Svelte + Tauri
- Backend: Rust + Tauri
- Database: SQLite

  
## Installation
1. [Download and install Ollama CLI](https://ollama.com/download). (This is what runs the DeepSeek model.)
2. Go to GitHub Releases page and install deep-journal installer.
3. Optional: install [DB Browser for SQLite](https://sqlitebrowser.org/) to browse/query/export chat data in your `journal.db` file.

## Model Setup
[Download Ollama CLI](https://ollama.com/download). The DeepSeek model is working if you can run the following:
```
ollama pull deepseek-r1:1.5b
ollama run deepseek-r1:1.5b
```

## Development Setup Notes
### Create Svelte Frontend
```powershell
npm create vite@latest tauri-app
```
- Framework: Svelte
- variant: TypeScript

```
cd tauri-app
npm install
npm run dev
```
### Add Tauri (Rust) to the project
```
npm install --save-dev @tauri-apps/cli
cargo init --bin src-tauri
npx tauri init --force
```
Set local dev server to Vite default port `5173` (instead of Tauri's default `8080`).
### Update package.json scripts
Add `tauri` to list of scripts like so:
```
"scripts": {
  "dev": "vite",
  "build": "vite build",
  "preview": "vite preview",
  "tauri": "tauri"
}
```
### Run app in dev mode
```
npm run tauri dev
```

## Development
Once setup complete, on opening of project run:
```
cd tauri-app
npm run tauri dev
```
(Open WebView console on app window with ctrl+shift+I to view Svelte console.log() output.)

## Building for Release
See `tauri-app/src-tauri/tauri.conf.json` for build configurations.
```
cd tauri-app
npm run build
npm run tauri build
```
On Windows, path to built program is: "tauri-app\src-tauri\target\release\app.exe"

