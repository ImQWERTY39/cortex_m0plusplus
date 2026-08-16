use std::io::Write;

use crate::parser::Function;

pub fn write_basic(main: Function) {
    let instructions_iter = main.instructions.into_iter();
    let mut labels_iter = main.labels.iter();
    let mut next_label = labels_iter.next();

    let mut f = std::fs::File::create("./main.s").unwrap();

    // AERA part could remain?
    // __Vectors gotta change once write_basic accepts Program
    // Reset_Handler will be changed to main
    writeln!(
        f,
        r#"	AREA	Reset, DATA, READONLY
	EXPORT	__Vectors

__Vectors
	DCD	0x20001000
	DCD	Reset_Handler

	AREA	|.text|, CODE, READONLY
	THUMB
	EXPORT	Reset_Handler

Reset_Handler PROC"#
    )
    .unwrap();

    for (index, instruction) in instructions_iter.enumerate() {
        while next_label.is_some_and(|(_, idx)| *idx == index) {
            let (label_name, _) = next_label.unwrap();
            f.write(label_name.as_bytes()).unwrap();
            f.write("\n".as_bytes()).unwrap();

            next_label = labels_iter.next();
        }

        writeln!(f, "{instruction}").unwrap();
    }

    if let Some((label, _)) = next_label {
        writeln!(f, "{}", label).unwrap();
        labels_iter.for_each(|(label, _)| writeln!(f, "{}", label).unwrap());
    }

    writeln!(f, "End_Loop\n\tB\t\tEnd_Loop\n\tENDP\n\n\tALIGN\n\tEND").unwrap();
}
