//! The tab's connection to the translation server.

use crate::{
    screen::Translation,
    server::{Server, ServerUpdate},
};

impl ServerUpdate for Translation {
    fn server_mut(&mut self) -> &mut Server {
        &mut self.server
    }
}
