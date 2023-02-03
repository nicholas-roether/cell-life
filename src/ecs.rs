#[derive(Debug)]
pub struct Ecs<C> {
	entity_components: Vec<Vec<C>>
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Entity(usize);

impl<C> Ecs<C> {
	pub fn new() -> Self {
		Self {
			entity_components: Vec::new()
		}
	}

	pub fn entity(&mut self) -> Entity {
		let id = self.entity_components.len();
		self.entity_components.push(Vec::new());
		Entity(id)
	}

	pub fn add_component(&mut self, entity: Entity, component: C) {
		if entity.0 >= self.entity_components.len() {
			panic!("Entity does not exist");
		}
		self.entity_components[entity.0].push(component);
	}

	pub fn components(&self, entity: Entity) -> &[C] {
		&self.entity_components[entity.0]
	}
}
