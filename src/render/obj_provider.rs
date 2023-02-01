pub trait ObjectProvider<I> {
	fn iter_objects(&self) -> Box<dyn Iterator<Item = I> + '_>;
}
