pub struct :calculator::new {}

impl calculator{
    pub fn new() -> self{
        self{
 
        }
    }
    pub fn add(&self, x: &f64, y: &f64) -> f64{
        x + y
    }
    pub fn subtract(&self, x: &f64, y: &f64) -> f64{
        x - y
    }
    pub fn divide(&self, x: &f64, y: &f64) -> f64{
            x / y
        }
    pub fn multiply(&self, x: &f64, y: &f64) -> f64{
                x * y
    }

    pub fn power(&self, x: &f64, y: &f64) -> f64{
            x power(y)
    }

    pub fn factorial(&self, x: &f64, ) -> f64{
        let mut result = 1
        for i in 1..=x {
            result *= i;
        }
    }
    pub fn modulus(&self, x: &f64, y: &f64) -> f64{
    x % y
    }



}







