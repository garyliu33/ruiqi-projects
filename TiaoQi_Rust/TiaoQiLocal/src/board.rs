use std::collections::HashSet;
pub(crate) use crate::piece_color::PieceColor;

pub struct Cell {
    pub color: Option<PieceColor>,
    pub neighbors: [Option<usize>; 6]
}

impl Cell {
    fn new(right: Option<usize>, top_right: Option<usize>, top_left: Option<usize>, left: Option<usize>, bottom_left: Option<usize>, bottom_right: Option<usize>) -> Self {
        Self { color: None, neighbors: [right, top_right, top_left, left, bottom_left, bottom_right] }
    }
}

pub struct Board {
    pub(crate) cells: [Cell; 121]
}

impl Board {
    pub fn new() -> Self {
        Self { cells: [
            Cell::new(None, None, None, None, Some(1), Some(2)),
            Cell::new(Some(2), Some(0), None, None, Some(3), Some(4)),
            Cell::new(None, None, Some(0), Some(1), Some(4), Some(5)),
            Cell::new(Some(4), Some(1), None, None, Some(6), Some(7)),
            Cell::new(Some(5), Some(2), Some(1), Some(3), Some(7), Some(8)),
            Cell::new(None, None, Some(2), Some(4), Some(8), Some(9)),
            Cell::new(Some(7), Some(3), None, None, Some(14), Some(15)),
            Cell::new(Some(8), Some(4), Some(3), Some(6), Some(15), Some(16)),
            Cell::new(Some(9), Some(5), Some(4), Some(7), Some(16), Some(17)),
            Cell::new(None, None, Some(5), Some(8), Some(17), Some(18)),
            Cell::new(Some(11), None, None, None, None, Some(23)),
            Cell::new(Some(12), None, None, Some(10), Some(23), Some(24)),
            Cell::new(Some(13), None, None, Some(11), Some(24), Some(25)),
            Cell::new(Some(14), None, None, Some(12), Some(25), Some(26)),
            Cell::new(Some(15), Some(6), None, Some(13), Some(26), Some(27)),
            Cell::new(Some(16), Some(7), Some(6), Some(14), Some(27), Some(28)),
            Cell::new(Some(17), Some(8), Some(7), Some(15), Some(28), Some(29)),
            Cell::new(Some(18), Some(9), Some(8), Some(16), Some(29), Some(30)),
            Cell::new(Some(19), None, Some(9), Some(17), Some(30), Some(31)),
            Cell::new(Some(20), None, None, Some(18), Some(31), Some(32)),
            Cell::new(Some(21), None, None, Some(19), Some(32), Some(33)),
            Cell::new(Some(22), None, None, Some(20), Some(33), Some(34)),
            Cell::new(None, None, None, Some(21), Some(34), None),
            Cell::new(Some(24), Some(11), Some(10), None, None, Some(35)),
            Cell::new(Some(25), Some(12), Some(11), Some(23), Some(35), Some(36)),
            Cell::new(Some(26), Some(13), Some(12), Some(24), Some(36), Some(37)),
            Cell::new(Some(27), Some(14), Some(13), Some(25), Some(37), Some(38)),
            Cell::new(Some(28), Some(15), Some(14), Some(26), Some(38), Some(39)),
            Cell::new(Some(29), Some(16), Some(15), Some(27), Some(39), Some(40)),
            Cell::new(Some(30), Some(17), Some(16), Some(28), Some(40), Some(41)),
            Cell::new(Some(31), Some(18), Some(17), Some(29), Some(41), Some(42)),
            Cell::new(Some(32), Some(19), Some(18), Some(30), Some(42), Some(43)),
            Cell::new(Some(33), Some(20), Some(19), Some(31), Some(43), Some(44)),
            Cell::new(Some(34), Some(21), Some(20), Some(32), Some(44), Some(45)),
            Cell::new(None, Some(22), Some(21), Some(33), Some(45), None),
            Cell::new(Some(36), Some(24), Some(23), None, None, Some(46)),
            Cell::new(Some(37), Some(25), Some(24), Some(35), Some(46), Some(47)),
            Cell::new(Some(38), Some(26), Some(25), Some(36), Some(47), Some(48)),
            Cell::new(Some(39), Some(27), Some(26), Some(37), Some(48), Some(49)),
            Cell::new(Some(40), Some(28), Some(27), Some(38), Some(49), Some(50)),
            Cell::new(Some(41), Some(29), Some(28), Some(39), Some(50), Some(51)),
            Cell::new(Some(42), Some(30), Some(29), Some(40), Some(51), Some(52)),
            Cell::new(Some(43), Some(31), Some(30), Some(41), Some(52), Some(53)),
            Cell::new(Some(44), Some(32), Some(31), Some(42), Some(53), Some(54)),
            Cell::new(Some(45), Some(33), Some(32), Some(43), Some(54), Some(55)),
            Cell::new(None, Some(34), Some(33), Some(44), Some(55), None),
            Cell::new(Some(47), Some(36), Some(35), None, None, Some(56)),
            Cell::new(Some(48), Some(37), Some(36), Some(46), Some(56), Some(57)),
            Cell::new(Some(49), Some(38), Some(37), Some(47), Some(57), Some(58)),
            Cell::new(Some(50), Some(39), Some(38), Some(48), Some(58), Some(59)),
            Cell::new(Some(51), Some(40), Some(39), Some(49), Some(59), Some(60)),
            Cell::new(Some(52), Some(41), Some(40), Some(50), Some(60), Some(61)),
            Cell::new(Some(53), Some(42), Some(41), Some(51), Some(61), Some(62)),
            Cell::new(Some(54), Some(43), Some(42), Some(52), Some(62), Some(63)),
            Cell::new(Some(55), Some(44), Some(43), Some(53), Some(63), Some(64)),
            Cell::new(None, Some(45), Some(44), Some(54), Some(64), None),
            Cell::new(Some(57), Some(47), Some(46), None, Some(65), Some(66)),
            Cell::new(Some(58), Some(48), Some(47), Some(56), Some(66), Some(67)),
            Cell::new(Some(59), Some(49), Some(48), Some(57), Some(67), Some(68)),
            Cell::new(Some(60), Some(50), Some(49), Some(58), Some(68), Some(69)),
            Cell::new(Some(61), Some(51), Some(50), Some(59), Some(69), Some(70)),
            Cell::new(Some(62), Some(52), Some(51), Some(60), Some(70), Some(71)),
            Cell::new(Some(63), Some(53), Some(52), Some(61), Some(71), Some(72)),
            Cell::new(Some(64), Some(54), Some(53), Some(62), Some(72), Some(73)),
            Cell::new(None, Some(55), Some(54), Some(63), Some(73), Some(74)),
            Cell::new(Some(66), Some(56), None, None, Some(75), Some(76)),
            Cell::new(Some(67), Some(57), Some(56), Some(65), Some(76), Some(77)),
            Cell::new(Some(68), Some(58), Some(57), Some(66), Some(77), Some(78)),
            Cell::new(Some(69), Some(59), Some(58), Some(67), Some(78), Some(79)),
            Cell::new(Some(70), Some(60), Some(59), Some(68), Some(79), Some(80)),
            Cell::new(Some(71), Some(61), Some(60), Some(69), Some(80), Some(81)),
            Cell::new(Some(72), Some(62), Some(61), Some(70), Some(81), Some(82)),
            Cell::new(Some(73), Some(63), Some(62), Some(71), Some(82), Some(83)),
            Cell::new(Some(74), Some(64), Some(63), Some(72), Some(83), Some(84)),
            Cell::new(None, None, Some(64), Some(73), Some(84), Some(85)),
            Cell::new(Some(76), Some(65), None, None, Some(86), Some(87)),
            Cell::new(Some(77), Some(66), Some(65), Some(75), Some(87), Some(88)),
            Cell::new(Some(78), Some(67), Some(66), Some(76), Some(88), Some(89)),
            Cell::new(Some(79), Some(68), Some(67), Some(77), Some(89), Some(90)),
            Cell::new(Some(80), Some(69), Some(68), Some(78), Some(90), Some(91)),
            Cell::new(Some(81), Some(70), Some(69), Some(79), Some(91), Some(92)),
            Cell::new(Some(82), Some(71), Some(70), Some(80), Some(92), Some(93)),
            Cell::new(Some(83), Some(72), Some(71), Some(81), Some(93), Some(94)),
            Cell::new(Some(84), Some(73), Some(72), Some(82), Some(94), Some(95)),
            Cell::new(Some(85), Some(74), Some(73), Some(83), Some(95), Some(96)),
            Cell::new(None, None, Some(74), Some(84), Some(96), Some(97)),
            Cell::new(Some(87), Some(75), None, None, Some(98), Some(99)),
            Cell::new(Some(88), Some(76), Some(75), Some(86), Some(99), Some(100)),
            Cell::new(Some(89), Some(77), Some(76), Some(87), Some(100), Some(101)),
            Cell::new(Some(90), Some(78), Some(77), Some(88), Some(101), Some(102)),
            Cell::new(Some(91), Some(79), Some(78), Some(89), Some(102), Some(103)),
            Cell::new(Some(92), Some(80), Some(79), Some(90), Some(103), Some(104)),
            Cell::new(Some(93), Some(81), Some(80), Some(91), Some(104), Some(105)),
            Cell::new(Some(94), Some(82), Some(81), Some(92), Some(105), Some(106)),
            Cell::new(Some(95), Some(83), Some(82), Some(93), Some(106), Some(107)),
            Cell::new(Some(96), Some(84), Some(83), Some(94), Some(107), Some(108)),
            Cell::new(Some(97), Some(85), Some(84), Some(95), Some(108), Some(109)),
            Cell::new(None, None, Some(85), Some(96), Some(109), Some(110)),
            Cell::new(Some(99), Some(86), None, None, None, None),
            Cell::new(Some(100), Some(87), Some(86), Some(98), None, None),
            Cell::new(Some(101), Some(88), Some(87), Some(99), None, None),
            Cell::new(Some(102), Some(89), Some(88), Some(100), None, None),
            Cell::new(Some(103), Some(90), Some(89), Some(101), None, Some(111)),
            Cell::new(Some(104), Some(91), Some(90), Some(102), Some(111), Some(112)),
            Cell::new(Some(105), Some(92), Some(91), Some(103), Some(112), Some(113)),
            Cell::new(Some(106), Some(93), Some(92), Some(104), Some(113), Some(114)),
            Cell::new(Some(107), Some(94), Some(93), Some(105), Some(114), None),
            Cell::new(Some(108), Some(95), Some(94), Some(106), None, None),
            Cell::new(Some(109), Some(96), Some(95), Some(107), None, None),
            Cell::new(Some(110), Some(97), Some(96), Some(108), None, None),
            Cell::new(None, None, Some(97), Some(109), None, None),
            Cell::new(Some(112), Some(103), Some(102), None, None, Some(115)),
            Cell::new(Some(113), Some(104), Some(103), Some(111), Some(115), Some(116)),
            Cell::new(Some(114), Some(105), Some(104), Some(112), Some(116), Some(117)),
            Cell::new(None, Some(106), Some(105), Some(113), Some(117), None),
            Cell::new(Some(116), Some(112), Some(111), None, None, Some(118)),
            Cell::new(Some(117), Some(113), Some(112), Some(115), Some(118), Some(119)),
            Cell::new(None, Some(114), Some(113), Some(116), Some(119), None),
            Cell::new(Some(119), Some(116), Some(115), None, None, Some(120)),
            Cell::new(None, Some(117), Some(116), Some(118), Some(120), None),
            Cell::new(None, Some(119), Some(118), None, None, None),
        ]}
    }

    pub fn setup(&mut self, ids: Vec<usize>) {
        for i in ids {
            for j in TRIANGLES[i] {
                self.cells[j].color = Some(PieceColor::get_color(i));
            }
        }
    }

    pub fn get_possible_moves(&self, i: usize) -> Vec<usize> {
        [self.get_all_single_steps(i), self.get_all_jumps(i)].concat()
    }

    fn get_all_single_steps(&self, i: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for dir in 0..6 {
            if let Some(neighbor) = self.get_neighbor(i, dir) {
                if self.is_empty(neighbor) {
                    result.push(neighbor)
                }
            }
        }

        result
    }

    fn get_all_jumps(&self, i: usize) -> Vec<usize> {
        let mut final_landings = HashSet::new();
        self.find_jumps_recursive(i, &mut final_landings, &mut HashSet::new());
        final_landings.into_iter().collect()
    }

    fn find_jumps_recursive(&self, i: usize, final_landings: &mut HashSet<usize>, path_visited: &mut HashSet<usize>) {
        for jump in self.get_single_jumps(i) {
            if !path_visited.contains(&jump) {
                final_landings.insert(jump);
                path_visited.insert(i);
                self.find_jumps_recursive(jump, final_landings, path_visited);
                path_visited.remove(&i);
            }
        }
    }

    fn get_single_jumps(&self, i: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for dir in 0..6 {
            if let Some(neighbor) = self.get_neighbor(i, dir) {
                if !self.is_empty(neighbor) {
                    if let Some(second_neighbor) = self.get_neighbor(neighbor, dir) {
                        if self.is_empty(second_neighbor) {
                            result.push(second_neighbor);
                        }
                    }
                }
            }
        }

        result
    }
    
    pub fn move_piece(&mut self, start: usize, end: usize) {
        self.cells[end].color = self.cells[start].color;
        self.cells[start].color = None;
    }

    fn is_empty(&self, i: usize) -> bool {
        self.cells[i].color.is_none()
    }
    
    fn get_neighbor(&self, i: usize, dir: usize) -> Option<usize> {
        self.cells[i].neighbors[dir]
    }
}

static TRIANGLES: [[usize; 10]; 6] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [19, 20, 21, 22, 32, 33, 34, 44, 45, 55],
    [74, 84, 85, 95, 96, 97, 107, 108, 109, 110],
    [111, 112, 113, 114, 115, 116, 117, 118, 119, 120],
    [65, 75, 76, 86, 87, 88, 98, 99, 100, 101],
    [10, 11, 12, 13, 23, 24, 25, 35, 36, 46]
];