use std::io;

fn main() {
    println!("Hello, world!");

    // for now the text buffer is a list of lines, each line is a string
    let mut buffer: Vec<String> = Vec::new();

    loop {
        let mut new_line = String::new();
        io::stdin()
            .read_line(&mut new_line)
            .expect("Failed to read input");
        buffer.push(new_line);
        for line in &buffer {
            let display_line = &line[0..line.len() - 1];
            println!("{display_line}");
        }
    }
}
