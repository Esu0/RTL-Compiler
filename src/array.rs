use std::fmt::Debug;

#[macro_export]
macro_rules! ary {
    ($($x:expr),+ $(,)?) => (
        $crate::array::Array::from_box(Box::new([$($x),+]))
    )
}

pub struct Array<T> {
    slc: Box<[T]>,
}

impl<T> Array<T> {
    pub fn new(size: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            slc: vec![T::default(); size].into_boxed_slice(),
        }
    }

    pub fn new_empty() -> Self {
        Self { slc: Box::new([]) }
    }

    pub fn from_box(b: Box<[T]>) -> Self {
        Self { slc: b }
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.slc.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.slc.iter_mut()
    }
}

impl<T, I> std::ops::Index<I> for Array<T>
where
    I: std::slice::SliceIndex<[T]>,
{
    type Output = <I as std::slice::SliceIndex<[T]>>::Output;
    fn index(&self, index: I) -> &Self::Output {
        &self.slc[index]
    }
}

impl<T, I> std::ops::IndexMut<I> for Array<T>
where
    I: std::slice::SliceIndex<[T]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.slc[index]
    }
}

impl<'a, T> IntoIterator for &'a Array<T> {
    type IntoIter = <&'a [T] as IntoIterator>::IntoIter;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.slc.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Array<T> {
    type IntoIter = <&'a mut [T] as IntoIterator>::IntoIter;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.slc.iter_mut()
    }
}

impl<T> std::fmt::Debug for Array<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for elem in self.iter() {
            write!(f, "{:?}", elem)?;
        }
        Ok(())
    }
}
