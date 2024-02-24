extern crate ncurses;

use ncurses::*;
use rand::Rng;

const PROB_DRIP_SPAWN: u32 = 65;
const PROB_DIM: u32 = 55;
const PROB_CHANGE: u32 = 95;
const PRINTABLE_CHARACTERS: u8 = 92;
const MAXX: usize = 160;
const MAXY: usize = 50;
const MAX_INTENSITY: u8 = 13;
const MIN_INTENSITY: u8 = 4;
const NUM_DRIPS: usize = 250;

#[derive(Default, Clone, Copy, Debug)]
struct Cell {
    char_value: u8,
    intensity: u8,
}

#[derive(Default, Clone, Copy, Debug)]
struct Drip {
    x: i32,
    y: i32,
    live: bool,
    bright: u8,
}


fn rnd(n: i32) -> i32 {
    rand::thread_rng().gen_range(1..=n)
}

fn rnd_bin() -> u8 {
    rand::thread_rng().gen_range(0..=1)
}

fn rnd_char() -> u8 {
    33 + rand::thread_rng().gen_range(1..=PRINTABLE_CHARACTERS)
}

fn add_drips(drips: &mut [Drip; NUM_DRIPS]) {
    for drip in drips {
        if !drip.live {
            drip.x = rnd(MAXX as i32) - 1;
            drip.y = rnd(MAXY as i32) - 1; 
            drip.live = true;
            drip.bright = rnd_bin();
        }
    }
}

fn update_drips(drips: &mut [Drip; NUM_DRIPS], matrix: &mut [[Cell; MAXY]; MAXX]) {
    for drip in drips {
        if drip.live {
            if drip.bright == 1 {
                matrix[drip.x as usize][drip.y as usize].intensity = MAX_INTENSITY;
            } else {
                matrix[drip.x as usize][drip.y as usize].intensity = MIN_INTENSITY;
            }
            drip.y += 1;
            if drip.y >= MAXY as i32 {
                drip.live = false;
            }
        }
    }
}

fn fade_n_change_matrix(matrix: &mut [[Cell; MAXY]; MAXX]) {
    for row in matrix {
        for cell in row {
            if rnd(100) < PROB_CHANGE as i32 && cell.char_value == 0 {
                cell.char_value = rnd_char();
            }
            if rnd(100) < PROB_DIM as i32 {
                if cell.intensity > 0 {
                    cell.intensity -= 1;
                }
            }
        }
    }
}

fn matrix_update(drips: &mut [Drip; NUM_DRIPS], matrix: &mut [[Cell; MAXY]; MAXX]) {
    if rnd(100) < PROB_DRIP_SPAWN as i32 {
        add_drips(drips);
    }
    update_drips(drips, matrix);
    fade_n_change_matrix(matrix);
}

fn show_matrix(matrix: [[Cell; MAXY]; MAXX]) {
    const COLOR_MAP: [i16; 14] = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 4, 5, 3, 6];  
    for x in 0..MAXX {
        for y in 0..MAXY {
            let intensity: usize = matrix[x][y].intensity as usize;
            color_set(COLOR_MAP[intensity]);
            mvaddch(y as i32, x as i32, matrix[x][y].char_value.into());
        }
    }
    refresh();
}

fn set_colors() {
    for n in 0..8 {
        init_pair(n+1, n, COLOR_BLACK);
    }
    for n in 0..5 {
        init_color(n, 0, n * 200, 0);
    }
    init_color(6, 800, 1000, 800);
}

fn main() {
    let mut matrix = [[Cell::default(); MAXY]; MAXX];
    let mut drips = [Drip::default(); NUM_DRIPS];
    let ms = std::time::Duration::from_millis(60);

    initscr();
    start_color();
    set_colors();
    loop {
        matrix_update(&mut drips, &mut matrix);
        show_matrix(matrix);
        std::thread::sleep(ms)
    }
}
