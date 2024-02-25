use bevy::prelude::{Commands, Entity, Query, With};

use crate::components::layout::{Louter, RootLouter};
use crate::components::view::{RootView, View, ViewMadeLayout};

pub fn despawn_view_layouts(query: Query<Entity, With<ViewMadeLayout>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn spawn_root_view_layouts<ViewMsg: 'static>(query: Query<&View<ViewMsg>, With<RootView>>, mut commands: Commands) {
	let view = query.single();
	for renderer in view.model.to_layouts() {
		let layout = Louter { layout: renderer };
		commands.spawn((ViewMadeLayout, layout, RootLouter));
	}
}
