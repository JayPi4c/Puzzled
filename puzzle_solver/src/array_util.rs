use ndarray::{Array2, Axis};

pub fn rotate_90(array: &Array2<bool>) -> Array2<bool> {
    let mut array = array.clone().reversed_axes();
    array.invert_axis(Axis(1));
    array
}

#[cfg(test)]
mod test {
    use crate::array_util::rotate_90;
    use ndarray::arr2;

    #[test]
    fn test_rotate_90_size_1() {
        dbg!("test_rotate_90_size_1");
        let array = arr2(&[[true]]);
        let rotated = rotate_90(&array);
        let expected = arr2(&[[true]]);
        assert_eq!(rotated, expected);
    }

    #[test]
    fn test_rotate_90_size_2() {
        let array = arr2(&[[true, false]]);
        let rotated = rotate_90(&array);
        let expected = arr2(&[[true], [false]]);
        assert_eq!(rotated, expected);
    }

    #[test]
    fn test_rotate_90() {
        let array = arr2(&[
            [true, false, false],
            [true, true, true],
            [true, false, true],
        ]);
        let rotated = rotate_90(&array);
        let expected = arr2(&[
            [true, true, true],
            [false, true, false],
            [true, true, false],
        ]);
        assert_eq!(rotated, expected);
    }
}
