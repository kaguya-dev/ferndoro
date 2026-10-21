# 🌿 FernDoro

> A sleek, elegant **Material You (Material 3)** study and productivity suite built with **Rust** and **egui**, featuring the **Rogue-Pomodoro Dungeon** game mode.

FernDoro integrates four core study and gamification methodologies into a native desktop suite:
1. ⏱ **Pomodoro Focus Timer**: Antialiased circular dial with clean time readouts and customizable rhythms.
2. 🔄 **Study Cycle Planner & Preset Builder**: Balances multi-course university workloads with 1-click undergrad templates.
3. 📝 **Test Mode & Exam Auto-Booster**: Schedules exam countdowns and auto-injects needed study points directly into cycle blocks.
4. ⚔️ **Rogue Dungeon ("The Study Dungeon")**: Gamified focus wagering, Floor Dealer fate cards, anti-cheat focus locks, and permanent relic progression.
5. 📊 **Frequency & Habit Tracker**: 20+ week GitHub-style consistency heatmap and streak counting.

---

## ✨ Features

### ⚔️ 1. Rogue Dungeon (Gamified Focus Mode)
*Based on the Contrarian Casino & Dungeon Crawl mechanics:*
- **The Ante (Difficulty Wagering)**: Choose your discipline tier before entering an encounter:
  - `1x Safe Bet`: Standard timer, up to 2 pauses allowed, base Gold/XP payouts.
  - `2x Double Down`: 2x rewards, zero pauses. Leaving the window triggers a 10s grace countdown before bust!
  - `3x Boss Fight`: Minimum 50m block, strict focus lock, rare relic guarantee. Failing wipes your unbanked pot!
  - `5x Nightmare Boss`: Maximum stakes and reward payout.
- **The Floor Dealer (Break-Time Casino)**:
  - Immediately upon clearing a floor, draw 1 of 3 Mystery Fate Cards while your guaranteed break runs in the background.
  - **Boons**: *Temporal Flux* (+20% rewards), *Golden Touch* (2x Gold), *Soul Shield* (+1 free pause in Hard Mode), *Jackpot* (+75 instant Gold).
  - **Curses**: *Blind Run* (timer masked as `??:??` until < 60s remain), *Sudden Death* (60s warmup lock), *Double or Nothing* (forces 2x with 3x payout).
- **The Vault & Relic Shop**:
  - Bank unbanked gold into your permanent Vault anytime.
  - Spend gold on relics: *Loaded Dice* (+15% Boon odds), *Merchant's Coin* (+20% gold), *Hourglass of Mercy* (+5s grace period), *Aegis of Will* (start with Soul Shield).

### 🎨 2. Material You 3 Design & Aesthetics
- **Tonal Color Palette**: Soft lavender (`#D0BCFF`), rich dark violet surfaces (`#1D1926`, `#262133`), and warm rose accents (`#EFB8C8`).
- **Pill Navigation & Cards**: Full Material 3 rounded pill navigation tabs, generous card roundings (18px), soft outlines without harsh borders.
- **Calm Elegance**: Designed for low cognitive fatigue and high readability during long study sessions.

### ⏱ 3. Pomodoro Focus Engine
- **Custom Vector Dial**: Material 3 rounded progress arc with smooth capsule tip indicator and pulse dot.
- **Direct Cycle Link**: Link any session directly to a Study Cycle subject. Time logged automatically feeds your cycle progress.
- **Quick Presets**: `25/5 Standard`, `50/10 Lecture Block`, and `90/20 Deep Analytical`.
- **Desktop Alerts**: Native Linux notifications (`notify-send`) and bell alerts.

### 🔄 4. Study Cycle Planner & Preset Builder
- **Undergrad Preset Builder**: One-click university semester templates (Computer Science, STEM Heavy, Balanced 4x45m, Finals Sprint) plus custom preset saving.
- **Upgraded Time Setters**: Quick duration chips (`25m`, `45m`, `60m`, `90m`), delta adjusters (`-15m`, `+15m`), and formatted human-readable durations (`1h 30m`).

### 📝 5. Test Mode (Exams & Auto-Cycle Booster)
- **Exam Countdown**: Track upcoming university exams with days remaining, difficulty (1-5), and importance weights.
- **Automatic Points Injection**: Dynamically calculates urgency (Critical, High, Medium) and auto-boosts required study minutes in your Study Cycle.
- **Auto-Balance**: Rebalance entire semester rotation queue with a single click.

### 📊 6. Frequency & Habit Tracker
- **GitHub-Style Consistency Heatmap**: 20+ week visual matrix showcasing study frequency and intensity tiers with interactive tooltips.
- **Streak Counter**: Real-time current streak and all-time best streak tracking.
- **Daily Target Progress**: Visual progression bar tracking progress toward daily study goals.
- **Daily Habits Checklist**: Track essential complementary habits (Deep Work, Flashcards, Coding, Stretch & Hydration).

### 🎨 4. Purple Dark High-Contrast Visuals
- **Deep Obsidian & Neon Violet Palette**: Built from the ground up for high readability and night study sessions.
- **Extensible Theme Architecture**: Switch on the fly between **Purple Dark**, **Midnight Violet**, **Cyber Neon**, and **Obsidian Amethyst**. Ready for custom user themes.

---

## 🛠️ Installation & Building

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (edition 2021+)
- Linux dependencies: OpenGL/Mesa and standard development headers (`pkg-config`, `libxkbcommon`, `wayland`/`x11`).

### Build and Run from Source
```bash
git clone https://github.com/kaguya-dev/ferndoro.git
cd ferndoro
cargo run --release
```

### Install Directly to Your Desktop (`~/.local/bin`)
```bash
cargo build --release
cp target/release/ferndoro ~/.local/bin/ferndoro
chmod +x ~/.local/bin/ferndoro

# Install application icon and desktop launcher
mkdir -p ~/.local/share/icons/hicolor/scalable/apps ~/.local/share/applications
cp assets/icon.svg ~/.local/share/icons/hicolor/scalable/apps/ferndoro.svg
cp ferndoro.desktop ~/.local/share/applications/ferndoro.desktop
update-desktop-database ~/.local/share/applications
```

Now you can launch **FernDoro** from your application launcher or terminal with `ferndoro`!

---

## 📂 Project Structure

For detailed architectural notes, module structure, and future plans, check [ROADMAP.md](ROADMAP.md).

```
ferndoro/
├── assets/                   # App icon and visual assets
├── src/
│   ├── main.rs               # Entry point and viewport initialization
│   ├── app.rs                # GUI application, top navigation, and views
│   ├── theme.rs              # High-contrast theme palette system
│   ├── pomodoro.rs           # Pomodoro timer and state machine
│   ├── cycle.rs              # Study cycle planner & rotation engine
│   ├── frequency.rs          # Daily logs, streaks, and heatmap data
│   ├── storage.rs            # JSON persistence to ~/.config/ferndoro/
│   ├── notification.rs       # Async desktop notification handler
│   └── widgets/              # Custom vector widgets (Dial, Progress, Heatmap, Cards)
├── Cargo.toml
├── LICENSE
├── README.md
└── ROADMAP.md
```

---

## ⚙️ Configuration & Storage

User data and settings are automatically persisted as JSON in:
```
~/.config/ferndoro/ferndoro_state.json
```
This ensures your cycles, streaks, subjects, and custom timings remain intact between launches.

---

## 📄 License

Distributed under the [MIT License](LICENSE).
