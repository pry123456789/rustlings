fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = &a[1..4]; // 前闭后开  切片的变量实际上并不能获得地址的所有权，仅仅是一个引用

        assert_eq!([2, 3, 4], nice_slice);
    }
}
