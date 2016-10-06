extern crate libc;
use libc::c_int;


#[link(name = "gringo")] 
extern {
    fn clingo_version(major: *mut c_int, minor: *mut c_int, revision: *mut c_int);
}

pub fn safe_clingo_version()-> (i32, i32, i32) {
    let mut m1 = 0;
    let ma = &mut m1 as *mut c_int;
   
    let mut m2 = 0;
    let mi = &mut m2 as *mut c_int;
    
    let mut m3 = 0;
    let re = &mut m3 as *mut c_int;

    unsafe { clingo_version(ma,mi,re) };

    let major = unsafe { *ma};
    let minor = unsafe { *mi};
    let revision = unsafe { *re};
    (major,minor,revision)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
       let (ma,mi,re) = safe_clingo_version();
       assert!(ma==5);
    }
}
