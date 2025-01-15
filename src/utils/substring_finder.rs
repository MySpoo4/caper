#[derive(Debug, Clone)]
pub struct StrFinder {
    suffix_arr: Vec<usize>,
}

impl StrFinder {
    pub fn new(str: &str) -> Self {
        let suffix_arr = make_suffix_array_by_induced_sorting(str.as_bytes(), 256);
        StrFinder { suffix_arr }
    }

    pub fn change(&mut self, str: &str) {
        self.suffix_arr = make_suffix_array_by_induced_sorting(str.as_bytes(), 256);
    }

    pub fn find_all(&self, haystack: &str, needle: &str) -> Vec<usize> {
        let mut left = 0;
        let mut right = self.suffix_arr.len();

        while left < right {
            let mid = (left + right) / 2;
            let start = self.suffix_arr[mid];
            let suffix = &haystack[start..];

            if suffix.starts_with(needle) {
                return self.find_matched(haystack, needle, mid);
            } else if suffix < needle {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        Vec::new()
    }

    fn find_matched(&self, haystack: &str, needle: &str, mid: usize) -> Vec<usize> {
        let mut matched = Vec::new();

        for i in (0..mid).rev() {
            let idx = self.suffix_arr[i];
            let suffix = &haystack[idx..];
            match suffix.starts_with(needle) {
                true => matched.push(idx),
                false => break,
            }
        }

        matched.push(self.suffix_arr[mid]);

        for i in (mid + 1)..self.suffix_arr.len() {
            let idx = self.suffix_arr[i];
            let suffix = &haystack[idx..];
            match suffix.starts_with(needle) {
                true => matched.push(idx),
                false => break,
            }
        }

        matched
    }
}

// default alphabet_size = 256 (u8)
fn make_suffix_array_by_induced_sorting(string: &[u8], alphabet_size: usize) -> Vec<usize> {
    let typemap = build_type_map(string);
    let bucket_sizes = find_bucket_sizes(string, alphabet_size);
    let mut guessed_suffix_array = guess_lms_sort(string, &bucket_sizes, &typemap);
    induced_sort_l(string, &mut guessed_suffix_array, &bucket_sizes, &typemap);
    induced_sort_s(string, &mut guessed_suffix_array, &bucket_sizes, &typemap);

    let (summary_string, summary_alphabet_size, summary_suffix_offsets) =
        summarize_suffix_array(string, &mut guessed_suffix_array, &typemap);

    let summary_suffix_array = make_summary_suffix_array(summary_string, summary_alphabet_size);

    let mut result = accurate_lms_sort(
        string,
        &bucket_sizes,
        summary_suffix_array,
        summary_suffix_offsets,
    );

    induced_sort_l(string, &mut result, &bucket_sizes, &typemap);
    induced_sort_s(string, &mut result, &bucket_sizes, &typemap);

    return result;
}

// False -> S-type, True -> L-type
fn build_type_map(data: &[u8]) -> Vec<bool> {
    let mut res = vec![false; data.len() + 1];
    if data.len() == 0 {
        return res;
    }

    res[data.len() - 1] = true;

    for i in (0..data.len() - 1).rev() {
        if data[i] > data[i + 1] {
            res[i] = true;
        } else if data[i] == data[i + 1] && res[i + 1] == true {
            res[i] = true;
        }
    }

    res
}

fn find_bucket_sizes(str: &[u8], alphabet_size: usize) -> Vec<usize> {
    let mut res = vec![0; alphabet_size];
    for char in str {
        res[*char as usize] += 1
    }

    res
}

fn find_bucket_heads(bucket_sizes: &Vec<usize>) -> Vec<usize> {
    let mut offset = 1;
    let mut res = Vec::new();
    for size in bucket_sizes {
        res.push(offset);
        offset += size;
    }

    res
}

fn find_bucket_tails(bucket_sizes: &Vec<usize>) -> Vec<usize> {
    let mut offset = 1;
    let mut res = Vec::new();
    for size in bucket_sizes {
        offset += size;
        res.push(offset - 1);
    }

    res
}

fn guess_lms_sort(string: &[u8], bucket_sizes: &Vec<usize>, typemap: &Vec<bool>) -> Vec<usize> {
    let mut guessed_suffix_array: Vec<usize> = vec![usize::MAX; string.len() + 1];
    let mut bucket_tails = find_bucket_tails(bucket_sizes);

    for i in 0..string.len() {
        if !is_lms_char(i, typemap) {
            continue;
        }

        let bucket_index = string[i] as usize;
        guessed_suffix_array[bucket_tails[bucket_index]] = i;
        bucket_tails[bucket_index] -= 1;
    }

    guessed_suffix_array[0] = string.len();
    guessed_suffix_array
}

fn is_lms_char(offset: usize, typemap: &Vec<bool>) -> bool {
    if offset == 0 {
        return false;
    }
    if typemap[offset] == false && typemap[offset - 1] == true {
        return true;
    }

    return false;
}

fn induced_sort_l(
    string: &[u8],
    guessed_suffix_array: &mut Vec<usize>,
    bucket_sizes: &Vec<usize>,
    typemap: &Vec<bool>,
) {
    let mut bucket_heads = find_bucket_heads(bucket_sizes);

    for i in 0..guessed_suffix_array.len() {
        if guessed_suffix_array[i] == usize::MAX {
            continue;
        }

        if guessed_suffix_array[i] == 0 {
            continue;
        }

        let j = guessed_suffix_array[i] - 1;

        if !typemap[j] {
            continue;
        }

        let bucket_index = string[j] as usize;
        guessed_suffix_array[bucket_heads[bucket_index]] = j;
        bucket_heads[bucket_index] += 1;
    }
}

fn induced_sort_s(
    string: &[u8],
    guessed_suffix_array: &mut Vec<usize>,
    bucket_sizes: &Vec<usize>,
    typemap: &Vec<bool>,
) {
    let mut bucket_tails = find_bucket_tails(bucket_sizes);

    for i in (0..guessed_suffix_array.len()).rev() {
        if guessed_suffix_array[i] == 0 {
            continue;
        }

        let j = guessed_suffix_array[i] - 1;

        if typemap[j] {
            continue;
        }

        let bucket_index = string[j] as usize;
        guessed_suffix_array[bucket_tails[bucket_index]] = j;
        bucket_tails[bucket_index] -= 1;
    }
}

fn lms_substrings_are_equal(
    string: &[u8],
    typemap: &Vec<bool>,
    offset_a: usize,
    offset_b: usize,
) -> bool {
    if offset_a == string.len() || offset_b == string.len() {
        return false;
    }

    let mut i = 0;
    loop {
        let a_is_lms = is_lms_char(i + offset_a, &typemap);
        let b_is_lms = is_lms_char(i + offset_b, &typemap);

        if i > 0 && a_is_lms && b_is_lms {
            return true;
        }

        if a_is_lms != b_is_lms {
            return false;
        }

        if string[i + offset_a] != string[i + offset_b] {
            return false;
        }

        i += 1;
    }
}

fn summarize_suffix_array(
    string: &[u8],
    guessed_suffix_array: &mut Vec<usize>,
    typemap: &Vec<bool>,
) -> (Vec<usize>, usize, Vec<usize>) {
    let mut lms_names: Vec<usize> = vec![usize::MAX; string.len() + 1];
    let mut current_name = 0;
    let mut last_lms_suffix_offset;

    lms_names[guessed_suffix_array[0]] = current_name;
    last_lms_suffix_offset = guessed_suffix_array[0];

    for i in 1..string.len() {
        let suffix_offset = guessed_suffix_array[i];

        if !is_lms_char(suffix_offset, typemap) {
            continue;
        }

        if !lms_substrings_are_equal(string, typemap, last_lms_suffix_offset, suffix_offset) {
            current_name += 1;
        }

        last_lms_suffix_offset = suffix_offset;

        lms_names[suffix_offset] = current_name;
    }

    let mut summary_suffix_offsets = Vec::new();
    let mut summary_string = Vec::new();
    for (idx, name) in lms_names.into_iter().enumerate() {
        if name == usize::MAX {
            continue;
        }

        summary_suffix_offsets.push(idx);
        summary_string.push(name);
    }

    let summary_alphabet_size = current_name + 1;

    return (
        summary_string,
        summary_alphabet_size,
        summary_suffix_offsets,
    );
}

fn make_summary_suffix_array(
    summary_string: Vec<usize>,
    summary_alphabet_size: usize,
) -> Vec<usize> {
    let mut summary_suffix_array;
    if summary_alphabet_size == summary_string.len() {
        summary_suffix_array = vec![usize::MAX; summary_string.len() + 1];
        summary_suffix_array[0] = summary_string.len();

        for i in 0..summary_string.len() {
            let y = summary_string[i];
            summary_suffix_array[y + 1] = i;
        }
    } else {
        summary_suffix_array = make_suffix_array_by_induced_sorting(
            summary_string
                .into_iter()
                .map(|v| v as u8)
                .collect::<Vec<u8>>()
                .as_slice(),
            summary_alphabet_size,
        );
    }

    return summary_suffix_array;
}

fn accurate_lms_sort(
    string: &[u8],
    bucket_sizes: &Vec<usize>,
    summary_suffix_array: Vec<usize>,
    summary_suffix_offsets: Vec<usize>,
) -> Vec<usize> {
    let mut suffix_offsets: Vec<usize> = vec![usize::MAX; string.len() + 1];
    let mut bucket_tails = find_bucket_tails(bucket_sizes);

    for i in (2..summary_suffix_array.len()).rev() {
        let string_index = summary_suffix_offsets[summary_suffix_array[i]];
        let bucket_index = string[string_index] as usize;

        suffix_offsets[bucket_tails[bucket_index]] = string_index;
        bucket_tails[bucket_index] -= 1;
    }

    suffix_offsets[0] = string.len();
    return suffix_offsets;
}

impl Default for StrFinder {
    fn default() -> Self {
        StrFinder {
            suffix_arr: Vec::new(),
        }
    }
}
