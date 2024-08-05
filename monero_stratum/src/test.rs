use crate::login::Login;
use crate::Request;

const POOL_HOSTNAME: &str = "monerop.com";
const POOL_PASSWORD: &str = "xx";
const POOL_PORT: u16 = 4242;
const WALLET: &str =  "888tNkZrPN6JsEgekjMnABU4TBzc2Dt29EPAvkRxbANsAnjyPbb3iQ1YBRk1UXcdRsiKc9dhwMVgN5S9cQUiyoogDavup3H";

#[tokio::test]
async fn connect_to_pool() {
    pretty_env_logger::init();

    let login = Login::new(1, WALLET.to_string(), POOL_PASSWORD.to_string());
    Login::request(POOL_HOSTNAME.to_string(), POOL_PORT, login)
        .await
        .expect("Response from pool failed");
}
