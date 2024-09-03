use rpgle_parser::{Span, Spec, CST};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct HighlightMeta {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
    pub hl_group: String,
}

impl fmt::Display for HighlightMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "({}, {}) -> ({}, {}): {}",
            self.start_row, self.start_col, self.end_row, self.end_col, self.hl_group
        );
        write!(f, "{}", s)
    }
}

// Deprecate this
impl HighlightMeta {
    pub fn new(sr: usize, sc: usize, er: usize, ec: usize, hl_group: &str) -> Self {
        Self {
            start_row: sr,
            start_col: sc,
            end_row: er,
            end_col: ec,
            hl_group: hl_group.to_string(),
        }
    }
}

impl From<(Span, String)> for HighlightMeta {
    fn from(value: (Span, String)) -> Self {
        Self {
            start_row: value.0.start.row,
            start_col: value.0.start.col,
            end_row: value.0.end.row,
            end_col: value.0.end.col,
            hl_group: value.1.to_string(),
        }
    }
}

// main
pub fn highlight_all(txt: &str) -> Vec<HighlightMeta> {
    if let Ok(cst) = CST::try_from(txt) {
        cst.specs
            .iter()
            .flat_map(|s| s.highlight())
            .map(|tup| HighlightMeta::from(tup))
            .collect::<Vec<HighlightMeta>>()
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta;

    #[test]
    fn test_highlights() {
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
        let highlights = highlight_all(input);
        insta::assert_yaml_snapshot!(highlights);
    }
}
