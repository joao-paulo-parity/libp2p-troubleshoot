This repository serves the purpose of showcasing a broken interaction between
`libp2p 0.49.0` and `libp2p-swarm-derive` when using both of those dependencies
within a library.

`cargo check` works for this project because that command considers the
Cargo.lock file. `libp2p-swarm-derive` is locked to `0.30.1` via Cargo.lock,
which is compatible with libp2p `0.49.0`.

`cargo package` does not work for this project because that command does not
consider the Cargo.lock file. Disregarding the lockfile causes `cargo` to use
`libp2p-swarm-derive 0.30.2`, which is incompatible with `libp2p 0.49.0`. Here's
the error message:

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

For further on this problem, see:

- https://github.com/rust-lang/cargo/issues/10733#issuecomment-1149279745
- https://doc.rust-lang.org/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries
