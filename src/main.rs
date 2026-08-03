use abacus::{
    evaluation::tokenizer::{registry::token_registry::TokenRegistry, tokenize::tokenize_string},
    AbacusError, UnitRegistry,
};

fn main() -> Result<(), AbacusError> {
    let operator_reg = TokenRegistry::standard();
    let unit_reg = UnitRegistry::standard();
    println!(
        "{:?}",
        tokenize_string(&operator_reg, &unit_reg, "(5m + 20cm) as m")?
    );

    Ok(())
}

