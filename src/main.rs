use futures::{future, prelude::*};
use libp2p::{
    identity,
    ping::{Ping, PingConfig},
    PeerId, Swarm,
};
use std::env;

fn main() {
    let id_keys = identity::Keypair::generate_ed25519();

    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    let transports = libp2p::build_development_transport(id_keys);

    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transports, behaviour, peer_id);

    if let Some(addr) = env::args().nth(1) {
        let remote_addr = addr.clone();
        match addr.parse() {
            Ok(remote) => match Swarm::dial_addr(&mut swarm, remote) {
                Ok(()) => println!("Dialed {:?}", remote_addr),
                Err(e) => println!("Dialing {:?}, failed with {:?}", remote_addr, e),
            },
            Err(err) => println!("Failed to parse address to dial {:?}", err),
        }
    }

    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    let mut listening = false;
    tokio::run(future::poll_fn(move || -> Result<_, ()> {
        loop {
            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(e)) => println!("{:?}", e),
                Async::Ready(None) | Async::NotReady => {
                    if !listening {
                        if let Some(a) = Swarm::listeners(&swarm).next() {
                            println!("Listening on {:?}", a);
                            listening = true;
                        }
                    }
                    return Ok(Async::NotReady);
                }
            }
        }
    }));
}
