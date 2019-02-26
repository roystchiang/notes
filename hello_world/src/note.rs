extern crate serde;

use std::path::Path;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Note {
    id: String,
    title: String,
    parents: Vec<String>,
    description: String,
}

#[derive(Debug)]
pub struct Graph {
    children: Vec<Graph>,
    note: Option<Note>,
    id: Option<String>
}

fn parse_note(content: &String) -> Result<Note, serde_json::Error> {
    return serde_json::from_str(&content);
}

fn read_note(path: &Path) -> Option<Note>{
    let contents = fs::read_to_string(path).expect(
        "Something went wrong while reading file");

    match parse_note(&contents) {
        Ok(parsed_note) => Some(parsed_note),
        Err(_err) => None
    }
}

pub fn list_notes(path: &str) -> Graph {
    // Given a path, return a list of Paths
    let path = Path::new(path);
    let path_iterator = fs::read_dir(path).expect("unable to find directory");

    let iter = path_iterator.map(|x| read_note(x.unwrap().path().as_path()));
    let graph = create_graph(iter);
    return graph;
}

fn create_graph<T: Iterator<Item=Option<Note>>>(notes: T) -> Graph{
    let mut root_graph= Graph{children: Vec::new(), note: None, id: None};
    for note in notes {
        if note.is_some() {
            let cur_note = note.unwrap();
            update_graph(&mut root_graph, cur_note);
            update_graph(&mut root_graph, cur_note);
        }
    }
    root_graph

}

fn update_graph (graph: &mut Graph, note: Note) {
    if note.parents.len() == 0 {
        let cur_note = note;
        let id = cur_note.id.clone();
        let new_graph: Graph = Graph{children: Vec::new(), note: Some(cur_note), id: Some(id)};
        graph.children.push(new_graph);
    }
}
