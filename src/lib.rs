// From https://docs.rs/countme/3.0.1/countme/

#[derive(Default)]
struct Widget {
    _c: countme::Count<Self>,
}

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use super::Widget;

    #[test]
    fn countme_orig_example() {
        countme::enable(true);

        let w1 = Widget::default();
        let w2 = Widget::default();
        let w3 = Widget::default();
        drop(w1);

        let counts = countme::get::<Widget>();
        assert_eq!(counts.live, 2);
        assert_eq!(counts.max_live, 3);
        assert_eq!(counts.total, 3);

        eprintln!("{}", countme::get_all());
    }
}
