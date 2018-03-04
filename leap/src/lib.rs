pub fn is_leap_year(year: i32) -> bool {
	let y = |x| year % x == 0;
	y(4) && (!y(100) || y(400))
}

