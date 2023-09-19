mod edge;
mod model;
mod subdiv_surface;
mod triangle;
mod vertex;
mod debug;

pub use edge::*;
pub use model::*;
pub use subdiv_surface::*;
pub use triangle::*;
pub use vertex::*;

macro_rules! def_next_id {
    () => {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

        fn next_id() -> usize {
            NEXT_ID.fetch_add(1, Ordering::SeqCst)
        }
    };
}

macro_rules! impl_eq_hash {
    ($ty:ty) => {
        impl PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl Eq for $ty {}

        impl Hash for $ty {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.id.hash(state);
            }
        }
    };
}

macro_rules! impl_weak_getter {
    ($name:ident, $ty:ty) => {
        pub fn $name(&self) -> Rc<RefCell<$ty>> {
            self
                .$name
                .upgrade()
                .expect(&format!("{} is gone", stringify!($name)))

        }
    };
}

pub(crate) use def_next_id;
pub(crate) use impl_eq_hash;
pub(crate) use impl_weak_getter;
