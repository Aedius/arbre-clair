use std::cmp::max;
use std::collections::HashMap;
use serde::{Serialize};

use crate::craft::{BaseResource, GroupResource, CraftedResource, Recipe, Item, RecipeSummary};
use crate::craft::cooking::get_recipe as getCookingRecipe;


#[derive(Debug)]
pub struct RecipeTree {
    pub base: HashMap<BaseResource, f32>,
    pub group: HashMap<GroupResource, f32>,
    pub recipe_list: HashMap<i32, HashMap<Recipe, f32>>,
    recipe_map: HashMap<CraftedResource, Recipe>,
}

#[derive(Debug, Serialize)]
pub struct BaseResponse {
    base: BaseResource,
    quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct GroupResponse {
    group: GroupResource,
    quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct RecipePartialResponse {
    recipe: Recipe,
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

pub fn handle(recipe_name: &str) -> Option<RecipeResponse> {

    //TODO add other recipe to the vec
    let recipes = getCookingRecipe();

    let mut recipe_map = HashMap::new();

    let mut current_recipe = None;

    for recipe in recipes {
        let out = recipe.output.0.clone();
        if out.get_information().is_some() {
            let info = out.get_information().unwrap();
            if recipe_name == info.key {
                current_recipe = Some(recipe.clone())
            }
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
    let max_level = tree.add_resource(120.0, current_recipe.clone().output.0);

    let mut recipe_response = RecipeResponse {
        base: vec![],
        group: vec![],
        recipe: vec![],
        summary: current_recipe,
        lvl: max_level,
    };

    for ( base, nb) in tree.base {
        recipe_response.base.push(BaseResponse{
            base,
            quantity: nb.round() as i32
        })
    }

    for ( group, nb) in tree.group {
        recipe_response.group.push(GroupResponse{
            group,
            quantity: nb.round() as i32
        })
    }

    for ( lvl, recipe_group_list) in tree.recipe_list {

        let mut list = vec![];

        for(recipe, nb) in recipe_group_list {
            list.push(RecipePartialResponse{
                recipe,
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