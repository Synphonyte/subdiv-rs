use crate::{subdivide_once, Edge, Triangle, Vertex};
use hashbrown::HashMap;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone)]
pub struct Model {
    pub vertices: HashMap<usize, Rc<RefCell<Vertex>>>,
    pub edges: Vec<Rc<RefCell<Edge>>>,
    pub triangles: Vec<Rc<RefCell<Triangle>>>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: Vec::new(),
            triangles: Vec::new(),
        }
    }
}

impl Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let verts: Vec<_> = self.vertices.values().map(|v| v.borrow().clone()).collect();
        let edges: Vec<_> = self.edges.iter().map(|e| e.borrow().clone()).collect();
        let triangles: Vec<_> = self.triangles.iter().map(|t| t.borrow().clone()).collect();

        f.debug_struct("Model")
            .field("vertices", &verts)
            .field("edges", &edges)
            .field("triangles", &triangles)
            .finish()
    }
}

impl Model {
    pub fn add_triangle(
        &mut self,
        v0: Rc<RefCell<Vertex>>,
        v1: Rc<RefCell<Vertex>>,
        v2: Rc<RefCell<Vertex>>,
    ) {
        self.vertices.insert(v0.borrow().id, Rc::clone(&v0));
        self.vertices.insert(v1.borrow().id, Rc::clone(&v1));
        self.vertices.insert(v2.borrow().id, Rc::clone(&v2));

        let edge0 = self.get_or_create_edge(Rc::clone(&v0), Rc::clone(&v1));
        let edge1 = self.get_or_create_edge(Rc::clone(&v1), Rc::clone(&v2));
        let edge2 = self.get_or_create_edge(Rc::clone(&v2), Rc::clone(&v0));

        let triangle = Rc::new(RefCell::new(Triangle::new(
            Rc::downgrade(&v0),
            Rc::downgrade(&v1),
            Rc::downgrade(&v2),
            Rc::downgrade(&edge0),
            Rc::downgrade(&edge1),
            Rc::downgrade(&edge2),
        )));

        self.triangles.push(Rc::clone(&triangle));

        v0.borrow_mut().add_triangle(Rc::downgrade(&triangle));
        v1.borrow_mut().add_triangle(Rc::downgrade(&triangle));
        v2.borrow_mut().add_triangle(Rc::downgrade(&triangle));

        edge0.borrow_mut().add_triangle(Rc::downgrade(&triangle));
        edge1.borrow_mut().add_triangle(Rc::downgrade(&triangle));
        edge2.borrow_mut().add_triangle(Rc::downgrade(&triangle));
    }

    pub fn get_or_create_edge(
        &mut self,
        v0: Rc<RefCell<Vertex>>,
        v1: Rc<RefCell<Vertex>>,
    ) -> Rc<RefCell<Edge>> {
        if let Some(edge) = v0.borrow().edges.iter().find(|e| {
            e.upgrade().is_some_and(|e| {
                let e = e.borrow();
                e.a() == v1 || e.b() == v1
            })
        }) {
            return edge.upgrade().expect("edge is gone");
        }

        let new_edge = Rc::new(RefCell::new(Edge::new(
            Rc::downgrade(&v0),
            Rc::downgrade(&v1),
        )));

        self.edges.push(Rc::clone(&new_edge));
        v0.borrow_mut().add_edge(Rc::downgrade(&new_edge));
        v1.borrow_mut().add_edge(Rc::downgrade(&new_edge));

        new_edge
    }

    pub fn subdivide(self, iterations: usize) -> Self {
        let mut model = self;
        for _ in 0..iterations {
            model = subdivide_once(model);
        }

        model
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    #[test]
    fn test_subdivide() {
        const SCALE: f32 = 10.0;

        let mut model = Model::default();

        let v0 = Rc::new(RefCell::new(Vertex::new(
            Vec3::new(-1.0, -1.0, 0.0) * SCALE,
        )));
        let v1 = Rc::new(RefCell::new(Vertex::new(Vec3::new(1.0, -1.0, 0.0) * SCALE)));
        let v2 = Rc::new(RefCell::new(Vertex::new(Vec3::new(1.0, 1.0, 0.0) * SCALE)));
        let v3 = Rc::new(RefCell::new(Vertex::new(Vec3::new(-1.0, 1.0, 0.0) * SCALE)));

        model.add_triangle(Rc::clone(&v0), Rc::clone(&v1), Rc::clone(&v2));
        model.add_triangle(Rc::clone(&v0), Rc::clone(&v2), Rc::clone(&v3));

        println!("model: {model:#?}");

        model.subdivide(1);
    }
}
