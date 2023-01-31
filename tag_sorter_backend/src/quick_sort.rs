use crate::item::Item;

#[allow(clippy::cast_sign_loss)]
const fn get_date(item: &Item) -> usize {
    item.day as usize + item.month as usize * 100 + item.year as usize * 10000
}



pub fn quick_sort(items: &mut [Item]) {
    if items.len() > 1 {
        bubble_sort_optimized(items);
    }
} 
pub fn bubble_sort_optimized(arr: &mut [Item]) {
    let mut new_len: usize;
    let mut len = arr.len();
    // Outer loop
    loop {
        new_len = 0;
        // Inner loop
        for i in 1..len {
            if get_date(&arr[i - 1]) > get_date(&arr[i]) {
                arr.swap(i - 1, i);
                new_len = i;
            }
        }
        if new_len == 0 {
            break;
        }
        len = new_len;
    }
}
pub fn sort_pairs<T>(arr: &mut [(T, usize)]) {
    let mut new_len: usize;
    let mut len = arr.len();
    // Outer loop
    loop {
        new_len = 0;
        // Inner loop
        for i in 1..len {
            if arr[i - 1].1 > arr[i].1 {
                arr.swap(i - 1, i);
                new_len = i;
            }
        }
        if new_len == 0 {
            break;
        }
        len = new_len;
    }
}