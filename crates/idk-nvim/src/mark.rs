struct Marker {
    buf: oxi::api::Buffer,
    namespace_id: u32,
}

impl Marker {
    pub fn mark_specs(&mut self) -> oxi::Result<()> {
        let count = self.buf.line_count()?;
        let lines = self.buf.get_lines(0..count, true)?;
        let mut input = String::new();
        for line in lines {
            input.push_str(&line.to_string());
            input.push_str("\n");
        }
        let lexer = new_lexer(&input);
        let mut front_kind = TokenKind::Eof;
        if let Ok(tok) = next_token(&lexer) {
            front_kind = tok.kind;
        }
        let mut counter = 0;
        let mut state = SpecState::new();
        while front_kind != TokenKind::Eof && counter < 1000000 {
            match next_token(&lexer) {
                Ok(tok) => {
                    state.evolve(&tok);
                    counter += 1;
                }
                Err(e) => {
                    oxi::print!("{}\n", e);
                    return Ok(());
                }
            }
        }
        let opts = oxi::api::opts::SetMarkOpts::default();
        let _ = self.buf.set_mark('h', state.get_control(), 1, &opts);
        let _ = self.buf.set_mark('f', state.get_file(), 1, &opts);
        let _ = self.buf.set_mark('d', state.get_definition(), 1, &opts);
        let _ = self.buf.set_mark('c', state.get_calculation(), 1, &opts);
        let _ = self.buf.set_mark('i', state.get_input(), 1, &opts);
        let _ = self.buf.set_mark('o', state.get_output(), 1, &opts);
        if env::var("DEBUG").is_ok() {
            oxi::print!("{}: {}\n", counter, state);
        }
        Ok(())
    }
}

// Can add this to lib.rs
// let mark_rpgle_specs = oxi::Function::from_fn(move |(): ()| {
//     let mut marker = Marker {
//         buf: oxi::api::Buffer::current(),
//         namespace_id: oxi::api::create_namespace("RPGLENamespace2"),
//     };
//     if let Err(e) = marker.mark_specs() {
//         oxi::print!("ERROR");
//         oxi::print!("\n");
//         oxi::print!("{}", e);
//         oxi::print!("\n");
//     }
// });
