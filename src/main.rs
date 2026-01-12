struct Point<T, U> {
    x: T,
    y: U,
}

trait Overview {
    fn overview(&self) -> String {
        String::from("This is a Rust course")
    }
}

struct Course {
    headline: String,
    author: String,
}

impl Drop for Course {
    fn drop(&mut self){
        println!("Dropping: {}",self.author)
    }
}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for Course {
    // fn overview(&self) -> String {
    //     format!("{}, {}", self.author, self.headline)
    // }
}
impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

fn main() {
    let coord = Point { x: 5.0, y: "me" };
    let course1 = Course {
        headline: String::from("Headline!"),
        author: String::from("Rudiger!"),
    };
    let course2 = AnotherCourse {
        headline: String::from("Another Headline!"),
        author: String::from("Deskillz47!"),
    };

    // println!("{}", course1.overview());
    // println!("{}", course2.overview());

    // call_overview(&course1);
    // call_overview(&course2);

  
}

fn call_overview(item: &impl Overview){
    println!("{}", item.overview());
}
