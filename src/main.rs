mod block;

use block::{Block, Element};
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::Read,
};

#[derive(Debug)]
enum ClassType {
    Block,
    BlockMod,
    Element,
    ElementMod,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Необходимо передать путь до файла в аргументах командной строки");
        return;
    }

    let file_path = &args[1];
    let mut file = File::open(file_path).expect("Не удалось открыть файл");
    let mut file_content = String::new();

    file.read_to_string(&mut file_content)
        .expect("Не удалось прочитать файл");

    file_content = file_content.replace("\n", "");
    let all_classes: Vec<_> = file_content
        .split(['<', '>'])
        .filter(|&s| s.contains("class=\""))
        .map(|s| {
            s.split("class=\"")
                .nth(1)
                .expect(&format!(
                    "Ошибка в поиске классов, ошибка при обработке: \n{:?}",
                    s.replace(" ", "")
                ))
                .to_string()
        })
        .map(|s| {
            s.split('"')
                .nth(0)
                .expect(&format!(
                    "Ошибка в поиске классов, ошибка при обработке: \n{:?}",
                    s.replace(" ", "")
                ))
                .to_string()
        })
        .collect::<HashSet<String>>()
        .into_iter()
        .map(|s| {
            s.split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let mut blocks: HashMap<String, Block> = HashMap::new();

    for class in all_classes {
        match define_class_type(&class) {
            ClassType::Block => {
                blocks.entry(class.to_string()).or_insert(Block {
                    name: class,
                    mods: vec![],
                    elements: vec![],
                });
            }
            ClassType::Element => {
                let class_items: Vec<_> = class.split("__").collect();
                let block = class_items[0];
                let element = class_items[1];

                blocks
                    .entry(block.to_string())
                    .and_modify(|b| {
                        b.elements.push(Element {
                            name: element.to_string(),
                            mods: vec![],
                        })
                    })
                    .or_insert(Block {
                        name: block.to_string(),
                        mods: vec![],
                        elements: vec![Element {
                            name: element.to_string(),
                            mods: vec![],
                        }],
                    });
            }
            ClassType::BlockMod => {
                let class_items: Vec<_> = class.split("--").collect();
                let block = class_items[0];
                let block_mod = class_items[1];

                blocks
                    .entry(block.to_string())
                    .and_modify(|b| b.mods.push(block_mod.to_string()))
                    .or_insert(Block {
                        name: block.to_string(),
                        mods: vec![block_mod.to_string()],
                        elements: vec![],
                    });
            }
            ClassType::ElementMod => {
                let class_items: Vec<_> = class.split("__").collect();
                let block = class_items[0];

                let class_items: Vec<_> = class_items[1].split("--").collect();

                let element = class_items[0];
                let element_mod = class_items[1];

                blocks
                    .entry(block.to_string())
                    .and_modify(|b| {
                        let element = b.elements.iter_mut().find(|e| e.name == element);
                        if let Some(e) = element {
                            e.mods.push(element_mod.to_string());
                        }
                    })
                    .or_insert(Block {
                        name: block.to_string(),
                        mods: vec![],
                        elements: vec![Element {
                            name: element.to_string(),
                            mods: vec![element_mod.to_string()],
                        }],
                    });
            }
        };
    }

    for b in blocks.values() {
        println!("{}", b.generate_scss());
    }
}

fn define_class_type(class: &str) -> ClassType {
    if class.contains("--") && class.contains("__") {
        return ClassType::ElementMod;
    }

    if class.contains("__") {
        return ClassType::Element;
    }

    if class.contains("--") {
        return ClassType::BlockMod;
    }

    ClassType::Block
}
