// fn main() {
//     let student1=Student::Online;
//     let student2=Student::Onsite;
//     println!("{:?}",student1);
// }
// #[derive(Debug)]
// enum Student{
//     Online,
//     Onsite
// }

// ======EMU< WITH DATA METHOD1======
// fn main(){
// let s1=IPAdd{
// over:IP::V6
// msg:String::from("SYED AHSAN ALI");
// }
// println!("{:#?}",s1);
// }
// #[derive(Debug)]
// enum IP{
//     V4.
//     V6,
// }
// #[derive(Debug)]
// struct IPAdd{
//     over:IP,
//     msg : String
// }
// =================ENUM WITH DATA METHOD @===========
// fn main(){
// let s1=IP::V4(String::from("SYED AHSAN ALI"));
// println!("{:#?}",s1);
// let s2=IP::V6(192,128,0,0,1);
// println!("{:?}",s2);
// }
// #[derive(Debug)]
// enum IP{
//     V4(String),
//     V6(u32,u32,u32,u32,u32)
// }
// CREATIng ENUM Message=========================
// #[derive(Debug)]
// enum Message{
//     Quit,
//     Msg(String),
//     Move{x:i32,y:i32},
//     ChangeColor(u32,u32),
// }
// fn main(){
//     let n1=Message::Quit;
//     let n2=Message::Msg(String::from("Syed Ahsan Ali"));
//     let n3=Message::Move{x:10,y:-9};
//     let n4=Message::ChangeColor(4,2);
//     println!("{:#?}",n1);
//     println!("{:#?}",n2);
//     println!("{:#?}",n3);
//     println!("{:#?}",n4);

// }

// =============METHOD IN ENUM
// fn main(){
//     let msg1=Message::Msg(String::from("Syed Ahsan Ali"));
//     msg1.call();
// }

// #[derive(Debug)]
// enum Message{
//         Quit,
//         Msg(String),
//         Move{x:i32,y:i32},
//         ChangeColor(u32,u32),
//     }
//     impl Message{
//         fn call (&self){
//             println!("{:?}",self)
//         }
//     }

// ENUM in FUNCTION=====================
// fn main(){
// let n=Ip::V4;
// let n1=Ip::V4;
// route(n);
// route(n1);
// }
// #[derive(Debug)]
// enum Ip{
//     V4,
//     V6
// }
// fn route(x:Ip){
//     println!("{:?}",x);
// }

// ===============THE OPTION ENUM==================
// fn main(){
// let s=Option::Some(7);
// println!("{:?}",s);
// }
// #[derive(Debug)]
// enum Option<T>{
// Some(T),
// None,
// }

// ===================MATCH CONTROL FLOW OPERATOR=============
// fn main(){
//     let x=2;
//     match x{
//         1 => println!("one"),
//         2 => println!("two"),
//         _ => println!("Blank")
//     }
// }

// COIN SORTING MACHINE (MATCH CONTROL FLOW OPERATOR)====================
// fn main(){
// let ss=Shoes::Vains;
// let z=typo(ss);
// println!("{:?}",z);
// }
// #[derive(Debug)]
// enum Shoes{
//     Sneakers,
//     Vains,
//     Toms,
//     UltraBoost
// }
// fn typo(shoes:Shoes) -> u8{
//     match shoes{
//        Shoes :: Sneakers => 10,
//        Shoes ::  Vains => 12,
//        Shoes ::  Toms => 2,
//        Shoes ::  UltraBoost => 8
//     }
// }


// fn main(){
//     let ss=Shoes::Vains(Cc::Black);
//     let z=typo(ss);
//     println!("{:?}",z);
//     }
//     #[derive(Debug)]
//     enum Shoes{
//         Sneakers,
//         Vains(Cc),
//         Toms,
//         UltraBoost
//     }
//     #[derive(Debug)]
//     enum Cc{
//         Maroon,
//         Black,
//     }
//     fn typo(shoes:Shoes) -> u8{
//         match shoes{
//            Shoes :: Sneakers => 10,
//            Shoes ::  Vains(Colore) =>{
// println!("{:?}",Colore);
// 12
//            } 
//            Shoes ::  Toms => 2,
//            Shoes ::  UltraBoost => 8
//         }
//     }

// =========MAtching With OPtion===================
// fn main(){
// let f=Some(4);
// let s=AddOne(f);
// println!("{:?}",s);
// }
// fn AddOne(x:Option<i32>)->Option<i32>{
//     match x{
//         None => None,
//         Some(i) => (i+1)
//     }
// }