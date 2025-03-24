pub trait Dataset {
    type Sample;

    fn len(&self) -> usize;

    fn get_item(&self, index: usize) -> Self::Sample;
}
