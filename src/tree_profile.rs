use id_arena::Id;

#[derive(Debug, Clone)]
pub struct Scope {
    pub width: u16,
    pub desc: String,
    pub color: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct TreeProfile {
    pub data: Scope,
    pub children: Vec<Id<TreeProfile>>,
}
