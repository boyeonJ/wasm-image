use wasm_bindgen::prelude::*;

/// RGBA 바이트 배열(Uint8ClampedArray와 동일 레이아웃)을 입력받아
/// 그레이스케일 결과를 새로 반환
#[wasm_bindgen]
pub fn grayscale(input: &[u8]) -> Vec<u8> {
    let mut out = input.to_vec();
    for px in out.chunks_mut(4) {
        // 정수 근사: 0.299, 0.587, 0.114 ≈ 38/128, 75/128, 15/128
        let y = (px[0] as u32 * 38 + px[1] as u32 * 75 + px[2] as u32 * 15) >> 7;
        let y8 = y as u8;
        px[0] = y8;
        px[1] = y8;
        px[2] = y8;
        // alpha(px[3])는 그대로 유지
    }
    out
}

/// 아주 단순한 3x3 박스 블러(경계는 유효 픽셀만 평균)
#[wasm_bindgen]
pub fn box_blur3(input: &[u8], width: u32, height: u32) -> Vec<u8> {
    let (w, h) = (width as usize, height as usize);
    let mut out = input.to_vec();
    for y in 0..h {
        for x in 0..w {
            let mut sum_r = 0u32;
            let mut sum_g = 0u32;
            let mut sum_b = 0u32;
            let mut sum_a = 0u32;
            let mut count = 0u32;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < w as isize && ny >= 0 && ny < h as isize {
                        let idx = ((ny as usize) * w + nx as usize) * 4;
                        sum_r += input[idx] as u32;
                        sum_g += input[idx + 1] as u32;
                        sum_b += input[idx + 2] as u32;
                        sum_a += input[idx + 3] as u32;
                        count += 1;
                    }
                }
            }

            let idx = (y * w + x) * 4;
            out[idx]     = (sum_r / count) as u8;
            out[idx + 1] = (sum_g / count) as u8;
            out[idx + 2] = (sum_b / count) as u8;
            out[idx + 3] = (sum_a / count) as u8;
        }
    }
    out
}
