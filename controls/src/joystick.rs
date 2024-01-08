use core::default;

pub struct Stick {
    pub x: f32,
    pub y: f32,
}


// x --> 
//   ^
// y |

pub struct Control {
    is_calibrated: bool,
    x_stick: Stick,
    y_stick: Stick,
    
    x_offset: f32,
    y_offset: f32,
}

impl Default for Control {
    fn default() -> Self {
        is_calibrated: false,

    }
}