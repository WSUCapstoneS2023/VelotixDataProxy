use rayon::prelude::*;

struct Person {
    age: u32,
}
fn main() {
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let num_under_30 = v.par_iter().filter(|&x| x.age < 30).count() as f32;
    let sum_over_30 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |x, y| x + y);

    let sum_under_30 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x < 30)
        .reduce(|| 0, |x, y| x + y);

    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let avg_under_30 = sum_under_30 as f32/num_under_30;

    println!("\n\nThe number of people older than 30 is {}", num_over_30);
    println!("The number of people younger than 30 is {}", num_under_30);

    println!("\nThe average age of people older than 30 is {}", avg_over_30);
    println!("The average age of people younger than 30 is {}", avg_under_30);

}


