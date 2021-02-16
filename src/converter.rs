pub trait Convertor {
    fn to_vec_i32(&self) -> Vec<Vec<i32>>;
}

impl Convertor for Vec<Vec<char>> {
    fn to_vec_i32(&self) -> Vec<Vec<i32>> {
        self.iter()
            .map(move |s| {
                s.iter()
                    .map(|&x| {
                        if x == '?' {
                            -1
                        } else {
                            (x as u8 - b'0') as i32
                        }
                    })
                    .collect::<Vec<i32>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_check_grp() {
        const N: usize = 9;
        let vv = vec![
            vec![1, 1, 1, 2, 2, 2, 3, 3, 3],
            vec![1, 1, 1, 2, 2, 2, 3, 3, 3],
            vec![1, 1, 1, 2, 2, 2, 3, 3, 3],
            vec![4, 4, 4, 5, 5, 5, 6, 6, 6],
            vec![4, 4, 4, 5, 5, 5, 6, 6, 6],
            vec![4, 4, 4, 5, 5, 5, 6, 6, 6],
            vec![7, 7, 7, 8, 8, 8, 9, 9, 9],
            vec![7, 7, 7, 8, 8, 8, 9, 9, 9],
            vec![7, 7, 7, 8, 8, 8, 9, 9, 9],
        ];

        let vv = vv as Vec<Vec<i32>>;
        let mut map = vec![vec![((0, 0), (N, N)); N]; N];
        for y in 0..9 {
            for x in 0..9 {
                map[y][x] = (
                    ((x / 3) * 3, (y / 3) * 3),
                    ((x / 3) * 3 + 3, (y / 3) * 3 + 3),
                );
            }
        }

        let (begin, end) = map[5][6];
        for i in begin.1..end.1 {
            for j in begin.0..end.0 {
                print!("{:3}", vv[i][j]);
            }
            println!();
        }
    }
}

#[allow(dead_code)]
fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    a.to_string()
}
