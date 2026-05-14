// Function to print header for each problem
fn problem_header(title: &str) {
    println!(r#"
\\==============================================
        {}
==============================================//"
       "#, title 
    );
}

//========== FUNCTION DEFINITIONS ==========\\

//----- Function to extract first and last bp
fn extract_dna_bp(sequence_vec: Vec<char>) {
    let first = sequence_vec.first().unwrap();
    let last = sequence_vec.last().unwrap();
    let sequence: String = sequence_vec.iter().collect();
    println!("- Sequence {} length is {}", sequence, sequence_vec.len());
    println!("- Sequence {} first basepair is {}", sequence, first);
    println!("- Sequence {} last basepair is {}", sequence, last)
}

//----- Function to calculate fragment statistics
fn sum_fragment_lengths(lengths: Vec<i32>) {
    let total: i32 = lengths.iter().sum();
    let average: f64 = total as f64 / lengths.len() as f64;
    println!("- Total length of fragments: {}", total);
    println!("- Average length of fragments: {}", average)
}

//----- Function to calculate GC
fn gc_counter(sequence_vec: Vec<char>) {
    
    let length = sequence_vec.len();
    let sequence: String = sequence_vec.iter().collect();

    let g_count = sequence_vec
        .iter()
        .filter(|base| **base == 'G')
        .count();
    
    let c_count = sequence_vec
        .iter()
        .filter(|base| **base == 'C')
        .count();
    
    let gc_percent: f64 = (g_count as f64 + c_count as f64)/length as f64 * 100.0;
    
    println!("- Sequence {} contains {} guanines", sequence, g_count);
    println!("- Sequence {} contains {} cytosines", sequence, c_count);
    println!("- GC % is {:.2}%", gc_percent)
}

//----- Function to filter vectors
fn filter_vector(raw_vec: Vec<i32>, cutoff: i32) {
    println!("-Raw quality scores: {:?}", raw_vec);
    let filtered_vec: Vec<i32> = raw_vec
        .into_iter()
        .filter(|&score| score > cutoff)
        .collect();
    println!("-Filtered quality scores: {:?}", filtered_vec)
}

//----- Function to take reverse complement
fn rev_comp(sequence_vec: Vec<char>) {
    let sequence: String = sequence_vec.iter().collect();
    let reverse_complement: Vec<char> = 
        sequence_vec
            .iter()
            .rev()
            .map(|base| match base {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => 'N',
            })
            .collect();
    let result: String = reverse_complement.iter().collect();
    println!("- Reverse complement of {} is {}", sequence, result)

}

// Function to find genes that contain the letter R
fn letter_r_gene(gene_vec: Vec<&str>) {
    gene_vec
        .iter()
        .for_each(|x|
            if x.contains('R') {
                println!("{x} contains the letter R.")
            });
}

// Function to find the longest motif
fn longest_motif(motif_vec: Vec<&str>) {
    let longest = motif_vec
        .iter()
        .max()
        .unwrap();
    let motif_length = longest
        .len();
    println!("{} is the longest motif with {} basepairs", longest, motif_length)
}

// Function to convert DNA to RNA
fn dna_2_rna(dna_string: &str) {
    let rna_string: String = dna_string
        .chars()
        .map(|base| match base {
            'T' => 'U',
            _ => base
        })
        .collect();
    println!("{} as RNA is {}", dna_string, rna_string)
}

// Function to get expression matrix stats
fn expression_stats(expression_matrix: Vec<Vec<i32>>) {
    for (i, gene) in expression_matrix.iter().enumerate() {
        let sum: i32 = gene.iter().sum();
        let mean = sum as f64 / gene.len() as f64;
        println!("Gene{} total counts: {}", i + 1, sum);
        println!("Gene{} average counts: {}", i + 1, mean);
    };

    let col_sums: Vec<i32> = (0..expression_matrix[0].len())
        .map(|col| expression_matrix.iter().map(|row| row[col]).sum())
        .collect();

    for (i, cell_sum) in col_sums.iter().enumerate() {
        println!("Cell{} total counts: {}", i + 1, cell_sum);
    }
}


fn main() {
    // Problem 1: Storing DNA bases
    problem_header("PROBLEM 1: STORING DNA BASES");
    let dna_bases = vec!['A', 'T', 'C', 'G'];
    extract_dna_bp(dna_bases.clone());

    // Problem 2: RNA fragment lengths
    problem_header("PROBLEM 2: RNA FRAGMENT LENGTHS");
    let rna_frag_lengths = vec![21, 19, 22, 18, 25];
    sum_fragment_lengths(rna_frag_lengths);

    // Problem 3: Count GC bases
    problem_header("PROBLEM 3: COUNT GC BASES");
    let dna_bases_2 = vec!['A', 'T', 'G', 'G', 'C', 'A', 'C', 'C', 'T'];
    gc_counter(dna_bases_2);


    // Problem 5: Remove low quality reads (option 1)
    problem_header("PROBLEM 5: REMOVE LOW QUAL READS");
    let qual_scores = vec![30, 28, 10, 35, 12, 40, 31];
    filter_vector(qual_scores, 20);


    // Problem 6: Reverse complementer
    problem_header("PROBLEM 6: REV COMP");
    rev_comp(dna_bases);

    // Problem 7: Store multiple genes
    problem_header("PROBLEM 7: MULTIPLE GENES");
    let gene_names = vec!["TP53", "EGFR", "MYC", "BRCA1"];
    letter_r_gene(gene_names);

    // Problem 8: Find the longest sequence
    problem_header("PROBLEM 8: FING LONGEST SEQ");
    let motifs = vec!["ATGC", "ATGCGCGT", "AT", "ATGCC"];
    longest_motif(motifs);

    // Problem 9: Convert DNA to RNA
    problem_header("PROBLEM 9: TRANSLATOR");
    let dna_string = "ATTCGTCGATTAGCT";
    dna_2_rna(dna_string);

    // Problem 10 and 11: Expression Matrix
    problem_header("PROBLEM2 10-11: EXPRESSION MATRIX");
    let exp_mat: Vec<Vec<i32>> = vec![
        vec![10, 12, 8, 15],
        vec![3, 5, 1, 2],
        vec![20, 18, 25, 30],
    ];
    expression_stats(exp_mat.clone());

 
}