use super::lexer::Operator;

pub fn parseExp(e: Vec<String>) {
    let mut inter = e[0].parse::<i32>().unwrap();

    let mut i = 1;
    while i < e.len() {
        let op = Operator::from_str(&e[i]).unwrap();
        let next = e[i+1].parse::<i32>().unwrap();

        match op {
            Operator::Add => inter += next,
            Operator::Sub => inter -= next,
            Operator::Mul => inter *= next,
            Operator::Div => inter /= next,
        }
        i += 2;
    }

    println!("\nResult: {}", inter);
}
