// src/lib.rs
pub mod animate_atlas;
pub mod sparrow_atlas;

// Re-export core data structures for clean root access
pub use animate_atlas::{AnimateAtlas, build_part_mesh, get_animate_parts, parse_animate_atlas};
pub use sparrow_atlas::{SparrowFrame, parse_sparrow};
