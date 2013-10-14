use std::num::abs;

enum Figura {
  Circulo(Pnt, float),
  Rectangulo(Pnt, Dim),
  Triangulo(Pnt, Dim)
}
struct Pnt {
  x: float,
  y: float
}
struct Dim {
  w: float,
  h: float
}

impl Figura {
  fn area(&self) -> float {
    match *self {
      Circulo(_, f) => (3.1416 * f * f),
      Rectangulo(_, ref dim) => dim.w * dim.h,
      Triangulo(_, ref dim) => (dim.w * dim.h) / 2.0
    }
  }
  // pretty sure that there is a better way to do this
  fn point(&self) -> Pnt {
    match *self {
      Circulo(ref point,_) => *point,
      Rectangulo(ref point,_) => *point,
      Triangulo(ref point,_) => *point
    }
  }
}

fn distance(figa: Figura, figb: Figura) -> float {
  let a =  figa.point();
  let b =  figb.point();

  abs(a.x - b.x)
}


fn main() {
  let rct = Rectangulo(Pnt {x: 4.0, y: 2f}, Dim {w: 10.0, h: 5.0});
  let trngl = Triangulo(Pnt {x: 15.0, y: 2f}, Dim {w: 5.0, h: 3.0});
  let crcl = Circulo(Pnt {x: 15.2, y: 0.0}, 5.0);
  let dis = distance(rct, trngl);

  println!("
  - Circulo: {}
  - Triangulo: {}
  - Rectangulo: {}
  -----
  Distancia entre rct, trngl: {}u",
  crcl.area(), trngl.area(), rct.area(), dis)
}

#[test]
fn areas(){
  let rcta = Rectangulo(Pnt {x: 0.0, y: 0.0}, Dim {w: 10.0, h: 5.0});
  let trngla = Triangulo(Pnt {x: 0.0, y: 5.2}, Dim {w: 5.0, h: 3.0});
  let circa = Circulo(Pnt {x: 15.2, y: 0.0}, 5.0);

  assert!(rcta.area() == 50.0)
  assert!(trngla.area() == 7.5)
  assert!(circa.area() == 78.54)
}


#[test]
fn distancias(){
  let rcta = Rectangulo(Pnt {x: 0.0, y: 0.0}, Dim {w: 10.0, h: 5.0});
  let trngla = Triangulo(Pnt {x: 0.0, y: 5.2}, Dim {w: 5.0, h: 3.0});
  let circa = Circulo(Pnt {x: 15.2, y: 0.0}, 5.0);

  assert!(distance(rcta, circa) == 15.2)
  assert!(distance(rcta, trngla) == 0.0)
}