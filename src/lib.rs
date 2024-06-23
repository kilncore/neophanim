#[cfg(test)]
pub mod tests;

use std::vec::Vec;
use std::time::Instant;
use std::time::Duration;

pub type NodeLabel = u32; 
pub type ColumnLabel = u32; 
pub type RowEntry = u16; 

pub const FALSE_COLUMN : ColumnLabel = 0;
pub const TRUE_COLUMN : ColumnLabel = 1;

// ----------------------------------------------------------------------------
pub struct NodeLabelList {
    pub l : Vec<NodeLabel>,
}
pub const EMPTY_NODELABELLIST : NodeLabelList = NodeLabelList {
    l : Vec::new(),
};
impl NodeLabelList {
    pub fn count(&self) -> usize {
        self.l.len()
    }
    pub fn add(&mut self, l: NodeLabel) {
        self.l.push(l);
    }
    pub fn get(&self, i: usize) -> NodeLabel {
        *self.l.get(i).unwrap()
    }
}

// ----------------------------------------------------------------------------
pub struct Link {
    pub n : NodeLabelList,
    pub c : ColumnLabel,
}
pub const EMPTY_LINK : Link = Link {
    n: EMPTY_NODELABELLIST,
    c: 0,
};

pub struct LinkList {
    pub l : Vec<Link>,
}
pub const EMPTY_LINKLIST : LinkList = LinkList {
    l : Vec::new(),
};
impl LinkList {
    pub fn count(&self) -> usize {
        self.l.len()
    }
    pub fn add(&mut self, l: Link) {
        self.l.push(l);
    }
    pub fn get(&self, i: usize) -> &Link {
        self.l.get(i).unwrap()
    }
    pub fn get_mut(&mut self, i: usize) -> &mut Link {
        self.l.get_mut(i).unwrap()
    }
}

// ----------------------------------------------------------------------------
pub struct ColumnLabelList {
    pub l : Vec<ColumnLabel>,
}
pub const EMPTY_COLUMNLABELLIST : ColumnLabelList = ColumnLabelList {
    l : Vec::new(),
};
impl ColumnLabelList {
    pub fn count(&self) -> usize {
        self.l.len()
    }
    pub fn add(&mut self, l: ColumnLabel) {
        self.l.push(l);
    }
    pub fn get(&self, i: usize) -> ColumnLabel {
        *self.l.get(i).unwrap()
    }
    pub fn remove(&mut self, i: usize) {
        self.l.remove(i);
    }
}

// ----------------------------------------------------------------------------
pub struct RowEntryList {
    pub l : Vec<RowEntry>,
}
pub const EMPTY_ROWENTRYLIST : RowEntryList = RowEntryList {
    l : Vec::new(),
};
impl RowEntryList {
    pub fn count(&self) -> usize {
        self.l.len()
    }
    pub fn add(&mut self, e: RowEntry) {
        self.l.push(e);
    }
    pub fn get(&self, i: usize) -> RowEntry {
        *self.l.get(i).unwrap()
    }
    pub fn get_mut(&mut self, i: usize) -> &mut RowEntry {
        self.l.get_mut(i).unwrap()
    }
    pub fn remove(&mut self, i: usize) {
        self.l.remove(i);
    }
}

// ----------------------------------------------------------------------------
pub struct Node {
    pub n : NodeLabel,
    pub c : ColumnLabelList,
    pub r : RowEntryList,
    pub a : bool,
}
impl Node {
    pub fn collapse_columns(&mut self) {
        let mut col = 0;
        while col < self.c.count() {
            let cl = self.c.get(col);
            let mut col2 = col + 1;
            while col2 < self.c.count() {
                let cl2 = self.c.get(col2);
                if cl == cl2 {
                    self.c.remove(col2);
                    let mut row = 0;
                    while row < self.r.count() {
                        let re = self.r.get_mut(row);
                        if ((*re >> col) & 1) != ((*re >> col2) & 1) {
                            self.r.remove(row);
                        } else {
                            let mask1 = (1 << col2) - 1;
                            let mut mask2 = 0xffff;
                            mask2 ^= 1 << col2;
                            mask2 ^= mask1;
                            *re = (*re & mask1) | ((*re & mask2) >> 1 );
                            row += 1;
                        }
                    }
                } else {
                    col2 += 1;
                }
            }
            col += 1;
        }
    }
}

// ----------------------------------------------------------------------------
pub struct NodeList {
    pub l : Vec<Node>,
}
pub const EMPTY_NODELIST : NodeList = NodeList {
    l : Vec::new(),
};
impl NodeList {
    pub fn count(&self) -> usize {
        self.l.len()
    }
    pub fn add(&mut self, mut n: Node) {
        n.collapse_columns();
        self.l.push(n);
    }
    pub fn get(&self, i: usize) -> &Node {
        self.l.get(i).unwrap()
    }
    pub fn get_mut(&mut self, i: usize) -> &mut Node {
        self.l.get_mut(i).unwrap()
    }
}

// ----------------------------------------------------------------------------
pub struct Graph {
    pub n : NodeList,
    pub l : LinkList,
    pub c : ColumnLabel,
}
pub const EMPTY_GRAPH : Graph = Graph {
    n : EMPTY_NODELIST,
    l : EMPTY_LINKLIST,
    c : 0,
};
impl Graph {
    pub fn init(&mut self) {
        assert!(self.c == 0);
        // false
        let nl : NodeLabel = self.n.count() as NodeLabel;
        let mut ccl = EMPTY_COLUMNLABELLIST;
        ccl.add(FALSE_COLUMN);
        let mut rel = EMPTY_ROWENTRYLIST;
        rel.add(0);
        self.n.add(Node{
            n: nl,
            c: ccl,
            r: rel,
            a: true,
        });
        // true
        let nl : NodeLabel = self.n.count() as NodeLabel;
        let mut ccl = EMPTY_COLUMNLABELLIST;
        ccl.add(TRUE_COLUMN);
        let mut rel = EMPTY_ROWENTRYLIST;
        rel.add(1);
        self.n.add(Node{
            n: nl,
            c: ccl,
            r: rel,
            a: true,
        });
        self.c = 2;
    }
    pub fn get_new_column_label(&mut self) -> ColumnLabel {
        if self.c == 0 { self.init(); }
        let r = self.c;
        self.c += 1;
        r
    }
    pub fn register_var(&mut self) -> ColumnLabel {
        let cl = self.get_new_column_label();
        let nl : NodeLabel = self.n.count() as NodeLabel;
        let mut ccl = EMPTY_COLUMNLABELLIST;
        ccl.add(cl);
        let mut rel = EMPTY_ROWENTRYLIST;
        rel.add(0);
        rel.add(1);
        self.n.add(Node{
            n: nl,
            c: ccl,
            r: rel,
            a: true,
        });
        cl
    }
    pub fn register_bit(&mut self, b: bool) -> ColumnLabel {
        if self.c == 0 { self.init(); }
        if b { TRUE_COLUMN } else { FALSE_COLUMN }
    }
    pub fn register_number(&mut self, n: String) -> ColumnLabelList {
        let n = n.chars().rev().collect::<String>();
        let mut l = EMPTY_COLUMNLABELLIST;
        for b in n.chars() {
            match b {
                '0' => { l.add(self.register_bit(false)); },
                '1' => { l.add(self.register_bit(true)); },
                _ => unreachable!(),
             }
        }
        l
    }
    pub fn extract_bit(&self, cl: ColumnLabel) -> bool {
        for n in &self.n.l {
            let mut offset = 0;
            for c in &n.c.l {
                if *c == cl {
                    assert!(n.r.count() == 1);
                    return (n.r.get(0) & (1 << offset)) != 0;
                }
                offset += 1;
            }
        }
        unreachable!()
    }
    pub fn extract_number(&self, l: &ColumnLabelList) -> String {
        assert!(l.count() > 0);
        let mut s = String::new();
        let mut col = l.count() - 1;
        loop {
            let cl = l.get(col);
            let v = self.extract_bit(cl);
            if v || s.len() > 0 {
                s.push_str(if v {"1"} else {"0"});
            }
            if col == 0 {
                break;
            }
            col -= 1;
        }
        if s.len() == 0 {
            s.push_str("0");
        }
        s
    }
    pub fn register_adder(&mut self, i: &ColumnLabelList) -> ColumnLabelList {
        assert!(i.count() == 3);
        let cl_c = self.get_new_column_label();
        let cl_s = self.get_new_column_label();
        let nl : NodeLabel = self.n.count() as NodeLabel;
        let mut ccl = EMPTY_COLUMNLABELLIST;
        ccl.add(cl_s);
        ccl.add(cl_c);
        for cl in &i.l {
            ccl.add(*cl);
        }
        let mut rel = EMPTY_ROWENTRYLIST;
        for e in 0..8 {
            let e16 = e as u16;
            let pc = e16.count_ones() as u16;
            rel.add((e16 << 2) | pc);
        }
        self.n.add(Node{
            n: nl,
            c: ccl,
            r: rel,
            a: true,
        });
        let mut ccl = EMPTY_COLUMNLABELLIST;
        ccl.add(cl_c);
        ccl.add(cl_s);
        ccl
    }
    pub fn add_numbers(&mut self, a: &ColumnLabelList, b: &ColumnLabelList) -> ColumnLabelList {
        assert!(a.count() > 0);
        assert!(b.count() > 0);
        let mut ia = 0;
        let mut ib = 0;
        let mut l = EMPTY_COLUMNLABELLIST;
        let mut c = FALSE_COLUMN;
        while ia < a.count() || ib < b.count() {
            let mut i = EMPTY_COLUMNLABELLIST;
            if ia < a.count() && ib < b.count() {
                i.add(a.get(ia));
                i.add(b.get(ib));
                ia += 1;
                ib += 1;
            } else if ia < a.count() {
                i.add(a.get(ia));
                i.add(FALSE_COLUMN);
                ia += 1;
            } else {
                assert!(ib < b.count());
                i.add(b.get(ib));
                i.add(FALSE_COLUMN);
                ib += 1;
            }
            i.add(c);
            let o = self.register_adder(&i);
            c = o.get(0);
            l.add(o.get(1));
        }
        l.add(c);
        l
    }
    pub fn register_and(&mut self, a: ColumnLabel, b: ColumnLabel) -> ColumnLabel {
        if a == b { return a; }
        if a == FALSE_COLUMN || b == FALSE_COLUMN { return FALSE_COLUMN; }
        if a == TRUE_COLUMN { return b; }
        if b == TRUE_COLUMN { return a; }
        let cl_o = self.get_new_column_label();
        let nl : NodeLabel = self.n.count() as NodeLabel;
        let mut ccl = EMPTY_COLUMNLABELLIST;
        ccl.add(cl_o);
        ccl.add(a);
        ccl.add(b);
        let mut rel = EMPTY_ROWENTRYLIST;
        for e in 0..4 {
            let e16 = e as u16;
            rel.add((e16 << 1) | ((e16 >> 1) & (e16 & 1)));
        }
        self.n.add(Node{
            n: nl,
            c: ccl,
            r: rel,
            a: true,
        });
        cl_o
    }
    pub fn register_switched_number(&mut self, i: &ColumnLabelList, s: ColumnLabel, o: usize) -> ColumnLabelList {
        let mut l = EMPTY_COLUMNLABELLIST;
        for _ in 0..o {
            l.add(FALSE_COLUMN);
        }
        for cl in &i.l {
            l.add(self.register_and(*cl, s));
        }
        l
    }
    pub fn mul_numbers(&mut self, a: &ColumnLabelList, b: &ColumnLabelList) -> ColumnLabelList {
        assert!(a.count() > 0);
        assert!(b.count() > 0);
        if a.count() < b.count() {
            return self.mul_numbers(b, a);
        }
        let mut list = Vec::new();
        let mut offset = 0;
        for cl in &b.l {
            list.push(self.register_switched_number(a, *cl, offset));
            offset += 1;
        }

        while list.len() > 1 {
            let mut list2 = Vec::new();
            let mut i = 0;
            while i < list.len() {
                if i + 2 <= list.len() {
                    let sum = self.add_numbers(list.get(i).unwrap(), list.get(i+1).unwrap());
                    list2.push(sum);
                    i += 2;
                } else {
                    list2.push(list.remove(i));
                }
            }
            list = list2;
        }

        assert!(list.len() == 1);
        list.remove(0)
    }
    /*pub fn force_smaller(&mut self, a: &ColumnLabelList, b: &ColumnLabelList) {
        todo!();
    }
    pub fn force_different(&mut self, a: &ColumnLabelList, b: &ColumnLabelList) {
        todo!();
    }*/
    pub fn rebuild_links(&mut self) {
        self.l = EMPTY_LINKLIST;
        for cl in 0..self.c {
            let mut link = EMPTY_LINK;
            link.c = cl;
            self.l.add(link);
        }
        for n in &self.n.l {
            for cl in &n.c.l {
                self.l.get_mut(*cl as usize).n.add(n.n);
            }
        }
    }
    pub fn collapse_fwd(&mut self) {
        let start = Instant::now();
        self.rebuild_links();
        let duration = start.elapsed();
        if duration > Duration::from_secs(1) {
            println!("slow rebuild_links(): {:?}", duration);
        }
        
        let mut changed = true;
        while changed {
            changed = false;
            for l in &self.l.l {
                let mut v = false;
                let mut found = false;
                for nl in &l.n.l {
                    let n = self.n.get(*nl as usize);
                    if n.r.count() == 1 {
                        let mut offset = 0;
                        for c in &n.c.l {
                            if *c == l.c {
                                v = (n.r.get(0) & (1 << offset)) != 0;
                                found = true;
                                break;
                            }
                            offset += 1;
                        }
                    }
                    if found { break; }
                }
                if !found {
                    continue;
                }
                for nl in &l.n.l {
                    let n = self.n.get_mut(*nl as usize);
                    if n.r.count() != 1 {
                        let mut offset = 0;
                        for c in &n.c.l {
                            if *c == l.c {
                                break;
                            }
                            offset += 1;
                        }
                        assert!(offset < n.c.count());

                        let mut row = 0;
                        while row < n.r.count() {
                            let v2 = (n.r.get(row) & (1 << offset)) != 0;
                            if v != v2 {
                                n.r.remove(row);
                                changed = true;
                            } else {
                                row += 1;
                            }
                        }
                    }
                }
            }
        }
    }
}
