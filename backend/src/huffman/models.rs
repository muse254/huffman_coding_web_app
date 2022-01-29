use multipart::server::Multipart;
use rocket::{
    data::{Data, FromData, Outcome, Transform, Transformed},
    Request,
};
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
pub struct Encoded {
    pub name: String,
    pub codes: HuffmanCodes,
    pub tree: HuffmanTree,
    pub encoded_text: String,
}

#[derive(Debug, Serialize)]
pub struct Decoded {
    pub name: String,
    pub text: Vec<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HuffmanTree {
    Leaf(HuffmanLeaf),
    Node(HuffmanNode),
}

impl HuffmanTree {
    pub fn freq(&self) -> u16 {
        match self {
            HuffmanTree::Leaf(leaf) => leaf.freq,
            HuffmanTree::Node(node) => node.freq,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct HuffmanLeaf {
    pub freq: u16,
    pub value: u8,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct HuffmanNode {
    pub freq: u16,
    pub left: Box<HuffmanTree>,
    pub right: Box<HuffmanTree>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct HuffmanCodes {
    pub huffman_codes: Vec<HuffmanCode>,
}

impl HuffmanCodes {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HuffmanCode {
    pub character: u8,
    pub frequency: u16,
    pub huffman_code: String,
}

#[derive(Deserialize, Debug)]
pub struct CompressRequest<'a> {
    pub text: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct CompressResponse {
    pub code: Vec<u8>,
}

#[derive(Debug)]
pub struct TextFile {
    pub alpha: String,
    pub file: Vec<u8>,
}

impl<'a> FromData<'a> for TextFile {
    type Owned = Vec<u8>;
    type Borrowed = [u8];
    type Error = ();

    fn transform(_request: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
        let mut d = Vec::new();
        data.stream_to(&mut d).expect("Unable to read");

        Transform::Owned(Outcome::Success(d))
    }

    fn from_data(request: &Request, outcome: Transformed<'a, Self>) -> Outcome<Self, Self::Error> {
        let d = outcome.owned()?;

        let ct = request
            .headers()
            .get_one("Content-Type")
            .expect("no content-type");
        let idx = ct.find("boundary=").expect("no boundary");
        let boundary = &ct[(idx + "boundary=".len())..];

        let mut mp = Multipart::with_body(&d[..], boundary);

        // Custom implementation parts
        let mut alpha = None;

        let mut file = None;

        mp.foreach_entry(|mut entry| match &*entry.headers.name {
            "alpha" => {
                let mut t = String::new();
                entry.data.read_to_string(&mut t).expect("not text");
                alpha = Some(t);
            }
            "one" => {}
            "file" => {
                let mut d = Vec::new();
                entry.data.read_to_end(&mut d).expect("not file");
                file = Some(d);
            }
            other => panic!("No known key {}", other),
        })
        .expect("Unable to iterate");

        let v = TextFile {
            alpha: alpha.expect("alpha not set"),

            file: file.expect("file not set"),
        };

        // End custom
        Outcome::Success(v)
    }
}
