use clap_binary_enum::YesNoArg;

#[derive(YesNoArg)]
enum WithData { A(i32), B }
