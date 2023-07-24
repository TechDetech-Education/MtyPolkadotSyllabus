use blockchain::{
    blocks::{Block, Chain},
    p2p::{self, ChainApp, ChainResponse, EventType, LocalChainRequest},
    Result,
};
use libp2p::{futures::StreamExt, gossipsub, mdns, swarm::SwarmEvent};
use std::time::Duration;
use tokio::{io::AsyncBufReadExt, sync::mpsc, time};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("Peer Id: {}", p2p::PEER_ID.clone());
    let (response_sender, mut response_rcv) = mpsc::unbounded_channel::<ChainResponse>();
    let (init_sender, mut init_rcv) = mpsc::unbounded_channel::<bool>();

    let mut chain_app = ChainApp::new(init_sender.clone(), response_sender);
    chain_app.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    tokio::spawn(async move {
        time::sleep(Duration::from_secs(1)).await;
        log::info!("sending init event");
        init_sender.send(true).expect("can send init event");
    });

    loop {
        let evt = tokio::select! {
            line = stdin.next_line() => Some(EventType::Input(line?.expect("can read line from stdin"))),
            response = response_rcv.recv() => Some(EventType::Response(response.expect("response exists"))),
            _init = init_rcv.recv() => Some(EventType::Init),
            event = chain_app.swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(event) => Some(event),
                _ => {
                    // log::info!("unhandled swarm event: {:?}", event);
                    None
                }
            }

        };
        if let Some(event) = evt {
            match event {
                EventType::Init => {
                    let peers = p2p::get_list_peers(&chain_app.swarm);
                    log::info!("connected nodes: {}", peers.len());
                    if !peers.is_empty() {
                        let req = p2p::LocalChainRequest {
                            from_peer_id: peers.iter().last().unwrap().to_string(),
                        };

                        let json = serde_json::to_string(&req)?;
                        chain_app
                            .swarm
                            .behaviour_mut()
                            .gossipsub
                            .publish(p2p::CHAIN_TOPIC.clone(), json.as_bytes())?;
                    }
                }
                EventType::Response(resp) => {
                    let json = serde_json::to_string(&resp)?;
                    chain_app
                        .swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(p2p::CHAIN_TOPIC.clone(), json.as_bytes())?;
                }
                EventType::Input(line) => match line.as_str() {
                    "ls p" => p2p::handle_print_peers(&chain_app.swarm),
                    cmd if cmd.starts_with("ls c") => p2p::handle_print_chain(&chain_app.chain),
                    cmd if cmd.starts_with("create b") => {
                        p2p::handle_create_block(cmd, &mut chain_app)
                    }
                    _ => log::error!("unknown command"),
                },
                EventType::Gossipsub(boxed_event) => {
                    let gossipsub::Event::Message {
                        propagation_source,
                        message_id: _id,
                        message: msg
                    } = *boxed_event else {
                        continue;
                    };
                    let peer_id = msg.source.unwrap_or(propagation_source);
                    if let Ok(resp) = serde_json::from_slice::<ChainResponse>(&msg.data) {
                        if resp.receiver == p2p::PEER_ID.to_string() {
                            log::info!("Response from: {}", peer_id);
                            resp.blocks.iter().for_each(|r| log::info!("{:#?}", r));
                            chain_app.chain.choose_chain(&Chain {
                                blocks: resp.blocks,
                            });
                        }
                    } else if let Ok(resp) = serde_json::from_slice::<LocalChainRequest>(&msg.data)
                    {
                        log::info!("sending local chain to {}", peer_id.to_string());
                        let peer_id = resp.from_peer_id;
                        if p2p::PEER_ID.to_string() == peer_id {
                            if let Err(e) = chain_app.response_sender.send(ChainResponse {
                                blocks: chain_app.chain.blocks.clone(),
                                receiver: peer_id.to_string(),
                            }) {
                                log::error!("error sending response via channel: {}", e);
                            }
                        }
                    } else if let Ok(block) = serde_json::from_slice::<Block>(&msg.data) {
                        log::info!("received new block from {}", peer_id.to_string());
                        chain_app.chain.try_add_block(block);
                    } else {
                        log::error!(
                            "couldn't deserialize msg: {:?} from: {}",
                            std::str::from_utf8(&msg.data),
                            peer_id.to_string()
                        );
                    }
                }
                EventType::Mdns(mdns::Event::Discovered(discovered_list)) => {
                    for (peer, _addr) in discovered_list {
                        chain_app
                            .swarm
                            .behaviour_mut()
                            .gossipsub
                            .add_explicit_peer(&peer);
                    }
                }
                EventType::Mdns(mdns::Event::Expired(expired_list)) => {
                    for (peer, _addr) in expired_list {
                        if !chain_app.swarm.behaviour().mdns.has_node(&peer) {
                            chain_app
                                .swarm
                                .behaviour_mut()
                                .gossipsub
                                .remove_explicit_peer(&peer);
                        }
                    }
                }
            }
        }
    }
}
