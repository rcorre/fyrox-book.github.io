use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        reflect::prelude::*,
        uuid::Uuid,
        uuid_provider,
        visitor::prelude::*,
        TypeUuidProvider,
    },
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    impl_component_provider,
    script::{ScriptContext, ScriptTrait},
};

// ANCHOR: mouse
#[derive(Clone, Debug, Reflect, Visit)]
pub struct Player {
    yaw: f32,
    pitch: f32,
}

impl ScriptTrait for Player {
    fn on_os_event(&mut self, event: &Event<()>, _ctx: &mut ScriptContext) {
        // We'll listen to MouseMotion raw device event to rotate an object. It provides
        // offsets only.
        if let Event::DeviceEvent {
            event: DeviceEvent::MouseMotion {
                delta: (dx, dy), ..
            },
            ..
        } = event
        {
            self.pitch = (self.pitch + *dy as f32)
                .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
            self.yaw += *dx as f32;
        }
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        let node = &mut ctx.scene.graph[ctx.handle];
        let transform = node.local_transform_mut();
        transform.set_rotation(
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.pitch)
                * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.yaw),
        );
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
// ANCHOR_END: mouse

impl_component_provider!(Player);
uuid_provider!(Player = "abbad54c-e267-4d7e-a3cd-e125a7e87ff0");

// ANCHOR: clicker
#[derive(Clone, Debug, Reflect, Visit)]
pub struct Clicker {
    counter: i32,
}

impl ScriptTrait for Clicker {
    fn on_os_event(&mut self, event: &Event<()>, _ctx: &mut ScriptContext) {
        if let Event::WindowEvent {
            event: WindowEvent::MouseInput { button, state, .. },
            ..
        } = event
        {
            if *state == ElementState::Pressed {
                match *button {
                    MouseButton::Left => {
                        self.counter -= 1;
                    }
                    MouseButton::Right => {
                        self.counter += 1;
                    }
                    _ => (),
                }
            }
        }
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
// ANCHOR_END: clicker

impl_component_provider!(Clicker);
uuid_provider!(Clicker = "abbad54c-e267-4d7e-a3cd-e125a7e87ff0");
