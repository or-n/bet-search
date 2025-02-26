use std::marker::PhantomData;
use std::sync::Arc;

pub struct Page<T>(Arc<str>, PhantomData<T>);

impl<T> Page<T> {
    pub fn new(html: &str) -> Self {
        Self(Arc::from(html), PhantomData)
    }
}

pub trait ToArcStr {
    fn as_arc_str(&self) -> Arc<str>;
}

impl<T> ToArcStr for Page<T> {
    fn as_arc_str(&self) -> Arc<str> {
        self.0.clone()
    }
}
