#[macro_use]
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

use vulkano::instance::layers_list;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use winit::EventsLoop;
use winit::WindowBuilder;
use winit::WindowEvent;
use winit::Event;


fn main() {
    let application_info = app_info_from_cargo_toml!();

    let instance_extensions = InstanceExtensions::none();

    let instance = Instance::new(Some(&application_info), &instance_extensions, None);

    let mut events_loop = EventsLoop::new();

    let window = WindowBuilder::new()
        .with_title(application_info.application_name.unwrap())
        .with_dimensions(1024, 768)
        .build(&events_loop);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::Closed => running = false,
                    _ => (),
                }
            }
            _ => (),
        });
    }

}
