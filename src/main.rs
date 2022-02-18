mod chapter3;
mod chapter4;
mod chapter5;
mod chapter6;
mod chapter8;
mod chapter9;

pub struct Guess {
    pub value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            // 予想の値は1から100の範囲でなければなりませんが、{}でした
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {
            value
        }
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    
    const LATEST_CHAPTER: i32 = 6;
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
        6 => chapter6::chapter6(),
        8 => chapter8::chapter8(),
        9 => chapter9::chapter9(),
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
