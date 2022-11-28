use libp2p_swarm_derive::NetworkBehaviour;
use libp2p_ping as ping;

#[derive(NetworkBehaviour)]
#[behaviour]
pub struct Behaviour {
  ping: ping::Behaviour,
}
