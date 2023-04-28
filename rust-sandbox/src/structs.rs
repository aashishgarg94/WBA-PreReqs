struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct Person {
    first_name: String,
    last_name: String,
    location: String
}

impl Person{
    fn new(first: &str, last: &str, loc: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            location: loc.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String, String){
        (self.first_name, self.last_name, self.location)
    }
}

pub fn run(){
    // Create struct
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // Create struct with function
    let mut p = Person::new("Brad", "Traversy", "Mass");
    println!("Person: {} {} {}", p.first_name, p.last_name, p.location);
    println!("Person: {}", p.full_name());

    p.set_last_name("Smith");
    println!("Person: {}", p.full_name());

    println!("Person Tuple: {:?}", p.to_tuple());
}