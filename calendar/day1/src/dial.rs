pub struct Dial {
    pos: u8,
}

impl Dial {
    pub fn new(pos: u8) -> Dial {
        Dial { pos }
    }

    pub fn rotate_left(&mut self, steps: u8) {
        if steps > self.pos {
            self.pos = 100 - steps + self.pos;
        } else {
            self.pos -= steps;
        }
    }

    pub fn rotate_right(&mut self, steps: u8) {
        if steps > 100 - self.pos {
            self.pos = steps - (100 - self.pos);
        } else {
            self.pos += steps;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_left() {
        let mut dial = Dial::new(82);
        dial.rotate_left(30);
        assert_eq!(dial.pos, 52);
    }

    #[test]
    fn test_rotate_left_overflow() {
        let mut dial = Dial::new(50);
        dial.rotate_left(68);
        assert_eq!(dial.pos, 75);
    }

    #[test]
    fn test_rotate_right() {
        let mut dial = Dial::new(50);
        dial.rotate_right(25);
        assert_eq!(dial.pos, 75);
    }

    #[test]
    fn test_rotate_right_overflow() {
        let mut dial = Dial::new(50);
        dial.rotate_right(75);
        assert_eq!(dial.pos, 25);
    }
}
