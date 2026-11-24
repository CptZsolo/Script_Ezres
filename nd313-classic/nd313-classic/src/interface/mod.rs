mod cockpit;
mod doors;
mod lights;
mod messages;
mod pneumatics;
mod power;
mod steering;
mod traction;

use lotus_extra::bb_system::{DomainInterface, DomainMessageInterface, VehicleInterface};
use lotus_script::{message, prelude::*};

use crate::{Backbone, Modules};

pub use cockpit::Nd313CockpitInterface;
pub use doors::Nd313DoorsInterface;
pub use lights::Nd313LightsInterface;
pub use messages::Nd313MessageInterface;
pub use pneumatics::Nd313PneumaticsInterface;
pub use power::Nd313PowerInterface;
pub use steering::Nd313SteeringInterface;
pub use traction::Nd313TractionInterface;

#[derive(Default)]
pub struct Nd313Interface {
    power: Nd313PowerInterface,
    pneumatics: Nd313PneumaticsInterface,
    traction: Nd313TractionInterface,
    steering: Nd313SteeringInterface,
    lights: Nd313LightsInterface,
    cockpit: Nd313CockpitInterface,
    doors: Nd313DoorsInterface,
    messages: Nd313MessageInterface,
}

impl VehicleInterface<Modules, Backbone> for Nd313Interface {
    fn after_init(&mut self, _modules: &Modules, _backbone: &mut Backbone) {
        log::info!("Initializing script ==========================================");
        set_var("Lm_MasterError", 0.0);
    }

    fn tick_interface(&mut self, modules: &Modules, backbone: &mut Backbone) {
        self.power.wire(modules, backbone);
        self.pneumatics.wire(modules, backbone);
        self.traction.wire(modules, backbone);
        self.steering.wire(modules, backbone);
        self.lights.wire(modules, backbone);
        self.cockpit.wire(modules, backbone);
        self.doors.wire(modules, backbone);
    }

    fn interface_on_message(
        &mut self,
        modules: &Modules,
        backbone: &mut Backbone,
        msg: &message::Message,
    ) {
        let _ = self.messages.on_message(modules, backbone, msg);
    }
}
