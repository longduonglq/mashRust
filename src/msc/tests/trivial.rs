mod tests {
    use std::slice::Iter;
    use std::rc::Rc;

    fn recursive(mut iter: Iter<i32>) {
        let val = iter.next();
        match val {
            Some(v) => {
                println!("{}", v);
                recursive(iter)
            },
            None => {}
        }
    }

    #[test]
    fn main() {
        let v = vec![1,2,3,4,5,6];
        recursive(v.iter());
        println!("end");

        let rc = Rc::new(v);
        println!("{:?}", rc);
    }
}