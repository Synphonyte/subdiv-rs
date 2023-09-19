use crate::debug::debug_weak;
use crate::{def_next_id, impl_eq_hash, impl_weak_getter, Edge, Vertex};
use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

def_next_id!();

#[derive(Clone)]
pub struct Triangle {
    pub id: usize,

    v0: Weak<RefCell<Vertex>>,
    v1: Weak<RefCell<Vertex>>,
    v2: Weak<RefCell<Vertex>>,

    e0: Weak<RefCell<Edge>>,
    e1: Weak<RefCell<Edge>>,
    e2: Weak<RefCell<Edge>>,
}

impl Debug for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v0 = debug_weak!(self.v0);
        let v1 = debug_weak!(self.v1);
        let v2 = debug_weak!(self.v2);

        let e0 = debug_weak!(self.e0);
        let e1 = debug_weak!(self.e1);
        let e2 = debug_weak!(self.e2);

        f.debug_struct("Triangle")
            .field("id", &self.id)
            .field("v0", &v0)
            .field("v1", &v1)
            .field("v2", &v2)
            .field("e0", &e0)
            .field("e1", &e1)
            .field("e2", &e2)
            .finish()
    }
}

impl Triangle {
    pub fn new(
        v0: Weak<RefCell<Vertex>>,
        v1: Weak<RefCell<Vertex>>,
        v2: Weak<RefCell<Vertex>>,
        e0: Weak<RefCell<Edge>>,
        e1: Weak<RefCell<Edge>>,
        e2: Weak<RefCell<Edge>>,
    ) -> Self {
        Self {
            id: next_id(),
            v0,
            v1,
            v2,
            e0,
            e1,
            e2,
        }
    }

    impl_weak_getter!(v0, Vertex);
    impl_weak_getter!(v1, Vertex);
    impl_weak_getter!(v2, Vertex);

    impl_weak_getter!(e0, Edge);
    impl_weak_getter!(e1, Edge);
    impl_weak_getter!(e2, Edge);

    pub fn get_opposite_vertex(&self, edge: &Edge) -> Rc<RefCell<Vertex>> {
        let v0 = self.v0.upgrade().expect("self.v0 is gone");
        let v1 = self.v1.upgrade().expect("self.v1 is gone");

        if !edge.has(&*v0.borrow()) {
            v0
        } else if !edge.has(&*v1.borrow()) {
            v1
        } else {
            self.v2.upgrade().expect("self.v2 is gone")
        }
    }
}

impl_eq_hash!(Triangle);
