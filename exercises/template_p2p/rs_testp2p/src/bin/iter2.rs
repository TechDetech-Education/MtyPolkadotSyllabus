use libp2p::{swarm::{SwarmEvent, SwarmBuilder, NetworkBehaviour, keep_alive}, futures::StreamExt, identity, Multiaddr, PeerId, ping};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peerid = PeerId::from(new_key.public());
    println!("local peer id is: {:?}", new_peerid);

    let transport = libp2p::tokio_development_transport(new_key)?;
    // let transport = tcp::tokio::Transport::new(tcp::Config::new(&id_keys));
    let behaviour = Behaviour::default();
    let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, new_peerid).build();
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(remote_peer) = std::env::args().nth(1) {
        let remote_peer_multiaddr: Multiaddr = remote_peer.parse()?;
        swarm.dial(remote_peer_multiaddr)?;
        println!("Dialed remote peer: {:?}", remote_peer);
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("listening on local address {:?}", address);
            }
            SwarmEvent::Behaviour(event) => println!("Event received: {:?}", event),
            _ => {}
        } 
    }
}

#[derive(NetworkBehaviour, Default)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}
