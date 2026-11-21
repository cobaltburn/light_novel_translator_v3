//! The glossary's connection to the server.

use crate::{
    screen::Glossary,
    server::{Server, ServerUpdate},
};

impl ServerUpdate for Glossary {
    fn server_mut(&mut self) -> &mut Server {
        &mut self.server
    }
}
