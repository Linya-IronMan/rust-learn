macro_rules! example {
    (A B) => {
        println!("A B =>>>>");
    };
    (A:B) => {
        println!("A:B====>>>");
    };
}

fn main() {
    example![A B];
    example! { A  B  };
    example!(A: B);
}
