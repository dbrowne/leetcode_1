pub mod one {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 2 {
            return s;
        }
        let chrs: Vec<char> = s.chars().collect();
        let len = chrs.len();
        let unit: usize = (2 * num_rows - 2) as usize;
        let mut sb = String::with_capacity(s.capacity());
        for row in 0..num_rows {
            let mut i = row as usize;
            let step1 = 2 * (num_rows - 1 - row) as usize;
            let step2 = (unit - step1) as usize;
            let mut trigger = false;
            while i < len {
                sb.push(chrs[i]);
                if step1 == 0 {
                    i += step2;
                } else if step2 == 0 {
                    i += step1
                } else {
                    i += if trigger { step2 } else { step1 };
                    trigger = !trigger;
                }
            }
        }
        sb
    }
}
