use crate::assert_js;

#[test]
fn expressions() {
    assert_js!(
        r#"
fn go() {
    True
    False
    Nil
}
"#,
        r#""use strict";

function go() {
  true;
  false;
  return undefined;
}
"#
    );
}

#[test]
fn constants() {
    assert_js!(
        r#"
const a = True
const b = False
const c = Nil
"#,
        r#""use strict";

const a = true;

const b = false;

const c = undefined;
"#
    );
}

#[test]
fn operators() {
    assert_js!(
        r#"
fn go() {
    True && True
    False || False
}
"#,
        r#""use strict";

function go() {
  true && true;
  return false || false;
}
"#
    );
}

#[test]
fn assigning() {
    assert_js!(
        r#"
fn go(x, y) {
    let True = x
    let False = x
    let Nil = y
}
"#,
        r#""use strict";

function go(x, y) {
  let $ = x;
  if (!($ === true)) throw new Error("Bad match");
  
  let $$1 = x;
  if (!($$1 === false)) throw new Error("Bad match");
  
  let $$2 = y;
  if (!($$2 === undefined)) throw new Error("Bad match");
}
"#
    );
}
