pub mod implementation;
pub mod scrapper;

use image::{Rgb, RgbImage};
use implementation::{Bin, Dimensions, Options};
use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, RectToInsert,
    RectanglePackOk, TargetBin,
};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut};
use imageproc::rect::Rect;
use std::fs;

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
            let game_location = game.1;
            if game_location.width() == 0 {
                continue;
            }
            let color = colors[color_index % colors.len()];
            color_index += 1;
            println!("{:?}", game);
            draw_hollow_rect_mut(
                &mut image,
                Rect::at(
                    game_location.x() as i32,
                    (bin.dimensions.height - game_location.y() - game_location.height()) as i32,
                )
                .of_size(game_location.width(), game_location.height()),
                color,
            );
            image.save(path).expect("Problem saving image");
        }
    }
}

fn main() {
    let game_collection = scrapper::get_collection("jenarvaezg".to_string());
    let kallax1 = &Bin::new(
        "Kallax1",
        Dimensions {
            height: 330,
            width: 330,
            depth: 390,
        },
    );
    let kallax2 = &Bin::new(
        "Kallax2",
        Dimensions {
            height: 330,
            width: 330,
            depth: 390,
        },
    );
    let kallax3 = &Bin::new(
        "Kallax3",
        Dimensions {
            height: 330,
            width: 330,
            depth: 390,
        },
    );
    let kallax4 = &Bin::new(
        "Kallax4",
        Dimensions {
            height: 330,
            width: 330,
            depth: 390,
        },
    );
    let kallax5 = &Bin::new(
        "Kallax5",
        Dimensions {
            height: 330,
            width: 330,
            depth: 390,
        },
    );
    let kallax6 = &Bin::new(
        "Kallax6",
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
    let bin1 = kallax1;
    let bin2 = kallax2;
    let bin3 = kallax3;
    let bin4 = kallax4;
    let bin5 = kallax5;
    let bin6 = kallax6;
    let bins = HashMap::from([
        (billy.name, billy),
        (kallax1.name, kallax1),
        (kallax2.name, kallax2),
        (kallax3.name, kallax3),
        (kallax4.name, kallax4),
        (kallax5.name, kallax5),
        (kallax6.name, kallax6),
    ]);
    let options = Options {
        allow_depth_overflow: true,
        vertical: true,
    };

    let mut rects_to_place = GroupedRectsToPlace::new();
    for (i, game) in game_collection.iter().enumerate() {
        let usable_dimensions = game.effective_dimensions_for_bins(bin1, options);

        rects_to_place.push_rect(
            game.name.to_string(),
            Some(vec![i]),
            RectToInsert::new(usable_dimensions.width, usable_dimensions.height, 1),
        );
    }

    let target_bin1 = TargetBin::new(bin1.dimensions.width, bin1.dimensions.height, 1);
    let target_bin2 = TargetBin::new(bin2.dimensions.width, bin2.dimensions.height, 1);
    let target_bin3 = TargetBin::new(bin3.dimensions.width, bin3.dimensions.height, 1);
    let target_bin4 = TargetBin::new(bin4.dimensions.width, bin4.dimensions.height, 1);
    let target_bin5 = TargetBin::new(bin5.dimensions.width, bin5.dimensions.height, 1);
    let target_bin6 = TargetBin::new(bin6.dimensions.width, bin6.dimensions.height, 1);
    let mut target_bins = BTreeMap::new();
    target_bins.insert(bin1.name, target_bin1);
    target_bins.insert(bin2.name, target_bin2);
    target_bins.insert(bin3.name, target_bin3);
    target_bins.insert(bin4.name, target_bin4);
    target_bins.insert(bin5.name, target_bin5);
    target_bins.insert(bin6.name, target_bin6);

    let rectangle_placements = pack_rects(
        &rects_to_place,
        &mut target_bins,
        &volume_heuristic,
        &contains_smallest_box,
    )
    .expect("Not all games will fit in bins");
    display(rectangle_placements, bins);
}
