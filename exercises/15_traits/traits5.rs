trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}


struct SomeStruct;
impl SomeTrait for SomeStruct {
    fn some_function(&self) -> bool {
        true
    }
}
impl OtherTrait for SomeStruct {
    fn other_function(&self) -> bool {
        true
    }
}

struct OtherStruct;
impl SomeTrait for OtherStruct {
    fn some_function(&self) -> bool {
        true
    }
}
impl OtherTrait for OtherStruct {
    fn other_function(&self) -> bool {
        true
    }
}

// TODO: Fix the compiler error by only changing the signature of this function.
fn some_func<T:SomeTrait + OtherTrait>(item:T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}
