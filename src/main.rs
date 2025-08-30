use rand::Rng;
use rand::prelude::IndexedRandom;
use rand::prelude::ThreadRng;
const MAX_DAYS: u32 = 7;

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

struct Patron {
    name: String,

}

#[derive(Debug, Clone)]
enum Action {
    Checkout(usize),
    Return(usize),
    Browse,
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Self { books: Vec::new() }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn display_book(book: &Book) {
        let status: &'static str = if book.available {
            "Available"
        } else {
            "Checked Out"
        };
        println!(
            "\"{}\" by {} ({} pages) {}",
            book.title, book.author, book.pages, status
        )
    }

    fn display_all_books(&self) {
        println!("\nLibrary Catalog");
        for (index, book) in self.books.iter().enumerate() {
            print!("{}. ", index);
            Self::display_book(book);
        }
    }

    fn checkout_book(&mut self, book_index: usize, patron: &Patron) -> bool {
        if let Some(book) = self.books.get_mut(book_index) {
            if book.available {
                book.available = false;
                println!(
                    "{} successfully checked out \"{}\".",
                    patron.name, book.title
                );
                return true;
            } else {
                println!("\"{}\" is already checked out!", book.title);
            }
        } else {
            println!("This index ({}) does not exist", book_index);
        }
        false
    }

    fn return_book(&mut self, book_index: usize) -> bool {
        if let Some(book) = self.books.get_mut(book_index) {
            if !book.available {
                book.available = true;
                println!("Successfully returned \"{}\".", book.title);
                return true;
            } else {
                println!("\"{}\" is NOT checked out!", book.title);
            }
        } else {
            println!("This index ({}) does not exist", book_index);
        }
        false
    }

    fn get_available_books_number(&self) -> usize {
        self.books
            .iter()
            .filter(|book: &&Book| book.available)
            .count()
    }
}

fn create_sample_patrons() -> Vec<Patron> {
    let names: [&'static str; 4] = ["Kostis", "Christina", "Daphne", "Theodora"];
    names
        .iter()
        
        .map(|&name| Patron {
            name: name.to_string(),
            
        })
        .collect()
}

fn generate_daily_action(
    rng: &mut impl Rng,
    book_count: usize,
    action_count: usize,
) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();

    for _ in 0..action_count {
        let action_type = rng.random_range(0..=10);
        let action = if book_count == 0 {
            Action::Browse
        } else {
            match action_type {
                0..=4 => Action::Checkout(rng.random_range(0..book_count)),
                5..=8 => Action::Return(rng.random_range(0..book_count)),
                _ => Action::Browse,
            }
        };
        actions.push(action);
    }
    actions
}

fn handle_action(library: &mut Library, action: Action, patrons: &[Patron], rng: &mut impl Rng) {
    if let Some(patron) = patrons.choose(rng) {
        match action {
            Action::Checkout(book_index) => {
                library.checkout_book(book_index, patron);
            }
            Action::Return(book_index) => {
                library.return_book(book_index);
            }
            Action::Browse => {
                println!("{} is browsing the library", patron.name);
            }
        }
    }
}

fn main() {
    println!("Library Simulation starting...");

    let mut library: Library = Library::new();
    let mut rng: ThreadRng = rand::rng();

    // Add 5 Rust-related books
    library.add_book(Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        pages: 560,
        available: true,
    });

    library.add_book(Book {
        title: String::from("Programming Rust"),
        author: String::from("Jim Blandy and Jason Orendorff"),
        pages: 735,
        available: true,
    });

    library.add_book(Book {
        title: String::from("Rust in Action"),
        author: String::from("Tim McNamara"),
        pages: 456,
        available: true,
    });

    library.add_book(Book {
        title: String::from("Zero to Production in Rust"),
        author: String::from("Luca Palmieri"),
        pages: 394,
        available: true,
    });

    library.add_book(Book {
        title: String::from("Hands-On Rust"),
        author: String::from("Herbert Wolverson"),
        pages: 342,
        available: true,
    });

    let patrons: Vec<Patron> = create_sample_patrons();

    let book_count: usize = library.books.len();

    let mut stats: (u32, u32, u32) = (0, 0, 0);

    library.display_all_books();

    for day in 1..MAX_DAYS {
        println!("\nDay {} ", day);
        let action_count: usize = rng.random_range(3..=7);
        let daily_actions: Vec<Action> = generate_daily_action(&mut rng, book_count, action_count);

        for action in daily_actions {
            match action {
                Action::Checkout(_) => {
                    stats.0 += 1;
                    handle_action(&mut library, action, &patrons, &mut rng);
                }
                Action::Return(_) => {
                    stats.1 += 1;
                    handle_action(&mut library, action, &patrons, &mut rng);
                }
                Action::Browse => {
                    stats.2 += 1;
                    handle_action(&mut library, action, &patrons, &mut rng);
                }
            }
        }

        let available = library.get_available_books_number();
        let available = format!("{} / {} books available", available, book_count);
        println!("End of day {}", available);
    }

    println!("\n Simulation Complete!");
    println!(" Final Statistics:");
    println!("  Total checkout attempts: {}", stats.0);
    println!("  Total return attempts: {}", stats.1);
    println!("  Total browse sessions: {}", stats.2);
    println!("  Total activities: {}", stats.0 + stats.1 + stats.2);

    library.display_all_books();
}
