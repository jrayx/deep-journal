# deep-journal
Like to use LLMs, but don't want to give your chat data up? Use this. Everything runs on your machine and stores your chat data to a local SQLite database file (data/journal.db). Installable via a lightweight installer.

Technologies:
- LLM Model: DeepSeek
- Frontend: Svelte + Tauri
- Database: SQLite
- Backend: Rust + Tauri
  
## Installation
1. [Download and install Ollama CLI](https://ollama.com/download). (This is what runs the DeepSeek model.)
2. Go to GitHub Releases page and install deep-journal installer.
3. Optional: install [DB Browser for SQLite](https://sqlitebrowser.org/) to browse/query/export chat data in your `journal.db` file.

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

<!-- ## Diesel Installation & Setup
Install the following:
1. [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) (Tools for Visual Studio > Build Tools for Visual Studio). Select "Desktop development with C++" in the Visual Studio installer UI.
2. SQLite binaries with .lib files. To do that:

Build necessary SQLite files:
```powershell
cd C:\projects
git clone https://github.com/microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg install sqlite3:x64-windows

Now open Command Prompt and set env vars:
```cmd
set VCPKGRS_DYNAMIC=1
set LIBRARY_PATH=C:\projects\vcpkg\installed\x64-windows\lib;%LIBRARY_PATH%
set INCLUDE=C:\projects\vcpkg\installed\x64-windows\include;%INCLUDE%
```

Set these too (tell Rust where to find the libs):
```cmd
set LIB=C:\projects\vcpkg\installed\x64-windows\lib;%LIB%
set INCLUDE=C:\projects\vcpkg\installed\x64-windows\include;%INCLUDE%
```

Finally, after setting all the env vars above (don't restart or close Command Prompt, or env vars lost!) run the command to install the Diesel CLI:
```
cargo install diesel_cli --no-default-features --features sqlite --verbose
```

Once that completes successfully, do:
```
cd C:\projects\deep-journal\tauri-app\src-tauri
cargo add diesel --features sqlite
cargo add dotenvy # for env var handling; Diesel will look for DATABASE_URL variable in .env file for SQLite .db file path
```

Add `tauri-app/src-tauri/.env` file with `DATABASE_URL=journal.db`, and then run Diesel setup using Diesel CLI:
```
diesel setup
diesel migration generate init
``` -->

## SeaORM Installation & Setup
Add the following dependencies to Cargo.toml:
```
sea-orm = { version = "1.1.0", features = ["sqlx-sqlite", "runtime-tokio-rustls", "macros"] }
tokio = { version = "1.42.0", features = ["full"] }
```

Install the SeaORM CLI:
```
cargo install sea-orm-cli
```

After creating tables, do the following to generate entity models:
```
sea-orm-cli generate entity -u sqlite://../data/journal.db?mode=rwc -o src/entity
```

Move the "entity" folder generated into "src" folder.


<!-- Initialize migrations folder:
```
sea-orm-cli migrate init
```

Create default migration files for our tables:
```
sea-orm-cli migrate generate create_tables
```

Edit the files to reflect how you want the tables to look.

Then apply migrations:
```
sea-orm-cli migrate up --database-url sqlite://../../data/journal.db
``` -->


