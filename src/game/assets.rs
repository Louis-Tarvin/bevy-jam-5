use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();

    app.register_type::<HandleMap<ObjectKey>>();
    app.init_resource::<HandleMap<ObjectKey>>();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    SpinnerCore,
    SpinnerFrame,
    Waypoint,
    Title,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                ImageKey::SpinnerCore,
                asset_server.load_with_settings(
                    "images/spinner_core.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::default();
                    },
                ),
            ),
            (
                ImageKey::SpinnerFrame,
                asset_server.load_with_settings(
                    "images/spinner_frame.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::default();
                    },
                ),
            ),
            (
                ImageKey::Waypoint,
                asset_server.load_with_settings(
                    "images/waypoint.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::default();
                    },
                ),
            ),
            (
                ImageKey::Title,
                asset_server.load_with_settings(
                    "images/title.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::default();
                    },
                ),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    Shoot,
    Explode,
    Build,
    Collect,
}

impl AssetKey for SfxKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SfxKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SfxKey::ButtonHover,
                asset_server.load("audio/sfx/hover.ogg"),
            ),
            (
                SfxKey::ButtonPress,
                asset_server.load("audio/sfx/select.ogg"),
            ),
            (SfxKey::Shoot, asset_server.load("audio/sfx/shoot.ogg")),
            (
                SfxKey::Explode,
                asset_server.load("audio/sfx/explosion.ogg"),
            ),
            (SfxKey::Build, asset_server.load("audio/sfx/build.ogg")),
            (SfxKey::Collect, asset_server.load("audio/sfx/collect.ogg")),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            SoundtrackKey::Gameplay,
            asset_server.load("audio/soundtracks/bgm_main.ogg"),
        )]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ObjectKey {
    ShipBody,
    ShipTurret,
    MiningShip,
    Asteroid,
    Decoy,
    Enemy,
    Station,
    Upgrade,
}

impl AssetKey for ObjectKey {
    type Asset = Scene;
}

impl FromWorld for HandleMap<ObjectKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                ObjectKey::ShipBody,
                asset_server.load("objects/ship.glb#Scene0"),
            ),
            (
                ObjectKey::ShipTurret,
                asset_server.load("objects/ship.glb#Scene1"),
            ),
            (
                ObjectKey::MiningShip,
                asset_server.load("objects/ship.glb#Scene2"),
            ),
            (
                ObjectKey::Asteroid,
                asset_server.load("objects/asteroid.glb#Scene0"),
            ),
            (
                ObjectKey::Decoy,
                asset_server.load("objects/decoy.glb#Scene0"),
            ),
            (
                ObjectKey::Enemy,
                asset_server.load("objects/enemy.glb#Scene0"),
            ),
            (
                ObjectKey::Station,
                asset_server.load("objects/wheel.glb#Scene0"),
            ),
            (
                ObjectKey::Upgrade,
                asset_server.load("objects/upgrade.glb#Scene0"),
            ),
        ]
        .into()
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
