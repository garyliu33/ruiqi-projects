use std::fmt;

use crate::schotten_totten_2::card::Card;
use crate::schotten_totten_2::com_st_proto;

#[derive(Debug, Clone, PartialEq)]
pub enum SchottenTotten2Move {
    PlayCard { card: Card, tile_index: usize },
    Retreat { tile_index: usize },
    ThrowOilCauldron { tile_index: usize },
}

impl SchottenTotten2Move {
    pub fn to_proto(&self) -> com_st_proto::ClientMoveProto {
        let (card_proto, tile_index) = match self {
            Self::PlayCard { card, tile_index } => (card.to_proto(), tile_index),
            Self::Retreat { tile_index } => (
                com_st_proto::CardProto {
                    color: Some(com_st_proto::ColorProto::Action.into()),
                    value: Some(-1),
                },
                tile_index,
            ),
            Self::ThrowOilCauldron { tile_index } => (
                com_st_proto::CardProto {
                    color: Some(com_st_proto::ColorProto::Action.into()),
                    value: Some(-2),
                },
                tile_index,
            ),
        };
        com_st_proto::ClientMoveProto {
            card: Some(card_proto),
            wall_index: Some(*tile_index as i32),
        }
    }
}

impl fmt::Display for SchottenTotten2Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchottenTotten2Move::PlayCard { card, tile_index } => {
                write!(f, "Play card {} on tile {}", card, tile_index)
            }
            SchottenTotten2Move::Retreat { tile_index } => {
                write!(f, "Retreat from tile {}", tile_index)
            }
            SchottenTotten2Move::ThrowOilCauldron { tile_index } => {
                write!(f, "Throw oil cauldron on tile {}", tile_index)
            }
        }
    }
}
