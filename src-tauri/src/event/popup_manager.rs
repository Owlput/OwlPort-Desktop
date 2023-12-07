use serde::Serialize;

#[derive(Debug,Clone,Serialize)]
pub struct Popup{
    pub timeout:u32,
    pub stamp:String,
    pub component:String,
    pub props:String,
}