use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

use crate::craft::BaseResource::*;

pub mod alchemy;
pub mod cooking;
pub mod jewelry;
pub mod stonemasonry;
pub mod recipe;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct RecipeSummary {
    pub key: &'static str,
    pub name: &'static str,
    pub profession: Profession,
    pub stat: &'static str,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Profession {
    Alchemy,
    Cooking,
    Jewelry,
    Stonemasonry,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
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
    pub fn crafted_data(&self) -> CraftedData {
        self.output.0.get_information()
    }
}

impl Hash for Recipe {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum BaseResource {
    Dust,
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
    MushroomButton,
    MushroomChanterelle,
    Copper,
    Iron,
    Tin,
    Silver,
    Aurelium,
    Slag,
    Oak,
    Birch,
    Spruce,
    Ash,
    Yew,
    Garlic,
    Granite,
    Limestone,
    Marble,
    Slate,
    Travertin,
    RoughSoulgem,
    CuttingGrit,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum GroupResource {
    AnimalMeat,
    Herb,
    Mushroom,
    MeatOrMushroom,
    NonBasicOre,
    NonBasicWood,
    NonBasicStone,
    Produce,
    Seasoning,
    Ore,
    WildRiceOrGnocchi,
    Stone,
    RoughGem,
    BloodOrWater,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum CraftedResource {
    AppleJuice,
    ArtisanCheese,
    BakedIceCream,
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
    CookingFoil,
    CrushedHerb,
    EmptyFlask,
    Gnocchi,
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
    MushroomStew,
    Paella,
    PasteurizedMilk,
    PestoGnocchi,
    PulverizedPotato,
    PotatoFlour,
    PotRoast,
    RedWine,
    RoastingStick,
    RoastedProduce,
    SumptuousPotPie,
    TrailMix,
    Yeast,
    PowderedStone,
    DiamondCuttingBlade,
    CutGemstoneBaguette,
    CutGemstoneFocusing,
    CutGemstoneMarquise,
    CutGemstoneOval,
    CutGemstoneRound,
    CutGemstoneSquare,
    CutGemstoneTriangle,
    ChaosEmberCuttingBlade,
    ExactingCutGemstoneBriolette,
    ExactingCutGemstoneHeart,
    ExactingCutGemstoneTrillion,
    CutGem,
    GrindingWheel,
    GrindGemstone,
    PolishingPaste,
    PolishGemstone,
    PolishSoulgem,
    NecklaceBail,
    NecklaceChain,
    Necklace,
    RingSetting,
    RingBand,
    Ring,
    PolishedExactingGemstone,
    PolishedGemstone,
}

pub struct CraftedData {
    pub key: &'static str,
    pub description: &'static str,
}

impl Item {
    pub fn get_name(&self) -> &'static str {
        match self {
            Item::Base(base) => { base.get_name() }
            Item::Group(group) => { group.get_name() }
            Item::Crafted(crafted) => { crafted.get_name() }
        }
    }
}

impl CraftedResource {
    pub fn get_information(&self) -> CraftedData {
        match self {
            CraftedResource::AppleJuice => {
                CraftedData {
                    description: "Find Weak Spot +5%",
                    key: "AppleJuice",
                }
            }
            CraftedResource::ArtisanCheese => {
                CraftedData {
                    description: "Harvest Critical Chance +5%",
                    key: "ArtisanCheese",
                }
            }
            CraftedResource::BakedIceCream => {
                CraftedData {
                    description: "Chance to do Fire damage 5%",
                    key: "BakedIceCream",
                }
            }
            CraftedResource::BoneBroth => {
                CraftedData {
                    description: "Incoming Healing +3%",
                    key: "BoneBroth",
                }
            }
            CraftedResource::Bread => {
                CraftedData {
                    description: "Pathfinding +10%",
                    key: "Bread",
                }
            }
            CraftedResource::BiscuitsAndGravy => {
                CraftedData {
                    description: "Food Regen Rate +20",
                    key: "BiscuitsAndGravy",
                }
            }
            CraftedResource::BonTippers => {
                CraftedData {
                    description: "Exp Difficulty Mod +15 on next experiment",
                    key: "BonTippers",
                }
            }
            CraftedResource::Butter => {
                CraftedData {
                    description: "Harvest Chance: Cutting Grit +2%",
                    key: "Butter",
                }
            }
            CraftedResource::Cake => {
                CraftedData {
                    description: "Health +150, Stamina -15",
                    key: "Cake",
                }
            }
            CraftedResource::CampfireMeatyStew => {
                CraftedData {
                    description: "Mounted Movement Speed +5%",
                    key: "CampfireMeatyStew",
                }
            }
            CraftedResource::CampfireMushroomStew => {
                CraftedData {
                    description: "Harvest Critical Chance +5%",
                    key: "CampfireMushroomStew",
                }
            }
            CraftedResource::CarrotJuice => {
                CraftedData {
                    description: "Ranged Distance Bonus +3m",
                    key: "CarrotJuice",
                }
            }
            CraftedResource::ChocolateBar => {
                CraftedData {
                    description: "Combat Movement +5% / Health -150",
                    key: "ChocolateBar",
                }
            }
            CraftedResource::ChocolateMilk => {
                CraftedData {
                    description: "HP +150, Stam -15",
                    key: "ChocolateMilk",
                }
            }
            CraftedResource::ChocolateMilkSpicy => {
                CraftedData {
                    description: "Elemental Damage +3%",
                    key: "ChocolateMilkSpicy",
                }
            }
            CraftedResource::Coffee => {
                CraftedData {
                    description: "Stamina +10",
                    key: "Coffee",
                }
            }
            CraftedResource::CookingFoil => {
                CraftedData {
                    description: "",
                    key: "CookingFoil",
                }
            }
            CraftedResource::CrushedHerb => {
                CraftedData {
                    description: "",
                    key: "CrushedHerb",
                }
            }
            CraftedResource::EmptyFlask => {
                CraftedData {
                    description: "",
                    key: "EmptyFlask",
                }
            }
            CraftedResource::Gnocchi => {
                CraftedData {
                    description: "Pathfinding Speed +10%",
                    key: "Gnocchi",
                }
            }
            CraftedResource::GrilledSandwichAuroch => {
                CraftedData {
                    description: "Fire Armor Bonus +3%",
                    key: "GrilledSandwichAuroch",
                }
            }
            CraftedResource::GrilledSandwichBoar => {
                CraftedData {
                    description: "Ice Armor Bonus +3%",
                    key: "GrilledSandwichBoar",
                }
            }
            CraftedResource::GrilledSandwichMushroom => {
                CraftedData {
                    description: "Harvest Critical Amount +1",
                    key: "GrilledSandwichMushroom",
                }
            }
            CraftedResource::GrilledSandwichBear => {
                CraftedData {
                    description: "Disease Armor Bonus +3%",
                    key: "GrilledSandwichBear",
                }
            }
            CraftedResource::GrilledSandwichElk => {
                CraftedData {
                    description: "Ice Armor Bonus +3%",
                    key: "GrilledSandwichElk",
                }
            }
            CraftedResource::GrilledSandwichWolf => {
                CraftedData {
                    description: "Electric Armor Bonus +3%",
                    key: "GrilledSandwichWolf",
                }
            }
            CraftedResource::GrilledSandwichSpider => {
                CraftedData {
                    description: "Poison Armor Bonus +3%",
                    key: "GrilledSandwichSpider",
                }
            }
            CraftedResource::GrilledSandwichBigCat => {
                CraftedData {
                    description: "Nature Armor Bonus +3%",
                    key: "GrilledSandwichBigCat",
                }
            }
            CraftedResource::IceCream => {
                CraftedData {
                    description: "Chance to do Ice damage 5%",
                    key: "IceCream",
                }
            }
            CraftedResource::KebabMushroom => {
                CraftedData {
                    description: "Plentiful Harvest Wood +1",
                    key: "KebabMushroom",
                }
            }
            CraftedResource::KebabAuroch => {
                CraftedData {
                    description: "Plentiful Harvest Ore +1",
                    key: "KebabAuroch",
                }
            }
            CraftedResource::KebabBear => {
                CraftedData {
                    description: "Plentiful Harvest Wood +1",
                    key: "KebabBear",
                }
            }
            CraftedResource::KebabBigCat => {
                CraftedData {
                    description: "Plentiful Harvest Ore +1",
                    key: "KebabBigCat",
                }
            }
            CraftedResource::KebabBoar => {
                CraftedData {
                    description: "Plentiful Harvest Stone +1",
                    key: "KebabBoar",
                }
            }
            CraftedResource::KebabElk => {
                CraftedData {
                    description: "Plentiful Harvest Stone +1",
                    key: "KebabElk",
                }
            }
            CraftedResource::KebabSpider => {
                CraftedData {
                    description: "Plentiful Harvest Graves +1",
                    key: "KebabSpider",
                }
            }
            CraftedResource::KebabWolf => {
                CraftedData {
                    description: "Plentiful Harvest Animal +1",
                    key: "KebabWolf",
                }
            }
            CraftedResource::LargeCookingPot => {
                CraftedData {
                    description: "",
                    key: "LargeCookingPot",
                }
            }
            CraftedResource::MarsalaStew => {
                CraftedData {
                    description: "Bard Songs +6 seconds",
                    key: "MarsalaStew",
                }
            }
            CraftedResource::MeatBurgundy => {
                CraftedData {
                    description: "Basic Attack Damage +10%",
                    key: "MeatBurgundy",
                }
            }
            CraftedResource::MushroomStew => {
                CraftedData {
                    description: "Harvest Critical Chance: All 5%",
                    key: "MushroomStew",
                }
            }
            CraftedResource::Paella => {
                CraftedData {
                    description: "Ranged Power Damage +5%",
                    key: "Paella",
                }
            }
            CraftedResource::PasteurizedMilk => {
                CraftedData {
                    description: "Incoming Healing +3%",
                    key: "PasteurizedMilk",
                }
            }
            CraftedResource::PestoGnocchi => {
                CraftedData {
                    description: "Harvest Pips +0.5",
                    key: "PestoGnocchi",
                }
            }
            CraftedResource::PulverizedPotato => {
                CraftedData {
                    description: "",
                    key: "PulverizedPotato",
                }
            }
            CraftedResource::PotatoFlour => {
                CraftedData {
                    description: "",
                    key: "PotatoFlour",
                }
            }
            CraftedResource::PotRoast => {
                CraftedData {
                    description: "Healing Bonus +3%",
                    key: "PotRoast",
                }
            }
            CraftedResource::RedWine => {
                CraftedData {
                    description: "Stamina +10, Food Regen -20",
                    key: "RedWine",
                }
            }
            CraftedResource::RoastingStick => {
                CraftedData {
                    description: "",
                    key: "RoastingStick",
                }
            }
            CraftedResource::RoastedProduce => {
                CraftedData {
                    description: "",
                    key: "RoastedProduce",
                }
            }
            CraftedResource::SumptuousPotPie => {
                CraftedData {
                    description: "General Crafting Exp. Points +1",
                    key: "SumptuousPotPie",
                }
            }
            CraftedResource::TrailMix => {
                CraftedData {
                    description: "Harvest Chance: Survivalist +3%",
                    key: "TrailMix",
                }
            }
            CraftedResource::Yeast => {
                CraftedData {
                    description: "",
                    key: "Yeast",
                }
            }
            CraftedResource::PowderedStone => {
                CraftedData {
                    description: "",
                    key: "PowderedStone",
                }
            }
            CraftedResource::DiamondCuttingBlade => {
                CraftedData {
                    description: "",
                    key: "DiamondCuttingBlade",
                }
            }
            CraftedResource::CutGemstoneBaguette => {
                CraftedData {
                    description: "",
                    key: "CutGemstoneBaguette",
                }
            }
            CraftedResource::CutGemstoneFocusing => {
                CraftedData {
                    description: "",
                    key: "CutGemstoneFocusing",
                }
            }
            CraftedResource::CutGemstoneMarquise => {
                CraftedData {
                    description: "",
                    key: "CutGemstoneMarquise",
                }
            }
            CraftedResource::CutGemstoneOval => {
                CraftedData {
                    description: "",
                    key: "CutGemstoneOval",
                }
            }
            CraftedResource::CutGemstoneRound => {
                CraftedData {
                    description: "",
                    key: "CutGemstoneRound",
                }
            }
            CraftedResource::CutGemstoneSquare => {
                CraftedData {
                    description: "",
                    key: "CutGemstoneSquare",
                }
            }
            CraftedResource::CutGemstoneTriangle => {
                CraftedData {
                    description: "",
                    key: "CutGemstoneTriangle",
                }
            }
            CraftedResource::ChaosEmberCuttingBlade => {
                CraftedData {
                    description: "",
                    key: "ChaosEmberCuttingBlade",
                }
            }
            CraftedResource::ExactingCutGemstoneBriolette => {
                CraftedData {
                    description: "",
                    key: "ExactingCutGemstoneBriolette",
                }
            }
            CraftedResource::ExactingCutGemstoneHeart => {
                CraftedData {
                    description: "",
                    key: "ExactingCutGemstoneHeart",
                }
            }
            CraftedResource::ExactingCutGemstoneTrillion => {
                CraftedData {
                    description: "",
                    key: "ExactingCutGemstoneTrillion",
                }
            }
            CraftedResource::CutGem => {
                CraftedData {
                    description: "",
                    key: "CutGem",
                }
            }
            CraftedResource::GrindingWheel => {
                CraftedData {
                    description: "",
                    key: "GrindingWheel",
                }
            }
            CraftedResource::GrindGemstone => {
                CraftedData {
                    description: "",
                    key: "GrindGemstone",
                }
            }
            CraftedResource::PolishingPaste => {
                CraftedData {
                    description: "",
                    key: "PolishingPaste",
                }
            }
            CraftedResource::PolishGemstone => {
                CraftedData {
                    description: "",
                    key: "PolishGemstone",
                }
            }
            CraftedResource::PolishSoulgem => {
                CraftedData {
                    description: "",
                    key: "PolishSoulgem",
                }
            }
            CraftedResource::NecklaceBail => {
                CraftedData {
                    description: "",
                    key: "NecklaceBail",
                }
            }
            CraftedResource::NecklaceChain => {
                CraftedData {
                    description: "",
                    key: "NecklaceChain",
                }
            }
            CraftedResource::Necklace => {
                CraftedData {
                    description: "",
                    key: "Necklace",
                }
            }
            CraftedResource::RingSetting => {
                CraftedData {
                    description: "",
                    key: "RingSetting",
                }
            }
            CraftedResource::RingBand => {
                CraftedData {
                    description: "",
                    key: "RingBand",
                }
            }
            CraftedResource::Ring => {
                CraftedData {
                    description: "",
                    key: "Ring",
                }
            }
            CraftedResource::PolishedExactingGemstone => {
                CraftedData {
                    description: "",
                    key: "PolishedExactingGemstone",
                }
            }
            CraftedResource::PolishedGemstone => {
                CraftedData {
                    description: "",
                    key: "PolishedGemstone",
                }
            }
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            CraftedResource::AppleJuice => { "Apple juice" }
            CraftedResource::ArtisanCheese => { "Artisan cheese" }
            CraftedResource::BakedIceCream => { "Baked ice cream" }
            CraftedResource::BoneBroth => { "Bone broth" }
            CraftedResource::Bread => { "Bread" }
            CraftedResource::BiscuitsAndGravy => { "Biscuits and gravy" }
            CraftedResource::BonTippers => { "Bon tippers" }
            CraftedResource::Butter => { "Butter" }
            CraftedResource::Cake => { "Cake" }
            CraftedResource::CampfireMeatyStew => { "Campfire meaty stew" }
            CraftedResource::CampfireMushroomStew => { "Campfire mushroom stew" }
            CraftedResource::CarrotJuice => { "Carrot juice" }
            CraftedResource::ChocolateBar => { "Chocolate bar" }
            CraftedResource::ChocolateMilk => { "Chocolate milk" }
            CraftedResource::ChocolateMilkSpicy => { "Chocolate milk spicy" }
            CraftedResource::Coffee => { "Coffee" }
            CraftedResource::CookingFoil => { "Cooking foil" }
            CraftedResource::CrushedHerb => { "Crushed herb" }
            CraftedResource::EmptyFlask => { "Empty flask" }
            CraftedResource::Gnocchi => { "Gnocchi" }
            CraftedResource::GrilledSandwichAuroch => { "Grilled sandwich auroch" }
            CraftedResource::GrilledSandwichBoar => { "Grilled sandwich boar" }
            CraftedResource::GrilledSandwichMushroom => { "Grilled sandwich mushroom" }
            CraftedResource::GrilledSandwichBear => { "Grilled sandwich bear" }
            CraftedResource::GrilledSandwichElk => { "Grilled sandwich elk" }
            CraftedResource::GrilledSandwichWolf => { "Grilled sandwich wolf" }
            CraftedResource::GrilledSandwichSpider => { "Grilled sandwich spider" }
            CraftedResource::GrilledSandwichBigCat => { "Grilled sandwich big cat" }
            CraftedResource::IceCream => { "Ice cream" }
            CraftedResource::KebabMushroom => { "Kebab mushroom" }
            CraftedResource::KebabAuroch => { "Kebab auroch" }
            CraftedResource::KebabBear => { "Kebab bear" }
            CraftedResource::KebabBigCat => { "Kebab big cat" }
            CraftedResource::KebabBoar => { "Kebab boar" }
            CraftedResource::KebabElk => { "Kebab elk" }
            CraftedResource::KebabSpider => { "Kebab spider" }
            CraftedResource::KebabWolf => { "Kebab wolf" }
            CraftedResource::LargeCookingPot => { "Large cooking pot" }
            CraftedResource::MarsalaStew => { "Marsala stew" }
            CraftedResource::MeatBurgundy => { "Meat burgundy" }
            CraftedResource::MushroomStew => { "Mushroom stew" }
            CraftedResource::Paella => { "Paella" }
            CraftedResource::PasteurizedMilk => { "Pasteurized milk" }
            CraftedResource::PestoGnocchi => { "Pesto gnocchi" }
            CraftedResource::PulverizedPotato => { "Pulverized potato" }
            CraftedResource::PotatoFlour => { "Potato flour" }
            CraftedResource::PotRoast => { "Pot roast" }
            CraftedResource::RedWine => { "Red wine" }
            CraftedResource::RoastingStick => { "Roasting stick" }
            CraftedResource::RoastedProduce => { "Roasted produce" }
            CraftedResource::SumptuousPotPie => { "Sumptuous pot pie" }
            CraftedResource::TrailMix => { "Trail mix" }
            CraftedResource::Yeast => { "Yeast" }
            CraftedResource::PowderedStone => { "Powdered stone" }
            CraftedResource::DiamondCuttingBlade => { "Diamond cutting blade" }
            CraftedResource::CutGemstoneBaguette => { "Cut gemstone baguette" }
            CraftedResource::CutGemstoneFocusing => { "Cut gemstone focusing" }
            CraftedResource::CutGemstoneMarquise => { "Cut gemstone marquise" }
            CraftedResource::CutGemstoneOval => { "Cut gemstone oval" }
            CraftedResource::CutGemstoneRound => { "Cut gemstone round" }
            CraftedResource::CutGemstoneSquare => { "Cut gemstone square" }
            CraftedResource::CutGemstoneTriangle => { "Cut gemstone triangle" }
            CraftedResource::ChaosEmberCuttingBlade => { "Chaos ember cutting blade" }
            CraftedResource::ExactingCutGemstoneBriolette => { "Exacting cut gemstone briolette" }
            CraftedResource::ExactingCutGemstoneHeart => { "Exacting cut gemstone heart" }
            CraftedResource::ExactingCutGemstoneTrillion => { "Exacting cut gemstone trillion" }
            CraftedResource::CutGem => { "Cut gem" }
            CraftedResource::GrindingWheel => { "Grinding wheel" }
            CraftedResource::GrindGemstone => { "Grind gemstone" }
            CraftedResource::PolishingPaste => { "Polishing paste" }
            CraftedResource::PolishGemstone => { "Polish gemstone" }
            CraftedResource::PolishSoulgem => { "Polish soulgem" }
            CraftedResource::NecklaceBail => { "Necklace bail" }
            CraftedResource::NecklaceChain => { "Necklace chain" }
            CraftedResource::Necklace => { "Necklace" }
            CraftedResource::RingSetting => { "Ring setting" }
            CraftedResource::RingBand => { "Ring band" }
            CraftedResource::Ring => { "Ring" }
            CraftedResource::PolishedExactingGemstone => { "Polished exacting gemstone" }
            CraftedResource::PolishedGemstone => { "Polished gemstone" }
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
                    MushroomButton,
                    MushroomChanterelle
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
                    Copper,
                    Iron,
                    Tin,
                    Silver,
                    Aurelium,
                ]
            }
            GroupResource::NonBasicWood => {
                vec![
                    Oak,
                    Birch,
                    Spruce,
                    Ash,
                    Yew,
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
                    Garlic,
                    Onion,
                ]
            }
            GroupResource::Ore => {
                vec![
                    Copper,
                    Iron,
                    Tin,
                    Silver,
                    Aurelium,
                    Slag
                ]
            }
            GroupResource::WildRiceOrGnocchi => {
                vec![
                    WildRice,
                    // Gnocchi,
                ]
            }
            GroupResource::NonBasicStone => {
                vec![
                    Granite,
                    Limestone,
                    Marble,
                    Slate,
                    Travertin,
                ]
            }
            GroupResource::Stone => {
                vec![
                    Granite,
                    Limestone,
                    Marble,
                    Slate,
                    Travertin,
                ]
            }
            GroupResource::RoughGem => {
                vec![
                    // TODO
                ]
            }
            GroupResource::BloodOrWater => {
                vec![
                    BaseResource::WaterFlask,
                    BaseResource::Blood
                ]
            }
        }
    }
    pub fn get_name(&self) -> &'static str {
        match self {
            GroupResource::AnimalMeat => { "Animal meat" }
            GroupResource::Herb => { "Herb" }
            GroupResource::Mushroom => { "Mushroom" }
            GroupResource::MeatOrMushroom => { "Meat or mushroom" }
            GroupResource::NonBasicOre => { "Non basic ore" }
            GroupResource::NonBasicWood => { "Non basic wood" }
            GroupResource::Produce => { "Produce" }
            GroupResource::Seasoning => { "Seasoning" }
            GroupResource::Ore => { "Ore" }
            GroupResource::WildRiceOrGnocchi => { "Wild rice or gnocchi" }
            GroupResource::NonBasicStone => { "Non basic stone" }
            GroupResource::Stone => { "Stone" }
            GroupResource::RoughGem => { "Rough gem" }
            GroupResource::BloodOrWater => { " Blood or Water" }
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
            CocoaBean => { "Cocoa bean" }
            CoffeeBean => { "Coffee bean" }
            GroundBlackPepper => { "Ground black pepper" }
            HotPepper => { "Hot pepper" }
            HungerShard => { "Hunger shard" }
            MeatAuroch => { "Meat auroch" }
            MeatBear => { "Meat bear" }
            MeatBigCat => { "Meat big cat" }
            MeatBoar => { "Meat boar" }
            MeatElk => { "Meat elk" }
            MeatSpider => { "Meat spider" }
            MeatWolf => { "Meat wolf" }
            MildPepper => { "Mild pepper" }
            PineNuts => { "Pine nuts" }
            Potato => { "Potato" }
            RawMilk => { "Raw milk" }
            SugarCane => { "Sugar cane" }
            SeasoningSalt => { "Seasoning salt" }
            SweetPepper => { "Sweet pepper" }
            WaterFlask => { "Water flask" }
            WildRice => { "Wild rice" }
            Onion => { "Onion" }
            MushroomButton => { "Button mushroom" }
            MushroomChanterelle => { "Chanterelle mushroom" }
            Copper => { "Copper" }
            Iron => { "Iron" }
            Tin => { "Tin" }
            Silver => { "Silver" }
            Aurelium => { "Aurelium" }
            Slag => { "Slag" }
            Oak => { "Oak" }
            Birch => { "Birch" }
            Spruce => { "Spruce" }
            Ash => { "Ash" }
            Yew => { "Yew" }
            Garlic => { "Garlic" }
            Dust => { "Ethereal dust" }
            Granite => { "Granite" }
            Limestone => { "Limestone" }
            Marble => { "Marble" }
            Slate => { "Slate" }
            Travertin => { "Travertin" }
            RoughSoulgem => { "Rough soulgem" }
            CuttingGrit => { "Cutting grit" }
        }
    }
}

impl Profession {
    pub fn get_name(&self) -> &'static str {
        match self {
            Profession::Cooking => { "Cooking" }
            Profession::Alchemy => { "Alchemy" }
            Profession::Jewelry => { "Jewelry" }
            Profession::Stonemasonry => { "Stonemasonry" }
        }
    }

    pub fn get_key(&self) -> &'static str {
        match self {
            Profession::Cooking => { "Cooking" }
            Profession::Alchemy => { "Alchemy" }
            Profession::Jewelry => { "Jewelry" }
            Profession::Stonemasonry => { "StoneMasonry" }
        }
    }
}