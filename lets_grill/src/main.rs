use std::io;

fn main(){
    let mut skewers:Vec<String>=Vec::new();

    println!("Enter the skewer string one after other ( to indicate finish type 'done')");

    loop{
        let mut input=String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failled to read");
        let skewer=input.trim();

        if skewer.eq_ignore_ascii_case("done"){
            break;
        }

        skewers.push(skewer.to_string());

    }

    let (veg, non_veg) = get_skewer_types(&skewers);
    println!("Vegetarian: {}, Non-Vegetarian: {}", veg, non_veg);
}

fn get_skewer_types(skewers: &Vec<String>) -> (i8, i8) {
    let mut veg_skewers = 0;
    let mut not_veg_skewers = 0;

    for skewer in skewers {
        if skewer.contains("x") {
            not_veg_skewers += 1;
        } else if skewer.contains("o") {
            veg_skewers += 1;
        }
    }

    (veg_skewers, not_veg_skewers)
}
