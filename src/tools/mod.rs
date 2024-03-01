#[allow(unused)]
pub use crossterm::style::Color;

pub mod captor;
pub mod console;
pub mod fill;
pub mod frame;
pub mod inset;
pub mod sample;
pub mod screen;
pub mod solar_dark;
pub mod views;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum UserEvent {
	Select,
}

