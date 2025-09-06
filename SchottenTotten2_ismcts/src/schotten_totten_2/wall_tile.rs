use std::fmt;

use crate::schotten_totten_2::card::Card;
use crate::schotten_totten_2::com_st_proto;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum WallPattern {
    Color,
    Run,
    Equal,
    Plus,
    Minus,
    None,
}

impl WallPattern {
    fn from_proto(pattern: i32) -> WallPattern {
        match com_st_proto::WallPatternProto::try_from(pattern) {
            Ok(com_st_proto::WallPatternProto::Color) => WallPattern::Color,
            Ok(com_st_proto::WallPatternProto::Run) => WallPattern::Run,
            Ok(com_st_proto::WallPatternProto::Equals) => WallPattern::Equal,
            Ok(com_st_proto::WallPatternProto::Plus) => WallPattern::Plus,
            Ok(com_st_proto::WallPatternProto::Minus) => WallPattern::Minus,
            Ok(com_st_proto::WallPatternProto::NonePattern) => WallPattern::None,
            _ => panic!("unknown wall patern"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WallTile {
    pub id: usize,
    pub intact_length: usize,
    pub intact_wall_pattern: WallPattern,
    pub damaged_length: usize,
    pub damaged_wall_pattern: WallPattern,
    pub is_damaged: bool,
    pub is_damaged_twice: bool,
    pub attacker_cards: Vec<Card>,
    pub defender_cards: Vec<Card>,
}

impl WallTile {
    pub fn from_proto(proto: &com_st_proto::WallProto) -> WallTile {
        let (is_damaged, is_damaged_twice) = WallTile::get_status(proto.status.unwrap());

        WallTile {
            id: proto.wall_index.unwrap() as usize,
            intact_length: proto.intact_length.unwrap() as usize,
            intact_wall_pattern: WallPattern::from_proto(proto.intact_pattern.unwrap()),
            damaged_length: proto.damaged_length.unwrap() as usize,
            damaged_wall_pattern: WallPattern::from_proto(proto.damaged_pattern.unwrap()),
            is_damaged: is_damaged,
            is_damaged_twice: is_damaged_twice,
            attacker_cards: Card::from_proto_array(&proto.attacker_cards),
            defender_cards: Card::from_proto_array(&proto.defender_cards),
        }
    }

    /// returns (is_damaged, is_damaged_twice)
    fn get_status(status: i32) -> (bool, bool) {
        match com_st_proto::StatusProto::try_from(status) {
            Ok(com_st_proto::StatusProto::Intact) => (false, false),
            Ok(com_st_proto::StatusProto::Damaged) => (true, false),
            Ok(com_st_proto::StatusProto::Broken) => (true, true),
            _ => panic!("unknown status"),
        }
    }

    pub fn get_length(&self) -> usize {
        if self.is_damaged {
            self.damaged_length
        } else {
            self.intact_length
        }
    }

    pub fn get_wall_pattern(&self) -> WallPattern {
        if self.is_damaged {
            self.damaged_wall_pattern
        } else {
            self.intact_wall_pattern
        }
    }
}

impl fmt::Display for WallTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tile {}: Pattern: {:?}, Attacker: {:?}, Defender: {:?}, isDamaged: {}",
            self.id,
            self.get_wall_pattern(),
            self.attacker_cards,
            self.defender_cards,
            self.is_damaged,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schotten_totten_2::card::Color;
    use crate::schotten_totten_2::com_st_proto::{self, CardProto, ColorProto};
    use crate::schotten_totten_2::com_st_proto::StatusProto;
    use crate::schotten_totten_2::com_st_proto::WallPatternProto;

    fn create_test_wall_proto() -> com_st_proto::WallProto {
        com_st_proto::WallProto {
            wall_index: Some(1),
            length: None,
            intact_length: Some(5),
            pattern: None,
            intact_pattern: Some(WallPatternProto::Color.into()),
            damaged_length: Some(3),
            damaged_pattern: Some(WallPatternProto::Run.into()),
            status: Some(StatusProto::Intact.into()),
            attacker_cards: vec![(CardProto {color: Some(ColorProto::Blue.into()), value: Some(8)})],
            defender_cards: vec![(CardProto {color: Some(ColorProto::Red.into()), value: Some(3)})],
            attacker_finished_first: Some(false),
        }
    }

    #[test]
    fn test_from_proto_intact() {
        let proto = create_test_wall_proto();
        let wall_tile = WallTile::from_proto(&proto);

        assert_eq!(wall_tile.id, 1);
        assert_eq!(wall_tile.intact_length, 5);
        assert_eq!(wall_tile.intact_wall_pattern, WallPattern::Color);
        assert_eq!(wall_tile.damaged_length, 3);
        assert_eq!(wall_tile.damaged_wall_pattern, WallPattern::Run);
        assert!(!wall_tile.is_damaged);
        assert!(!wall_tile.is_damaged_twice);
        assert_eq!(wall_tile.attacker_cards.len(), 1);
        assert_eq!(wall_tile.defender_cards.len(), 1);
        assert_eq!(wall_tile.attacker_cards[0].color, Color::Blue);
        assert_eq!(wall_tile.defender_cards[0].color, Color::Red);
    }

    #[test]
    fn test_from_proto_damaged() {
        let mut proto = create_test_wall_proto();
        proto.set_status(StatusProto::Damaged.into());
        let wall_tile = WallTile::from_proto(&proto);
        assert!(wall_tile.is_damaged);
        assert!(!wall_tile.is_damaged_twice);
    }

    #[test]
    fn test_from_proto_broken() {
        let mut proto = create_test_wall_proto();
        proto.set_status(StatusProto::Broken.into());
        let wall_tile = WallTile::from_proto(&proto);
        assert!(wall_tile.is_damaged);
        assert!(wall_tile.is_damaged_twice);
    }

    #[test]
    fn test_get_length_intact() {
        let proto = create_test_wall_proto();
        let wall_tile = WallTile::from_proto(&proto);
        assert_eq!(wall_tile.get_length(), 5);
    }

    #[test]
    fn test_get_length_damaged() {
        let mut proto = create_test_wall_proto();
        proto.set_status(StatusProto::Damaged.into());
        let wall_tile = WallTile::from_proto(&proto);
        assert_eq!(wall_tile.get_length(), 3);
    }

    #[test]
    fn test_get_wall_pattern_intact() {
        let proto = create_test_wall_proto();
        let wall_tile = WallTile::from_proto(&proto);
        assert_eq!(wall_tile.get_wall_pattern(), WallPattern::Color);
    }

    #[test]
    fn test_get_wall_pattern_damaged() {
        let mut proto = create_test_wall_proto();
        proto.set_status(StatusProto::Damaged.into());
        let wall_tile = WallTile::from_proto(&proto);
        assert_eq!(wall_tile.get_wall_pattern(), WallPattern::Run);
    }

    #[test]
    fn test_wall_pattern_from_proto() {
        assert_eq!(WallPattern::from_proto(WallPatternProto::Color.into()), WallPattern::Color);
        assert_eq!(WallPattern::from_proto(WallPatternProto::Run.into()), WallPattern::Run);
        assert_eq!(WallPattern::from_proto(WallPatternProto::Equals.into()), WallPattern::Equal);
        assert_eq!(WallPattern::from_proto(WallPatternProto::Plus.into()), WallPattern::Plus);
        assert_eq!(WallPattern::from_proto(WallPatternProto::Minus.into()), WallPattern::Minus);
        assert_eq!(WallPattern::from_proto(WallPatternProto::NonePattern.into()), WallPattern::None);
    }

    #[test]
    #[should_panic(expected = "unknown wall patern")]
    fn test_wall_pattern_from_proto_unknown_panic() {
        WallPattern::from_proto(99);
    }
}