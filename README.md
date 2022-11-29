This repository serves the purpose of showcasing a broken interaction between
`libp2p 0.49.0` and `libp2p-swarm-derive` when using both of those dependencies
within a library. See https://github.com/libp2p/rust-libp2p/issues/3176 for context.

`cargo check` works for this project because that command considers the
Cargo.lock file. `libp2p-swarm-derive` is locked to `0.30.1` via Cargo.lock,
which is compatible with libp2p `0.49.0`.

`cargo package` does not work for this project because that command does not
consider the Cargo.lock file. Disregarding the lockfile causes `cargo` to use
`libp2p-swarm-derive 0.30.2` due to the `[dependencies.libp2p-swarm-derive] version = "0.30.1"` section of `libp2p 0.49.0` 's `Cargo.toml`. [`libp2p-swarm-derive 0.30.2` is supposed to be compatible with `libp2p 0.49.0`, but actually isn't](https://github.com/libp2p/rust-libp2p/issues/3176#issuecomment-1330006284); [a fix for that is underway](https://github.com/libp2p/rust-libp2p/pull/3178).

Here's the error message for `cargo package`:

```
error[E0433]: failed to resolve: could not find `derive_prelude` in `swarm`
 --> src/lib.rs:4:10
  |
4 | #[derive(NetworkBehaviour)]
  |          ^^^^^^^^^^^^^^^^ could not find `derive_prelude` in `swarm`
  |
  = note: this error originates in the derive macro `NetworkBehaviour` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0432]: unresolved import `libp2p`
 --> src/lib.rs:4:10
  |
4 | #[derive(NetworkBehaviour)]
  |          ^^^^^^^^^^^^^^^^ could not find `derive_prelude` in `swarm`
  |
  = note: this error originates in the derive macro `NetworkBehaviour` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0433]: failed to resolve: could not find `derive_prelude` in `swarm`
 --> src/lib.rs:4:10
  |
4 | #[derive(NetworkBehaviour)]
  |          ^^^^^^^^^^^^^^^^ not found in `libp2p::swarm::derive_prelude`
  |
  = note: this error originates in the derive macro `NetworkBehaviour` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this trait
  |
1 | use libp2p_swarm::NetworkBehaviour;
  |
help: if you import `NetworkBehaviour`, refer to it directly
  |
4 | #[derive(NetworkBehaviour)]
  |

error[E0433]: failed to resolve: could not find `derive_prelude` in `swarm`
 --> src/lib.rs:4:10
  |
4 | #[derive(NetworkBehaviour)]
  |          ^^^^^^^^^^^^^^^^ not found in `libp2p::swarm::derive_prelude`
  |
  = note: this error originates in the derive macro `NetworkBehaviour` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this enum
  |
1 | use libp2p_swarm::NetworkBehaviourAction;
  |
help: if you import `NetworkBehaviourAction`, refer to it directly
  |
4 | #[derive(NetworkBehaviour)]
  |
```
