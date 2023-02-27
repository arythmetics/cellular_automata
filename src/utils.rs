use bevy::math::IVec3;

pub fn keep_in_bounds(bounds: i32, pos: &mut IVec3) {
    //pos.x >= -bounds && pos.y >= -bounds && pos.z >= -bounds && pos.x <= bounds && pos.y <= bounds && pos.z <= bounds
    if pos.x <= -bounds {
        pos.x = bounds - 1;
    } else if pos.x >= bounds {
        pos.x = -bounds + 1;
    }
    if pos.y <= -bounds {
        pos.y = bounds - 1;
    } else if pos.y >= bounds {
        pos.y = -bounds + 1;
    }
    if pos.z <= -bounds {
        pos.z = bounds - 1;
    } else if pos.z >= bounds {
        pos.z = -bounds + 1;
    }
}