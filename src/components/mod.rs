use amane::Widget;

pub mod bar;
pub mod button;
pub mod icon;
pub mod label;
pub mod mrow;
pub mod statusicons;

pub type Widgets = Vec<Box<dyn Widget>>;
