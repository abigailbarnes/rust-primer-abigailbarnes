fn task1() {
    /*
    TASK 1
    *******
    Clippy is a tool that allows you to catch mistakes and improve your code. For this
    exercise, run ``` cargo clippy ``` to see the suggestions and correct the errors.
    */
    let x = 1.2331f64;
    let y = 1.2332f64;
    if y != x {
        // clippy will suggest adding in a margin of error
        println!("Success!");
    }

    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        // clippy will suggest changing this to 'if let Some(x) = option'
        res += x;
    }
    println!("{}", res);
}

/*
    TASK 2
    *******
    Go back to the exercises in Module 1 and Module 2 and run cargo fmt
    Note if this changes your formatting.

    !!!done!!!



    TASK 3
    *******
    Generics Task:
    Create an animal trait, with characteristics such as type, and weight. Then initialize two different animal structs using the animal trait.
    NOTE FROM ABBY: it says in the read me to do three animals as well as age that differs from notes here
    */

struct Animal{
    id: u32,
    type_of_animal: String,
    name: String,
    weight: u32,
    age: u32
}

fn task3(){
    let _otty_the_otter = Animal{
        id: 101,
        type_of_animal: String::from("Otter"),
        name: String::from("Otty"),
        weight: 10,
        age: 3
    };

    let _funky_the_frog = Animal{
        id: 102,
        type_of_animal: String::from("Frog"),
        name: String::from("Funky"),
        weight: 2,
        age: 14
    };

    let _gina_the_giraffe = Animal{
        id: 103,
        type_of_animal: String::from("Giraffe"),
        name:  String::from("Gina"),
        weight: 1500,
        age: 30
    };
}


fn main()
{
    task1();
    task3();
}
