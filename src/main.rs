use std::io;
enum Room {
    Entrance,
    Hallway,
    Kitchen,
    Library,
    SecretChamber,
}
fn describe_room(room: &Room) -> &str {
    match room {
        Room::Entrance => "You are at the entrance of a dark and dusty castle.",
        Room::Hallway => {
            "You've entered a long hallway. There are doors to the kitchen and the library."
        }
        Room::Kitchen => "You're in the kitchen. There's a rusty old stove and a table.",
        Room::Library => "You're in the library. Bookshelves line the walls.",
        Room::SecretChamber => "You've found a secret chamber! There's a treasure chest here.",
    }
}
enum Command {
    Go(String),
    Look,
    Quit,
}

fn handle_command(command: Command, current_room: &mut Room) {
    match command {
        Command::Go(direction) => {
            // Logic to change the room based on the direction
            println!("You go {}", direction);
        }
        Command::Look => {
            println!("{}", describe_room(current_room));
        }
        Command::Quit => {
            println!("Goodbye!");
            // Logic to exit the game
        }
    }
}
fn main() {
    let mut current_room = Room::Entrance;

    loop {
        println!("{}", describe_room(&current_room));
        println!("What do you do?");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        let argument = parts.next().unwrap_or("").to_string();

        let command = match command {
            "go" => Command::Go(argument),
            "look" => Command::Look,
            "quit" => Command::Quit,
            _ => {
                println!("I don't understand that command.");
                continue;
            }
        };

        match command {
            Command::Go(direction) => {
                let next_room = match (&current_room, direction.as_str()) {
                    (Room::Entrance, "north") => Some(Room::Hallway),
                    (Room::Hallway, "south") => Some(Room::Entrance),
                    (Room::Hallway, "east") => Some(Room::Kitchen),
                    (Room::Hallway, "west") => Some(Room::Library),
                    (Room::Kitchen, "west") => Some(Room::Hallway),
                    (Room::Library, "east") => Some(Room::Hallway),
                    (Room::Library, "north") => Some(Room::SecretChamber),
                    _ => None,
                };

                if let Some(room) = next_room {
                    current_room = room;
                } else {
                    println!("You can't go that way.");
                }
            }
            Command::Look => {
                // The room description is already printed at the start of the loop.
            }
            Command::Quit => {
                println!("Goodbye!");
                break;
            }
        }
    }
}
