use cstr::cstr;
use qmetaobject::prelude::*;
use calculator::parse;
use calculator::parser::Expr;
use calculator::parser::Compute;

#[derive(QObject, Default)]
pub struct Calculator {
    base: qt_base_class!(trait QObject),
    result: qt_property!(QString; NOTIFY result_changed),
    result_changed: qt_signal!(),
    compute: qt_method!(fn compute(&mut self, input: String) {
        let last = self.result.to_string().parse::<f32>().unwrap_or(0.0);

        let tree = if input.is_empty() && self.last_tree.is_some() {
            self.last_tree.clone().unwrap()
        } else if let Some(tree) = parse(&input) {
            tree
        } else {
            return;
        };

        let num = tree.compute(last);
        self.last_tree = Some(tree);
        self.set_result(num.to_string().into());

    }),
    last_tree: Option<Expr>
}

impl Calculator {
    fn set_result(&mut self, s: QString) {
        self.result = s;
        self.result_changed()
    }
}


fn main() {
    qml_register_type::<Calculator>(cstr!("Calculator"), 1, 0, cstr!("Calculator"));

    let mut engine = QmlEngine::new();

    engine.load_file("ui/Button.qml".into());
    engine.load_file("ui/Main.qml".into());

    engine.exec();
}
