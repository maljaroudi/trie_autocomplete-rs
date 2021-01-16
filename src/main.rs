use std::collections::HashMap;
use std::ops::{Deref, Index};


struct Trie {
    root: TrieNode
}


pub struct TrieNode{
    children: HashMap<char,TrieNode>,
    is_word_end: bool
}

impl Trie{
    fn new() -> Self{
        Trie { root: TrieNode::new() }
    }



    fn insert (&mut self, key:String) {
        let mut current = &mut self.root;
        for w in key.chars() {
            current = current.children.entry(w).or_insert(TrieNode::new());
        }

        if !current.is_word_end {
            current.is_word_end = true;
        }
    }


    fn search(&mut self, word: String) -> bool{
        let mut current = &mut self.root;
        for level in word.chars(){
            match current.children.get(&level){
                Some(_x) => {current = current.children.entry(level).or_insert(TrieNode::new());},
                None => return false
            }
        }
        current.is_word_end
    }


    fn start_with(&mut self, prefix: String) -> bool {
        let mut current = &mut self.root;
        for level in prefix.chars(){
            match current.children.get(&level){
                Some(_x) => {current = current.children.entry(level).or_insert(TrieNode::new());},
                None => return false
            }
        }
        true
    }










}


impl TrieNode{

    fn new() -> Self {
        TrieNode{
            children: HashMap::new(),
            is_word_end: false
        }
    }

    fn suggestion_rec(&mut self, mut curr_prefix: String) {
        let root = self;

        if root.is_word_end{
            println!("{}", &curr_prefix);
        }

        if root.last_node(){
            return;
        }
        //let mut prefixed = curr_prefix.to_string();
        for i in (b'A'..=b'z'){
            if root.children.contains_key(&(i as char)){
                curr_prefix.push(i as char);
                root.children.get_mut(&(i as char)).unwrap().suggestion_rec(curr_prefix.clone());
                curr_prefix.pop();
            }

        }
    }

    fn last_node(&mut self) ->bool{

            if !self.children.is_empty(){
                return false;
            }

        return true;
    }

    fn collect_all_matches(&mut self, prefix: &str){
        let mut root = self;

        for c in prefix.chars(){
            if !root.children.contains_key(&c){
                return;
            }
            root = root.children.get_mut(&c).unwrap();
        }
        let is_word = root.is_word_end;
        let is_last = root.last_node();

        if (is_word&&is_last){
            println!("{}",prefix);
            return;
        }
        if !is_last {
            root.suggestion_rec(prefix.parse().unwrap());
            return;
        }
    }


}



fn main() {
    let mut obj = Trie::new();
    obj.insert(String::from("hello"));
    obj.insert(String::from("heli"));
    obj.insert(String::from("hell"));
    //println!("{}", obj.start_with(String::from("hel")));
    let test_string="hel";
    let test_string1 = "NoEntryAvailableTest";
    let v = obj.root.collect_all_matches(test_string);
    println!("+++++++++++++++++");
    obj.root.collect_all_matches(test_string1);

    //println!("{:?}",v);


}
