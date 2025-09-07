use std::fmt;

use crate::schotten_totten_2::com_st_proto::{self, CardProto};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    Gray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card {
    pub value: u8,
    pub color: Color,
}

impl Card {
    pub fn to_proto(&self) -> com_st_proto::CardProto {
        let color = match self.color {
            Color::Red => com_st_proto::ColorProto::Red.into(),
            Color::Blue => com_st_proto::ColorProto::Blue.into(),
            Color::Yellow => com_st_proto::ColorProto::Yellow.into(),
            Color::Green => com_st_proto::ColorProto::Green.into(),
            Color::Gray => com_st_proto::ColorProto::Gray.into(),
        };
        com_st_proto::CardProto {
            color: Some(color),
            value: Some(self.value.into()),
        }
    }

    pub fn from_proto(proto: &com_st_proto::CardProto) -> Card {
        if let (Some(color), Some(value)) = (proto.color, proto.value) {
            match com_st_proto::ColorProto::try_from(color) {
                Ok(com_st_proto::ColorProto::Red) => {
                    return Card {
                        color: Color::Red,
                        value: value as u8,
                    };
                }
                Ok(com_st_proto::ColorProto::Blue) => {
                    return Card {
                        color: Color::Blue,
                        value: value as u8,
                    };
                }
                Ok(com_st_proto::ColorProto::Yellow) => {
                    return Card {
                        color: Color::Yellow,
                        value: value as u8,
                    };
                }
                Ok(com_st_proto::ColorProto::Green) => {
                    return Card {
                        color: Color::Green,
                        value: value as u8,
                    };
                }
                Ok(com_st_proto::ColorProto::Gray) => {
                    return Card {
                        color: Color::Gray,
                        value: value as u8,
                    };
                }
                _ => panic!("Unknown color"),
            }
        }
        panic!("missing field in CardProto");
    }

    pub fn from_proto_array(proto_array: &[CardProto]) -> Vec<Card> {
        let mut cards = vec![];
        for card_proto in proto_array {
            cards.push(Card::from_proto(card_proto));
        }
        cards
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {:?})", self.value, self.color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_to_and_from_proto_red() {
        let original_card = Card {
            value: 5,
            color: Color::Red,
        };
        let proto_card = original_card.to_proto();
        let new_card = Card::from_proto(&proto_card);
        assert_eq!(original_card, new_card);
    }

    #[test]
    fn test_card_to_and_from_proto_blue() {
        let original_card = Card {
            value: 10,
            color: Color::Blue,
        };
        let proto_card = original_card.to_proto();
        let new_card = Card::from_proto(&proto_card);
        assert_eq!(original_card, new_card);
    }

    #[test]
    fn test_card_to_and_from_proto_yellow() {
        let original_card = Card {
            value: 0,
            color: Color::Yellow,
        };
        let proto_card = original_card.to_proto();
        let new_card = Card::from_proto(&proto_card);
        assert_eq!(original_card, new_card);
    }

    #[test]
    fn test_card_to_and_from_proto_green() {
        let original_card = Card {
            value: 1,
            color: Color::Green,
        };
        let proto_card = original_card.to_proto();
        let new_card = Card::from_proto(&proto_card);
        assert_eq!(original_card, new_card);
    }

    #[test]
    fn test_card_to_and_from_proto_gray() {
        let original_card = Card {
            value: 11,
            color: Color::Gray,
        };
        let proto_card = original_card.to_proto();
        let new_card = Card::from_proto(&proto_card);
        assert_eq!(original_card, new_card);
    }

    #[test]
    fn test_card_from_proto_array() {
        let mut proto_array = Vec::new();
        proto_array.push(Card{value: 11, color: Color::Blue}.to_proto());
        proto_array.push(Card{value: 8, color: Color::Yellow}.to_proto());
        proto_array.push(Card{value: 0, color: Color::Red}.to_proto());

        let cards = Card::from_proto_array(&proto_array);
        assert_eq!(cards.len(), 3);
        assert_eq!(cards[0].value, 11);
        assert_eq!(cards[0].color, Color::Blue);
        assert_eq!(cards[2].value, 0);
        assert_eq!(cards[2].color, Color::Red);
    }
}
