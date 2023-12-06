use std::fs::File;
use std::io::Read;
use std::path::Path;


fn main() {
    let path = Path::new("src/input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => ()
    }

    let mut count = 0;
    for (idx, game) in s.lines().enumerate() {
        println!("{} -> {:?}", (idx + 1), game);
        let red_count = count_colour(game, "red");
        let blue_count = count_colour(game, "blue");
        let green_count = count_colour(game, "green");
        println!("Max Red: {:?}", red_count);
        println!("Max Blue: {:?}", blue_count);
        println!("Max Green: {:?}", green_count);

        let valid_red = red_count < 13;
        let valid_green = green_count < 14;
        let valid_blue = blue_count < 15;

        if valid_blue & valid_green & valid_red {
            println!("**Valid game**! Game idx = {:?}", (idx+1));
            count = count + (idx + 1)
        } else {
            println!("Not a valid game");
        }
        println!("====================");
    }
    println!("Total count = {:?}", count);

}

fn count_colour(game: &str, color: &str) -> i32 {
    let mut max_count = 0;

    for draw in game.split(";") {
        let mut count = 0;
        for (idx, _) in draw.match_indices(color) {
            let x = &draw[..idx];
            let color_draw = match x.trim_end().rsplit(" ").next().expect("No numbers at end!").parse::<i32>() {
                Ok(number)  => number,
                Err(_) => 0,
            };
            count = count + color_draw;
        }
        max_count = if max_count > count { max_count } else { count };
    }

    max_count
}