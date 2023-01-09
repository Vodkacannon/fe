pub mod particle_system {
    use particle;

    #[repr(C)]
    pub struct ParticleSystem {
        should_render: bool,
        has_gravity: bool,
        buffer: Vec<Particle>
    }

    impl ParticleSystem {
        fn new(should_render: bool, has_gravity: bool, buffer: Vec<Particle>) {
            ParticleSystem { should_render: should_render, has_gravity: has_gravity,  buffer: buffer };
        }
    }
}