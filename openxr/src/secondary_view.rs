use crate::*;

pub struct SecondaryViewState {
    pub ty: ViewConfigurationType,
    pub active: bool,
}

pub struct SecondaryEndInfo<'a, 'b, 'c, G: Graphics> {
    pub ty: ViewConfigurationType,
    pub environment_blend_mode: EnvironmentBlendMode,
    pub layers: &'a [&'b CompositionLayer<'c, G>],
}
