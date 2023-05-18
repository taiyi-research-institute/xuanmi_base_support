pub trait QuickSort {
    fn quicksort(&mut self) -> Vec<usize>;
}

pub trait Permute<T> {
    fn permute_by(&mut self, order: &Vec<usize>) -> Result<(), ()>;
}

impl<T> Permute<T> for Vec<T>
where
    T: Clone + PartialOrd,
{
    fn permute_by(&mut self, order: &Vec<usize>) -> Result<(), ()> {
        let len = self.len();

        //#region Validate `order` in O(len) time complexity.
        if len != order.len() {
            return Err(());
        }
        let mut used = vec![false; len];
        for i in order {
            if *i < len && !used[*i] {
                used[*i] = true;
            } else {
                return Err(());
            }
        }
        for cond in used {
            if !cond {
                return Err(());
            }
        }
        //#endregion

        //#region Do actual permutation
        let mut order = order.clone();
        for i in 0..self.len() {
            if i != order[i] {
                self.swap(i, order[i]);
                for j in i + 1..order.len() {
                    if order[j] == i {
                        order[j] = order[i];
                        break;
                    }
                }
            }
        }
        //#endregion
        return Ok(());
    }
}

impl<T> QuickSort for Vec<T>
where
    T: Clone + PartialOrd,
{
    fn quicksort(&mut self) -> Vec<usize> {
        let mut order: Vec<usize> = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            order.push(i);
        }
        let mut stack: Vec<usize> = vec![0; 256];
        let mut h: usize = 0;
        stack[0] = self.len() - 1;
        stack[1] = 0;
        h += 2;
        while h > 0 {
            let beg = stack[h - 1];
            let end = stack[h - 2];
            h -= 2;
            if end > beg {
                let piv = partition(self, &mut order, beg, end);
                stack[h] = end;
                stack[h + 1] = piv + 1;
                stack[h + 2] = piv - 1;
                stack[h + 3] = beg;
                h += 4;
            }
        }
        return order;
    }
}

// 将pivot放在正序位置. 也就是说, pivot是第几, 就成为左起第几个元素.
// pivot左边的所有元素都小于pivot.
fn partition<T>(data: &mut Vec<T>, order: &mut Vec<usize>, beg: usize, end: usize) -> usize
where
    T: Clone + PartialOrd,
{
    // 如果至少3个元素, 以三数取中法确定pivot
    if end - beg > 1 {
        let mid: usize = (end + beg) >> 1;
        if data[end] < data[beg] {
            // 标准库为 &mut [T] 挂载了 swap 函数
            // &mut Vec<T> 可以 Deref 到 &mut [T]
            // 所以, &mut Vec<T> 具有 swap 成员函数
            data.swap(beg, end);
            order.swap(beg, end);
        }
        if data[mid] < data[end] {
            data.swap(mid, end);
            order.swap(mid, end);
        }
        if data[end] < data[beg] {
            data.swap(beg, end);
            order.swap(beg, end);
        }
    }
    let pivot: T = data[end].clone();
    let mut i: usize = beg;
    for j in beg..end {
        if data[j] < pivot {
            data.swap(i, j);
            order.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, end);
    order.swap(i, end);
    return i;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_qsort() {
        println!("Test sorting Vec<i32>");
        let mut data = vec![11, 4, 5, 14, 19, 19, 8, 10, 89, 3];
        println!("Original: {:?}", data);
        let order = data.quicksort();
        println!("Sorted: {:?}", data);
        println!("Argsort: {:?}\n", order);

        println!("Test sorting Vec<&str>");
        let mut data = vec!["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛"];
        println!("Original: {:?}", data);
        let order = data.quicksort();
        println!("Sorted: {:?}", data);
        println!("Argsort: {:?}\n", order);
    }
    #[test]
    fn test_premute() {
        println!("Test permuting Vec<&str>");
        let mut data = vec!["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛"];
        let order: Vec<usize> = vec![5, 3, 1, 7, 2, 0, 4, 6];
        println!("Original: {:?}", data);
        println!("PermuteBy: {:?}", order);
        data.permute_by(&order).unwrap();
        println!("Permuted: {:?}", data);
    }
}
