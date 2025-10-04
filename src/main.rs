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
fn main() {}
