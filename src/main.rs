macro_rules! import_days {
    ($($day:tt),*) => {

        $(
            mod $day;
        )*

        fn main()
        {
            $($day::run();)*
        }


    };
}



import_days!(day01, day02, day03);

mod macros
{
    macro_rules! run_parts {
    ($day:ident: $($part:ident),+)
    => {
        let input = include_str!("input.txt");
        println!("{}:",stringify!($day));
        $(
            println!(" {} {}",stringify!($part), $part::solve(input));
        )*
    };
    }

    pub(crate) use run_parts;
}
