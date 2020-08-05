pub fn run() {

    let oof = 42;
    let owie = false;

    println!(
        "{oof} * {owie} = {yikers}",
        oof = oof,
        owie = owie,
        yikers = if owie {oof} else {0}
    );
}
