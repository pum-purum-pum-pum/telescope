// pub struct Scope {
// 	start: u64,
// 	end: u64,
// 	depth: u64,

// }

#[derive(Debug)]
pub struct Region<T> {
    pub start: T,
    pub end: T,
}
