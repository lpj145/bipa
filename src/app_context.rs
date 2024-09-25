// This module will shared context data to functions
// Like mongodb connection
#[derive(Clone)]
pub struct AppContext {}

impl AppContext {
    pub fn new() -> Self {
        AppContext {}
    }
}
