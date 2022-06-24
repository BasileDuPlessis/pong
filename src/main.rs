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
    //turn ball direction 180°
    fn bounce(&mut self, wall_angle:f32) {
        let wall_orthogonal_angle = wall_angle + 90.;
        let orthogonal_bounce_angle = self.a + 180.;

        self.a = 2. * wall_orthogonal_angle - orthogonal_bounce_angle;

    }
    //move ball one step ahead
    fn step(&mut self) {
        self.w += (self.a.cos() * self.s) as i16;
        self.h += (self.a.sin() * self.s) as i16;
    }
}



#[test]
//Test wether bounce effectively change direction of the ball like a wall do
fn test_bounce_effective_rotation() {
    let mut ball = Ball { w: 0, h: 0, a: 30., s:2. };
    
    ball.bounce(110.);    
    assert_eq!(ball.a, 190.);
}


#[test]
//Test step one pixel ahead in line
fn test_bounce_step() {
    let mut ball = Ball { w: 100, h: 100, a: 0., s:1. };

    ball.step();
    assert_eq!( (ball.w, ball.h), (101, 100) );
}

#[test]
//Test step in random conditio
fn test_bounce_step_random() {
    let mut ball = Ball { w: 100, h: 100, a: 345., s:3. };

    ball.step();
    assert_eq!( (ball.w, ball.h), (101, 100) );
}
