// Initial System

interface Exp {
  eval(): number;
}

class Lit implements Exp {
  constructor(protected x: number) {}
  eval() {
    return this.x;
  }
}

class Add<E extends Exp> implements Exp {
  constructor(protected e1: E, protected e2: E) {}
  eval() {
    return this.e1.eval() + this.e2.eval();
  }
}

// Adding a New Variant

class Sub<E extends Exp> implements Exp {
  constructor(protected e1: E, protected e2: E) {}
  eval() {
    return this.e1.eval() - this.e2.eval();
  }
}

// Adding a New Operation

interface ExpP extends Exp {
  print(): string;
}

class LitP extends Lit implements ExpP {
  print() {
    return this.x.toString();
  }
}

class AddP<E extends ExpP> extends Add<E> implements ExpP {
  print() {
    return this.e1.print() + " + " + this.e2.print();
  }
}

const sub = new Sub(new Lit(3), new Lit(2));
console.assert(sub.eval() === 1);

const addP = new AddP(new LitP(1), new LitP(2));
console.assert(addP.print() === "1 + 2");
