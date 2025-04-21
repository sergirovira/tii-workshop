fn main() {
    let arr = [2, 4, 8, 16];

    let mut n = 2;
    let increased = {
        let temp = increased_by_first_item(&arr, &mut n);
        *temp
    };

    let nth = nth_item(&arr, &n);
    let values = TwoValues::new(&arr[3], &increased);

    assert_eq!(*values.get_first(), 16);

    let value = values.get_second();

    assert_eq!(*value, 4);
    assert_eq!(*nth, 8);
}

fn nth_item<'a>(data: &'a [usize], n: &'a usize) -> &'a usize {
    &data[*n]
}

fn increased_by_first_item<'a>(data: &'a [usize], n: &'a mut usize) -> &'a mut usize {
    *n += data[0];
    n
}

struct TwoValues<'a> {
    first: &'a usize,
    second: &'a usize,
}

impl<'a> TwoValues<'a> {
    pub fn new(first: &'a usize, second: &'a usize) -> Self {
        Self { first, second }
    }

    pub fn get_first(&self) -> &usize {
        self.first
    }

    pub fn get_second(&self) -> &usize {
        self.second
    }
}
