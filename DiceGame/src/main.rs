use rand::Rng;
use std::io;

struct Player {
    name: String,
    points: i32,
}

fn create_player(name: &String) -> Player {
    Player {
        name: name.to_string(),
        points: 0,
    }
}

fn before_game() {
    let mut players: Vec<Player> = Vec::new();

    loop {
        println!("1. Add new player, 2. Start game");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Error");

        let command: u32 = match command.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Failed"),
        };

        match command {
            1 => {
                let mut name: String = String::new();
                println!("Enter name of the player: ");
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let new_player: Player = create_player(&name);
                players.push(new_player);
                println!("Player created");
                println!("Either create more players or start the game");
                continue;
            }
            2 => {
                if players.len() < 2 {
                    println!("You have to register at least 2 players");
                    continue;
                }
                break;
            }
            _ => {
                println!("Wrong input enter either 1 or 2");
                continue;
            }
        };
    }
    start_game(&mut players);
}

fn start_game(players: &mut Vec<Player>) {
    println!("Starting game...");

    for n in players.iter_mut() {
        println!("It's {}s turn", n.name);

        'inner: loop {
            println!("Input 1 to throw the dice");

            let mut command = String::new();

            io::stdin()
                .read_line(&mut command)
                .expect("Failed to read line");

            let command: u32 = match command.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You need to input a number");
                    continue 'inner;
                }
            };
            match command {
                1 => break 'inner,
                _ => {
                    println!("You need to input 1");
                    continue;
                }
            };
        }

        let random_num: i32 = rand::thread_rng().gen_range(1..=6);

        println!("{} threw number {}", n.name, random_num);

        n.points = random_num;
    }

    let mut winner: String = String::new();
    let mut most_points: i32 = 0;
    for i in players.iter_mut() {
        if i.points > most_points {
            most_points = i.points;
            winner = i.name.clone();
        }
    }

    println!("WINNER IS {} CONGRATS!!!", winner);
}

fn main() {
    before_game();
}
