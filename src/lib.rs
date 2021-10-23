use std::{fmt, iter::{IntoIterator, Iterator}, marker::PhantomData, ops::{Index, IndexMut}, slice, borrow, cell::Cell};

#[macro_export]
macro_rules! matrix {
    [$([$($e:expr),*]),*] => {
        Matrix::from_vec(vec![$($($e),*),*], {
            let mut rows = 0;
            let mut row_len = None;
            $(
                rows += 1;
                let mut this_row_len = 0;
                $(this_row_len += 1;$e;)*
                match row_len {
                    Some(n) => {
                        if this_row_len != n {
                            panic!("Attempted to make jagged matrix with macro");
                        }
                    },
                    None => row_len = Some(this_row_len),
                }
            )*
            //println!("{}", rows);
            rows
        })
    };
}

#[derive(Clone)]
pub struct Matrix<T> {
    v: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    pub fn new() -> Self {
        Self {
            v: Vec::new(),
            rows: 0,
            cols: 0,
        }
    }

    pub fn with_capacity(rows: usize, cols: usize) -> Self {
        Self {
            v: Vec::with_capacity(rows * cols),
            rows: 0,
            cols: 0,
        }
    }

    pub fn from_vec(v: Vec<T>, rows: usize) -> Self {
        if v.len() % rows != 0 {
            panic!("Row doesn't work for this size")
        }
        Self {
            cols: v.len() / rows,
            v,
            rows,
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, i: usize) -> &[T] {
        unsafe { slice::from_raw_parts(&self.v[i * self.cols] as *const T, self.cols) }
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, i: usize) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(&mut self.v[i * self.cols] as &mut T, self.cols) }
    }
}

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub struct Iter<'a, T: 'a> {
    matrix: &'a Matrix<T>,
    i: usize,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.matrix.rows {
            None
        }
        else {
            let row = &self.matrix[self.i];
            self.i += 1;
            Some(row)
        }
    }
}

impl<'a, T: 'a> IntoIterator for &'a Matrix<T> {
    type Item = &'a [T];
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            matrix: self,
            i: 0,
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    matrix: Cell<Matrix<T>>,
    i: usize,
    _marker: PhantomData<&'a mut T>
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut [T];

    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        if self.i >= self.matrix.borrow().rows {
            None
        }
        else {
            self.i += 1;
            Some(&mut self.matrix.borrow_mut()[self.i - 1])
        }
    }
}

impl<'a, T: 'a> IntoIterator for &'a mut Matrix<T> {
    type Item = &'a mut [T];
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            matrix: self,
            i: 0,
        }
    }
}

/*pub struct JaggedMatrix<T> {
    pub v: Vec<T>,
    pub rows: usize,
    pub cols: Vec<usize>,
}

impl<T> Index<usize> for JaggedMatrix<T> {
    type Output = [T];

    fn index(&self, i: usize) -> &[T] {
        unsafe { slice::from_raw_parts(&self.v[i * self.cols[0..i].iter().sum::<usize>()] as *const T, self.cols[i]) }
    }
}

impl<T> IndexMut<usize> for JaggedMatrix<T> {
    fn index_mut(&mut self, i: usize) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(&mut self.v[i * self.cols[0..i].iter().sum::<usize>()] as &mut T, self.cols[i]) }
    }
}*/