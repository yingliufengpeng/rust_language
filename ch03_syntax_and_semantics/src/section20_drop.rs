#[cfg(test)]
mod tests {

    struct HasDrop;

    impl Drop for HasDrop {
        fn drop(&mut self) {
            println!("Dropping...");
        }
    }

    struct FireWork {
        strength: i32,
    }

    impl Drop for FireWork {
        fn drop(&mut self) {
            println!("BOOM times {}!!!", self.strength);
        }
    }

    #[test]
    fn test() {
        // let x = HasDrop;

        let firecracker = FireWork { strength: 1 };

        let tnt = FireWork { strength: 100 };



    }
}
