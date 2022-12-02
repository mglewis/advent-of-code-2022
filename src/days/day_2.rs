use std::str::FromStr;
use DesiredOutcome::{Draw, Loss, Win};
use Selection::{Paper, Rock, Scissors};

const WIN_POINTS: u32 = 6;
const DRAW_POINTS: u32 = 3;
const LOSS_POINTS: u32 = 0;

const WINNING_PLAYS: &[(Selection, Selection); 3] = &[
    (Paper, Rock), // players paper beats opponents rock
    (Rock, Scissors),
    (Scissors, Paper),
];

#[derive(Copy, Clone, PartialEq)]
enum Selection {
    Rock,
    Paper,
    Scissors,
}

impl Selection {
    fn bonus(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl FromStr for Selection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissors),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq)]
enum DesiredOutcome {
    Win,
    Loss,
    Draw,
}

impl FromStr for DesiredOutcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(()),
        }
    }
}

fn score_game(player: Selection, opponent: Selection) -> u32 {
    let result: u32;
    let bonus: u32 = player.bonus();

    if player == opponent {
        result = DRAW_POINTS;
    } else if WINNING_PLAYS.contains(&(player, opponent)) {
        result = WIN_POINTS;
    } else {
        result = LOSS_POINTS;
    }

    result + bonus
}

fn determine_selection(opponent: Selection, outcome: DesiredOutcome) -> Selection {
    if outcome == Draw {
        opponent
    } else if outcome == Win {
        WINNING_PLAYS
            .iter()
            .find(|(_, o)| o == &opponent)
            .unwrap()
            .0
    } else {
        WINNING_PLAYS
            .iter()
            .find(|(o, _)| o == &opponent)
            .unwrap()
            .1
    }
}

pub fn part_a(input: &str) -> u32 {
    let games = input.lines().collect::<Vec<&str>>();

    let game_scores: Vec<u32> = games
        .iter()
        .map(|x| {
            let player_str = &x[x.len() - 1..];
            let opponent_str = &x[..1];

            let player_selection = Selection::from_str(player_str).unwrap();
            let opponent_selection = Selection::from_str(opponent_str).unwrap();

            score_game(player_selection, opponent_selection)
        })
        .collect();

    game_scores.iter().sum()
}

pub fn part_b(input: &str) -> u32 {
    let games = input.lines().collect::<Vec<&str>>();

    let game_scores: Vec<u32> = games
        .iter()
        .map(|x| {
            let outcome_str = &x[x.len() - 1..];
            let opponent_str = &x[..1];

            let outcome = DesiredOutcome::from_str(outcome_str).unwrap();
            let opponent_selection = Selection::from_str(opponent_str).unwrap();

            let player_selection = determine_selection(opponent_selection, outcome);

            score_game(player_selection, opponent_selection)
        })
        .collect();

    game_scores.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_test_file;

    #[test]
    fn test_score_game_win() {
        assert_eq!(score_game(Rock, Scissors), 7);
        assert_eq!(score_game(Scissors, Paper), 9);
        assert_eq!(score_game(Paper, Rock), 8);
    }

    #[test]
    fn test_score_game_loss() {
        assert_eq!(score_game(Rock, Paper), 1);
        assert_eq!(score_game(Paper, Scissors), 2);
        assert_eq!(score_game(Scissors, Rock), 3);
    }

    #[test]
    fn test_score_game_draw() {
        assert_eq!(score_game(Rock, Rock), 4);
        assert_eq!(score_game(Paper, Paper), 5);
        assert_eq!(score_game(Scissors, Scissors), 6)
    }

    #[test]
    fn test_determine_selection() {
        assert!(determine_selection(Rock, Draw) == Rock);
        assert!(determine_selection(Paper, Win) == Scissors);
        assert!(determine_selection(Scissors, Loss) == Paper);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(2);
        assert_eq!(part_a(&input), 15);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(2);
        assert_eq!(part_b(&input), 12);
    }
}
