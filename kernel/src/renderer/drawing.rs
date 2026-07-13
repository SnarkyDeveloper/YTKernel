use crate::BootInfo;
use crate::renderer::bitmap::*;

pub unsafe fn draw_char(info: &BootInfo, start_x: u32, start_y: u32, c: char, color: u32, scale: u32) {
    if info.fb_base.is_null() || scale == 0 { return; }

    let font_idx = get_font_index(c);

    for row in 0..8 {
        let row_byte = FONT_8X8[font_idx + row as usize];

        for scale_y in 0..scale {
            let current_y = start_y + (row * scale) + scale_y;
            if current_y >= info.fb_height { break; }

            let row_start: *mut u32 = info.fb_base.add((current_y * info.fb_stride) as usize);

            for col in 0..8 {
                let bit_mask = 0x80 >> col;
                if (row_byte & bit_mask) != 0 {
                    
                    for scale_x in 0..scale {
                        let current_x = start_x + (col * scale) + scale_x;
                        if current_x >= info.fb_width { break; }

                        *row_start.add(current_x as usize) = color;
                    }
                }
            }
        }
    }
}



pub unsafe fn draw_string(info: &BootInfo, start_x: u32, mut y: u32, s: &str, color: u32, scale: u32) {
    if info.fb_base.is_null() || scale == 0 { return; }

    let mut x = start_x;
    let char_size = 8 * scale;

    for c in s.chars() {
        if c == '\n' {
            x = start_x; 
            y += char_size + 5; 
            continue;          
        }

        if c == '\r' {
            x = start_x;
            continue;
        }

        if x + char_size > info.fb_width {
            x = start_x;
            y += char_size;
        }

        if y + char_size > info.fb_height {
            break;
        }

        unsafe { draw_char(info, x, y, c, color, scale); }
        
        x += char_size;
    }
}

