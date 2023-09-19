use crate::{Edge, Model, Vertex};
use hashbrown::HashMap;
use std::cell::RefCell;
use std::f32::consts::TAU;
use std::rc::Rc;

pub fn subdivide_once(model: Model) -> Model {
    let mut edge_points = HashMap::new();

    // TODO: parallelize
    for edge in model.edges.iter() {
        let edge = &*edge.borrow();
        edge_points.insert(edge.id, Rc::new(RefCell::new(subdivide_edge(edge))));
    }

    let mut vertex_points = HashMap::new();

    for vertex in model.vertices.values() {
        let vertex = &*vertex.borrow();
        vertex_points.insert(vertex.id, Rc::new(RefCell::new(move_vertex(vertex))));
    }

    let new_triangles: Vec<_> = model
        .triangles
        .iter()
        .flat_map(|triangle| {
            let triangle = triangle.borrow();

            let new_edge_v0 = edge_points
                .get(&triangle.e0().borrow().id)
                .expect("just added above");
            let new_edge_v1 = edge_points
                .get(&triangle.e1().borrow().id)
                .expect("just added above");
            let new_edge_v2 = edge_points
                .get(&triangle.e2().borrow().id)
                .expect("just added above");

            let new_vertex0 = vertex_points
                .get(&triangle.v0().borrow().id)
                .expect("just added above");
            let new_vertex1 = vertex_points
                .get(&triangle.v1().borrow().id)
                .expect("just added above");
            let new_vertex2 = vertex_points
                .get(&triangle.v2().borrow().id)
                .expect("just added above");

            [
                [
                    Rc::clone(new_vertex0),
                    Rc::clone(new_edge_v0),
                    Rc::clone(new_edge_v2),
                ],
                [
                    Rc::clone(new_edge_v0),
                    Rc::clone(new_vertex1),
                    Rc::clone(new_edge_v1),
                ],
                [
                    Rc::clone(new_edge_v0),
                    Rc::clone(new_edge_v1),
                    Rc::clone(new_edge_v2),
                ],
                [
                    Rc::clone(new_edge_v2),
                    Rc::clone(new_edge_v1),
                    Rc::clone(new_vertex2),
                ],
            ]
        })
        .collect();

    let mut new_model = Model::default();

    for [v0, v1, v2] in new_triangles {
        new_model.add_triangle(v0, v1, v2);
    }

    new_model
}

fn subdivide_edge(edge: &Edge) -> Vertex {
    let a = edge.a();
    let a = a.borrow();

    if edge.triangles.len() != 2 {
        // boundary case for edge
        let m = (a.position + edge.b().borrow().position) * 0.5;
        Vertex::with_index(m, a.index)
    } else {
        const ALPHA: f32 = 3.0 / 8.0;
        const BETA: f32 = 1.0 / 8.0;

        let left = edge.triangles[0]
            .upgrade()
            .expect("edge.triangles[0] is gone")
            .borrow()
            .get_opposite_vertex(edge);
        let left = left.borrow();

        let right = edge.triangles[1]
            .upgrade()
            .expect("edge.triangles[1] is gone")
            .borrow()
            .get_opposite_vertex(edge);
        let right = right.borrow();

        let b = edge.b();
        let b = b.borrow();

        Vertex::with_index(
            (a.position + b.position) * ALPHA + (left.position + right.position) * BETA,
            a.index,
        )
    }
}

fn get_adjacent_vertices(vertex: &Vertex) -> Vec<Rc<RefCell<Vertex>>> {
    let mut adjancent_vertices = Vec::with_capacity(vertex.edges.len());

    let mut is_boundary = false;

    for edge in vertex.edges.iter() {
        let edge = edge.upgrade().expect("edge is gone");
        let edge = edge.borrow();

        let is_boundary_edge = edge.triangles.len() != 2;

        if is_boundary_edge && !is_boundary {
            adjancent_vertices.clear();

            is_boundary = true;
        }

        if !is_boundary || is_boundary_edge {
            adjancent_vertices.push(edge.get_other_vertex(vertex));
        }
    }

    adjancent_vertices
}

fn move_vertex(vertex: &Vertex) -> Vertex {
    let adjancent_vertices = get_adjacent_vertices(vertex);

    let n = adjancent_vertices.len();

    if n < 2 {
        Vertex::with_index(vertex.position, vertex.index)
    } else if n < 3 {
        // boundary case for corner

        let v0 = adjancent_vertices[0].borrow();
        let v1 = adjancent_vertices[1].borrow();

        const K0: f32 = 3.0 / 4.0;
        const K1: f32 = 1.0 / 8.0;

        Vertex::with_index(
            K0 * vertex.position + K1 * (v0.position + v1.position),
            vertex.index,
        )
    } else {
        const K0: f32 = 5.0 / 8.0;
        const K1: f32 = 3.0 / 8.0;
        const K2: f32 = 1.0 / 4.0;
        let alpha = if n == 3 {
            3.0 / 16.0
        } else {
            let n = n as f32;
            (1.0 / n) * (K0 - (K1 + K2 * (TAU / n).cos()).powi(2))
        };

        let mut new_pos = (1.0 - n as f32 * alpha) * vertex.position;

        for ajdacent_vertex in adjancent_vertices.iter() {
            new_pos += ajdacent_vertex.borrow().position * alpha;
        }

        Vertex::with_index(new_pos, vertex.index)
    }
}
