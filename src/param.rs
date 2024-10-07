use std::io::Read;
use yaml_rust::YamlLoader;

pub struct Param
{
    yaml : Vec<yaml_rust::Yaml>
}

impl Param {
    pub fn new()->Option<Self>
    {
        let file_name = match std::env::args().nth(1) {
            Some(name)=> {name}
            None=>{String::new()}
        };

        match std::fs::File::open(file_name) {
            Ok(mut f)=>{
                let mut content = String::new();

                let _ = f.read_to_string(&mut content);

                match YamlLoader::load_from_str(&content) {
                    Ok(docs)=>{
                        Some(Param { yaml: docs })
                    }
                    Err(_e)=>{
                        None
                    }
                }
            }
            Err(_e)=>{
                None
            }
        }
    }

    pub fn get_str_parameter(&self, name : &str)->String
    {
        self.yaml[0]["fls"][name].as_str().unwrap().to_string()
    }
}