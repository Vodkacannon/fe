/// This is the core of the game engine.
pub mod engine {
    use crate::renderer::Renderer;

    pub struct Engine {
        /// Whether the game engine is running.
        pub is_shutdown: bool,
        pub renderer: Renderer,
    }
}
