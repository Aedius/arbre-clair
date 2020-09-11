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

    let stonemasonry_components = "Stonemasonry components";

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