
/// Pages - mod.rs
/// ==============
/// These pages correspond to the routes defined in the Route module.
/// This file is like __init__.py in Python. It is the entry point for the module.
/// It makes the each page module available to the rest of the application.
pub mod home;
pub use home::Home;

pub mod error;
pub use error::Error;

pub mod students;
pub use students::Students;