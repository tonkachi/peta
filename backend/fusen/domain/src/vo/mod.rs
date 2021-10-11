#[allow(clippy::module_inception)]
mod vo;

mod id;
mod note;
mod title;

pub use self::id::Id;
pub use self::note::FusenNote;
pub use self::title::FusenTitle;
pub use self::vo::ValueObject;
