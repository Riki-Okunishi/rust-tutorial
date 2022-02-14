mod chapter3;
mod chapter4;
mod chapter5;

fn main() {
    
    const LATEST_CHAPTER: i32 = 5;
    println!("What chapter do you want to test? (3 ~ {})", LATEST_CHAPTER);

    let mut chapter: i32 = LATEST_CHAPTER;
    loop {
        let input = get_input();
        chapter = match input.parse() {
            Ok(i) => {
                println!("Your select is Chapter {}\n", i);
                i
            },
            Err(_) => {
                if input == "q" {
                    println!("Exit.");
                    return;
                } else if input == "" {
                    break;
                }
                println!("your input is {}", input);
                continue;
            },
        };
        break;
    }

    match chapter {
        3 => chapter3::chapter3(),
        4 => chapter4::chapter4(),
        5 => chapter5::chapter5(),
        _ => {
            println!("Not found chapter {}", chapter);
        }
    }

}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}
