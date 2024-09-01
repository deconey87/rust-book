pub trait Draw {
    fn draw(&self);
}

// Box<dyn Draw> はトレイトオブジェクト
// Drawトレイトの実装のBox、という感じ
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}

// Drawを実装している具体的なstruct
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button: draw implementation...");
    }
}

// トレイト境界を伴うジェネリックな型引数を使用する構造体だと、
// Tが何かに決まったらそれしか引き受けられない

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw()
//         }
//     }
// }
