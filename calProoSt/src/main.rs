fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect = Rect{
        width:30,
        height:50
    };
    //先说结论: println!宏参数是通过引用传递的。
    println!("Area: {}",area(width1,height1));
    println!("Area: {}",area1((width1,height1)));
    println!("Area: {}",area2(&rect));
    println!("Area: {}",rect.area());
    println!("{:#?}", rect);
    dbg!(&rect);
    //
    println!("{}", rect.can_hold(&Rect {
        width: 10,
        height: 10
    }));
    println!("{}", rect.can_hold(&Rect::square(1)));
}
fn area(width:u32, height:u32) -> u32{
    width*height
}
fn area1(dem:(u32,u32)) ->u32{
    dem.0*dem.1
}
fn area2(rect: &Rect) -> u32{
    rect.width*rect.height
}

#[derive(Debug)]
struct Rect{
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32{
        self.height*self.width
    }
    fn can_hold(&self, other: &Rect) -> bool{
        self.width>other.width && self.height>other.height
    }
    fn square(size: u32)->Self{
        Self{
            width:size,
            height:size
        }
    }
}