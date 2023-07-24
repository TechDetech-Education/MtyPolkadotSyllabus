use libp2p::{identity, PeerId, swarm::{dummy, SwarmBuilder, SwarmEvent}, futures::StreamExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peerid = PeerId::from(new_key.public());
    println!("local peer id: {:?}", new_peerid);

    let behaviour = dummy::Behaviour;
    let transport = libp2p::tokio_development_transport(new_key)?;
    let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, new_peerid).build();
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        if let SwarmEvent::NewListenAddr { address, .. } = swarm.select_next_some().await {
                println!("Listing on local address {:?}", address)
        };
    }
}

