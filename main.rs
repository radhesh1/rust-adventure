// Import necessary libraries
use std::io::{self, Write};

// Define a custom error type
#[derive(Debug)]
enum AdventureError {
    NotFound,
}

// Define a Result type for our custom error
type Result<T> = std::result::Result<T, AdventureError>;

// Define a simple item struct
struct Item {
    name: String,
    description: String,
}

// Implement the item
impl Item {
    fn new(name: &str, description: &str) -> Self {
        Item {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

// Define a simple room struct
struct Room {
    name: String,
    description: String,
    item: Option<Item>,
    locked: bool,
}

// Implement the room
impl Room {
    fn new(name: &str, description: &str, item: Option<Item>, locked: bool) -> Self {
        Room {
            name: name.to_string(),
            description: description.to_string(),
            item,
            locked,
        }
    }

    fn unlock(&mut self) {
        self.locked = false;
    }
}

// Define the main adventure struct
struct Adventure {
    rooms: Vec<Room>,
    current_room: usize,
}

// Implement the adventure
impl Adventure {
    fn new() -> Self {
        let room1 = Room::new("Start", "You are at the start of your adventure.", None, false);
        let room2 = Room::new(
            "Locked Room",
            "This room is locked. Find a key!",
            Some(Item::new("Key", "A shiny golden key.")),
            true,
        );

        Adventure {
            rooms: vec![room1, room2],
            current_room: 0,
        }
    }

    fn go(&mut self, direction: &str) -> Result<()> {
        match direction {
            "north" => {
                if self.current_room == 0 {
                    self.current_room = 1;
                    Ok(())
                } else {
                    Err(AdventureError::NotFound)
                }
            }
            "south" => {
                if self.current_room == 1 {
                    self.current_room = 0;
                    Ok(())
                } else {
                    Err(AdventureError::NotFound)
                }
            }
            _ => Err(AdventureError::NotFound),
        }
    }

    fn look_around(&self) -> &str {
        &self.rooms[self.current_room].description
    }

    fn take_item(&mut self) -> Result<Item> {
        if let Some(item) = self.rooms[self.current_room].item.take() {
            Ok(item)
        } else {
            Err(AdventureError::NotFound)
        }
    }

    fn use_item(&mut self, item: Item) -> Result<()> {
        if self.current_room == 1 && item.name == "Key" {
            self.rooms[1].unlock();
            Ok(())
        } else {
            Err(AdventureError::NotFound)
        }
    }
}

fn main() {
    let horse = r"
                   _____,,;;;`; 
                ,~(  )  , )~~\|  
                ' / / --`--,     
                 /  \    | '  
 ";
    println!("{}", horse);
    let mut adventure = Adventure::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "look" => println!("{}", adventure.look_around()),
            "north" | "south" => {
                if let Err(err) = adventure.go(&input) {
                    match err {
                        AdventureError::NotFound => println!("You can't go that way!"),
                        _ => (),
                    }
                }
            }
            "take" => {
                if let Ok(item) = adventure.take_item() {
                    println!("You've taken the {}", item.name);
                } else {
                    println!("No item to take here!");
                }
            }
            "use key" => {
                let key = Item::new("Key", "A shiny golden key.");
                if let Err(err) = adventure.use_item(key) {
                    match err {
                        AdventureError::NotFound => println!("You can't use that here!"),
                        _ => (),
                    }
                } else {
                    println!("You've unlocked the room!");
                }
            }
            "quit" => break,
            _ => println!("Invalid command!"),
        }
    }
}
