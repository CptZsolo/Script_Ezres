use lotus_extra::bb_system::vdv_dashboard::VdvDisplayDoorState;
use lotus_extra::bb_system::{DomainInterface, basic::BackBone};
use lotus_extra::messages::std::StopRequest;
use lotus_script::message::{MessageTarget, send_message};

use crate::{Backbone, Modules};

#[derive(Default)]
pub struct Nd313CockpitInterface;

impl DomainInterface<Modules, Backbone> for Nd313CockpitInterface {
    fn wire(&mut self, modules: &Modules, backbone: &mut Backbone) {
        let idx = modules.electricity.indices();
        let bb_cockpit = &mut backbone.cockpit.vdv_dashboard;
        let bb_doors = &mut backbone.doors;
        let electric = &modules.electricity;

        bb_cockpit.voltage_normalized =
            electric.unit_voltage_normalized(&backbone.electricity, idx.fuse_ignition);
        bb_cockpit.voltage_available_normalized =
            electric.unit_voltage_normalized(&backbone.electricity, idx.fuse_dauerplus);

        bb_cockpit.pneumatics = backbone.pneumatics;

        bb_cockpit
            .indicator_switch
            .set_steering_normalized(backbone.steering.angle_normalized);

        backbone
            .lights
            .indicator
            .blink_relay()
            .forward_on_changed(&mut bb_cockpit.indicators_bulbs);

        bb_doors.door_closed(0, 0).call_on_changed_two(
            bb_doors.door_closed(0, 1),
            |closed1, closed2| {
                bb_cockpit.il_doors_target[0].set(!(closed1 && closed2));
            },
        );

        bb_doors.door_closed(1, 0).call_on_changed_two(
            bb_doors.door_closed(2, 0),
            |closed1, closed2| {
                bb_cockpit.rear_doors = !(closed1 && closed2);
            },
        );

        bb_doors.stop_sign(1).call_on_changed(|value| {
            bb_cockpit.stop_request_middle.set(value);
            send_message(&StopRequest { value }, [MessageTarget::ModuleSlot(10)]);
        });

        bb_doors.stop_sign(2).call_on_changed(|value| {
            bb_cockpit.stop_request_rear.set(value);
            send_message(
                &StopRequest { value },
                [MessageTarget::ModuleSlot(11), MessageTarget::ModuleSlot(12)],
            );
        });

        bb_doors
            .stop_brake()
            .forward_on_changed(&mut bb_cockpit.stop_brake);

        let door_released =
            bb_doors.release_target(0).get_state() && bb_doors.release_activatable.get_state();

        bb_cockpit
            .display_door_1_1
            .set_if_different(vdv_display_door_state(
                bb_doors.door_closed(0, 0).get_state(),
                bb_doors.wing_lock(0, 0).get_state(),
                door_released,
            ));
        bb_cockpit
            .display_door_1_2
            .set_if_different(vdv_display_door_state(
                bb_doors.door_closed(0, 1).get_state(),
                bb_doors.wing_lock(0, 1).get_state(),
                door_released,
            ));
        bb_cockpit
            .display_door_2
            .set_if_different(vdv_display_door_state(
                bb_doors.door_closed(1, 0).get_state(),
                false,
                door_released,
            ));
        bb_cockpit
            .display_door_3
            .set_if_different(vdv_display_door_state(
                bb_doors.door_closed(2, 0).get_state(),
                false,
                door_released,
            ));

        bb_cockpit
            .engine_running
            .set_if_different(backbone.piston_traction_transfer.rpm > 100.0);
    }
}

fn vdv_display_door_state(closed: bool, blocked: bool, released: bool) -> VdvDisplayDoorState {
    if blocked {
        VdvDisplayDoorState::Blocked
    } else if !closed {
        VdvDisplayDoorState::Open
    } else if released {
        VdvDisplayDoorState::Released
    } else {
        VdvDisplayDoorState::Closed
    }
}
