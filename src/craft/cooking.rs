use crate::craft::{Item, Profession, Recipe, RecipeSummary};
use crate::craft::BaseResource::*;
use crate::craft::CraftedResource::*;
use crate::craft::GroupResource::*;

pub fn get_all_recipe() -> Vec<RecipeSummary>{

    let mut ret = Vec::new();
    let all_recipe = get_recipe();

    for recipe in all_recipe{
        if let Some(data) = recipe.crafted_data(){

            let summary = RecipeSummary{
                key: data.key,
                name: recipe.name,
                profession: recipe.profession,
                stat: data.stat
            };

            ret.push(summary);
        }
    }

    ret.sort_by(|a, b| a.name.cmp(&b.name));

    ret
}

pub fn get_recipe() -> Vec<Recipe> {
    let mut ret = Vec::new();

    let component = "Components";

    ret.push(Recipe {
        name: "Cooking Foil",
        input: vec![
            (Item::Group(Ore), 2),
        ],
        output: (CookingFoil, 10),
        profession: Profession::Cooking,
        menu: component,
    });

    ret.push(Recipe {
        name: "Grind Food Items",
        input: vec![
            (Item::Base(Potato), 2),
        ],
        output: (PulverizedPotato, 1),
        profession: Profession::Cooking,
        menu: component,
    });

    ret.push(Recipe {
        name: "Grind Food Items",
        input: vec![
            (Item::Group(Herb), 2),
        ],
        output: (CrushedHerb, 1),
        profession: Profession::Cooking,
        menu: component,
    });

    ret.push(Recipe {
        name: "Grind Food Items : Bulk",
        input: vec![
            (Item::Base(Potato), 20),
        ],
        output: (PulverizedPotato, 10),
        profession: Profession::Cooking,
        menu: component,
    });

    ret.push(Recipe {
        name: "Grind Food Items: Bulk",
        input: vec![
            (Item::Group(Herb), 20),
        ],
        output: (CrushedHerb, 10),
        profession: Profession::Cooking,
        menu: component,
    });

    ret.push(Recipe {
        name: "Large Cooking Pot",
        input: vec![
            (Item::Group(NonBasicOre), 6),
        ],
        output: (LargeCookingPot, 1),
        profession: Profession::Cooking,
        menu: component,
    });

    ret.push(Recipe {
        name: "Roasting Stick",
        input: vec![
            (Item::Group(NonBasicWood), 4),
        ],
        output: (RoastingStick, 10),
        profession: Profession::Cooking,
        menu: component,
    });

    let cooking_basic = "Cooking Basics";

    ret.push(Recipe {
        name: "Chocolate Bar",
        input: vec![
            (Item::Base(RawMilk), 10),
            (Item::Crafted(EmptyFlask), 5),
            (Item::Base(SugarCane), 2),
            (Item::Base(CocoaBean), 5),
        ],
        output: (ChocolateBar, 10),
        profession: Profession::Cooking,
        menu: cooking_basic,
    });

    ret.push(Recipe {
        name: "Juice",
        input: vec![
            (Item::Base(WaterFlask), 10),
            (Item::Base(Carrot), 10),
        ],
        output: (CarrotJuice, 10),
        profession: Profession::Cooking,
        menu: cooking_basic,
    });

    ret.push(Recipe {
        name: "Juice",
        input: vec![
            (Item::Base(WaterFlask), 10),
            (Item::Base(Apple), 10),
        ],
        output: (AppleJuice, 10),
        profession: Profession::Cooking,
        menu: cooking_basic,
    });

    ret.push(Recipe {
        name: "Potato Flour",
        input: vec![
            (Item::Crafted(PulverizedPotato), 8),
            (Item::Crafted(CookingFoil), 1),
        ],
        output: (PotatoFlour, 10),
        profession: Profession::Cooking,
        menu: cooking_basic,
    });

    ret.push(Recipe {
        name: "Seasoned Mushrooms",
        input: vec![
            (Item::Group(Mushroom), 10),
            (Item::Crafted(CookingFoil), 4),
            (Item::Group(Seasoning), 1),
        ],
        output: (SeasonedMushroom, 10),
        profession: Profession::Cooking,
        menu: cooking_basic,
    });

    ret.push(Recipe {
        name: "Trail Mix",
        input: vec![
            (Item::Crafted(CookingFoil), 1),
            (Item::Group(Seasoning), 1),
            (Item::Base(Apple), 5),
            (Item::Crafted(ChocolateBar), 1),
            (Item::Base(PineNuts), 5),
        ],
        output: (TrailMix, 10),
        profession: Profession::Cooking,
        menu: cooking_basic,
    });

    let campfire = "Campfire Cooking";

    ret.push(Recipe {
        name: "Basic Roasted Meats",
        input: vec![
            (Item::Group(AnimalMeat), 2),
            (Item::Group(Seasoning), 1),
        ],
        output: (BasicRoastedMeat, 1),
        profession: Profession::Cooking,
        menu: campfire,
    });

    ret.push(Recipe {
        name: "Bread",
        input: vec![
            (Item::Crafted(Yeast), 4),
            (Item::Group(Seasoning), 1),
            (Item::Crafted(PotatoFlour), 6),
            (Item::Crafted(CookingFoil), 1),
            (Item::Base(WaterFlask), 5),
        ],
        output: (Bread, 12),
        profession: Profession::Cooking,
        menu: campfire,
    });

    ret.push(Recipe {
        name: "Campfire Stew",
        input: vec![
            (Item::Crafted(CookingFoil), 4),
            (Item::Group(MeatOrMushroom), 4),
            (Item::Group(Seasoning), 1),
        ],
        output: (CampfireMeatyStew, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });

    ret.push(Recipe {
        name: "Campfire Stew",
        input: vec![
            (Item::Crafted(CookingFoil), 4),
            (Item::Group(Mushroom), 4),
            (Item::Group(Seasoning), 1),
        ],
        output: (CampfireMushroomStew, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });

    ret.push(Recipe {
        name: "Coffee",
        input: vec![
            (Item::Base(CoffeeBean), 10),
            (Item::Base(WaterFlask), 5),
            (Item::Crafted(EmptyFlask), 5),
            (Item::Crafted(PasteurizedMilk), 1),
            (Item::Base(SugarCane), 5),
        ],
        output: (Coffee, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });

    ret.push(Recipe {
        name: "Kebab Skewers",
        input: vec![
            (Item::Group(Mushroom), 4),
            (Item::Crafted(RoastingStick), 5),
            (Item::Group(Seasoning), 5),
            (Item::Group(Produce), 5),
        ],
        output: (KebabMushroom, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });
    ret.push(Recipe {
        name: "Kebab Skewers",
        input: vec![
            (Item::Group(MeatOrMushroom), 4),
            (Item::Crafted(RoastingStick), 5),
            (Item::Group(Seasoning), 5),
            (Item::Group(Mushroom), 5),
        ],
        output: (KebabMushroom, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });
    ret.push(Recipe {
        name: "Kebab Skewers",
        input: vec![
            (Item::Base(MeatAuroch), 4),
            (Item::Crafted(RoastingStick), 5),
            (Item::Group(Seasoning), 5),
            (Item::Group(Produce), 5),
        ],
        output: (KebabAuroch, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });
    ret.push(Recipe {
        name: "Kebab Skewers",
        input: vec![
            (Item::Base(MeatBear), 4),
            (Item::Crafted(RoastingStick), 5),
            (Item::Group(Seasoning), 5),
            (Item::Group(Produce), 5),
        ],
        output: (KebabBear, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });
    ret.push(Recipe {
        name: "Kebab Skewers",
        input: vec![
            (Item::Base(MeatBigCat), 4),
            (Item::Crafted(RoastingStick), 5),
            (Item::Group(Seasoning), 5),
            (Item::Group(Produce), 5),
        ],
        output: (KebabBigCat, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });
    ret.push(Recipe {
        name: "Kebab Skewers",
        input: vec![
            (Item::Base(MeatBoar), 4),
            (Item::Crafted(RoastingStick), 5),
            (Item::Group(Seasoning), 5),
            (Item::Group(Produce), 5),
        ],
        output: (KebabBoar, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });
    ret.push(Recipe {
        name: "Kebab Skewers",
        input: vec![
            (Item::Base(MeatElk), 4),
            (Item::Crafted(RoastingStick), 5),
            (Item::Group(Seasoning), 5),
            (Item::Group(Produce), 5),
        ],
        output: (KebabElk, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });
    ret.push(Recipe {
        name: "Kebab Skewers",
        input: vec![
            (Item::Base(MeatSpider), 4),
            (Item::Crafted(RoastingStick), 5),
            (Item::Group(Seasoning), 5),
            (Item::Group(Produce), 5),
        ],
        output: (KebabSpider, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });
    ret.push(Recipe {
        name: "Kebab Skewers",
        input: vec![
            (Item::Base(MeatWolf), 4),
            (Item::Crafted(RoastingStick), 5),
            (Item::Group(Seasoning), 5),
            (Item::Group(Produce), 5),
        ],
        output: (KebabWolf, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });

    ret.push(Recipe {
        name: "Pasteurized Milk",
        input: vec![
            (Item::Base(RawMilk), 10),
            (Item::Crafted(EmptyFlask), 5),
        ],
        output: (PasteurizedMilk, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });

    ret.push(Recipe {
        name: "RoastedProduce",
        input: vec![
            (Item::Group(Produce), 10),
            (Item::Crafted(RoastingStick), 4),
            (Item::Group(Seasoning), 1),
        ],
        output: (RoastedProduce, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });

    ret.push(Recipe {
        name: "Yeast",
        input: vec![
            (Item::Group(Mushroom), 10),
            (Item::Crafted(CookingFoil), 1),
            (Item::Base(WaterFlask), 5),
        ],
        output: (Yeast, 10),
        profession: Profession::Cooking,
        menu: campfire,
    });

    let sous_chef = "Sous Chef";

    ret.push(Recipe {
        name: "Artisan Cheese",
        input: vec![
            (Item::Crafted(PasteurizedMilk), 10),
            (Item::Crafted(Yeast), 5),
            (Item::Crafted(CookingFoil), 1),
            (Item::Base(Beeswax), 1),
        ],
        output: (ArtisanCheese, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Bloodworm Stew",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Group(Seasoning), 1),
            (Item::Base(WaterFlask), 5),
            (Item::Base(Bloodworm), 8),
        ],
        output: (BloodwormStew, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Bone Broth",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Group(AnimalMeat), 4),
            (Item::Base(Blood), 2),
            (Item::Base(Bone), 2),
            (Item::Base(WaterFlask), 8),
            (Item::Group(Seasoning), 1),
        ],
        output: (BoneBroth, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Chocolate Milk",
        input: vec![
            (Item::Crafted(ChocolateBar), 5),
            (Item::Crafted(PasteurizedMilk), 5),
            (Item::Base(SugarCane), 2),
        ],
        output: (ChocolateMilk, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });

    ret.push(Recipe {
        name: "Chocolate Milk",
        input: vec![
            (Item::Crafted(ChocolateBar), 5),
            (Item::Crafted(PasteurizedMilk), 5),
            (Item::Base(SugarCane), 2),
            (Item::Base(HotPepper), 2),
        ],
        output: (ChocolateMilkSpicy, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });

    ret.push(Recipe {
        name: "Cookie",
        input: vec![
            (Item::Crafted(Butter), 1),
            (Item::Base(SugarCane), 5),
            (Item::Crafted(PasteurizedMilk), 1),
            (Item::Base(WaterFlask), 2),
            (Item::Crafted(PotatoFlour), 4),
            (Item::Crafted(CookingFoil), 1),
        ],
        output: (Cookie, 12),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Cookie",
        input: vec![
            (Item::Crafted(Butter), 1),
            (Item::Base(SugarCane), 5),
            (Item::Crafted(PasteurizedMilk), 1),
            (Item::Base(WaterFlask), 2),
            (Item::Crafted(ChocolateBar), 1),
            (Item::Crafted(PotatoFlour), 4),
            (Item::Crafted(CookingFoil), 1),
        ],
        output: (CookieChocolate, 12),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Fresh Butter",
        input: vec![
            (Item::Crafted(PasteurizedMilk), 10),
            (Item::Crafted(CookingFoil), 1),
            (Item::Base(WaterFlask), 2),
        ],
        output: (Butter, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Gnocchi",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Crafted(PotatoFlour), 10),
            (Item::Base(WaterFlask), 5),
        ],
        output: (Gnocchi, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Grilled Cheese Sandwich",
        input: vec![
            (Item::Crafted(CookingFoil), 2),
            (Item::Group(Seasoning), 1),
            (Item::Crafted(Bread), 8),
            (Item::Crafted(ArtisanCheese), 8),
            (Item::Crafted(Butter), 1),
        ],
        output: (GrilledCheeseSandwich, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });

    ret.push(Recipe {
        name: "Grilled Sandwich",
        input: vec![
            (Item::Crafted(CookingFoil), 2),
            (Item::Base(MeatBoar), 8),
            (Item::Group(Herb), 1),
            (Item::Crafted(Bread), 8),
            (Item::Crafted(Butter), 1),
            (Item::Crafted(ArtisanCheese), 4),
        ],
        output: (GrilledSandwichBoar, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Grilled Sandwich",
        input: vec![
            (Item::Crafted(CookingFoil), 2),
            (Item::Group(Mushroom), 8),
            (Item::Group(Herb), 1),
            (Item::Crafted(Bread), 8),
            (Item::Crafted(Butter), 1),
            (Item::Crafted(ArtisanCheese), 4),
        ],
        output: (GrilledSandwichMushroom, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Grilled Sandwich",
        input: vec![
            (Item::Crafted(CookingFoil), 2),
            (Item::Base(MeatBear), 8),
            (Item::Group(Herb), 1),
            (Item::Crafted(Bread), 8),
            (Item::Crafted(Butter), 1),
            (Item::Crafted(ArtisanCheese), 4),
        ],
        output: (GrilledSandwichBear, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Grilled Sandwich",
        input: vec![
            (Item::Crafted(CookingFoil), 2),
            (Item::Base(MeatElk), 8),
            (Item::Group(Herb), 1),
            (Item::Crafted(Bread), 8),
            (Item::Crafted(Butter), 1),
            (Item::Crafted(ArtisanCheese), 4),
        ],
        output: (GrilledSandwichElk, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Grilled Sandwich",
        input: vec![
            (Item::Crafted(CookingFoil), 2),
            (Item::Base(MeatWolf), 8),
            (Item::Group(Herb), 1),
            (Item::Crafted(Bread), 8),
            (Item::Crafted(Butter), 1),
            (Item::Crafted(ArtisanCheese), 4),
        ],
        output: (GrilledSandwichWolf, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Grilled Sandwich",
        input: vec![
            (Item::Crafted(CookingFoil), 2),
            (Item::Base(MeatSpider), 8),
            (Item::Group(Herb), 1),
            (Item::Crafted(Bread), 8),
            (Item::Crafted(Butter), 1),
            (Item::Crafted(ArtisanCheese), 4),
        ],
        output: (GrilledSandwichSpider, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Grilled Sandwich",
        input: vec![
            (Item::Crafted(CookingFoil), 2),
            (Item::Base(MeatBigCat), 8),
            (Item::Group(Herb), 1),
            (Item::Crafted(Bread), 8),
            (Item::Crafted(Butter), 1),
            (Item::Crafted(ArtisanCheese), 4),
        ],
        output: (GrilledSandwichBigCat, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });

    ret.push(Recipe {
        name: "Mead",
        input: vec![
            (Item::Base(WaterFlask), 1),
            (Item::Crafted(EmptyFlask), 5),
            (Item::Crafted(PulverizedPotato), 4),
            (Item::Crafted(Yeast), 4),
        ],
        output: (Mead, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });
    ret.push(Recipe {
        name: "Mushroom Stew",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Group(WildRiceOrGnocchi), 2),
            (Item::Base(WaterFlask), 5),
            (Item::Group(Mushroom), 10),
            (Item::Group(Seasoning), 1),
        ],
        output: (MushroomStew, 10),
        profession: Profession::Cooking,
        menu: sous_chef,
    });

    let head_chef = "Head Chef";

    ret.push(Recipe {
        name: "Baked Ice Cream",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Crafted(IceCream), 10),
            (Item::Base(HotPepper), 5),
        ],
        output: (BakedIceCream, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });

    ret.push(Recipe {
        name: "BiscuitsAndGravy",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Base(WaterFlask), 5),
            (Item::Group(MeatOrMushroom), 4),
            (Item::Crafted(PotatoFlour), 10),
            (Item::Crafted(BoneBroth), 2),
            (Item::Group(Seasoning), 1),
        ],
        output: (BiscuitsAndGravy, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });

    ret.push(Recipe {
        name: "Bon Tippers",
        input: vec![
            (Item::Base(WaterFlask), 1),
            (Item::Crafted(EmptyFlask), 5),
            (Item::Crafted(RedWine), 10),
        ],
        output: (BonTippers, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });
    ret.push(Recipe {
        name: "Cake",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Crafted(Butter), 2),
            (Item::Crafted(PasteurizedMilk), 5),
            (Item::Crafted(PotatoFlour), 10),
            (Item::Base(SugarCane), 5),
            (Item::Base(WaterFlask), 5),
        ],
        output: (Cake, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });
    ret.push(Recipe {
        name: "Ice Cream",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Base(HungerShard), 2),
            (Item::Crafted(PasteurizedMilk), 5),
            (Item::Base(SugarCane), 5),
            (Item::Base(WaterFlask), 5),
        ],
        output: (IceCream, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });
    ret.push(Recipe {
        name: "Marsala Stew",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Group(WildRiceOrGnocchi), 2),
            (Item::Group(MeatOrMushroom), 5),
            (Item::Crafted(RedWine), 4),
            (Item::Group(MeatOrMushroom), 5),
            (Item::Crafted(Butter), 1),
            (Item::Crafted(CrushedHerb), 5),
            (Item::Group(Seasoning), 1),
        ],
        output: (MarsalaStew, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });
    ret.push(Recipe {
        name: "Meat Burgundy",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Crafted(CrushedHerb), 5),
            (Item::Crafted(RedWine), 4),
            (Item::Group(AnimalMeat), 4),
            (Item::Group(WildRiceOrGnocchi), 2),
            (Item::Group(Mushroom), 5),
            (Item::Group(Produce), 2),
            (Item::Group(Seasoning), 1),
        ],
        output: (MeatBurgundy, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });
    ret.push(Recipe {
        name: "Paella",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Crafted(CrushedHerb), 5),
            (Item::Group(MeatOrMushroom), 4),
            (Item::Crafted(CrushedHerb), 5),
            (Item::Crafted(Butter), 1),
            (Item::Group(Seasoning), 1),
        ],
        output: (Paella, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });
    ret.push(Recipe {
        name: "Pesto Gnocchi",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Base(PineNuts), 1),
            (Item::Group(Mushroom), 1),
            (Item::Crafted(Gnocchi), 10),
            (Item::Crafted(CrushedHerb), 5),
            (Item::Crafted(CrushedHerb), 5),
            (Item::Crafted(Butter), 1),
            (Item::Group(Seasoning), 1),
        ],
        output: (PestoGnocchi, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });
    ret.push(Recipe {
        name: "Pot Roast",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Group(Seasoning), 1),
            (Item::Crafted(BoneBroth), 2),
            (Item::Group(AnimalMeat), 4),
            (Item::Group(WildRiceOrGnocchi), 2),
            (Item::Group(AnimalMeat), 4),
            (Item::Crafted(CrushedHerb), 2),
        ],
        output: (PotRoast, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });
    ret.push(Recipe {
        name: "Red Wine",
        input: vec![
            (Item::Base(WaterFlask), 1),
            (Item::Base(Apple), 5),
            (Item::Crafted(EmptyFlask), 5),
            (Item::Base(SugarCane), 2),
            (Item::Crafted(Yeast), 4),
        ],
        output: (RedWine, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });
    ret.push(Recipe {
        name: "Roasted Pig",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Base(MeatBoar), 15),
            (Item::Group(Seasoning), 1),
            (Item::Base(Apple), 5),
        ],
        output: (RoastedPig, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });
    ret.push(Recipe {
        name: "Sumptuous Pot Pie",
        input: vec![
            (Item::Crafted(LargeCookingPot), 1),
            (Item::Crafted(CrushedHerb), 5),
            (Item::Group(MeatOrMushroom), 4),
            (Item::Base(WaterFlask), 5),
            (Item::Crafted(Butter), 1),
            (Item::Crafted(Bread), 4),
            (Item::Group(Produce), 2),
            (Item::Group(Seasoning), 1),
        ],
        output: (SumptuousPotPie, 10),
        profession: Profession::Cooking,
        menu: head_chef,
    });

    ret
}