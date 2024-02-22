#[derive(Debug, Default, Copy, Clone)]
pub struct Keccak256Record {
    pub count: u64,
    pub time_cycles: u64,
}

impl Keccak256Record {
    pub fn new() -> Self {
        Self { count: 0, time_cycles: 0 }
    }

    pub fn add_other(&mut self, other: Self) {
        self.count = self.count.checked_add(other.count).expect("overflow");
        self.time_cycles = self.time_cycles.checked_add(other.time_cycles).expect("overvflow");
    }

    pub fn add(&mut self, count: u64, time_cycles: u64) {
        self.count = self.count.checked_add(count).expect("overflow");
        self.time_cycles = self.time_cycles.checked_add(time_cycles).expect("overflow");
    }

    pub fn checked_add(&mut self, rhs: Self) -> Option<Self>{
        self.count = self.count.checked_add(rhs.count).expect("overflow");
        self.time_cycles = self.time_cycles.checked_add(rhs.time_cycles).expect("overvflow");

        Some(self.clone())
    }
}
