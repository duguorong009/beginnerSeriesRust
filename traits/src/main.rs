
struct Film {
    title: String,
    director: String,
    studio: String,
}

struct Book {
    title : String,
    author : String,
    publisher: String,
}

trait Catalog {
    fn describe(&self);
}

impl Catalog for Film {
    fn describe(&self) {
        println!("{} was directed by {} through {} studios",
            self.title,
            self.director, 
            self.studio
        )
    }
}

impl Catalog for Book {
    fn describe(&self) {
        println!("{} was written by {} and published by {}",
            self.title,
            self.author, 
            self.publisher
        )
    }
}

fn main() {
    let capt_marvel = Film {
        title: String::from("captain marvel"),
        director: String::from("Anna Boden and Ryan Fleck"),
        studio: String::from("marvel")
    };
    capt_marvel.describe();
}
