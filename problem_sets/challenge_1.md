🧬 Rust Biology Challenge: Gene Expression Classifier

🧠 Scenario

You are analyzing a small gene expression dataset from an experiment.

Each gene has an expression value (FPKM-like values). Your task is to:
	1.	Classify gene expression levels
	2.	Compute summary statistics
	3.	Print a per-gene report

⸻

📦 Input Data

let genes = vec![
    ("GeneA", 5.2),
    ("GeneB", 42.0),
    ("GeneC", 88.1),
    ("GeneD", 120.5),
    ("GeneE", 12.3),
];

Each entry is:

(&str, f64)  // (gene_name, expression_value)


⸻

🎯 Tasks

🧪 1. Classify gene expression

Write a function:

fn classify_expression(x: f64) -> &'static str

Rules:
	•	< 10.0 → “low”
	•	10.0 – 50.0 → “moderate”
	•	50.0 – 100.0 → “high”
	•	> 100.0 → “very_high”

👉 Must use if as an expression (no println! inside)

⸻

🧪 2. Loop over genes

For each gene, print:

GeneA: 5.2 -> low
GeneB: 42.0 -> moderate
...

Hint:

for (gene, value) in &genes {


⸻

🧪 3. Track summary statistics

Inside the loop, compute:
	•	total expression
	•	number of genes
	•	average expression

⸻

🧪 4. Final report

After the loop, print:

Total genes: 5
Average expression: XX.XX


⸻

🧪 5. Bonus challenge

Count how many genes fall into each category:
	•	low
	•	moderate
	•	high
	•	very_high

Print:

Low: X
Moderate: X
High: X
Very high: X


⸻

🧠 Constraints

You must use:
	•	variables
	•	mutability (mut)
	•	functions
	•	if / else if
	•	loops over vectors
	•	tuple unpacking
	•	expression-based returns

⸻

💡 Helpful Hints

Iteration

for (gene, value) in &genes {

Accumulators

let mut total = 0.0;
let mut count = 0;

Type casting (if needed)

value as f64


⸻

🔥 Bonus Extension

Store all “high” and “very_high” genes in a separate vector and print them at the end.

let mut high_genes: Vec<&str>


⸻

🧬 What this simulates

This is a simplified version of:
	•	RNA-seq expression summaries
	•	gene filtering pipelines
	•	QC reporting tools
	•	bioinformatics aggregation logic

⸻

Happy coding 🚀
