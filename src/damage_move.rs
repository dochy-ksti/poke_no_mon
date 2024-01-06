#[derive(Debug, Clone)]
pub struct DamageMove {
	pub name : String,
    pub kind: MoveKind,
    pub move_type: Types,
    pub power: u32,
    /// そのうち使いたい
    //pub contact: bool,
	pub priority : i32,
	pub drain : Option<PNum>,
}

impl DamageMove{
    const fn new(kind : MoveKind, t : Types, power : u32) -> Self {
        Self { kind, move_type : t, power, priority: 0, drain: None }
    }
}
