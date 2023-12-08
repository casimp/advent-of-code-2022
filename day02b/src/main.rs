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

    let mut total_count = 0;
    for (idx, game) in s.lines().enumerate() {
        println!("{} -> {:?}", (idx + 1), game);
        let red_count = count_colour(game, "red");
        let blue_count = count_colour(game, "blue");
        let green_count = count_colour(game, "green");
        let count = red_count * blue_count * green_count;
        total_count = total_count + count;
    };
    println!("Total count = {:?}", total_count);

}

fn count_colour(game: &str, color: &str) -> i32 {
    let mut max_count = 0;
    for draw in game.split(";") {
        let mut count = 0;
        for (idx, _) in draw.match_indices(color) {
            let x = &draw[..idx];
            let color_draw = match x
            .trim_end()
            .rsplit(" ")
            .next()
            .expect("No numbers at end!")
            .parse::<i32>() {
                Ok(number)  => number,
                Err(_) => 0,
            };
            count = count + color_draw;
        }
        max_count = if max_count > count { max_count } else { count };
    }

    max_count
}