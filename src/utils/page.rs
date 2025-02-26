use std::marker::PhantomData;

#[derive(Debug)]
pub struct Tag<PhantomT, T>(PhantomData<PhantomT>, T);

impl<PhantomT, T: Clone> Tag<PhantomT, T> {
    pub fn new(value: T) -> Self {
        Self(PhantomData, value)
    }

    pub fn inner(&self) -> T {
        self.1.clone()
    }
}

pub trait Name {
    fn name(&self) -> String;
}
