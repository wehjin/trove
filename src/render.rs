use chad_core::core::{Lot, Squad, SquadMember};
use yui::{ArcYard, Before, Cling, Confine, Pack, Padding, SenderLink, yard};
use yui::palette::{FillColor, StrokeColor};
use yui::yard::{ButtonState, Pressable};

use crate::{sprint, YardId};
use crate::sprint::amount_prefix;

pub fn lot_summary(lot: &Lot) -> (u8, ArcYard) {
	(1, yard::label(&format!("{} shares in {} account", amount_prefix(lot.shares, ""), &lot.account), StrokeColor::BodyOnBackground, Cling::Left))
}

pub fn member_view(member: &SquadMember, squad: &Squad, add_lot_link: SenderLink<()>) -> ArcYard {
	let lots = squad.lots.iter().filter(|it| it.symbol == member.symbol).collect::<Vec<_>>();
	let header = {
		let title = yard::title(&member.symbol, StrokeColor::BodyOnPrimary, Cling::Left);
		let shares = lots.iter().map(|it| it.shares).sum::<f64>();
		let shares_label = yard::label(format!("Shares: {}", sprint::amount_prefix(shares, "")), StrokeColor::BodyOnPrimary, Cling::LeftBottom);
		let market_value = shares * squad.prices[&member.symbol];
		let market_label = yard::label(format!("Market value: {}", sprint::amount(market_value)), StrokeColor::BodyOnPrimary, Cling::Left);
		let front = title
			.pack_bottom(2, shares_label)
			.pack_bottom(1, market_label)
			.pad(1);
		front.before(yard::fill(FillColor::Primary))
	};
	let content = {
		let lots_label = yard::label(format!("Lots ({})", lots.len()), StrokeColor::CommentOnBackground, Cling::Left);
		let lot_list = if lots.is_empty() {
			yard::label("No Lots", StrokeColor::CommentOnBackground, Cling::Center)
		} else {
			let lot_items = lots.into_iter().map(lot_summary).collect();
			yard::list(YardId::MemberLotList.as_i32(), 0, lot_items)
		}.pack_top(2, lots_label.confine_height(1, Cling::Top));
		let add_button = yard::button("Add Lot", ButtonState::default(add_lot_link.map(|_| ())));
		lot_list.pack_bottom(3, add_button.confine(13, 3, Cling::Top))
	};
	content.pad(1).pack_top(7, header)
}

pub fn member_summary(member: &SquadMember, index: usize, shares: f64, select_link: SenderLink<(u64, String)>) -> (u8, ArcYard) {
	let symbol = format!("{}", member.symbol);
	let price = member.price;
	let market_amount = shares * price;
	let target_amount = 8500000.0;
	let drift_amount = market_amount - target_amount;
	let rank = format!("R{} (50%)", index + 1);
	let drift = format!("{} {}", sprint::amount(drift_amount.abs()), if drift_amount.is_sign_positive() { "Over" } else { "Under" });
	let transact_shares = if price == 0.0 { "??".to_string() } else { sprint::amount_prefix(drift_amount.abs() / price, "") };
	let motion = format!("~{} sh", transact_shares);
	let left = yard::label(symbol, StrokeColor::BodyOnBackground, Cling::LeftBottom)
		.pack_bottom(
			1,
			yard::label(rank, StrokeColor::CommentOnBackground, Cling::LeftTop),
		);
	let right = yard::label(drift, StrokeColor::BodyOnBackground, Cling::RightBottom)
		.pack_bottom(
			1,
			yard::label(motion, StrokeColor::CommentOnBackground, Cling::RightTop),
		);
	let center = {
		let (top, bottom) = if drift_amount.is_sign_positive() {
			let top = yard::label(sprint::amount(market_amount), StrokeColor::BodyOnBackground, Cling::LeftBottom);
			let bottom = yard::label(format!("\\- {}", sprint::amount(target_amount)), StrokeColor::CommentOnBackground, Cling::RightTop);
			(top, bottom)
		} else {
			let top = yard::label(format!("{}", sprint::amount(target_amount)), StrokeColor::CommentOnBackground, Cling::RightTop);
			let bottom = yard::label(format!("{} -/", sprint::amount(market_amount)), StrokeColor::BodyOnBackground, Cling::LeftBottom);
			(top, bottom)
		};
		top.pack_bottom(1, bottom).confine_width(10, Cling::Center)
	};
	let content = center
		.pack_left(12, left)
		.pack_right(12, right);
	let cell = content.pad(1).pressable(select_link.map({
		let squad_id = member.squad_id;
		let symbol = member.symbol.to_string();
		move |_| (squad_id, symbol.clone())
	})).confine_width(50, Cling::Custom { x: 0.1, y: 0.5 });
	(4, cell)
}

pub fn squad(squad: &Squad, add_member_link: SenderLink<()>, view_member_link: SenderLink<(u64, String)>) -> ArcYard {
	let title = yard::title(&squad.name, StrokeColor::BodyOnPrimary, Cling::LeftBottom);
	let header = title.pad(1).before(yard::fill(FillColor::Primary));
	let content = {
		let unspent = {
			let label_text = "Unspent: ";
			let label = yard::label(label_text, StrokeColor::CommentOnBackground, Cling::Left);
			let button_text = sprint::amount(squad.unspent);
			let button = yard::button(&button_text, ButtonState::default(SenderLink::ignore()));
			yard::empty()
				.pack_left(button_text.len() as i32 + 6, button)
				.pack_left(label_text.len() as i32, label)
		};
		let members = {
			let member_count = squad.members.len();
			let label_text = format!("Members ({})", member_count);
			let label = yard::label(label_text, StrokeColor::CommentOnBackground, Cling::LeftBottom);
			let list = if member_count == 0 {
				yard::label("No members", StrokeColor::CommentOnBackground, Cling::Center)
			} else {
				let shares = squad.shares();
				let items = squad.members.iter().enumerate().rev().map(|(index, member)| {
					let shares = *shares.get(&member.symbol).unwrap_or(&0.0);
					member_summary(member, index, shares, view_member_link.clone())
				}).collect();
				yard::list(YardId::SquadMembersList.as_i32(), 0, items)
			};
			let button = yard::button("Add Member", ButtonState::enabled(add_member_link.map(|_| ())));
			list
				.pack_top(1, label)
				.pack_bottom(3, button)
		};
		members.pack_top(3, unspent)
	}.pad(1);
	content.pack_top(4, header)
}

pub fn dialog(title: &str, close_link: SenderLink<()>, submit_button_state: ButtonState, content: ArcYard) -> ArcYard {
	const LEFT_COLS: i32 = 7;
	let close = yard::button("x", ButtonState::default(close_link.map(|_| ())));
	let title = yard::title(title, StrokeColor::BodyOnBackground, Cling::LeftBottom).pack_top(1, yard::empty());
	let submit = yard::button("Submit", submit_button_state);
	let header = title.pad_cols(2).pack_left(LEFT_COLS, close);
	let footer = submit.confine(14, 3, Cling::Top);
	let content = content.pad(1).pack_left(LEFT_COLS, yard::empty());
	content.pack_top(3, header).pack_bottom(4, footer)
}