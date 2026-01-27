# Contributing to Pomodori

Thank you for your interest in contributing to Pomodori!

## Prerequisites

- Bun
- Rust toolchain (stable)
- Tauri system dependencies for your OS: `https://tauri.app/start/prerequisites/`

## Setup

```bash
bun install
```

## Running locally

### Desktop app (recommended)

```bash
bun run tauri dev
```

### Frontend only

```bash
bun run dev
```

## Making changes

### Branching

- Create a branch from `main`
- Keep changes focused and reasonably small

### Code quality

- Prefer clear naming and self-documenting code
- Avoid unrelated refactors in the same PR

### Testing

- Smoke test the full app via `bun run tauri dev`
- If your change touches settings, tray, or stats, verify those flows manually

## Pull requests

Include:

- What changed and why
- How you tested it (commands + steps)
- Screenshots/GIFs for UI changes (if applicable)

## Reporting issues

When filing a bug, include:

- OS + version
- Steps to reproduce
- Expected vs actual behavior
- Any relevant logs/error output
