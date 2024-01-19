#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Ranks {
    pub ranks: [i8; 8],
}

impl Ranks {
    pub fn new(atk: i32, def: i32, satk: i32, sdef: i32, speed: i32) -> Ranks {
        fn a(i: i32) -> i8 {
            i as i8
        }
        Self {
            ranks: [a(atk), a(def), a(satk), a(sdef), a(speed), 0, 0, 0],
        }
    }
    pub fn atk(&self) -> i32 {
        self.ranks[0] as i32
    }
    pub fn def(&self) -> i32 {
        self.ranks[1] as i32
    }
    pub fn satk(&self) -> i32 {
        self.ranks[2] as i32
    }
    pub fn sdef(&self) -> i32 {
        self.ranks[3] as i32
    }
    pub fn speed(&self) -> i32 {
        self.ranks[4] as i32
    }
    pub fn accuracy(&self) -> i32 {
        self.ranks[5] as i32
    }
    pub fn evasion(&self) -> i32 {
        self.ranks[6] as i32
    }
    pub fn critical(&self) -> i32 {
        self.ranks[7] as i32
    }
}
