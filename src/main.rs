use ab_glyph::{FontRef, PxScale};
use imageproc::image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rand::Rng;

fn main() {
	let render_me = "Hello world!\nfoobar";
	let padding: i32 = 32; // pixels
    let height = 360.0;
    let scale = PxScale {
        x: height,
        y: height,
    };
	let color = Rgba([0u8, 0u8, 255u8, 255u8]);



	let mut rng = rand::thread_rng();
	let fonts = [
		//("dejavu", FontRef::try_from_slice(include_bytes!("DejaVuSans.ttf")).unwrap()),
		("2peas", FontRef::try_from_slice(include_bytes!("2PeasGift-Jp8B.ttf")).unwrap()),
		("fanzine", FontRef::try_from_slice(include_bytes!("Fanzine-og8V.ttf")).unwrap()), // has blank empty at bottom
		("ransom", FontRef::try_from_slice(include_bytes!("Ransom4.ttf")).unwrap())
	];

	let mut lines: Vec<&str> = render_me.split('\n').collect();
	let mut font_indices: Vec<Vec<u8>> = Vec::new(); // which font for each character
	let mut widths: Vec<u32> = Vec::new(); 
	for line in &lines {
		let mut line_indices: Vec<u8> = Vec::new();
		let mut line_width = 0;
		for c in line.chars() {
			let font_index: usize = rng.gen_range(0..fonts.len());
			line_indices.push(font_index as u8);
			line_width += text_size(scale, &fonts[font_index].1, &c.to_string()).0;
		}
		font_indices.push(line_indices);
		widths.push(line_width);
	}
	println!("{:?}\n{:?}\n{:?}", &lines, &font_indices, &widths);

	let h = ((height+1.0) as i32 * lines.len() as i32) + (padding * 2);	
	let canvas_width: u32 = match widths.iter().max() {
		Some(w) => *w + (padding as u32 * 2),
		None => panic!("no width"),
	};
	let mut image = RgbaImage::new(
		canvas_width, 
		u32::try_from(h).unwrap()
	);


	let (mut x, mut y) = (padding, padding);
	for i in 0..lines.len() {
		let line = lines[i];
		x = padding + ((canvas_width - widths[i])/2) as i32;
		for j in 0..line.len() {
			let char_as_string = line.chars().nth(j).unwrap().to_string();
			let font = &fonts[font_indices[i as usize][j as usize] as usize].1;
			draw_text_mut(
				&mut image,
				color,x, y, scale,
				&font,
				&char_as_string
			);
			x+=text_size(scale, &font, &char_as_string).0 as i32;
		}
		y+=height as i32;
	}
	println!("Text size: {}x{}", canvas_width, h);
	image.save(format!("outputs/greet.png")).unwrap();
	
	
}