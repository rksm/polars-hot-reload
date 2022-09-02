use polars::prelude::*;

#[derive(Default, Debug)]
pub struct State {
    pub version: usize,
}

#[no_mangle]
pub fn step(state: &mut State) {
    run().unwrap();
}

fn run() -> Result<()> {
    let file = todo!();
    let df = LazyCsvReader::new(file).has_header(true).finish()?;

    println!("{:?}", df.schema()?);
    println!("{:#?}", df.dtypes());

    let df = df.collect();
    println!("{df}");

    Ok(())
}
