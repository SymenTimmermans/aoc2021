/// Our input is an enhancement algorithm string, and an image.
/// The enhancement algorithm is interpreted as a 512-length array of bits.
type Algorithm = [bool; 512];

/// the string contains bits that are 0 or 1, these are represented as "." or "#".
/// Each char in the string becomes a bit in the array.
fn read_algorithm(input: &str) -> Algorithm {
    let mut algo = [false; 512];
    input
        .chars()
        .enumerate()
        .for_each(|(i, c)| algo[i] = c == '#');
    algo
}

/// The image is a 2D array of pixels.
type Image = Vec<Vec<bool>>;

fn read_image(txt: &str) -> Image {
    let mut image = Vec::new();
    let mut row = Vec::new();
    for line in txt.lines() {
        for c in line.chars() {
            match c {
                '.' => row.push(false),
                '#' => row.push(true),
                _ => panic!("Invalid character in image: {}", c),
            }
        }
        image.push(row);
        row = Vec::new();
    }
    image
}

fn add_margin(image: &Image, margin: usize) -> Image {
    // determine the size of the source image
    let (width, height) = (image[0].len(), image.len());
    // determine the size of the new image
    let new_width = width + 2 * margin;
    let new_height = height + 2 * margin;
    // create the new image
    let mut new_image = vec![vec![false; new_width]; new_height];
    // copy the old image into the new image, centered
    for y in 0..height {
        for x in 0..width {
            new_image[y + margin][x + margin] = image[y][x];
        }
    }
    // return new_image
    new_image
}

fn remove_margin(image: &Image, margin: usize) -> Image {
    // determine the size of the source image
    let (width, height) = (image[0].len(), image.len());
    // determine the size of the new image
    let new_width = width - 2 * margin;
    let new_height = height - 2 * margin;
    // create the new image
    let mut new_image = vec![vec![false; new_width]; new_height];
    // copy the old image into the new image, centered
    for y in 0..new_height {
        for x in 0..new_width {
            new_image[y][x] = image[y + margin][x + margin];
        }
    }
    // return new_image
    new_image
}

fn read_input(s: &str) -> (Algorithm, Image) {
    // split the string on the empty line
    let (a, i) = s.split_once("\n\n").unwrap();
    let algo = read_algorithm(a);
    let image = read_image(i);
    (algo, image)
}

/// The mask value is determined by reading the nine pixels around and including the current pixel.
/// So the pixels from x-1,y-1 to x+1,y+1 are read.
/// If we read outside the image, consider those pixels to be 0.
fn mask_value(image: &Image, x: i32, y: i32) -> usize {
    let mut value = 0;
    for yy in y - 1..y + 2 {
        for xx in x - 1..x + 2 {
            if xx >= 0 && xx < image[0].len() as i32 && yy >= 0 && yy < image.len() as i32 {
                // count the index in in the 3x3 square we are reading
                let index = (yy - y + 1) * 3 + (xx - x + 1);
                value += if image[yy as usize][xx as usize] {
                    1 << (8 - index)
                } else {
                    0
                };
            }
        }
    }
    value as usize
}

fn apply_algorithm(algo: &Algorithm, image: &Image) -> Image {
    // determine the size of the image
    let (width, height) = (image[0].len(), image.len());

    // create the new image to hold the new pixels
    let mut new_image = vec![vec![false; width]; height];

    // iterate over the image, applying the algorithm
    for y in 0..height {
        for x in 0..width {
            let value = mask_value(image, x as i32, y as i32);
            new_image[y][x] = algo[value];
        }
    }
    new_image
}

/// Count the number of pixels that are on in the image.
fn lit_pixels(image: &Image) -> usize {
    image
        .iter()
        .map(|row| row.iter().filter(|&&p| p).count())
        .sum()
}

/// Print the image to the screen
fn print_image(image: &Image) {
    for row in image {
        for pixel in row {
            print!("{}", if *pixel { '#' } else { '.' });
        }
        println!();
    }
}

fn main() {
    // read the algorithm and image from day20.txt
    let (algo, orig_image) = read_input(include_str!("../../input/day20.txt"));

    // add a margin to the image
    let image = add_margin(&orig_image, 2);

    // apply the algorithm to the image
    let image = apply_algorithm(&algo, &image);

    // apply the algorithm one more time
    let image = apply_algorithm(&algo, &image);

    // shrink to compensate for edge cases:
    let image = remove_margin(&image, 1);

    // count the number of pixels that are on in the image
    let lit = lit_pixels(&image);

    // print how many pixels are on
    println!("Part 1: {} pixels are lit", lit);

    // Part 2
    // Apply the algorithm 50 times.
    // --------------------------------------------------

    // first grow the image to allow for a size increase of 50
    // the problem is, edge cases can grow inward and still influence the picture after 50 iterations
    // so we need some kind of a safe distance. Maybe like 104 pixels.
    let mut image = add_margin(&orig_image, 104);

    // apply the algorithm 50 times
    for _ in 0..50 {
        image = apply_algorithm(&algo, &image);
    }

    // now we need to shrink it again so much that we can't have any artifacts of edge cases.
    // 52 should be enough
    let image = remove_margin(&image, 52);

    // count the number of pixels that are on in the image
    let lit = lit_pixels(&image);

    // print how many pixels are on
    println!("Part 2: {} pixels are lit", lit);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_read_algorithm() {
        // take input from the file, and read it into the algorithm
        let (algorithm, image) = read_input(include_str!("../../input/day20_ex.txt"));

        // check that the algorithm is correct
        assert!(!algorithm[0]);
        assert!(!algorithm[1]);
        assert!(algorithm[2]);
        assert!(!algorithm[3]);
        assert!(algorithm[4]);

        // assert that the image is 5x5
        assert_eq!(image.len(), 5);
        assert_eq!(image[0].len(), 5);

        // assert that pixel 0,0 = true
        assert!(image[0][0]);

        // assert that pixel 2,2 = false
        assert!(!image[2][2]);
    }

    #[test]
    fn test_grow_image() {
        let margin = 5;

        // make a small image of 1x1
        let image = vec![vec![true]];

        // add a margin
        let image = add_margin(&image, margin);

        // assert that the image is now twice the MARGIN + 1
        assert_eq!(image.len(), margin * 2 + 1);

        // pixel 0,0 should be false
        assert!(!image[0][0]);

        // pixel MARGIN,MARGIN should be true
        assert!(image[margin][margin]);
    }

    #[test]
    fn test_shrink_image() {
        let grow = 10;
        let shrink = 5;

        // make a small image of 1x1
        let image = vec![vec![true]];

        // add a margin
        let image = add_margin(&image, grow);

        // assert that the image is now twice the MARGIN + 1
        assert_eq!(image.len(), grow * 2 + 1);

        // remove the margin
        let image = remove_margin(&image, shrink);

        // assert the image is now 1 + 2 * (grow - shrink)
        assert_eq!(image.len(), 1 + 2 * (grow - shrink));

        // middle pixel should be true
        assert!(image[grow - shrink][grow - shrink]);
    }

    #[test]
    fn test_mask_value() {
        let (algo, image) = read_input(include_str!("../../input/day20_ex.txt"));

        // get the mask value of the middle pixel
        let mask = mask_value(&image, 2, 2);
        // according to the example, the mask should be 34
        assert_eq!(mask, 34);

        // check the algorithm at that position to get the new value of the pixel.
        let new_value = algo[mask];

        // new value should be true
        assert!(new_value);
    }

    #[test]
    fn test_grow_and_apply() {
        // read the input
        let (algo, image) = read_input(include_str!("../../input/day20_ex.txt"));

        // add a margin
        let image = add_margin(&image, 2);

        // print the new image
        print_image(&image);

        // apply the algorithm and get a new image
        let new_image = apply_algorithm(&algo, &image);

        // print the new image
        print_image(&new_image);

        // apply the algorithm to the new image
        let new_image = apply_algorithm(&algo, &new_image);

        // print the new image
        print_image(&new_image);

        // assert that 35 pixels are lit in the new image
        assert_eq!(lit_pixels(&new_image), 35);
    }

    #[test]
    fn test_algo_fifty_times() {
        // read the input
        let (algo, image) = read_input(include_str!("../../input/day20_ex.txt"));

        // add a margin, accounting for enough space to apply the algorithm 50 times
        let mut image = add_margin(&image, 50);

        // apply the algorithm 50 times
        for _ in 0..50 {
            let new_image = apply_algorithm(&algo, &image);
            image = new_image;
        }

        // print the new image
        print_image(&image);

        // assert that 35 pixels are lit in the new image
        assert_eq!(lit_pixels(&image), 3351);
    }
}
