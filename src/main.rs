use rand::Rng;

const TIMESTAMPS_COUNT: usize = 50000;

const PROBABILITY_SCORE_CHANGED: f64 = 0.0001;

const PROBABILITY_HOME_SCORE: f64 = 0.45;

const OFFSET_MAX_STEP: i32 = 3;

const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

#[derive(Debug, Clone, Copy)]
struct Score {
    home: i32,
    away: i32,
}

#[derive(Debug, Clone, Copy)]
struct Stamp {
    offset: i32,
    score: Score,
}

fn generate_stamp(previous_value: Stamp) -> Stamp {
    let score_changed: bool = rand::thread_rng().gen_bool(PROBABILITY_SCORE_CHANGED);
    let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);
    let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);

    Stamp {
        offset: previous_value.offset + offset_change,
        score: Score {
            home: previous_value.score.home + if score_changed && home_score_change { 1 } else { 0 },
            away: previous_value.score.away + if score_changed && !home_score_change { 1 } else { 0 },
        },
    }
}

fn generate_game() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}


fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {

    if game_stamps.len() >= offset as usize || offset < 0  {
        let score = game_stamps[offset as usize];
        (score.score.home, score.score.away) 
    } else {
        unimplemented!("Out of bounds offset value is  '{}'", offset);
    }
    
}


#[cfg(test)]
mod tests {
use crate::*;
    #[test]
    fn bounds() {
        let game = generate_game();

        println!("{:?}", get_score(&game, 50000));
        println!("{:?}", get_score(&game, 0));
    }

    #[test]
    fn out_bounds() {
        let game = generate_game();

        println!("{:?}", get_score(&game, 50001));
        println!("{:?}", get_score(&game, -1));
    }

    #[test]
    fn rand() {
        let game = generate_game();

        println!("{:?}", get_score(&game, rand::random::<i32>()));
    }
}