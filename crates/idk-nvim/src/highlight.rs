use dds_parser;
use rpgle_parser;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct HighlightMeta {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
    pub hl_group: String,
    pub src: String,
}
impl fmt::Display for HighlightMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "({}, {}) -> ({}, {}): {}, src={}",
            self.start_row, self.start_col, self.end_row, self.end_col, self.hl_group, self.src,
        );
        write!(f, "{}", s)
    }
}
impl From<((usize, usize), (usize, usize), &str, &str)> for HighlightMeta {
    fn from(value: ((usize, usize), (usize, usize), &str, &str)) -> Self {
        Self {
            start_row: value.0 .0,
            start_col: value.0 .1,
            end_row: value.1 .0,
            end_col: value.1 .1,
            hl_group: value.2.to_string(),
            src: value.3.to_string(),
        }
    }
}

// main
pub fn highlight_rpgle(txt: &str) -> Vec<HighlightMeta> {
    if let Ok(cst) = rpgle_parser::CST::try_from(txt) {
        if env::var("DEBUG").is_ok() {
            let _ = std::fs::write("/tmp/cst.txt", format!("{:#?}", &cst));
        }
        let mut out = rpgle_parser::highlight_cst(&cst)
            .into_iter()
            .map(|tup| HighlightMeta::from((tup.0, tup.1, tup.2.as_str(), "CST")))
            .collect::<Vec<HighlightMeta>>();
        let ast = rpgle_parser::AST::from(&cst);
        if env::var("DEBUG").is_ok() {
            let _ = std::fs::write("/tmp/ast.txt", format!("{:#?}", &ast));
        }
        out.append(
            &mut rpgle_parser::highlight_ast(ast)
                .into_iter()
                .map(|tup| HighlightMeta::from((tup.0, tup.1, tup.2.as_str(), "AST")))
                .collect::<Vec<HighlightMeta>>(),
        );
        out
    } else {
        vec![]
    }
}

pub fn highlight_pfdds(txt: &str) -> Vec<HighlightMeta> {
    if let Ok(cst) = dds_parser::pfdds::CST::try_from(txt) {
        if env::var("DEBUG").is_ok() {
            let _ = std::fs::write("/tmp/cst.txt", format!("{:#?}", &cst));
        }
        let mut out = dds_parser::pfdds::highlight_cst(&cst)
            .into_iter()
            .map(|tup| HighlightMeta::from((tup.0, tup.1, tup.2.as_str(), "CST")))
            .collect::<Vec<HighlightMeta>>();
        let ast = dds_parser::pfdds::AST::from(&cst);
        if env::var("DEBUG").is_ok() {
            let _ = std::fs::write("/tmp/ast.txt", format!("{:#?}", &ast));
        }
        out.append(
            &mut dds_parser::pfdds::highlight_ast(ast)
                .into_iter()
                .map(|tup| HighlightMeta::from((tup.0, tup.1, tup.2.as_str(), "AST")))
                .collect::<Vec<HighlightMeta>>(),
        );
        out
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta;

    fn dfmslike_fixture() -> String {
        let input = &r#"
     H OPTION(*nodebugio:*srcstmt)
     FCowEvt    UF A E           K DISK
     FBornEvt   UF A E           K DISK
     FCowEvtL2  IF   E           K DISK     Rename(EVTFMT:VEVTFMT)
     F                                     Prefix(V)
     F**********************************************************************************************
     D**********************************************************************************************
     D LastId          S              8  0
     D QCmdExc         PR                  EXTPGM('QCMDEXC')
     D  Command                    2000
     D  Length                       15  5
     C**********************************************************************************************
      /free
       Exsr $SetLstId;
       Exsr $CrtEvts;
       QCmdExc(Foo:Bar);
       *inlr = *on;

       Begsr $SetLstId;
         SetLL *Loval CowEvtL2;
         If Not %Eof;
           Read CowEvtL2;
             QCmdExc(FOO:BaR);
           LastId = Vid;
         Else;
          LastId = 1;
         Endif;
       Endsr;

     C     $CrtBRNEVT    BegSr
         EID = Id;
         BNAME = 'BESSE';
         BDAT = 20240101;
         Write BORNFMT;
     C                   ENDSR

       Begsr $CrtCowEvt;
         Id = LastId + 1;
         Edat = 20240101;
         Etim = 125959;
         Etyp = 'BORN';
         Write EVTFMT;
       Endsr;

       Begsr $CrtEvts;
         Exsr $CrtCowEvt;
         Exsr $CrtBrnEvt;
       Endsr;                                                                                       "#
            [1..];
        input.to_string()
    }

    #[test]
    fn test_highlights_snapshot() {
        let input = dfmslike_fixture();
        let highlights = highlight_rpgle(input.as_str());
        insta::assert_yaml_snapshot!(highlights);
    }
}
