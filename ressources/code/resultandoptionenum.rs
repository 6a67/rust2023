enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    None,
    Some(T),
}