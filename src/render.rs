use yui::{ArcYard, Cling, Confine, Pack, Padding, SenderLink, yard};
use yui::palette::StrokeColor;
use yui::yard::Pressable;

pub fn member_summary() -> ArcYard {
	let symbol = format!("SQ");
	let rank = format!("Rank 5 (50%)");
	let drift = format!("$3.3M Over");
	let motion = format!("Sell 20");
	let market_value = format!("$10.8M");
	let target_value = format!("\\_$7.5M");
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
	let center = yard::label(market_value, StrokeColor::BodyOnBackground, Cling::LeftBottom)
		.pack_bottom(
			1,
			yard::label(target_value, StrokeColor::CommentOnBackground, Cling::RightTop),
		).confine_width(10, Cling::Center);
	let full = center
		.pack_left(12, left)
		.pack_right(12, right);
	full.pad(1).pressable(SenderLink::ignore()).confine_width(50, Cling::Custom { x: 0.1, y: 0.5 })
}