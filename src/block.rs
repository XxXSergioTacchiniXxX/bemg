#[derive(Debug)]
pub struct Block {
    pub name: String,
    pub mods: Vec<String>,
    pub elements: Vec<Element>,
}

impl Block {
    pub fn generate_scss(&self) -> String {
        let mut result = String::new();

        let block_mods_scss = self
            .mods
            .iter()
            .map(|m| format!("\n   &--{} {{}}", m))
            .collect::<Vec<String>>()
            .join("");

        let mut elements_scss = String::new();

        for element in self.elements.iter() {
            let scss = element.generate_scss();
            elements_scss += &scss;
        }

        result = result + "." + &self.name + " {";
        result = result + &block_mods_scss;
        result = result + &elements_scss;
        result = result + "\n}";
        result
    }
}

#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub mods: Vec<String>,
}

impl Element {
    pub fn generate_scss(&self) -> String {
        let element_mods_scss = self
            .mods
            .iter()
            .map(|m| format!("\n   &--{} {{}} \n", m))
            .collect::<Vec<String>>()
            .join("");

        format!("\n   &__{} {{{}}}", self.name, element_mods_scss)
    }
}
