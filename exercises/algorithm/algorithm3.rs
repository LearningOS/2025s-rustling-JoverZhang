/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

struct QuickSort<T>(std::marker::PhantomData<T>);

impl<T> QuickSort<T>
where
    T: PartialOrd + Copy,
{
    pub fn sort(array: &mut [T], low: usize, high: usize) {
        if low < high {
            let p = Self::partition(array, low, high);
            if p > 0 {
                Self::sort(array, low, p - 1);
            }
            Self::sort(array, p + 1, high);
        }
    }

    fn partition(arr: &mut [T], low: usize, high: usize) -> usize {
        let pivot = arr[high];
        let mut i = low;

        for j in low..high {
            if arr[j] < pivot {
                arr.swap(i, j);
                i += 1;
            }
        }

        arr.swap(i, high);
        i
    }
}

fn sort<T>(array: &mut [T])
where
    T: PartialOrd + Copy,
{
    QuickSort::sort(array, 0, array.len() - 1);

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}