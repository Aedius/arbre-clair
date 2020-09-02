use std::cmp::max;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::craft::{BaseResource, GroupResource, CraftedResource, Recipe, Item};
use crate::craft::cooking::get_recipe as getCookingRecipe;
use crate::craft::alchemy::get_recipe as getAlchemyRecipe;
use crate::craft::jewelry::get_recipe as getJewelCraftingRecipe;


#[derive(Debug)]
pub struct RecipeTree {
    pub base: HashMap<BaseResource, f32>,
    pub group: HashMap<GroupResource, f32>,
    pub recipe_list: HashMap<i32, HashMap<Recipe, f32>>,
    recipe_map: HashMap<CraftedResource, Recipe>,
    already_crafted: HashMap<CraftedResource, f32>
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

//let mut base_list:Vec<&'static str> = group.clone().get_base().into_iter().map(|x| x.get_name()).collect();

fn get_displayable_recipe(recipe: &Recipe) -> DisplayableRecipe{
    DisplayableRecipe{
        name: recipe.name,
        input: recipe.input.clone().into_iter().map(|x| (x.0.get_name(), x.1 )).collect(),
        output: (recipe.output.0.get_name(), recipe.output.1),
        profession: recipe.profession.get_name(),
        menu: recipe.menu,
    }
}

#[derive(Debug, Deserialize)]
pub struct CraftedQuantity{
    name: CraftedResource,
    qty: i32,
}

pub fn handle(recipe_name: &str, quantity: i32, got : Vec<CraftedQuantity>) -> Option<RecipeResponse> {

    let mut already_crafted = HashMap::new();

    for gotcha in got {
        already_crafted.insert(gotcha.name, gotcha.qty as f32);
    }

    //TODO add other recipe to the vec
    let cooking = getCookingRecipe();
    let jewel_crafting = getJewelCraftingRecipe();
    let alchemy = getAlchemyRecipe();
    let recipes = [&cooking[..], &alchemy[..], &jewel_crafting[..]].concat();

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
        already_crafted,
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

                        let current_already_crafted = self.already_crafted.entry(crafted.clone()).or_insert(0.0);

                        let mut to_craft = nb * *qty as f32;

                        if *current_already_crafted > 0.0 {

                            if *current_already_crafted as f32 > to_craft{
                                *current_already_crafted = *current_already_crafted - to_craft ;
                                to_craft = 0.0;
                            }else{
                                *current_already_crafted = 0.0;
                                to_craft = to_craft - *current_already_crafted;
                            }

                        }

                        if to_craft > 0.0 {
                            let current_lvl = self.add_resource(to_craft, crafted.clone());

                            max_lvl = max(max_lvl, current_lvl);
                        }
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