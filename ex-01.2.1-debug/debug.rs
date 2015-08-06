#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Tim(i32);

fn main() {
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name. SOUDELOR {typhoon:?}",
             "Slater",
             "Christian",
             actor="actor's",
             typhoon="coming 'soon'...to TAIWAN");

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));

    println!("Now {:?} will print!", Tim(100));
}
