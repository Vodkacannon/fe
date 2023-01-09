
pub mod world {
    use crate::area_lights::fe::area_lights::AreaLights;
    use crate::players::players::{Players};
        
    pub(crate) struct World<Player, SpotLights, AreaLight, Objects3D> {
        load: bool,
        objects: Objects3D,
        players: Players<Player>,
        spot_lights: SpotLights,
        area_lights: AreaLights<AreaLight>,
    }

    impl<Player, SpotLights, AreaLight, Objects3D> World<Player, SpotLights, AreaLight, Objects3D> {
        pub(crate) fn new(load: bool, objects: Objects3D, players: Players<Player>, spot_lights: SpotLights, area_lights: AreaLights<AreaLight>) -> Self { Self { load, objects, players, spot_lights, area_lights } }
            
        }
    }