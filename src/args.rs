fn get_size_nth(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args{
    pub image_1: String,
    pub image_2: String,
    pub output: String
}

impl Args {
    pub fn new() -> Self {
        Args {
           image_1: get_size_nth(1),
           image_2: get_size_nth(2),
           output: get_size_nth(3),
        }
    }
}