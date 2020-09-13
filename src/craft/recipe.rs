use std::cmp::max;
use std::collections::HashMap;
use serde::{Serialize};

use crate::craft::{BaseResource, GroupResource, CraftedResource, Recipe, Item, RecipeSummary};
use crate::craft::cooking::get_recipe as getCookingRecipe;
use crate::craft::alchemy::get_recipe as getAlchemyRecipe;
use crate::craft::jewelry::get_recipe as getJewelCraftingRecipe;
use crate::craft::stonemasonry::get_recipe as getStoneMasonry;
use crate::craft::Profession::{Cooking, Jewelry, Stonemasonry, Alchemy};


#[derive(Debug)]
pub struct RecipeTree {
    pub base: HashMap<BaseResource, f32>,
    pub group: HashMap<GroupResource, f32>,
    pub recipe_list: HashMap<i32, HashMap<Recipe, f32>>,
    recipe_map: HashMap<CraftedResource, Recipe>,
}

#[derive(Debug, Serialize)]
pub struct DisplayableRecipe {
    pub name: &'static str,
    pub input: Vec<(&'static str, i32)>,
    pub output: (&'static str, i32),
    pub profession: &'static str,
    pub menu: &'static str,
}

#[derive(Debug, Serialize)]
pub struct BaseResponse {
    base: &'static str,
    quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct GroupResponse {
    group: &'static str,
    base_list: Vec<&'static str>,
    quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct RecipePartialResponse {
    recipe: DisplayableRecipe,
    quantity: i32,

}

#[derive(Debug, Serialize)]
pub struct RecipGroupResponse {
    level: i32,
    recipe_list: Vec<RecipePartialResponse>,
}

#[derive(Debug, Serialize)]
pub struct RecipeResponse {
    pub base: Vec<BaseResponse>,
    pub group: Vec<GroupResponse>,
    pub recipe: Vec<RecipGroupResponse>,
    pub summary: Recipe,
    pub lvl: i32,
}


pub fn recipe_to_recipe_summary(all_recipe :Vec<Recipe>) -> Vec<RecipeSummary>{

    let mut ret = Vec::new();

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


fn get_displayable_recipe(recipe: &Recipe) -> DisplayableRecipe{
    DisplayableRecipe{
        name: recipe.name,
        input: recipe.input.clone().into_iter().map(|x| (x.0.get_name(), x.1 )).collect(),
        output: (recipe.output.0.get_name(), recipe.output.1),
        profession: recipe.profession.get_name(),
        menu: recipe.menu,
    }
}

pub fn handle_craft(craft_name : &str) -> Option<Vec<RecipeSummary>> {

    let cooking = Alchemy;
    if craft_name == cooking.get_key() {
        return Some(recipe_to_recipe_summary(getAlchemyRecipe()));
    }

    let cooking = Cooking;
    if craft_name == cooking.get_key() {
        return Some(recipe_to_recipe_summary(getCookingRecipe()));
    }

    let jewelry =  Jewelry;
    if craft_name == jewelry.get_key() {
        return Some(recipe_to_recipe_summary(getJewelCraftingRecipe()))
    }

    let stonemasonry =  Stonemasonry;
    if craft_name == stonemasonry.get_key() {
        return Some(recipe_to_recipe_summary(getStoneMasonry()))
    }
    return None;
}

pub fn handle_recipe(recipe_name: &str, quantity: i32) -> Option<RecipeResponse> {

    //TODO add other recipe to the vec
    let cooking = getCookingRecipe();
    let jewel_crafting = getJewelCraftingRecipe();
    let alchemy = getAlchemyRecipe();
    let stone_masonry = getStoneMasonry();
    let recipes = [&cooking[..], &alchemy[..], &jewel_crafting[..], &stone_masonry[..]].concat();



    let mut recipe_map = HashMap::new();

    let mut current_recipe = None;

    for recipe in recipes {
        let out = recipe.output.0.clone();
        let info = out.get_information();
        if recipe_name == info.key {
            current_recipe = Some(recipe.clone())
        }

        recipe_map.insert(out, recipe.clone());
    }


    if current_recipe.is_none() {
        return None;
    }

    let mut tree = RecipeTree {
        base: HashMap::new(),
        group: HashMap::new(),
        recipe_list: HashMap::new(),
        recipe_map,
    };

    let current_recipe = current_recipe.unwrap();

    // TODO : change qte.
    let max_level = tree.add_resource(quantity as f32, current_recipe.clone().output.0);

    let mut recipe_response = RecipeResponse {
        base: vec![],
        group: vec![],
        recipe: vec![],
        summary: current_recipe,
        lvl: max_level,
    };

    for ( base, nb) in tree.base {
        recipe_response.base.push(BaseResponse{
            base : base.get_name(),
            quantity: nb.round() as i32
        })
    }

    recipe_response.base.sort_by(|a, b| a.base.cmp(&b.base));

    for ( group, nb) in tree.group {


        let mut base_list:Vec<&'static str> = group.clone().get_base().into_iter().map(|x| x.get_name()).collect();

        base_list.sort_by(|a,b | a.cmp(&b));

        recipe_response.group.push(GroupResponse{
            group: group.get_name(),
            quantity: nb.round() as i32,
            base_list
        })
    }


    recipe_response.group.sort_by(|a, b| a.group.cmp(&b.group));


    for ( lvl, recipe_group_list) in tree.recipe_list {

        let mut list = vec![];

        for(recipe, nb) in recipe_group_list {
            list.push(RecipePartialResponse{
                recipe: get_displayable_recipe(&recipe),
                quantity: nb.round() as i32
            })
        }

        recipe_response.recipe.push(RecipGroupResponse{
            recipe_list: list,
            level: lvl
        });

        recipe_response.recipe.sort_by(|a, b| a.level.cmp(&b.level))
    }

    Some(recipe_response)
}


impl RecipeTree {
    fn add_resource(&mut self, qte: f32, crafted: CraftedResource) -> i32 {
        let mut max_lvl = 0;

        let recipe = self.recipe_map.get(&crafted);
        if recipe.is_some() {
            let recipe = recipe.unwrap().clone();

            let nb = (qte / recipe.output.1 as f32).ceil();


            for (item, qty) in &recipe.input {
                match item {
                    Item::Base(base) => {
                        let current_base = self.base.entry(base.clone()).or_insert(0.0);
                        *current_base += nb * *qty as f32;
                    }
                    Item::Group(group) => {
                        let current_group = self.group.entry(group.clone()).or_insert(0.0);
                        *current_group += nb * *qty as f32;
                    }
                    Item::Crafted(crafted) => {
                        let current_lvl = self.add_resource(nb * *qty as f32, crafted.clone());

                        max_lvl = max(max_lvl, current_lvl);
                    }
                }
            }

            max_lvl += 1;

            let recipe_list = self.recipe_list.entry(max_lvl).or_insert(HashMap::new());
            let current_recipe = recipe_list.entry(recipe).or_insert(0.0);
            *current_recipe += nb as f32;
        }
        max_lvl
    }

}