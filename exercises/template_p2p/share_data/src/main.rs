use libp2p::{
    core::transport::upgrade,
    floodsub::{self, Floodsub, FloodsubEvent},
    futures::StreamExt,
    identity::{self, Keypair},
    mdns, noise,
    swarm::{NetworkBehaviour, Swarm, SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Transport,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::{fs, io::AsyncBufReadExt, sync::mpsc};

const STORAGE_FILE_PATH: &str = "./recipes.json";
static KEYS: Lazy<identity::Keypair> = Lazy::new(Keypair::generate_ed25519);
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from_public_key(&KEYS.public()));
static TOPIC: Lazy<floodsub::Topic> = Lazy::new(|| floodsub::Topic::new("recipes"));

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
type Recipes = Vec<Recipe>;

#[derive(Debug)]
enum EventType {
    Response(ListResponse),
    Input(String),
    Floodsub(FloodsubEvent),
    Mdns(mdns::Event),
}

#[derive(Debug, Deserialize, Serialize)]
struct Recipe {
    id: usize,
    name: String,
    ingredients: String,
    instructions: String,
    public: bool,
}

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "EventType")]
struct RecipeBehaviour {
    mdns: mdns::tokio::Behaviour,
    floodsub: Floodsub,
}

impl From<FloodsubEvent> for EventType {
    fn from(event: FloodsubEvent) -> Self {
        EventType::Floodsub(event)
    }
}

impl From<mdns::Event> for EventType {
    fn from(event: mdns::Event) -> Self {
        EventType::Mdns(event)
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum ListMode {
    All,
    One(String),
}

#[derive(Debug, Deserialize, Serialize)]
struct ListRequest {
    mode: ListMode,
}

#[derive(Debug, Deserialize, Serialize)]
struct ListResponse {
    mode: ListMode,
    data: Recipes,
    receiver: String,
}

#[tokio::main]
async fn main() -> MyResult<()> {
    pretty_env_logger::init();

    log::info!("Peer ID: {}", PEER_ID.clone());
    let (response_sender, mut response_rcv) = mpsc::unbounded_channel::<ListResponse>();
    let transport = tcp::tokio::Transport::default()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::Config::new(&KEYS).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    let mut behaviour = RecipeBehaviour {
        mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), *PEER_ID)?,
        floodsub: Floodsub::new(*PEER_ID),
    };
    behaviour.floodsub.subscribe(TOPIC.clone());

    let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, *PEER_ID).build();
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    loop {
        let evt = {
            tokio::select! {
                line = stdin.next_line() => Some(EventType::Input(line?.expect("can read line from stdin"))),
                response = response_rcv.recv() => Some(EventType::Response(response.expect("response exists"))),
                event = swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(event) => Some(event),
                    _ => {
                        // log::info!("unhandled swarm event: {:?}", event);
                        None
                    }
                }
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Response(response) => {
                    let json = serde_json::to_string(&response)?;
                    swarm
                        .behaviour_mut()
                        .floodsub
                        .publish(TOPIC.clone(), json.as_bytes());
                }
                EventType::Input(line) => match line.as_str() {
                    "ls p" => handle_list_peers(&mut swarm),
                    cmd if cmd.starts_with("ls r") => handle_list_recipes(cmd, &mut swarm).await,
                    cmd if cmd.starts_with("create r") => handle_create_recipe(cmd).await,
                    cmd if cmd.starts_with("publish r") => handle_publish_recipe(cmd).await,
                    _ => log::error!("unknown command"),
                },
                EventType::Floodsub(FloodsubEvent::Message(msg)) => {
                    if let Ok(resp) = serde_json::from_slice::<ListResponse>(&msg.data) {
                        if resp.receiver == PEER_ID.to_string() {
                            log::info!("Response from {}: ", msg.source);
                            resp.data.iter().for_each(|r| log::info!("{:#?}", r));
                        }
                    } else if let Ok(req) = serde_json::from_slice::<ListRequest>(&msg.data) {
                        match req.mode {
                            ListMode::All => {
                                log::info!("Received ALL request: {:?} from {:?}", req, msg.source);
                                respond_with_public_recipes(
                                    response_sender.clone(),
                                    msg.source.to_string(),
                                )
                                .await?;
                            }
                            ListMode::One(ref peer_id) => {
                                if peer_id == &PEER_ID.to_string() {
                                    log::info!(
                                        "Received ONE request: {:?} from {:?}",
                                        req,
                                        msg.source
                                    );
                                    respond_with_public_recipes(
                                        response_sender.clone(),
                                        msg.source.to_string(),
                                    )
                                    .await?;
                                }
                            }
                        }
                    }
                }
                EventType::Mdns(mdns::Event::Discovered(discovered_list)) => {
                    for (peer, _addr) in discovered_list {
                        swarm
                            .behaviour_mut()
                            .floodsub
                            .add_node_to_partial_view(peer);
                    }
                }
                EventType::Mdns(mdns::Event::Expired(expired_list)) => {
                    for (peer, _addr) in expired_list {
                        if !swarm.behaviour().mdns.has_node(&peer) {
                            swarm
                                .behaviour_mut()
                                .floodsub
                                .remove_node_from_partial_view(&peer);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn respond_with_public_recipes(
    sender: mpsc::UnboundedSender<ListResponse>,
    receiver: String,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        match read_local_recipes().await {
            Ok(recipes) => {
                let resp = ListResponse {
                    mode: ListMode::All,
                    receiver,
                    data: recipes.into_iter().filter(|r| r.public).collect(),
                };
                if let Err(e) = sender.send(resp) {
                    log::error!("error sending response via chanel: {}", e)
                }
            }
            Err(e) => log::error!("error fetching local recipes to answer ALL requests: {}", e),
        }
    })
}

async fn read_local_recipes() -> MyResult<Recipes> {
    let content = fs::read(&STORAGE_FILE_PATH).await?;
    let result = serde_json::from_slice(&content)?;
    Ok(result)
}

async fn write_local_recipes(recipes: &Recipes) -> MyResult<()> {
    let json = serde_json::to_string(&recipes)?;
    fs::write(STORAGE_FILE_PATH, &json).await?;
    Ok(())
}

async fn create_new_recipe(name: &str, ingredients: &str, instructions: &str) -> MyResult<()> {
    let mut local_recipes = read_local_recipes().await?;
    let new_id = match local_recipes.iter().max_by_key(|r| r.id) {
        Some(v) => v.id + 1,
        None => 0,
    };
    local_recipes.push(Recipe {
        id: new_id,
        name: name.to_owned(),
        ingredients: ingredients.to_owned(),
        instructions: instructions.to_owned(),
        public: false,
    });
    write_local_recipes(&local_recipes).await?;
    log::info!("Created recipe!");
    Ok(())
}

async fn publish_recipe(id: usize) -> MyResult<()> {
    let mut local_recipes = read_local_recipes().await?;
    local_recipes
        .iter_mut()
        .filter(|r| r.id == id)
        .for_each(|r| r.public = true);
    write_local_recipes(&local_recipes).await?;
    Ok(())
}

fn handle_list_peers(swarm: &mut Swarm<RecipeBehaviour>) {
    log::info!("Discovered Peers: ");
    let nodes = swarm.behaviour().mdns.discovered_nodes();
    let mut unique_peers = HashSet::new();
    for peer in nodes {
        unique_peers.insert(peer);
    }
    unique_peers.iter().for_each(|p| log::info!("{}", p));
}

async fn handle_list_recipes(cmd: &str, swarm: &mut Swarm<RecipeBehaviour>) {
    let rest = cmd.strip_prefix("ls r ");
    match rest {
        Some("all") => {
            let req = ListRequest {
                mode: ListMode::All,
            };
            let json = serde_json::to_string(&req).expect("can jsonify request");
            swarm
                .behaviour_mut()
                .floodsub
                .publish(TOPIC.clone(), json.as_bytes());
        }
        Some(recipes_peer_id) => {
            let req = ListRequest {
                mode: ListMode::One(recipes_peer_id.to_owned()),
            };
            let json = serde_json::to_string(&req).expect("can jsonify request");
            swarm
                .behaviour_mut()
                .floodsub
                .publish(TOPIC.clone(), json.as_bytes());
        }
        None => match read_local_recipes().await {
            Ok(v) => {
                log::info!("Local Recipes ({})", v.len());
                v.iter().for_each(|r| log::info!("{:#?}", r));
            }
            Err(e) => log::error!("error fetching local recipes: {}", e),
        },
    }
}

async fn handle_create_recipe(cmd: &str) {
    if let Some(rest) = cmd.strip_prefix("create r") {
        let elements: Vec<&str> = rest.split('|').collect();
        if elements.len() < 3 {
            log::error!("too few arguments => Format: name|ingredients|instructions");
        } else {
            let name = elements.first().expect("name is there");
            let ingredients = elements.get(1).expect("ingredients is there");
            let instructions = elements.get(2).expect("instructions is there");
            if let Err(e) = create_new_recipe(name, ingredients, instructions).await {
                log::error!("error creating recipe: {}", e);
            }
        }
    }
}

async fn handle_publish_recipe(cmd: &str) {
    if let Some(rest) = cmd.strip_prefix("publish r") {
        match rest.trim().parse::<usize>() {
            Ok(id) => {
                if let Err(e) = publish_recipe(id).await {
                    log::error!("error publishing recipe with id {}, {}", id, e);
                } else {
                    log::info!("Published Recipe with id: {}", id);
                }
            }
            Err(e) => log::error!("invalid id: {}, {}", rest.trim(), e),
        }
    }
}
