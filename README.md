# Pomodori

Pomodori is a lightweight **Pomodoro timer** desktop app built with **Tauri + Vite + TypeScript**. It includes **tray controls**, **desktop notifications**, **settings**, and **basic focus statistics** stored locally.

## Features

- **Timer**: Work / Short break / Long break sessions
- **Automation**: Auto-start breaks and/or auto-start work sessions
- **Tray menu**: Start, pause, stop, show/hide window, quit
- **Desktop notifications**: Optional notifications on session completion
- **Sound effects**: Optional sound on ticks/completion (configurable)
- **Stats window**: Today + all-time session count and focus time
- **Local persistence**: Stores settings + session history on your machine

## Keyboard shortcuts (main window)

- **Space**: Start / Pause / Resume (depending on current state)
- **Esc** or **S**: Stop (when running/paused)
- **R**: Reset
- **Cmd + ,** (macOS): Open Settings

## Tech stack

- **Frontend**: Vite + TypeScript (vanilla DOM)
- **Desktop**: Tauri 2
- **Storage**:
  - Settings: `tauri-plugin-store`
  - Sessions/Stats: SQLite (via `sqlx`) in the Tauri app data directory

## Setup

### Prerequisites

- **Bun** (the project is configured to use Bun for Tauri hooks)
- **Rust toolchain** (stable)
- **Tauri system dependencies** (vary by OS)

Tauri prerequisites are documented here: `https://tauri.app/start/prerequisites/`.

### Install

```bash
bun install
```

## Development

### Run the desktop app (recommended)

```bash
bun run tauri dev
```

### Run the frontend only (Vite)

```bash
bun run dev
```

Vite runs at `http://localhost:1420`.

## Build

### Build the frontend bundle

```bash
bun run build
```

### Build the desktop app (bundles)

```bash
bun run build:app
```

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

This project is licensed under the MIT License. See the [`LICENSE`](LICENSE) file for details.
