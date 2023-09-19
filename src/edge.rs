use crate::debug::debug_weak;
use crate::{def_next_id, impl_eq_hash, impl_weak_getter, Triangle, Vertex};
use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

def_next_id!();

#[derive(Clone)]
pub struct Edge {
    pub id: usize,

    a: Weak<RefCell<Vertex>>,
    b: Weak<RefCell<Vertex>>,
    pub triangles: Vec<Weak<RefCell<Triangle>>>,
}

impl Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = debug_weak!(self.a);
        let b = debug_weak!(self.b);

        let triangles = self
            .triangles
            .iter()
            .map(|t| debug_weak!(t))
            .collect::<Vec<_>>();

        f.debug_struct("Edge")
            .field("id", &self.id)
            .field("a", &a)
            .field("b", &b)
            .field("triangles", &triangles)
            .finish()
    }
}

impl Edge {
    pub fn new(a: Weak<RefCell<Vertex>>, b: Weak<RefCell<Vertex>>) -> Self {
        Self {
            id: next_id(),
            a,
            b,
            triangles: Vec::new(),
        }
    }

    impl_weak_getter!(a, Vertex);
    impl_weak_getter!(b, Vertex);

    pub fn add_triangle(&mut self, triangle: Weak<RefCell<Triangle>>) {
        self.triangles.push(triangle);
    }

    pub fn has(&self, vertex: &Vertex) -> bool {
        &*self.a.upgrade().expect("self.a is gone").borrow() == vertex
            || &*self.b.upgrade().expect("self.b is gone").borrow() == vertex
    }

    pub fn get_other_vertex(&self, vertex: &Vertex) -> Rc<RefCell<Vertex>> {
        let a = self.a.upgrade().expect("self.a is gone");
        let b = self.b.upgrade().expect("self.b is gone");
        if &*a.borrow() == vertex {
            b
        } else {
            a
        }
    }
}

impl_eq_hash!(Edge);
