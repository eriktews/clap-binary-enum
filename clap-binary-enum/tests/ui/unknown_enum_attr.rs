use clap_binary_enum::YesNoArg;

#[derive(YesNoArg)]
#[yesno(unknown = "x")]
enum Foo { A, B }
