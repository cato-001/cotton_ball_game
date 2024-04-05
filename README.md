# Cotton Ball Game

This project can be used to simulate the cotton ball game from this video:

![](https://www.youtube.com/watch?v=LUCvSsx6-EU)

## The Rules

- You have 2 cotton balls 
- 1 is blue, 1 is red
- You draw a ball
- If it is blue the game is finished
- Else add a red ball and try again

This program plays the game an averages the total rounds.

## Usage

1. Install the rustup toolchain.
2. Clone this repository.
3. Build and Run the executable in release mode:

    cargo run --release

    // run custom number of games
    cargo run --release -- --count 1000000

