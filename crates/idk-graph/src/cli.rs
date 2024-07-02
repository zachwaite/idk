use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;

use crate::parser::{parse_program, Parser as RpgleParser};
use crate::renderer::render_dot;
use rpgle_lexer::new_lexer;

#[derive(Subcommand, Debug)]
enum Command {
    Dot,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// command: what action to perform
    #[command(subcommand)]
    command: Command,

    /// input: either stdin or filepath
    #[clap(value_parser, default_value = "-", global = true)]
    input: String,

    /// output: either stdout or filepath
    #[clap(value_parser, default_value = "-", global = true)]
    output: String,
}

fn read_input(input: &str) -> Result<String, Box<dyn Error>> {
    let mut buf = String::new();
    let mut rdr: Box<dyn io::Read> = match input {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(input)?),
    };
    rdr.read_to_string(&mut buf)?;
    Ok(buf)
}

fn write_output(output: &str, target: &str) -> Result<(), Box<dyn Error>> {
    let mut writer: Box<dyn io::Write> = match target {
        "-" => Box::new(io::stdout()),
        _ => Box::new(File::create(&Path::new(target))?),
    };
    writer.write(output.as_bytes())?;
    Ok(())
}

fn dot(input: &str) -> Result<String, Box<dyn Error>> {
    let lexer = new_lexer(&input);
    let parser = RpgleParser::new(&lexer).unwrap();
    let pgm = parse_program(&parser)?;
    let output = render_dot(pgm);
    Ok(output)
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.command {
        Command::Dot => {
            let input = read_input(args.input.as_str())?;
            let output = dot(&input)?;
            write_output(&output, args.output.as_str())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dot1() {
        let input = &r#"
     H OPTION(*nodebugio:*srcstmt)                                                                  
     FCowEvt    UF A E           K DISK                                                             
     FBornEvt   UF A E           K DISK                                                             
     FCowEvtL2  IF   E           K DISK     Rename(EVTFMT:VEVTFMT) Prefix(V)                        
     F**********************************************************************************************
     D**********************************************************************************************
     D LastId          S              8  0                                                          
     C**********************************************************************************************
      /free                                                                                         
       Exsr $SetLstId;                                                                              
       Exsr $CrtEvts;                                                                               
       *inlr = *on;                                                                                 
                                                                                                    
       Begsr $SetLstId;                                                                             
         SetLL *Loval CowEvtL2;                                                                     
         If Not %Eof;                                                                               
           Read CowEvtL2;                                                                           
           LastId = Vid;                                                                            
         Else;                                                                                      
          LastId = 1;                                                                               
         Endif;                                                                                     
       Endsr;                                                                                       
                                                                                                    
       Begsr $CrtBrnEvt;                                                                            
         EID = Id;                                                                                  
         BNAME = 'BESSE';                                                                           
         BDAT = 20240101;                                                                           
         Write BORNFMT;                                                                             
       Endsr;                                                                                       
                                                                                                    
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
       Endsr;                                                                                       
"#[1..];
        let expected = &r#"
digraph g {
  fontname="Helvetica,Arial,sans-serif"
  node [fontname="Helvetica,Arial,sans-serif"]
  edge [fontname="Helvetica,Arial,sans-serif"]
  graph [fontsize=30 labelloc="t" label="" splines=true overlap=false rankdir = "LR"];
  ratio = auto;
  
  "MAIN" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        MAIN
        </font>
      </td>
    </tr>
    
  </table>
  > ];
    

  "$SETLSTID" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $SETLSTID
        </font>
      </td>
    </tr>
    
  </table>
  > ];
    

  "$CRTEVTS" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $CRTEVTS
        </font>
      </td>
    </tr>
    
  </table>
  > ];
    

  "$CRTCOWEVT" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $CRTCOWEVT
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Write: EVTFMT</td></tr>
  </table>
  > ];
    

  "$CRTBRNEVT" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $CRTBRNEVT
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Write: BORNFMT</td></tr>
  </table>
  > ];
    
  "MAIN" -> "$SETLSTID" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
"MAIN" -> "$CRTEVTS" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
"$CRTEVTS" -> "$CRTCOWEVT" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
"$CRTEVTS" -> "$CRTBRNEVT" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
}
"#;
        let observed = dot(&input).unwrap();
        assert_eq!(&observed, expected);
    }

    #[test]
    fn test_dot2() {
        let input = &r#"
     H OPTION(*nodebugio:*srcstmt)                                                                  
     FCowEvt    UF A E           K DISK                                                             
     FBornEvt   UF A E           K DISK                                                             
     FCowEvtL2  IF   E           K DISK     Rename(EVTFMT:VEVTFMT) Prefix(V)                        
     F**********************************************************************************************
     D**********************************************************************************************
     D LastId          S              8  0                                                          
     C**********************************************************************************************
      /free                                                                                         
       update foo;                                                                                  
                                        UPDATE fOo;                                                 
       Exsr $SetLstId;                                                                              
       Exsr $CrtEvts;                                                                               
       *inlr = *on;                                                                                 
                                                                                                    
       Begsr $SetLstId;                                                                             
         SetLL *Loval CowEvtL2;                                                                     
         If Not %Eof;                                                                               
           Read CowEvtL2;                                                                           
           LastId = Vid;                                                                            
           update bar;                                                                              
         Else;                                                                                      
          LastId = 1;                                                                               
          write baz;                                                                                
         Endif;                                                                                     
       Endsr;                                                                                       
                                                                                                    
       Begsr $CrtBrnEvt;                                                                            
         EID = Id;                                                                                  
         BNAME = 'BESSE';                                                                           
         BDAT = 20240101;                                                                           
         Write BORNFMT;                                                                             
              write FOO;                                                                            
       Endsr;                                                                                       
                                                                                                    
       Begsr $CrtCowEvt;                                                                            
         Id = LastId + 1;                                                                           
         Edat = 20240101;                                                                           
         Etim = 125959;                                                                             
         Etyp = 'BORN';                                                                             
         Write EVTFMT;                                                                              
              update FOO;                                                                           
       Endsr;                                                                                       
                                                                                                    
       Begsr $CrtEvts;                                                                              
              update FOO;                                                                           
         Exsr $CrtCowEvt;                                                                           
         Exsr $CrtBrnEvt;                                                                           
       Endsr;                                                                                       
"#[1..];
        let expected = &r#"
digraph g {
  fontname="Helvetica,Arial,sans-serif"
  node [fontname="Helvetica,Arial,sans-serif"]
  edge [fontname="Helvetica,Arial,sans-serif"]
  graph [fontsize=30 labelloc="t" label="" splines=true overlap=false rankdir = "LR"];
  ratio = auto;
  
  "MAIN" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        MAIN
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Update: FOO</td></tr>
  </table>
  > ];
    

  "$SETLSTID" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $SETLSTID
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Update: BAR</td></tr><tr><td align="left" port="r0">Write: BAZ</td></tr>
  </table>
  > ];
    

  "$CRTEVTS" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $CRTEVTS
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Update: FOO</td></tr>
  </table>
  > ];
    

  "$CRTCOWEVT" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $CRTCOWEVT
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Update: FOO</td></tr><tr><td align="left" port="r0">Write: EVTFMT</td></tr>
  </table>
  > ];
    

  "$CRTBRNEVT" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $CRTBRNEVT
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Write: BORNFMT</td></tr><tr><td align="left" port="r0">Write: FOO</td></tr>
  </table>
  > ];
    
  "MAIN" -> "$SETLSTID" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
"MAIN" -> "$CRTEVTS" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
"$CRTEVTS" -> "$CRTCOWEVT" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
"$CRTEVTS" -> "$CRTBRNEVT" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
}
"#;
        let observed = dot(&input).unwrap();
        assert_eq!(&observed, expected);
    }

    #[test]
    fn test_dot3_cspec_begsr() {
        let input = &r#"
     H OPTION(*nodebugio:*srcstmt)                                                                  
     FCowEvt    UF A E           K DISK                                                             
     FBornEvt   UF A E           K DISK                                                             
     FCowEvtL2  IF   E           K DISK     Rename(EVTFMT:VEVTFMT) Prefix(V)                        
     F**********************************************************************************************
     D**********************************************************************************************
     D LastId          S              8  0                                                          
     C**********************************************************************************************
      /free                                                                                         
       update foo;                                                                                  
                                        UPDATE fOo;                                                 
       Exsr $SetLstId;                                                                              
       Exsr $CrtEvts;                                                                               
       *inlr = *on;                                                                                 
                                                                                                    
     C     $SetLstId     BegSr                                                                      
         SetLL *Loval CowEvtL2;                                                                     
         If Not %Eof;                                                                               
           Read CowEvtL2;                                                                           
           LastId = Vid;                                                                            
           update bar;                                                                              
         Else;                                                                                      
          LastId = 1;                                                                               
          write baz;                                                                                
         Endif;                                                                                     
       Endsr;                                                                                       
                                                                                                    
     C     $CrtBRNEVT    BegSr                                                                      
         EID = Id;                                                                                  
         BNAME = 'BESSE';                                                                           
         BDAT = 20240101;                                                                           
         Write BORNFMT;                                                                             
              write FOO;                                                                            
       Endsr;                                                                                       
                                                                                                    
     C     $CRTCOWEVT    BegSr                                                                      
         Id = LastId + 1;                                                                           
         Edat = 20240101;                                                                           
         Etim = 125959;                                                                             
         Etyp = 'BORN';                                                                             
         Write EVTFMT;                                                                              
              update FOO;                                                                           
       Endsr;                                                                                       
                                                                                                    
       Begsr $CrtEvts;                                                                              
              update FOO;                                                                           
         Exsr $CrtCowEvt;                                                                           
         Exsr $CrtBrnEvt;                                                                           
       Endsr;                                                                                       
"#[1..];

        let expected = &r#"
digraph g {
  fontname="Helvetica,Arial,sans-serif"
  node [fontname="Helvetica,Arial,sans-serif"]
  edge [fontname="Helvetica,Arial,sans-serif"]
  graph [fontsize=30 labelloc="t" label="" splines=true overlap=false rankdir = "LR"];
  ratio = auto;
  
  "MAIN" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        MAIN
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Update: FOO</td></tr>
  </table>
  > ];
    

  "$SETLSTID" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $SETLSTID
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Update: BAR</td></tr><tr><td align="left" port="r0">Write: BAZ</td></tr>
  </table>
  > ];
    

  "$CRTEVTS" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $CRTEVTS
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Update: FOO</td></tr>
  </table>
  > ];
    

  "$CRTCOWEVT" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $CRTCOWEVT
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Update: FOO</td></tr><tr><td align="left" port="r0">Write: EVTFMT</td></tr>
  </table>
  > ];
    

  "$CRTBRNEVT" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $CRTBRNEVT
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Write: BORNFMT</td></tr><tr><td align="left" port="r0">Write: FOO</td></tr>
  </table>
  > ];
    
  "MAIN" -> "$SETLSTID" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
"MAIN" -> "$CRTEVTS" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
"$CRTEVTS" -> "$CRTCOWEVT" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
"$CRTEVTS" -> "$CRTBRNEVT" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
}
"#;
        let observed = dot(&input).unwrap();
        assert_eq!(&observed, expected);
    }

    #[test]
    fn test_dot4_cspec_endsr() {
        let input = &r#"
     H OPTION(*nodebugio:*srcstmt)                                                                  
     FCowEvt    UF A E           K DISK                                                             
     FBornEvt   UF A E           K DISK                                                             
     FCowEvtL2  IF   E           K DISK     Rename(EVTFMT:VEVTFMT) Prefix(V)                        
     F**********************************************************************************************
     D**********************************************************************************************
     D LastId          S              8  0                                                          
     C**********************************************************************************************
      /free                                                                                         
       update foo;                                                                                  
                                        UPDATE fOo;                                                 
       Exsr $SetLstId;                                                                              
       Exsr $CrtEvts;                                                                               
       *inlr = *on;                                                                                 
                                                                                                    
     C     $SetLstId     BegSr                                                                      
         SetLL *Loval CowEvtL2;                                                                     
         If Not %Eof;                                                                               
           Read CowEvtL2;                                                                           
           LastId = Vid;                                                                            
           update bar;                                                                              
         Else;                                                                                      
          LastId = 1;                                                                               
          write baz;                                                                                
         Endif;                                                                                     
       Endsr;                                                                                       
                                                                                                    
     C     $CrtBRNEVT    BegSr                                                                      
         EID = Id;                                                                                  
         BNAME = 'BESSE';                                                                           
         BDAT = 20240101;                                                                           
         Write BORNFMT;                                                                             
              write FOO;                                                                            
     C                   ENDSR                                                                      
                                                                                                    
     C     $CRTCOWEVT    BegSr                                                                      
         Id = LastId + 1;                                                                           
         Edat = 20240101;                                                                           
         Etim = 125959;                                                                             
         Etyp = 'BORN';                                                                             
         Write EVTFMT;                                                                              
              update FOO;                                                                           
       Endsr;                                                                                       
                                                                                                    
       Begsr $CrtEvts;                                                                              
              update FOO;                                                                           
         Exsr $CrtCowEvt;                                                                           
         Exsr $CrtBrnEvt;                                                                           
       Endsr;                                                                                       
"#[1..];

        let expected = &r#"
digraph g {
  fontname="Helvetica,Arial,sans-serif"
  node [fontname="Helvetica,Arial,sans-serif"]
  edge [fontname="Helvetica,Arial,sans-serif"]
  graph [fontsize=30 labelloc="t" label="" splines=true overlap=false rankdir = "LR"];
  ratio = auto;
  
  "MAIN" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        MAIN
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Update: FOO</td></tr>
  </table>
  > ];
    

  "$SETLSTID" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $SETLSTID
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Update: BAR</td></tr><tr><td align="left" port="r0">Write: BAZ</td></tr>
  </table>
  > ];
    

  "$CRTEVTS" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $CRTEVTS
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Update: FOO</td></tr>
  </table>
  > ];
    

  "$CRTCOWEVT" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $CRTCOWEVT
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Update: FOO</td></tr><tr><td align="left" port="r0">Write: EVTFMT</td></tr>
  </table>
  > ];
    

  "$CRTBRNEVT" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        $CRTBRNEVT
        </font>
      </td>
    </tr>
    <tr><td align="left" port="r0">Write: BORNFMT</td></tr><tr><td align="left" port="r0">Write: FOO</td></tr>
  </table>
  > ];
    
  "MAIN" -> "$SETLSTID" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
"MAIN" -> "$CRTEVTS" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
"$CRTEVTS" -> "$CRTCOWEVT" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
"$CRTEVTS" -> "$CRTBRNEVT" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];
}
"#;
        let observed = dot(&input).unwrap();
        // let _ = std::fs::write("/tmp/expected.gv", expected);
        // let _ = std::fs::write("/tmp/observed.gv", &observed);
        assert_eq!(&observed, expected);
    }
}
