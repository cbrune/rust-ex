#![allow(dead_code)]

trait Color {
    fn show(&mut self);
}

#[derive(Debug)]
struct Red {
    val: [u8; 1],
}

impl Red {
    pub const fn default() -> Self {
        Self { val: [0u8; 1] }
    }
}

const RED: Red = Red::default();

impl Color for Red {
    fn show(&mut self) {
        self.val[0] += 1;
        println!("red: {:?}", self);
    }
}

#[derive(Default, Debug)]
struct Green {
    val: [u8; 3],
}

impl Color for Green {
    fn show(&mut self) {
        self.val[0] += 1;
        println!("green: {:?}", self);
    }
}

#[derive(Default, Debug)]
struct Blue {
    val: [u8; 5],
}

impl Color for Blue {
    fn show(&mut self) {
        self.val[0] += 1;
        println!("blue: {:?}", self);
    }
}

#[derive(Debug)]
enum ColorEnum {
    Red(Red),
    Green(Green),
    Blue(Blue),
}

impl Default for ColorEnum {
    fn default() -> Self {
        ColorEnum::Red(Red::default())
    }
}

impl ColorEnum {
    fn show(&mut self) {
        match self {
            ColorEnum::Red(red) => red.show(),
            ColorEnum::Green(green) => green.show(),
            ColorEnum::Blue(blue) => blue.show(),
        }
    }
}

fn main() {
    let mut color_vec = vec![
        ColorEnum::Red(Red::default()),
        ColorEnum::Green(Green::default()),
        ColorEnum::Blue(Blue::default()),
        ColorEnum::Green(Green::default()),
        ColorEnum::Red(Red::default()),
    ];

    println!("begin dyn-ex!");
    println!("Size of Red  : {}", std::mem::size_of::<Red>());
    println!("Size of Green: {}", std::mem::size_of::<Green>());
    println!("Size of Blue : {}", std::mem::size_of::<Blue>());
    println!("Size of ColorEnum : {}", std::mem::size_of::<ColorEnum>());
    println!("Size of &Blue : {}", std::mem::size_of::<&Blue>());
    println!("Size of &ColorEnum : {}", std::mem::size_of::<&ColorEnum>());
    println!(
        "Size of color_enum_vec : {}",
        std::mem::size_of::<Vec<ColorEnum>>()
    );

    for (i, color) in color_vec.iter_mut().enumerate() {
        println!("color {}:", i);
        color.show();
    }

    let mut red1 = Red::default();
    let mut red2 = Red::default();
    let mut green1 = Green::default();
    let mut green2 = Green::default();
    let mut blue = Blue::default();
    let mut color_dyn_vec: Vec<&mut dyn Color> =
        vec![&mut red1, &mut green1, &mut blue, &mut green2, &mut red2];

    for (i, color) in color_dyn_vec.iter_mut().enumerate() {
        println!("color_dyn {}:", i);
        color.show();
    }
    println!(
        "Size of color_dyn : {}",
        std::mem::size_of::<&mut dyn Color>()
    );
    println!(
        "Size of color_dyn_vec : {}",
        std::mem::size_of::<Vec<&dyn Color>>()
    );
}
