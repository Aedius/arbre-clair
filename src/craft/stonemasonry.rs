use crate::craft::{Item, Profession, Recipe};

use crate::craft::BaseResource::*;
use crate::craft::CraftedResource::*;
use crate::craft::GroupResource::*;

pub fn get_recipe() -> Vec<Recipe> {
    let mut ret = Vec::new();

    let architecture_components = "Architecture components";

    ret.push(Recipe {
        name: "Architectural Arches",
        input: vec![
            (Item::Crafted(Bricks), 2),
            (Item::Crafted(Frame), 2),
            (Item::Crafted(Nails), 1),
        ],
        output: (ArchitecturalArches, 1),
        profession: Profession::Stonemasonry,
        menu: architecture_components,
    });

    ret.push(Recipe {
        name: "Architectural Walls",
        input: vec![
            (Item::Crafted(Bricks), 2),
            (Item::Crafted(Frame), 4),
            (Item::Crafted(Nails), 1),
            (Item::Crafted(WallInsulation), 3),
        ],
        output: (ArchitecturalWall, 1),
        profession: Profession::Stonemasonry,
        menu: architecture_components,
    });

    ret.push(Recipe {
        name: "Foundation Segment",
        input: vec![
            (Item::Crafted(Bricks), 4),
            (Item::Crafted(Gravel), 2),
        ],
        output: (FoundationSegment, 1),
        profession: Profession::Stonemasonry,
        menu: architecture_components,
    });

    ret.push(Recipe {
        name: "Frame",
        input: vec![
            (Item::Group(NonBasicWood), 20),
            (Item::Crafted(Nails), 2),
            (Item::Group(NonBasicWood), 20),
            (Item::Group(NonBasicWood), 20),
            (Item::Group(NonBasicWood), 20),
        ],
        output: (Frame, 1),
        profession: Profession::Stonemasonry,
        menu: architecture_components,
    });

    ret.push(Recipe {
        name: "Roof Segment",
        input: vec![
            (Item::Crafted(Nails), 4),
            (Item::Crafted(WoodShingles), 4),
        ],
        output: (RoofSegment, 1),
        profession: Profession::Stonemasonry,
        menu: architecture_components,
    });

    ret.push(Recipe {
        name: "Roof Segment",
        input: vec![
            (Item::Group(Hide), 10),
            (Item::Crafted(Mulch), 4),
        ],
        output: (WallInsulation, 1),
        profession: Profession::Stonemasonry,
        menu: architecture_components,
    });

    ret.push(Recipe {
        name: "Wall Section",
        input: vec![
            (Item::Crafted(WoodStakes), 8),
            (Item::Crafted(Frame), 2),
        ],
        output: (WoodenWallSection, 1),
        profession: Profession::Stonemasonry,
        menu: architecture_components,
    });

    ret.push(Recipe {
        name: "Wall Section",
        input: vec![
            (Item::Crafted(Bricks), 8),
            (Item::Crafted(Frame), 2),
        ],
        output: (StoneWallSection, 1),
        profession: Profession::Stonemasonry,
        menu: architecture_components,
    });

    let geomancy_components = "Geomancy components";

    ret.push(Recipe {
        name: "Gravel",
        input: vec![
            (Item::Group(NonBasicStone), 20),
        ],
        output: (Gravel, 1),
        profession: Profession::Stonemasonry,
        menu: geomancy_components,
    });

    ret.push(Recipe {
        name: "Mulch",
        input: vec![
            (Item::Group(NonBasicWood), 20),
        ],
        output: (Mulch, 1),
        profession: Profession::Stonemasonry,
        menu: geomancy_components,
    });

    ret.push(Recipe {
        name: "Ore Concentrate",
        input: vec![
            (Item::Group(NonBasicOre), 20),
        ],
        output: (OreConcentrate, 1),
        profession: Profession::Stonemasonry,
        menu: geomancy_components,
    });

    ret.push(Recipe {
        name: "Soil",
        input: vec![
            (Item::Crafted(Gravel), 8),
            (Item::Crafted(Mulch), 8),
            (Item::Crafted(OreConcentrate), 8),
        ],
        output: (Soil, 1),
        profession: Profession::Stonemasonry,
        menu: geomancy_components,
    });

    let stonemasonry_components = "Stonemasonry components";

    ret.push(Recipe {
        name: "Bricks",
        input: vec![
            (Item::Group(NonBasicStone), 20),
            (Item::Group(NonBasicStone), 20),
            (Item::Group(NonBasicStone), 20),
            (Item::Base(Dust), 5),
        ],
        output: (Bricks, 1),
        profession: Profession::Stonemasonry,
        menu: stonemasonry_components,
    });

    ret.push(Recipe {
        name: "Carpentry Nails",
        input: vec![
            (Item::Group(NonBasicOre), 15),
            (Item::Group(NonBasicOre), 15),
            (Item::Group(NonBasicOre), 15),
            (Item::Base(Dust), 5),
        ],
        output: (Nails, 1),
        profession: Profession::Stonemasonry,
        menu: stonemasonry_components,
    });

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
        name: "FloorTiles",
        input: vec![
            (Item::Group(NonBasicStoneOrWood), 20),
            (Item::Group(NonBasicStoneOrWood), 20),
            (Item::Group(NonBasicStoneOrWood), 20),
            (Item::Base(Dust), 5),
        ],
        output: (FloorTiles, 1),
        profession: Profession::Stonemasonry,
        menu: stonemasonry_components,
    });

    ret.push(Recipe {
        name: "Grinding wheel",
        input: vec![
            (Item::Group(NonBasicStone), 2),
			(Item::Group(NonBasicStone), 2),
			(Item::Group(NonBasicStone), 2),
        ],
        output: (GrindingWheel, 1),
        profession: Profession::Stonemasonry,
        menu: stonemasonry_components,
    });

    ret.push(Recipe {
        name: "Polishing paste",
        input: vec![
            (Item::Crafted(PowderedStone), 4),
            (Item::Base(WaterFlask), 1),
        ],
        output: (DiamondCuttingBlade, 1),
        profession: Profession::Stonemasonry,
        menu: stonemasonry_components,
    });

    ret.push(Recipe {
        name: "Quality Assurance Control Kit",
        input: vec![
            (Item::Group(NonBasicOre), 20),
            (Item::Base(Dust), 15),
            (Item::Group(NonBasicWood), 20),
            (Item::Group(Hide), 20),
            (Item::Group(NonBasicStone), 20),
        ],
        output: (QualityAssurance, 1),
        profession: Profession::Stonemasonry,
        menu: stonemasonry_components,
    });

    ret.push(Recipe {
        name: "Wood Shingles",
        input: vec![
            (Item::Group(NonBasicWood), 20),
            (Item::Group(NonBasicWood), 20),
            (Item::Group(NonBasicWood), 20),
            (Item::Base(Dust), 15),
        ],
        output: (WoodShingles, 1),
        profession: Profession::Stonemasonry,
        menu: stonemasonry_components,
    });

    ret.push(Recipe {
        name: "Wood Stakes",
        input: vec![
            (Item::Group(NonBasicWood), 20),
            (Item::Group(NonBasicWood), 20),
            (Item::Group(NonBasicWood), 20),
            (Item::Base(Dust), 15),
        ],
        output: (WoodStakes, 1),
        profession: Profession::Stonemasonry,
        menu: stonemasonry_components,
    });

    let crafting_station = "Crafting Station";

    ret.push(Recipe {
        name: "Crafting Station : Runemaking",
        input: vec![
            (Item::Base(Dust), 15),
            (Item::Group(NonBasicOre), 18),
            (Item::Group(NonBasicWood), 18),
            (Item::Group(NonBasicStone), 18),
            (Item::Group(NonBasicWood), 18),
            (Item::Group(NonBasicStone), 18),
            (Item::Group(NonBasicOre), 18),
        ],
        output: (StationRunemaking, 1),
        profession: Profession::Stonemasonry,
        menu: crafting_station,
    });

    let vendor = "Vendor and Stalls";

    ret.push(Recipe {
        name: "Rune themed stall",
        input: vec![
            (Item::Crafted(ArchitecturalArches), 1),
            (Item::Crafted(FoundationSegment), 1),
            (Item::Group(NonBasicWood), 15),
            (Item::Crafted(Frame), 2),
            (Item::Base(Dust), 10),
            (Item::Group(HarvestingTools), 1),
        ],
        output: (StallRune, 1),
        profession: Profession::Stonemasonry,
        menu: vendor,
    });

    let backers_reward = "Backer rewards";

    ret.push(Recipe {
        name: "Caldera parcel",
        input: vec![
            (Item::Crafted(BackerCapitalParcel), 1),
            (Item::Crafted(BackerCapitalParcel), 1),
            (Item::Crafted(BackerCapitalParcel), 1),
        ],
        output: (BackerCalderaParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });

    ret.push(Recipe {
        name: "Capital parcel",
        input: vec![
            (Item::Crafted(BackerCityParcel), 1),
            (Item::Crafted(BackerCityParcel), 1),
        ],
        output: (BackerCapitalParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });

    ret.push(Recipe {
        name: "City parcel",
        input: vec![
            (Item::Crafted(BackerFiefParcel), 1),
            (Item::Crafted(BackerFiefParcel), 1),
        ],
        output: (BackerCityParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });

    ret.push(Recipe {
        name: "Estate parcel",
        input: vec![
            (Item::Crafted(BackerWoodlandHillsParcel), 1),
            (Item::Crafted(BackerWoodlandGroveParcel), 1),
        ],
        output: (BackerEstateParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });

    ret.push(Recipe {
        name: "Fief parcel",
        input: vec![
            (Item::Crafted(BackerTownParcel), 1),
            (Item::Crafted(BackerShireParcel), 1),
        ],
        output: (BackerFiefParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });

    ret.push(Recipe {
        name: "Hamlet parcel",
        input: vec![
            (Item::Crafted(BackerEstateParcel), 1),
            (Item::Crafted(BackerWoodlandHillsParcel), 1),
        ],
        output: (BackerHamletParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });

    ret.push(Recipe {
        name: "Homestead parcel",
        input: vec![
            (Item::Base(BackerGrasslandsParcel), 1),
            (Item::Base(BackerGrasslandsParcel), 1),
            (Item::Base(BackerGrasslandsParcel), 1),
        ],
        output: (BackerHomesteadParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });

    ret.push(Recipe {
        name: "Mountain Citadel parcel",
        input: vec![
            (Item::Crafted(BackerCapitalParcel), 1),
            (Item::Crafted(BackerCapitalParcel), 1),
            (Item::Crafted(BackerCapitalParcel), 1),
        ],
        output: (BackerMountainCitadelParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });

    ret.push(Recipe {
        name: "Shire Parcel",
        input: vec![
            (Item::Crafted(BackerHamletParcel), 1),
            (Item::Crafted(BackerHamletParcel), 1),
        ],
        output: (BackerShireParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });

    ret.push(Recipe {
        name: "Shire Parcel",
        input: vec![
            (Item::Crafted(BackerVillageParcel), 1),
            (Item::Crafted(BackerHamletParcel), 1),
        ],
        output: (BackerTownParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });

    ret.push(Recipe {
        name: "Village Parcel",
        input: vec![
            (Item::Crafted(BackerShireParcel), 1),
            (Item::Crafted(BackerHamletParcel), 1),
        ],
        output: (BackerVillageParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });

    ret.push(Recipe {
        name: "Woodland Creek Parcel",
        input: vec![
            (Item::Crafted(BackerEstateParcel), 1),
            (Item::Crafted(BackerWoodlandGroveParcel), 1),
        ],
        output: (BackerWoodlandCreekParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });
    ret.push(Recipe {
        name: "Woodland Grove Parcel",
        input: vec![
            (Item::Base(BackerGrasslandsParcel), 1),
            (Item::Base(BackerGrasslandsParcel), 1),
            (Item::Base(BackerGrasslandsParcel), 1),
        ],
        output: (BackerWoodlandGroveParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });
    ret.push(Recipe {
        name: "Woodland Hills Parcel",
        input: vec![
            (Item::Crafted(BackerHomesteadParcel), 1),
            (Item::Crafted(BackerWoodlandGroveParcel), 1),
        ],
        output: (BackerWoodlandHillsParcel, 1),
        profession: Profession::Stonemasonry,
        menu: backers_reward,
    });


    ret
}