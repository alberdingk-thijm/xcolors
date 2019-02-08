// x11api code is taken from github.com/jD91mZM2/xidlehook
mod x11api;
mod xcolors;

pub use crate::xcolors::XColors;

#[cfg(test)]
mod tests {
    #[test]
    fn xterm() {
        use crate::xcolors::XColors;
        let xterm = XColors::new("xterm");
        println!("{:?}", xterm);
        assert!(xterm.is_some());
    }
}
