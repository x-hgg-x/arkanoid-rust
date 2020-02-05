use amethyst::input::BindingTypes;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum AxisBinding {
    Paddle,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum ActionBinding {
    ReleaseBall,
    BallAttraction,
}

#[derive(Debug)]
pub struct ArkanoidBindings;

impl BindingTypes for ArkanoidBindings {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}
