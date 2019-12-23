static INPUT: &str = include_str!("../input.txt");

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let layer = INPUT
        .as_bytes()
        .chunks_exact(25 * 6)
        .min_by_key(|c| c.iter().filter(|&&b| b == b'0').count())
        .unwrap();

    let part1 =
        layer.iter().filter(|&&d| d == b'1').count() * layer.iter().filter(|&&d| d == b'2').count();

    println!("Part 1: {}", part1);

    let mut output = vec![b'2'; 25 * 6];
    INPUT.as_bytes().chunks_exact(25 * 6).for_each(|layer| {
        layer
            .iter()
            .zip(output.iter_mut())
            .filter(|(&l, &mut o)| o == b'2' && (l == b'0' || l == b'1'))
            .for_each(|(&l, o)| *o = l);
    });

    println!("Part 2:");
    output
        .chunks_exact(25)
        .for_each(|b| println!("{}", String::from_utf8(b.into()).unwrap().replace("0", " ")));


    Ok(())
}
