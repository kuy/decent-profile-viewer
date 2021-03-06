use std::{
    convert::TryFrom,
    str::{self, FromStr},
};

use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_till, take_until};
use nom::character::{
    complete::{multispace0, multispace1, space1, u16},
    is_newline, is_space,
    streaming::digit1 as digit,
};
use nom::combinator::{map, map_res, opt, peek, recognize};
use nom::multi::separated_list0;
use nom::sequence::{delimited, tuple};
use nom::{
    error::{ErrorKind, ParseError},
    Err, IResult,
};

fn unsigned_float(i: &[u8]) -> IResult<&[u8], f32> {
    let float_bytes = recognize(alt((
        delimited(digit, tag("."), opt(digit)),
        delimited(opt(digit), tag("."), digit),
    )));
    let float_str = map_res(float_bytes, str::from_utf8);
    map_res(float_str, FromStr::from_str)(i)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Prop {
    ExitIf(bool),
    Flow(f32),
    Volume(f32),
    MaxFlowOrPressureRange(f32),
    Transition(TransitionType),
    ExitFlowUnder(f32),
    Temperature(f32),
    Name(String),
    Pressure(f32),
    Sensor(SensorType),
    Pump(PumpType),
    ExitType(ExitType),
    ExitFlowOver(f32),
    ExitPressureOver(f32),
    MaxFlowOrPressure(f32),
    ExitPressureUnder(f32),
    Seconds(f32),
    Weight(f32),
    Unknown((String, String)),
}

#[derive(Clone, Debug)]
pub struct ConvertError(String);

impl TryFrom<TransitionType> for Prop {
    type Error = ConvertError;

    fn try_from(value: TransitionType) -> Result<Self, Self::Error> {
        Ok(Prop::Transition(value))
    }
}

trait ParsableEnumProp {
    fn parse(i: &[u8]) -> IResult<&[u8], Prop>;
}

#[derive(Clone, Debug)]
pub struct UnexpectedValueError(String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransitionType {
    Fast,
    Smooth,
}

impl TryFrom<&[u8]> for TransitionType {
    type Error = UnexpectedValueError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value = str::from_utf8(value).expect("should be converted");
        let ret = match value {
            "fast" => TransitionType::Fast,
            "smooth" => TransitionType::Smooth,
            _ => return Err(UnexpectedValueError(value.into())),
        };
        Ok(ret)
    }
}

impl ParsableEnumProp for TransitionType {
    fn parse(i: &[u8]) -> IResult<&[u8], Prop> {
        let (i, (_, _, val)) = tuple((tag("transition"), space1, transition_val))(i)?;
        Ok((i, Prop::Transition(val)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SensorType {
    Coffee,
    Water,
}

impl TryFrom<&[u8]> for SensorType {
    type Error = UnexpectedValueError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value = str::from_utf8(value).expect("should be converted");
        let ret = match value {
            "coffee" => SensorType::Coffee,
            "water" => SensorType::Water,
            _ => return Err(UnexpectedValueError(value.into())),
        };
        Ok(ret)
    }
}

impl ParsableEnumProp for SensorType {
    fn parse(i: &[u8]) -> IResult<&[u8], Prop> {
        let (i, (_, _, val)) = tuple((tag("sensor"), space1, sensor_val))(i)?;
        Ok((i, Prop::Sensor(val)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PumpType {
    Flow,
    Pressure,
}

impl TryFrom<&[u8]> for PumpType {
    type Error = UnexpectedValueError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value = str::from_utf8(value).expect("should be converted");
        let ret = match value {
            "flow" => PumpType::Flow,
            "pressure" => PumpType::Pressure,
            _ => return Err(UnexpectedValueError(value.into())),
        };
        Ok(ret)
    }
}

impl ParsableEnumProp for PumpType {
    fn parse(i: &[u8]) -> IResult<&[u8], Prop> {
        let (i, (_, _, val)) = tuple((tag("pump"), space1, pump_val))(i)?;
        Ok((i, Prop::Pump(val)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExitType {
    PressureUnder,
    PressureOver,
    FlowUnder,
    FlowOver,
}

impl TryFrom<&[u8]> for ExitType {
    type Error = UnexpectedValueError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value = str::from_utf8(value).expect("should be converted");
        let ret = match value {
            "pressure_under" => ExitType::PressureUnder,
            "pressure_over" => ExitType::PressureOver,
            "flow_under" => ExitType::FlowUnder,
            "flow_over" => ExitType::FlowOver,
            _ => return Err(UnexpectedValueError(value.into())),
        };
        Ok(ret)
    }
}

impl ParsableEnumProp for ExitType {
    fn parse(i: &[u8]) -> IResult<&[u8], Prop> {
        let (i, (_, _, val)) = tuple((tag("exit_type"), space1, exit_type_val))(i)?;
        Ok((i, Prop::ExitType(val)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Step(pub Vec<Prop>);

impl Step {
    fn get(&self, prop_name: &str) -> Option<&Prop> {
        self.0.iter().find(|prop| match prop {
            Prop::ExitIf(_) => prop_name == "exit_if",
            Prop::Flow(_) => prop_name == "flow",
            Prop::Volume(_) => prop_name == "volume",
            Prop::MaxFlowOrPressureRange(_) => prop_name == "max_flow_or_pressure_range",
            Prop::Transition(_) => prop_name == "transition",
            Prop::ExitFlowUnder(_) => prop_name == "exit_flow_under",
            Prop::Temperature(_) => prop_name == "temperature",
            Prop::Name(_) => prop_name == "name",
            Prop::Pressure(_) => prop_name == "pressure",
            Prop::Sensor(_) => prop_name == "sensor",
            Prop::Pump(_) => prop_name == "pump",
            Prop::ExitType(_) => prop_name == "exit_type",
            Prop::ExitFlowOver(_) => prop_name == "exit_flow_over",
            Prop::ExitPressureOver(_) => prop_name == "exit_pressure_over",
            Prop::MaxFlowOrPressure(_) => prop_name == "max_flow_or_pressure",
            Prop::ExitPressureUnder(_) => prop_name == "exit_pressure_under",
            Prop::Seconds(_) => prop_name == "seconds",
            Prop::Weight(_) => prop_name == "weight",
            _ => false,
        })
    }

    pub fn seconds(&self) -> f32 {
        let prop = self.0.iter().find(|prop| matches!(prop, Prop::Seconds(_)));
        match prop {
            Some(Prop::Seconds(v)) => *v,
            _ => panic!("not found: seconds"),
        }
    }

    pub fn pump(&self) -> PumpType {
        let prop = self.0.iter().find(|prop| matches!(prop, Prop::Pump(_)));
        match prop {
            Some(Prop::Pump(v)) => *v,
            _ => panic!("not found: pump"),
        }
    }

    pub fn transition(&self) -> TransitionType {
        match self.get("transition") {
            Some(Prop::Transition(v)) => *v,
            _ => panic!("not found: transition"),
        }
    }

    pub fn exit_flow(&self) -> Option<f32> {
        match (self.get("exit_if"), self.get("exit_type")) {
            (Some(Prop::ExitIf(true)), Some(Prop::ExitType(ExitType::FlowOver))) => {
                match self.get("exit_flow_over") {
                    Some(Prop::ExitFlowOver(v)) => Some(*v),
                    _ => None,
                }
            }
            (Some(Prop::ExitIf(true)), Some(Prop::ExitType(ExitType::FlowUnder))) => {
                match self.get("exit_flow_under") {
                    Some(Prop::ExitFlowUnder(v)) => Some(*v),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

fn transition_val(i: &[u8]) -> IResult<&[u8], TransitionType> {
    map_res(alt((tag("fast"), tag("smooth"))), TransitionType::try_from)(i)
}

fn sensor_val(i: &[u8]) -> IResult<&[u8], SensorType> {
    map_res(alt((tag("coffee"), tag("water"))), SensorType::try_from)(i)
}

fn pump_val(i: &[u8]) -> IResult<&[u8], PumpType> {
    map_res(alt((tag("flow"), tag("pressure"))), PumpType::try_from)(i)
}

fn exit_type_val(i: &[u8]) -> IResult<&[u8], ExitType> {
    map_res(
        alt((
            tag("pressure_under"),
            tag("pressure_over"),
            tag("flow_under"),
            tag("flow_over"),
        )),
        ExitType::try_from,
    )(i)
}

fn bool_val(i: &[u8]) -> IResult<&[u8], bool> {
    let (i, v) = alt((tag("0"), tag("1")))(i)?;
    Ok((i, v == &b"1"[..]))
}

fn number_val(i: &[u8]) -> IResult<&[u8], f32> {
    match peek(unsigned_float)(i) {
        Ok((i, _)) => unsigned_float(i),
        _ => map(u16, |v| v as f32)(i),
    }
}

fn plain_string_val(i: &[u8]) -> IResult<&[u8], String> {
    let (i, v) = take_till(|c| is_space(c) || is_newline(c))(i)?;
    Ok((
        i,
        String::from_utf8(v.to_vec()).expect("should be converted"),
    ))
}

fn bracket_string_val(i: &[u8]) -> IResult<&[u8], String> {
    let (i, (_, v, _)) = tuple((tag("{"), take_until("}"), tag("}")))(i)?;
    Ok((
        i,
        String::from_utf8(v.to_vec()).expect("should be converted"),
    ))
}

fn nested_bracket_string_val(input: &[u8]) -> IResult<&[u8], String> {
    let (mut input, _) = tag("{")(input)?;

    let input_len = input.len();
    let mut close_i = None;
    let mut cursor = 0;
    let mut nest = 0;
    while cursor < input_len {
        match input[cursor] as char {
            '{' => {
                nest += 1;
            }
            '}' => {
                if nest == 0 {
                    close_i = Some(cursor);
                    break;
                } else {
                    nest -= 1;
                }
            }
            _ => (),
        }
        cursor += 1;
    }

    if close_i.is_some() {
        let inner = &input[0..cursor];
        input = &input[(cursor + 1)..input_len];

        Ok((
            input,
            String::from_utf8(inner.to_vec()).expect("should be converted"),
        ))
    } else {
        Err(Err::Error(nom::error::Error::from_error_kind(
            input,
            ErrorKind::Fail,
        )))
    }
}

fn string_val(i: &[u8]) -> IResult<&[u8], String> {
    alt((
        nested_bracket_string_val,
        bracket_string_val,
        plain_string_val,
    ))(i)
}

fn prop_bool(name: &str) -> impl Fn(&[u8]) -> IResult<&[u8], Prop> {
    let name = name.to_string();
    move |i: &[u8]| {
        let (i, (_, _, val)) = tuple((tag(name.as_bytes()), space1, bool_val))(i)?;
        let prop = match name.as_str() {
            "exit_if" => Prop::ExitIf(val),
            _ => Prop::Unknown((name.clone(), format!("{}", val))),
        };
        Ok((i, prop))
    }
}

fn prop_number(name: &str) -> impl Fn(&[u8]) -> IResult<&[u8], Prop> {
    let name = name.to_string();
    move |i: &[u8]| {
        let (i, (_, _, val)) = tuple((tag(name.as_bytes()), space1, number_val))(i)?;
        let prop = match name.as_str() {
            "flow" => Prop::Flow(val),
            "volume" => Prop::Volume(val),
            "max_flow_or_pressure_range" => Prop::MaxFlowOrPressureRange(val),
            "exit_flow_under" => Prop::ExitFlowUnder(val),
            "temperature" => Prop::Temperature(val),
            "pressure" => Prop::Pressure(val),
            "exit_flow_over" => Prop::ExitFlowOver(val),
            "exit_pressure_over" => Prop::ExitPressureOver(val),
            "max_flow_or_pressure" => Prop::MaxFlowOrPressure(val),
            "exit_pressure_under" => Prop::ExitPressureUnder(val),
            "seconds" => Prop::Seconds(val),
            "weight" => Prop::Weight(val),
            _ => Prop::Unknown((name.clone(), format!("{}", val))),
        };
        Ok((i, prop))
    }
}

fn prop_enum<E>() -> impl Fn(&[u8]) -> IResult<&[u8], Prop>
where
    E: ParsableEnumProp,
{
    |i: &[u8]| E::parse(i)
}

pub fn prop_string(name: &str) -> impl Fn(&[u8]) -> IResult<&[u8], Prop> {
    let name = name.to_string();
    move |i: &[u8]| {
        let (i, (_, _, val)) = tuple((tag(name.as_bytes()), space1, string_val))(i)?;
        let prop = match name.as_str() {
            "name" => Prop::Name(val),
            _ => Prop::Unknown((name.clone(), val)),
        };
        Ok((i, prop))
    }
}

fn prop(i: &[u8]) -> IResult<&[u8], Prop> {
    alt((
        prop_bool("exit_if"),
        prop_number("flow"),
        prop_number("volume"),
        prop_number("max_flow_or_pressure_range"),
        prop_enum::<TransitionType>(),
        prop_number("exit_flow_under"),
        prop_number("temperature"),
        prop_string("name"),
        prop_number("pressure"),
        prop_enum::<SensorType>(),
        prop_enum::<PumpType>(),
        prop_enum::<ExitType>(),
        prop_number("exit_flow_over"),
        prop_number("exit_pressure_over"),
        prop_number("max_flow_or_pressure"),
        prop_number("exit_pressure_under"),
        prop_number("seconds"),
        prop_number("weight"),
    ))(i)
}

fn props(i: &[u8]) -> IResult<&[u8], Vec<Prop>> {
    separated_list0(multispace1, prop)(i)
}

fn step(i: &[u8]) -> IResult<&[u8], Step> {
    let (i, (_, _, v, _, _)) = tuple((tag("{"), multispace0, props, multispace0, tag("}")))(i)?;
    Ok((i, Step(v)))
}

pub fn steps(i: &[u8]) -> IResult<&[u8], Vec<Step>> {
    separated_list0(multispace0, step)(i)
}

trait ParsableEnumCommand {
    fn parse(i: &[u8]) -> IResult<&[u8], Command>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BeverageType {
    Calibrate,
    Cleaning,
    Espresso,
    Filter,
    Manual,
    Pourover,
    TeaPortafilter,
}

impl TryFrom<&[u8]> for BeverageType {
    type Error = UnexpectedValueError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value = str::from_utf8(value).expect("should be converted");
        let ret = match value.to_lowercase().as_str() {
            "calibrate" => BeverageType::Calibrate,
            "cleaning" => BeverageType::Cleaning,
            "espresso" => BeverageType::Espresso,
            "filter" => BeverageType::Filter,
            "manual" => BeverageType::Manual,
            "pourover" => BeverageType::Pourover,
            "tea" => BeverageType::TeaPortafilter,
            "tea_portafilter" => BeverageType::TeaPortafilter,
            _ => return Err(UnexpectedValueError(value.into())),
        };
        Ok(ret)
    }
}

impl ParsableEnumCommand for BeverageType {
    fn parse(i: &[u8]) -> IResult<&[u8], Command> {
        let (i, (_, _, val)) = tuple((tag("beverage_type"), space1, beverage_type_val))(i)?;
        Ok((i, Command::BeverageType(val)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProfileType {
    Settings1,
    Settings2,
    Settings2A,
    Settings2B,
    Settings2C,
    Settings2C2,
}

impl TryFrom<&[u8]> for ProfileType {
    type Error = UnexpectedValueError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value = str::from_utf8(value).expect("should be converted");
        let ret = match value {
            "settings_1" => ProfileType::Settings1,
            "settings_2" => ProfileType::Settings2,
            "settings_2a" => ProfileType::Settings2A,
            "settings_2b" => ProfileType::Settings2B,
            "settings_2c" => ProfileType::Settings2C,
            "settings_2c2" => ProfileType::Settings2C2,
            _ => return Err(UnexpectedValueError(value.into())),
        };
        Ok(ret)
    }
}

impl ParsableEnumCommand for ProfileType {
    fn parse(i: &[u8]) -> IResult<&[u8], Command> {
        let (i, (_, _, val)) = tuple((tag("settings_profile_type"), space1, profile_type_val))(i)?;
        Ok((i, Command::SettingsProfileType(val)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    AdvancedShot(String),
    Author(String),
    BeverageType(BeverageType),
    EspressoDeclineTime(f32),
    EspressoHoldTime(f32),
    EspressoPressure(f32),
    EspressoTemperature(f32),
    EspressoTemperature0(f32),
    EspressoTemperature1(f32),
    EspressoTemperature2(f32),
    EspressoTemperature3(f32),
    EspressoTemperatureStepsEnabled(bool),
    FinalDesiredShotVolume(f32),
    FinalDesiredShotVolumeAdvanced(f32),
    FinalDesiredShotVolumeAdvancedCountStart(f32),
    FinalDesiredShotWeight(f32),
    FinalDesiredShotWeightAdvanced(f32),
    FlowProfileDecline(f32),
    FlowProfileDeclineTime(f32),
    FlowProfileHold(f32),
    FlowProfileHoldTime(f32),
    FlowProfileMinimumPressure(f32),
    FlowProfilePreinfusion(f32),
    FlowProfilePreinfusionTime(f32),
    MaximumFlow(f32),
    MaximumFlowRange(f32),
    MaximumFlowRangeAdvanced(f32),
    MaximumFlowRangeDefault(f32),
    MaximumPressure(f32),
    MaximumPressureRange(f32),
    MaximumPressureRangeAdvanced(f32),
    MaximumPressureRangeDefault(f32),
    PreinfusionFlowRate(f32),
    PreinfusionGuarantee(bool),
    PreinfusionStopPressure(f32),
    PreinfusionTime(f32),
    PressureEnd(f32),
    ProfileHide(bool),
    ProfileLanguage(String),
    ProfileNotes(String),
    ProfileTitle(String),
    SettingsProfileType(ProfileType),
    TankDesiredWaterTemperature(f32),
    WaterTemperature(f32),
    BeanBrand(String),
    BeanType(String),
    GrinderDoseWeight(f32),
    GrinderModel(String),
    GrinderSetting(String),
    Unknown((String, String)),
}

fn command_bool(name: &str) -> impl Fn(&[u8]) -> IResult<&[u8], Command> {
    let name = name.to_string();
    move |i: &[u8]| {
        let (i, (_, _, val)) = tuple((tag(name.as_bytes()), space1, bool_val))(i)?;
        let cmd = match name.as_str() {
            "espresso_temperature_steps_enabled" => Command::EspressoTemperatureStepsEnabled(val),
            "preinfusion_guarantee" => Command::PreinfusionGuarantee(val),
            "profile_hide" => Command::ProfileHide(val),
            _ => Command::Unknown((name.clone(), format!("{}", val))),
        };
        Ok((i, cmd))
    }
}

fn command_number(name: &str) -> impl Fn(&[u8]) -> IResult<&[u8], Command> {
    let name = name.to_string();
    move |i: &[u8]| {
        let (i, (_, _, val)) = tuple((tag(name.as_bytes()), space1, number_val))(i)?;
        let cmd = match name.as_str() {
            "espresso_decline_time" => Command::EspressoDeclineTime(val),
            "espresso_hold_time" => Command::EspressoHoldTime(val),
            "espresso_pressure" => Command::EspressoPressure(val),
            "espresso_temperature" => Command::EspressoTemperature(val),
            "espresso_temperature_0" => Command::EspressoTemperature0(val),
            "espresso_temperature_1" => Command::EspressoTemperature1(val),
            "espresso_temperature_2" => Command::EspressoTemperature2(val),
            "espresso_temperature_3" => Command::EspressoTemperature3(val),
            "final_desired_shot_volume" => Command::FinalDesiredShotVolume(val),
            "final_desired_shot_volume_advanced" => Command::FinalDesiredShotVolumeAdvanced(val),
            "final_desired_shot_volume_advanced_count_start" => {
                Command::FinalDesiredShotVolumeAdvancedCountStart(val)
            }
            "final_desired_shot_weight" => Command::FinalDesiredShotWeight(val),
            "final_desired_shot_weight_advanced" => Command::FinalDesiredShotWeightAdvanced(val),
            "flow_profile_decline" => Command::FlowProfileDecline(val),
            "flow_profile_decline_time" => Command::FlowProfileDeclineTime(val),
            "flow_profile_hold" => Command::FlowProfileHold(val),
            "flow_profile_hold_time" => Command::FlowProfileHoldTime(val),
            "flow_profile_minimum_pressure" => Command::FlowProfileMinimumPressure(val),
            "flow_profile_preinfusion" => Command::FlowProfilePreinfusion(val),
            "flow_profile_preinfusion_time" => Command::FlowProfilePreinfusionTime(val),
            "maximum_flow" => Command::MaximumFlow(val),
            "maximum_flow_range" => Command::MaximumFlowRange(val),
            "maximum_flow_range_advanced" => Command::MaximumFlowRangeAdvanced(val),
            "maximum_flow_range_default" => Command::MaximumFlowRangeDefault(val),
            "maximum_pressure" => Command::MaximumPressure(val),
            "maximum_pressure_range" => Command::MaximumPressureRange(val),
            "maximum_pressure_range_advanced" => Command::MaximumPressureRangeAdvanced(val),
            "maximum_pressure_range_default" => Command::MaximumPressureRangeDefault(val),
            "preinfusion_flow_rate" => Command::PreinfusionFlowRate(val),
            "preinfusion_stop_pressure" => Command::PreinfusionStopPressure(val),
            "preinfusion_time" => Command::PreinfusionTime(val),
            "pressure_end" => Command::PressureEnd(val),
            "tank_desired_water_temperature" => Command::TankDesiredWaterTemperature(val),
            "water_temperature" => Command::WaterTemperature(val),
            "grinder_dose_weight" => Command::GrinderDoseWeight(val),
            _ => Command::Unknown((name.clone(), format!("{}", val))),
        };
        Ok((i, cmd))
    }
}

fn command_string(name: &str) -> impl Fn(&[u8]) -> IResult<&[u8], Command> {
    let name = name.to_string();
    move |i: &[u8]| {
        let (i, (_, _, val)) = tuple((tag(name.as_bytes()), space1, string_val))(i)?;
        let cmd = match name.as_str() {
            "advanced_shot" => Command::AdvancedShot(val),
            "author" => Command::Author(val),
            "profile_language" => Command::ProfileLanguage(val),
            "profile_notes" => Command::ProfileNotes(val),
            "profile_title" => Command::ProfileTitle(val),
            "bean_brand" => Command::BeanBrand(val),
            "bean_type" => Command::BeanType(val),
            "grinder_model" => Command::GrinderModel(val),
            "grinder_setting" => Command::GrinderSetting(val),
            _ => Command::Unknown((name.clone(), val)),
        };
        Ok((i, cmd))
    }
}

fn beverage_type_val(i: &[u8]) -> IResult<&[u8], BeverageType> {
    map_res(
        alt((
            tag_no_case("calibrate"),
            tag_no_case("cleaning"),
            tag_no_case("espresso"),
            tag_no_case("filter"),
            tag_no_case("manual"),
            tag_no_case("pourover"),
            tag_no_case("tea_portafilter"),
            tag_no_case("tea"),
        )),
        BeverageType::try_from,
    )(i)
}

fn profile_type_val(i: &[u8]) -> IResult<&[u8], ProfileType> {
    map_res(
        alt((
            tag("settings_1"),
            tag("settings_2c2"),
            tag("settings_2a"),
            tag("settings_2b"),
            tag("settings_2c"),
            tag("settings_2"),
        )),
        ProfileType::try_from,
    )(i)
}

fn command_enum<E>() -> impl Fn(&[u8]) -> IResult<&[u8], Command>
where
    E: ParsableEnumCommand,
{
    |i: &[u8]| E::parse(i)
}

fn command(i: &[u8]) -> IResult<&[u8], Command> {
    // NOTE: Nested `alt` combinator was caused by nom's limitation of maximum 21 parsers.
    alt((
        alt((
            command_string("advanced_shot"),
            command_string("author"),
            command_enum::<BeverageType>(),
            command_number("espresso_decline_time"),
            command_number("espresso_hold_time"),
            command_number("espresso_pressure"),
            command_number("espresso_temperature"),
            command_number("espresso_temperature_0"),
            command_number("espresso_temperature_1"),
            command_number("espresso_temperature_2"),
            command_number("espresso_temperature_3"),
            command_bool("espresso_temperature_steps_enabled"),
            command_number("final_desired_shot_volume"),
            command_number("final_desired_shot_volume_advanced"),
            command_number("final_desired_shot_volume_advanced_count_start"),
            command_number("final_desired_shot_weight"),
            command_number("final_desired_shot_weight_advanced"),
            command_number("flow_profile_decline"),
            command_number("flow_profile_decline_time"),
            command_number("flow_profile_hold"),
        )),
        alt((
            command_number("flow_profile_hold_time"),
            command_number("flow_profile_minimum_pressure"),
            command_number("flow_profile_preinfusion"),
            command_number("flow_profile_preinfusion_time"),
            command_number("maximum_flow"),
            command_number("maximum_flow_range"),
            command_number("maximum_flow_range_advanced"),
            command_number("maximum_flow_range_default"),
            command_number("maximum_pressure"),
            command_number("maximum_pressure_range"),
            command_number("maximum_pressure_range_advanced"),
            command_number("maximum_pressure_range_default"),
            command_number("preinfusion_flow_rate"),
            command_bool("preinfusion_guarantee"),
            command_number("preinfusion_stop_pressure"),
            command_number("preinfusion_time"),
            command_number("pressure_end"),
            command_bool("profile_hide"),
            command_string("profile_language"),
            command_string("profile_notes"),
        )),
        alt((
            command_string("profile_title"),
            command_enum::<ProfileType>(),
            command_number("tank_desired_water_temperature"),
            command_number("water_temperature"),
            command_string("bean_brand"),
            command_string("bean_type"),
            command_number("grinder_dose_weight"),
            command_string("grinder_model"),
            command_string("grinder_setting"),
        )),
    ))(i)
}

pub fn profile(i: &[u8]) -> IResult<&[u8], Vec<Command>> {
    separated_list0(multispace0, command)(i)
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};

    use super::*;

    #[test]
    fn test_number_val() {
        assert_eq!(number_val(b"8;"), Ok((&b";"[..], 8.0f32)));
        assert_eq!(number_val(b"80;"), Ok((&b";"[..], 80.0f32)));
        assert_eq!(number_val(b"8.;"), Ok((&b";"[..], 8.0f32)));
        assert_eq!(number_val(b"8.0;"), Ok((&b";"[..], 8.0f32)));
        assert_eq!(number_val(b".8;"), Ok((&b";"[..], 0.8f32)));
    }

    #[test]
    fn test_nested_bracket_string_val() {
        assert_eq!(
            nested_bracket_string_val(b"{} rest"),
            Ok((&b" rest"[..], "".into()))
        );
        assert_eq!(
            nested_bracket_string_val(b"{volume 150.0} rest"),
            Ok((&b" rest"[..], "volume 150.0".into()))
        );
        assert_eq!(
            nested_bracket_string_val(b"{{exit_if 0} {volume 150.0}} rest"),
            Ok((&b" rest"[..], "{exit_if 0} {volume 150.0}".into()))
        );
        assert_eq!(
            nested_bracket_string_val(b"{{exit_if 0} {volume 150.0 name {Hoge}}}"),
            Ok((&b""[..], "{exit_if 0} {volume 150.0 name {Hoge}}".into()))
        );

        assert_eq!(
            nested_bracket_string_val(b"{{exit_if 0} {volume 150.0} unmatched"),
            Err(Err::Error(Error::from_error_kind(
                &b"{exit_if 0} {volume 150.0} unmatched"[..],
                ErrorKind::Fail
            )))
        );
    }

    #[test]
    fn test_string_val() {
        assert_eq!(string_val(b"Fill ;"), Ok((&b" ;"[..], "Fill".into())));
        assert_eq!(string_val(b"Fill\n;"), Ok((&b"\n;"[..], "Fill".into())));
        assert_eq!(
            string_val(b"{Pressure Up};"),
            Ok((&b";"[..], "Pressure Up".into()))
        );
        assert_eq!(
            string_val(b"{New\n\"Line\"\n\nSupported \n};"),
            Ok((&b";"[..], "New\n\"Line\"\n\nSupported \n".into()))
        );
        assert_eq!(
            string_val(b"{{exit_if 0 flow 5.0} {volume 200.0 name {Rust}}};"),
            Ok((
                &b";"[..],
                "{exit_if 0 flow 5.0} {volume 200.0 name {Rust}}".into()
            ))
        );
    }

    #[test]
    fn test_transition_val() {
        assert_eq!(
            transition_val(b"fast;"),
            Ok((&b";"[..], TransitionType::Fast))
        );
        assert_eq!(
            transition_val(b"smooth;"),
            Ok((&b";"[..], TransitionType::Smooth))
        );
        assert_eq!(
            transition_val(b"slow;"),
            Err(nom::Err::Error(Error::new(&b"slow;"[..], ErrorKind::Tag)))
        );
    }

    #[test]
    fn test_prop_bool() {
        let prop_exit_if = prop_bool("exit_if");
        assert_eq!(
            prop_exit_if(b"exit_if 1;"),
            Ok((&b";"[..], Prop::ExitIf(true)))
        );
        assert_eq!(
            prop_exit_if(b"exit_if 0;"),
            Ok((&b";"[..], Prop::ExitIf(false)))
        );
        assert_eq!(
            prop_exit_if(b"exit_if x;"),
            Err(nom::Err::Error(Error::new(&b"x;"[..], ErrorKind::Tag)))
        );
    }

    #[test]
    fn test_prop_enum() {
        let prop_transition = prop_enum::<TransitionType>();
        assert_eq!(
            prop_transition(b"transition fast;"),
            Ok((&b";"[..], Prop::Transition(TransitionType::Fast)))
        );
        assert_eq!(
            prop_transition(b"transition smooth;"),
            Ok((&b";"[..], Prop::Transition(TransitionType::Smooth)))
        );
        assert_eq!(
            prop_transition(b"transition smooooch;"),
            Err(nom::Err::Error(Error::new(
                &b"smooooch;"[..],
                ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn test_prop_string() {
        let prop_name = prop_string("name");
        assert_eq!(
            prop_name(b"name Fill\n"),
            Ok((&b"\n"[..], Prop::Name("Fill".into())))
        );
        assert_eq!(
            prop_name(b"name {Pressure Up}\n"),
            Ok((&b"\n"[..], Prop::Name("Pressure Up".into())))
        );
        assert_eq!(
            prop_name(b"name {}\n"),
            Ok((&b"\n"[..], Prop::Name("".into())))
        );
    }

    #[test]
    fn test_prop() {
        assert_eq!(prop(b"flow 8;"), Ok((&b";"[..], Prop::Flow(8.0))));
        assert_eq!(prop(b"volume 100;"), Ok((&b";"[..], Prop::Volume(100.0))));
        assert_eq!(
            prop(b"exit_pressure_over 1.5;"),
            Ok((&b";"[..], Prop::ExitPressureOver(1.5)))
        );
    }

    #[test]
    fn test_step_inner() {
        let payload = include_str!("../../fixtures/step.inner");
        assert_eq!(
            props(payload.as_bytes()),
            Ok((
                &b".00"[..],
                vec![
                    Prop::ExitIf(true),
                    Prop::Flow(8.0),
                    Prop::Volume(100.0),
                    Prop::MaxFlowOrPressureRange(0.6),
                    Prop::Transition(TransitionType::Fast),
                    Prop::ExitFlowUnder(0.0),
                    Prop::Temperature(94.0),
                    Prop::Name("Fill".into()),
                    Prop::Pressure(2.0),
                    Prop::Sensor(SensorType::Coffee),
                    Prop::Pump(PumpType::Pressure),
                    Prop::ExitType(ExitType::PressureOver),
                    Prop::ExitFlowOver(6.0),
                    Prop::ExitPressureOver(1.5),
                    Prop::MaxFlowOrPressure(0.0),
                    Prop::ExitPressureUnder(0.0),
                    Prop::Seconds(25.0),
                ]
            ))
        );
    }

    #[test]
    fn test_step_outer() {
        let payload = include_str!("../../fixtures/step.outer");
        assert_eq!(
            step(payload.as_bytes()),
            Ok((
                &b""[..],
                Step(vec![
                    Prop::ExitIf(true),
                    Prop::Flow(8.0),
                    Prop::Volume(100.0),
                    Prop::MaxFlowOrPressureRange(0.6),
                    Prop::Transition(TransitionType::Fast),
                    Prop::ExitFlowUnder(0.0),
                    Prop::Temperature(94.0),
                    Prop::Name("Fill".into()),
                    Prop::Pressure(2.0),
                    Prop::Sensor(SensorType::Coffee),
                    Prop::Pump(PumpType::Pressure),
                    Prop::ExitType(ExitType::PressureOver),
                    Prop::ExitFlowOver(6.0),
                    Prop::ExitPressureOver(1.5),
                    Prop::MaxFlowOrPressure(0.0),
                    Prop::ExitPressureUnder(0.0),
                    Prop::Seconds(25.0),
                ])
            ))
        );
    }

    #[test]
    fn test_steps_inner() {
        assert_eq!(
            steps(&b"{volume 100}\n{flow 8}\n"[..]),
            Ok((
                &b"\n"[..],
                vec![
                    Step(vec![Prop::Volume(100.0),]),
                    Step(vec![Prop::Flow(8.0)])
                ]
            ))
        );

        let payload = include_str!("../../fixtures/steps.inner");
        assert_eq!(
            steps(payload.as_bytes()),
            Ok((
                &b"\n"[..],
                vec![
                    Step(vec![
                        Prop::ExitIf(true),
                        Prop::Flow(8.0),
                        Prop::Volume(100.0),
                        Prop::MaxFlowOrPressureRange(0.6),
                        Prop::Transition(TransitionType::Fast),
                        Prop::ExitFlowUnder(0.0),
                        Prop::Temperature(94.0),
                        Prop::Name("Fill".into()),
                        Prop::Pressure(2.0),
                        Prop::Sensor(SensorType::Coffee),
                        Prop::Pump(PumpType::Pressure),
                        Prop::ExitType(ExitType::PressureOver),
                        Prop::ExitFlowOver(6.0),
                        Prop::ExitPressureOver(1.5),
                        Prop::MaxFlowOrPressure(0.0),
                        Prop::ExitPressureUnder(0.0),
                        Prop::Seconds(25.0),
                    ]),
                    Step(vec![
                        Prop::ExitIf(false),
                        Prop::Volume(100.0),
                        Prop::MaxFlowOrPressureRange(0.6),
                        Prop::Transition(TransitionType::Fast),
                        Prop::ExitFlowUnder(0.0),
                        Prop::Temperature(93.0),
                        Prop::Weight(0.0),
                        Prop::Name("Pressure Up".into()),
                        Prop::Pressure(9.0),
                        Prop::Sensor(SensorType::Coffee),
                        Prop::Pump(PumpType::Pressure),
                        Prop::ExitFlowOver(6.0),
                        Prop::ExitPressureOver(11.0),
                        Prop::MaxFlowOrPressure(0.0),
                        Prop::Seconds(4.0),
                        Prop::ExitPressureUnder(0.0),
                    ])
                ]
            ))
        );
    }

    #[test]
    fn test_command() {
        assert_eq!(
            command(b"author Decent"),
            Ok((&b""[..], Command::Author("Decent".into())))
        );
        assert_eq!(
            command(b"author {Decent}"),
            Ok((&b""[..], Command::Author("Decent".into())))
        );

        assert_eq!(
            command(b"beverage_type filter"),
            Ok((&b""[..], Command::BeverageType(BeverageType::Filter)))
        );
        assert_eq!(
            command(b"beverage_type cleaning"),
            Ok((&b""[..], Command::BeverageType(BeverageType::Cleaning)))
        );
        assert_eq!(
            command(b"beverage_type Cleaning"),
            Ok((&b""[..], Command::BeverageType(BeverageType::Cleaning)))
        );
        assert_eq!(
            command(b"beverage_type tea"),
            Ok((
                &b""[..],
                Command::BeverageType(BeverageType::TeaPortafilter)
            ))
        );
        assert_eq!(
            command(b"beverage_type tea_portafilter"),
            Ok((
                &b""[..],
                Command::BeverageType(BeverageType::TeaPortafilter)
            ))
        );
    }

    #[test]
    fn test_profile_file() {
        let payload = include_str!("../../fixtures/profile.tcl");
        assert_eq!(
            profile(payload.as_bytes()),
            Ok((
                &b"\n"[..],
                vec![
                    Command::AdvancedShot(
                        "{exit_if 0 flow 4.0} {temperature 98.00 name {3 mL/s} seconds 60.00}"
                            .into()
                    ),
                    Command::Author("Decent".into()),
                    Command::BeverageType(BeverageType::Pourover),
                    Command::EspressoDeclineTime(0.),
                    Command::EspressoHoldTime(25.0),
                    Command::EspressoPressure(8.6),
                    Command::EspressoTemperature(98.),
                    Command::EspressoTemperature0(90.),
                    Command::EspressoTemperature1(88.),
                    Command::EspressoTemperature2(88.),
                    Command::EspressoTemperature3(88.),
                    Command::EspressoTemperatureStepsEnabled(true),
                    Command::FinalDesiredShotVolume(36.),
                    Command::FinalDesiredShotVolumeAdvanced(0.),
                    Command::FinalDesiredShotVolumeAdvancedCountStart(0.),
                    Command::FinalDesiredShotWeight(100.),
                    Command::FinalDesiredShotWeightAdvanced(100.),
                    Command::FlowProfileDecline(1.2),
                    Command::FlowProfileDeclineTime(17.),
                    Command::FlowProfileHold(2.),
                    Command::FlowProfileHoldTime(8.),
                    Command::FlowProfileMinimumPressure(4.),
                    Command::FlowProfilePreinfusion(4.),
                    Command::FlowProfilePreinfusionTime(5.),
                    Command::MaximumFlow(0.),
                    Command::MaximumFlowRangeAdvanced(1.),
                    Command::MaximumFlowRangeDefault(1.),
                    Command::MaximumPressure(0.),
                    Command::MaximumPressureRangeAdvanced(0.9),
                    Command::MaximumPressureRangeDefault(0.9),
                    Command::PreinfusionFlowRate(4.),
                    Command::PreinfusionStopPressure(4.),
                    Command::PreinfusionTime(0.),
                    Command::PressureEnd(6.),
                    Command::ProfileHide(true),
                    Command::ProfileLanguage("en".into()),
                    Command::ProfileNotes("first line\n\nafter blank line\nlast line".into()),
                    Command::ProfileTitle("Filter 2.1".into()),
                    Command::SettingsProfileType(ProfileType::Settings2C),
                    Command::TankDesiredWaterTemperature(0.),
                ]
            ))
        );
    }
}
