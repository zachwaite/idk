pub struct StatementChunk {
    statement: Statement,
    span: Span,
    token: Token,
}

pub struct CommentChunk {
    comment: Comment,
    span: Span,
    token: Token,
}

pub struct IdkChunk {
    idk: Idk,
    span: Span,
    token: Token,
}

pub enum Chunk {
    Statement(StatementChunk),
    Comment(CommentChunk),
    Idk(IdkChunk),
}

pub struct File {
    pub chunks: Vec<Chunk>,
}

impl File {
    pub fn token_literal(&self) -> String {
        if self.chunks.len() > 0 {
            self.chunks[0].token_literal()
        } else {
            "".to_string()
        }
    }
    pub fn string(&self) -> String {
        self.chunks
            .iter()
            .map(|s| s.string())
            .collect::<Vec<String>>()
            .join("")
    }
}
