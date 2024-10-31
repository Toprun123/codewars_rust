use itertools::Itertools;
use num::bigint::BigInt;
use num::traits::Num;
use num::ToPrimitive;

fn main() {
    println!("Hello, world!");
    assert_eq!(multiplication_table(3), [[1, 2, 3], [2, 4, 6], [3, 6, 9]]);
    assert_eq!(solution(10), 23);
    assert_eq!(find_odd(&vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 10, 10]), 1);
    assert_eq!(
        good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"),
        "Battle Result: Good triumphs over Evil"
    );
    assert_eq!(choose_best_sum(163, 3, &vec![50, 55, 56, 57, 58]), 163);
    assert_eq!(find_outlier(&[2, 3, 4, 6, 8]), 3);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376",
                "2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(num_as_roman(1990), "MCMXC");
    assert_eq!(meeting("Alex:Arno;Alissa:Cornwell;Sarah:Bell;Andrew:Dorries;Ann:Kern;Haley:Arno;Paul:Dorny;Madison:Kern"),
                "(ARNO, ALEX)(ARNO, HALEY)(BELL, SARAH)(CORNWELL, ALISSA)(DORNY, PAUL)(DORRIES, ANDREW)(KERN, ANN)(KERN, MADISON)");
    assert_eq!(sum_intervals(&[(1, 20), (2, 19), (5, 15), (8, 12)]), 19);
    assert_eq!(list_squared(1, 250), vec![(1, 1), (42, 2500), (246, 84100)]);
    assert_eq!(mix("aaa bb", "a bbb"), "1:aaa/2:bbb");
    println!("All tests passed successfully!!")
}

fn mix(s1: &str, s2: &str) -> String {
    let mut r: Vec<String> = vec![];
    for i in 97..123 as u8 {
        let c: char = i as char;
        let x: usize = s1.matches(c).count();
        let y: usize = s2.matches(c).count();
        if x > 1 || y > 1 {
            r.push(if x > y {
                ["1:", c.to_string().repeat(x).as_str(), "/"].concat()
            } else if y > x {
                ["2:", c.to_string().repeat(y).as_str(), "/"].concat()
            } else {
                ["=:", c.to_string().repeat(x).as_str(), "/"].concat()
            })
        }
    }
    r.sort_by(|a: &String, b: &String| a.cmp(&b));
    r.sort_by(|a: &String, b: &String| b.len().cmp(&a.len()));
    let mut x: String = r.into_iter().join("");
    x.pop();
    x
}

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    let mut r: Vec<(u64, u64)> = vec![];
    for i in m..=n {
        let s: u64 = s_sd(i);
        if i_ps(s) {
            r.push((i, s));
        }
    }
    r
}
fn s_sd(n: u64) -> u64 {
    let mut s: u64 = 0;
    for i in 1..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            s += i * i;
            if i != n / i {
                s += (n / i) * (n / i);
            }
        }
    }
    s
}
fn i_ps(n: u64) -> bool {
    let r = (n as f64).sqrt() as u64;
    r * r == n
}

fn mi(i: &[(i32, i32)]) -> Vec<(i32, i32)> {
    if i.is_empty() {
        return vec![];
    }
    let mut i: Vec<(i32, i32)> = i.to_vec();
    i.sort_by(|a: &(i32, i32), b: &(i32, i32)| a.0.cmp(&b.0));
    let mut me: Vec<(i32, i32)> = vec![i[0]];
    for &(s, e) in &i[1..] {
        let l: &mut (i32, i32) = me.last_mut().unwrap();
        if s <= l.1 {
            l.1 = l.1.max(e);
        } else {
            me.push((s, e));
        }
    }
    me
}

fn sum_intervals(i: &[(i32, i32)]) -> i32 {
    let mi: Vec<(i32, i32)> = mi(i);
    mi.iter().map(|&(s, e)| e - s).sum()
}

fn meeting(s: &str) -> String {
    let s: String = s.to_uppercase();
    let mut r: String = String::new();
    let mut p: Vec<Vec<&str>> = s
        .split(";")
        .map(|x: &str| x.split(":").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    p.sort_by(|a: &Vec<&str>, b: &Vec<&str>| a[0].cmp(b[0]));
    p.sort_by(|a: &Vec<&str>, b: &Vec<&str>| a[1].cmp(b[1]));
    for i in p {
        r.push_str(format!("({}, {})", i[1], i[0]).as_str());
    }
    r
}

fn num_as_roman(num: i32) -> String {
    let rome: Vec<&str> = Vec::from([
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ]);
    let byzantium: Vec<i32> = Vec::from([1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1]);
    let mut x: i32 = num;
    let mut roman: String = String::new();
    let mut i: usize = 0;
    while x > 0 {
        while x >= byzantium[i] {
            x -= byzantium[i];
            roman.push_str(rome[i]);
        }
        i += 1;
    }
    roman
}

fn last_digit(a: &str, b: &str) -> i32 {
    match b.parse::<i32>() {
        Ok(b) => {
            if b == 0 {
                1
            } else {
                ca(a, &b.to_string())
            }
        }
        Err(_) => ca(a, b),
    }
}

fn ca(a: &str, b: &str) -> i32 {
    let b: BigInt = BigInt::from_str_radix(b, 10).expect("Error");
    let d: BigInt = BigInt::from_str_radix(a, 10).expect("Error") % BigInt::from(10);
    let mut c: Vec<BigInt> = vec![];
    let mut r: BigInt = d.clone();
    while !c.contains(&r) {
        c.push(r.clone());
        r = (r * d.clone()) % BigInt::from(10);
    }
    let c_len: BigInt = BigInt::from(c.len());
    let mut p: BigInt = (b.clone() % c_len.clone()) - BigInt::from(1);
    if p == BigInt::from(-1) {
        p = c_len.clone() - BigInt::from(1)
    }
    c[p.to_usize().expect("Error")]
        .clone()
        .to_i32()
        .expect("Error")
}

fn find_odd(arr: &[i32]) -> i32 {
    for e in arr.iter() {
        if arr.iter().filter(|&x| x == e).count() % 2 == 1 {
            return e.clone();
        }
    }
    -1
}

fn find_outlier(v: &[i32]) -> i32 {
    let t: i32 = (if v[0] % 2 == v[1] % 2 {
        v[0] % 2
    } else {
        v[2] % 2
    })
    .abs();
    for i in v {
        if (i % 2).abs() != t {
            return i.clone();
        }
    }
    0
}

fn solution(num: i32) -> i32 {
    let mut x: i32 = 0;
    for i in 0..num {
        if i % 3 == 0 || i % 5 == 0 {
            x += i;
        }
    }
    x
}

fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();
    for i in 1..len + 1 {
        let mut row: Vec<usize> = Vec::new();
        for j in (i..i * len + 1 as usize).step_by(i) {
            row.push(j);
        }
        result.push(row);
    }
    result
}

fn good_vs_evil(g: &str, b: &str) -> String {
    let gw: [i32; 6] = [1, 2, 3, 3, 4, 10];
    let bw: [i32; 7] = [1, 2, 2, 2, 3, 5, 10];
    let mut gs: i32 = 0;
    let mut es: i32 = 0;
    for (i, s) in g.split(' ').enumerate() {
        gs += gw[i] * s.parse::<i32>().unwrap();
    }
    for (i, s) in b.split(' ').enumerate() {
        es += bw[i] * s.parse::<i32>().unwrap();
    }
    if gs > es {
        "Battle Result: Good triumphs over Evil".to_string()
    } else if es > gs {
        "Battle Result: Evil eradicates all trace of Good".to_string()
    } else {
        "Battle Result: No victor on this battle field".to_string()
    }
}

fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    let v: Vec<Vec<i32>> = ls.clone().into_iter().combinations(k as usize).collect();
    let v = v.iter().map(|e: &Vec<i32>| e.iter().sum::<i32>());
    let mut max: i32 = -1;
    for e in v {
        if e > max && e <= t {
            max = e;
        }
    }
    max
}
