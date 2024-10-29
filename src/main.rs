use std::{fs, io, io::Write};


fn byte_shift(text: Vec<u8>, shift_by: u8, backwards: bool) -> Vec<u8> {
    text.iter()
        .map(|byte| {
            if backwards {
                byte.wrapping_sub(shift_by)
            }else {
                byte.wrapping_add(shift_by)
            }
        })

        .collect()
}


fn main(){
    loop {
        println!("\n\nMenu:");
        println!("me1: Encrpty the files");
        println!("2: Decrypt the file");
        println!("0 : Quit");

        println!("Your choice");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let input = buf.as_str().trim();

        // let trimmed_path = buf.as_str().trim();


        match input {
            "1" => {
                println!("Path to file encryted");
                let mut buf  = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                let trimmed_path = buf.as_str().trim();

                match fs::read(trimmed_path){
                    Ok(contents) => {
                        let new_contents = byte_shift(contents, 2, false);
                        let mut new_file = fs::OpenOptions::new().write(true).open(trimmed_path).unwrap();


                        if let Err(e) = new_file.write_all(&new_contents) {
                            println!("Error  :\t {:?}", e)
                        }
                    }

                    Err(e) => {
                        println!("Couldn't open the file {trimmed_path} : {e}")
                    }

                }
            }
            "2" => {
                println!("Path to file decryted");
                let mut buf  = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                let trimmed_path = buf.as_str().trim();

                match fs::read(trimmed_path){
                    Ok(contents) => {
                        let new_contents = byte_shift(contents, 2, true);
                        let mut new_file = fs::OpenOptions::new().write(true).open(trimmed_path).unwrap();


                        if let Err(e) = new_file.write_all(&new_contents) {
                            println!("Error  :\t {:?}", e)
                        }
                    }

                    Err(e) => {
                        println!("Couldn't open the file {trimmed_path} : {e}")
                    }

                }
            }
            "0" => {
                return();
            }

            _ => {
                println!("Error : \t INVALID INPUT {input}. Please try again ." );
            }

            }
       }
}
