
#[derive(Debug, Clone)]
struct Club {
    name: String,
    attack: i32,
    defence: i32,
    played: u32,
    won: u32,
    drawn: u32,
    lost: u32,
    goals_for: u32,
    goals_against: u32,
    points: u32,
}

impl Club {
    fn new(name: &str, attack: i32, defence: i32) -> Self {
        Self {
            name: name.to_string(),
            attack,
            defence,
            played: 0,
            won: 0,
            drawn: 0,
            lost: 0,
            goals_for: 0,
            goals_against: 0,
            points: 0,
        }
    }

    fn goal_difference(&self) -> i32 {
        self.goals_for as i32 - self.goals_against as i32
    }
}

// A temporary, deterministic match simulation.
// Later we'll simulate actual players, tactics and events.
fn simulate_match(home: &Club, away: &Club) -> (u32, u32) {
    let home_goals =
        (2 + (home.attack - away.defence) / 15).clamp(0, 5);

    let away_goals =
        (1 + (away.attack - home.defence) / 15).clamp(0, 5);

    (home_goals as u32, away_goals as u32)
}

fn record_result(club: &mut Club, scored: u32, conceded: u32) {
    club.played += 1;
    club.goals_for += scored;
    club.goals_against += conceded;

    if scored > conceded {
        club.won += 1;
        club.points += 3;
    } else if scored == conceded {
        club.drawn += 1;
        club.points += 1;
    } else {
        club.lost += 1;
    }
}

fn play_match(clubs: &mut [Club], home_id: usize, away_id: usize) {
    let (home_goals, away_goals) =
        simulate_match(&clubs[home_id], &clubs[away_id]);

    println!(
        "{} {} - {} {}",
        clubs[home_id].name,
        home_goals,
        away_goals,
        clubs[away_id].name
    );

    record_result(&mut clubs[home_id], home_goals, away_goals);
    record_result(&mut clubs[away_id], away_goals, home_goals);
}

fn print_league_table(clubs: &[Club]) {
    let mut table = clubs.to_vec();

    table.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then(b.goal_difference().cmp(&a.goal_difference()))
            .then(b.goals_for.cmp(&a.goals_for))
            .then(a.name.cmp(&b.name))
    });

    println!("\nLEAGUE TABLE");
    println!("{:<18} {:>3} {:>3} {:>3} {:>3} {:>4} {:>4} {:>4}",
        "Club", "P", "W", "D", "L", "GF", "GA", "Pts"
    );

    for club in &table {
        println!(
            "{:<18} {:>3} {:>3} {:>3} {:>3} {:>4} {:>4} {:>4}",
            club.name,
            club.played,
            club.won,
            club.drawn,
            club.lost,
            club.goals_for,
            club.goals_against,
            club.points
        );
    }
}

fn main() {
    println!("FOOTBALL MANAGER");
    println!("Season 2002/03\n");

    let mut clubs = vec![
        Club::new("Northbridge FC", 85, 80),
        Club::new("Eastport United", 77, 74),
        Club::new("Westford City", 72, 78),
        Club::new("Southvale Town", 65, 68),
    ];

    println!("MATCH DAY 1");
    play_match(&mut clubs, 0, 1);
    play_match(&mut clubs, 2, 3);

    print_league_table(&clubs);
}