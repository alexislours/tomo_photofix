#![allow(non_snake_case)]

use std::path::Path;

const OUT_DIR: &str = "sd:/tomo_photofix";

const PHOTO_W: usize = 1280;
const BUF_H: usize = 768;
const PHOTO_H: usize = 720;

extern "C" {
    #[link_name = "_ZN2nn5album14SaveScreenshotEPKvmNS0_9ImageSizeENS0_17AlbumReportOptionE"]
    fn SaveScreenshot(image: *const u8, size: usize, image_size: u32, report_option: u32) -> u32;
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &b in bytes {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

fn push_chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let start = out.len();
    out.extend_from_slice(kind);
    out.extend_from_slice(data);
    let crc = crc32(&out[start..]);
    out.extend_from_slice(&crc.to_be_bytes());
}

fn adler32(bytes: &[u8]) -> u32 {
    const MOD: u32 = 65521;
    let (mut a, mut b) = (1u32, 0u32);
    for chunk in bytes.chunks(5552) {
        for &byte in chunk {
            a += byte as u32;
            b += a;
        }
        a %= MOD;
        b %= MOD;
    }
    (b << 16) | a
}

fn zlib_stored(raw: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(raw.len() + raw.len() / 65535 * 5 + 16);
    out.extend_from_slice(&[0x78, 0x01]);
    let mut chunks = raw.chunks(0xFFFF).peekable();
    while let Some(chunk) = chunks.next() {
        let last = chunks.peek().is_none();
        out.push(if last { 1 } else { 0 });
        let len = chunk.len() as u16;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(chunk);
    }
    out.extend_from_slice(&adler32(raw).to_be_bytes());
    out
}

fn encode_png(rgba: &[u8], w: usize, h: usize) -> Vec<u8> {
    let mut raw = Vec::with_capacity(h * (1 + w * 4));
    for y in 0..h {
        raw.push(0);
        raw.extend_from_slice(&rgba[y * w * 4..(y + 1) * w * 4]);
    }
    let idat = zlib_stored(&raw);

    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&(w as u32).to_be_bytes());
    ihdr.extend_from_slice(&(h as u32).to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);

    let mut out = Vec::with_capacity(idat.len() + 64);
    out.extend_from_slice(&[137, 80, 78, 71, 13, 10, 26, 10]);
    push_chunk(&mut out, b"IHDR", &ihdr);
    push_chunk(&mut out, b"IDAT", &idat);
    push_chunk(&mut out, b"IEND", &[]);
    out
}

fn timestamp() -> String {
    let cal = skyline::nn::time::get_calendar_time();
    format!(
        "{:04}{:02}{:02}_{:02}{:02}{:02}",
        cal.year, cal.month, cal.day, cal.hour, cal.minute, cal.second
    )
}

fn next_free_path(ext: &str) -> String {
    let ts = timestamp();
    let path = format!("{}/photo_{}.{}", OUT_DIR, ts, ext);
    if !Path::new(&path).exists() {
        return path;
    }

    let mut n = 0u32;
    loop {
        let path = format!("{}/photo_{}_{:03}.{}", OUT_DIR, ts, n, ext);
        if !Path::new(&path).exists() {
            return path;
        }
        n += 1;
    }
}

#[skyline::hook(replace = SaveScreenshot)]
unsafe fn save_screenshot_hook(
    image: *const u8,
    size: usize,
    image_size: u32,
    report_option: u32,
) -> u32 {
    if image.is_null() || size == 0 {
        println!("[photofix] SaveScreenshot null/empty buffer (size={})", size);
    } else {
        let bytes = std::slice::from_raw_parts(image, size);

        if size == PHOTO_W * BUF_H * 4 {
            let path = next_free_path("png");
            let png = encode_png(bytes, PHOTO_W, PHOTO_H);
            match std::fs::write(&path, &png) {
                Ok(()) => println!(
                    "[photofix] saved {} ({}x{}, {} bytes png)",
                    path, PHOTO_W, PHOTO_H, png.len()
                ),
                Err(e) => println!("[photofix] FAILED to write {}: {:?}", path, e),
            }
        } else {
            let raw_path = next_free_path("rgba");
            let _ = std::fs::write(&raw_path, bytes);
            println!(
                "[photofix] unexpected size {} (expected {}x{}x4); dumped raw to {}",
                size, PHOTO_W, BUF_H, raw_path
            );
        }
    }

    original!()(image, size, image_size, report_option)
}

#[skyline::main(name = "tomo_photofix")]
pub fn main() {
    println!("[photofix] loaded — hooking nn::album::SaveScreenshot, PNGs -> {}", OUT_DIR);
    match std::fs::create_dir_all(OUT_DIR) {
        Ok(()) => println!("[photofix] output dir ready: {}", OUT_DIR),
        Err(e) => println!("[photofix] WARNING: could not create {}: {:?}", OUT_DIR, e),
    }
    skyline::install_hook!(save_screenshot_hook);
}
