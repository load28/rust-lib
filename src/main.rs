extern crate communicator;

use communicator::client;
use communicator::network::bar::foo::{foo_connect};

fn main() {
    client::connect();
    foo_connect();
}
