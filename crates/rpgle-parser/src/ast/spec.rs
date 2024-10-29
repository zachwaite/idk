pub enum Spec {
    H {
        sequence: FieldResult<SequenceField>,
        form_type: FieldResult<FormtypeField>,
        keywords: FieldResult<HKeywordsField>,
    },
}
