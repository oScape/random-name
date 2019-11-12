use crate::error_handler::MyError;
use crate::macos_debug;
use glib::object::Cast;
use std::sync::{Arc, Mutex};

fn stream(uri: String) -> Result<(), MyError> {
    gstreamer::init()?;

    let main_loop = glib::MainLoop::new(None, false);
    let dispatcher = gstreamer_player::PlayerGMainContextSignalDispatcher::new(None);
    let player = gstreamer_player::Player::new(
        None,
        Some(&dispatcher.upcast::<gstreamer_player::PlayerSignalDispatcher>()),
    );

    player.set_uri(&uri);

    let error = Arc::new(Mutex::new(Ok(())));

    let main_loop_clone = main_loop.clone();
    player.connect_end_of_stream(move |player| {
        let main_loop = &main_loop_clone;
        player.stop();
        main_loop.quit();
    });

    let main_loop_clone = main_loop.clone();
    let error_clone = Arc::clone(&error);
    player.connect_error(move |player, err| {
        let main_loop = &main_loop_clone;
        let error = &error_clone;

        *error.lock().unwrap() = Err(err.clone());

        player.stop();
        main_loop.quit();
    });

    player.play();
    main_loop.run();

    let guard = error.as_ref().lock().unwrap();
    guard.clone().map_err(|e| e.into())
}

#[allow(unused_variables)]
fn example_main(uri: String) {
    stream(uri).expect("an error occured");
}

pub fn launch(uri: String) {
    macos_debug::run(example_main, uri);
}
