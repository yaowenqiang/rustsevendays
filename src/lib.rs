//! A library for handling money

use std::ops::AddAssign;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt;
use std::str::FromStr;
use self::LinkedList::*;
use std::rc::Rc;
use std::cell::RefCell;


pub mod parse;
use parse::*;

static N:i32 = 55;
static mut M:i32 = 55;

pub fn make_rc(i:i32) -> Rc<RefCell<i32>> {
    let a = Rc::new(RefCell::new(i));
    let b = a.clone();
    let m = &mut *a.borrow_mut();
    *m += 2;
    return b;
}

pub fn add_stat(n:i32) -> i32 {
    unsafe {
        M += n;
        M
    }
}

pub fn get_st() -> &'static i32 {
    &N
}
pub fn stat_str() -> &'static str {
    "hello"
}


#[derive(PartialEq, Debug)]
pub struct USD (i32);

#[derive(PartialEq, Debug)]
pub enum LinkedList<T> {
    Tail,
    Head(T, Box<LinkedList<T>>)
}

impl<T> LinkedList<T> {
    pub fn empty() -> Self {
        Tail
    }
    pub fn new(t:T) -> Self {
        Head(t, Box::new(Tail))
    }

    pub fn push(self, t:T) -> Self {
        Head(t, Box::new(self))
    }
    pub fn push_back(&mut self, t:T) {
        match self {
            Tail => {
                *self = LinkedList::new(t);
            },
            Head(_,n) => {
                n.push_back(t);
            }
        }
    }
}

impl Display for USD {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let r = (self.0 as f32) / 100.;
        if r < 0. {
            write!(f, "-$ {:.2}", -r);
        }
        write!(f, "${:.2}", r)
    }
}

pub fn trim_left<'a>(s:&'a str) ->&'a str {
    for (i, c) in s.char_indices() {
        if c == ' ' {
            continue;
        }
        return s.get(i..s.len()).unwrap();
    }
    ""
}

impl Clone for USD {
    fn clone(&self)->USD {
        USD(self.0)
    }
}

impl Copy for USD {
}

/// Parse your money from a string
/// ```
/// use sevenday::*;
/// let g = "£32.45".parse();
/// assert_eq!(g, Ok(GBP(3245)));
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct GBP (i32);

/*
fn money_pointer<'a>(i:i32) -> &'a GBP {
    let g = GBP(i);
    &g
}
*/

fn on_money(a:i32, b:i32) -> GBP {
    let mut g = GBP(a);
    let r;
    {
        r = &g;
        g.0 += 2;
    }
    let res = GBP(r.0 + b);
    res
}

impl FromStr for GBP {
    //type Err = ParseMoneyError;
    type Err = GBPError;
    fn from_str(s:&str) -> Result<Self, Self::Err> {
        Ok(GBP(parse_sym_money(s,'£', 2)?))
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Stepper<T> {
    curr:T,
    step: T,
    stop: T,
}

impl<T>Stepper<T> {
    pub fn new(start: T, stop: T, step:T) -> Self {
        Stepper{
            curr: start,
            stop: stop,
            step: step,
        }
    }
}
fn sum_list<I, S>(i:I, mut s:S) -> S 
    //where I: Iterator<Item=S>,
    where I: IntoIterator<Item=S>,
          S: AddAssign
    {
        /*
    for n in i {
        s += n;
    }
    s
    */
    let mut it = i.into_iter();
    while let Some(n) = it.next() {
        s += n;
    }
    s
}

impl<T> Iterator for Stepper<T>
where T:AddAssign + Copy + PartialOrd {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.curr >= self.stop {
            return None;
        }
        println!("step next");
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}



#[derive(PartialEq, Debug, Clone)]
pub struct CAD (i32);
pub trait ToUSDv<F>{
    fn to_uv(&self, g:F)->f32;
}
pub trait FromUSDv<F>{
    fn from_uv(&self, f:f32)->F;
}
impl Account for Ex {
    fn id(&self) -> i32 {self.ac_id}
}

pub struct Ex {
    ac_id: i32,
    cad: f32,
    gbp: f32,
}
#[derive(PartialEq, Debug)]
pub struct Transaction<A> {
    from_id:i32,
    to_id:i32,
    amount:A,
}


#[derive(PartialEq, Debug)]
pub enum GBPError {
    ParseError(ParseMoneyError),
    OtherError,
}


impl From<ParseMoneyError> for GBPError {
    fn from(p:ParseMoneyError) -> Self {
        GBPError::ParseError(p)
    }
}

pub trait Account {
    fn id(&self) -> i32;
}

pub trait Exchange<F, T>{
    fn convert(&self, f:F) -> T;
}

impl<E, F, T> Exchange< F, T> for E 
    where E:ToUSDv<F> + FromUSDv<T>
{
    fn convert(&self, f:F) -> T {
        self.from_uv(self.to_uv(f))
    }
}
impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, g:GBP) -> f32 {
        g.0 as f32 * self.gbp
    }
}
impl FromUSDv<CAD> for Ex {
    fn from_uv(&self, f:f32) -> CAD {
        CAD((f / self.cad) as i32)
    }
}

pub trait ToUSD {
    fn to_usd(&self) -> USD; 
    fn convert<T:FromUSD>(&self) -> T {
        T::from_usd(&self.to_usd())
    }
}

pub trait FromUSD {
    fn from_usd(u:&USD) -> Self; 
}
impl ToUSD for GBP {
    fn to_usd(&self)->USD{
       USD(self.0 * 130 / 100) 
    }
}

impl FromUSD for CAD {
    fn from_usd(u: &USD)->Self{
       CAD(u.0 * 130 / 100) 
    }
}

pub trait ExchangeAccount<F, T> {
    fn exchange(&self, f_id:i32, t_id:i32, amount:F)
    -> (Transaction<F>, Transaction<T>);
}

impl<E, F, T> ExchangeAccount<F, T> for E
where E:Exchange<F, T> + Account, 
      F:Clone,
{
    fn exchange(&self, f_id:i32, t_id:i32, amount:F)
    ->(Transaction<F>, Transaction<T>) {
        let ft = Transaction{from_id: f_id, to_id:self.id(), amount:amount.clone()};
        let tt = Transaction{from_id: self.id(), to_id:t_id, amount:self.convert(amount)};
        (ft, tt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]


    fn it_works() {
        let r = make_rc(5);
        assert_eq!(*r.borrow(), 7);


        assert_eq!(*get_st(), 55);
        assert_eq!(stat_str(), "hello");
        assert_eq!(add_stat(5), 60);

        let mut l = LinkedList::new(3);
        l = l.push(4);
        assert_eq!(l, Head(4, Box::new(Head(3, Box::new(Tail)))));
        l.push_back(2);
        assert_eq!(l, Head(4, Box::new(Head(3, Box::new(Head(2, Box::new(Tail)))))));

        //let g = money_pointer(3);
        assert_eq!(trim_left("     hello"), "hello");
        let mut s = " .   hello".to_string();
        {
            let s2 = trim_left(&s);
            assert_eq!(s2, "hello");
        }
        s.push_str(" world");
        assert_eq!(trim_left(&s), "hello world");
        let g = on_money(1, 2);
        assert_eq!(g, GBP(3));
        let u = USD(230);
        assert_eq!(u.to_string(), "$2.30".to_string());

        //let b = u.clone();
        let b = u;
        assert_eq!(u, b);

        let mut c = 0;
        for n in Stepper::new(2, 10, 2) {
            c += n;
        }
        assert_eq!(c, 20);


        let sl = sum_list(Stepper::new(3, 10, 2), 0);
        assert_eq!(sl, 24);

        let fl = Stepper::new(4, 10, 2).fold(0, | acc, x | acc + x);
        assert_eq!(fl, 18);

        let g = GBP(200);
        let u = g.to_usd();
        assert_eq!(u, USD(260));
        let c = CAD::from_usd(&u);
        assert_eq!(c, CAD(338));

        let c2:CAD = g.convert();
        assert_eq!(c2, c);

        let g = GBP(200);
        let ex = Ex{ac_id:0,cad: 0.7, gbp: 1.3};
        let c = ex.from_uv(ex.to_uv(g));
        assert_eq!(c, CAD(371));

        let g2 = GBP(200);
        let d:CAD = ex.convert(g2);
        assert_eq!(d, CAD(371));

        let ex2 = Ex{ac_id:30, cad:0.7, gbp:1.3};
        let (ft, tt) = ex2.exchange(20, 40, GBP(200));
        assert_eq!(ft, Transaction{from_id:20, to_id:30, amount:GBP(200)});
        assert_eq!(tt, Transaction{from_id:30, to_id:40, amount:CAD(371)});

    }
}