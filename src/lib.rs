use std::vec::Vec;

pub type NodeLabel = u32; 
pub type ColumnLabel = u32; 
pub type RowEntry = u32; 

pub struct NodeLabelList {
    pub l : Vec<NodeLabel>,
}

pub struct Link {
    pub n : NodeLabelList,
    pub c : ColumnLabel,
}

pub struct LinkList {
    pub l : Vec<Link>,
}

pub struct ColumnLabelList {
    pub l : Vec<ColumnLabel>,
}

pub struct RowEntryList {
    pub l : Vec<RowEntry>,
}

pub struct Node {
    pub n : NodeLabel,
    pub c : ColumnLabelList,
    pub r : RowEntryList,
}

pub struct NodeList {
    pub l : Vec<Node>,
}

pub struct Graph {
    pub n : NodeList,
    pub l : LinkList,
}
