use libp2p::{
    futures::StreamExt,
    identity,
    swarm::{DummyBehaviour, Swarm, SwarmEvent},
    PeerId,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());
    println!("Local Peer ID is: {:?}", new_peer_id);

    let behaviour = DummyBehaviour::default();
    let transport = libp2p::development_transport(new_key).await?;
    let mut swarm = Swarm::new(transport, behaviour, new_peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on Local address {:?}", address);
            }
            _ => {}
        }
    }

    // Ok(())
}
