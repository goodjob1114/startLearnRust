fn main() {
    println!("{} days", 32);

    println!("{0}, this is {1}. {1}, this is {0}. {0}!={1}", "Alice", "Bob");

    println!("{subject} {verb} {predicate}....yo man....Tim is a {tim}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps",
             tim="handsome boy");

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("My name is {0}, {1} {0}", "Bond", "Tim");
                            
    struct Structure(i32);

    // this structure require more complicated handling. This will not work. Just comment it.
    //println!("This struct `{}` won't print...", Structure(3));
}
