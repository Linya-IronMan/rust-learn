trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _:&Self::B) -> bool;
}

// 不使用关联类型
fn difference<A, B, C>(contains: &C) -> i32 where 
    C: Contains<A, B>
{}

fn difference<C>(contains: &C) -> i32 {...}


