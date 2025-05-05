use monero_stratum::{login::Login, Request};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    run().await;
}

#[derive(Debug, Default)]
struct Miner {
    network_request_id: usize,
}

impl Miner {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_network_request_id(&mut self) -> usize {
        let old_id = self.network_request_id;
        self.network_request_id += 1;
        old_id
    }
}

async fn run() {
    let mut miner = Miner::new();

    let login = Login::new(miner.get_network_request_id(), "888tNkZrPN6JsEgekjMnABU4TBzc2Dt29EPAvkRxbANsAnjyPbb3iQ1YBRk1UXcdRsiKc9dhwMVgN5S9cQUiyoogDavup3H".to_string(), "xx".to_string());
    let response = Login::request("monerop.com".to_string(), 4242, login)
        .await
        .unwrap();

    log::info!("{:?}", response);
}
