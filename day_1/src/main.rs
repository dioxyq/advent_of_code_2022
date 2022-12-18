use std::{fs, cmp};


trait VecExt<T> 
where
    T: Ord,
    Self: Clone
{
    fn sorted(self) -> Vec<T>;
    
    fn reversed(self) -> Vec<T>;
}

impl<T> VecExt<T> for Vec<T>
where
    T: Ord,
    Self: Clone
{
    fn sorted(self) -> Vec<T>
    {
        let mut v = self.clone();
        v.sort();
        v
    }

    fn reversed(self) -> Vec<T>
    {
        let mut v = self.clone();
        v.reverse();
        v
    }

}


trait MaxN: Iterator {
    fn max_n(self, n: usize) -> Vec<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
        Self::Item: Clone
    ;
}

impl<I: Iterator> MaxN for I {
    fn max_n(mut self, n: usize) -> Vec<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
        Self::Item: Clone
    {
        let init = vec![self.next().unwrap().clone(); n];
        self.fold(init, |mut acc, x| {
            let (min, min_index) = acc.iter().enumerate().map(|(i, v)| (v, i)).min().unwrap();
            acc[min_index] = cmp::max(min, &x).clone();
            acc
        } ).sorted().reversed()
    }
}


fn main() {
    let input_path = "input.txt";
    let input = fs::read_to_string(input_path).unwrap();
    let elf_food: Vec<String> = input.split("\n\n").map(str::to_string).collect();
    let elf_food_total: Vec<i32> = elf_food.iter().map(|s| str::split_whitespace(s).map(|v| v.parse::<i32>().unwrap()).sum()).collect();
    let elf_food_enumerate = elf_food_total.iter().enumerate().map(|(i, v)| (v, i));
    // part 1
    let (max_food, max_food_elf_index) = elf_food_enumerate.clone().max().unwrap();
    println!("Elf {} has the most food at a total of {}", max_food_elf_index, max_food);
    // part 2
    let max_3_food = elf_food_enumerate.max_n(3).iter().map(|(v, _i)| **v).sum::<i32>();
    println!("The top 3 elves with the most food have a collective {}", max_3_food);
}
