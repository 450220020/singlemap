use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::any::Any;

type SingleKeyType = String;
#[warn(dead_code)]
type SingleType = (dyn Any + std::marker::Send + Sync);

#[warn(dead_code)]
static INSTANCE: OnceCell<DashMap<SingleKeyType, Box<SingleType>>> = OnceCell::new();

pub static INSTANCE_SINGLE: OnceCell<&DashMap<SingleKeyType, Box<SingleType>>> = OnceCell::new();




#[macro_export]
macro_rules! for_substring {
    ($str:expr,$first:expr,$last:expr) => {
        $str.split_at($first).1.split_at($last-$first).0
    };
}

#[macro_export]
macro_rules! get_map {
    () => {
        $crate::com::map::INSTANCE_SINGLE.get_or_init(|| $crate::com::map::single_init())
    };
}

#[macro_export]
macro_rules! single_push {
    ($key:expr,$val:expr) => {
        get_map!().entry($key.to_string()).insert($val)
    };
}

#[macro_export]
macro_rules! single_or_push {
    ($key:expr,$val:expr) => {
        get_map!().entry($key.to_string()).or_insert($val)
    };
}



#[macro_export]
macro_rules! single_get_unwrap {
    ($key:expr,$b:ty) => {
        get_map!()
            .get($key)
            .unwrap()
            .downcast_ref::<$b>()
            .unwrap().clone()
    };
}


#[macro_export]
macro_rules! single_get_str {
    ($key:expr) => {
        get_map!()
            .get($key)
            .unwrap()
            .downcast_ref::<String>()
            .unwrap().clone()
    };
}




#[warn(dead_code)]
pub fn single_init<'a>(
) -> &'a DashMap<SingleKeyType, Box<(dyn Any + Sync + std::marker::Send + 'a)>> {
    return INSTANCE.get_or_init(|| DashMap::new());
}


pub fn get_idx_val(key:&str)->String{
    let key_idx_max_id = key.to_string()+"_max";
    let key_idx_id = key.to_string()+"_sqe_idx";
    let max_idx = single_get_unwrap!(&key_idx_max_id,i32);
    let map_entity = get_map!();
    if map_entity.contains_key(&key_idx_id){
        map_entity.alter(&key_idx_id,  |_, v| {
            let cur_id = *v.downcast_ref::<i32>().unwrap();
            if max_idx>cur_id{
                return Box::new(cur_id+1);
            }else{
                return Box::new(1);
            }   
        });
    }else{
        single_or_push!(key_idx_id,Box::new(1i32));
    }
 
    let cur_idx = single_get_unwrap!(&key_idx_id,i32);
    let cur_key = key.to_string()+"_"+&cur_idx.to_string();
    return single_get_str!(&cur_key);
}


#[macro_export]
macro_rules! single_get_sqe_str {
    ($key:expr) => {
        $crate::com::map::get_idx_val($key)
    };
}

#[macro_export]
macro_rules! single_push_sql_max {
    ($key:expr,$val:expr) => {
        get_map!().entry($key.to_string()+"_max").insert($val)
    };
}