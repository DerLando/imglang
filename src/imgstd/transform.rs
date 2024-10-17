#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub(crate) inner: piet::kurbo::Affine,
}

impl Transform {
    pub const IDENTITY: Transform = Self {
        inner: piet::kurbo::Affine::IDENTITY,
    };

    pub(crate) fn get_pre_transform(canvas_width: f64, canvas_height: f64) -> Self {
        let translation = piet::kurbo::Affine::translate((canvas_width / 2.0, canvas_height / 2.0));
        let mirror = piet::kurbo::Affine::FLIP_Y;

        Self {
            inner: translation * mirror,
        }
    }

    pub fn translate(x: f64, y: f64) -> Self {
        Self {
            inner: piet::kurbo::Affine::translate((x, y)),
        }
    }
}

impl std::ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Self::Output {
        Transform {
            inner: self.inner * rhs.inner,
        }
    }
}
