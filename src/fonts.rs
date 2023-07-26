use fontdue::Font;

use crate::vec3::Vec3;

pub const CHARACTER_PX: usize = 14;

#[derive(Debug, Clone)]
pub struct RasterizedCharacter {
    pub bitmap: [u8; CHARACTER_PX * CHARACTER_PX],
}

pub fn rasterize_alphabet() -> [Option<RasterizedCharacter>; 128] {
    let mut alphabet: [Option<RasterizedCharacter>; 128] = std::array::from_fn(|_| None);
    let font = include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8];
    let font = Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

    for c in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz.".as_bytes() {
        let (metrics, bitmap) = font.rasterize(*c as char, CHARACTER_PX as f32);
        let mut character = [0; CHARACTER_PX * CHARACTER_PX];
        let x_pos = (CHARACTER_PX - metrics.width) / 2;
        let y_pos = CHARACTER_PX - 1;

        for y in 0..metrics.height {
            for x in 0..metrics.width {
                character[x_pos + x + (y_pos - y) * CHARACTER_PX] = bitmap[x + (metrics.height - 1 - y) * metrics.width];
            }
        }     
        alphabet[*c as usize] = Some(RasterizedCharacter{
            bitmap: character,
        });     
    }
    alphabet
}

pub fn render_string(s: &str, alphabet: &[Option<RasterizedCharacter>; 128]) -> Vec<RasterizedCharacter> {
    let mut ret = Vec::with_capacity(s.len());
    for c in s.as_bytes() {
        if let Some(character) = alphabet[*c as usize].clone() {
            ret.push(character.clone());
        }
    }
    ret
}


// using this cause lower performance than just copying the code and using it. even with inline_always
// #[inline(always)]
// pub fn draw_string(s: &str, alphabet: &[Option<RasterizedCharacter>; 128], framebuffer: &mut Vec<Vec3<u8>>, window_width: u32, x_pos: usize, y_pos: usize) {
//     let mut x_pos = x_pos;
//     let string = render_string(s, &alphabet);
//     for c in string {
//         for x in 0..CHARACTER_PX {
//             for y in 0..CHARACTER_PX {
//                 let gray_pixel = c.bitmap[(x + y * CHARACTER_PX) as usize];
//                 if gray_pixel == 0 {
//                     continue;
//                 }
//                 framebuffer[(x_pos + x + (y_pos + y) * window_width as usize) as usize] = Vec3::new(gray_pixel, gray_pixel, gray_pixel);
//             }
//         }
//         x_pos += CHARACTER_PX;
//     }    
// }
#[macro_export]
macro_rules! draw_string{
    // macth like arm for macro
       ($s:expr, $alphabet:expr, $framebuffer:expr, $window_width:expr, $x_pos:expr, $y_pos:expr)=>{
    // macro expand to this code
           {
   // $a and $b will be templated using the value/variable provided to macro
                let mut x_pos = $x_pos;
                let string = render_string($s, &$alphabet);
                for c in string {
                    for x in 0..CHARACTER_PX {
                        for y in 0..CHARACTER_PX {
                            let gray_pixel = c.bitmap[(x + y * CHARACTER_PX) as usize];
                            if gray_pixel == 0 {
                                continue;
                            }
                            $framebuffer[(x_pos + x + ($y_pos + y) * $window_width as usize) as usize] = Vec3::new(gray_pixel, gray_pixel, gray_pixel);
                        }
                    }
                    x_pos += CHARACTER_PX;
                }    
           }
       }
   }