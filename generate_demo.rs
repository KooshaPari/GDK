use gdk::visualization::generate_sample_tree;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate sample tree data
    let commits = generate_sample_tree(20, 4);
    println!("Generated {} sample commits with branching", commits.len());

    // Create ASCII visualization
    let ascii_output = gdk::visualization::export_tree_ascii(&commits, None)?;
    let mut file = File::create("demo_tree.txt")?;
    file.write_all(ascii_output.as_bytes())?;
    println!("🌳 ASCII tree saved to: demo_tree.txt");

    // Create SVG visualization
    let svg_output = gdk::visualization::export_tree_svg(&commits, None)?;
    let mut file = File::create("demo_tree.svg")?;
    file.write_all(svg_output.as_bytes())?;
    println!("📊 SVG tree saved to: demo_tree.svg");

    // Create HTML visualization
    let html_output = gdk::visualization::export_tree_html(&commits, None)?;
    let mut file = File::create("demo_tree.html")?;
    file.write_all(html_output.as_bytes())?;
    println!("🌐 HTML tree saved to: demo_tree.html");

    println!("\n🎉 Demo visualizations created!");
    println!("Open demo_tree.html in your browser to see the interactive tree");

    Ok(())
}