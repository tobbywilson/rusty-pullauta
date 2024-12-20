use crate::io::bytes::FromToBytes;

/// Vector for storing 2-dimensional grid-like data in a contigous memory block, removes one layer of indirection.
#[derive(Debug, Clone, PartialEq)]
pub struct Vec2D<T> {
    data: Box<[T]>, // the size is fixed, so we can use a Box slice instead of Vec
    w: usize,
    h: usize,
}

impl<T> Vec2D<T> {
    pub fn new(w: usize, h: usize, default: T) -> Vec2D<T>
    where
        T: Clone,
    {
        Vec2D {
            data: vec![default; w * h].into(),
            w,
            h,
        }
    }

    pub fn width(&self) -> usize {
        self.w
    }
    pub fn height(&self) -> usize {
        self.h
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> + '_ {
        let h = self.h;
        self.data.iter_mut().enumerate().map(move |(i, v)| {
            let x = i / h;
            let y = i % h;
            (x, y, v)
        })
    }
}

impl<T: Copy> Vec2D<T> {
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, T)> + '_ {
        self.data.iter().enumerate().map(move |(i, v)| {
            let x = i / self.h;
            let y = i % self.h;
            (x, y, *v)
        })
    }
}

impl Vec2D<f64> {
    pub fn is_any_nan(&self) -> bool {
        self.data.iter().any(|x| x.is_nan())
    }
}

impl<T> std::ops::Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    /// Index is (x,y)
    fn index(&self, index: (usize, usize)) -> &T {
        if index.0 >= self.w || index.1 >= self.h {
            panic!(
                "index out of bounds: the len is ({}, {}) but the index is ({}, {})",
                self.w, self.h, index.0, index.1
            );
        }
        // SAFETY: the index is checked to be within bounds
        unsafe { self.data.get_unchecked(index.0 * self.h + index.1) }
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Vec2D<T> {
    /// Index is (x,y)
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        if index.0 >= self.w || index.1 >= self.h {
            panic!(
                "index out of bounds: the len is ({}, {}) but the index is ({}, {})",
                self.w, self.h, index.0, index.1
            );
        }
        // SAFETY: the index is checked to be within bounds
        unsafe { self.data.get_unchecked_mut(index.0 * self.h + index.1) }
    }
}

/// Implement the FromToBytes trait for Vec2D<T> where T implements FromToBytes.
impl<T: FromToBytes> FromToBytes for Vec2D<T> {
    fn from_bytes<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let w = usize::from_bytes(reader)?;
        let h = usize::from_bytes(reader)?;

        let mut data = Vec::with_capacity(w * h);
        for _ in 0..w * h {
            data.push(T::from_bytes(reader)?);
        }

        Ok(Vec2D {
            data: data.into(),
            w,
            h,
        })
    }

    fn to_bytes<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.w.to_bytes(writer)?;
        self.h.to_bytes(writer)?;

        for item in self.data.iter() {
            item.to_bytes(writer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vec2d: Vec2D<i32> = Vec2D::new(3, 2, 0);
        assert_eq!(vec2d.w, 3);
        assert_eq!(vec2d.h, 2);
        assert_eq!(vec2d.data, vec![0; 6].into());
    }

    #[test]
    fn test_index() {
        let vec2d: Vec2D<i32> = Vec2D::new(3, 2, 1);
        assert_eq!(vec2d[(0, 0)], 1);
        assert_eq!(vec2d[(2, 1)], 1);
    }

    #[test]
    fn test_index_skewed_0() {
        let vec2d: Vec2D<i32> = Vec2D::new(10, 2, 1);
        assert_eq!(vec2d[(9, 0)], 1);
        assert_eq!(vec2d[(9, 1)], 1);
    }

    #[test]
    fn test_index_skewed_1() {
        let vec2d: Vec2D<i32> = Vec2D::new(3, 10, 1);
        assert_eq!(vec2d[(0, 9)], 1);
        assert_eq!(vec2d[(2, 9)], 1);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is (3, 2) but the index is (3, 0)")]
    fn test_index_out_of_bounds_0() {
        let vec2d: Vec2D<i32> = Vec2D::new(3, 2, 1);
        let _ = vec2d[(3, 0)];
    }
    #[test]
    #[should_panic(expected = "index out of bounds: the len is (3, 2) but the index is (0, 2)")]
    fn test_index_out_of_bounds_1() {
        let vec2d: Vec2D<i32> = Vec2D::new(3, 2, 1);
        let _ = vec2d[(0, 2)];
    }

    #[test]
    fn test_index_mut() {
        let mut vec2d: Vec2D<i32> = Vec2D::new(3, 2, 1);
        vec2d[(1, 1)] = 5;
        assert_eq!(vec2d[(1, 1)], 5);
    }

    #[test]
    fn test_index_mut_skewed_0() {
        let mut vec2d: Vec2D<i32> = Vec2D::new(10, 2, 1);
        vec2d[(9, 1)] = 5;
        assert_eq!(vec2d[(9, 1)], 5);
    }

    #[test]
    fn test_index_mut_skewed_1() {
        let mut vec2d: Vec2D<i32> = Vec2D::new(3, 10, 1);
        vec2d[(2, 9)] = 5;
        assert_eq!(vec2d[(2, 9)], 5);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is (3, 2) but the index is (3, 0)")]
    fn test_index_mut_out_of_bounds_0() {
        let mut vec2d: Vec2D<i32> = Vec2D::new(3, 2, 1);
        vec2d[(3, 0)] = 5;
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is (3, 2) but the index is (0, 2)")]
    fn test_index_mut_out_of_bounds_1() {
        let mut vec2d: Vec2D<i32> = Vec2D::new(3, 2, 1);
        vec2d[(0, 2)] = 5;
    }
}
