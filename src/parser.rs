use crate::lexer::Lexeme;

pub trait Parse {
    fn parse(l: &[Lexeme]) -> Option<(Self,usize)> where Self: Sized;
}

pub trait Compute {
    fn compute(&self, def: f32) -> f32;
}


#[derive(Clone)]
pub enum Expr {
    Add(AddExpr),
    AddEnd(AddExprEnd),
    MultEnd(ProdExprEnd),
}

impl Parse for Expr {
    fn parse(l: &[Lexeme]) -> Option<(Self,usize)> {
        if let Some((add, n)) = AddExpr::parse(l) {
            return Some((Self::Add(add), n));
        }

        if let Some((end, n)) = AddExprEnd::parse(l) {
            return Some((Self::AddEnd(end), n));

        }

        if let Some((end, n)) = ProdExprEnd::parse(l) {
            return Some((Self::MultEnd(end), n));
        }

        None
    }
}

impl Compute for Expr {
     fn compute(&self, def: f32) -> f32 {
         match self {
             Self::Add(add) => add.compute(def),
             Self::AddEnd(add) => def + add.compute(def),
             Self::MultEnd(mult) => def * mult.compute(def)
         }
     }
}

#[derive(Clone)]
pub struct AddExpr(ProdExpr, Option<AddExprEnd>);

impl Parse for AddExpr {
    fn parse(l: &[Lexeme]) -> Option<(Self,usize)> {
        let mut tokens = 0;

        if let Some((prod, n)) = ProdExpr::parse(l) {
            tokens += n;

            if let Some ((end, n)) = AddExprEnd::parse(&l[tokens..]) {
                tokens += n;
                Some((Self(prod, Some(end)), tokens))
            } else {
                Some((Self(prod, None), tokens))
            }
        } else {
            None
        }
    }
}

impl Compute for AddExpr {
    fn compute(&self, def: f32) -> f32 {
        self.0.compute(def) + self.1.clone().map(|e| e.compute(def)).unwrap_or(0.0)
    }
}

#[derive(Clone)]
pub enum AddExprEnd {
    Add(ProdExpr, Option<Box<AddExprEnd>>),
    Sub(ProdExpr, Option<Box<AddExprEnd>>),
}

impl Parse for AddExprEnd {
    fn parse(l: &[Lexeme]) -> Option<(Self,usize)> {
        let mut tokens = 0;

        let next = l.get(tokens);

        if Some(&Lexeme::Plus) == next || Some(&Lexeme::Minus) == next {
            tokens += 1;

            if let Some((base, n)) = ProdExpr::parse(&l[tokens..]) {
                tokens += n;
                let (end, n) = AddExprEnd::parse(&l[tokens..]).map(|(e, n)| {
                    (Some(Box::new(e)), n)
                }).unwrap_or((None, 0));
                tokens += n;

                return match next {
                    Some(Lexeme::Plus) => Some((Self::Add(base, end), tokens)),
                    Some(Lexeme::Minus) => Some((Self::Sub(base, end), tokens)),
                    _ => unreachable!()

                };
            }
        }

        None
    }
}

impl Compute for AddExprEnd {
    fn compute(&self, def: f32) -> f32 {
        match self {
            Self::Add(prod, end) => prod.compute(def) + end.clone().map(|e| e.compute(def)).unwrap_or(0.0),
            Self::Sub(prod, end) => -prod.compute(def) + end.clone().map(|e| e.compute(def)).unwrap_or(0.0),
        }
    }
}

#[derive(Clone)]
pub struct ProdExpr(BaseExpr, Option<ProdExprEnd>);

impl Parse for ProdExpr {
    fn parse(l: &[Lexeme]) -> Option<(Self,usize)> {
        let mut tokens = 0;

        if let Some((base, n)) = BaseExpr::parse(l) {
            tokens += n;

            if let Some ((end, n)) = ProdExprEnd::parse(&l[tokens..]) {
                tokens += n;
                Some((Self(base, Some(end)), tokens))
            } else {
                Some((Self(base, None), tokens))
            }
        } else {
            None
        }
    }
}

impl Compute for ProdExpr {
    fn compute(&self, def: f32) -> f32 {
        let base = self.0.compute(def);
        base * self.1.clone().map(|e| e.compute(def)).unwrap_or(1.0)
    }
}

#[derive(Clone)]
pub enum ProdExprEnd {
    Mult(BaseExpr, Option<Box<ProdExprEnd>>),
    Div(BaseExpr, Option<Box<ProdExprEnd>>),
}

impl Parse for ProdExprEnd {
    fn parse(l: &[Lexeme]) -> Option<(Self,usize)> {
        let mut tokens = 0;

        let next = l.get(tokens);

        if Some(&Lexeme::Times) == next || Some(&Lexeme::Divide) == next {
            tokens += 1;

            if let Some((base, n)) = BaseExpr::parse(&l[tokens..]) {
                tokens += n;
                let (end, n) = ProdExprEnd::parse(&l[tokens..]).map(|(e, n)| {
                    (Some(Box::new(e)), n)
                }).unwrap_or((None, 0));
                tokens += n;

                return match next {
                    Some(Lexeme::Times) => Some((Self::Mult(base, end), tokens)),
                    Some(Lexeme::Divide) => Some((Self::Div(base, end), tokens)),
                    _ => unreachable!()

                };
            }
        }

        None
    }
}

impl Compute for ProdExprEnd {
    fn compute(&self, def: f32) -> f32 {
        match self {
            Self::Mult(base, end) => base.compute(def) * end.clone().map(|e| e.compute(def)).unwrap_or(1.0),
            Self::Div(base, end) => 1.0 / base.compute(def) * end.clone().map(|e| e.compute(def)).unwrap_or(1.0),
        }
    }
}

#[derive(Clone)]
pub enum BaseExpr {
    Paren(Box<Expr>),
    Num(f32)
}

impl Parse for BaseExpr {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> {
        let mut tokens = 0;

        if let Some(Lexeme::LeftParen) = l.get(tokens) {
            tokens += 1;
            if let Some((expr, n)) = Expr::parse(&l[tokens..]) {
                tokens += n;

                if let Some(Lexeme::RightParen) = l.get(tokens) {
                    tokens += 1;
                    return Some((Self::Paren(Box::new(expr)), tokens));
                }

                tokens -= n;
            }

            tokens -= 1;
        }

        if let Some(Lexeme::Num(n)) = l.get(tokens) {
            tokens += 1;
            return Some((Self::Num(*n), tokens))
        }

        None
    }
}

impl Compute for BaseExpr {
    fn compute(&self, def: f32) -> f32 {
        match self {
            Self::Num(n) => *n,
            Self::Paren(expr) => expr.compute(def)
        }
    }
}
