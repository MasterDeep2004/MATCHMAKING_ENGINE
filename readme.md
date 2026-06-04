# 5v5 Game Matchmaking Engine

## Overview
This project implements a thread-safe, low-latency matchmaking server in Rust using `axum`. It accepts player requests, stores them, and matches players into teams of 5 based on their skill levels.

## Architecture & Approach
- REST API endpoints for adding players and fetching matches.
- `tokio` for asynchronous concurrency.
- `RwLock` for shared mutable state.
- Simple queue logic: first 10 players form a match.
- Future improvements include skill balancing, latency considerations, and time-based relaxation.

## Load Testing
- Used a Python script with `aiohttp` to inject 1000 concurrent players.
- The system maintains performance by using async primitives and minimal locking.

## Scaling & Challenges
- To handle larger scale, consider distributed queues, sharding, and more sophisticated matching algorithms.
- Latency optimizations include caching and network improvements.

## Future Work
- Implement skill-based ranking.
- Add latency and network conditions to matching criteria.
- Persist player data or integrate with real-time data streams.

---

## Final Notes
- Run the server: `cargo run`
- Run load test: `python load_test.py`
