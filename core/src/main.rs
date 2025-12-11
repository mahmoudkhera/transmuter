use  decoded_tree::generate_decode_file;

fn main() {

    generate_decode_file("src/output.rs", "a32.decode").unwrap();
      
}
