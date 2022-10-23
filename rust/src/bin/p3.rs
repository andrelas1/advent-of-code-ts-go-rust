fn get_input() -> &'static str {
  return "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
}

fn main() {
    let tree_count = get_input()
        .lines()
        .enumerate() // takes an iterator and attaches the index to it
        .flat_map(|(idx, line)|{
            return line
                .chars()
                .nth(idx * 3 % line.len());
        })
        .filter(|&x| x == '#')
        .count();

        println!("{}", tree_count)

}


