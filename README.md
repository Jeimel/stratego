# stratego

Implementation of Stratego Quick Battle for bachelor thesis.

## Usage

1. Install rust
2. To play against the agent use:

```shell
git clone https://github.com/Jeimel/stratego.git
cargo run -r --no-default-features --bin human
```

Note: To run code besides the human binary, PyTorch must be installed in order to use [tch-rs](https://github.com/LaurentMazare/tch-rs).

### Deployment

The human binary allows to make generate custom deployments.

The deployment must be from the point of view of the chosen side. Red starts from a1, going from left to right and upwards. Blue starts from a8, going from left to right and downwards. The symbols for each piece are displayed during execution, red pieces are denoted using upper-case symbols. In the notation, numbers denote the empty squares between pieces, which is equal to the FEN-Notation. Two example positions for the two sides are given during execution.

Note: There is no formal validation of the input.

### Notation

Each move is denoted using the starting and target square. If the move captures a piece, the rank is denoted as suffix with a dividing `x`. If the last move was a capture, the rank of the attcked piece is denoted as prefix with a dividing `x`
