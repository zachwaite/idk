use super::idk_field::IdkField;

pub enum FieldResult<T> {
    Ok(T),
    Idk(IdkField),
}
