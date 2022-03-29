use planout::parse::compile;

fn main() {
    compile(
        r#"
        if (x) { 
            y = 5;
        }

        return y;
        "#,
    );
}
