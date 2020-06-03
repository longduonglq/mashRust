mod tests {
    use std::slice::Iter;
    use std::rc::Rc;
    use std::cell::RefCell;

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

        let refc = RefCell::new(v);
        println!("{:?}", refc.borrow());
        let rc = Rc::new(refc.into_inner());
        println!("{:?}", rc);
    }
}