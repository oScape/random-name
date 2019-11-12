mod network_connection;

#[derive(Debug)]
struct MyError {}

fn main() -> Result<(), MyError> {
    network_connection::setup_network();
    Ok(())
}
