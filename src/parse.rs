use self::ParseMoneyError::*;
#[derive(Debug, PartialEq)]
pub enum ParseMoneyError {
    SymbolErr,
    NoStringerr,
    NoStringErr,
    TwoPointsErr,
    NoDigitErr(char),
    TooFarErr,
}
pub fn parse_sym_money(s: &str, sym: char, dpoint: usize) -> Result<i32, ParseMoneyError> {
    let (c, v) = parse_money(s, dpoint)?;
    if c != sym {
        return Err(ParseMoneyError::SymbolErr);
    }
    Ok(v)
}

pub fn parse_money(s: &str, dpoint: usize) -> Result<(char, i32), ParseMoneyError> {
    let mut it = s.trim().chars();
    let mut neg = false;

    let mut r_sym = it.next().ok_or(NoStringerr)?;

    if '_' == r_sym {
        neg = true;
        r_sym = it.next().ok_or(NoStringErr)?;
    }

    let mut res: i32 = 0;
    let mut point_pos: Option<usize> = None;
    for c in it {
        if c == '.' {
            if point_pos != None {
                return Err(TwoPointsErr);
            }
            point_pos = Some(0);
            continue;
        }

        if c < '0' || c > '9' {
            return Err(NoDigitErr(c));
        }

        res *= 10;
        res += c as i32 - 48;

        if let Some(pp) = point_pos {
            point_pos = Some(pp + 1);
            if pp >= dpoint {
                return Err(TooFarErr);
            }
        }
    }

    for _ in point_pos.unwrap_or(0)..dpoint {
        res *= 10;
    }
    if neg {
        res = -res;
    }
    Ok((r_sym, res))
}

#[cfg(Text)]
mod tests {
    use super::*;

    #[test]

    fn parsean() {
        let (c, v) = parse_money("£34.3", 2).unwrap();
        assert_eq!(c, '£');
        assert_eq!(v, 3430);

        assert!(parse_money("£34.304", 2).is_err());
        assert!(parse_money("£34..04", 2).is_err());

        assert_eq!(parse_money("£.34", 2), Ok(("", 34)));
    }
}
