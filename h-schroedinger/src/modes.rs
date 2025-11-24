use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ViewMode {
    Graphs2D,
    Orbitals3D,
    EnergyLevels2D,
}
