use self::book::Book;



pub mod order;
pub mod book;
pub mod trade_history;

static mut DB: Option<Book> = None;

pub fn initialize_db() {
    unsafe {
        DB = Some(Book::new());
    }
}

