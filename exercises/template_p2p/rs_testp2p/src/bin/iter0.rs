use libp2p::{identity, PeerId};

#[tokio::main]
async fn main() {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peerid = PeerId::from(new_key.public());
    println!("New peer id: {:?}", new_peerid);
}

