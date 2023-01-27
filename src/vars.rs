pub fn run() {
    
    let name = "Ribeiro";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 34;

    println!("My name is {} and I am {}", name, age);

    // Const
    const ID: i32 = 001;
    println!("ID {}",ID);

    let (myname, myage) = ("Joao", 52);
    println!("{} is {}", myname, myage);
}