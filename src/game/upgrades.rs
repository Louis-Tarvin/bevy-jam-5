use bevy::{ecs::component::StorageType, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Upgrades>();
    app.init_resource::<Upgrades>();
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct Upgrades {
    pub mining_speed: u8,
    pub fire_rate: u8,
}

#[derive(Clone, Debug)]
pub enum UpgradeType {
    MiningSpeed,
    FireRate,
}

pub struct Upgrade(pub UpgradeType);
impl Component for Upgrade {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let upgrade_type = world.get::<Upgrade>(entity).unwrap().0.clone();
            let mut upgrades = world.resource_mut::<Upgrades>();

            match upgrade_type {
                UpgradeType::MiningSpeed => upgrades.mining_speed += 1,
                UpgradeType::FireRate => upgrades.fire_rate += 1,
            }
        });

        hooks.on_remove(|mut world, entity, _component_id| {
            let upgrade_type = world.get::<Upgrade>(entity).unwrap().0.clone();
            let mut upgrades = world.resource_mut::<Upgrades>();

            match upgrade_type {
                UpgradeType::MiningSpeed => upgrades.mining_speed -= 1,
                UpgradeType::FireRate => upgrades.fire_rate -= 1,
            }
        });
    }
}
