use crate::craft::{Item, Profession, Recipe};

use crate::craft::BaseResource::*;
use crate::craft::CraftedResource::*;
use crate::craft::GroupResource::*;


pub fn get_recipe() -> Vec<Recipe> {
    let mut ret = Vec::new();

    let stonemasonry_components = " Stonemasonry components";

    ret.push(Recipe {
        name: "Diamond cutting blade",
        input: vec![
            (Item::Group(Ore), 9),
            (Item::Base(CuttingGrit), 3),
        ],
        output: (DiamondCuttingBlade, 1),
        profession: Profession::Stonemasonry,
        menu: stonemasonry_components,
    });

    ret.push(Recipe {
        name: "Grinding wheel",
        input: vec![
            (Item::Group(NonBasicStone), 8),
			(Item::Group(NonBasicStone), 8),
			(Item::Group(NonBasicStone), 8),
        ],
        output: (GrindingWheel, 1),
        profession: Profession::Stonemasonry,
        menu: stonemasonry_components,
    });

    ret.push(Recipe {
        name: "Polishing paste",
        input: vec![
            (Item::Crafted(PowderedStone), 8),
            (Item::Base(WaterFlask), 3),
        ],
        output: (DiamondCuttingBlade, 1),
        profession: Profession::Stonemasonry,
        menu: stonemasonry_components,
    });


    ret
}