///! # Topology Module
///!
///! Topology is how things are connected; the precise coordinates or "where things are"
///! are the [geometry module](../geometry/mod.rs)'s responsibility.

mod solid;
pub use solid::SolidKey;
mod face;
mod edge_loop;
mod edge;
mod half_edge;
mod vertex;

pub mod euler;
