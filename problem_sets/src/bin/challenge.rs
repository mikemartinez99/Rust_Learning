pub fn run() {
    println!("Running gene expression challenge question");

    // Define gene expression vector of tuple elements
    // each entry is `&str` and `f64`
    let genes = vec![
        ("GeneA", 5.2),
        ("GeneB", 42.0),
        ("GeneC", 88.1),
        ("GeneD", 120.5),
        ("GeneE", 12.3),
    ];
    // Initialize empty f64 variable to hold total gene expression
    let mut total_expression: f64 = 0.0;
    let mut n_genes: i64 = 0;

    //===== Classify gene expression
    for (gene, value) in &genes {
        let expression_strength: &'static str = classify_expression(*value);
        println!("{gene} - {expression_strength}");

        // Calculate total expression and number of genes
        total_expression = total_expression + *value;
        n_genes = n_genes + 1;
    }

    // Calculate average expression
    let avg_expression: f64 = total_expression / n_genes as f64;
   
    // Print summary statistics
    println!("Total expression across all genes: {total_expression}");
    println!("Total number of genes: {n_genes}");
    println!("Average gene expression: {avg_expression}");
    
}

// Step 1: Function to classify gene expression strength
fn classify_expression(expression: f64) -> &'static str {
    if expression >= 100.0 {
        "Very High"
    } else if expression >= 50.0 {
        "High"
    } else if expression >= 10.0 {
        "Moderate"
    } else {
        "Low"
    }
}


