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
    #[derive(Debug)]
    enum note {
        Note(i32),
        Chord (Vec<note>)
    }
    #[test]
    fn test2() {
        let nt = note::Note(3);
        let nt2 = note::Note(2);
        let ch = note::Chord(vec![nt, nt2]);
        println!("{:?}", ch);
    }
}