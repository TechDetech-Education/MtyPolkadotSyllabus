use crate::blocks::{Block, Chain};
use libp2p::{
    core::{
        muxing::StreamMuxerBox,
        transport::{self, Boxed},
    },
    gossipsub::{self, IdentTopic},
    identity, mdns, noise,
    swarm::{NetworkBehaviour, Swarm, SwarmBuilder},
    tcp, yamux, PeerId, Transport,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::Duration;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use tokio::sync::mpsc;

pub static KEYS: Lazy<identity::Keypair> = Lazy::new(identity::Keypair::generate_ed25519);
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
pub static CHAIN_TOPIC: Lazy<IdentTopic> = Lazy::new(|| IdentTopic::new("chains"));
pub static BLOCK_TOPIC: Lazy<IdentTopic> = Lazy::new(|| IdentTopic::new("blocks"));

pub struct AppTransport(Boxed<(PeerId, StreamMuxerBox)>);

impl Default for AppTransport {
    fn default() -> Self {
        let transport = tcp::tokio::Transport::default()
            .upgrade(transport::upgrade::Version::V1)
            .authenticate(noise::Config::new(&KEYS).unwrap())
            .multiplex(yamux::Config::default())
            .boxed();
        AppTransport(transport)
    }
}

pub struct ChainApp {
    pub swarm: Swarm<AppBehaviour>,
    pub chain: Chain,
    pub init_sender: mpsc::UnboundedSender<bool>,
    pub response_sender: mpsc::UnboundedSender<ChainResponse>,
}

impl ChainApp {
    pub fn new(
        init_sender: mpsc::UnboundedSender<bool>,
        response_sender: mpsc::UnboundedSender<ChainResponse>,
    ) -> Self {
        let AppTransport(transport) = AppTransport::default();

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

        let mut behaviour = AppBehaviour {
            mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), *PEER_ID)
                .expect("can create behaviour"),
            gossipsub: gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(KEYS.to_owned()),
                gossipsub_config,
            )
            .expect("correct configuration"),
        };
        let _ = behaviour
            .gossipsub
            .subscribe(&CHAIN_TOPIC)
            .expect("can subscribe");
        let _ = behaviour
            .gossipsub
            .subscribe(&BLOCK_TOPIC)
            .expect("can subscribe");

        let swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, *PEER_ID).build();

        Self {
            swarm,
            chain: Chain::default(),
            init_sender,
            response_sender,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainResponse {
    pub blocks: Vec<Block>,
    pub receiver: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalChainRequest {
    pub from_peer_id: String,
}

pub enum EventType {
    Response(ChainResponse),
    Input(String),
    Init,
    Gossipsub(Box<gossipsub::Event>),
    Mdns(mdns::Event),
}

impl From<gossipsub::Event> for EventType {
    fn from(event: gossipsub::Event) -> Self {
        EventType::Gossipsub(Box::new(event))
    }
}

impl From<mdns::Event> for EventType {
    fn from(event: mdns::Event) -> Self {
        EventType::Mdns(event)
    }
}

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "EventType")]
pub struct AppBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub fn get_list_peers(swarm: &Swarm<AppBehaviour>) -> HashSet<PeerId> {
    let nodes = swarm.behaviour().mdns.discovered_nodes();
    let mut unique_peers = HashSet::new();
    for peer in nodes {
        unique_peers.insert(*peer);
    }
    unique_peers
}

pub fn handle_print_peers(swarm: &Swarm<AppBehaviour>) {
    log::info!("Discovered Peers:");
    let unique_peers = get_list_peers(swarm);
    unique_peers
        .iter()
        .for_each(|p| log::info!("{}", p.to_string()))
}

pub fn handle_print_chain(chain: &Chain) {
    log::info!("Local Blockchain:");
    let pretty_json = serde_json::to_string_pretty(&chain.blocks).expect("can jsonify blocks");
    log::info!("{}", pretty_json);
}

pub fn handle_create_block(cmd: &str, chain_app: &mut ChainApp) {
    if let Some(data) = cmd.strip_prefix("create b") {
        let behaviour = chain_app.swarm.behaviour_mut();
        let Ok(latest_block) = chain_app.chain.add_data(String::from(data)) else {
            panic!("error creating block");
        };
        let json = serde_json::to_string(&latest_block).expect("can jsonify request");
        log::info!("broadcasting new block");
        if let Err(e) = behaviour
            .gossipsub
            .publish(BLOCK_TOPIC.clone(), json.as_bytes())
        {
            log::error!("can publish: {}", e);
        }
    }
}
