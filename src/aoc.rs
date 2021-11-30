use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Eq)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
        }
    }
}

impl ops::Mul<i64> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: i64) -> Vec2 {
        Vec2 {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

pub fn vec_in_range(v: Vec2, low_incl: Vec2, high_excl: Vec2) -> bool {
    low_incl <= v && v < high_excl
}

impl Vec2 {
    pub fn neighbours(&self) -> Vec<Vec2> {
        vec![
            Vec2 {
                x: self.x + 1,
                y: self.y,
            },
            Vec2 {
                x: self.x,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x - 1,
                y: self.y,
            },
            Vec2 {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }

    pub fn neighbours_diag(&self) -> Vec<Vec2> {
        vec![
            Vec2 {
                x: self.x + 1,
                y: self.y,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x - 1,
                y: self.y - 1,
            },
            Vec2 {
                x: self.x - 1,
                y: self.y,
            },
            Vec2 {
                x: self.x - 1,
                y: self.y + 1,
            },
            Vec2 {
                x: self.x,
                y: self.y + 1,
            },
            Vec2 {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_ops() {
        let a = Vec2 { x: 1, y: 2 };
        let b = Vec2 { x: 5, y: -1 };

        assert_eq!(a + b, Vec2 { x: 6, y: 1 });
        assert_eq!(a - b, Vec2 { x: -4, y: 3 });
        assert_eq!(a == b, false);
        assert_eq!(a == Vec2 { ..a }, true);
        assert_eq!(a < b, true);
        assert_eq!(b < a, false);
        assert_eq!(b > a, true);
    }

    #[test]
    fn test_in_range() {
        let a = Vec2 { x: 1, y: 2 };
        let b = Vec2 { x: 5, y: -1 };
        assert_eq!(vec_in_range(a, a, b), true);
        assert_eq!(vec_in_range(a, b, b), false);
        assert_eq!(vec_in_range(a, a, a), false);
    }

    #[test]
    fn test_neighbours() {
        let a = Vec2 { x: 1, y: 2 };
        let mut set: HashSet<Vec2> = HashSet::new();
        for neigh in a.neighbours().iter() {
            let diff = a - *neigh;
            assert_eq!(diff.x.abs() + diff.y.abs(), 1);
            set.insert(*neigh);
        }
        assert_eq!(set.len(), 4);

        for neigh in a.neighbours_diag().iter() {
            set.insert(*neigh);
        }
        assert_eq!(set.len(), 8);
    }
}
