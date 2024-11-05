# Card Deck Simulator

A Rust implementation of a card deck that allows shuffling and dealing cards.

## Features

- Create a standard deck of cards
- Shuffle the deck randomly
- Deal a specified number of cards
- Basic error handling (TODO)

## Installation

1. Make sure you have Rust installed on your system
2. Add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"
```

## Usage

```rust
// Create a new deck
let mut deck = Deck::new();

// Shuffle the deck
deck.shuffle();

// Deal 3 cards
let hand = deck.deal(3);
```

## Structure

The project consists of a `Deck` struct with the following methods:

- `new()`: Creates a new deck with all cards
- `shuffle()`: Randomly shuffles all cards in the deck
- `deal(num_cards)`: Deals the specified number of cards from the deck

## TODO

- [ ] Add proper error handling for dealing cards
- [ ] Implement full 52-card deck
- [ ] Add card validation
- [ ] Add game rules implementation

## License

This project is licensed under the MIT License - see the LICENSE file for details.
