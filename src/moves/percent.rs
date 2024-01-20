
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percent{
	pub val : u32
}

impl Percent{
    pub fn new(val: u32) -> Self { Self { val } }

	pub const V0 : Percent = Percent{ val : 0 };
}