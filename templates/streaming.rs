#![crate_type = "lib"]

#[no_mangle]
pub extern fn rust_iters(high: isize) -> isize {
    (1..high + 1)
        .filter(|x| x % 2 == 0)
        .map(|x| x * 2)
        .sum()
}

#[no_mangle]
pub extern fn rust_cheating(high: isize) -> isize {
    let mut total = 0;
    let mut i = 4;
    let high = high * 2;
    while i <= high {
        total += i;
        i += 4;
    }
    total
}

#[no_mangle]
pub extern fn rust_loop(high: isize) -> isize {
    let mut total = 0;
    let mut i = 1;
    while i <= high {
        if i % 2 == 0 {
            total += i * 2;
        }

        i += 1;
    }

    total
}

#[no_mangle]
pub extern fn rust_stream(high: isize) -> isize {
    sum_stream(
        map_stream(
            (|x| x * 2),
            filter_stream
                ( (|x| x % 2 == 0),
                    range_stream(1, high + 1)
                )
        )
    )
}

fn range_stream(low: isize, high: isize) -> RangeStream {
    RangeStream {
        low: low,
        high: high,
    }
}
struct RangeStream {
    low: isize, // FIXME generalize
    high: isize,
}
impl Stream for RangeStream {
    type Item = isize;
    #[inline]
    fn next(&mut self) -> Step<isize> {
        if self.low >= self.high {
            Step::Done
        } else {
            let ret = self.low;
            self.low += 1;
            Step::Yield(ret)
        }
    }
}

struct FilterStream<S, P> {
    stream: S,
    predicate: P,
}
fn filter_stream<S: Stream, P>(predicate: P, stream: S) -> FilterStream<S, P>
    where P: FnMut(&S::Item) -> bool {
    FilterStream {
        predicate: predicate,
        stream: stream,
    }
}
impl<S: Stream, P> Stream for FilterStream<S, P>
    where P: FnMut(&S::Item) -> bool {
    type Item = S::Item;
    #[inline]
    fn next(&mut self) -> Step<S::Item> {
        match self.stream.next() {
            Step::Done => Step::Done,
            Step::Skip => Step::Skip,
            Step::Yield(x) => {
                if (self.predicate)(&x) {
                    Step::Yield(x)
                } else {
                    Step::Skip
                }
            }
        }
    }
}

struct MapStream<S, F> {
    stream: S,
    f: F,
}
fn map_stream<S, F>(f: F, stream: S) -> MapStream<S, F> {
    MapStream {
        stream: stream,
        f: f,
    }
}
impl<B, S: Stream, F> Stream for MapStream<S, F>
    where F: Fn(S::Item) -> B {
    type Item = B;

    #[inline]
    fn next(&mut self) -> Step<B> {
        match self.stream.next() {
            Step::Done => Step::Done,
            Step::Skip => Step::Skip,
            Step::Yield(x) => {
                Step::Yield((self.f)(x))
            }
        }
    }
}

#[inline]
fn sum_stream<S>(mut stream: S) -> isize // FIXME generalize from isize
    where S: Stream<Item=isize>, {
    let mut total = 0;
    loop {
        match stream.next() {
            Step::Done => {
                return total;
            }
            Step::Skip => (),
            Step::Yield(i) => {
                total += i;
            }
        }
    }
}

enum Step<T> {
    Done,
    Skip,
    Yield(T),
}

trait Stream {
    type Item;
    fn next(&mut self) -> Step<Self::Item>;
}

#[no_mangle]
pub extern fn rust_stream_immut(high: isize) -> isize {
    sum_stream_immut(
        map_stream_immut(
            (|x| x * 2),
            filter_stream_immut
                ( (|x| x % 2 == 0),
                    range_stream_immut(1, high + 1)
                )
        )
    )
}

enum StepI<S, T> {
    Done,
    Skip(S),
    Yield(S, T),
}

trait StreamI where Self: Sized {
    type Item;
    fn next(self) -> StepI<Self, Self::Item>;
}

struct RangeStreamI {
    low: isize, // FIXME: generalize
    high: isize,
}
impl StreamI for RangeStreamI {
    type Item = isize;
    fn next(self) -> StepI<Self, Self::Item> {
        if self.low >= self.high {
            StepI::Done
        } else {
            let res = self.low;
            let new_self = RangeStreamI {
                low: self.low + 1,
                high: self.high,
            };
            StepI::Yield(new_self, res)
        }
    }
}
fn range_stream_immut(low: isize, high: isize) -> RangeStreamI {
    RangeStreamI {
        low: low,
        high: high,
    }
}
struct FilterStreamI<S, P> {
    stream: S,
    predicate: P,
}
fn filter_stream_immut<S: StreamI, P>(predicate: P, stream: S) -> FilterStreamI<S, P>
    where P: FnMut(&S::Item) -> bool {
    FilterStreamI {
        predicate: predicate,
        stream: stream,
    }
}
impl<S: StreamI, P> StreamI for FilterStreamI<S, P>
    where P: FnMut(&S::Item) -> bool {
    type Item = S::Item;
    #[inline]
    fn next(self) -> StepI<Self, S::Item> {
        match self.stream.next() {
            StepI::Done => StepI::Done,
            StepI::Skip(stream) => {
                StepI::Skip(FilterStreamI {
                    stream: stream,
                    predicate: self.predicate,
                })
            }
            StepI::Yield(stream, x) => {
                let mut p = self.predicate;
                if p(&x) {
                    let new_self = FilterStreamI {
                        stream: stream,
                        predicate: p,
                    };
                    StepI::Yield(new_self, x)
                } else {
                    StepI::Skip(FilterStreamI {
                        stream: stream,
                        predicate: p,
                    })
                }
            }
        }
    }
}

struct MapStreamI<S, F> {
    stream: S,
    f: F,
}
fn map_stream_immut<S, F>(f: F, stream: S) -> MapStreamI<S, F> {
    MapStreamI {
        stream: stream,
        f: f,
    }
}
impl<B, S: StreamI, F> StreamI for MapStreamI<S, F>
    where F: Fn(S::Item) -> B {
    type Item = B;

    #[inline]
    fn next(self) -> StepI<Self, B> {
        let f = self.f;
        match self.stream.next() {
            StepI::Done => StepI::Done,
            StepI::Skip(stream) => {
                StepI::Skip(MapStreamI {
                    f: f,
                    stream: stream,
                })
            }
            StepI::Yield(stream, x) => {
                let y = f(x);
                let new_self = MapStreamI {
                    f: f,
                    stream: stream,
                };
                StepI::Yield(new_self, y)
            }
        }
    }
}

#[inline]
fn sum_stream_immut<S>(stream: S) -> isize // FIXME generalize from isize
    where S: StreamI<Item=isize>, {
    let mut total = 0;
    let mut stream_option = Some(stream);
    while let Some(stream) = stream_option.take() {
        match stream.next() {
            StepI::Done => (),
            StepI::Skip(new_stream) => {
                stream_option = Some(new_stream);
            }
            StepI::Yield(new_stream, i) => {
                total += i;
                stream_option = Some(new_stream);
            }
        }
    }
    total
}

struct NoTrait<A> {
    next: Box<(FnMut() -> Option<A>)>,
}

#[no_mangle]
pub extern fn rust_no_trait(high: isize) -> isize {
    sum_nt(
        map_nt(
            (|x| x * 2),
            filter_nt
                ( (|x| x % 2 == 0),
                    range_nt(1, high + 1)
                )
        )
    )
}

fn sum_nt(mut nt: NoTrait<isize>) -> isize {
    let mut total = 0;
    while let Some(x) = (nt.next)() {
        total += x;
    }
    total
}

fn map_nt<F, A, B>(f: F, mut nt: NoTrait<A>) -> NoTrait<B>
    where F: Fn(A) -> B + 'static,
          A: 'static {
    NoTrait {
        next: Box::new(move || {
            match (nt.next)() {
                None => None,
                Some(x) => Some(f(x)),
            }
        })
    }
}

fn filter_nt<F, A>(f: F, mut nt: NoTrait<A>) -> NoTrait<A>
    where F: Fn(&A) -> bool + 'static,
          A: 'static {
    NoTrait {
        next: Box::new(move || {
            loop {
                match (nt.next)() {
                    None => {
                        return None;
                    }
                    Some(x) => {
                        if f(&x) {
                            return Some(x);
                        }
                    }
                }
            }
        })
    }
}

fn range_nt(mut low: isize, high: isize) -> NoTrait<isize> {
    NoTrait {
        next: Box::new(move || {
            if low >= high {
                None
            } else {
                let res = low;
                low += 1;
                Some(res)
            }
        })
    }
}