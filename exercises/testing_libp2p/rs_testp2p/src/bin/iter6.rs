use async_std::io;
use futures::{select, StreamExt, AsyncBufReadExt};
use futures::FutureExt;
use libp2p::{
    core::upgrade,
    gossipsub, identity, mdns, noise,
    swarm::NetworkBehaviour,
    swarm::{Swarm, SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Transport
};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use rustyline::{history::FileHistory, Editor};
use rustyline::error::ReadlineError;
use thiserror;
use anyhow::Result;
use std::sync::{Arc, Mutex};



#[derive(Debug, thiserror::Error)]
enum ChatError {
    #[error("Input error: {0}")]
    InputError(#[from] ReadlineError)
}

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a random PeerId
    let id_keys = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {local_peer_id}");

    // Set up an encrypted DNS-enabled TCP Transport over the yamux protocol.
    let tcp_transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
        .upgrade(upgrade::Version::V1Lazy)
        .authenticate(noise::Config::new(&id_keys).expect("signing libp2p-noise static keypair"))
        .multiplex(yamux::Config::default())
        .timeout(std::time::Duration::from_secs(20))
        .boxed();

    // To content-address message, we can take the hash of message and use it as an ID.
    let message_id_fn = |message: &gossipsub::Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
    };

    // Set a custom gossipsub configuration
    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
        .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
        .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
        .build()
        .expect("Valid config");

    // build a gossipsub network behaviour
    let mut gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(id_keys),
        gossipsub_config,
    ).expect("Correct configuration");

    // Create a Gossipsub topic
    let topic = gossipsub::IdentTopic::new("test-net");
    // subscribes to our topic
    gossipsub.subscribe(&topic)?;

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)?;
        let behaviour = MyBehaviour { gossipsub, mdns };
        SwarmBuilder::with_tokio_executor(tcp_transport, behaviour, local_peer_id).build()
    };

    // Read full lines from stdin
    //let mut stdin = tokio::io::BufReader::new(tokio::io::stdin().read_line());
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    println!("Enter messages via STDIN and they will be sent to connected peers using Gossipsub");
    let mut rl = Editor::<(), FileHistory>::new()?;
    let swarm = Arc::new(Mutex::new(swarm));
    // Kick it off
    loop {
        select! {
            _ = handle_readline_event(Arc::clone(&swarm), &mut rl, local_peer_id.clone(), topic.clone()).fuse() => {},
            _ = handle_libp2p_event(Arc::clone(&swarm)).fuse() => {}
        }
    }
}

async fn handle_libp2p_event(arc_swarm: Arc<Mutex<Swarm<MyBehaviour>>>) -> Result<()> {
    let Ok(mut swarm) = arc_swarm.lock() else {
        unreachable!()
    };
    let event = swarm.select_next_some().await;

    match event {
        SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
            for (peer_id, _multiaddr) in list {
                println!("mDNS discovered a new peer: {peer_id}");
                swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
            }
        },
        SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
            for (peer_id, _multiaddr) in list {
                println!("mDNS discover peer has expired: {peer_id}");
                swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
            }
        },
        SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
            propagation_source: peer_id,
            message_id: id,
            message,
        })) => println!(
                "{peer_id} => id {id}\n\t{}",
                String::from_utf8_lossy(&message.data),
            ),
        SwarmEvent::NewListenAddr { address, .. } => {
            println!("Local node is listening on {address}");
        }
        _ => {}
    }
    Ok(())
}

async fn handle_readline_event(
    swarm: Arc<Mutex<Swarm<MyBehaviour>>>, 
    rl: &mut Editor<(), FileHistory>, 
    local_peer_id: PeerId, 
    topic: gossipsub::IdentTopic
) -> Result<()> {
    let readline = rl.readline(format!("{} >> ", local_peer_id).as_str());
    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str());
            if let Err(e) = swarm.lock().unwrap()
                .behaviour_mut().gossipsub
                .publish(topic.clone(), line.as_bytes()) {
                println!("Publish error: {:?}", e);
            }
        },
        Err(err) => {
            return Err(ChatError::InputError(err).into());
        }
    }
    Ok(())
}
