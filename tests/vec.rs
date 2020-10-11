// These tests are ripped straight from the std test suite
//
// # `std`'s MIT license.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use alt_dedup::NewDedup;

#[test]
fn test_new_dedup() {
    fn case(a: Vec<i32>, b: Vec<i32>) {
        let mut v = a;
        v.new_dedup();
        assert_eq!(v, b);
    }
    case(vec![], vec![]);
    case(vec![1], vec![1]);
    case(vec![1, 1], vec![1]);
    case(vec![1, 2, 3], vec![1, 2, 3]);
    case(vec![1, 1, 2, 3], vec![1, 2, 3]);
    case(vec![1, 2, 2, 3], vec![1, 2, 3]);
    case(vec![1, 2, 3, 3], vec![1, 2, 3]);
    case(vec![1, 1, 2, 2, 2, 3, 3], vec![1, 2, 3]);
}

#[test]
fn test_new_dedup_by_key() {
    fn case(a: Vec<i32>, b: Vec<i32>) {
        let mut v = a;
        v.new_dedup_by_key(|i| *i / 10);
        assert_eq!(v, b);
    }
    case(vec![], vec![]);
    case(vec![10], vec![10]);
    case(vec![10, 11], vec![10]);
    case(vec![10, 20, 30], vec![10, 20, 30]);
    case(vec![10, 11, 20, 30], vec![10, 20, 30]);
    case(vec![10, 20, 21, 30], vec![10, 20, 30]);
    case(vec![10, 20, 30, 31], vec![10, 20, 30]);
    case(vec![10, 11, 20, 21, 22, 30, 31], vec![10, 20, 30]);
}

#[test]
fn test_new_dedup_by() {
    let mut vec = vec!["foo", "bar", "Bar", "baz", "bar"];
    vec.new_dedup_by(|a, b| a.eq_ignore_ascii_case(b));

    assert_eq!(vec, ["foo", "bar", "baz", "bar"]);

    let mut vec = vec![("foo", 1), ("foo", 2), ("bar", 3), ("bar", 4), ("bar", 5)];
    vec.new_dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });

    assert_eq!(vec, [("foo", 3), ("bar", 12)]);
}

#[test]
fn test_new_dedup_unique() {
    let mut v0: Vec<Box<_>> = vec![Box::new(1), Box::new(1), Box::new(2), Box::new(3)];
    v0.new_dedup();
    let mut v1: Vec<Box<_>> = vec![Box::new(1), Box::new(2), Box::new(2), Box::new(3)];
    v1.new_dedup();
    let mut v2: Vec<Box<_>> = vec![Box::new(1), Box::new(2), Box::new(3), Box::new(3)];
    v2.new_dedup();
    // If the boxed pointers were leaked or otherwise misused, valgrind
    // and/or rt should raise errors.
}
