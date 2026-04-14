fn main() {
   // Problem 1.1: DNA Sequence printing and length
   let dna: &str = "ATCGAATTGGCC"; //length of 12
   let length = dna.len();
   println!("{dna} length is  {length}");

   // Left over question, difference between `&str` and `String`?

   // Problem 1.2: Watson-Crick complementer
   let a_complement: char = complement('A');
   let t_complement: char = complement('T');
   let c_complement: char = complement('C');
   let g_complement: char = complement('G');
   println!("Complement of A is {a_complement}");
   println!("Complement of T is {t_complement}");
   println!("Complement of C is {c_complement}");
   println!("Complement of G is {g_complement}");

   // Problem 1.3: Codon to amino acid
   let start_codon: &str = translator("ATG");
   let stop_codon: &str = translator("TGA");
   let other_codon: &str = translator("AAA");
   dbg!(start_codon, stop_codon, other_codon);

   // Problem 1.4: Count the Gs
   let mut g_count: i64 = 0;
   
   for base in dna.chars() {
     if base == 'G' {
        g_count = g_count + 1;
     };
   };
   println!("Number of G in {dna} is {g_count}")

}

// Problem 1.2: Watson-Crick complementer
fn complement(base: char) -> char {
    match base {
        'A' => 'T',
        'a' => 'T',
        'T' => 'A',
        't' => 'A',
        'C' => 'G',
        'c' => 'G',
        'G' => 'C',
        'g' => 'C',
        _ => 'X',
    }
}

// Problem 1.3: Codon to amino acid
fn translator(codon: &str) -> &'static str {
    match codon {
        "ATG" | "atg" => "Met (Start)",
        "TAA" | "taa" | "TGA" | "tga" => "Stop",
        _ => "Other",
    }
}