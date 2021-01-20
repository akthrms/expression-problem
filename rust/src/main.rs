// Initial System

trait Exp {
    fn eval(self) -> i64;
}

struct Lit {
    x: i64,
}

struct Add<E> {
    e1: E,
    e2: E,
}

impl Exp for Lit {
    fn eval(self) -> i64 {
        self.x
    }
}

impl<E: Exp> Exp for Add<E> {
    fn eval(self) -> i64 {
        self.e1.eval() + self.e2.eval()
    }
}

// Adding a New Variant

struct Sub<E> {
    e1: E,
    e2: E,
}

impl<E: Exp> Exp for Sub<E> {
    fn eval(self) -> i64 {
        self.e1.eval() - self.e2.eval()
    }
}

// Adding a New Operation

trait ExpP {
    fn print(&self) -> String;
}

impl ExpP for Lit {
    fn print(&self) -> String {
        self.x.to_string()
    }
}

impl<E: ExpP> ExpP for Add<E> {
    fn print(&self) -> String {
        format!("{} + {}", self.e1.print(), self.e2.print())
    }
}

fn main() {
    let sub = Sub {
        e1: Lit { x: 3 },
        e2: Lit { x: 2 },
    };
    assert_eq!(sub.eval(), 1);

    let add_p = Add {
        e1: Lit { x: 1 },
        e2: Lit { x: 2 },
    };
    assert_eq!(add_p.print(), "1 + 2".to_string());
}
