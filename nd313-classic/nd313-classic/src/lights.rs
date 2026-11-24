use lotus_extra::bb_system::{
    basic::{BackBoneResetInputOutput, BackBoneResetType, ModuleInit, ModuleTickInput},
    lights::{BBIndicatorLights, Bulb, IndicatorLights, IndicatorState},
};

const MIX_FADE: (f32, f32) = (20.0, 15.0);

pub struct Nd313Lights {
    pub blinker: IndicatorLights,
    pub dim_scale: Bulb,
    pub dim_blue: Bulb,
    pub rear_n_brake: Bulb,
}

impl Default for Nd313Lights {
    fn default() -> Self {
        Self {
            blinker: IndicatorLights::new(
                vec![
                    Bulb::new("Light_Indicator_Left".to_string())
                        .with_exp_fade_in_out((20.0, 15.0)),
                    Bulb::new("Light_Indicator_Left_LED".to_string()),
                ],
                vec![
                    Bulb::new("Light_Indicator_Right".to_string())
                        .with_exp_fade_in_out((20.0, 15.0)),
                    Bulb::new("Light_Indicator_Right_LED".to_string()),
                ],
                0.40,
                0.35,
                0.43,
            )
            .with_sound("snd_IndicatorRelayOn", "snd_IndicatorRelayOff"),
            dim_scale: Bulb::new("Light_DimLight_Scale").with_exp_fade_in_out(MIX_FADE),
            dim_blue: Bulb::new("Light_DimLight_Blue").with_exp_fade_in_out(MIX_FADE),
            rear_n_brake: Bulb::new("Light_RearNBrake").with_exp_fade_in_out(MIX_FADE),
        }
    }
}

#[derive(Default)]
pub struct BBLights {
    pub indicator: BBIndicatorLights,
    pub dim_scale: f32,
    pub dim_blue: f32,
    pub rear_n_brake: f32,
}

pub struct Nd313LightsInput {
    pub indicator: IndicatorState,
    pub indicator_voltage_normalized: f32,
    pub dim_scale: f32,
    pub dim_blue: f32,
    pub rear_n_brake: f32,
}

impl ModuleInit<BBLights> for Nd313Lights {
    fn init(&self, backbone: &mut BBLights) {
        self.blinker.init(&mut backbone.indicator);
        self.dim_scale.init(&mut backbone.dim_scale);
        self.dim_blue.init(&mut backbone.dim_blue);
        self.rear_n_brake.init(&mut backbone.rear_n_brake);
    }
}

impl ModuleTickInput<BBLights, Nd313LightsInput> for Nd313Lights {
    fn tick(&self, backbone: &mut BBLights, input: Nd313LightsInput) {
        self.blinker.tick(
            &mut backbone.indicator,
            (input.indicator, input.indicator_voltage_normalized),
        );
        self.dim_scale
            .tick(&mut backbone.dim_scale, input.dim_scale);
        self.dim_blue.tick(&mut backbone.dim_blue, input.dim_blue);
        self.rear_n_brake
            .tick(&mut backbone.rear_n_brake, input.rear_n_brake);
    }
}

impl BackBoneResetInputOutput for BBLights {
    fn reset(&mut self, reset_type: BackBoneResetType) {
        self.indicator.reset(reset_type);
    }
}
