mod network_connection;
mod error_handler;
mod playback;
mod macos_debug;

use error_handler::MyError;

fn main() -> Result<(), MyError> {
    network_connection::setup_network();
    Ok(())
}
