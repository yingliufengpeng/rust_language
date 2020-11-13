
macro_rules! call_with_larch {
    ($callback:ident) => { $callback!(larch)  };
}

macro_rules! expand_to_larch {
    () => { larch };
}


macro_rules! recognise_tree {
    (larch) => { println!("#1, the Larch.") };
    (redwood) => { println!("#2, the Mighty Redwood.") };
    (fir) => { println!("#3, the Fir.") };
    (chestnut) => { println!("#4, the Horse Chestnut.") };
    (pine) => { println!("#5, the Scots Pine.") };
    ($($other:tt)*) => { println!("I don't know; some kind of birch maybe?") };

}


macro_rules! callback {
    ($callback:ident($($args:tt)*)) => {
        $callback!( $($args)* )
    };

    () => {println!("end");}
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {
        recognise_tree!(expand_to_larch!());
        call_with_larch!(recognise_tree);

        callback!(callback(callback(println("ok")) ));
    }

}