use clap_binary_enum::YesNoArg;

#[derive(YesNoArg)]
enum Bar {
    #[yesno(unknown = "x")]
    A,
    B,
}
