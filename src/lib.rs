pub mod front_of_house {
    pub mod hosting {
        fn seat_at_table() {}
        pub fn add_to_waitlist() {}
    }
    mod serving {
        fn take_order() {
            // 通过suer使用父模块的内容
            super::hosting::add_to_waitlist();
        }
        fn serve_order() {}
        fn take_payment() {}
    }
}
