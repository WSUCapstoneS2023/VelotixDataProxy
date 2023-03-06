struct User<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}


struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}

    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    // testing normal struct
    let user1 = User {
        active: true,
        username: "someoneusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
    println!("username {}", user1.username);

    // Testing template struct
	let p = Point {x: "one", y: "two"};
    let p2 = Point {x: 1, y: 2};
	println!("p.x = {} p.y = {},", p.x(), p.y());
    println!("p.x = {} p.y = {},", p2.x(), p2.y());
}