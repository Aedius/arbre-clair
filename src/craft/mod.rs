use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

use crate::craft::BaseResource::*;

pub mod cooking;
pub mod recipe;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct RecipeSummary{
    pub key: &'static str,
    pub name: &'static str,
    pub profession: Profession,
    pub stat: &'static str,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Profession {
    Cooking
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize )]
pub enum Item {
    Base(BaseResource),
    Group(GroupResource),
    Crafted(CraftedResource),
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub name: &'static str,
    pub input: Vec<(Item, i32)>,
    pub output: (CraftedResource, i32),
    pub profession: Profession,
    pub menu: &'static str,
}


impl Recipe {
    pub fn crafted_data(&self) -> Option<CraftedData> {
        self.output.0.get_information()
    }
}

impl Hash for Recipe{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum BaseResource {
    Apple,
    Beeswax,
    Bloodworm,
    Blood,
    Bone,
    Carrot,
    CocoaBean,
    CoffeeBean,
    GroundBlackPepper,
    HotPepper,
    HungerShard,
    MeatAuroch,
    MeatBear,
    MeatBigCat,
    MeatBoar,
    MeatElk,
    MeatSpider,
    MeatWolf,
    MildPepper,
    PineNuts,
    Potato,
    RawMilk,
    SugarCane,
    SeasoningSalt,
    SweetPepper,
    WaterFlask,
    WildRice,
    Onion,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum GroupResource {
    AnimalMeat,
    Herb,
    Mushroom,
    MeatOrMushroom,
    NonBasicOre,
    NonBasicWood,
    Produce,
    Seasoning,
    Ore,
    WildRiceOrGnocchi,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum CraftedResource {
    AppleJuice,
    ArtisanCheese,
    BakedIceCream,
    BasicRoastedMeat,
    BloodwormStew,
    BoneBroth,
    Bread,
    BiscuitsAndGravy,
    BonTippers,
    Butter,
    Cake,
    CampfireMeatyStew,
    CampfireMushroomStew,
    CarrotJuice,
    ChocolateBar,
    ChocolateMilk,
    ChocolateMilkSpicy,
    Coffee,
    Cookie,
    CookieChocolate,
    CookingFoil,
    CrushedHerb,
    EmptyFlask,
    Gnocchi,
    GrilledCheeseSandwich,
    GrilledSandwichAuroch,
    GrilledSandwichBoar,
    GrilledSandwichMushroom,
    GrilledSandwichBear,
    GrilledSandwichElk,
    GrilledSandwichWolf,
    GrilledSandwichSpider,
    GrilledSandwichBigCat,
    IceCream,
    KebabMushroom,
    KebabAuroch,
    KebabBear,
    KebabBigCat,
    KebabBoar,
    KebabElk,
    KebabSpider,
    KebabWolf,
    LargeCookingPot,
    MarsalaStew,
    MeatBurgundy,
    Mead,
    MushroomStew,
    Paella,
    PasteurizedMilk,
    PestoGnocchi,
    PulverizedPotato,
    PotatoFlour,
    PotRoast,
    RedWine,
    RoastingStick,
    RoastedPig,
    RoastedProduce,
    SeasonedMushroom,
    SumptuousPotPie,
    TrailMix,
    Yeast,
}

pub struct CraftedData {
    pub key: &'static str,
    pub name: &'static str,
    pub stat: &'static str,
}


impl CraftedResource {
    pub fn get_information(&self) -> Option<CraftedData> {
        match self {
            CraftedResource::AppleJuice => {
                Some(
                    CraftedData {
                        name: "Apple Juice",
                        stat: "Find Weak Spot +5%",
                        key: "AppleJuice",
                    }
                )
            }
            CraftedResource::ArtisanCheese => {
                Some(
                    CraftedData {
                        name: "Artisan Cheese",
                        stat: "Harvest Critical Chance +5%",
                        key: "ArtisanCheese",
                    }
                )
            }
            CraftedResource::BakedIceCream => {
                Some(
                    CraftedData {
                        name: "Baked Ice Cream",
                        stat: "Chance to do Fire damage 5%",
                        key: "BakedIceCream",
                    }
                )
            }
            CraftedResource::BasicRoastedMeat => { None }
            CraftedResource::BloodwormStew => {
                Some(
                    CraftedData {
                        name: "Bloodworm Stew",
                        stat: "Food Regen +10, grants Trailblazer/master",
                        key: "BloodwormStew",
                    }
                )
            }
            CraftedResource::BoneBroth => {
                Some(
                    CraftedData {
                        name: "Bone Broth",
                        stat: "Incoming Healing +3%",
                        key: "BoneBroth",
                    }
                )
            }
            CraftedResource::Bread => {
                Some(
                    CraftedData {
                        name: "Bread",
                        stat: "Pathfinding +10%",
                        key: "Bread",
                    }
                )
            }
            CraftedResource::BiscuitsAndGravy => {
                Some(
                    CraftedData {
                        name: "Biscuits and Gravy",
                        stat: "Food Regen Rate +20",
                        key: "BiscuitsAndGravy",
                    }
                )
            }
            CraftedResource::BonTippers => {
                Some(
                    CraftedData {
                        name: "Bon Tippers",
                        stat: "Exp Difficulty Mod +15 on next experiment",
                        key: "BonTippers",
                    }
                )
            }
            CraftedResource::Butter => {
                Some(
                    CraftedData {
                        name: "Fresh Butter",
                        stat: "Harvest Chance: Cutting Grit +2%",
                        key: "Butter",
                    }
                )
            }
            CraftedResource::Cake => {
                Some(
                    CraftedData {
                        name: "Cake",
                        stat: "Health +150, Stamina -15",
                        key: "Cake",
                    }
                )
            }
            CraftedResource::CampfireMeatyStew => {
                Some(
                    CraftedData {
                        name: "Campfire Meaty Stew",
                        stat: "Mounted Movement Speed +5%",
                        key: "CampfireMeatyStew",
                    }
                )
            }
            CraftedResource::CampfireMushroomStew => {
                Some(
                    CraftedData {
                        name: "Campfire Mushroom Stew",
                        stat: "Harvest Critical Chance +5%",
                        key: "CampfireMushroomStew",
                    }
                )
            }
            CraftedResource::CarrotJuice => {
                Some(
                    CraftedData {
                        name: "Carrot Juice",
                        stat: "Ranged Distance Bonus +3m",
                        key: "CarrotJuice",
                    }
                )
            }
            CraftedResource::ChocolateBar => {
                Some(
                    CraftedData {
                        name: "Chocalate Bar",
                        stat: "Combat Movement +5% / Health -150",
                        key: "ChocolateBar",
                    }
                )
            }
            CraftedResource::ChocolateMilk => {
                Some(
                    CraftedData {
                        name: "Chocolate Milk",
                        stat: "HP +150, Stam -15",
                        key: "ChocolateMilk",
                    }
                )
            }
            CraftedResource::ChocolateMilkSpicy => {
                Some(
                    CraftedData {
                        name: "Spicy Chocolate Milk",
                        stat: "Elemental Damage +3%",
                        key: "ChocolateMilkSpicy",
                    }
                )
            }
            CraftedResource::Coffee => {
                Some(
                    CraftedData {
                        name: "Coffee",
                        stat: "Stamina +10",
                        key: "Coffee",
                    }
                )
            }
            CraftedResource::Cookie => {
                Some(
                    CraftedData {
                        name: "Cookies",
                        stat: "Combat Movement Speed +5%",
                        key: "Cookie",
                    }
                )
            }
            CraftedResource::CookieChocolate => {
                Some(
                    CraftedData {
                        name: "Chocolate Cookies",
                        stat: "Combat Movement Speed +5%",
                        key: "CookieChocolate",
                    }
                )
            }
            CraftedResource::CookingFoil => { None }
            CraftedResource::CrushedHerb => { None }
            CraftedResource::EmptyFlask => { None }
            CraftedResource::Gnocchi => {
                Some(
                    CraftedData {
                        name: "Gnocchi",
                        stat: "Pathfinding Speed +10%",
                        key: "Gnocchi",
                    }
                )
            }
            CraftedResource::GrilledCheeseSandwich => {
                Some(
                    CraftedData {
                        name: "Grilled Cheese Sandwich",
                        stat: "Harvest Critical Amount +1",
                        key: "GrilledCheeseSandwich",
                    }
                )
            }
            CraftedResource::GrilledSandwichAuroch => {
                Some(
                    CraftedData {
                        name: "Auroch Meat Sandwich",
                        stat: "Fire Armor Bonus +3%",
                        key: "GrilledSandwichAuroch",
                    }
                )
            }
            CraftedResource::GrilledSandwichBoar => {
                Some(
                    CraftedData {
                        name: "Boar Meat Sandwich",
                        stat: "Ice Armor Bonus +3%",
                        key: "GrilledSandwichBoar",
                    }
                )
            }
            CraftedResource::GrilledSandwichMushroom => {
                Some(
                    CraftedData {
                        name: "Mushroom Sandwich",
                        stat: "Harvest Critical Amount +1",
                        key: "GrilledSandwichMushroom",
                    }
                )
            }
            CraftedResource::GrilledSandwichBear => {
                Some(
                    CraftedData {
                        name: "Bear Meat Sandwich",
                        stat: "Disease Armor Bonus +3%",
                        key: "GrilledSandwichBear",
                    }
                )
            }
            CraftedResource::GrilledSandwichElk => {
                Some(
                    CraftedData {
                        name: "Elk Meat Sandwich",
                        stat: "Ice Armor Bonus +3%",
                        key: "GrilledSandwichElk",
                    }
                )
            }
            CraftedResource::GrilledSandwichWolf => {
                Some(
                    CraftedData {
                        name: "Wolf Meat Sandwich",
                        stat: "Electric Armor Bonus +3%",
                        key: "GrilledSandwichWolf",
                    }
                )
            }
            CraftedResource::GrilledSandwichSpider => {
                Some(
                    CraftedData {
                        name: "Spider Meat Sandwich",
                        stat: "Poison Armor Bonus +3%",
                        key: "GrilledSandwichSpider",
                    }
                )
            }
            CraftedResource::GrilledSandwichBigCat => {
                Some(
                    CraftedData {
                        name: "Big Cat Meat Sandwidch",
                        stat: "Nature Armor Bonus +3%",
                        key: "GrilledSandwichBigCat",
                    }
                )
            }
            CraftedResource::IceCream => {
                Some(
                    CraftedData {
                        name: "Ice Cream",
                        stat: "Chance to do Ice damage 5%",
                        key: "IceCream",
                    }
                )
            }
            CraftedResource::KebabMushroom => {
                Some(
                    CraftedData {
                        name: "Mushroom Kebab",
                        stat: "Plentiful Harvest Wood +1",
                        key: "KebabMushroom",
                    }
                )
            }
            CraftedResource::KebabAuroch => {
                Some(
                    CraftedData {
                        name: "Auroch Meat Kebab",
                        stat: "Plentiful Harvest Ore +1",
                        key: "KebabAuroch",
                    }
                )
            }
            CraftedResource::KebabBear => {
                Some(
                    CraftedData {
                        name: "Bear Meat Kebab",
                        stat: "Plentiful Harvest Wood +1",
                        key: "KebabBear",
                    }
                )
            }
            CraftedResource::KebabBigCat => {
                Some(
                    CraftedData {
                        name: "Big Cat Meat Kebab",
                        stat: "Plentiful Harvest Ore +1",
                        key: "KebabBigCat",
                    }
                )
            }
            CraftedResource::KebabBoar => {
                Some(
                    CraftedData {
                        name: "Boar Meat Kebab",
                        stat: "Plentiful Harvest Stone +1",
                        key: "KebabBoar",
                    }
                )
            }
            CraftedResource::KebabElk => {
                Some(
                    CraftedData {
                        name: "Elk Meat Kebab",
                        stat: "Plentiful Harvest Stone +1",
                        key: "KebabElk",
                    }
                )
            }
            CraftedResource::KebabSpider => {
                Some(
                    CraftedData {
                        name: "Spider Meat Kebab",
                        stat: "Plentiful Harvest Graves +1",
                        key: "KebabSpider",
                    }
                )
            }
            CraftedResource::KebabWolf => {
                Some(
                    CraftedData {
                        name: "Wolf Meat Kebab",
                        stat: "Plentiful Harvest Animal +1",
                        key: "KebabWolf",
                    }
                )
            }
            CraftedResource::LargeCookingPot => { None }
            CraftedResource::MarsalaStew => {
                Some(
                    CraftedData {
                        name: "Marsala Stew",
                        stat: "Bard Songs +6 seconds",
                        key: "MarsalaStew",
                    }
                )
            }
            CraftedResource::MeatBurgundy => {
                Some(
                    CraftedData {
                        name: "Meat Burgundy",
                        stat: "Basic Attack Damage +10%",
                        key: "MeatBurgundy",
                    }
                )
            }
            CraftedResource::Mead => { None }
            CraftedResource::MushroomStew => {
                Some(
                    CraftedData {
                        name: "Mushroom Stew",
                        stat: "Harvest Critical Chance: All 5%",
                        key: "MushroomStew",
                    }
                )
            }
            CraftedResource::Paella => {
                Some(
                    CraftedData {
                        name: "Paella",
                        stat: "Ranged Power Damage +5%",
                        key: "Paella",
                    }
                )
            }
            CraftedResource::PasteurizedMilk => {
                Some(
                    CraftedData {
                        name: "Pasteurized Milk",
                        stat: "Incoming Healing +3%",
                        key: "PasteurizedMilk",
                    }
                )
            }
            CraftedResource::PestoGnocchi => {
                Some(
                    CraftedData {
                        name: "Pesto Gnocchi",
                        stat: "Harvest Pips +0.5",
                        key: "PestoGnocchi",
                    }
                )
            }
            CraftedResource::PulverizedPotato => { None }
            CraftedResource::PotatoFlour => { None }
            CraftedResource::PotRoast => {
                Some(
                    CraftedData {
                        name: "Pot Roast",
                        stat: "Healing Bonus +3%",
                        key: "PotRoast",
                    }
                )
            }
            CraftedResource::RedWine => {
                Some(
                    CraftedData {
                        name: "Red Wine",
                        stat: "Stamina +10, Food Regen -20",
                        key: "RedWine",
                    }
                )
            }
            CraftedResource::RoastingStick => { None }
            CraftedResource::RoastedPig => {
                Some(
                    CraftedData {
                        name: "Roasted Pig",
                        stat: "Healing Bonus +3%",
                        key: "RoastedPig",
                    }
                )
            }
            CraftedResource::RoastedProduce => { None }
            CraftedResource::SeasonedMushroom => {
                Some(
                    CraftedData {
                        name: "Seasoned Mushroom",
                        stat: "Harvest Chance: Soulgems +2%",
                        key: "SeasonedMushroom",
                    }
                )
            }
            CraftedResource::SumptuousPotPie => {
                Some(
                    CraftedData {
                        name: "Sumptuous Pot Pie",
                        stat: "General Crafting Exp. Points +1",
                        key: "SumptuousPotPie",
                    }
                )
            }
            CraftedResource::TrailMix => {
                Some(
                    CraftedData {
                        name: "Trail Mix",
                        stat: "Harvest Chance: Survivalist +3%",
                        key: "TrailMix",
                    }
                )
            }
            CraftedResource::Yeast => { None }
        }
    }
}


impl GroupResource {
    fn get_base(self) -> Vec<BaseResource> {
        match self {
            GroupResource::AnimalMeat => {
                vec![
                    MeatBear,
                    MeatBigCat,
                    MeatBoar,
                    MeatElk,
                    MeatSpider,
                    MeatWolf,
                ]
            }
            GroupResource::Herb => {
                vec![
                    Onion
                ]
            }
            GroupResource::Mushroom => {
                vec![
                    //TODO
                ]
            }
            GroupResource::MeatOrMushroom => {
                vec![
                    MeatBear,
                    MeatBigCat,
                    MeatBoar,
                    MeatElk,
                    MeatSpider,
                    MeatWolf,
                ]
            }
            GroupResource::NonBasicOre => {
                vec![
                    //TODO
                ]
            }
            GroupResource::NonBasicWood => {
                vec![
                    //TODO
                ]
            }
            GroupResource::Produce => {
                vec![
                    Apple,
                    Carrot,
                    Potato,
                    // Mushroom,
                    Onion,
                    HotPepper,
                    MildPepper,
                    HotPepper
                ]
            }
            GroupResource::Seasoning => {
                vec![
                    //TODO
                ]
            }
            GroupResource::Ore => {
                vec![
                    //TODO
                ]
            }
            GroupResource::WildRiceOrGnocchi => {
                vec![
                    WildRice,
                    // Gnocchi,
                ]
            }
        }
    }
    pub fn get_name(&self) -> &'static str {
        match self {
            GroupResource::AnimalMeat => { "AnimalMeat" }
            GroupResource::Herb => { "Herb" }
            GroupResource::Mushroom => { "Mushroom" }
            GroupResource::MeatOrMushroom => { "MeatOrMushroom" }
            GroupResource::NonBasicOre => { "NonBasicOre" }
            GroupResource::NonBasicWood => { "NonBasicWood" }
            GroupResource::Produce => { "Produce" }
            GroupResource::Seasoning => { "Seasoning" }
            GroupResource::Ore => { "Ore" }
            GroupResource::WildRiceOrGnocchi => { "WildRiceOrGnocchi" }
        }
    }
}

impl BaseResource {
    pub fn get_name(&self) -> &'static str {
        match self {
            Apple => { "Apple" }
            Beeswax => { "Beeswax" }
            Bloodworm => { "Bloodworm" }
            Blood => { "Blood" }
            Bone => { "Bone" }
            Carrot => { "Carrot" }
            CocoaBean => { "CocoaBean" }
            CoffeeBean => { "CoffeeBean" }
            GroundBlackPepper => { "GroundBlackPepper" }
            HotPepper => { "HotPepper" }
            HungerShard => { "HungerShard" }
            MeatAuroch => { "MeatAuroch" }
            MeatBear => { "MeatBear" }
            MeatBigCat => { "MeatBigCat" }
            MeatBoar => { "MeatBoar" }
            MeatElk => { "MeatElk" }
            MeatSpider => { "MeatSpider" }
            MeatWolf => { "MeatWolf" }
            MildPepper => { "MildPepper" }
            PineNuts => { "PineNuts" }
            Potato => { "Potato" }
            RawMilk => { "RawMilk" }
            SugarCane => { "SugarCane" }
            SeasoningSalt => { "SeasoningSalt" }
            SweetPepper => { "SweetPepper" }
            WaterFlask => { "WaterFlask" }
            WildRice => { "WildRice" }
            Onion => { "Onion" }
        }
    }
}

impl Profession {
    pub fn get_name(&self) -> &'static str {
        match self { Profession::Cooking => { "Cooking" } }
    }
}