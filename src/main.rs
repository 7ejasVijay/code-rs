mod methods;

use methods::CarRace;

fn main() {
    let mut race = CarRace::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(86);
    race.print_laps();
    race.add_lap(100);
    race.print_laps();
    race.finish();
}
