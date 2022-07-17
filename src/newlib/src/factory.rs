pub mod produce_refrigerator{
    pub fn produce_re(){
        println!("冰箱");
    }
}
pub mod produce_washing_machine{
    pub fn produce_re(){
        println!("洗衣机");
        super::produce_refrigerator::produce_re()
    }
}