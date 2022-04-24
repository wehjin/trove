use chad_core::core::{DriftReport, Squad};
use rand::random;
use yui::{SenderLink, SyncLink};
use yui::yard::{ButtonModel, PressModel, Priority};
use yui::yard::model::{ScrollAction, ScrollModel};

use crate::{sprint, YardId};

#[derive(Debug, Clone)]
pub enum SquadViewAction {
	ScrollSquadMembers(ScrollAction),
	ReleaseUnspent { squad_id: u64, unspent: Option<f64> },
	PressUnspent,
	ReleaseMember { squad_id: u64, symbol: String },
	PressMember,
	ReleaseAddMember { squad_id: u64 },
	PressAddMember,
}

#[derive(Debug, Clone)]
pub struct SquadViewModel {
	squad: Squad,
	drift_reports: Vec<DriftReport>,
	view_member_report: SenderLink<(u64, String)>,
	unspent_press: ButtonModel,
	squad_members_scroll: ScrollModel,
	squad_member_presses: Vec<PressModel>,
	add_member_press: ButtonModel,
	action_link: SenderLink<SquadViewAction>,
}

impl SquadViewModel {
	pub fn new(squad: &Squad,
	           _set_unspent_report: SenderLink<(u64, Option<f64>)>,
	           view_member_report: SenderLink<(u64, String)>,
	           action_link: SenderLink<SquadViewAction>,
	) -> Self {
		let mut drift_reports = squad.drift_reports();
		drift_reports.reverse();
		let drift_reports_count = drift_reports.len();
		let squad_members_presses = {
			let mut vec = Vec::new();
			for index in 0..drift_reports_count {
				let report = &drift_reports[index];
				let squad_id = report.member.squad_id;
				let symbol = report.symbol().to_string();
				let press_model = PressModel::new(
					random(),
					action_link.to_trigger(SquadViewAction::ReleaseMember { squad_id, symbol }),
				);
				vec.push(press_model);
			}
			vec
		};
		SquadViewModel {
			squad: squad.clone(),
			drift_reports,
			view_member_report,
			unspent_press: ButtonModel::enabled(
				&sprint::amount(squad.unspent),
				action_link.to_trigger(SquadViewAction::ReleaseUnspent { squad_id: squad.id, unspent: if squad.unspent == 0.0 { None } else { Some(squad.unspent) } }),
				action_link.to_sync().map(|_| SquadViewAction::PressUnspent),
				Priority::Default,
			),
			squad_members_scroll: ScrollModel::new_count_height(YardId::SquadMembersList.as_i32(), drift_reports_count, 4, 0),
			squad_member_presses: squad_members_presses,
			add_member_press: ButtonModel::enabled(
				"Add Member",
				action_link.to_trigger(SquadViewAction::ReleaseAddMember { squad_id: squad.id }),
				action_link.to_sync().map(|_| SquadViewAction::PressAddMember),
				Priority::None,
			),
			action_link,
		}
	}

	pub fn squad_name(&self) -> &String { &self.squad.name }
	pub fn squad_members_count(&self) -> usize { self.squad.members.len() }
	pub fn drift_reports(&self) -> &Vec<DriftReport> { &self.drift_reports }
	pub fn squad_members_scroll_model_and_link(&self) -> (&ScrollModel, SyncLink<ScrollAction>) { (&self.squad_members_scroll, self.action_link.to_sync().map(|action| SquadViewAction::ScrollSquadMembers(action))) }
	pub fn squad_member_press_model_and_link(&self, index: usize) -> (&PressModel, SyncLink<i32>) { (&self.squad_member_presses[index], self.action_link.to_sync().map(|_| SquadViewAction::PressMember)) }
	pub fn unspent_press_model_and_text_len(&self) -> (&ButtonModel, usize) { (&self.unspent_press, self.unspent_press.label.len()) }
	pub fn add_member_press_model(&self) -> &ButtonModel { &self.add_member_press }
}
