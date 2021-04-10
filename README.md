# Fixed Physics

Made for Deterministic Lockstep, but specifically for Rollback Netcode in fighting games.

Uses [fixed](https://docs.rs/fixed/1.7.0/fixed/), [fixed_sqrt](https://crates.io/crates/fixed-sqrt), the rust std libary and more.



Based on how I imagine skullgirls collision works, as well as rivals of aether.



Right now, we are restricted to AABB 2d box style collision with no rotation, because that is the simplest to make. Once I'm able to prove that rollback works using this library and the Bevy engine, then I'll start adding in new features.