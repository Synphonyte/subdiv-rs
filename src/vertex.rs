use crate::debug::debug_weak;
use crate::{def_next_id, impl_eq_hash, Edge, Triangle};
use glam::Vec3;
use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Weak;

def_next_id!();

#[derive(Clone)]
pub struct Vertex {
    pub id: usize,

    pub position: Vec3,
    pub edges: Vec<Weak<RefCell<Edge>>>,
    pub triangles: Vec<Weak<RefCell<Triangle>>>,

    // reference index to original vertex
    pub index: Option<usize>,
}

impl Debug for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let edges = self
            .edges
            .iter()
            .map(|e| debug_weak!(e))
            .collect::<Vec<_>>();

        let triangles = self
            .triangles
            .iter()
            .map(|t| debug_weak!(t))
            .collect::<Vec<_>>();

        f.debug_struct("Vertex")
            .field("id", &self.id)
            .field("position", &self.position)
            .field("edges", &edges)
            .field("triangles", &triangles)
            .field("index", &self.index)
            .finish()
    }
}

impl Vertex {
    pub fn new(position: Vec3) -> Self {
        Self {
            id: next_id(),
            position,
            edges: Vec::new(),
            triangles: Vec::new(),
            index: None,
        }
    }

    pub fn with_index(position: Vec3, index: Option<usize>) -> Self {
        Self {
            id: next_id(),
            position,
            edges: Vec::new(),
            triangles: Vec::new(),
            index,
        }
    }

    pub fn add_edge(&mut self, edge: Weak<RefCell<Edge>>) {
        self.edges.push(edge);
    }

    pub fn add_triangle(&mut self, triangle: Weak<RefCell<Triangle>>) {
        self.triangles.push(triangle);
    }
}

impl_eq_hash!(Vertex);
