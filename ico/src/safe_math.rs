use cosmwasm_std::{StdError, generic_err};

macro_rules! add {
    ($x: expr, $y: expr) => {
        match $x+$y {
            c if c > $x => {
                Ok(c)
            },
            _ => {
                Err(StdError::generic_err(format!("overflow: x: {}, y: {} x+y: {}", $x, $y, $x+$y)))
            }
        }
    };
}

macro_rules! sub {
    ($x: expr, $y: expr) => {
        match $x-$y {
            c if c < $x => {
                Ok(c)
            },
            _ => {
                Err(StdError::generic_err(format!("underflow: x: {}, y: {} x+y: {}", $x, $y, $x-$y)))
            }
        }
    };
}

macro_rules! mul {
    ($x: expr, $y: expr) => {
        match $x*$y {
            c if c / $x == $y => {
                Ok(c)
            },
            _ => {
                Err(StdError::generic_err(format!("multiplication overflow: x: {}, y: {} x*y: {}", $x, $y, $x*$y)))
            }
        }
    };
}

macro_rules! div {
    ($x: expr, $y: expr) => {
        match $y {
            b if b > 0 => {
                Ok($x/$y)
            },
            _ => {
                Err(StdError::generic_err(format!("divided by zero: y: {}", $y)))
            }
        }
    };
}