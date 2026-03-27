#[derive(Debug)]
pub struct CarRace {
    pub name: String,
    pub laps: Vec<i32>
}

impl CarRace {
    // No receiver, a static method
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            laps: Vec::new()
        }
    }

    // Exclusive borrowed read-write access to self
    pub fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // Shared and read-only borrowed access to self
    pub fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    // Exclusive ownership of self
    pub fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}
