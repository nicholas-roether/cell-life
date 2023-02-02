pub trait Accumulator<T, R> {
	fn accumulate(&mut self, item: T);

	fn complete(&mut self) -> R;
}
