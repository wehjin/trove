use chad_core::core::{Squad, SquadMember};
use yui::{ArcYard, Before, Cling, Confine, Pack, Padding, SenderLink, yard};
use yui::palette::{FillColor, StrokeColor};
use yui::yard::{ButtonState, Pressable};

use crate::{sprint, YardId};

pub fn member_view(member: &SquadMember, squad: &Squad, add_lot: SenderLink<()>) -> ArcYard {
	let header = yard::title(&member.symbol, StrokeColor::BodyOnPrimary, Cling::Left)
		.pad(1).before(yard::fill(FillColor::Primary));
	let content = {
		let lots = squad.lots.iter().filter(|it| it.symbol == member.symbol).collect::<Vec<_>>();
		let shares = lots.iter().map(|it| it.shares).sum::<f64>();
		let shares_label = yard::label(format!("{} Shares", sprint::amount_prefix(shares, "")), StrokeColor::BodyOnBackground, Cling::Left);
		let market_value = shares * squad.prices[&member.symbol];
		let market_label = yard::label(format!("{} Market Value", sprint::amount(market_value)), StrokeColor::BodyOnBackground, Cling::Left);
		let lots_label = yard::label(format!("Lots ({})", lots.len()), StrokeColor::CommentOnBackground, Cling::Left);
		let button = yard::button("Add Lot", ButtonState::default(add_lot.map(|_| ())));
		yard::empty()
			.pack_top(2, lots_label.confine_height(1, Cling::Top))
			.pack_top(2, market_label.confine_height(1, Cling::Top))
			.pack_top(2, shares_label.confine_height(1, Cling::Top))
			.pack_bottom(3, button.confine(13, 3, Cling::Top))
	};
	content.pad(1).pack_top(4, header)
}

pub fn member_summary(member: &SquadMember, index: usize, select_link: SenderLink<(u64, String)>) -> ArcYard {
	let symbol = format!("{}", member.symbol);
	let shares = 0.0;
	let price = member.price;
	let market_amount = shares * price;
	let target_amount = 8500000.0;
	let drift_amount = market_amount - target_amount;
	let rank = format!("Rank {} (50%)", index + 1);
	let drift = format!("{} {}", sprint::amount(drift_amount.abs()), if drift_amount.is_sign_positive() { "Over" } else { "Under" });
	let transact_shares = if price == 0.0 { "??".to_string() } else { sprint::amount_prefix(drift_amount.abs() / price, "") };
	let motion = format!("{} {}", if drift_amount.is_sign_positive() { "Sell" } else { "Buy" }, transact_shares);
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
			let bottom = yard::label(format!("\\_{}", sprint::amount(target_amount)), StrokeColor::CommentOnBackground, Cling::RightTop);
			(top, bottom)
		} else {
			let top = yard::label(format!("{}", sprint::amount(target_amount)), StrokeColor::CommentOnBackground, Cling::RightTop);
			let bottom = yard::label(format!("{}_/", sprint::amount(market_amount)), StrokeColor::BodyOnBackground, Cling::LeftBottom);
			(top, bottom)
		};
		top.pack_bottom(1, bottom).confine_width(10, Cling::Center)
	};
	let full = center
		.pack_left(12, left)
		.pack_right(12, right);
	full
		.pad(1)
		.pressable(select_link.map({
			let squad_id = member.squad_id;
			let symbol = member.symbol.to_string();
			move |_| (squad_id, symbol.clone())
		}))
		.confine_width(50, Cling::Custom { x: 0.1, y: 0.5 })
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
				let items = squad.members.iter().enumerate().rev().map(|(index, it)| {
					let rendering = member_summary(it, index, view_member_link.clone());
					(4, rendering)
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