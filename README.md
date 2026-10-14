# 🌿 FernDoro

> A sleek, high-contrast dark purple study and productivity suite built with **Rust** and **egui**.

FernDoro combines three core study methodologies into an integrated native desktop app:
1. ⏱ **Pomodoro Focus Timer** with glowing dials and precision intervals.
2. 🔄 **Study Cycle Rotation** to balance multi-subject learning without rigid calendar schedules.
3. 📊 **Frequency & Habit Tracker** with a GitHub-style consistency heatmap and streak counting.

---

## ✨ Features

### ⏱ 1. Pomodoro Focus Engine
- **Custom Circular Dial**: Antialiased circular gauge with 60 precision ticks, glowing progress sweep, tip indicator, and large digital readout.
- **Dynamic Phases**: Work sessions, short breaks, and long breaks with customizable intervals.
- **Direct Cycle Link**: Link any session directly to a Study Cycle subject. Time logged automatically feeds your cycle progress.
- **Quick Presets**: One-click switching between `25/5 Classic`, `50/10 Deep Work`, and `90/20 Ultradian` rhythms.
- **Desktop Alerts**: Native Linux notifications (`notify-send`) and bell alerts when intervals complete.

### 🔄 2. Study Cycle Planner
- **Rotation-Based Learning**: Avoid burnout by rotating through subjects based on weight and target duration rather than rigid times of day.
- **Subject Queue Management**: Add, edit, reorder, and color-code subjects (e.g. Systems Programming, Algorithms, Mathematics).
- **Progress Tracking**: Individual subject progress bars plus an overall cycle completion gauge.
- **Cycle Rollover**: Automatically or manually advance through cycles (Cycle #1, Cycle #2, ...) while retaining all historical statistics.

### 📊 3. Frequency & Habit Tracker
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
git clone https://github.com/yourusername/ferndoro.git
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
