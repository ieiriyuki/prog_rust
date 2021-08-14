fn main() {
    let mut a = Application { name: "Zeus".to_string(),
                              nicknames: vec!["cloud collector".to_string(),
                                              "king of the gods".to_string()] };
    println!("befor assignment");
    a = Application { name: "Hera".to_string(), nicknames: vec![] };
    println!("at the end of block");

    // 13.2
    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: "lunch".to_string()
    };

    let box_displayable: &RcBox<Display> = &boxed_lunch;

    display(&boxed_lunch);

    // 13.3

}

struct Application {
    name: String,
    nicknames: Vec<String>
}

impl Drop for Application {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

// 13.2
struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T,
}

use std::fmt::Display;

fn display(boxed: &RcBox<Display>) {
    println!("For your enjoyment: {}", &boxed.value);
}
