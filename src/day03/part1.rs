pub(crate) fn solve(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    //let maybe_parts = vec![];

    /*for line in lines {
        let mut chars = line.chars();
        let (mut s, mut e) = (0usize,0usize);
        let num = String::new();

        match chars.next() {
            Some('.') => {
                if !num.is_empty() {
                   // maybe_parts
                }
            }
        }
    }*/

    return 0;
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