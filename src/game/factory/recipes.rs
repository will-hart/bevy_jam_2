use std::hash::Hash;

use bevy::utils::HashMap;

use crate::game::components::BoxType;

#[derive(Eq, Debug, Clone, Copy)]
pub struct RecipeInputs(pub BoxType, pub BoxType);

impl RecipeInputs {
    fn from_inputs(inputs: &[Option<BoxType>; 2]) -> Option<Self> {
        match inputs {
            [None, None] | [Some(_), None] | [None, Some(_)] => None,
            [Some(a), Some(b)] => Some(RecipeInputs(*a, *b)),
        }
    }
}

impl PartialEq for RecipeInputs {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.1 == other.0 && self.0 == other.1)
    }
}

impl Hash for RecipeInputs {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut items = vec![self.0 as usize, self.1 as usize];
        items.sort();

        items[0].hash(state);
        items[1].hash(state);
    }
}

pub struct Recipes(pub HashMap<RecipeInputs, BoxType>);

impl Recipes {
    pub fn get_output(&self, inputs: &[Option<BoxType>; 2]) -> Option<BoxType> {
        let recipe_input = if let Some(ri) = RecipeInputs::from_inputs(inputs) {
            ri
        } else {
            return None;
        };

        match self.0.get(&recipe_input) {
            Some(&r) => Some(r),
            None => None,
        }
    }
}

impl Default for Recipes {
    fn default() -> Self {
        let mut hm = HashMap::new();

        hm.insert(
            RecipeInputs(BoxType::Glassware, BoxType::Wheat),
            BoxType::Beer,
        );
        hm.insert(
            RecipeInputs(BoxType::Glassware, BoxType::Grapes),
            BoxType::Wine,
        );
        hm.insert(
            RecipeInputs(BoxType::Glassware, BoxType::Apples),
            BoxType::Cider,
        );
        hm.insert(
            RecipeInputs(BoxType::Glassware, BoxType::Honey),
            BoxType::Mead,
        );

        Self(hm)
    }
}
