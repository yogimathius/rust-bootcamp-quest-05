#[derive(Debug, Clone, Copy)]
struct Origin<'a> {
    x: i32,
    y: i32,
    direction: &'a str,
}

fn my_spaceship(directions: &String) -> String {
    let mut origin: Origin = Origin {
        x: 0,
        y: 0,
        direction: "up",
    };

    directions.chars().for_each(|direction| {
        if direction == 'A' {
            // println!("advancing");
            advance_in_direction(&mut origin)
        } else {
            rotate(&mut origin, direction)
        }
    });
    let result = format!(
        "{{x: {}, y: {}, direction: '{}'}}",
        origin.x, origin.y, origin.direction
    );

    result
}

fn advance_in_direction(origin: &mut Origin) {
    match origin.direction {
        "up" => {
            // println!("updating y");
            origin.y -= 1;
            // println!("new y: {}", origin.y);
        }
        "down" => origin.y += 1,
        "left" => origin.x -= 1,
        "right" => origin.x += 1,
        _ => panic!("Unhandled origin.direction value: {}", origin.direction),
    }
}

fn rotate(origin: &mut Origin, direction_turning: char) {
    let directions = vec!["up", "right", "down", "left"];

    let mut new_direction_index = directions
        .iter()
        .position(|&dir| {
            // println!("dir: {}", dir);
            // println!("origin.direction: {}", origin.direction);
            dir == origin.direction
        })
        .unwrap();
    // println!("new direction index: {}", new_direction_index);

    let last_array_index = directions.len() - 1;
    match direction_turning {
        'L' => {
            if new_direction_index == 0 {
                new_direction_index = last_array_index
            } else {
                new_direction_index -= 1;
            }
            // println!("new NEW direction index: {}", new_direction_index);
            origin.direction = directions[new_direction_index];
            // println!("new oriign.direction: {}", origin.direction);
        }
        'R' => {
            if new_direction_index == last_array_index {
                new_direction_index = 0
            } else {
                new_direction_index += 1;
            }
            origin.direction = directions[new_direction_index];
        }
        _ => panic!("Unhandled direction_turning value: {}", direction_turning),
    }
}

// fn main() {
//     println!("{}", my_spaceship("RAALALL".to_string()));
// }
