// Week 3: Custom Types - Advanced Patterns
// Combine structs, enums, and type aliases to build complex types

use std::collections::HashMap;

// TODO 1: Type aliases for clarity
type UserId = u32;
type Username = String;
type Score = i32;

// TODO 2: Newtype pattern for type safety
struct Kilometers(f64);
struct Miles(f64);

impl Kilometers {
    fn to_miles(&self) -> Miles {
        Miles(self.0 * 0.621371)
    }
}

impl Miles {
    fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.0 * 1.60934)
    }
}

// TODO 3: Complex enum representing a game state
#[derive(Debug)]
enum GameState {
    Menu,
    Playing {
        level: u32,
        score: Score,
        player: Player,
    },
    Paused {
        level: u32,
        score: Score,
    },
    GameOver {
        final_score: Score,
        high_score: Score,
    },
}

// TODO 4: Struct with generic lifetime
#[derive(Debug, Clone)]
struct Player {
    id: UserId,
    name: Username,
    health: u32,
    inventory: Vec<Item>,
}

// TODO 5: Enum for inventory items
#[derive(Debug, Clone)]
enum Item {
    Weapon { name: String, damage: u32 },
    Armor { name: String, defense: u32 },
    Consumable { name: String, effect: Effect },
}

#[derive(Debug, Clone)]
enum Effect {
    Heal(u32),
    Boost(StatBoost),
    Teleport,
}

#[derive(Debug, Clone)]
struct StatBoost {
    stat: String,
    amount: i32,
    duration: u32,
}

// TODO 6: Implementing Display trait for custom formatting
use std::fmt;

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Player {} (ID: {}) - Health: {} - Items: {}",
            self.name,
            self.id,
            self.health,
            self.inventory.len()
        )
    }
}

// TODO 7: State machine with methods
impl GameState {
    fn transition(&mut self, event: GameEvent) {
        *self = match (self.clone(), event) {
            (GameState::Menu, GameEvent::StartGame(player)) => GameState::Playing {
                level: 1,
                score: 0,
                player,
            },
            (GameState::Playing { level, score, player }, GameEvent::Pause) => {
                GameState::Paused { level, score }
            }
            (GameState::Paused { level, score }, GameEvent::Resume(player)) => {
                GameState::Playing { level, score, player }
            }
            (GameState::Playing { score, .. }, GameEvent::EndGame(high_score)) => {
                GameState::GameOver {
                    final_score: score,
                    high_score,
                }
            }
            (state, _) => state, // Invalid transition, keep current state
        };
    }
}

// Make GameState cloneable for the transition method
impl Clone for GameState {
    fn clone(&self) -> Self {
        match self {
            GameState::Menu => GameState::Menu,
            GameState::Playing { level, score, player } => GameState::Playing {
                level: *level,
                score: *score,
                player: Player {
                    id: player.id,
                    name: player.name.clone(),
                    health: player.health,
                    inventory: player.inventory.clone(),
                },
            },
            GameState::Paused { level, score } => GameState::Paused {
                level: *level,
                score: *score,
            },
            GameState::GameOver { final_score, high_score } => GameState::GameOver {
                final_score: *final_score,
                high_score: *high_score,
            },
        }
    }
}

#[derive(Debug)]
enum GameEvent {
    StartGame(Player),
    Pause,
    Resume(Player),
    EndGame(Score),
}

// TODO 8: Generic struct
struct Container<T> {
    items: Vec<T>,
    capacity: usize,
}

impl<T> Container<T> {
    fn new(capacity: usize) -> Self {
        Container {
            items: Vec::new(),
            capacity,
        }
    }
    
    fn add(&mut self, item: T) -> Result<(), String> {
        if self.items.len() >= self.capacity {
            Err(String::from("Container is full"))
        } else {
            self.items.push(item);
            Ok(())
        }
    }
    
    fn remove(&mut self) -> Option<T> {
        self.items.pop()
    }
}

// TODO 9: Recursive enum (for tree structures)
#[derive(Debug)]
enum Expression {
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn evaluate(&self) -> f64 {
        match self {
            Expression::Number(n) => *n,
            Expression::Add(left, right) => left.evaluate() + right.evaluate(),
            Expression::Multiply(left, right) => left.evaluate() * right.evaluate(),
        }
    }
}

// TODO 10: Associated constants
struct Config;

impl Config {
    const MAX_HEALTH: u32 = 100;
    const DEFAULT_LEVEL: u32 = 1;
    const INITIAL_LIVES: u32 = 3;
}

fn main() {
    // TODO 11: Using type aliases
    let user_scores: HashMap<UserId, Score> = HashMap::new();
    println!("Created user scores map");
    
    // TODO 12: Newtype pattern
    let distance_km = Kilometers(42.0);
    let distance_mi = distance_km.to_miles();
    println!("Marathon distance: {} km = {} miles", distance_km.0, distance_mi.0);
    
    // TODO 13: Creating complex game state
    let player = Player {
        id: 1,
        name: String::from("Hero"),
        health: Config::MAX_HEALTH,
        inventory: vec![
            Item::Weapon {
                name: String::from("Sword"),
                damage: 25,
            },
            Item::Armor {
                name: String::from("Shield"),
                defense: 15,
            },
            Item::Consumable {
                name: String::from("Health Potion"),
                effect: Effect::Heal(50),
            },
        ],
    };
    
    println!("{}", player);
    
    // TODO 14: State transitions
    let mut game = GameState::Menu;
    println!("Game state: {:?}", game);
    
    game.transition(GameEvent::StartGame(player.clone()));
    println!("Game state after start: {:?}", game);
    
    game.transition(GameEvent::Pause);
    println!("Game state after pause: {:?}", game);
    
    // TODO 15: Generic container
    let mut item_container: Container<Item> = Container::new(5);
    
    let sword = Item::Weapon {
        name: String::from("Magic Sword"),
        damage: 50,
    };
    
    match item_container.add(sword) {
        Ok(()) => println!("Added item to container"),
        Err(e) => println!("Error: {}", e),
    }
    
    // TODO 16: Expression tree evaluation
    let expr = Expression::Add(
        Box::new(Expression::Multiply(
            Box::new(Expression::Number(3.0)),
            Box::new(Expression::Number(4.0)),
        )),
        Box::new(Expression::Number(5.0)),
    );
    
    println!("Expression: (3 * 4) + 5 = {}", expr.evaluate());
    
    // TODO 17: Pattern matching on complex types
    let current_item = Item::Consumable {
        name: String::from("Speed Boost"),
        effect: Effect::Boost(StatBoost {
            stat: String::from("Speed"),
            amount: 10,
            duration: 30,
        }),
    };
    
    match current_item {
        Item::Weapon { name, damage } => {
            println!("Weapon: {} (Damage: {})", name, damage);
        }
        Item::Armor { name, defense } => {
            println!("Armor: {} (Defense: {})", name, defense);
        }
        Item::Consumable { name, effect } => {
            match effect {
                Effect::Heal(amount) => println!("{}: Heals {} HP", name, amount),
                Effect::Boost(boost) => {
                    println!(
                        "{}: Boosts {} by {} for {} seconds",
                        name, boost.stat, boost.amount, boost.duration
                    );
                }
                Effect::Teleport => println!("{}: Teleports player", name),
            }
        }
    }
    
    // TODO 18: Using associated constants
    println!("Game Configuration:");
    println!("  Max Health: {}", Config::MAX_HEALTH);
    println!("  Default Level: {}", Config::DEFAULT_LEVEL);
    println!("  Initial Lives: {}", Config::INITIAL_LIVES);
}
