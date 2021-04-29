use ctex::util::write_ctex;
use ctex::flags::Flags;
use glob::glob;

fn main() {
    let paths = glob("input/*.png").unwrap();

    paths.for_each(|p| {
        let p = p.unwrap();
        let input_name = p.clone();
        let input_name = input_name.file_name();
        let mut output_name = p.clone();
        output_name.set_extension("ctex");
        let output_name = output_name.file_name().unwrap();

        let mut input_path = String::from("input/");
        let mut output_path = String::from("output/");

        input_path.push_str(input_name.unwrap().to_str().unwrap());
        output_path.push_str(output_name.to_str().unwrap());

        let output_path = output_path.as_str();
        let input_path = input_path.as_str();

        write_ctex(input_path, output_path, Flags::default()).unwrap();
    });
}
