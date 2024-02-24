use bevy::prelude::Component;

#[derive(Component)]
pub struct Panel;

#[derive(Component, Copy, Clone, Eq, PartialEq)]
pub struct Position {
	pub left: u16,
	pub top: u16,
	pub right: u16,
	pub bottom: u16,
	pub near: u16,
	pub far: u16,
}

