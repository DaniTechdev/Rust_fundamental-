//Implement `fn summary` to make the code work
//fix the erorr without removing any code line

use std::iter::Sum;

trait Summary {
    fn summarize(&self) -> String;
}


#[derive(Debug)]
struct Post {
    title:String,
    author:String,
    content:String
}

impl Summary for Post{
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct  Weibo{
    username: String,
    content:String
}

impl Summary for Weibo{
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}


fn main(){
    let post: Post = Post {
        title:"Popular Rust".to_string(),
        author:"Sunface".to_string(),
        content:"Rust is awesome".to_string()
    };

    let weibo :Weibo = Weibo {
        username: "sunface".to_string(), 
        content: "Weibo seems to be worse than Tweet".to_string() 
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}",post);
    println!("{:?}", weibo);

    println!("{} post.summarize", post.summarize());
}


//Implement `fn summary` below

// fn summary(item: &impl Summary) {
//  let output:String = item.summarize();

//  println!("{}", output)
// }

//Or

fn summary<T: Summary>(item: &T) {
    
    let output:String = item.summarize();
   
    println!("{}", output)
   }


