/// Re-exports of shared geometric and identity types from `tezzera-trace`.
pub use tezzera_trace::event::{AtomId, ComponentId, Location, Point, Rect, Size};

/// A stable identity key used to reconcile elements across rebuilds.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Key(pub u64);
