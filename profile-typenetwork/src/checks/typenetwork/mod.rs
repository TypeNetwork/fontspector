#[allow(non_snake_case)]
pub mod weightclass;
pub use weightclass::weightclass;

// pub mod composite_glyphs;
// pub use composite_glyphs::composite_glyphs;

pub mod family;
pub use family::{duplicated_names, equal_numbers_of_glyphs, tnum_horizontal_metrics};
pub mod name;
pub mod varfont;
pub use varfont::{axes_have_variation};

pub mod valid_underline;
pub use valid_underline::valid_underline;

pub mod valid_strikeout;
pub use valid_strikeout::valid_strikeout;