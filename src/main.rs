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
    //make ball bounce against the right wall 
    fn bounce_right(&mut self) {
        if self.a > 270. {
            self.a = 360. + 180. - self.a;
        } else {
            self.a = 180. - self.a;
        }
    }

     //make ball bounce against the left wall 
     fn bounce_left(&mut self) {
        if self.a < 180. {
            self.a = 180. - self.a;
        } else {
            self.a = 360. + 180. - self.a;
        }
    }

    //make ball bounce against the ceil 
    fn bounce_ceil(&mut self) {
        self.a = 360. - self.a;
    }

    //make ball bounce against the ceil 
    fn bounce_floor(&mut self) {
        self.a = 360. - self.a;
    }

    //move ball one step ahead
    fn step(&mut self) {
        self.w += (self.a.cos() * self.s) as i16;
        self.h += (self.a.sin() * self.s) as i16;
    }
}



#[test]
//Test bounce from RIGHT to LEFT
fn test_bounce_right_to_left() {
    let mut ball = Ball { w: 0, h: 0, a: 340., s:2. };    
    ball.bounce_right();    
    assert_eq!(ball.a, 200.);

    ball.a = 20.;
    ball.bounce_right();    
    assert_eq!(ball.a, 160.);
}

#[test]
//Test bounce from LEFT to RIGHT
fn test_bounce_left_to_right() {
    let mut ball = Ball { w: 0, h: 0, a: 160., s:2. };    
    ball.bounce_left();    
    assert_eq!(ball.a, 20.);

    ball.a = 200.;
    ball.bounce_left();    
    assert_eq!(ball.a, 340.);
}

#[test]
//Test bounce on ceil
fn test_bounce_ceil() {
    let mut ball = Ball { w: 0, h: 0, a: 20., s:2. };    
    ball.bounce_ceil();    
    assert_eq!(ball.a, 340.);

    ball.a = 160.;
    ball.bounce_ceil();    
    assert_eq!(ball.a, 200.);
}

#[test]
//Test bounce on floor
fn test_bounce_floor() {
    let mut ball = Ball { w: 0, h: 0, a: 340., s:2. };    
    ball.bounce_floor();    
    assert_eq!(ball.a, 20.);

    ball.a = 200.;
    ball.bounce_floor();    
    assert_eq!(ball.a, 160.);
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
    assert_eq!( (ball.w, ball.h), (102, 99) );
}