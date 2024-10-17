use super::{geometry::Geometry, transform::Transform};

#[derive(Clone, Copy, Debug)]
pub struct Shape {
    pub(super) geometry: Geometry,
    transform: Transform,
}

impl Shape {
    pub(crate) fn with_geometry(geometry: Geometry) -> Self {
        Self {
            geometry,
            transform: Transform::IDENTITY,
        }
    }

    pub(crate) fn geometry(&self) -> &Geometry {
        &self.geometry
    }

    pub(crate) fn transform(&self) -> &Transform {
        &self.transform
    }

    pub(crate) fn new(geometry: Geometry, transform: Transform) -> Self {
        Self {
            geometry,
            transform,
        }
    }

    pub(crate) fn pre_transform(&mut self, pre: Transform) {
        self.transform = pre * self.transform;
    }
}
