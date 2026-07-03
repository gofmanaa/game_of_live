
# 🧬 Conway's Game of Life (Terminal Edition)

A small project built with **Rust** and **Ratatui** that brings Conway's famous cellular automaton to the terminal.

Sometimes it's fun to put aside large frameworks and build something with your own hands. A terminal application may seem simple, but it is a great place to explore algorithms, event loops, rendering, and architecture. Watching a tiny world evolve from a handful of cells is surprisingly satisfying.

## ✨ Features

- 🎲 Random world generation
- ⏯ Pause and resume simulation
- 👣 Step one generation at a time
- 🔄 Regenerate a new random world
- 🖥 Terminal UI powered by Ratatui
- ⚡ Fast and lightweight
- 🦀 Written in safe, idiomatic Rust

## 🎮 Controls

| Key | Action |
| :-- | :----- |
| `Space` | Pause / Resume |
| `N` | Advance one generation |
| `R` | Randomize the grid |
| `H` | Toggle help |
| `Q` | Quit |

## 🧠 About Conway's Game of Life

Every cell is either **alive** or **dead**.

For each generation:

- A live cell with **2 or 3** neighbors survives.
- A live cell with **fewer than 2** neighbors dies.
- A live cell with **more than 3** neighbors dies.
- A dead cell with **exactly 3** neighbors becomes alive.

From these four simple rules, surprisingly complex and beautiful patterns emerge.

## 🚀 Running

```bash
cargo run
````

## ❤️ Why build this?

Not every project needs to change the world.

Sometimes it's simply enjoyable to create something yourself—to understand how it works, improve it piece by piece, and watch it come to life.

This project was built for exactly that reason: curiosity, learning, and the joy of creating something with your own hands.

If it inspires you to experiment, improve it, or build something entirely new, then it has already done its job.

Happy coding! 🦀


