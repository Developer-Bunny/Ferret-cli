pub fn sanitize_degrees_double(degrees: f64) -> f64 {
	let degrees = degrees % 360.0;
	if degrees < 0.0 {
		degrees + 360.0
	} else {
		degrees
	}
}

pub fn sanitize_degrees_int(degrees: i32) -> i32 {
	let degrees = degrees % 360;
	if degrees < 0 {
		degrees + 360
	} else {
		degrees
	}
}

pub fn difference_degrees(a: f64, b: f64) -> f64 {
	180.0 - ((a - b).abs() - 180.0).abs()
}

pub fn rotation_direction(from: f64, to: f64) -> f64 {
	let a = to - from;
	let b = to - from + 360.0;
	let c = to - from - 360.0;

	let a_abs = a.abs();
	let b_abs = b.abs();
	let c_abs = c.abs();

	if a_abs <= b_abs && a_abs <= c_abs {
		if a >= 0.0 { 1.0 } else { -1.0 }
	} else if b_abs <= a_abs && b_abs <= c_abs {
		if b >= 0.0 { 1.0 } else { -1.0 }
	} else {
		if c >= 0.0 { 1.0 } else { -1.0 }
	}
}
