use super::N;

pub trait Checker {
    fn check_all(&self, pos: (usize, usize)) -> bool;
    fn check_row(&self, row: usize) -> bool;
    fn check_col(&self, col: usize) -> bool;
    fn check_grp(&self, pos: (usize, usize)) -> bool;
}

impl Checker for Vec<Vec<i32>> {
    fn check_all(&self, pos: (usize, usize)) -> bool {
        if !self.check_row(pos.1) {
            return false;
        }

        if !self.check_col(pos.0) {
            return false;
        }

        if !self.check_grp(pos) {
            return false;
        }

        true
    }

    /// (i, row) の値が正しいか判定
    fn check_row(&self, row: usize) -> bool {
        let mut list = vec![0; 10];
        for x in 0..N {
            let index = self[row][x];
            if index == -1 {
                continue;
            }

            list[index as usize] += 1;

            if list[index as usize] >= 2 {
                return false;
            }
        }

        true
    }

    /// (col, j) の値が正しいか判定
    fn check_col(&self, col: usize) -> bool {
        let mut list = vec![0; 10];
        for y in 0..N {
            let index = self[y][col];
            if index == -1 {
                continue;
            }

            list[index as usize] += 1;

            if list[index as usize] >= 2 {
                return false;
            }
        }

        true
    }

    /// (col, row) が属するグループが正しいか判定
    fn check_grp(&self, pos: (usize, usize)) -> bool {
        let mut vv = vec![vec![((0, 0), (N, N)); N]; N];
        for y in 0..N {
            for x in 0..N {
                vv[y][x] = (
                    ((x / 3) * 3, (y / 3) * 3),
                    ((x / 3) * 3 + 3, (y / 3) * 3 + 3),
                );
            }
        }

        let mut list = vec![0; 10];
        let (begin, end) = vv[pos.1][pos.0];
        for i in begin.1..end.1 {
            for j in begin.0..end.0 {
                let index = self[i][j];
                if index == -1 {
                    continue;
                }
                list[index as usize] += 1;

                if list[index as usize] >= 2 {
                    return false;
                }
            }
        }

        true
    }
}
