use std::fmt;

pub type Duration = fraction::GenericFraction<u16>;

// impl fmt::Debug for Duration {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if self.denom == 1 {
//             self.fmt.write_fmt(format_args!("{}", self.numer))
//         } else {
//             self.fmt.write_fmt(format_args!("{}/{}", self.numer, self.denom))
//         }
//     }
// }

mod tests {
    use super::*;

    #[test]
    fn test() {
        let f = Duration::new(1u16, 8u16);
        print!("{:?}", f);
        let f = Duration::new(8u16, 1u16);
        print!("{:?}", f);
    }
}