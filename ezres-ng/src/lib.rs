#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, nonstandard_style, dead_code, unused)]

// --- FUGGOSEGEK --- //

// LOTUS
use lotus_script::prelude::*;
use lotus_script::input::*;
use lotus_script::action::*;
use lotus_script::{script};


// SAJÁT
use types::Vars;


// --- MODULOK --- //
pub mod config;
pub mod electronics;
pub mod cockpit;
pub mod types;
pub mod traction;
// pub mod passenger;
pub mod func;



// --- SCRIPT ------------------------------------------------------------------------- // 

script!(ScriptEzres);

#[derive(Default)]
pub struct ScriptEzres {
    // warnlight_A_RET: t_warnlight,
    // sw_A_railbrake: t_switch_bistate,
    // sw_A_wiperwasher: t_switch_bistate,
    // timer: f32,
    vars: types::Vars,
}

impl Script for ScriptEzres {

    // --- CUSTOM HOTKEYS ------------ //
      fn actions() -> Vec<RegisterAction> {
        ActionsBuilder::new()
        .push("PantographUp", KeyCode::KeyE)
        // .push("PantographDn", KeyCode::KeyR)

        .push("BKV1000_fekszelepPlusz", KeyCode::KeyP)
        .push("BKV1000_fekszelepMinusz", KeyCode::KeyO)


        // .push("InsertKey_Reverser", KeyCode::NumpadSubtract)
        // .push("HighVoltageMainSwitchOn", KeyCode::NumpadSubtract)
        // .push("HighVoltageMainSwitchOff", KeyCode::NumpadSubtract)
        // .push("ConverterOn", KeyCode::NumpadSubtract)
        // .push("FrontLightFull", KeyCode::NumpadSubtract)
        // .push("FrontLightDim", KeyCode::NumpadSubtract)
        // .push("WiperToggle", KeyCode::NumpadSubtract)
        // .push("SwitchToggleButton", KeyCode::NumpadSubtract)
        // .push("DriverHeaterToggle", KeyCode::NumpadSubtract)
        // .push("CockpitLightToggle", KeyCode::NumpadSubtract)
        // .push("FrontWindowHeaterPlus", KeyCode::NumpadSubtract)
        // .push("FrontWindowHeaterMinus", KeyCode::NumpadSubtract)
        // .push("WindscreenWasher", KeyCode::NumpadSubtract)
        // .push("Speaker", KeyCode::NumpadSubtract)
        // .push("PassengerHeater", KeyCode::NumpadSubtract)
        // .push("EmergencyLights", KeyCode::NumpadSubtract)
        // .push("PassengerLights", KeyCode::NumpadSubtract)
        // .push("ICS_vilagitas", KeyCode::NumpadSubtract)
        // .push("ICS_kenyszkapcs", KeyCode::NumpadSubtract)
        // .push("ICS_szuksegmenet", KeyCode::NumpadSubtract)
        // .push("FrontLightFlash", KeyCode::NumpadSubtract)
        .build()
    }

    // --- ACCESSORS FOR VARS -------- //

    // --- INIT ---------------------- //
    fn init(&mut self) {
        log::info!("// Init main script -----------------------------------------------------");

        // GLOBÁLIS VÁLTOZÓ HANDLE
        self.vars = Vars::new();

        // Nem használt értékek (eredeti LOTUS-ban volt értelmük)
        set_var("kasztni_bolintas", 0.0);
        set_var("KAMERA_BOLINT", 0.0);
        set_var("KAMERA_OLDAL", 0.0);
        
        // SUBSYSTEMS ------------ //
        cockpit::init_cockpit(&mut self.vars);
        electronics::init_electronics(&mut self.vars);
        traction::init_traction(&mut self.vars);
    }

    // --- FRAME  -------------------- //
    fn tick(&mut self) {

        // Valtozo konverziok
        let v_ground = get_var::<f32>("v_Axle_mps_0_0");
        let v_kmph = v_ground * 3.6;
        set_var("v_kmph", v_kmph);
        set_var("v_abs_kmph", get_var::<f32>("v_kmph").abs());


        // Egyeb valtozok
        if !get_var::<bool>("coupled_0") {
            set_var("couplingState_0", 1);
        };
        if !get_var::<bool>("coupled_1") {
            set_var("couplingState_1", 1);
        };

        // Tobbi modul
        traction::simstep_traction(v_kmph, &mut self.vars);
        cockpit::simstep_cockpit(v_kmph, &mut self.vars);
        electronics::simstep_electronics(v_kmph, &mut self.vars);

    }


    // HANDLE INCOMING MESSAGES
    fn on_message(&mut self, msg: Message) {

        msg.handle(|a: ActionEvent| {
            // log::info!("Message: {:?}", a);

            // Global actions

            // --- COUPLINGS --- //
            if (a.name == "Coupling_front") && (a.state.kind.is_just_pressed()) { set_var("couplingState_0", false); }
            if (a.name == "Coupling_back") && (a.state.kind.is_just_pressed()) { set_var("couplingState_1", false); }

            // Send to cockpit func
            cockpit::on_message_cockpit(&mut self.vars, a);
            Ok(())



        })
        .ok();

    }
}

