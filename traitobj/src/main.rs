use traitobj::{Button, Draw, Screen};

// ユーザー側で定義するDrawの実装例
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox: draw implementation...");
    }
}

fn main() {
    // Drawを実装している、違う型をcomponentsが保持できていることに注目
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
        ],
    };
    println!("Hello, world!");
}
