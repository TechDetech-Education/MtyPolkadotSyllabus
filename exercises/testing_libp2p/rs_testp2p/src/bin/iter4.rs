use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    futures::StreamExt,
    identity,
    mdns,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    PeerId, Multiaddr
};
use rand::seq::SliceRandom;
use std::error::Error;

#[derive(NetworkBehaviour)]
struct MyBehaviour {
    mdns: mdns::tokio::Behaviour,
    floodsub: floodsub::Floodsub
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    let my_floodsub = Floodsub::new(peer_id);
    let mut my_swarm = {
        let transport = libp2p::tokio_development_transport(id_keys)?;
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id)?;
        let behaviour = MyBehaviour { mdns, floodsub: my_floodsub };
        SwarmBuilder::with_tokio_executor(transport, behaviour, peer_id).build()
    };
    my_swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let mut discovered_peers = Vec::<(PeerId, Multiaddr)>::new();

    loop {
        match my_swarm.select_next_some().await {
            SwarmEvent::Behaviour(MyBehaviourEvent::Floodsub(FloodsubEvent::Message(message))) => {
                let sender = message.source;
                let msg = String::from_utf8_lossy(&message.data);
                println!("Received message from {:?}: {}", sender, msg);
            }
            SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                for (peer, addr) in peers {
                    discovered_peers.push((peer, addr));
                }
                if !discovered_peers.is_empty() {
                    // Choose a random peer from the list
                    let (peer, addr) = discovered_peers.choose(&mut rand::thread_rng()).unwrap();
                    let idx = discovered_peers.iter().position(|&(ref p, _)| p == peer).unwrap();
                    let message = format!("I'm {:?} and you are the No.{} in my list", peer_id, idx + 1);
                }
            }
            _ => {}
        }
    }
}
