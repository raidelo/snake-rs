# snake-rs

A terminal Snake game written in Rust, featuring an async architecture, a lives and invincibility system, and multiple visual themes.

<!-- Replace the line below with your actual demo gif/video -->
<!-- ![demo](assets/demo.gif) -->

## Features

- **Async game loop** — input and rendering run independently via Tokio and `crossterm::EventStream`, keeping the game responsive at all times
- **Lives & invincibility** — the snake has multiple lives and becomes temporarily invincible after each impact, with a visual blink effect
- **11 built-in themes** — Classic, Neon V1, Neon V2, Organic V1, Organic V2, Dracula, Ocean, Matrix, Sunset, Nord, Retro CGA
- **Live theme preview** — switch themes from the start menu or mid-game pause menu and see changes instantly
- **Pause menu** — pause at any time, change theme, then resume without losing state
- **Terminal-safe** — restores the terminal on clean exit, errors, and panics

## Requirements

- Rust 1.85 or later (edition 2024)
- A terminal with true color support (most modern terminals qualify)

## Installation

```bash
git clone https://github.com/raidelo/snake-rs
cd snake-rs
cargo run --release
```

## Controls

| Key       | Action              |
| --------- | ------------------- |
| `↑` `W`   | Move up             |
| `↓` `S`   | Move down           |
| `←` `A`   | Move left           |
| `→` `D`   | Move right          |
| `Enter`   | Confirm / Start     |
| `P`       | Open theme selector |
| `Esc` `Q` | Pause / Quit        |

## Themes

| Name       | Description                                    |
| ---------- | ---------------------------------------------- |
| Classic    | Game Boy green palette                         |
| Neon V1    | Dark background with electric cyan and magenta |
| Neon V2    | Pure black with cyan and yellow-green          |
| Organic V1 | Dark earth tones with lime accent              |
| Organic V2 | Gruvbox-inspired warm greens                   |
| Dracula    | The popular Dracula color scheme               |
| Ocean      | Deep navy with cyan and white                  |
| Matrix     | Black and green, terminal classic              |
| Sunset     | Dark amber with orange and gold                |
| Nord       | Arctic blue-grey palette                       |
| Retro CGA  | Classic CGA 4-color palette                    |

## Project structure

```
src/
├── main.rs          # Entry point, panic hook, keyboard listener task
├── app.rs           # Game loop, run loop, screen transitions
├── menu.rs          # Reusable centered menu renderer
├── constants.rs     # Game tuning values and glyphs
├── errors.rs        # AppError type
├── helpers.rs       # Terminal setup/reset
├── screens/
│   ├── start.rs     # Start screen
│   ├── pause.rs     # Pause screen
│   ├── game_over.rs # Game over screen
│   └── palette.rs   # Interactive theme selector
└── types/
    ├── window.rs    # Window, border rendering, bounds checking
    ├── snake.rs     # Snake, movement, lives, invincibility, blink
    ├── fruit.rs     # Fruit, random placement
    ├── palette.rs   # Theme and palette definitions (all const)
    ├── square.rs    # Renderable terminal cell
    ├── snake_part.rs
    ├── axes.rs      # 2D position
    ├── direction.rs
    ├── helpers.rs   # next_position, check_impact, random_free_position
    └── render.rs    # Render trait
```

## Dependencies

| Crate                                                    | Purpose                          |
| -------------------------------------------------------- | -------------------------------- |
| [`crossterm`](https://github.com/crossterm-rs/crossterm) | Cross-platform terminal control  |
| [`tokio`](https://tokio.rs)                              | Async runtime                    |
| [`futures`](https://docs.rs/futures)                     | `StreamExt` for the event stream |
| [`rand`](https://docs.rs/rand)                           | Random fruit placement           |

## License

MIT
