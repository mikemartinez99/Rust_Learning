fn main() {
   // Problem 1.1: DNA Sequence printing and length
   let dna: &str = "ATCGAATTGGCCTGA"; //length of 15
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
   println!("Number of G in {dna} is {g_count}");

   // Problem 2.1: GC content as a percentage
   let gc_prop: f64 = gc_content(dna);
   println!("GC content: {:.2}%", gc_prop);

   // Problem 2.2: Transcriber
   let rna_seq: String = transcriber(dna);
   println!("{} transcribed to RNA is {}", dna, rna_seq);

   // Problem 2.3: First stop codon
   let first_stop_codon: i32 = stop_codon_index(dna);
   println!("Index of first stop codon is {}", first_stop_codon);

   // Problem 3: Ownership and borrowing
   let new_string = String::from("ACGT");
   print_length(&new_string); // This is better because less memory allocation
   print_length(&new_string.clone());


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

// Problem 2.1: GC content as a percentage
fn gc_content(seq: &str) -> f64 {
    let mut gc_count: f64 = 0.0;
    let length = seq.len() as f64;
    for base in seq.chars() {
        if base == 'G' || base == 'C' {
            gc_count += 1.0
        };
    };
    let gc_prop: f64 = gc_count/length * 100.0;
    gc_prop
}

// Problem 2.2: Transcriber
// This is a longer more verbose way to do this...
fn transcriber(dna: &str) -> String {
    // Initialize a new string
    let mut rna: String = String::new();
    
    // Loop through the characters of DNA string
    for base in dna.chars(){
        if base == 'A' {
            rna.push('T')
        } else if base == 'T' {
            rna.push('U')
        } else if base == 'C' {
            rna.push('G')
        } else {
            rna.push('C')
        };
    };
    rna
}

// Problem 2.3: Find the first stop codon (had a hard time with this one)
fn stop_codon_index(seq: &str) -> i32 {
    // Create an iterator
    let mut i = 0;
    while i + 3 <= seq.len() {
        let codon =  &seq[i..i+3];
        if codon == "TAA" || codon == "TAG" || codon == "TGA" {
            return i as i32
        }
        i += 3;
    };

    -1
}

// Problem 3: Ownership and borrowing
fn print_length(seq: &str) {
    let sequence_length = seq.len();
    println!("{sequence_length}")
}