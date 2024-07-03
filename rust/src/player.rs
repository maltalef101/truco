use godot::{engine::Engine, prelude::*};

#[derive(Debug, Eq, PartialEq)]
pub enum Carta {
    Basto(u8),
    Espada(u8),
    Copa(u8),
    Oro(u8),
}

impl Carta {
    fn new(card_type: Self) -> Self {
        match card_type {
            Self::Basto(num) => {
                match num {
                    1 => Self::Basto(1),
                    2 => Self::Basto(2),
                    3 => Self::Basto(3),
                    4 => Self::Basto(4),
                    5 => Self::Basto(5),
                    6 => Self::Basto(6),
                    7 => Self::Basto(7),
                    10 => Self::Basto(10),
                    11 => Self::Basto(11),
                    12 => Self::Basto(12),
                    _ => panic!("invalid value!"),
                }
            },
            Self::Espada(num) => {
                match num {
                    1 => Self::Espada(1),
                    2 => Self::Espada(2),
                    3 => Self::Espada(3),
                    4 => Self::Espada(4),
                    5 => Self::Espada(5),
                    6 => Self::Espada(6),
                    7 => Self::Espada(7),
                    10 => Self::Espada(10),
                    11 => Self::Espada(11),
                    12 => Self::Espada(12),
                    _ => panic!("invalid value!"),
                }
            },
            Self::Copa(num) => {
                match num {
                    1 => Self::Copa(1),
                    2 => Self::Copa(2),
                    3 => Self::Copa(3),
                    4 => Self::Copa(4),
                    5 => Self::Copa(5),
                    6 => Self::Copa(6),
                    7 => Self::Copa(7),
                    10 => Self::Copa(10),
                    11 => Self::Copa(11),
                    12 => Self::Copa(12),
                    _ => panic!("invalid value!"),
                }
            },
            Self::Oro(num) => {
                match num {
                    1 => Self::Oro(1),
                    2 => Self::Oro(2),
                    3 => Self::Oro(3),
                    4 => Self::Oro(4),
                    5 => Self::Oro(5),
                    6 => Self::Oro(6),
                    7 => Self::Oro(7),
                    10 => Self::Oro(10),
                    11 => Self::Oro(11),
                    12 => Self::Oro(12),
                    _ => panic!("invalid value!"),
                }
            }
        }
    }

    fn rank(&self) -> u8 {
        match self {
            Carta::Espada(1) => 14,
            Carta::Basto(1) => 13,
            Carta::Espada(7) => 12,
            Carta::Oro(7) => 11,
            Carta::Basto(3) | Carta::Copa(3) | Carta::Espada(3) | Carta::Oro(3) => 10,
            Carta::Basto(2) | Carta::Copa(2) | Carta::Espada(2) | Carta::Oro(2) => 9,
            Carta::Oro(1) | Carta::Copa(1) => 8,
            Carta::Basto(12) | Carta::Copa(12) | Carta::Espada(12) | Carta::Oro(12) => 7,
            Carta::Basto(11) | Carta::Copa(11) | Carta::Espada(11) | Carta::Oro(11) => 6,
            Carta::Basto(10) | Carta::Copa(10) | Carta::Espada(10) | Carta::Oro(10) => 5,
            Carta::Copa(7) | Carta::Basto(7) => 4,
            Carta::Basto(6) | Carta::Copa(6) | Carta::Espada(6) | Carta::Oro(6) => 3,
            Carta::Basto(5) | Carta::Copa(5) | Carta::Espada(5) | Carta::Oro(5) => 2,
            Carta::Basto(4) | Carta::Copa(4) | Carta::Espada(4) | Carta::Oro(4) => 1,
            _ => 0,
        }
    }
}

impl PartialOrd for Carta {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Carta {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Player {
    name: String,
    points: u8,
    hand: Option<[Carta; 3]>,

    base: Base<Node2D>
}

#[godot_api]
impl INode2D for Player {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            name: "asdf".into(),
            points: 0,
            // FIXME: asignar cartas aleatorias
            hand: None,

            base
        }
    }
}

#[godot_api]
impl Player {
    fn deal_cards(&mut self) {
    }
}
