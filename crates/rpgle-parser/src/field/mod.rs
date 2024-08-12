mod comment_field;
mod filetype_field;
mod formtype_field;
mod idk_field;
mod keywords_field;
mod name_field;
mod result;
mod sequence_field;

pub use comment_field::CommentField;
pub use filetype_field::FiletypeField;
pub use formtype_field::{Formtype, FormtypeField};
pub use idk_field::IdkField;
pub use keywords_field::KeywordsField;
pub use name_field::NameField;
pub use result::FieldResult; // used by each field type
pub use sequence_field::SequenceField;
