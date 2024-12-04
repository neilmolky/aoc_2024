use std::ops::Range;

use array2d::Array2D;

pub enum Gradient {
    Horizontal,       // : -
    PositiveDiagonal, // : /
    Vertical,         // : |
    NegativeDiagonal, // : \
}

pub enum ArrayOrientation {
    Diagonal,
    Linear,
    Full,
}

pub struct ArrayUtils2D {
    height: usize,
    width: usize,
}

impl ArrayUtils2D {
    pub fn from_array<T>(a: &Array2D<T>) -> ArrayUtils2D {
        let height = a.num_rows();
        let width = a.num_columns();
        ArrayUtils2D { height, width }
    }

    pub fn vertical_range(&self) -> Range<usize> {
        0..self.height
    }
    pub fn horizontal_range(&self) -> Range<usize> {
        0..self.width
    }
    pub fn diagonal_range(&self) -> Range<usize> {
        0..(self.width.min(self.height))
    }

    pub fn all_points(&self) -> Vec<(usize, usize)> {
        self.vertical_range()
            .flat_map(|i| self.horizontal_range().map(move |j| (i, j)))
            .collect()
    }

    pub fn neighbors(&self, point: (usize, usize), g: ArrayOrientation) -> Vec<(usize, usize)> {
        match g {
            ArrayOrientation::Linear => [
                self.as_index_tuple(point.0 + 1, point.1),
                self.subtract_option(point.0, 1).map(|i| (i, point.1)),
                self.as_index_tuple(point.0, point.1 + 1),
                self.subtract_option(point.1, 1).map(|j| (point.0, j)),
            ]
            .into_iter()
            .flatten()
            .collect(),
            ArrayOrientation::Diagonal => [
                self.subtract_option(point.0, 1)
                    .and_then(|i| self.as_index_tuple(i, point.1 + 1)),
                self.subtract_option(point.0, 1)
                    .and_then(|i| self.subtract_option(point.1, 1).map(|j| (i, j))),
                self.subtract_option(point.1, 1)
                    .and_then(|j| self.as_index_tuple(point.0 + 1, j)),
                self.subtract_option(point.1, 1)
                    .and_then(|j| self.subtract_option(point.0, 1).map(|i| (i, j))),
            ]
            .into_iter()
            .flatten()
            .collect(),
            ArrayOrientation::Full => self
                .neighbors(point, ArrayOrientation::Linear)
                .iter()
                .chain(self.neighbors(point, ArrayOrientation::Diagonal).iter())
                .map(|x| *x)
                .collect(),
        }
    }

    pub fn as_index_tuple(&self, i: usize, j: usize) -> Option<(usize, usize)> {
        if self.in_limits(i, j) {
            Some((i, j))
        } else {
            None
        }
    }

    pub fn in_limits(&self, i: usize, j: usize) -> bool {
        (i < self.height) & (j < self.width)
    }

    pub fn subtract_option(&self, x: usize, y: usize) -> Option<usize> {
        if x < y {
            None
        } else {
            Some(x.abs_diff(y))
        }
    }

    pub fn lines(&self, d: Gradient) -> Vec<Vec<(usize, usize)>> {
        match d {
            Gradient::Horizontal => self
                .vertical_range()
                .map(|i| self.horizontal_range().map(|j| (i, j)).collect())
                .collect(),
            Gradient::Vertical => self
                .horizontal_range()
                .map(|j| self.vertical_range().map(|i| (i, j)).collect())
                .collect(),
            Gradient::NegativeDiagonal => self
                .vertical_range()
                .rev()
                .map(|i| {
                    self.diagonal_range()
                        .flat_map(|d| self.as_index_tuple(i + d, d))
                        .collect()
                })
                .chain({
                    self.horizontal_range().skip(1).map(|j| {
                        self.diagonal_range()
                            .flat_map(|d| self.as_index_tuple(d, j + d))
                            .collect()
                    })
                })
                .collect(),
            Gradient::PositiveDiagonal => self
                .vertical_range()
                .map(|i| {
                    self.diagonal_range()
                        .flat_map(|d| {
                            self.subtract_option(i, d)
                                .and_then(|i2| self.as_index_tuple(i2, d))
                        })
                        .collect()
                })
                .chain({
                    self.horizontal_range().skip(1).map(|j| {
                        self.diagonal_range()
                            .flat_map(|d| {
                                self.subtract_option(self.height, d + 1)
                                    .and_then(|i| self.as_index_tuple(i, j + d))
                            })
                            .collect()
                    })
                })
                .collect(),
        }
    }
}

pub fn demo() -> () {
    let mut demo: Array2D<usize> = Array2D::from_rows(&[
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ])
    .unwrap();
    let a = ArrayUtils2D::from_array(&demo);
    for (i, ln) in a.lines(Gradient::NegativeDiagonal).iter().enumerate() {
        for pair in ln {
            demo[*pair] = i;
        }
    }
    for r in demo.as_rows() {
        println!("{:?}", r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn negative_diagonal_test() {
        let mut pre_transform: Array2D<usize> = Array2D::from_rows(&[
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ])
        .unwrap();
        let expected: Array2D<usize> = Array2D::from_rows(&[
            vec![3, 4, 5, 6, 7],
            vec![2, 3, 4, 5, 6],
            vec![1, 2, 3, 4, 5],
            vec![0, 1, 2, 3, 4],
        ])
        .unwrap();

        let a = ArrayUtils2D::from_array(&pre_transform);
        for (i, ln) in a.lines(Gradient::NegativeDiagonal).iter().enumerate() {
            for pair in ln {
                pre_transform[*pair] = i;
            }
        }
        assert!(pre_transform == expected)
    }
    #[test]
    fn positive_diagonal_test() {
        let mut pre_transform: Array2D<usize> = Array2D::from_rows(&[
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ])
        .unwrap();
        let expected: Array2D<usize> = Array2D::from_rows(&[
            vec![0, 1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6],
            vec![3, 4, 5, 6, 7],
        ])
        .unwrap();

        let a = ArrayUtils2D::from_array(&pre_transform);
        for (i, ln) in a.lines(Gradient::PositiveDiagonal).iter().enumerate() {
            for pair in ln {
                pre_transform[*pair] = i;
            }
        }
        assert!(pre_transform == expected)
    }

    #[test]
    fn horizontal_test() {
        let mut pre_transform: Array2D<usize> = Array2D::from_rows(&[
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ])
        .unwrap();
        let expected: Array2D<usize> = Array2D::from_rows(&[
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
            vec![2, 2, 2, 2, 2],
            vec![3, 3, 3, 3, 3],
        ])
        .unwrap();

        let a = ArrayUtils2D::from_array(&pre_transform);
        for (i, ln) in a.lines(Gradient::Horizontal).iter().enumerate() {
            for pair in ln {
                pre_transform[*pair] = i;
            }
        }
        assert!(pre_transform == expected)
    }

    #[test]
    fn vertical_test() {
        let mut pre_transform: Array2D<usize> = Array2D::from_rows(&[
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ])
        .unwrap();
        let expected: Array2D<usize> = Array2D::from_rows(&[
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3, 4],
        ])
        .unwrap();

        let a = ArrayUtils2D::from_array(&pre_transform);
        for (i, ln) in a.lines(Gradient::Vertical).iter().enumerate() {
            for pair in ln {
                pre_transform[*pair] = i;
            }
        }
        assert!(pre_transform == expected)
    }
}
