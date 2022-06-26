use std::f32::consts::PI;

fn main() {
    println!("Hello, world!");
}

struct Ball {
    w: i16,
    h: i16,
    a: f32,
    s: f32,
    
}

impl Ball {
    //make ball bounce against a vertical wall 
    fn bounce_vertical(&mut self) {
        if self.a > PI {
            self.a = 3. * PI - self.a;
        } else {
            self.a = PI - self.a;
        }
    }

    //make ball bounce against the ceil 
    fn bounce_horizontal(&mut self) {
        self.a = 2. * PI - self.a;
    }

    //move ball one step ahead
    fn step(&mut self) {
        self.w += (self.a.cos() * self.s) as i16;
        self.h += (self.a.sin() * self.s) as i16;
    }
}



#[test]
//Test bounce from RIGHT to LEFT
fn test_bounce_total_radian_eq_pi_divided_2() {
    let a = 0.5;
    let mut ball = Ball { w: 0, h: 0, a: a, s:2. };    
    ball.bounce_vertical();    
    assert_eq!(ball.a + a, PI);
}

#[test]
//Test bounce from LEFT to RIGHT
fn test_bounce_total_radian_eq_1dot5_pi() {
    let a = PI + 0.5;
    let mut ball = Ball { w: 0, h: 0, a: a, s:2. };    
    ball.bounce_vertical();    
    assert_eq!(ball.a + a, 3. * PI);
}

#[test]
//Test bounce on ceil
fn test_bounce_total_radian_eq_pi() {
    let a = PI + 0.5;
    let mut ball = Ball { w: 0, h: 0, a: a, s:2. };    
    ball.bounce_horizontal();    
    assert_eq!(ball.a + a, 2. * PI);
}



#[test]
//Test step right and bottom
fn test_bounce_step_right_bottom() {
    let w = 100;
    let h = 100;
    let mut ball = Ball { w: 100, h: 100, a: 2. * PI - 0.4, s:3. };

    ball.step();
    assert_eq!( ball.w > w, true);
    assert_eq!( ball.h < h, true);
}