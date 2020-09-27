pub fn amount(amount: f64) -> String {
	let is_positive = amount >= 0.0;
	let mut amount = amount.abs();
	let mut shift_count = 0usize;
	while amount >= 1000.0 {
		shift_count += 1;
		amount /= 1000.0;
	}
	const SUFFIXES: [&str; 5] = ["", "K", "M", "B", "T"];
	let suffix = if shift_count < SUFFIXES.len() {
		SUFFIXES[shift_count].to_string()
	} else {
		format!("e{}", shift_count)
	};
	let pos_amount = if amount < 10.0 {
		format!("${:.02}{}", amount, suffix)
	} else if amount < 100.0 {
		format!("${:.01}{}", amount, suffix)
	} else {
		format!("${:.00}{}", amount, suffix)
	};
	if is_positive { pos_amount } else { format!("({})", pos_amount) }
}

#[cfg(test)]
mod tests {
	#[test]
	fn amount_works_for_scale() {
		let inputs = [
			0.001f64, 0.01, 0.1,
			1.0, 10.0, 100.0,
			1000.0, 10000.0, 100000.0,
			1000000.0, 10000000.0, 100000000.0,
			1000000000.0, 10000000000.0, 100000000000.0,
		];
		let expected = [
			"$0.00", "$0.01", "$0.10",
			"$1.00", "$10.0", "$100",
			"$1.00K", "$10.0K", "$100K",
			"$1.00M", "$10.0M", "$100M",
			"$1.00B", "$10.0B", "$100B",
		].to_vec();
		let outputs = inputs.iter().cloned().map(super::amount).collect::<Vec<_>>();
		assert_eq!(expected, outputs);
	}

	#[test]
	fn amount_works_for_negative() {
		assert_eq!("($100)", super::amount(-100.0));
	}
}


