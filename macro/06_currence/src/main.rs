macro_rules! count {
    () => {0_usize};
    ($_: expr) => {1_usize};
    ($_: expr, $($tail: expr), *) => {
        1 + count!($($tail), *)
    }
}

#[allow(unused_macros)]
macro_rules! recurrence {
    (items:$ty:ty => $($inits: expr), * ; ... ; $seq: ident[$idx: ident] = $recur: expr) => {{
        use std::ops::Index;
        const LEN: usize = count!($($inits), *);

        println!("Len: {}", LEN);

        struct Recurrence {
            mem: [$ty; LEN],
            pos: usize,
        }

        struct IndexOffset<'a> {
            slice: &'a [$ty; LEN],
            offset: usize,
        }

        impl<'a> Index<usize> for IndexOffset<'a> {
            type Output = $ty;
            fn index(&self, index: usize) -> &Self::Output {
                // real_index 计算的是在 mem 中的 index
                let real_index = index - self.offset;

                // mem[real_index]
                &self.slice[real_index]
            }
        }

        impl Iterator for Recurrence {
            type Item = $ty;

            fn next(&mut self) -> Option<Self::Item> {
                if self.pos < LEN {
                    let next_item = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_item)
                } else {
                    let next_item = {
                        // 将标识符换成 macro 中传过来的
                        let $idx = self.pos;
                        let $seq = IndexOffset {
                            slice: &self.mem,
                            offset: $idx - LEN,
                        };
                        // a[n - 1] + a[n - 2]
                        $recur
                    };

                    {
                        use std::mem::swap;
                        let mut swap_tmp = next_item;
                        for i in (0..self.mem.len()).rev() {
                            swap(&mut swap_tmp, &mut self.mem[i])
                        }
                    }
                    self.pos += 1;
                    Some(next_item)
                }
            }
        }

        Recurrence {
            mem: [$($inits), *],
            pos: 0,
        }
    }};
}

fn main() {
    let lib = recurrence!(items:u64 => 0, 1, 3, 4; ... ; a[n] = a[n-3] + a[n-4]);

    for (index, val) in lib.take(20).enumerate() {
        println!("F[{}] = {}", index, val);
    }
    // recurrence! {
    //     1, 2;
    //     ...;
    //     f[n] = f[n-1] + f[n-2]
    // }
}
