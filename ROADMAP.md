# FernDoro - Architecture & Future Roadmap

**FernDoro** is a high-contrast dark purple study and productivity suite built in Rust with `egui` / `eframe`. It harmonizes **Pomodoro focus intervals**, **Study Cycle rotation**, and **Daily Frequency & Consistency tracking** into a single native desktop workflow.

---

## 🏛️ Project Architecture & Directory Structure

```
ferndoro/
├── assets/
│   └── icon.svg              # Scalable SVG application icon
├── src/
│   ├── main.rs               # Desktop application entry point & native viewport setup
│   ├── app.rs                # Main FernDoroApp state, tab router, top navigation & views
│   ├── theme.rs              # Extensible palette system & high-contrast Purple Dark visuals
│   ├── storage.rs            # State persistence to ~/.config/ferndoro/ferndoro_state.json
│   ├── notification.rs       # Async desktop notifications (notify-send) and chimes
│   ├── pomodoro.rs           # Pomodoro state machine, timers, presets & auto-advance
│   ├── cycle.rs              # Study cycle planner, subject rotations & progress calculations
│   ├── frequency.rs          # Daily logs, streaks, habit checklists & heatmap aggregation
│   └── widgets/
│       ├── mod.rs            # Widget module re-exports
│       ├── dial.rs           # Custom circular timer dial with glow & ticks
│       ├── progress.rs       # High-contrast rounded progression bars
│       ├── heatmap.rs        # GitHub-style consistency matrix / activity grid
│       └── cards.rs          # Stat cards, glowing frames & status badges
├── Cargo.toml                # Crate manifest and dependency configuration
├── .gitignore                # Git ignore rules for Rust builds and local state
├── LICENSE                   # MIT License
├── README.md                 # Project presentation, installation & usage guide
└── ROADMAP.md                # Architecture documentation & future feature roadmap
```

---

## 🧩 Architectural Design

### 1. Extensible Theme Engine (`src/theme.rs`)
- Designed around the `ThemePalette` struct and `ThemeMode` enum.
- Default: **Purple Dark** (Deep obsidian violet `#0F0C18`, amethyst accents `#A855F7`, fuchsia glow `#E879F9`).
- Includes built-in variants: **Midnight Violet**, **Cyber Neon**, and **Obsidian Amethyst**.
- New themes can be added simply by adding a new variant to `ThemeMode` and providing its `ThemePalette`.

### 2. Connected Workflow Architecture
- **Pomodoro ⇄ Study Cycle Link**: Selecting an active subject in the Study Cycle automatically updates the active Pomodoro target. Completing a Pomodoro session automatically logs focused minutes to the active subject in the Study Cycle and advances the cycle if the target is met.
- **Pomodoro ⇄ Frequency Tracker Link**: Completed focus blocks automatically update today's total study minutes, increment the daily Pomodoro count, and extend your consecutive day streak.

### 3. Custom Vector Widgets (`src/widgets/`)
- Pure immediate-mode vector drawing using `egui::Painter`.
- **`TimerDial`**: Features an outer glow halo, 60 precision tick marks, dynamic sweep arc with tip indicator dot, large time readout, and running pulse indicator.
- **`CustomProgressBar`**: Rounded progress bars with glowing highlight strips, custom colors, subject tags, and percentage readouts.
- **`FrequencyHeatmap`**: Multi-week matrix showing intensity tiers and interactive hover tooltips with daily stats.

---

## 🚀 List of Planned Features (Future Roadmap)

### 🌿 Version 0.2: Audio & Immersion
- [ ] **Ambient Soundscapes Engine**: Built-in sound generator for brown noise, rain, forest breeze, and binaural 40Hz focus tones using `rodio`.
- [ ] **Custom Audio Alerts**: Selectable session completion chimes (Tibetan singing bowl, digital beep, soft bell).
- [ ] **Lofi / Web Stream Integration**: Optional background ambient internet radio stream.

### 🔄 Version 0.3: Deep Study & Cycle Enhancements
- [ ] **Subject Task Checklists**: Drill down into specific chapter/topic checklists inside each study cycle block.
- [ ] **Cycle Weight Auto-Balancing**: Algorithm to dynamically recommend subject time blocks based on upcoming exam dates or perceived difficulty.
- [ ] **Session Pause Reason Tracker**: Log reasons for pauses/interruptions (e.g. phone, question, fatigue) to identify distraction patterns.

### 📊 Version 0.4: Advanced Analytics & Anki Sync
- [ ] **Interactive Detailed Analytics Tab**:
  - Weekly and monthly time breakdown pie charts and bar graphs.
  - Hour-of-day heatmaps to identify your peak productivity hours.
- [ ] **CSV / JSON Export**: Export study logs for external spreadsheets or Obsidian personal knowledge management.
- [ ] **Anki Spaced Repetition Hook**: Track Anki flashcard review counts alongside study blocks.

### ⚡ Version 0.5: Desktop Productivity Integration
- [ ] **System Tray Minimization**: Minimize to Linux system tray with countdown tooltip and tray icon.
- [ ] **Floating Mini-Widget Mode**: Compact always-on-top pill timer for multi-monitor setups.
- [ ] **Global Keyboard Shortcuts**: Start/Pause/Skip via global hotkeys (e.g. `Super + Alt + Space`).
- [ ] **DND (Do Not Disturb) Automation**: Trigger desktop Do Not Disturb mode while a focus session is active.

### 🌐 Version 0.6: Cloud Sync & Mobile Companion
- [ ] **Self-Hosted Sync Server**: Lightweight backend for synchronizing cycle progress and habit streaks across laptop and desktop.
- [ ] **WebAssembly (WASM) Build**: Compile FernDoro directly to web browsers via `Trunk` for on-the-go access.
