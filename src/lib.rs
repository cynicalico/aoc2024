use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct Vec2D<T>
where
    T: Clone + Default,
{
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Vec2D<T>
where
    T: Clone + Default,
{
    pub fn new(width: Option<usize>, height: Option<usize>) -> Self {
        let width = width.unwrap_or(0);
        let height = height.unwrap_or(0);

        Self {
            width,
            height,
            data: vec![Default::default(); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, y: usize, x: usize) -> Option<&T> {
        if y < self.height && x < self.width {
            Some(&self.data[(y * self.width) + x])
        } else {
            None
        }
    }

    pub fn push_row(&mut self, new_row: Vec<T>) {
        if self.width > 0 {
            assert_eq!(self.width, new_row.len());
        } else {
            self.width = new_row.len();
        }

        self.data.extend(new_row.into_iter());
        self.height += 1;
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T>
where
    T: Clone + Default,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[(index.0 * self.width) + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T>
where
    T: Clone + Default,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[(index.0 * self.width) + index.1]
    }
}
