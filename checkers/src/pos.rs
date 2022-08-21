#[derive(
    Debug,
    Clone, Copy,
    PartialEq, Eq,
)]
pub struct Pos(u8);

impl Pos {
    pub fn new(x: u8, y: u8) -> Pos {
        debug_assert!(x < 4);
        debug_assert!(y < 8);

        Pos((y<<2) + x)
    }

    pub fn graphical(x: u8, y: u8) -> Option<Pos> {
        if (x + y) % 2 == 0 {
            // it means, unused cell so there are no internal expression.
            return None
        }

        let iy = 7 - y;
        let ix = (7-x) / 2;
        Some(Pos::new(ix, iy))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn graphical_expression_converted_correctly() {
        let cases = [
            ((0, 0), None, "Left-Top"),
            ((1, 0), Some(31), "Right side from Left-Top"),
            ((0, 1), Some(27), "Below side from Left-Top"),
            ((7, 0), Some(28), "Right-top"),
            ((6, 0), None, "Left side from Right-top"),
            ((7, 1), None, "Below side from Left-Top"),
            ((3, 4), Some(14), "Center active cell"),
            ((4, 4), None, "Center inactive cell"),
            ((0, 7), Some(3), "Left-Bottom"),
            ((0, 6), None, "Above side from Left-Bottom"),
            ((1, 7), None, "Right side from Left-Bottom"),
            ((7, 7), None, "Right-Bottom"),
            ((6, 7), Some(0), "Left side from Right-Bottom"),
            ((7, 6), Some(4), "Above side from Right-Bottom"),
        ];

        for ((x, y), exp, msg) in cases {
            let expect = exp.map(|v| Pos(v));
            let actual = Pos::graphical(x, y);
            assert_eq!(expect, actual, "{}", msg);
        }
    }
}
