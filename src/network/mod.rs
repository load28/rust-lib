pub mod server;
pub mod bar;

pub fn connect() {
    server::connect();
    bar::foo::foo_connect();
}

