/// Nova 10.0 Core Library
/// Reusable components for mesh, hyperspace, self-improving engine, and swarm coordination.

pub mod mesh;
pub mod hyperspace;
pub mod self_improving;
pub mod hardware;
pub mod swarm;

/// Re-export main types
pub use mesh::MeshNode;
pub use hyperspace::HyperspaceManager;

/// Current version of the Nova 10.0 core
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
