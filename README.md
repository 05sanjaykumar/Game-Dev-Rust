# Bevy Platformer – Mock Gameplay Prototype

This is a **mock gameplay prototype** built using [Bevy Engine](https://bevyengine.org/), written in Rust. The goal is to explore and test core platformer mechanics like movement, collision, and camera behavior in a structured and modular codebase.

> ⚠️ This project is not yet a complete game. It's a **testbed for experimenting** with fundamental 2D platforming features.

---

## ✅ Features Implemented (So Far)

- **Basic Player Movement**
  - Horizontal movement (`A` / `D` keys)
  - Jumping with spacebar
  - Gravity simulation
  - Variable jump height (better feel)

- **Polished Jump Physics**
  - Fast fall mechanic
  - Coyote time
  - Jump buffering

- **Platforms**
  - Static platforms
  - Moving platforms with customizable direction, speed, and range

- **Collision Detection**
  - Ground collision
  - Platform landing detection
  - Prevents player from jumping up through platforms

- **Camera System**
  - Smooth camera follow that tracks the player

- **Modular Architecture**
  - Player, platform, and camera logic separated into clean modules
  - Future-ready structure (particles, enemies, UI, etc.)

---

## 📁 Folder Structure Overview

```

src/
├── main.rs
├── components.rs
├── camera.rs
├── player/
│   ├── mod.rs
│   ├── movement.rs
│   └── spawn.rs
├── platform/
│   ├── mod.rs
│   ├── spawn.rs
│   ├── collision.rs
│   └── moving.rs

````

---

## 🚧 Work In Progress

- Neon-style particle effects for jumping and landing
- Scoring based on height (e.g., vertical progression)
- Better visual effects (glow, trails, etc.)
- Enemies, UI, sound effects (future roadmap)

---

## 🛠️ Run It

Make sure you have Rust and Bevy set up:

```sh
cargo run
````

---

## 💡 Note

This project is meant for **learning, rapid testing, and iteration.** The mechanics, systems, and codebase are evolving — feel free to explore, modify, and break things!

