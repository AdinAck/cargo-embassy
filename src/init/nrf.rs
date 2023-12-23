use super::common::init_file;

pub fn init_memory_x() {
    init_file("memory.x", include_str!("../templates/memory.x.template"));
}
