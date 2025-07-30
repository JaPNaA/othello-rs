# Othello in Rust

This is a web application that allows you to play Othello. You can play against yourself or bots that use a variety of strategies.

## Rules of Othello

Othello is a 2-player (black vs white pieces) board game. The two players compete to get the most number of pieces on the board.

The board starts with two pieces of black and white placed in a cross.

```othello
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ o x _ _ _
_ _ _ x o _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
```

- The player playing **black makes the first move**.
- Players alternate plays (black, white, black, white, ...)
- Players must always make a play, unless they have no valid plays.
- The game ends with both players have no valid plays.

### Valid plays

A valid play is a play that sandwiches your opponent pieces between your pieces.

For example, the following are **valid** plays for black.

```othello
_ _ _ _ _
_ x o X _
_ _ _ _ _
```

---

Players can sandwich any number of opponent pieces.

```othello
_ _ _ _ _ _ _ _
_ o x x x x O _
_ _ _ _ _ _ _ _
```

```othello
_ _ _ _ _ _
_ X o o x _
_ _ _ _ _ _
```

```othello
_ _ _ _ _ _ _ _
_ x o x o o o X 
_ _ _ _ _ _ _ _
```

---

Vertical and diagonal sandwiches are valid.

```othello
_ _ _
_ x _
_ o _
_ o _
_ X _
_ _ _
```

```othello
_ _ _ _ _ _
_ X _ _ _ _
_ _ o _ _ _
_ _ _ o _ _
_ _ _ _ x _
_ _ _ _ _ _
```

---

Players can sandwich in multiple directions.

```othello
_ _ _ _ _ _
_ _ o x O _
_ _ _ x x _
_ _ x _ x _
_ o _ _ x _
_ _ _ _ o _
```

---

Note that the following is **not a valid play**.

White sandwiches black, but there is a hole in black's line, so white cannot play there.

```othello
_ _ _ _ _ _ _ _
_ o x _ x x O _
_ _ _ _ _ _ _ _
```

### Effect of play

After making a play, all opponent pieces the player sandwiches become the player's piece.

For example, if black plays, sandwiching one white piece...

```othello
_ _ _ _ _
_ x o X _
_ _ _ _ _
```

The white piece becomes black.

```othello
_ _ _ _ _
_ x X x _
_ _ _ _ _
```

---

After playing the highlighted move...

```othello
_ _ _ _ _ o _ _
_ _ _ _ x _ _ _
_ o x O x _ x o
_ _ x x _ _ _ _
_ x _ x _ _ _ _
o _ _ x _ _ _ _
_ _ _ o _ _ _ _
_ _ _ _ _ _ _ _
```

All sandwiched black pieces become white pieces.

```othello
_ _ _ _ _ o _ _
_ _ _ _ O _ _ _
_ o O o x _ x o
_ _ O O _ _ _ _
_ O _ O _ _ _ _
o _ _ O _ _ _ _
_ _ _ o _ _ _ _
_ _ _ _ _ _ _ _
```

Note the right horizontal sandwich has a hole in it, so those black pieces do not become white.

### Skips

The only time a player can skip is when the player has no valid moves.

For example:

```othello
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ x o o o o
_ _ _ o _ _ _ _
_ _ _ o _ _ _ _
_ _ _ o _ _ _ _
_ _ _ o _ _ _ _
_ _ _ o _ _ _ _
```

Black has no valid moves, so black must skip.

---

When both players have no valid moves, the game ends.

In the following example, black cannot move because there are no black pieces on the board to use to sandwich white. White also cannot move because there are no black pieces to sandwich.

```othello
_ _ _ _ _ _ _ _
_ _ _ o _ _ _ _
_ _ _ o o o o o
_ _ _ o _ _ _ _
_ _ _ o _ _ _ _
_ _ _ o _ _ _ _
_ _ _ o _ _ _ _
_ _ _ o _ _ _ _
```

### Game end

The game ends when there are no valid moves for both players. Most commonly, this is when the board fills up.

When the game ends, the **winner** is the player with the **most number of pieces** on the board.

If both players have the same number of pieces, the game ends in a tie.

# Bots

This app has several bots that employ various strategies.

The list is ordered from best strategies to worst.

- Deep Negative Heuristic Bot (Elo: -334)
  - This bot plays the worst moves it can find
  - This bot is the opposite of the Deep Heuristic Bot
- Center Bot (Elo: -243)
  - Tries to play move closest to the center of the board
- Bottom Right Bot (Elo: -52)
  - Opposite of the Top Left Bot, plays the last move in reading order
- Top Left Bot (Elo: -8)
  - This bot plays the first move it finds in reading order (highest first, then left-to-right).
- Random Bot (Elo: 0)
  - Plays a valid move randomly
- Shallow Score Bot (Elo: 67)
  - Plays the move that maximizes the number of pieces sandwiched.
- Edge Bot (Elo: 96)
  - Opposite of the Center Bot, tries to play moves closest to the edge of the board
- Edge Exclusive Bot (Elo: 128)
  - Plays corners if possible. If not, plays edges if possible. If not, plays a random move.
- Minmax Score Bot (Elo: 233)
  - Maximizes the number of pieces sandwiched, while minimizing the number of pieces that the opponent can sandwich.
- Deep Score Bot (Elo: 459)
  - Maximizes the score it can get, looking 5 moves ahead.
- Deep Heuristic Bot (Elo: 1040)
  - Maximizes a heuristic, looking 5 moves ahead. The heuristic is the score with additional points for corner and edges. The extra score for corners and edges decreases as the board is filled up.
