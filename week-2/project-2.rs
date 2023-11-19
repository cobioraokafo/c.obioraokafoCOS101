fn main(){
      let qt = 2.0
      let t = 450000.0;
      println!("This is the quantity for {} {} and this is the price: {}" , "TOSHIBA:",qt,t);

      let qm = 1.0;
      let m = 1500000.0;
      println!("This is the quantity for {} {} and this is the price: {}" , "MAC:",qm,m);

      let qh = 3.0;
      let h = 750000.0;
      println!("This is the quantity for {} {} and this is the price: {}" , "HP:",qh,h);

      let qd = 3.0;
      let d = 2850000.0;
      println!("This is the quantity for {} {} and this is the price: {}" , "DELL:",qd,d);

      let qa = 1.0;
      let a = 250000.0
      println!("This is the quantity for {} {} and this is the price: {}" , "ACER:",qa,a);

      //total amount
      let ta = (qt*t) + (qm*m) + (qh*h) + (qa*a);

      //Average
      let avg = ta/(qt+qm+qh+qd+qa);
      println!("The average is {}",avg)