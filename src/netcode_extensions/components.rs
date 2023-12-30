use bevy::prelude::Component;

//#[derive(Component, Default)]
//pub struct InputBuffer<T>{
//
//}

// like that but where T is Serialize, Deserialize

#[derive(Component, Default)]
pub struct InputBuffer<T>{
    pub buffer : Vec<T>,
}

