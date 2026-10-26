//! The story: Jour de jeu, the Timeline of what happens each day, and the mail it delivers.
//!
//! Real time and real weather never enter here directly: they come in through
//! `StoryClock` construction and `ReadContext` fields, both built by the app.

pub mod clock;
pub mod content;
pub mod event;
pub mod sources;
pub mod timeline;

pub use clock::StoryClock;
pub use event::Entry;
pub use timeline::Timeline;
