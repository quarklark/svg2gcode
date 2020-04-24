/// TODO: Documentation

use crate::code::*;

// TODO: Documentation
// Generic machine state simulation, assuming nothing is known about the machine when initialized.
pub struct Machine {
    tool_state: Option<Tool>,
    distance_mode: Option<Distance>,
    pub tool_on_action: Vec<GCode>,
    pub tool_off_action: Vec<GCode>,
}

// TODO: Documentation
// Assigns reasonable default settings that apply to most gcode applications.
impl Default for Machine {
    fn default() -> Self {
        Self {
            tool_state: None,
            distance_mode: None,
            tool_on_action: vec![
                GCode::LinearInterpolation {
                    x: None,
                    y: None,
                    z: Some(-1.0),
                    f: Some(8000.0),
                }
            ],
            tool_off_action: vec![
                GCode::LinearInterpolation {
                    x: None,
                    y: None,
                    z: Some(1.0),
                    f: Some(3000.0),
                }
            ],
        }
    }
}

// TODO: Documentation
// Implements the state machine functions to export Gcode.
impl Machine {
    // Outputs gcode to turn the tool on.
    // Args:
    // - tool_on_power (f64): 0.0 to 1.0 float that specifies the percentage of max depth to output
    // as a negative z value.
    pub fn tool_on(&mut self, tool_on_power: f64) -> Vec<GCode> {
        if self.tool_state == Some(Tool::Off) || self.tool_state == None {
            self.tool_state = Some(Tool::On);
            // Using a hard-coded version of tool_on to support dynamic z-depth
            // Here's the old version:
            // self.tool_on_action.clone()
            vec![
                GCode::LinearInterpolation {
                    x: None,
                    y: None,
                    // Z height is a range, with a minimum offset.
                    // This is a quick and dirty way of setting it, and you'll
                    // need to re-compile each time if you want to change
                    // settings here. 
                    //
                    // The first negative item is the max depth the tool
                    // will go, if given a fully black stroke line.
                    //
                    // The last item subtracted is the minimum depth the
                    // toolbit will enter the piece when turned on.
                    // This happens with a completely white line.
                    z: Some(-0.8 * tool_on_power - 0.2),
                    f: Some(8000.0),
                }
            ]
        } else {
            vec![]
        }
    }

    // Outputs gcode to turn the tool off.
    pub fn tool_off(&mut self) -> Vec<GCode> {
        if self.tool_state == Some(Tool::On) || self.tool_state == None {
            self.tool_state = Some(Tool::Off);
            self.tool_off_action.clone()
        } else {
            vec![]
        }
    }

    // Outputs gcode for how distance should be measured: relative or absolute.
    pub fn distance(&mut self, is_absolute: bool) -> Vec<GCode> {
        if is_absolute {
            self.absolute()
        } else {
            self.incremental()
        }
    }

    // Outputs gcode command to use absolute motion
    pub fn absolute(&mut self) -> Vec<GCode> {
        if self.distance_mode == Some(Distance::Incremental) || self.distance_mode == None {
            self.distance_mode = Some(Distance::Absolute);
            vec![GCode::DistanceMode(Distance::Absolute)]
        } else {
            vec![]
        }
    }

    // Outputs gcode command to use relative motion
    pub fn incremental(&mut self) -> Vec<GCode> {
        if self.distance_mode == Some(Distance::Absolute) || self.distance_mode == None {
            self.distance_mode = Some(Distance::Incremental);
            vec![GCode::DistanceMode(Distance::Incremental)]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_on() {
        panic!("TODO: basic passing test");
    }

    #[test]
    fn test_tool_off() {
        panic!("TODO: basic passing test");
    }

    #[test]
    fn test_distance() {
        panic!("TODO: basic passing test");
    }

    #[test]
    fn test_absolute() {
        panic!("TODO: basic passing test");
    }

    #[test]
    fn test_incremental() {
        panic!("TODO: basic passing test");
    }
}
