use std::fs;
struct Field {
    value: u16,
    drawn: bool,
}

struct Board {
    fields: Vec<Field>,
    won: bool,
    score: u32,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (p, i) in self.fields.iter().enumerate() {
            if p % 5 == 0 {
                write!(f, "\n").ok();
            }
            if i.drawn {
                write!(f, "[{:2}] ", i.value).ok();
            } else {
                write!(f, " {:2}  ", i.value).ok();
            }
        }
        writeln!(f, "")
    }
}

impl Board {
    fn mark_number(&mut self, draw: u16) -> Option<u32> {
        if self.won {
            return None;
        }
        
        let field = self.fields
            .iter_mut()
            .find(|f| f.value == draw);

        if field.is_none() {
            return None
        }

        field.unwrap().drawn = true;
        // check if won
        for i in 0..5 {
            // lines
            if self.fields.iter().skip(i * 5).take(5).all(|f| f.drawn) {
                self.won = true;
                break;
            }

            // columns
            if self.fields.iter().skip(i).step_by(5).all(|f| f.drawn) {
                self.won = true;
                break;
            }
        }

        // calc score
        if self.won {
            let undrawn_sum = self.fields.iter().filter(|f| !f.drawn).fold(0, |accm, f| accm + f.value );
            self.score = undrawn_sum as u32 * draw as u32;
            return Some(self.score);
        }

        return None;
    }
}

fn main() {
    let input = fs::read_to_string("./data/input").expect("Unable to read file");
    let draws = parse_draws(&input);
    println!("draws: {:?}", draws);

    let mut boards = parse_boards(&input);
    let mut last_board_won: usize = usize::MAX;
    let mut last_board_won_score: u32 = u32::MAX;

    for draw in draws.iter() {
        println!("draw: {:?}", draw);
        for (index, board) in boards.iter_mut().enumerate() {
            let score = board.mark_number(*draw);
            println!("board {}:\n{}", index, board);

            if score.is_some() {
                last_board_won_score = score.unwrap();
                last_board_won = index;
            }
        }
    }

    if last_board_won_score < u32::MAX {
        println!("board {} won last with score {}", last_board_won, last_board_won_score);
    } else {
        println!("only loosers here");
    }
}

fn parse_draws(data: &String) -> Vec<u16> {
    // maybe do split_whitespace as well here?
    return data
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u16>().unwrap())
        .collect();
}

fn parse_boards(data: &String) -> Vec<Board> {
    let words = data
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    let board_fields: Vec<Vec<u16>> = words.chunks(25).map(|c| c.to_vec()).collect();

    return board_fields
        .iter()
        .map(|bf| Board {
            fields: bf
                .iter()
                .map(|f| Field {
                    value: f.clone(),
                    drawn: false,
                })
                .collect(),
            won: false,
            score: 0,
        })
        .collect();
}
