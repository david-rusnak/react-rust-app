#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Ingredient {
    unit: String,
    amount: i32,
    ingredient_name: String,
    label: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    name: String,
    glass: String,
    category: String,
    ingredients: Vec<Ingredient>,
    garnish: String,
    preparation: String
}


#[tauri::command]
fn hello(handle: tauri::AppHandle, user_ingredients: Vec<String>) -> Vec<String> {
    let resource_path = handle.path_resolver()
        .resolve_resource("data/drinks.json")
        .expect("failed to resolve resource");
    let file = std::fs::File::open(&resource_path).unwrap();
    let data: serde_json::Value = serde_json::from_reader(file).unwrap();

    println!("{}", user_ingredients[0]);
  
    let mut i: usize = 0;
    let mut vec: Vec<Vec<String>> = vec![[].to_vec(); 77];
    let mut vec_of_recipes: Vec<String> = vec![];
    let vec_of_index: Vec<usize> = vec![];

    while i < 77 {
        for ingred in data[i]["ingredients"].as_array().unwrap() {
            let s = ingred["ingredient"].as_str();
            if s.is_some() {
                vec[i].push(ingred["ingredient"].to_string().to_lowercase());
                //println!("{}", ingred["ingredient"]);
            } else {
                vec[i].push(ingred["special"].to_string().to_lowercase());
                //println!("{}", ingred["special"]);
            }
            
        }
        if user_ingredients.iter().all(|item| vec[i].contains(item)) {
            println!("{}", data[i]["name"].to_string());
        }
        i+=1;
    }
    
    // At this point, we have an array of each recipes ingredients in lowercase
    // Now check out list of ingredients, and if all ingredients in one of the recipes appear in the user list,
    // add that index to the index array

    
    

    for i in vec_of_index {
        vec_of_recipes.push(data[i].to_string());
    }
  
    vec_of_recipes
}
  
  fn main() {
    tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![hello])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }
  
