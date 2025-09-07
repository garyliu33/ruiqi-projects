use crate::schotten_totten_2::card::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WinningType {
    DamagedTwice,
    DamagedFourTiles,
    EmptyDeck,
    NoSpace,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Attacker,
    Defender,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum FormationType {
    Sum,
    Run,
    Color,
    SameStrength,
    ColorRun,
}

impl FormationType {
    // Logic to evaluate a formation based on the rules.
    pub fn evaluate_formation(cards: &[Card]) -> (Option<FormationType>, u32) {
        if cards.len() < 2 {
            return (None, 0);
        }

        let sum: u32 = cards.iter().map(|c| c.value as u32).sum();

        let first_color = cards[0].color;
        let same_color = cards.iter().all(|c| c.color == first_color);

        let first_strength = cards[0].value;
        let same_strength = cards.iter().all(|c| c.value == first_strength);

        let mut sorted_cards = cards.to_vec();
        sorted_cards.sort_by_key(|c| c.value);
        let mut is_run = true;
        for i in 0..sorted_cards.len() - 1 {
            if sorted_cards[i].value + 1 != sorted_cards[i + 1].value {
                is_run = false;
                break;
            }
        }

        if is_run && same_color {
            return (Some(FormationType::ColorRun), sum);
        }
        if same_strength {
            return (Some(FormationType::SameStrength), sum);
        }
        if same_color {
            return (Some(FormationType::Color), sum);
        }
        if is_run {
            return (Some(FormationType::Run), sum);
        }
        return (Some(FormationType::Sum), sum);
    }
}

#[cfg(test)]
mod tests {
    use crate::schotten_totten_2::card::Color;
    use super::*;

    #[test]
    fn test_evaluate_formation_less_than_two_cards() {
        let cards = vec![Card {
            value: 1,
            color: Color::Red,
        }];
        assert_eq!(FormationType::evaluate_formation(&cards), (None, 0));
    }

    #[test]
    fn test_evaluate_formation_color_run() {
        let cards = vec![
            Card {
                value: 3,
                color: Color::Red,
            },
            Card {
                value: 4,
                color: Color::Red,
            },
            Card {
                value: 5,
                color: Color::Red,
            },
        ];
        let (formation_type, sum) = FormationType::evaluate_formation(&cards);
        assert_eq!(formation_type, Some(FormationType::ColorRun));
        assert_eq!(sum, 12);
    }

    #[test]
    fn test_evaluate_formation_same_strength() {
        let cards = vec![
            Card {
                value: 7,
                color: Color::Red,
            },
            Card {
                value: 7,
                color: Color::Blue,
            },
            Card {
                value: 7,
                color: Color::Green,
            },
        ];
        let (formation_type, sum) = FormationType::evaluate_formation(&cards);
        assert_eq!(formation_type, Some(FormationType::SameStrength));
        assert_eq!(sum, 21);
    }

    #[test]
    fn test_evaluate_formation_color() {
        let cards = vec![
            Card {
                value: 2,
                color: Color::Yellow,
            },
            Card {
                value: 8,
                color: Color::Yellow,
            },
            Card {
                value: 10,
                color: Color::Yellow,
            },
        ];
        let (formation_type, sum) = FormationType::evaluate_formation(&cards);
        assert_eq!(formation_type, Some(FormationType::Color));
        assert_eq!(sum, 20);
    }

    #[test]
    fn test_evaluate_formation_run() {
        let cards = vec![
            Card {
                value: 9,
                color: Color::Green,
            },
            Card {
                value: 10,
                color: Color::Red,
            },
            Card {
                value: 11,
                color: Color::Blue,
            },
        ];
        let (formation_type, sum) = FormationType::evaluate_formation(&cards);
        assert_eq!(formation_type, Some(FormationType::Run));
        assert_eq!(sum, 30);
    }

    #[test]
    fn test_evaluate_formation_sum() {
        let cards = vec![
            Card {
                value: 1,
                color: Color::Green,
            },
            Card {
                value: 5,
                color: Color::Red,
            },
            Card {
                value: 9,
                color: Color::Blue,
            },
        ];
        let (formation_type, sum) = FormationType::evaluate_formation(&cards);
        assert_eq!(formation_type, Some(FormationType::Sum));
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_evaluate_formation_unshuffled_order_does_not_matter() {
        // Test that the order of cards doesn't change the outcome for run or color run.
        let cards = vec![
            Card {
                value: 5,
                color: Color::Red,
            },
            Card {
                value: 3,
                color: Color::Red,
            },
            Card {
                value: 4,
                color: Color::Red,
            },
        ];
        let (formation_type, sum) = FormationType::evaluate_formation(&cards);
        assert_eq!(formation_type, Some(FormationType::ColorRun));
        assert_eq!(sum, 12);
    }
}
