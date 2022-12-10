use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cell::Ref;
#[derive(Default)]
struct Trie {
    pas:i32,
    end:i32,
    nexts:[Option<Box<Trie>>;26]
}
impl Trie {

    fn new() -> Self {
        Trie::default()
    }
    fn insert(&mut self, word: String) {
        let mut node=self;
      for s in word.bytes(){
          let index=(s - b'a') as usize;
       if node.nexts[index].is_none(){
           //不存在
           node.nexts[index]=Some(Box::new(Trie::new()));
           node=node.nexts[index].as_deref_mut().unwrap();
           node.pas+=1;
       }else{
     //存在
           node=node.nexts[index].as_deref_mut().unwrap();
           node.pas+=1;
       }
      }
        node.end+=1;
    }

    fn search(& self, word: String) -> bool {
        let mut node=self;
        for s in word.bytes(){
            let index=(s-b'a') as usize;
            if !node.nexts[index].is_none(){
                node= node.nexts[index].as_deref().unwrap();
            }else{
                return false
            }
        }
       if node.end!=0{
           return true
       }else{
           return false
       }
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node=self;
        for s in prefix.bytes(){
            let index=(s-b'a') as usize;
            if !node.nexts[index].is_none(){
                node= node.nexts[index].as_deref().unwrap();
            }else{
                return false
            }
        }
        return true
    }
}

fn main(){

}