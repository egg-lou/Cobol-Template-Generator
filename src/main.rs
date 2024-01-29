use std::fs::File;
use std::io::{self, Write};

use clap::{App, Arg};

fn main() {
    let matches = App::new("COBOL File Creator")
    .version("1.0")
    .author("Egglou")
    .about("Creates a COBOL file")
    .arg(
        Arg::with_name("filename")
        .help("Sets the output filename")
        .required(true)
        .index(1),
    )
    .get_matches();

let output_file = matches.value_of("filename").unwrap();

if let Err(err) = create_cob_file(output_file) {
    eprintln!("Error: {}", err);
    std::process::exit(1);
    }
    
    println!("File {}.cob created successfully", output_file);
}

fn create_txt_file(input_file: &str) -> io::Result<()> {
    let mut file = File::create(format!("{}.txt", input_file))?;
    file.write_all(b"")?;
    Ok(())
}

fn create_cob_file(file_name: &str) -> io::Result<()> {
    let mut file = File::create(format!("{}.cob", file_name))?;
    
    fn read_line() -> String {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        line.trim().to_string()
    }

    println!();

    print!("Enter Program-ID: ");
    io::stdout().flush().unwrap();
    let program_id = read_line().to_uppercase();

    print!("Enter Author: ");
    io::stdout().flush().unwrap();
    let author = read_line().to_uppercase();

    print!("Enter Installation: ");
    io::stdout().flush().unwrap();
    let installation = read_line().to_uppercase();

    print!("Enter Remarks: ");
    io::stdout().flush().unwrap();
    let remarks = read_line().to_uppercase();

    print!("Enter Infile variable name: ");
    io::stdout().flush().unwrap();
    let infile_var = read_line().to_uppercase();
    
    print!("Enter Outfile variable name: ");
    io::stdout().flush().unwrap();
    let outfile_var = read_line().to_uppercase();

    print!("Enter Input file name: ");
    io::stdout().flush().unwrap();
    let input_file = read_line().to_uppercase();
    
    print!("Enter Output file name: ");
    io::stdout().flush().unwrap();
    let output_file = read_line().to_uppercase();
    
    match create_txt_file(&input_file) {
        Ok(_) => println!("Created {}.txt", input_file),
        Err(e) => eprintln!("failed to create {}.txt: {}", input_file, e)
    };

    write!(
        file,
        r#"       IDENTIFICATION DIVISION.
       PROGRAM-ID. {}.
      *AUTHOR. {}.
      *INSTALLATION. {}.
      *DATE-WRITTEN. {}.
      *DATE-COMPILED. {}.
      *SECURITY. Exclusive.
      *REMARKS {}.

       ENVIRONMENT DIVISION.
       CONFIGURATION SECTION.
       SOURCE-COMPUTER. ASUSTEK-PC.
       OBJECT-COMPUTER. ASUSTEK-PC.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT {} ASSIGN TO '{}.txt'.
           SELECT {} ASSIGN TO '{}'.

       DATA DIVISION.
       FILE SECTION.
       FD  {}
           LABEL RECORDS ARE STANDARD
           RECORD CONTAINS    CHARACTERS
           DATA RECORD IS INREC.
       01  INREC.
           
       FD  {}
           LABEL RECORDS ARE OMITTED
           RECORD CONTAINS 80 CHARACTERS
           DATA RECORD IS OUTREC.
       01  OUTREC.
           02 FILLER PIC X(80).

       WORKING-STORAGE SECTION.
       01  EOFSW PIC 9 VALUE ZERO.

       SCREEN SECTION.
       01  SCRE.
           02 BLANK SCREEN.

       PROCEDURE DIVISION.
       MAIN-RTN.
       INIT-RTN.
           OPEN INPUT {}, OUTPUT {}.
           READ {} AT END PERFORM END-RTN
           GO TO INIT-RTN-END.


           PERFORM HEADING-RTN.
       INIT-RTN-END.

       END-RTN.
           MOVE 1 TO EOFSW.
           DISPLAY 'EMPTY FILE' LINE 3 COLUMN 30.

       HEADING-RTN.

       PROCESS-RTN.
           DISPLAY SCRE.
           DISPLAY 'PROCESSING' LINE 3 COLUMN 30.


           READ {} AT END MOVE 1 TO EOFSW PERFORM BREAK-RTN.

       BREAK-RTN.

       FINISH-RTN.
           CLOSE {}, {}.
           DISPLAY 'PROCESSING DOBE!' LINE 5 COLUMN 30"#,
        program_id,
        author,
        installation,
        chrono::Local::now().format("%Y-%m-%d").to_string(),
        chrono::Local::now().format("%Y-%m-%d").to_string(),
        remarks,
        infile_var,
        input_file,
        outfile_var,
        output_file,
        infile_var,
        outfile_var,
        infile_var,
        outfile_var,
        infile_var,
        infile_var,
        infile_var,
        outfile_var,
    )?;

    Ok(())

}
