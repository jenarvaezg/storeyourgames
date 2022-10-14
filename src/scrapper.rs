use super::implementation;
use boardgamegeek;
use reqwest::{self};
use serde::Deserialize;
use tokio::runtime::Runtime;

#[derive(Deserialize, Debug, Clone)]
struct Item {
    length: String,
    width: String,
    depth: String,
    linkedname: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Versions {
    items: Vec<Item>,
}

fn inches_to_mm(inches: String) -> u32 {
    (inches.parse::<f32>().unwrap() * 25.4) as u32
}

async fn get_game(id: i64, is_expansion: bool) -> implementation::Game {
    let count = 20;
    let url = format!("https://api.geekdo.com/api/geekitem/linkeditems?linkdata_index=boardgameversion&objectid={}&objecttype=thing&pageid=1&showcount={}&subtype=boardgameversion", id, count);
    let resp = reqwest::get(url).await.unwrap().text().await.unwrap();

    let v: Versions = serde_json::from_str(&resp).expect("JSON decode error");

    let mut name = String::from("");

    for item in v.items {
        if name.is_empty() {
            name = item.linkedname.clone();
        }
        if item.length != "0" {
            return implementation::Game {
                name: item.linkedname.clone(),
                is_expansion: is_expansion,
                dimensions: implementation::Dimensions {
                    width: inches_to_mm(item.width),
                    height: inches_to_mm(item.length),
                    depth: inches_to_mm(item.depth),
                },
            };
        }
    }

    return implementation::Game {
        name: name,
        is_expansion: is_expansion,
        dimensions: implementation::Dimensions {
            width: 0,
            height: 0,
            depth: 0,
        },
    };
}

pub fn get_collection(username: String) -> Vec<implementation::Game> {
    let rt = Runtime::new().unwrap();

    let client = boardgamegeek::Client::new();
    let games_future = client.get_collection(&username, boardgamegeek::CollectionType::BoardGames);
    let expansions_future = client.get_collection(
        &username,
        boardgamegeek::CollectionType::BoardGameExpansions,
    );

    let games = rt.block_on(games_future).unwrap().items;
    let expansions = rt.block_on(expansions_future).unwrap().items;

    let mut games_vec = Vec::new();

    let mut total = 0;

    for game in games {
        if game.is_owned() && total < 20 {
            total += 1;
            games_vec.push(rt.block_on(get_game(game.id, false)))
        }
    }

    for game in expansions {
        if game.is_owned() && total < 40 {
            total += 1;
            games_vec.push(rt.block_on(get_game(game.id, true)))
        }
    }

    println!("{:?}", games_vec);

    return games_vec;
}
