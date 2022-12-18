macro_rules! recurrence {
    ($($inits: expr), * ; ... ; $seq: ident[$idx: ident] = $recur: expr) => {};
}
fn main() {
    let lib = {
        struct Recurrence {
            mem: [u64; 2],
            pos: usize,
        }

        struct IndexOffset<'a> {
            slice: &'a [u64; 2],
            offset: usize,
        }

        impl<'a> Index<usize> for IndexOffset<'a> {
            type Output = u64;
            fn index(&self, index: usize) -> &Self::Output {
                // real_index 计算的是在 mem 中的 index
                let real_index = index - self.offset;

                // mem[real_index]
                &self.slice[real_index]
            }
        }

        impl Iterator for Recurrence {
            type Item = u64;

            fn next(&mut self) -> Option<Self::Item> {
                if self.pos < 2 {
                    let next_item = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_item)
                } else {
                    let next_item = {
                        let n = self.pos;
                        a[n - 1] + a[n - 2]
                    };
                    Some(next_item)
                }
            }
        }

        Recurrence {
            mem: [0, 1],
            pos: 0,
        }
    };

    for item in lib.take(10) {
        println!("{}", item);
    }
    // recurrence! {
    //     1, 2;
    //     ...;
    //     f[n] = f[n-1] + f[n-2]
    // }
}
