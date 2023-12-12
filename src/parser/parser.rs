use super::Token;

#[derive(Debug, Clone, PartialEq)]
struct Identifier {
    raw: String,
}

impl Identifier {
    pub fn dump(&self) -> String {
        "id".to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SubroutineCall {
    id: Identifier,
    raw: String,
}

impl SubroutineCall {
    pub fn dump(&self) -> String {
        "call".to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SubroutineChunk {
    SubroutineCall(Identifier, SubroutineCall),
    Raw(String),
}

impl SubroutineChunk {
    pub fn dump(&self) -> String {
        match self {
            SubroutineChunk::SubroutineCall(id, call) => "chunk".to_string(),
            SubroutineChunk::Raw(raw) => "chunk".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SubroutineDefinition {
    id: Identifier,
    raw: String,
    chunks: Vec<SubroutineChunk>,
}

impl SubroutineDefinition {
    pub fn dump(&self) -> String {
        "def".to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ProgramChunk {
    SubroutineDefinition(Identifier, SubroutineDefinition),
    SubroutineCall(Identifier, SubroutineCall),
    Raw(String),
}

impl ProgramChunk {
    pub fn dump(&self) -> String {
        match self {
            ProgramChunk::SubroutineDefinition(id, def) => "chunk".to_string(),
            ProgramChunk::SubroutineCall(id, call) => "chunk".to_string(),
            ProgramChunk::Raw(raw) => "chunk".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    chunks: Vec<ProgramChunk>,
}

impl Program {
    pub fn dump(&self) -> String {
        let mut out: String = "".to_string();
        for chunk in self.chunks.iter() {
            out.push_str(&chunk.dump());
        }
        out
    }
}

#[derive(Debug, Clone)]
struct RawBuilder {
    tokens: Vec<Token>,
}

impl RawBuilder {
    fn new(input: Token) -> Self {
        match input {
            Token::Raw(_) | Token::Newline(_) | Token::Whitespace(_) => Self {
                tokens: vec![input],
            },
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
struct DefBuilder {
    begsr: String,
    subname: Option<String>,
    semicolon: Option<String>,
    tokens: Vec<Token>,
}

impl DefBuilder {
    fn new(begsr: Token) -> Self {
        match &begsr {
            Token::Begsr(s) => Self {
                begsr: s.clone(),
                subname: None,
                semicolon: None,
                tokens: vec![begsr.clone()],
            },
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
enum CallBuilderState {
    Exsr,
    Space,
    Identifier,
    Semicolon,
}

#[derive(Debug, Clone)]
struct CallBuilder {
    state: CallBuilderState,
    subname: Option<String>,
    tokens: Vec<Token>,
}

impl CallBuilder {
    fn new(exsr: Token) -> Self {
        match &exsr {
            Token::Exsr(s) => Self {
                state: CallBuilderState::Exsr,
                subname: None,
                tokens: vec![exsr.clone()],
            },
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
enum ProgramState {
    Neutral,
    BuildingRaw,
    BuildingCall,
    BuildingDef,
}

#[derive(Debug, Clone)]
struct ProgramBuilder {
    program: Program,
    state: ProgramState,
    raw_builder: Option<RawBuilder>,
    def_builder: Option<DefBuilder>,
    call_builder: Option<CallBuilder>,
}

impl ProgramBuilder {
    fn new() -> Self {
        Self {
            program: Program { chunks: vec![] },
            state: ProgramState::Neutral,
            raw_builder: None,
            def_builder: None,
            call_builder: None,
        }
    }

    fn consume(&mut self, token: &Token) -> Result<(), String> {
        match self.state {
            ProgramState::Neutral => {
                match token {
                    Token::Begsr(_) => {
                        self.state = ProgramState::BuildingDef;
                        self.def_builder = Some(DefBuilder::new(token.clone()));
                    }
                    Token::Exsr(_) => {
                        self.state = ProgramState::BuildingCall;
                        self.call_builder = Some(CallBuilder::new(token.clone()));
                    }
                    Token::Raw(_) | Token::Newline(_) | Token::Whitespace(_) => {
                        self.state = ProgramState::BuildingRaw;
                        self.raw_builder = Some(RawBuilder::new(token.clone()));
                    }
                    _ => unimplemented!(),
                }
                Ok(())
            }
            ProgramState::BuildingRaw => {
                match &mut self.raw_builder {
                    Some(builder) => {
                        match token {
                            Token::Raw(_) | Token::Newline(_) | Token::Whitespace(_) => {
                                builder.tokens.append(&mut vec![token.clone()]);
                                Ok(())
                            }
                            _ => {
                                let mut raw: String = "".to_string();
                                for t in builder.tokens.iter() {
                                    raw.push_str(t.unwrap());
                                }
                                let chunk = ProgramChunk::Raw(raw);
                                self.program.chunks.append(&mut vec![chunk]);
                                self.raw_builder = None;
                                self.state = ProgramState::Neutral;
                                self.consume(token); // so the NeutralHandler hits
                                Ok(())
                            }
                        }
                    }
                    None => unimplemented!(),
                }
            }
            ProgramState::BuildingCall => match &mut self.call_builder {
                Some(builder) => match builder.state {
                    CallBuilderState::Exsr => match token {
                        Token::Whitespace(_) => {
                            builder.tokens.append(&mut vec![token.clone()]);
                            builder.state = CallBuilderState::Space;
                            Ok(())
                        }
                        _ => Err(">>>>>>>>>>>>>>>>>>>>>>ERROR".to_string()),
                    },
                    CallBuilderState::Space => match token {
                        Token::Whitespace(_) => {
                            builder.tokens.append(&mut vec![token.clone()]);
                            Ok(())
                        }
                        Token::Raw(s) => {
                            builder.tokens.append(&mut vec![token.clone()]);
                            builder.state = CallBuilderState::Identifier;
                            builder.subname = Some(s.clone());
                            Ok(())
                        }
                        _ => {
                            unimplemented!()
                        }
                    },
                    CallBuilderState::Identifier => match token {
                        Token::Whitespace(_) => {
                            builder.tokens.append(&mut vec![token.clone()]);
                            Ok(())
                        }
                        Token::Semicolon(_) => {
                            builder.tokens.append(&mut vec![token.clone()]);
                            let mut raw: String = "".to_string();
                            for token in builder.tokens.iter() {
                                raw.push_str(&token.unwrap());
                            }
                            match &builder.subname {
                                Some(s) => {
                                    let id = Identifier { raw: s.clone() };
                                    let call = SubroutineCall {
                                        id: id.clone(),
                                        raw,
                                    };
                                    let chunk = ProgramChunk::SubroutineCall(id, call);
                                    self.program.chunks.append(&mut vec![chunk]);
                                    self.call_builder = None;
                                    self.state = ProgramState::Neutral;
                                }
                                None => unimplemented!(),
                            }
                            Ok(())
                        }
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                },
                None => unimplemented!(),
            },
            ProgramState::BuildingDef => {
                match &mut self.def_builder {
                    Some(builder) => {
                        // match builder.state {
                        // }
                        Ok(())
                    }
                    None => unimplemented!(),
                }
            }
        }
    }

    fn build(&self) -> Result<Program, String> {
        match self.state {
            _ => Ok(self.program.clone()),
        }
    }
}

pub fn parse(input: &Vec<Token>) -> Result<Program, String> {
    let mut builder = ProgramBuilder::new();
    for token in input.iter() {
        builder.consume(token)?;
    }
    dbg!("{}", builder.clone());
    builder.build()
}
