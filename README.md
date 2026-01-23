# Dutch Auction

A Solana smart contract implementing a Dutch auction mechanism for token-to-token swaps. The price decreases linearly from a start price to an end price over a specified time period.

## Overview

This Anchor program enables sellers to create auctions where:
- The price starts at `start_price` and decreases linearly to `end_price`
- The auction runs from `start_time` to `end_time`
- Buyers can purchase tokens at the current price (which decreases over time)
- Sellers can cancel the auction before it ends

## Features

- **Linear Price Decay**: Price decreases linearly from start to end price over the auction duration
- **Token-to-Token Swaps**: Supports swapping any SPL token for any other SPL token
- **Time-Based Pricing**: Current price is calculated based on elapsed time
- **Seller Cancellation**: Sellers can cancel auctions and reclaim their tokens

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://www.anchor-lang.com/docs/installation)
- [Yarn](https://yarnpkg.com/getting-started/install) or npm

## Installation

```bash
# Install dependencies
yarn install

# Install Anchor dependencies (if needed)
anchor build
```

## Building

```bash
anchor build
```

## Testing

```bash
anchor test
# or
yarn test
```

## Program ID

```
BZSZmLcYCDUSjPmSfB8PdQoHbNJ9WEAFJcPhnbkCoQzT
```

## Instructions

### `init`

Creates a new Dutch auction.

**Parameters:**
- `start_price`: Starting price (in buy token units)
- `end_price`: Ending price (in buy token units)
- `start_time`: Unix timestamp when auction starts
- `end_time`: Unix timestamp when auction ends
- `sell_amount`: Amount of sell tokens to auction

### `buy`

Purchases tokens from the auction at the current price.

**Parameters:**
- `max_price`: Maximum price the buyer is willing to pay

### `cancel`

Cancels the auction and returns tokens to the seller. Only the seller can call this.

## License

ISC
