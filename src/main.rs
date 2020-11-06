mod item;

use std::io;
use item::Item;

const CLEAR_SCREEN: &str = "\x1B[2J";

fn print_options() {
    println!("");
    println!("n: New item");
    println!("d: Delete item");
    println!("q: Quit")
}

fn read_opt() -> Option<char> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let line = buf.trim();
    line.chars().next()
}

fn read_item() -> io::Result<Item> {
    println!("Title: ");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let title = buf.trim();
    if title.len() == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Item must have a title"));
    }

    println!("Description: (leave empty if none)");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let desc = buf.trim();

    Ok(Item::new(
        &title,
        if desc.len() > 0 { Some(&desc) } else { None }
    ))
}

fn read_index() -> Result<usize, ()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).map_err(|x| ())?;
    buf.trim().parse::<usize>().map_err(|x| ())
}

fn main() -> io::Result<()> {
    let mut items = Vec::<Item>::new();
    loop {
        for i in &items {
            println!("{}", i.to_string());
        }

        print_options();
        let opt = read_opt();
        print!("{}", CLEAR_SCREEN);

        match opt {
            Some('n') => {
                match read_item() {
                    Ok(it) => { items.push(it) },
                    Err(io::ErrorKind::InvalidInput) => {
                        println!("{}", msg);
                    },
                    Err(error) => { return error; }
                }
            },
            Some('d') => {
                match read_index() {
                    Ok(idx) => {
                        if idx < items.len() {
                            items.remove(idx);
                        } else {
                            println!("Invalid index");
                        }
                    },
                    Err(_) => {
                    }
                }
            },
            Some('q') => break,
            Some(other) => {
                println!("Invalid option: '{}'", other);
            }
            None => {}
        }
    }
    Ok(())
}
