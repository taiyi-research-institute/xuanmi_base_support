extern crate xuanmi_base_support;
use xuanmi_base_support::*;

fn main() {
    fn actual_test() -> Outcome<()> {
        use std::fs::File;
        let path = "!!$%!$>TXT";
        let _f = File::open("!!$%!$>TXT");
        _f.catch(
            "IntendedException",
            &format!("Path \"{}\" has no file.", path),
        )?;
        Ok(())
    }
    fn actual_test2() -> Outcome<()> {
        let _x = actual_test().catch("AnotherIntendedException", "")?;
        Ok(())
    }
    let x = actual_test2();
    println!("{:#?}", x);
}
