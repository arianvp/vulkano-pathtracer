#[macro_use]
extern crate vulkano;

use vulkano::instance::layers_list;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;

fn main() {
    let application_info = app_info_from_cargo_toml!();

    let instance_extensions = InstanceExtensions::none();

    // Debug layers can be enabled with environment variables. We prefer that
    let layers = None;

    let instance = Instance::new(Some(&application_info), &instance_extensions, layers);
}
