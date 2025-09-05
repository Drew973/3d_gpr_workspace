//stuff used by any part

pub type Amplitude = i16;//defining alias in case this changes.

pub type Longitudinal = u32;  //X-lines~11019. 4,294,967,295
pub type Transverse = u8;//usually 25. 255 should be more than big enough
pub type Depth = u8; //255 exactly.
//48 bit vs 192(usize) per point


/*
#[derive(Serialize,Debug,PartialEq,Hash, Eq, Copy,Clone)]
pub struct XY {
	pub x:usize,
	pub y: usize,
}
*/


pub fn usize_last_multiple(number:usize , of:usize) -> usize{
	of*number/of
}

pub fn usize_next_multiple(number:usize , of:usize) -> usize{
	usize_last_multiple(number,of) + of
}

pub fn longitudinal_difference(a: Longitudinal, b: Longitudinal) -> Longitudinal {
	if a>b{
		return a-b;
	}
	else{
		return b-a;
	}
}



//absolute difference between a and b
pub fn usize_dif(a : usize , b:usize) -> usize{
	if a>b {
		return a-b;
		}
	else{
		return b-a;
	}
}

//a-b or 0
pub fn usize_subtract(a : usize , b:usize) -> usize{
	if a>b {
		return a-b;
		}
	else{
		return 0;
	}
}



pub fn clamp(value:usize, min:usize , max:usize) -> usize{
	if value < min{
		return min;
	}
	if value > max{
		return max;
	}
	return value;
}

	