use crate::craft::{Item, Profession, Recipe, RecipeSummary};

use crate::craft::BaseResource::*;
use crate::craft::CraftedResource::*;
use crate::craft::GroupResource::*;

pub fn get_all_recipe() -> Vec<RecipeSummary>{

    let mut ret = Vec::new();
    let all_recipe = get_recipe();

    for recipe in all_recipe{
        let data=  recipe.crafted_data();

        let summary = RecipeSummary{
            key: data.key,
            name: recipe.name,
            profession: recipe.profession,
            stat: data.description
        };

        ret.push(summary);

    }

    ret.sort_by(|a, b| a.name.cmp(&b.name));

    ret
}

pub fn get_recipe() -> Vec<Recipe> {
    let mut ret = Vec::new();

    let cut = "Cut";

    ret.push(Recipe {
        name: "Cut Gemstone : baguette",
        input: vec![
            (Item::Group(RoughGem), 1),
            (Item::Crafted(DiamondCuttingBlade), 1),
            (Item::Base(WaterFlask), 1),
        ],
        output: (CutGemstoneBaguette, 1),
        profession: Profession::Jewelry,
        menu: cut,
    });

    ret.push(Recipe {
        name: "Cut Gemstone : focusing",
        input: vec![
            (Item::Group(RoughGem), 1),
            (Item::Crafted(DiamondCuttingBlade), 1),
            (Item::Base(WaterFlask), 1),
        ],
        output: (CutGemstoneFocusing, 1),
        profession: Profession::Jewelry,
        menu: cut,
    });

    ret.push(Recipe {
        name: "Cut Gemstone : marquise",
        input: vec![
            (Item::Group(RoughGem), 1),
            (Item::Crafted(DiamondCuttingBlade), 1),
            (Item::Base(WaterFlask), 1),
        ],
        output: (CutGemstoneMarquise, 1),
        profession: Profession::Jewelry,
        menu: cut,
    });

    ret.push(Recipe {
        name: "Cut Gemstone : oval",
        input: vec![
            (Item::Group(RoughGem), 1),
            (Item::Crafted(DiamondCuttingBlade), 1),
            (Item::Base(WaterFlask), 1),
        ],
        output: (CutGemstoneOval, 1),
        profession: Profession::Jewelry,
        menu: cut,
    });

    ret.push(Recipe {
        name: "Cut Gemstone : round",
        input: vec![
            (Item::Group(RoughGem), 1),
            (Item::Crafted(DiamondCuttingBlade), 1),
            (Item::Base(WaterFlask), 1),
        ],
        output: (CutGemstoneRound, 1),
        profession: Profession::Jewelry,
        menu: cut,
    });

    ret.push(Recipe {
        name: "Cut Gemstone : square",
        input: vec![
            (Item::Group(RoughGem), 1),
            (Item::Crafted(DiamondCuttingBlade), 1),
            (Item::Base(WaterFlask), 1),
        ],
        output: (CutGemstoneSquare, 1),
        profession: Profession::Jewelry,
        menu: cut,
    });

    ret.push(Recipe {
        name: "Cut Gemstone : triangle",
        input: vec![
            (Item::Group(RoughGem), 1),
            (Item::Crafted(DiamondCuttingBlade), 1),
            (Item::Base(WaterFlask), 1),
        ],
        output: (CutGemstoneTriangle, 1),
        profession: Profession::Jewelry,
        menu: cut,
    });

    ret.push(Recipe {
        name: "Exacting Cut Gemstone : briolette",
        input: vec![
            (Item::Group(RoughGem), 1),
            (Item::Crafted(ChaosEmberCuttingBlade), 1),
            (Item::Base(WaterFlask), 1),
        ],
        output: (ExactingCutGemstoneBriolette, 1),
        profession: Profession::Jewelry,
        menu: cut,
    });

    ret.push(Recipe {
        name: "Exacting Cut Gemstone : heart",
        input: vec![
            (Item::Group(RoughGem), 1),
            (Item::Crafted(ChaosEmberCuttingBlade), 1),
            (Item::Base(WaterFlask), 1),
        ],
        output: (ExactingCutGemstoneHeart, 1),
        profession: Profession::Jewelry,
        menu: cut,
    });

    ret.push(Recipe {
        name: "Exacting Cut Gemstone : trillion",
        input: vec![
            (Item::Group(RoughGem), 1),
            (Item::Crafted(ChaosEmberCuttingBlade), 1),
            (Item::Base(WaterFlask), 1),
        ],
        output: (ExactingCutGemstoneTrillion, 1),
        profession: Profession::Jewelry,
        menu: cut,
    });
    
    let grind_and_polish ="Grind and polish";

    ret.push(Recipe {
        name: "Grind gemstone",
        input: vec![
            (Item::Crafted(CutGem), 1),
            (Item::Crafted(GrindingWheel), 1),
            (Item::Group(BloodOrWater), 1),
        ],
        output: (GrindGemstone, 1),
        profession: Profession::Jewelry,
        menu: grind_and_polish,
    });

    ret.push(Recipe {
        name: "Polish gemstone",
        input: vec![
            (Item::Crafted(CutGem), 1),
            (Item::Crafted(PolishingPaste), 1),
            (Item::Base(WaterFlask), 1),
        ],
        output: (PolishGemstone, 1),
        profession: Profession::Jewelry,
        menu: grind_and_polish,
    });

    ret.push(Recipe {
        name: "Polish soulgem",
        input: vec![
            (Item::Base(RoughSoulgem), 1),
            (Item::Crafted(PolishingPaste), 1),
            (Item::Base(WaterFlask), 1),
        ],
        output: (PolishSoulgem, 1),
        profession: Profession::Jewelry,
        menu: grind_and_polish,
    });

    let settings= "Settings";

    ret.push(Recipe {
        name: "NecklaceBail",
        input: vec![
            (Item::Group(Ore), 10),
            (Item::Base(Silver), 10),
            (Item::Base(Dust), 5),
        ],
        output: (NecklaceBail, 1),
        profession: Profession::Jewelry,
        menu: settings,
    });

    ret.push(Recipe {
        name: "Necklace chain",
        input: vec![
            (Item::Group(Ore), 4),
			(Item::Group(Ore), 8),
			(Item::Group(Ore), 8),
            (Item::Crafted(NecklaceBail), 1),
        ],
        output: (NecklaceChain, 1),
        profession: Profession::Jewelry,
        menu: settings,
    });

    ret.push(Recipe {
        name: "Ring band",
        input: vec![
            (Item::Group(Ore), 4),
			(Item::Group(Ore), 8),
			(Item::Group(Ore), 8),
            (Item::Crafted(RingSetting), 1),
        ],
        output: (RingBand, 1),
        profession: Profession::Jewelry,
        menu: settings,
    });

    ret.push(Recipe {
        name: "RingSetting",
        input: vec![
            (Item::Group(Ore), 10),
            (Item::Base(Aurelium), 10),
            (Item::Base(Dust), 5),
        ],
        output: (RingSetting, 1),
        profession: Profession::Jewelry,
        menu: settings,
    });

    let jewelry = "Jewelry";

    ret.push(Recipe {
        name: "Necklace",
        input: vec![
            (Item::Crafted(PolishedExactingGemstone), 1),
            (Item::Crafted(NecklaceChain), 1),
        ],
        output: (Necklace, 1),
        profession: Profession::Jewelry,
        menu: jewelry,
    });

    ret.push(Recipe {
        name: "Ring",
        input: vec![
            (Item::Crafted(PolishedGemstone), 1),
            (Item::Crafted(RingBand), 1),
        ],
        output: (Ring, 1),
        profession: Profession::Jewelry,
        menu: jewelry,
    });


    ret
}