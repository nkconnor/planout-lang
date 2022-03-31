use planout::compile::compile;

fn main() {
    compile(
        r#"
        if (x) { 
            y = [5, 3];
        }

        return y;
        "#,
    );
}
