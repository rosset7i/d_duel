# d_duel

A deterministic, turn-based duel engine written in Rust.

---

## Current state

The project is currently a **duel-focused MVP engine skeleton** with:

- deterministic RNG seeded at game creation
- AP-based turn flow
- grid/map-based movement
- basic melee-style attacks with range checks
- state hashing for determinism checks
- simple simulation-ready architecture

This is not a full game.

---

## Design goals

The engine is built around a few strict goals:

- **Deterministic simulation**  
  Given the same seed and the same sequence of actions, the engine should always produce the same result.

- **Clean simulation pipeline**  
  Player/AI intent is separated from world mutation:
  - `Action`: external intent
  - `Event`: resolved facts
  - `resolve()`: state mutation
