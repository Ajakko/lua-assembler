use std::{
    fs::{read_dir, File, ReadDir},
    io::{BufReader, Error, Read, Write},
};

pub struct LuaFile {
    pub name: String,
    pub file: File,
}

pub struct LuaAssembler<'a> {
    files: Vec<LuaFile>,
    file_name: &'a str,
}

impl<'a> LuaAssembler<'a> {
    pub fn new(file_name: &'a str) -> Self {
        Self {
            files: Vec::new(),
            file_name,
        }
    }

    pub fn add_files(&mut self, path: &'a str) -> Result<(), Error> {
        let paths: ReadDir = read_dir(path)?;
        for path in paths {
            let display: &String = &path?.path().display().to_string();
            if display != self.file_name {
                self.files.push(LuaFile {
                    name: display.to_owned(),
                    file: File::open(display)?,
                });
            }
        }
        Ok(())
    }

    pub fn compile(&mut self) -> Result<(), Error> {
        let mut index_file: File = File::create(self.file_name)?;
        let mut contents: String = String::new();

        for lua_file in self.files.iter() {
            println!("Appending {:?} to {:?}", &lua_file.name, &self.file_name);
            let mut reader: BufReader<&File> = BufReader::new(&lua_file.file);
            reader.read_to_string(&mut contents)?;
            reader.read(&mut [0xD, 0xA])?; // no new line?
        }
        index_file.write_all(&contents.as_bytes())?;

        Ok(())
    }
}
