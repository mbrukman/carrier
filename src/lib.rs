pub extern crate prost;

mod revision;
pub use revision::REVISION;
include!(concat!(env!("OUT_DIR"), "/build_id.rs"));

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/carrier.broker.v1.rs"));
    include!(concat!(env!("OUT_DIR"), "/carrier.certificate.v1.rs"));
    include!(concat!(env!("OUT_DIR"), "/carrier.sysinfo.v1.rs"));
    include!(concat!(env!("OUT_DIR"), "/carrier.discovery.v1.rs"));
    include!(concat!(env!("OUT_DIR"), "/genesis.v1.rs"));
}




#[path = "../target/debug/rs/err.rs"]
pub mod err;

#[path = "../target/debug/rs/carrier_identity_kit.rs"]
pub mod identity_kit;

pub use err::Err as Error;

pub const ERR_TAIL : usize = 1000;
impl Error {
    pub fn check(&mut self) -> Result<(), std::io::Error> {
        unsafe {
            let this_file = file!();
            let this_line = line!();
            let e = err::check(
                self._self(),
                ERR_TAIL,
                this_file.as_bytes().as_ptr() as *const u8,
                std::ptr::null(),
                this_line as usize
            );
            if e  {
                Err(std::io::Error::new(std::io::ErrorKind::Other, self.clone()))
            } else {
                Ok(())
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = [0u8;1024];
        err::to_str(
            self._self(),
            s.as_mut_ptr(),
            s.len()
            );
        let ll = libc::strlen(s.as_ptr() as *const i8) + 1;
        let s :String = String::from_utf8_lossy(&s[..ll]).into();
        write!(f, "{}", s)?;
        Ok(())
    }
}
