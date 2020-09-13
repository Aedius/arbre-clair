use crate::craft::{Item, Profession, Recipe};

use crate::craft::BaseResource::*;
use crate::craft::CraftedResource::*;
use crate::craft::GroupResource::*;


pub fn get_recipe() -> Vec<Recipe> {
    let mut ret = Vec::new();

    let processing = "Ressources processing";

    ret.push(Recipe {
        name: "Grind Resource",
        input: vec![
            (Item::Group(NonBasicStone), 1),
        ],
        output: (PowderedStone, 1),
        profession: Profession::Alchemy,
        menu: processing,
    });


    ret.push(Recipe {
        name: "Powdered stone",
        input: vec![
            (Item::Group(NonBasicStone), 1),
        ],
        output: (PowderedStone, 1),
        profession: Profession::Alchemy,
        menu: processing,
    });

    ret.push(Recipe {
        name: "Ground Mineral",
        input: vec![
            (Item::Group(Minerals), 1),
        ],
        output: (GroundMineral, 1),
        profession: Profession::Alchemy,
        menu: processing,
    });

    let components = "Components";

    ret.push(Recipe {
        name: "Empty Flask",
        input: vec![
            (Item::Crafted(PowderedStone), 4),
            (Item::Crafted(PowderedStone), 2),
            (Item::Base(Dust), 2),
        ],
        output: (EmptyFlask, 1),
        profession: Profession::Alchemy,
        menu: components,
    });

    let mixings = "Arcane mixings";

    ret.push(Recipe {
        name: "Ambrosia",
        input: vec![
            (Item::Crafted(EmptyFlask), 1),
            (Item::Crafted(GroundCinnabar), 2),
            (Item::Crafted(GroundHalite), 2),
            (Item::Base(WaterFlask), 2),
            (Item::Base(Apple), 2),
            (Item::Base(Dust), 10),
        ],
        output: (Ambrosia, 1),
        profession: Profession::Alchemy,
        menu: mixings,
    });

    ret.push(Recipe {
        name: "Armor coating",
        input: vec![
            (Item::Crafted(PhilosopherSolution), 1),
            (Item::Crafted(PhilosopherSolution), 1),
            (Item::Crafted(PhilosopherSolution), 1),
            (Item::Crafted(PhilosopherSolution), 1),
        ],
        output: (ArmorCoating, 1),
        profession: Profession::Alchemy,
        menu: mixings,
    });

    ret.push(Recipe {
        name: "Philoshopher stone",
        input: vec![
            (Item::Crafted(PhilosopherSolution), 1),
            (Item::Crafted(PhilosopherSolution), 1),
        ],
        output: (PhilosopherSolution, 1),
        profession: Profession::Alchemy,
        menu: mixings,
    });

    let crafting = "Crafting potions";

    ret.push(Recipe {
        name: "Potion of Saphot",
        input: vec![
            (Item::Crafted(EmptyFlask), 1),
            (Item::Base(Blood), 1),
            (Item::Base(Dust), 2),
            (Item::Crafted(PowderedAnimalBone), 6),
        ],
        output: (PotionOfSapho, 1),
        profession: Profession::Alchemy,
        menu: crafting,
    });

    let harvesting = "Harvesting potions";

    ret.push(Recipe {
        name: "Potion of harvest : Gems and Minerals",
        input: vec![
            (Item::Crafted(EmptyFlask), 1),
            (Item::Base(WaterFlask), 1),
            (Item::Crafted(PowderedAnimalBone), 6),
        ],
        output: (PotionOfHarvestGemsAndMinerals, 1),
        profession: Profession::Alchemy,
        menu: harvesting,
    });

    ret.push(Recipe {
        name: "Potion of harvest : Digging",
        input: vec![
            (Item::Crafted(EmptyFlask), 1),
            (Item::Base(WaterFlask), 1),
            (Item::Group(AnimalMeat), 8),
        ],
        output: (PotionOfHarvestDigging, 1),
        profession: Profession::Alchemy,
        menu: harvesting,
    });

    ret.push(Recipe {
        name: "Potion of harvest : Logging",
        input: vec![
            (Item::Crafted(EmptyFlask), 1),
            (Item::Base(WaterFlask), 1),
            (Item::Group(NonBasicWood), 6),
        ],
        output: (PotionOfHarvestLogging, 1),
        profession: Profession::Alchemy,
        menu: harvesting,
    });

    ret.push(Recipe {
        name: "Potion of harvest : Mining",
        input: vec![
            (Item::Crafted(EmptyFlask), 1),
            (Item::Base(WaterFlask), 1),
            (Item::Group(NonBasicOre), 6),
        ],
        output: (PotionOfHarvestMining, 1),
        profession: Profession::Alchemy,
        menu: harvesting,
    });

    ret.push(Recipe {
        name: "Potion of harvest : Quarrying",
        input: vec![
            (Item::Crafted(EmptyFlask), 1),
            (Item::Base(WaterFlask), 1),
            (Item::Group(NonBasicStone), 6),
        ],
        output: (PotionOfHarvestQuarrying, 1),
        profession: Profession::Alchemy,
        menu: harvesting,
    });

    ret.push(Recipe {
        name: "Potion of harvest : Skinning",
        input: vec![
            (Item::Crafted(EmptyFlask), 1),
            (Item::Base(WaterFlask), 1),
            (Item::Group(Hide), 6),
        ],
        output: (PotionOfHarvestSkinning, 1),
        profession: Profession::Alchemy,
        menu: harvesting,
    });

    let toxin = "Toxins";

    ret.push(Recipe {
        name: "Nature toxin",
        input: vec![
            (Item::Crafted(EmptyFlask), 1),
            (Item::Base(PineNuts), 2),
            (Item::Base(WaterFlask), 1),
        ],
        output: (NatureToxin, 1),
        profession: Profession::Alchemy,
        menu: toxin,
    });

    ret.push(Recipe {
        name: "Poison toxin",
        input: vec![
            (Item::Crafted(EmptyFlask), 1),
            (Item::Base(Grubs), 2),
            (Item::Base(WaterFlask), 1),
        ],
        output: (PoisonToxin, 1),
        profession: Profession::Alchemy,
        menu: toxin,
    });

    ret.push(Recipe {
        name: "Disease toxin",
        input: vec![
            (Item::Crafted(EmptyFlask), 1),
            (Item::Base(BloodWorm), 2),
            (Item::Base(WaterFlask), 1),
        ],
        output: (DiseaseToxin, 1),
        profession: Profession::Alchemy,
        menu: toxin,
    });

    let transmutations = "Transmutations";

    ret.push(Recipe {
        name: "Adhesive fixation",
        input: vec![
            (Item::Base(Adhesive), 1),
            (Item::Base(AppearenceCompendium), 1),
        ],
        output: (AdhesiveFixation, 1),
        profession: Profession::Alchemy,
        menu: transmutations,
    });

    ret.push(Recipe {
        name: "Transmute chaos",
        input: vec![
            (Item::Base(ChaosEmber), 1),
        ],
        output: (TransmuteChaos, 1),
        profession: Profession::Alchemy,
        menu: transmutations,
    });

    ret
}