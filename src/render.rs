

use chad_core::core::{DriftReport, Lot, Squad, SquadMember};
use rand::random;
use yui::{ArcYard, Before, Cling, Confine, Pack, Padding, SenderLink, SyncLink, Trigger,  yard};
use yui::palette::{FillColor, StrokeColor};
use yui::palette::FillGrade::Plain;
use yui::yard::{ ButtonModel, PressModel, Priority};
use yui::yard::model::{ScrollAction, ScrollModel};

use crate::{sprint, YardId};
use crate::edit_squad::DialogModel;
use crate::models::squad_view::SquadViewModel;
use crate::sprint::amount_prefix;

#[derive(Debug, Clone)]
pub struct SquadMemberViewModel {
	member: SquadMember,
	lots: Vec<Lot>,
	lots_scroll: ScrollModel,
	lot_presses: Vec<PressModel>,
	add_lot_press: ButtonModel,
	action_link: SenderLink<SquadMemberViewAction>,
}

#[derive(Debug, Clone)]
pub enum SquadMemberViewAction {
	PressAddLot,
	ReportLotPress { squad_id: u64, symbol: String, lot_id: Option<u64> },
	PressLot(usize),
	ScrollLots(ScrollAction),
}


impl SquadMemberViewModel {

	// fn reset_member_lots_scroll_model(&self) -> ScrollModel {
	// 	ScrollModel::new_count_height(self.member_lots_scroll.id, 0, 1, 0)
	// }
	// pub fn member_lots_scroll_for_pick(&self, pick: &Option<(u64, Option<String>)>, squads: &Vec<Squad>) -> ScrollModel {
	// 	if let Some((squad_id, member_sym)) = pick {
	// 		if let Some(member_sym) = member_sym {
	// 			if let Some(index) = squads.iter().position(|squad| squad.id == *squad_id) {
	// 				let squad = &squads[index];
	// 				let lots = squad.lots.iter().filter(|lot| lot.symbol == *member_sym).collect::<Vec<_>>();
	// 				ScrollModel::new_count_height(self.member_lots_scroll.id, lots.len(), 1, 0)
	// 			} else {
	// 				self.reset_member_lots_scroll_model()
	// 			}
	// 		} else {
	// 			self.reset_member_lots_scroll_model()
	// 		}
	// 	} else {
	// 		self.reset_member_lots_scroll_model()
	// 	}
	// }

	pub fn new(squad: &Squad, member: &SquadMember, add_lot_trigger: Trigger, action_link: SenderLink<SquadMemberViewAction>) -> Self {
		let member = member.clone();
		let lots = squad.lots.iter().filter(|it| it.symbol == member.symbol).cloned().collect::<Vec<_>>();
		let lots_scroll = ScrollModel::new_count_height(YardId::MemberLotList.as_i32(), lots.len(), 1, 0);

		let mut lot_presses = Vec::new();
		for index in 0..lots.len() {
			let lot = &lots[index];
			let release_action = SquadMemberViewAction::ReportLotPress { squad_id: lot.squad_id, symbol: lot.symbol.to_owned(), lot_id: Some(lot.id) };
			let press = PressModel::new(random(), action_link.map(move |_| release_action.clone()));
			lot_presses.push(press);
		}
		let add_lot_press = ButtonModel::enabled(
			"Add Lot",
			add_lot_trigger,
			action_link.to_sync().map(|_| SquadMemberViewAction::PressAddLot),
			Priority::Default,
		);
		SquadMemberViewModel { member, lots, lots_scroll, lot_presses, add_lot_press, action_link }
	}
	pub fn symbol(&self) -> &String { &self.member.symbol }
	pub fn shares(&self) -> f64 { self.lots.iter().map(|it| it.shares).sum::<f64>() }
	pub fn market_value(&self) -> f64 { self.shares() * self.member.price }
	pub fn lots(&self) -> &Vec<Lot> { &self.lots }
	pub fn lot_press_model_and_link(&self, index: usize) -> (&PressModel, SyncLink<i32>) {
		(
			&self.lot_presses[index],
			self.action_link.to_sync().map(move |_| SquadMemberViewAction::PressLot(index))
		)
	}
	pub fn lots_scroll_model_and_link(&self) -> (ScrollModel, SyncLink<ScrollAction>) { (self.lots_scroll.clone(), self.action_link.to_sync().map(|action| SquadMemberViewAction::ScrollLots(action))) }
	pub fn add_lot_button(&self) -> &ButtonModel { &self.add_lot_press }

	pub fn to_lots_scroll_link(&self) -> SyncLink<ScrollAction> {
		unimplemented!()
	}
}

pub fn member_view(member_view: &SquadMemberViewModel) -> ArcYard {
	let header = {
		let title = yard::title(member_view.symbol(), StrokeColor::BodyOnPrimary, Cling::Left);
		let shares = member_view.shares();
		let shares_label = yard::label(format!("Shares: {}", sprint::amount_prefix(shares, "")), StrokeColor::BodyOnPrimary, Cling::LeftBottom);
		let market_label = yard::label(format!("Market value: {}", sprint::amount(member_view.market_value())), StrokeColor::BodyOnPrimary, Cling::Left);
		let front = title
			.pack_bottom(2, shares_label)
			.pack_bottom(1, market_label)
			.pad(1);
		front.before(yard::fill(FillColor::Primary, Plain))
	};
	let content = {
		let lots = member_view.lots();
		let lots_label = yard::label(format!("Lots ({})", lots.len()), StrokeColor::BodyOnBackground, Cling::Left);
		let lot_list =
			if lots.is_empty() {
				yard::label("No Lots", StrokeColor::CommentOnBackground, Cling::Center)
			} else {
				let yards = lots.iter().enumerate().map(|(index, lot)| {
					let shares_in_account = format!("{} shares in {} account", amount_prefix(lot.shares, ""), &lot.account);
					let label = yard::label(&shares_in_account, StrokeColor::BodyOnBackground, Cling::Left);
					let (press_model, press_link) = member_view.lot_press_model_and_link(index);
					yard::pressable(label, press_model, press_link)
				}).collect::<Vec<_>>();

				let (lots_scroll_model, lots_scroll_link) = member_view.lots_scroll_model_and_link();
				yard::list(yards, lots_scroll_model, lots_scroll_link)
			}.pack_top(2, lots_label.confine_height(1, Cling::Top));

		let add_button = yard::button(member_view.add_lot_button());
		lot_list.pack_bottom(3, add_button.confine(13, 3, Cling::Top))
	};
	content.pad(1).pack_top(7, header)
}

pub fn drift_summary(report: &DriftReport, press_model: &PressModel, press_link: SyncLink<i32>) -> ArcYard {
	let drift_amount = report.drift_amount();
	let left = {
		let symbol = format!("{}", report.symbol());
		let rank = format!("R{}({}%)", report.rank, sprint::amount_prefix(report.target_portion * 100.0, ""));
		yard::label(symbol, StrokeColor::BodyOnBackground, Cling::LeftBottom)
			.pack_bottom(
				1,
				yard::label(rank, StrokeColor::CommentOnBackground, Cling::LeftTop),
			)
	};
	let right = {
		let relative_drift = {
			if drift_amount.is_sign_positive() {
				format!("{} {}", "Over", sprint::amount(drift_amount.abs()))
			} else {
				format!("{} {}", "Under", sprint::amount(drift_amount.abs()))
			}
		};
		let shares_motion = match report.drift_shares() {
			None => "??".to_string(),
			Some(drift_shares) => {
				if drift_shares.is_sign_negative() {
					format!("+={} sh", sprint::amount_prefix(drift_shares.abs(), ""))
				} else {
					format!("-={} sh", sprint::amount_prefix(drift_shares.abs(), ""))
				}
			}
		};
		let (top, bottom) = if drift_amount.is_sign_positive() {
			(
				yard::label(relative_drift, StrokeColor::BodyOnBackground, Cling::RightBottom),
				yard::label(shares_motion, StrokeColor::CommentOnBackground, Cling::RightTop)
			)
		} else {
			(
				yard::label(shares_motion, StrokeColor::CommentOnBackground, Cling::RightTop),
				yard::label(relative_drift, StrokeColor::BodyOnBackground, Cling::RightBottom)
			)
		};
		top.pack_bottom(1, bottom)
	};
	let center = {
		let market_value = report.market_value;
		let target_value = report.target_value;
		let (top, bottom) = if drift_amount.is_sign_positive() {
			let top = yard::label(sprint::amount(market_value), StrokeColor::BodyOnBackground, Cling::LeftBottom);
			let bottom = yard::label(format!("\\- {}", sprint::amount(target_value)), StrokeColor::CommentOnBackground, Cling::RightTop);
			(top, bottom)
		} else {
			let top = yard::label(format!("{}", sprint::amount(target_value)), StrokeColor::CommentOnBackground, Cling::RightTop);
			let bottom = yard::label(format!("{} -/", sprint::amount(market_value)), StrokeColor::BodyOnBackground, Cling::LeftBottom);
			(top, bottom)
		};
		top.pack_bottom(1, bottom).confine_width(10, Cling::Center)
	};
	let content = center.pack_left(12, left).pack_right(12, right);
	yard::pressable(content.pad(1), press_model, press_link).confine_width(50, Cling::Custom { x: 0.1, y: 0.5 })
}

pub fn squad(squad_view: &SquadViewModel) -> ArcYard {
	let title = yard::title(&squad_view.squad_name(), StrokeColor::BodyOnPrimary, Cling::LeftBottom);
	let header = title.pad(1).before(yard::fill(FillColor::Primary, Plain));
	let content = {
		let unspent = {
			let label = yard::label("Unspent: ", StrokeColor::BodyOnBackground, Cling::Left);
			let (unspent_press_model, unspent_press_text_len) = squad_view.unspent_press_model_and_text_len();
			let button = yard::button(unspent_press_model);
			yard::empty()
				.pack_left(unspent_press_text_len as i32 + 6, button)
				.pack_left("Unspent: ".len() as i32, label)
		};
		let members = {
			let member_count = squad_view.squad_members_count();
			let label_text = format!("Members ({})", member_count);
			let label = yard::label(label_text, StrokeColor::BodyOnBackground, Cling::LeftBottom);
			let list = if member_count == 0 {
				yard::label("No members", StrokeColor::CommentOnBackground, Cling::Center)
			} else {
				let summary_yards = squad_view.drift_reports()
					.iter()
					.enumerate()
					.map(|(index, report)| {
						let (press_model, press_link) = squad_view.squad_member_press_model_and_link(index);
						drift_summary(report, press_model, press_link)
					})
					.collect();

				let (scroll_model, scroll_link) = squad_view.squad_members_scroll_model_and_link();
				yard::list(summary_yards, scroll_model.clone(), scroll_link)
			};
			let button = yard::button(squad_view.add_member_press_model());
			list.pack_top(1, label).pack_bottom(3, button)
		};
		members.pack_top(3, unspent)
	}.pad(1);
	content.pack_top(4, header)
}

pub fn dialog(title: &str, dialog_model: &DialogModel, content: ArcYard) -> ArcYard {
	const LEFT_COLS: i32 = 7;
	let header
		= yard::title(title, StrokeColor::BodyOnBackground, Cling::LeftBottom)
		.pack_top(1, yard::empty())
		.pad_cols(2)
		.pack_left(LEFT_COLS, yard::button(&dialog_model.close));
	let footer = {
		let submit_button = yard::button(&dialog_model.submit);
		match &dialog_model.delete {
			None => submit_button.confine(14, 3, Cling::Top),
			Some(delete_model) => {
				yard::empty().pack_left(14, submit_button).pack_right(14, yard::button(delete_model))
			}
		}
	};
	let content = content.pad(1).pack_left(LEFT_COLS, yard::empty());
	content
		.pack_top(3, header)
		.pack_bottom(4, footer.confine_height(3, Cling::Top).pad_cols(2).pack_left(LEFT_COLS, yard::empty()))
}