pub mod day_9 {
    use std::collections::VecDeque;

    pub fn flood_fill3(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        if image[sr as usize][sc as usize] == color {
            return image;
        }
        let mut image_clone = image.clone();
        let old_color = image_clone[sr as usize][sc as usize];
        fill(&mut image_clone, sr, sc, old_color, color);
        image_clone
    }
    fn fill(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, old_color: i32, new_color: i32) {
        if sc < 0
            || sr < 0
            || sr >= image.len() as i32
            || sc >= image[sr as usize].len() as i32
            || image[sr as usize][sc as usize] != old_color
        {
            return;
        }
        image[sr as usize][sc as usize] = new_color;
        fill(image, sr + 1, sc, old_color, new_color);
        fill(image, sr - 1, sc, old_color, new_color);
        fill(image, sr, sc + 1, old_color, new_color);
        fill(image, sr, sc - 1, old_color, new_color);
    }

    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((sr, sc));
        let mut new_image: Vec<Vec<i32>> = image.clone();
        let c0 = image[sr as usize][sc as usize];
        if c0 != color {
            while let Some((sr, sc)) = q.pop_front() {
                if new_image[sr as usize][sc as usize] == c0 {
                    new_image[sr as usize][sc as usize] = color;
                    for delta in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let new_r: i32 = sr + delta.0;
                        let new_c: i32 = sc + delta.1;
                        if (0..new_image.len() as i32).contains(&new_r)
                            && (0..new_image[0].len() as i32).contains(&new_c)
                        {
                            q.push_back((new_r, new_c));
                        }
                    }
                }
            }
        }
        new_image
    }

    pub fn flood_fill2(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        fn flood_fill_dfs(
            image: &Vec<Vec<i32>>,
            row: i32,
            col: i32,
            new_color: i32,
            old_color: i32,
            result: &mut Vec<Vec<i32>>,
        ) {
            if row < 0 || row > image.len() as i32 - 1 || col < 0 || col > image[0].len() as i32 - 1
            {
                return;
            }

            if result[row as usize][col as usize] == old_color {
                result[row as usize][col as usize] = new_color;

                flood_fill_dfs(image, row + 1, col, new_color, old_color, result);
                flood_fill_dfs(image, row - 1, col, new_color, old_color, result);
                flood_fill_dfs(image, row, col + 1, new_color, old_color, result);
                flood_fill_dfs(image, row, col - 1, new_color, old_color, result);
            }
        }

        let mut result: Vec<Vec<i32>> = image.clone();
        let old_color = image[sr as usize][sc as usize];

        if new_color != old_color {
            flood_fill_dfs(&image, sr, sc, new_color, old_color, &mut result);
        }

        result
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let (height, width) = (grid.len(), grid[0].len());
        let mut parent = vec![vec![(width, height); width]; height];
        for i in 0..height {
            for j in 0..width {
                if grid[i][j] != '1' {
                    continue;
                }
                parent[i][j] = (i, j);
                if i > 0 && grid[i - 1][j] == '1' {
                    union(&mut parent, (i, j), (i - 1, j));
                }
                if j > 0 && grid[i][j - 1] == '1' {
                    union(&mut parent, (i, j), (i, j - 1));
                }
            }
        }
        let mut cnt = 0;
        for i in 0..height {
            for j in 0..width {
                if parent[i][j] == (i, j) {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    fn get_parent(parent: &mut Vec<Vec<(usize, usize)>>, p: (usize, usize)) -> (usize, usize) {
        let mut child = p;
        let mut p = p;
        while parent[p.0][p.1] != p {
            p = parent[p.0][p.1];
        }
        // path compression, adjust all the node's parent to root along the path
        while child != p {
            let temp = parent[child.0][child.1];
            parent[child.0][child.1] = p;
            child = temp;
        }
        p
    }

    fn union(parent: &mut Vec<Vec<(usize, usize)>>, p1: (usize, usize), p2: (usize, usize)) {
        let p1 = get_parent(parent, p1);
        let p2 = get_parent(parent, p2);
        if p1 == p2 {
            return;
        }
        parent[p1.0][p1.1] = p2
    }
}

#[cfg(test)]
mod test {
    use crate::leetweektwo::day9::day_9::flood_fill;
    #[test]
    fn test_lc_default() {
        assert_eq!(
            flood_fill(
                vec![vec![1 as i32, 1, 1], vec![1, 1 as i32, 0], vec![1, 0, 1]],
                1,
                1,
                2
            ),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }
}
