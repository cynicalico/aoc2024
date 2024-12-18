pub fn get_neighbors(
    pos: (usize, usize),
    w: usize,
    h: usize,
    offsets: &[(i32, i32)],
) -> Vec<(usize, usize)> {
    offsets
        .iter()
        .flat_map(|(dy, dx)| {
            let new_pos = (pos.0 as i32 + dy, pos.1 as i32 + dx);
            (new_pos.0 >= 0 && new_pos.0 < h as i32 && new_pos.1 >= 0 && new_pos.1 < w as i32)
                .then_some((new_pos.0 as usize, new_pos.1 as usize))
        })
        .collect()
}

pub fn get_neighbors_4(pos: (usize, usize), w: usize, h: usize) -> Vec<(usize, usize)> {
    get_neighbors(pos, w, h, &[(-1, 0), (1, 0), (0, -1), (0, 1)])
}

pub fn get_neighbors_8(pos: (usize, usize), w: usize, h: usize) -> Vec<(usize, usize)> {
    get_neighbors(
        pos,
        w,
        h,
        &[
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ],
    )
}
