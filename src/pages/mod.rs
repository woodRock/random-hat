
/// Pages - mod.rs
/// ==============
/// These pages correspond to the routes defined in the Route module.
/// This file is like __init__.py in Python. It is the entry point for the module.
/// It makes the each page module available to the rest of the application.
pub mod home;
pub use home::Home;

pub mod error;
<<<<<<< HEAD
pub use error::Error;

pub mod students;
pub use students::Students;
=======
pub use error::Error;
>>>>>>> 6345a0cba85677e4b142caa2251896a883202e9f
