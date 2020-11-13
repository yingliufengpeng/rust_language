
// As an exercise, let's take the proposed input and feet it through the rule,
// to see how it is processed. The "Position" column will show which part of the
// syntax pattern needs to be matched against nex, denoted by a "⌂". Note that in some
// cases, there must be more than one possible "next" element to match against. "Input"
// will contain all of tokens that have not been consumed yet. inits and recur will contain
// the contents of those bindings.

// Note: there are two ⌂ here, because the next input token might match either the comma separator
// or between element in the repetition, or the comma after the repetition. The macro system will keeyp
// track of possibilities, until it is able to decide which noe to follow.

// Note: the third, crossed-out marker indicates that marco system has, as a consequences of the last
// token consumed, eliminated one of the previous possible branches.

// Note: this particular step should make it clear that a binding like $recur:expr will consume an entire
// expression, using the compiler's knowledge of what constitutes a valid expression. As will be noted later,
// you can do this for other language constructs.

macro_rules! recurrence {
    (a[n]: $sty:ty = $($inits:expr) , +  ;  $recur:expr) => {{




    }};
}


#[cfg(test)]
mod tests {
    use std::ops::Index;

    #[test]
    fn test_001() {
        let fib = recurrence![a[n]: u64 = 0, 1  ;  a[n-1] + a[n - 2] ];

        let fib = {

            struct IndexOffset<'a> {
                slice: &'a [u64; 2],
                offset: usize
            }

            impl <'a> Index<usize> for IndexOffset<'a> {
                type Output = u64;

                #[inline(always)]
                fn index(&self, index: usize) -> &Self::Output {
                    use std::num::Wrapping;
                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(2);

                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }

            struct Recurrence {
                mem: [u64; 2],
                pos: usize,
            };

            impl Iterator for Recurrence {
                type Item = u64;

                #[inline]
                fn next(&mut self) -> Option<Self::Item> {
                    if self.pos < 2 {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        // Note that I've changed the order of declarations of n and a, as well as wrapped
                        // them (along with the recurrence expression) in a block. The reason for the first
                        // should be obvious(n needs to be defined first so i can use if for a). The reason
                        // for the second is that the borrowed reference &self.mem will prevent the swaps later
                        // on from happening(you cannot mutate something that is aliased elsewhere). The block
                        // ensures that the &self.mem borrow expires before then.
                        let next_val = {
                            let n = self.pos;
                            let a = IndexOffset {slice: &self.mem, offset: n};
                            (a[n - 1] + a[n - 2])
                        };
                        {
                            use std::mem::swap;

                            let mut swap_tmp = next_val;
                            for i in (0..2).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }

                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }



            Recurrence { mem: [0, 1], pos: 0}
        };

        for e in fib.take(50) {
            println!("{}", e);
        }
    }
}