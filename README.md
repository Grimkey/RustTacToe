# RustTacToe
A simple tictactoe game in Rust

This repo is just to help me learn rust. I attempted to write a tic tac toe game with very little mutability. It should be clippy clean.

## Game

When you start the time, you'll be presented with a board looking like this:

```
-------------
| 1 | 2 | 3 |
|---|---|---|
| 4 | 5 | 6 |
|---|---|---|
| 7 | 8 | 9 |
-------------

Player X turn.
```
Alternating between players X and O, you select an unused number between 1 and 9. Error handling should prevent any incorrect input from getting through and the game should indicate as soon as someone has won. This is player-verus-player so no computer auto-play has been enabled.
