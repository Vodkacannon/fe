use fe::player::Player;
use fe::rgb::Rgb;
use fe::audio_sample::audio_sample::AudioSample;
use fe::texture_2d::Texture2D;

fn main() {
   let color = Rgb::new(10, 10, 10);
   println!("{}", color.to_str());
   let test: u128 = u128::MAX;
   let p = Player::new("0000::0000::0000::0000".to_string(), true, "hi".to_string());
   let my_texture = Texture2D::new(false);
}