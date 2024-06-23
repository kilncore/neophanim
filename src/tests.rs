use crate::*;
use rug::Integer;

// cargo test --release -p neophanim -- --nocapture --test-threads=1

fn dump_graph(g: &Graph) {
    let mut i = 0;
    while i < g.n.count() {
        println!("node {}", i);
        let n = g.n.get(i);

        print!("\tcl:");
        for cl in &n.c.l {
            print!(" {}", cl);
        }
        println!("");

        println!("\tre:");
        for re in &n.r.l {
            println!("\t\t{:016b}", re);
        }
        println!("");

        i += 1;
    }
}

fn parse_decimal(s: &str) -> Integer {
    Integer::from_str_radix(s, 10).unwrap()
}

fn test_pair_fwd(a: &str, b: &str) {
    let a = parse_decimal(a);
    let a_2 = a.to_string_radix(2);

    let b = parse_decimal(b);
    let b_2 = b.to_string_radix(2);

    let sum = a.clone() + b.clone();
    let sum_2 = sum.to_string_radix(2);

    let mul = a * b;
    let mul_2 = mul.to_string_radix(2);

    let mut g = EMPTY_GRAPH;

    let g_a = g.register_number(a_2.clone());
    assert_eq!(g.extract_number(&g_a), a_2);

    let g_b = g.register_number(b_2.clone());
    assert_eq!(g.extract_number(&g_b), b_2);

    let g_sum = g.add_numbers(&g_a, &g_b);

    let debug = false;
    if debug {
        print!("g_sum:");
        for cl in &g_sum.l {
            print!(" {}", cl);
        }
        println!();
        
        println!("before collapse:");
        dump_graph(&g);
    }

    g.collapse_fwd();

    if debug {
        println!("after collapse:");
        dump_graph(&g);
    }

    assert_eq!(g.extract_number(&g_sum), sum_2);

    let g_mul = g.mul_numbers(&g_a, &g_b);
    g.collapse_fwd();
    assert_eq!(g.extract_number(&g_mul), mul_2);
}


#[test]
fn test_mini_fwd() {
    test_pair_fwd("2", "3"); 
    test_pair_fwd("3", "5");
    test_pair_fwd("5", "7");
    test_pair_fwd("11", "23");
    test_pair_fwd("73", "97");
}

#[test]
fn test_rsa100_fwd() {
    test_pair_fwd(  "37975227936943673922808872755445627854565536638199",
                    "40094690950920881030683735292761468389214899724061");
}

#[test]
fn test_rsa170_fwd() {
    test_pair_fwd(  "3586420730428501486799804587268520423291459681059978161140231860633948450858040593963",
                    "7267029064107019078863797763923946264136137803856996670313708936002281582249587494493");
}

#[test]
fn test_rsa250_fwd() {
    test_pair_fwd(  "64135289477071580278790190170577389084825014742943447208116859632024532344630238623598752668347708737661925585694639798853367",
                    "33372027594978156556226010605355114227940760344767554666784520987023841729210037080257448673296881877565718986258036932062711");
}


