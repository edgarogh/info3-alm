use std::str::FromStr;

macro_rules! default {
    ($enum:ident :: $variant:ident) => {
        impl ::std::default::Default for $enum {
            fn default() -> Self {
                $enum::$variant
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ExpressionType {
    Eq,
    Gt,
    Lt,
    Ge,
    Le,
}

impl ExpressionType {
    pub fn flags(self) -> Option<&'static str> {
        match self {
            Self::Eq => None,
            Self::Gt => Some("!C | Z"),
	    Self::Lt => Some("!C"),
	    Self::Ge => Some("C"),
            Self::Le => Some("C & !Z"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Sub,
    Decr,
    Incr,
    Mul2,
    Div2,
}

default!(Operator::Sub);

impl Operator {
    pub fn codes(self) -> &'static [&'static str] {
        match self {
            Self::Sub => &[],
            Self::Incr => &["uc0"],
            Self::Decr => &["uc1"],
            Self::Mul2 => &["uc0", "uc1", "lsl"],
            Self::Div2 => &["uc0", "uc1"],
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Assignee {
    Q,
    R,
    A,
    B,
    K,
}

impl FromStr for Assignee {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "q" => Ok(Self::Q),
            "r" => Ok(Self::R),
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "k" => Ok(Self::K),
            _ => Err("cannot assign to unknown register"),
        }
    }
}

impl Assignee {
    pub fn codes(self) -> &'static [&'static str] {
        match self {
            Self::Q => &["chQ"],
            Self::R => &["chR"],
            Self::A => &["chA"],
            Self::B => &["chB"],
            Self::K => &["chK"],
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operand1 {
    Q,
    R,
    B,
    K,
}

impl FromStr for Operand1 {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "q" => Ok(Self::Q),
            "r" => Ok(Self::R),
            "b" => Ok(Self::B),
            "k" => Ok(Self::K),
            _ => Err(()),
        }
    }
}

impl Operand1 {
    pub fn codes(self) -> &'static [&'static str] {
        match self {
            Self::Q => &[],
            Self::R => &["var2"],
            Self::B => &["var1"],
            Self::K => &["var1", "var2"],
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operand2 {
    A,
    B,
}

default!(Operand2::B);

impl FromStr for Operand2 {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            _ => Err(()),
        }
    }
}

impl Operand2 {
    pub fn codes(self) -> &'static [&'static str] {
        match self {
            Self::A => &["var3"],
            Self::B => &[],
        }
    }
}

trait ParseChar<T: FromStr> {
    fn parse(self) -> Result<T, T::Err>;
}

impl<T: FromStr> ParseChar<T> for char {
    fn parse(self) -> Result<T, T::Err> {
        let mut char = [0u8; 4];
        self.encode_utf8(&mut char).parse()
    }
}

fn main() {
    let tokens = std::env::args()
	.skip(1)
	.collect::<String>()
	.chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<Vec<_>>();
        
    let typ = tokens.get(1).expect("expected 2nd token to be an = or comparison symbol");
    
    let mut outputs = Vec::<&'static str>::new();
    
    let typ = match typ {
        '=' => ExpressionType::Eq,
        '≤' => ExpressionType::Le,
	'≥' => ExpressionType::Ge,
	'<' => ExpressionType::Lt,
        '>' => ExpressionType::Gt,
        c => unimplemented!("unknown operator \"{}\"", c),
    };
    
    let (op, operand1, operand2): (_, Operand1, Operand2) = if !matches!(typ, ExpressionType::Eq) {
        assert_eq!(tokens.len(), 3, "too many tokens, expected 3");

	let operand1 = tokens[0].parse().unwrap();
	let operand2 = tokens[2];
	if operand2 == '0' {
            (Operator::Div2, operand1, Default::default())
	} else {
	    (Operator::Sub, operand1, operand2.parse().unwrap())
    	}
    } else {
        outputs.extend(ParseChar::<Assignee>::parse(tokens[0]).unwrap().codes());
    
        let operand1 = tokens[2];
        let operand2 = tokens[4];
    
        match tokens.get(3).expect("expected an operator in position 4") {
            '-' if operand2 == '1' => (
                Operator::Decr,
                operand1.parse().unwrap(),
                Default::default(),
            ),
            '-' => (
                Operator::Sub,
                operand1.parse().unwrap(),
                operand2.parse().unwrap()
            ),
            '+' => (
                Operator::Incr,
                operand1.parse().unwrap(),
                { assert_eq!(operand2, '1'); Default::default() },
            ),
            '/' => (
                Operator::Div2,
                operand1.parse().unwrap(),
                { assert_eq!(operand2, '2'); Default::default() },
            ),
            '*' => (
                Operator::Mul2,
                operand1.parse().unwrap(),
                { assert_eq!(operand2, '2'); Default::default() },
            ),
            c => unimplemented!("unknown operator \"{}\"", c),
        }
    };
    
    outputs.extend(op.codes());
    outputs.extend(operand1.codes());
    outputs.extend(operand2.codes());
    
    outputs.sort();
    
    println!("outputs: {}", outputs.join(", "));
    
    if let Some(flags) = typ.flags() {
        println!(" inputs: {}", flags);
    }
}

