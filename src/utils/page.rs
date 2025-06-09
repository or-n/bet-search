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

impl<PhantomT> Tag<PhantomT, String> {
    pub fn document(&self) -> Tag<PhantomT, scraper::Html> {
        let html = scraper::Html::parse_document(&self.inner());
        Tag(PhantomData, html)
    }
}

pub trait Name {
    fn name(&self) -> String;
}

pub trait Url {
    fn url(&self) -> String;
}
