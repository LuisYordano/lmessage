use lunatic::{process, Mailbox};

#[lunatic::main]
fn main(m: Mailbox<()>) {
    let proc = process::spawn(|mailbox| {
        let message = mailbox.receive().unwrap();
        println!("Hello {}", message);
    })
    .unwrap();
    
    proc.send("Developers Rust 2022!".to_string());
}
