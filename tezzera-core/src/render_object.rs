use crate::types::{Point, Size};

/// Placeholder canvas type for Phase 1.
///
/// The real Skia-backed canvas is provided by `tezzera-render`; this stub
/// keeps `tezzera-core` free of rendering dependencies during the component
/// model phase.
pub struct Canvas;

/// Describes a bound along a single layout axis.
#[derive(Clone, Debug, PartialEq)]
pub enum AxisBound {
    /// The axis has a finite pixel limit.
    Bounded(f32),
    /// The axis is unconstrained — the child may be any size.
    Unbounded,
    /// The axis must shrink to fit its content (intrinsic sizing).
    Shrink,
}

/// Layout constraints passed down the render tree during the measure pass.
///
/// Distinct from `tezzera_trace::event::TraceConstraints`, which is a
/// simplified snapshot for tracing purposes only.
#[derive(Clone, Debug)]
pub struct Constraints {
    /// Minimum allowed width in logical pixels.
    pub min_width: f32,
    /// Maximum allowed width.
    pub max_width: AxisBound,
    /// Minimum allowed height in logical pixels.
    pub min_height: f32,
    /// Maximum allowed height.
    pub max_height: AxisBound,
}

impl Constraints {
    /// Loose constraints: minimum is zero, maximum is the given dimensions.
    ///
    /// The child may be any size up to `width` × `height`.
    pub fn loose(width: f32, height: f32) -> Self {
        Constraints {
            min_width: 0.0,
            max_width: AxisBound::Bounded(width),
            min_height: 0.0,
            max_height: AxisBound::Bounded(height),
        }
    }

    /// Tight constraints: minimum equals maximum at the given dimensions.
    ///
    /// The child must be exactly `width` × `height`.
    pub fn tight(width: f32, height: f32) -> Self {
        Constraints {
            min_width: width,
            max_width: AxisBound::Bounded(width),
            min_height: height,
            max_height: AxisBound::Bounded(height),
        }
    }

    /// Fully unbounded constraints: the child may take any size on both axes.
    pub fn unbounded() -> Self {
        Constraints {
            min_width: 0.0,
            max_width: AxisBound::Unbounded,
            min_height: 0.0,
            max_height: AxisBound::Unbounded,
        }
    }
}

/// The core trait implemented by every node in the render tree.
///
/// Each `RenderObject` is responsible for measuring itself given layout
/// `Constraints`, painting into a `Canvas`, and reporting hit-test results.
pub trait RenderObject: 'static {
    /// Measures the object under the given constraints and returns its size.
    fn layout(&mut self, constraints: Constraints) -> Size;

    /// Paints the object into `canvas` at the origin, occupying `size`.
    fn paint(&self, canvas: &mut Canvas, size: Size);

    /// Returns `true` if `point` lies within the object's bounding box.
    ///
    /// The default implementation uses the full axis-aligned bounding box
    /// anchored at the origin.
    fn hit_test(&self, point: Point, size: Size) -> bool {
        point.x >= 0.0
            && point.x <= size.width
            && point.y >= 0.0
            && point.y <= size.height
    }
}
