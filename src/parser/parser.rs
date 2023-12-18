use super::scan;
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
    fn try_new(input: Token) -> Result<Self, String> {
        match input {
            Token::Raw(_) | Token::Newline(_) | Token::Whitespace(_)| Token::Semicolon(_) => Ok(Self {
                tokens: vec![input],
            }),
            Token::Begsr(_) => Err(format!("Parse error: Attempted to construct RawBuilder from Begsr token")),
            Token::Endsr(_) => Err(format!("Parse error: Attempted to construct RawBuilder from Endsr token")),
            Token::Exsr(_) => Err(format!("Parse error: Attempted to construct RawBuilder from Exsr token")),
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
    fn try_new(begsr: Token) -> Result<Self, String> {
        match &begsr {
            Token::Begsr(s) => Ok(Self {
                begsr: s.clone(),
                subname: None,
                semicolon: None,
                tokens: vec![begsr.clone()],
            }),
            Token::Raw(_) => Err(format!("Parse error: Attempted to construct DefBuilder from Raw token")),
            Token::Newline(_) => Err(format!("Parse error: Attempted to construct DefBuilder from Newline token")),
            Token::Whitespace(_) => Err(format!("Parse error: Attempted to construct DefBuilder from Whitespace token")) ,
            Token::Semicolon(_) => Err(format!("Parse error: Attempted to construct DefBuilder from Semicolon token")),
            Token::Endsr(_) => Err(format!("Parse error: Attempted to construct DefBuilder from Endsr token")),
            Token::Exsr(_) => Err(format!("Parse error: Attempted to construct DefBuilder from Exsr token")),
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
    fn try_new(exsr: Token) -> Result<Self, String> {
        match &exsr {
            Token::Exsr(s) => Ok(Self {
                state: CallBuilderState::Exsr,
                subname: None,
                tokens: vec![exsr.clone()],
            }),
            Token::Raw(_) => Err(format!("Parse error: Attempted to construct CallBuilder from Raw token")),
            Token::Newline(_) => Err(format!("Parse error: Attempted to construct CallBuilder from Newline token")),
            Token::Whitespace(_) => Err(format!("Parse error: Attempted to construct CallBuilder from Whitespace token")) ,
            Token::Semicolon(_) => Err(format!("Parse error: Attempted to construct CallBuilder from Semicolon token")),
            Token::Begsr(_) => Err(format!("Parse error: Attempted to construct CallBuilder from Begsr token")),
            Token::Endsr(_) => Err(format!("Parse error: Attempted to construct CallBuilder from Endsr token")),
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

    fn consume_state_neutral(&mut self, token: &Token) -> Result<(), String> {
        match token {
            Token::Begsr(_) => {
                self.state = ProgramState::BuildingDef;
                self.def_builder = Some(DefBuilder::try_new(token.clone())?);
                Ok(())
            }
            Token::Exsr(_) => {
                self.state = ProgramState::BuildingCall;
                self.call_builder = Some(CallBuilder::try_new(token.clone())?);
                Ok(())
            }
            Token::Raw(_) | Token::Newline(_) | Token::Whitespace(_) | Token::Semicolon(_) => {
                self.state = ProgramState::BuildingRaw;
                self.raw_builder = Some(RawBuilder::try_new(token.clone())?);
                Ok(())
            }
            Token::Endsr(_) => Err(format!("Syntax error: Encounterd Endsr without Begsr")),
        }
    }

    fn consume_state_building_raw(&mut self, token: &Token) -> Result<(), String> {
        let builder = self.raw_builder.as_mut().unwrap();
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

    fn consume_state_building_call(&mut self, token: &Token) -> Result<(), String> {
        let builder = self.call_builder.as_mut().unwrap();
        match builder.state {
            CallBuilderState::Exsr => match token {
                Token::Whitespace(_) => {
                    builder.tokens.append(&mut vec![token.clone()]);
                    builder.state = CallBuilderState::Space;
                    Ok(())
                }
                _ => Err(format!("Syntax error: Expected Whitespace, then Identifier"))
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
                Token::Semicolon(_) => {
                    Err(format!("Syntax error: Semicolon encountered before Identifier"))
                }
                Token::Exsr(_) | Token::Begsr(_) | Token::Endsr(_) => {
                    Err(format!("Syntax error: Keyword cannot be used as Identifier"))
                }
                Token::Newline(_) => {
                    Err(format!("Syntax error: Unexpected Newline"))
                }

            },
            CallBuilderState::Identifier => match token {
                Token::Whitespace(_) => {
                    builder.tokens.append(&mut vec![token.clone()]);
                    Ok(())
                },
                Token::Semicolon(_) => {
                    builder.tokens.append(&mut vec![token.clone()]);
                    let mut raw: String = "".to_string();
                    for token in builder.tokens.iter() {
                        raw.push_str(&token.unwrap());
                    }
                    let s = builder.subname.as_mut().unwrap();
                    let id = Identifier { raw: s.clone() };
                    let call = SubroutineCall {
                        id: id.clone(),
                        raw,
                    };
                    let chunk = ProgramChunk::SubroutineCall(id, call);
                    self.program.chunks.append(&mut vec![chunk]);
                    self.call_builder = None;
                    self.state = ProgramState::Neutral;
                    Ok(())
                },
                _ => Err(format!("Programming error: unreachable destination. Only Whitespace or Semicolons allowed after Identifier.")),
            },
            CallBuilderState::Semicolon => Err(format!("Programming error: unreachable destination. The builder should be dropped as soon as the semicolon is consumed.")),
        }
    }

    fn consume_state_building_def(&mut self, token: &Token) -> Result<(), String> {
        let &mut _ = self.def_builder.as_mut().unwrap();
        Ok(())
    }

    fn consume(&mut self, token: &Token) -> Result<(), String> {
        match self.state {
            ProgramState::Neutral => self.consume_state_neutral(token),
            ProgramState::BuildingRaw => self.consume_state_building_raw(token),
            ProgramState::BuildingCall => self.consume_state_building_call(token),
            ProgramState::BuildingDef => self.consume_state_building_def(token),
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
    builder.build()
}

pub fn debug(input: &str) -> Result<String, String> {
    let tokens = scan(input);
    let pgm = parse(&tokens).unwrap();
    dbg!("{}", pgm);
    println!("TODO: implement consume_state_building_def");
    println!("TODO: add custom errors. Maybe thiserror?");
    Ok("".to_string())
}
