use std::io;


fn print_triangle(arr: &Vec<Vec<i32>>) {
    let mut rows: Vec<String> = vec![];
    
    for row in arr {
        let s: Vec<String> = row.iter().map( |i| {
            i.to_string()
        }).collect();

        let joined_s = s.join("     ");
        rows.push(joined_s);
    }

    let max_len = rows.iter().last().expect("arr is empty").len();
    for row in rows {
        println!("{:^width$}", row, width=max_len);
    }
}



fn main() {
    let mut triangle: Vec<Vec<i32>> = vec![vec![1], vec![1, 1]];
    println!("Enter a Pascal triangle height: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Can't read user input!");
    
    let triangle_height = match input.trim_matches('\n').parse() {
        Ok(n) => n,
        Err(err) => {
            println!("Can't parse user input: {}. Triangle height is assigned default value 0", err);
            1
        }
    };

    println!("Pascal triangle height is: {}", triangle_height);

    for i in 2..triangle_height {
        let mut v: Vec<i32> = vec![];

        let row = &triangle[i - 1];

        for (idx, value) in row.iter().enumerate() {
            if idx != 0 {
                v.push(row[idx - 1] + value)
            }
        }

        v.insert(0, 1);
        v.push(1);
        triangle.push(v);
    }

    print_triangle(&triangle);
}
