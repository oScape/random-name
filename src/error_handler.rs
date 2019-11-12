#[derive(Debug)]
pub enum MyError {
    GlibError(glib::Error),
    // DisplayError(std::fmt::Display),
}

impl From<glib::Error> for MyError {
    fn from(e: glib::Error) -> Self {
        MyError::GlibError(e)
    }
}

// impl From<std::fmt::Display> for MyError {
//     fn from(e: std::fmt::Display) -> Self {
//         MyError::DisplayError(e)
//     }
// }
