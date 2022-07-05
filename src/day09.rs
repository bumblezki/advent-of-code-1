use id_tree::*;

pub fn day09(input_lines: &[Vec<String>]) -> (String, String) {
    nice_input(input_lines);
    let answer1 = 0;
    let answer2 = 0;
    (format!("{:#?}", answer1), format!("{}", answer2))
}

// Do we need `i32` for the marble values and scores? All the values are positive so unsigned integer types should work, e.g., `u32`.

struct Board {
    // What is the `centre_marble`? The value of the centre marble and it's index in the `order`?
    // Why not have two fields in this struct?
    // centre_marble_value: i32
    // centre_marble_index: usize
    // In fact, do you ever even use the value of the centre marble? Do you just need to keep track of it's index?
    centre_marble: (i32, i32),
    order: Vec<i32>
}

impl Board {
    fn new() -> Board {
        Board {
            centre_marble: (0, 0),
            order: vec!(0)
        }
    }
}

// You don't ever actually use `input_lines`, right? Perhaps it's needed for the framework though.
fn nice_input(input_lines: &[Vec<String>]) {
    let marble_count = 7105800;
    
    // You only ever use player_count to index the vec, right?
    // Why not `let player_count: usize = 491;`?
    // That way you don't need to do the type casting.
    let player_count: i32 = 491;
    let mut scores_on_the_doors: Vec<i32> = vec![0; player_count as usize];
    let mut board: Board = Board::new();
    
    // `let mut turn: usize = 1;`?
    let mut turn = 1;
    // Is this a bug? `marble_count` won't be included in the list, so if it's a multiple of 23 it won't be added to the score. `1..=marble_count`?
    for marble in 1..marble_count {
        if marble%23 ==0 {
            scores_on_the_doors[(turn%player_count) as usize] += marble;
            if board.centre_marble.0 >= 7 {
                let index = board.centre_marble.0 -7;
                let number = board.order[index as usize];
                scores_on_the_doors[(turn%player_count) as usize] += number;
                board.order.remove(index as usize);
                board.centre_marble = (index, number);
            } else {
                // Lots of type casting here that could be avoided by using more suitable types from the outset.
                let index = board.order.len() as i32  + (board.centre_marble.0-7);
                let number = board.order[index as usize];
                scores_on_the_doors[(turn%player_count) as usize] += number;
                // O(n) to remove from a Vec. https://doc.rust-lang.org/std/collections/index.html#sequences
                board.order.remove(index as usize);
                board.centre_marble = (index, number);
            }

        }
        else {
            // You have the `Board` struct, but there are no methods implemented in it.
            // You could consider if any of the logic below could be a method on board.
            // [-] (0)                      case 1
            // [1]  0 (1)                   case 1
            // [2]  0 (2) 1                 case 3
            // [3]  0  2  1 (3)             case 1
            // [4]  0 (4) 2  1  3           case 3
            // [5]  0  4  2 (5) 1  3        case 3
            
            // Coould use a match statement here if you like.
            if board.centre_marble.0 as usize + 1 == board.order.len() {  // case 1
                // O(n) to insert into a Vec. https://doc.rust-lang.org/std/collections/index.html#sequences
                board.order.insert(1, marble);
                board.centre_marble = (1, marble)
            }
            // Does this case ever get hit? E.g., if `board.order.len() == 5` and the centre marble is at the RHS, it's index is 4.
            else if board.centre_marble.0 as usize == board.order.len() { // case 2
                board.order.push(marble);
                board.centre_marble = (board.order.len() as i32 - 1, marble)
            }
            else {  // case 3
                board.order.insert(board.centre_marble.0 as usize + 2, marble);
                board.centre_marble = (board.centre_marble.0 + 2, marble)
            }  
                   
        }
        // Is turn redundant? Could you just use the `marble` to keep track of the turns?
        turn += 1;  
    }
    println!("here's the max score: {:#?}", scores_on_the_doors.iter().max())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score() {
        assert_eq!(
            32,
            max_score(Rules {
                players: 9,
                marbles: 25
            }),
        );
        assert_eq!(
            8317,
            max_score(Rules {
                players: 10,
                marbles: 1618
            }),
        );
        assert_eq!(
            146373,
            max_score(Rules {
                players: 13,
                marbles: 7999
            }),
        );
        assert_eq!(
            2764,
            max_score(Rules {
                players: 17,
                marbles: 1104
            }),
        );
        assert_eq!(
            54718,
            max_score(Rules {
                players: 21,
                marbles: 6111
            }),
        );
        assert_eq!(
            37305,
            max_score(Rules {
                players: 30,
                marbles: 5807
            }),
        );
    }
}
