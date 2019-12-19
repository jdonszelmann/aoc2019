use crate::day11::challenge1::get_cpu;

fn main_func(input: &str, area: usize) -> u64 {
    let mut cpu = get_cpu(input);


    let mut xindex = 0;
    let mut yindex = 0;

    for i in 325..2000 {

        let mut count = 0;

        for j in 0..2000 {
            cpu.add_to_input(i as isize);
            cpu.add_to_input(j as isize);
            cpu.run();

            let op = cpu.get_output()[0];

            if op == 1{
                count+=1;
            }

            cpu.reset();
            if count >= 100 && op == 0 {
                let xpos = j-100;
                let mut ycount = 0;
                for y in i..2000 {
                    cpu.add_to_input(y as isize);
                    cpu.add_to_input(xpos as isize);
                    cpu.run();
                    let op = cpu.get_output()[0];

                    if op == 1{
                        ycount+=1;
                    }
                    if ycount >= 100 {
                        break;
                    }
                    cpu.reset();
                }
                if ycount >= 100 {
                    xindex = xpos;
                    yindex = i;
                    return (yindex*10000 + xindex) as u64
                }

                break;
            }
        }
    }

    cpu.reset();

    yindex*10000 + xindex
}

#[cfg(test)]
mod test {
    use crate::day19::challenge2::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input, 50);
        assert_eq!(result, 6950903);
        println!("challenge 19.1: {}", result);
    }

}
