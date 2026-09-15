# Modular Linux installer

TUI installer for the **Modular Linux** distribution. It is specific to Modular
(partitioning, packages, branding, and post-install setup) and is not a generic
Arch installer with a skin.

Built with [Ratatui](https://ratatui.rs/) and Crossterm.

## Re-use

Reuse is welcome. The flow is a list of small **steps**; each step owns its own
input handling and rendering. To adapt it for another distro or layout:

1. Add or remove a module under `src/steps/`
2. Implement `StepTrait` (`on_key` + `render`)
3. Register it in `StepId`, `next` / `back`, and the `match` arms in `AppState`
4. Store any new answers on `State`
5. Call into `src/utils/install.rs` (or your own install helpers) when Confirm runs

UI widgets live in `src/components/` so steps stay thin.

## Architecture
```
┌─────────────────────────────────────────────────────────┐
│  AppState                                                │
│  ├── state: State      form answers (user, disk, …)     │
│  ├── install_*         progress for the Installing step │
│  ├── step: StepId      current wizard screen            │
│  └── install_rx        channel from the install thread  │
└─────────────────────────────────────────────────────────┘
         │                              │
         │ UI thread                    │ background thread
         ▼                              ▼
   steps/* (Welcome → … → Confirm)   utils/install.rs
         │                              │
         │ Confirm spawns thread ───────┘
         │   clones State, sends InstallEvent::Progress
         ▼
   Installing (gauge) → Done
``` 


### State vs UI

| Type | Role |
|------|------|
| `State` | Install answers only — safe to clone into the install thread |
| `AppState` | Wizard + progress — step id, gauge, channel, event loop |

Install logic takes `&State` plus an `on_progress` callback. It never talks to Ratatui.

### Steps

Each step is a small type in `src/steps/` implementing `StepTrait`:

- **`on_key`** — handle input, return `Transition::{None, Next, Back, Quit}`
- **`render`** — draw into the shared card layout

Order (see `StepId`):

`Welcome → Username → Hostname → Password → Timezone → Locale → Keymap → BootManager → Disk → Swap → Confirm → Installing → Done`

### Install pipeline

`utils/install.rs` runs after Confirm, roughly:

1. Partition / format / mount
2. `pacstrap` (base system + Modular packages)
3. Configure hostname, locale, keymap, users, sudo
4. Bootloader + enable services
5. Unmount

Progress is reported over an `mpsc` channel as `InstallEvent`; the UI polls it while on the Installing step and updates the gauge.

### Layout of the crate
```
src/
├── main.rs              entry — runs AppState
├── components/          Prompt, ScrollList, Card, Gauge, …
├── layout/              shared vertical layout helpers
├── steps/               wizard screens + AppState / State
└── utils/
    ├── cmd.rs           run / chroot helpers
    ├── install.rs       real install sequence
    ├── focus.rs         multi-field focus
    └── key.rs           shared key handling
```


## Build

```bash
cargo build --release
```
```

