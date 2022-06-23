fn main() {
    println!("Hello, world!");
}

struct Ball {
    w: i16,
    h: i16,
    r: f32,
    s: f32,
}

impl Ball {
    //turn ball direction 180°
    fn bounce(&mut self) {
        if self.r < 180. {
            self.r += 180.;
        } else {
            self.r -= 180.;
        }
    }
    //move ball one step ahead
    fn step(&mut self) {
        self.w += (self.r.cos() * self.s) as i16;
        self.h += (self.r.sin() * self.s) as i16;
    }
}



#[test]
//Test wether bounce effectively change horizontal direction of the ball
//From left to right or right to left
fn test_bounce_effective_rotation() {
    let mut ball = Ball { w: 0, h: 0, r: 0., s:0. };
    
    ball.bounce();    
    assert_eq!(ball.r, 180.);
    ball.bounce();
    assert_eq!(ball.r, 0.);
}

#[test]
//Test wether bounce maintain r value between 0 and 360°
fn test_bounce_angle_value() {
    let mut ball = Ball { w: 0, h: 0, r: 340., s:0. };
    
    ball.bounce();    
    assert_eq!(ball.r, 160.);
    ball.bounce();
    assert_eq!(ball.r, 340.);
}

#[test]
//Test step one pixel ahead in line
fn test_bounce_step() {
    let mut ball = Ball { w: 100, h: 100, r: 0., s:1. };

    ball.step();
    assert_eq!( (ball.w, ball.h), (101, 100) );
}

#[test]
//Test step in random conditio
fn test_bounce_step_random() {
    let mut ball = Ball { w: 100, h: 100, r: 345., s:3. };

    ball.step();
    assert_eq!( (ball.w, ball.h), (101, 100) );
}