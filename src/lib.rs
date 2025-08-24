extern crate dashmap;
extern crate once_cell;
pub mod com;



#[test]
fn test_map(){
    single_push!("a",Box::new("aaaa".to_string()));
    let straa = single_get_unwrap!("a",String);
    println!("rustl:{:?}",straa);
    single_push_sql_max!("url",Box::new(3));
    let na = "url_1";
    single_push!(na,Box::new("url1111111".to_string()));
    single_push!("url_2",Box::new("url1111111222222".to_string()));
    single_push!("url_3",Box::new("url111111133333333".to_string()));
    let a = single_get_sqe_str!("url");
    let b = single_get_sqe_str!("url");
    let c = single_get_sqe_str!("url");
    let d = single_get_sqe_str!("url");
    println!("a:{:?},b:{:?},c:{:?},d:{:?}",a,b,c,d);
}