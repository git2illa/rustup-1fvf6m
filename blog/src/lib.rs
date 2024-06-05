
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post{
    pub fn new() -> Post{
        Post{
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State{}

struct Draft {}

impl State for Draft {}
