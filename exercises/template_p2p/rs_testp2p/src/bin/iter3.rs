use libp2p::{futures::StreamExt, identity, mdns, swarm::{SwarmEvent, SwarmBuilder}, PeerId};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    let transport = libp2p::tokio_development_transport(id_keys)?;
    let behaviour = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id)?;
    let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, peer_id).build();
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr{ address, .. } => {
                println!("Listening on local address {:?}", address);
            }
            SwarmEvent::Behaviour(mdns::Event::Discovered(peers)) => {
                for (peer, addr) in peers {
                    println!("discovered {} {}", peer, addr);
                }
            }
            SwarmEvent::Behaviour(mdns::Event::Expired(expired)) => {
                for (peer, addr) in expired {
                    println!("expired {} {}", peer, addr);
                }
            }
            _ => {}
        }
    }
}
