// use std::io;
// use rand::Rng;

fn main(){
    /////Structs/////
    // #[derive(Debug)]
    // struct User {
    //     name: String,
    //     age: u32,
    //     is_married: bool,
    //     my_type: String

    // }

    // impl User {
    //     fn greeter(&self){
    //         println!("hello im {} and im {} years old and im into {}", self.name, self.age, self.my_type);
    //     }
    // }

    // let user = User {
    //     name: String::from("Arokoyu Olaniyi"),
    //     age: 22,
    //     is_married: false,
    //     my_type: String::from("Light Skins")
    // };

        // #[derive(Debug)]
        // struct Product {
        //     name: String,
        //     price: f64,
        //     in_stock: bool,
        //     message_is_in_stock: String
        // }

        // impl Product{
        //     fn is_in_stock(&mut self) {
        //         if self.in_stock{
        //             self.message_is_in_stock=String::from("Product is in stock!!");
        //         }else{
        //             self.message_is_in_stock=String::from("Product is out of stock!!");
        //         }
        //     }
        // }


        // let mut actual_product = Product {
        //     name: String::from("Nike Air Force 1s"),
        //     price: 50.99,
        //     in_stock:true,
        //     message_is_in_stock: String::from("")
        // };

        // actual_product.is_in_stock();


        // println!("{:?}", actual_product);

        // enum OrderStatus {
        //     Pending,
        //     Shipped,
        //     Delivered
        // }

        // let status = OrderStatus::Pending;


        // match status {
        //     OrderStatus::Pending => println!("your order is pending!!"),
        //     OrderStatus::Delivered=>println!("your order is delivered!!"),
        //     OrderStatus::Shipped=>println!("your order is shipped!!")

        // }
        
     #[derive(Debug)]
        struct Book1 {
            title: String,
            author: String,
            pages:u32,
            status:Status
        }

        struct Book2 {
            title: String,
            author: String,
            pages:u32,
            is_available:bool
        }

        impl Book1 {
            fn status(&self){
                match self.status {
                    Status::Available=>println!("book is available"),
                    Status::CheckedOut=>println!("book is not available")
                }
            }
        }
        
        let actual_book = Book1 {
            title:String::from("game of thrones"),
            author:String::from("sanza stark"),
            pages: 5000,
            status:Status::Available
            // is_available: true
        };
#[derive(Debug)]
        enum Status {
    Available,
    CheckedOut,
}


        println!("{:?}", actual_book);
        actual_book.status()


    //////// ENUMS ////////////
    //  #[derive(Debug)]
    // enum GameDirections {
    //     Up,
    //     Down,
    //     Left,
    //     Right
    // }

    // let direction = GameDirections::Left;

    // match direction {
    //     GameDirections::Up => println!("you moved up"),
    //     GameDirections::Down => println!("you moved down"),
    //     GameDirections::Left => println!("you moved left"),
    //     GameDirections::Right => println!("you moved right"),
    // }

    // println!("{:?}", direction);

    // println!("{:?}",user);
    // user.greeter();
}

