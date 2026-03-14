# @affectively/wasm-data-aggregator

`@affectively/wasm-data-aggregator` is a Rust/WebAssembly module for common statistics and grouping tasks.

The fair brag is that it covers a useful middle ground: more than one numeric helper, but still small enough to drop into a project as a focused utility.

## What It Helps You Do

- compute summary statistics
- group and aggregate records
- run simple time-series smoothing operations

## Installation

```bash
npm install @affectively/wasm-data-aggregator
```

## Quick Start

```ts
import init, {
  compute_stats,
  group_by,
  rolling_mean,
} from '@affectively/wasm-data-aggregator';

await init();

const stats = compute_stats(values);
const grouped = group_by(data, 'category', 'sum');
const smoothed = rolling_mean(timeSeries, 7);
```

## Why This README Is Grounded

This package does not need a larger analytics story. The strongest fair brag is that it already gives you a focused WASM module for common aggregation tasks.
