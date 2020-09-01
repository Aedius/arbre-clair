use crate::craft::{Item, Profession, Recipe};

use crate::craft::BaseResource::*;
use crate::craft::CraftedResource::*;
use crate::craft::GroupResource::*;


pub fn get_recipe() -> Vec<Recipe> {
    let mut ret = Vec::new();

    let component = "Components";

    ret.push(Recipe {
        name: "Empty Flask",
        input: vec![
            (Item::Crafted(PowderedStone), 3),
            (Item::Crafted(PowderedStone), 3),
            (Item::Base(Dust), 3),
        ],
        output: (EmptyFlask, 1),
        profession: Profession::Alchemy,
        menu: component,
    });

    ret.push(Recipe {
        name: "Grind Resource",
        input: vec![
            (Item::Group(NonBasicOre), 2),
        ],
        output: (PowderedStone, 1),
        profession: Profession::Alchemy,
        menu: component,
    });

    ret
}