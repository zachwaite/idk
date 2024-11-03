use super::srcline::{CSrcline, Srcline};
use crate::field::{
    CodeField, DKeywordsField, DatastructureTypeField, DatatypeField, DecimalsField,
    DefinitionTypeField, DeviceField, EndfileField, ExternalDescriptionField, FKeywordsField,
    FieldResult, FileAdditionField, FileDesignationField, FileFormatField, FileOrganizationField,
    FileSequenceField, FiletypeField, FormtypeField, HKeywordsField, KeyLengthField,
    LimitsProcessingField, NameField, POSField, RecordAddressTypeField, RecordLengthField,
    ReservedField, SequenceField,
};

use crate::free::{
    legacy_tokenize_dspec_kw, legacy_tokenize_fspec_kw, legacy_tokenize_hspec_kw, Op,
};
use crate::meta::partition::partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Spec {
    H {
        sequence: FieldResult<SequenceField>,
        form_type: FieldResult<FormtypeField>,
        keywords: FieldResult<HKeywordsField>,
    },
    F {
        sequence: FieldResult<SequenceField>,
        form_type: FieldResult<FormtypeField>,
        name: FieldResult<NameField>,
        filetype: FieldResult<FiletypeField>,
        file_designation: FieldResult<FileDesignationField>,
        endfile: FieldResult<EndfileField>,
        file_addition: FieldResult<FileAdditionField>,
        file_sequence: FieldResult<FileSequenceField>,
        file_format: FieldResult<FileFormatField>,
        record_length: FieldResult<RecordLengthField>,
        limits_processing: FieldResult<LimitsProcessingField>,
        keylength: FieldResult<KeyLengthField>,
        record_address_type: FieldResult<RecordAddressTypeField>,
        file_organization: FieldResult<FileOrganizationField>,
        device: FieldResult<DeviceField>,
        reserved: FieldResult<ReservedField>,
        keywords: FieldResult<FKeywordsField>,
    },
    D {
        sequence: FieldResult<SequenceField>,
        form_type: FieldResult<FormtypeField>,
        name: FieldResult<NameField>,
        external_description: FieldResult<ExternalDescriptionField>,
        datastructure_type: FieldResult<DatastructureTypeField>,
        definition_type: FieldResult<DefinitionTypeField>,
        from_position: FieldResult<POSField>,
        to_length: FieldResult<POSField>,
        datatype: FieldResult<DatatypeField>,
        decimals: FieldResult<DecimalsField>,
        reserved: FieldResult<ReservedField>,
        keywords: FieldResult<DKeywordsField>,
    },
    C {
        code: FieldResult<CodeField>,
    },
}

#[derive(Debug)]
pub enum ParseError {
    EmptyInput,
    Unhandled,
}

// H ---------------------------------------------------------------------------------
fn try_hspec(input: &[Srcline]) -> Option<(Spec, &[Srcline])> {
    let first = input.get(0)?;
    let spec = match first {
        Srcline::H {
            sequence,
            form_type,
            keywords,
        } => {
            let tokens = legacy_tokenize_hspec_kw(keywords);
            let spec = Spec::H {
                sequence: sequence.clone(),
                form_type: form_type.clone(),
                keywords: FieldResult::Ok(HKeywordsField { tokens }),
            };
            Some(spec)
        }
        Srcline::Idk { .. }
        | Srcline::Comment { .. }
        | Srcline::CompilerDirective { .. }
        | Srcline::F { .. }
        | Srcline::FCont { .. }
        | Srcline::D { .. }
        | Srcline::DCont { .. }
        | Srcline::C(_) => None,
    }?;
    if input.len() > 1 {
        Some((spec, &input[1..]))
    } else {
        Some((spec, &[]))
    }
}

// YAGNI
// fn hspec(input: &[Srcline]) -> Result<(Spec, &[Srcline]), ParseError> {
//     try_hspec(&input).ok_or(ParseError::Unhandled)
// }

// F ---------------------------------------------------------------------------------
fn try_fspec(input: &[Srcline]) -> Option<(Spec, &[Srcline])> {
    let mut idx = 0;
    let first = input.get(idx)?;
    idx += 1;
    let spec = match first {
        Srcline::F {
            sequence,
            form_type,
            name,
            filetype,
            file_designation,
            endfile,
            file_addition,
            file_sequence,
            file_format,
            record_length,
            limits_processing,
            keylength,
            record_address_type,
            file_organization,
            device,
            reserved,
            keywords,
        } => {
            let mut cont_keywords = vec![];
            // collect kw results from continuations
            loop {
                if let Some(Srcline::FCont { keywords: kw, .. }) = input.get(idx) {
                    cont_keywords.push(kw);
                    idx += 1;
                } else {
                    break;
                }
            }
            // parse fspec kw into ftokens
            let tokens = legacy_tokenize_fspec_kw(keywords, &cont_keywords);
            let spec = Spec::F {
                sequence: sequence.clone(),
                form_type: form_type.clone(),
                name: name.clone(),
                filetype: filetype.clone(),
                file_designation: file_designation.clone(),
                endfile: endfile.clone(),
                file_addition: file_addition.clone(),
                file_sequence: file_sequence.clone(),
                file_format: file_format.clone(),
                record_length: record_length.clone(),
                limits_processing: limits_processing.clone(),
                keylength: keylength.clone(),
                record_address_type: record_address_type.clone(),
                file_organization: file_organization.clone(),
                device: device.clone(),
                reserved: reserved.clone(),
                keywords: FieldResult::Ok(FKeywordsField { tokens }),
            };
            Some(spec)
        }
        Srcline::Idk { .. }
        | Srcline::Comment { .. }
        | Srcline::CompilerDirective { .. }
        | Srcline::H { .. }
        | Srcline::FCont { .. }
        | Srcline::D { .. }
        | Srcline::DCont { .. }
        | Srcline::C(_) => None,
    }?;
    if input.len() > 1 {
        Some((spec, &input[idx..]))
    } else {
        Some((spec, &[]))
    }
}

// D ---------------------------------------------------------------------------------
fn try_dspec(input: &[Srcline]) -> Option<(Spec, &[Srcline])> {
    let mut idx = 0;
    let first = input.get(idx)?;
    idx += 1;
    let spec = match first {
        Srcline::D {
            sequence,
            form_type,
            name,
            external_description,
            datastructure_type,
            definition_type,
            from_position,
            to_length,
            datatype,
            decimals,
            reserved,
            keywords,
        } => {
            let mut cont_keywords = vec![];
            // collect continuations
            loop {
                if let Some(Srcline::DCont { keywords: kw, .. }) = input.get(idx) {
                    cont_keywords.push(kw);
                    idx += 1;
                } else {
                    break;
                }
            }
            // parse dspec kw into dtokens
            let tokens = legacy_tokenize_dspec_kw(keywords, &cont_keywords);
            let spec = Spec::D {
                sequence: sequence.clone(),
                form_type: form_type.clone(),
                name: name.clone(),
                external_description: external_description.clone(),
                datastructure_type: datastructure_type.clone(),
                definition_type: definition_type.clone(),
                from_position: from_position.clone(),
                to_length: to_length.clone(),
                datatype: datatype.clone(),
                decimals: decimals.clone(),
                reserved: reserved.clone(),
                keywords: FieldResult::Ok(DKeywordsField { tokens }),
            };
            Some(spec)
        }
        Srcline::Idk { .. }
        | Srcline::Comment { .. }
        | Srcline::CompilerDirective { .. }
        | Srcline::H { .. }
        | Srcline::F { .. }
        | Srcline::FCont { .. }
        | Srcline::DCont { .. }
        | Srcline::C(_) => None,
    }?;
    if input.len() > 1 {
        Some((spec, &input[idx..]))
    } else {
        Some((spec, &[]))
    }
}

// C ---------------------------------------------------------------------------------
fn try_cspec_free(input: &[Srcline]) -> Option<(Spec, &[Srcline])> {
    let mut idx = 0;
    let first = input.get(0)?;
    idx += 1;
    let spec = match first {
        Srcline::C(CSrcline::Free {
            code: codefield, ..
        }) => {
            // TODO: collect continuations
            let conts = vec![];
            // TODO: Op parser uses legacy style parser
            let op = Op::from((codefield, conts.as_slice()));
            let fld = CodeField { op };
            let code = FieldResult::Ok(fld);
            let spec = Spec::C { code };
            Some(spec)
        }
        Srcline::Idk { .. }
        | Srcline::Comment { .. }
        | Srcline::CompilerDirective { .. }
        | Srcline::H { .. }
        | Srcline::F { .. }
        | Srcline::FCont { .. }
        | Srcline::D { .. }
        | Srcline::DCont { .. }
        | Srcline::C(_) => None,
    }?;
    if input.len() > 1 {
        Some((spec, &input[idx..]))
    } else {
        Some((spec, &[]))
    }
}

fn try_cspec_extf2(input: &[Srcline]) -> Option<(Spec, &[Srcline])> {
    todo!()
}

fn try_cspec_traditional(input: &[Srcline]) -> Option<(Spec, &[Srcline])> {
    let mut idx = 0;
    let first = input.get(0)?;
    idx += 1;
    let spec = match first {
        Srcline::C(CSrcline::Traditional {
            factor1, operation, ..
        }) => {
            // TODO: collect continuations
            let conts = vec![];
            // TODO: Op parser uses legacy style parser
            let op = Op::from(((operation, factor1), conts.as_slice()));
            let fld = CodeField { op };
            let code = FieldResult::Ok(fld);
            let spec = Spec::C { code };
            Some(spec)
        }
        Srcline::Idk { .. }
        | Srcline::Comment { .. }
        | Srcline::CompilerDirective { .. }
        | Srcline::H { .. }
        | Srcline::F { .. }
        | Srcline::FCont { .. }
        | Srcline::D { .. }
        | Srcline::DCont { .. }
        | Srcline::C(_) => None,
    }?;
    if input.len() > 1 {
        Some((spec, &input[idx..]))
    } else {
        Some((spec, &[]))
    }
}

fn spec(input: &[Srcline]) -> Result<(Spec, &[Srcline]), ParseError> {
    // This parser implements the `choice` pattern using partial parsers.
    // Input flows through the first parser and if the parser succeeds,
    // it will return, else input flows to the next parser.
    // If no parsers succeed, a ParseError is returned.
    let parse_hspec = || try_hspec(input);
    let parse_fspec = || try_fspec(input);
    let parse_dspec = || try_dspec(input);
    let parse_cspec_free = || try_cspec_free(input);
    // let parse_cspec_extf2 = || try_cspec_extf2(input); // Not Implemented
    let parse_cspec_traditional = || try_cspec_traditional(input);
    parse_hspec()
        .or_else(parse_fspec)
        .or_else(parse_dspec)
        .or_else(parse_cspec_free)
        .or_else(parse_cspec_traditional)
        .ok_or(ParseError::Unhandled)
}

pub fn ast(input: &mut [Srcline]) -> Result<(Vec<Spec>, &[Srcline]), ParseError> {
    // This filter splits the input into things you can handle and things you
    // can't. This will prevent erroring out due to not implemented features.
    // This is mostly useful for development, but think of it like a breaker
    // box or water supply manifold... it's better to have it available for
    // maintenance than to need to stop using the whole thing.
    let (keep, ignore) = partition(input, |line| match line {
        Srcline::Idk { .. } => false,
        Srcline::Comment { .. } => false,
        Srcline::CompilerDirective { .. } => false,
        Srcline::H { .. } => true,
        Srcline::F { .. } => true,
        Srcline::FCont { .. } => true,
        Srcline::D { .. } => true,
        Srcline::DCont { .. } => true,
        Srcline::C(_) => true,
    });
    let mut _input = keep;
    let mut outs: Vec<Spec> = vec![];
    loop {
        let (s, _rest) = spec(&_input)?;
        outs.push(s);
        if _rest.len() == 0 {
            break;
        } else {
            _input = _rest;
        }
    }
    Ok((outs, ignore))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::srcline::srcline_from_specline, cst::CST};
    use insta;

    #[test]
    fn test_hspec_01() {
        // empty input
        let lines = vec![];
        let observed = try_hspec(&lines);
        insta::assert_yaml_snapshot!(observed);
    }

    #[test]
    fn test_hspec_02() {
        // 1 hline
        let input = r#"
     H OPTION(*nodebugio:*srcstmt)                                                                  "#[1..].to_string();
        let cst = CST::try_from(input.as_str()).unwrap();
        let lines = cst
            .lines
            .iter()
            .map(|line| srcline_from_specline(line))
            .collect::<Vec<Srcline>>();
        let observed = try_hspec(&lines);
        insta::assert_yaml_snapshot!(observed);
    }

    #[test]
    fn test_fspec_01() {
        // empty input
        let lines = vec![];
        let observed = try_fspec(&lines);
        insta::assert_yaml_snapshot!(observed);
    }

    #[test]
    fn test_fspec_02() {
        // 1 fline
        let input = r#"
     FCowEvt    UF A E           K DISK                                                             "#
            [1..].to_string();
        let cst = CST::try_from(input.as_str()).unwrap();
        let lines = cst
            .lines
            .iter()
            .map(|line| srcline_from_specline(line))
            .collect::<Vec<Srcline>>();
        let observed = try_fspec(&lines);
        insta::assert_yaml_snapshot!(observed);
    }

    #[test]
    fn test_fspec_03() {
        // fline + continuation
        let input = r#"
     FCowEvtL2  IF   E           K DISK     Rename(EVTFMT:VEVTFMT)                                  
     F                                     Prefix(V)                                                "#
            [1..].to_string();
        let cst = CST::try_from(input.as_str()).unwrap();
        let lines = cst
            .lines
            .iter()
            .map(|line| srcline_from_specline(line))
            .collect::<Vec<Srcline>>();
        let observed = try_fspec(&lines);
        insta::assert_yaml_snapshot!(observed);
    }

    #[test]
    fn test_dspec_01() {
        // empty input
        let lines = vec![];
        let observed = try_dspec(&lines);
        insta::assert_yaml_snapshot!(observed);
    }

    #[test]
    fn test_dspec_02() {
        // 1 dline
        let input = r#"
     D LastId          S              8  0                                                          "#
            [1..].to_string();
        let cst = CST::try_from(input.as_str()).unwrap();
        let lines = cst
            .lines
            .iter()
            .map(|line| srcline_from_specline(line))
            .collect::<Vec<Srcline>>();
        let observed = try_dspec(&lines);
        insta::assert_yaml_snapshot!(observed);
    }

    #[test]
    fn test_cspec_free_01() {
        // empty input
        let lines = vec![];
        let observed = try_cspec_free(&lines);
        insta::assert_yaml_snapshot!(observed);
    }

    #[test]
    fn test_cspec_free_02() {
        // 1 cline
        let input = r#"
       Exsr $SetLstId;                                                                              "#
            [1..].to_string();
        let cst = CST::try_from(input.as_str()).unwrap();
        let lines = cst
            .lines
            .iter()
            .map(|line| srcline_from_specline(line))
            .collect::<Vec<Srcline>>();
        let observed = try_cspec_free(&lines);
        insta::assert_yaml_snapshot!(observed);
    }

    // TODO: extf2

    #[test]
    fn test_cspec_traditional_01() {
        // empty input
        let lines = vec![];
        let observed = try_cspec_traditional(&lines);
        insta::assert_yaml_snapshot!(observed);
    }

    #[test]
    fn test_cspec_traditional_02() {
        // 1 cline
        let input = r#"
     C     $CrtBRNEVT    BegSr                                                                      "#
            [1..].to_string();
        let cst = CST::try_from(input.as_str()).unwrap();
        let lines = cst
            .lines
            .iter()
            .map(|line| srcline_from_specline(line))
            .collect::<Vec<Srcline>>();
        let observed = try_cspec_traditional(&lines);
        insta::assert_yaml_snapshot!(observed);
    }

    #[test]
    fn test_ast_snapshot() {
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
        let cst = CST::try_from(input).unwrap();
        let mut lines = cst
            .lines
            .iter()
            .map(|line| srcline_from_specline(line))
            .collect::<Vec<Srcline>>();
        let (specs, rest) = ast(&mut lines).unwrap();
        for ln in rest.iter() {
            println!("{:?}", ln);
        }
        // expect 3 Comments and a CompilerDirective to be ignored
        assert_eq!(rest.len(), 4);
        insta::assert_yaml_snapshot!(specs);
    }
}
