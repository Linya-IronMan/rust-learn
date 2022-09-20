fn main() {
    println!("HEllo world");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_otherbin() {
        assert_eq!(2 + 2, 4);
    }
}
