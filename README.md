# deep-journal
Like to use LLMs, but don't want to give your chat data up? Use this. Everything runs on your machine and stores your chat data to a local SQLite database file (data/journal.db). Installable via a lightweight installer. (Use [DB Browser for SQLite](https://sqlitebrowser.org/) to easily browse/query/export data from SQLite .db files.)

Technologies:
- LLM Model: DeepSeek
- Frontend: Svelte
- Database: SQLite
- Backend: Rust + Tauri
  - Database ORM + Migrations: Diesel

## App Setup
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

<!-- ## Development with WSL
Setup commands:

Set up WSL:
```powershell
wsl --install
wsl.exe -d Ubuntu
```

Install rustup for Rust lang:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install Node.js:
```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
\. "$HOME/.nvm/nvm.sh"
nvm install 22
```

Tauri+Svelte app initialization commands (set Identifier as `com.deep-journal.tauri-app` and package manager `npm`):
```
npm create tauri-app@latest tauri-app -- --template svelte
cd tauri-app
sudo apt update
sudo apt install build-essential
npm install
```

Run development server:
```
npm run tauri dev
``` -->

## Model Setup
[Download Ollama CLI](https://ollama.com/download).
```
ollama pull deepseek-r1:1.5b
ollama run deepseek-r1:1.5b
```

