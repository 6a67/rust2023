enum Status {
    Valid,
    Error(String),
    Warning { message: String,
                 code: i32 },
    Pending(i32, String),
}
fn main() {
    let valid = Status::Valid;
    let warning = Status::Warning {
        message: String::from("Warn!"),
        code: 42,
    };
}
