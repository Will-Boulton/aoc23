use std::cmp::min;

pub(crate) fn solve(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input.lines().map(|l|l.chars().collect()).map(|mut x:Vec<char>|{x.push('.');x} ).collect();
    let mut parts = vec![];
    let h = lines.len();
    let w = lines.first().unwrap().len();
    println!("h{} w {}", h,w);
    for y in 0..h {
        let mut num_start:usize = 0;
        let mut num = String::new();
        for x in 0..w {
            match lines[y][x] {
                s if s.is_digit(10) => {
                    if num.is_empty() {
                        num_start = x;
                    }
                    num.push(s);
                },
                ch => {

                    if !num.is_empty() {
                        let parsed = num.parse::<usize>().unwrap();
                        let mut real_part = false;
                        let (x1, x2, y1, y2) = (num_start.checked_sub(1).unwrap_or(0usize), min(x,w-1), y.checked_sub(1).unwrap_or(0usize),min(y+1,h-1));
                        for a in x1..=x2  {
                            for b in y1..=y2{
                                //print!("{} {}",a,b);
                                if b == y && (a >= num_start && a <= x-1) {
                                    continue
                                }
                                if is_symbol(lines[b][a]) {
                                    real_part = true;
                                    break
                                }
                            }
                            if real_part {break}
                        }
                        if real_part {
                            parts.push(parsed);
                            print!("\x1b[32m{}\x1b[0m",parsed);
                        }
                        else{
                            print!("\x1b[31m{}\x1b[0m",parsed);
                        }
                        num_start = 0;
                        num.clear();
                    }
                    if is_symbol(ch) {
                        print!("\x1b[35m{}\x1b[0m",ch);
                    }
                    else {print!("{}",ch);}
                },
            }

        }
        println!()
    }

    return parts.iter().sum();
}

fn is_symbol(c :char) -> bool{
    if c.is_digit(10) {
        return false;
    }
    if c == '.' {
        return false;
    }
    return true;
}

fn filter_parts(part: &Part, lines: &Vec<&str>) -> Option<usize>
{

return None;
}

struct Part
{
    number : usize,
    line : usize,
    range : (usize, usize)
}

// 476765 too low