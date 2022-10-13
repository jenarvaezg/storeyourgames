mod implementation;
use image::{Rgb, RgbImage};
use implementation::{Bin, Dimensions, Game, Options};
use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, RectToInsert,
    RectanglePackOk, TargetBin,
};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use std::fs;

const TM_DICE: Game = Game {
    name: "Terraforming Mars: The Dice Game",
    dimensions: Dimensions {
        height: 310,
        width: 234,
        depth: 53,
    },
};

const KDM: Game = Game {
    name: "Kingdom Death: Monster",
    dimensions: Dimensions {
        width: 610,
        height: 318,
        depth: 114,
    },
};

fn display(packed_items: RectanglePackOk<String, &str>, bins: HashMap<&str, &Bin>) {
    let mut reversed = HashMap::new();

    for (k, v) in packed_items.packed_locations() {
        reversed
            .entry(v.0)
            .or_insert_with(Vec::new)
            .push((k.to_string(), v.1));
    }

    let red = Rgb([255u8, 0u8, 0u8]);
    let green = Rgb([0u8, 255u8, 0u8]);
    let blue = Rgb([0u8, 0u8, 255u8]);
    let colors = vec![red, green, blue];
    let mut color_index = 0;
    fs::create_dir_all("images").expect("Problem creating images dir");

    for (bin_id, games) in reversed {
        let bin = bins.get(bin_id).unwrap();
        let mut image = RgbImage::new(bin.dimensions.width, bin.dimensions.height);
        let path_str = format!("images/{}.jpg", bin_id);
        let path = Path::new(&path_str);

        for game in games {
            let color = colors[color_index % colors.len()];
            let game_location = game.1;
            color_index += 1;
            draw_filled_rect_mut(
                &mut image,
                Rect::at(
                    game_location.x() as i32,
                    (bin.dimensions.height - game_location.y() - game_location.height()) as i32,
                )
                .of_size(game_location.width(), game_location.height()),
                color,
            );
            println!("{:?}", game);
            image.save(path).expect("Problem saving image");
        }
    }
}

fn main() {
    let kallax = &Bin::new(
        "Kallax",
        Dimensions {
            height: 330,
            width: 330,
            depth: 390,
        },
    );

    let billy = &Bin::new(
        "Billy",
        Dimensions {
            height: 400,
            width: 760,
            depth: 390,
        },
    );
    let game1 = KDM;
    let game2 = TM_DICE;
    let bin = kallax;
    let bins = HashMap::from([(billy.name, billy), (kallax.name, kallax)]);
    let options = Options {
        allow_depth_overflow: true,
        vertical: false,
    };

    let mut rects_to_place = GroupedRectsToPlace::new();
    let games = vec![game1, game2];
    for (i, game) in games.iter().enumerate() {
        let usable_dimensions = game.effective_dimensions_for_bins(bin, options);

        rects_to_place.push_rect(
            game.name.to_string(),
            Some(vec![i]),
            RectToInsert::new(usable_dimensions.width, usable_dimensions.height, 1),
        );
    }

    let target_bin = TargetBin::new(bin.dimensions.width, bin.dimensions.height, 1);
    let mut target_bins = BTreeMap::new();
    target_bins.insert(bin.name, target_bin);

    let rectangle_placements = pack_rects(
        &rects_to_place,
        &mut target_bins,
        &volume_heuristic,
        &contains_smallest_box,
    )
    .expect("Not all games will fit in bins");
    display(rectangle_placements, bins);
}
