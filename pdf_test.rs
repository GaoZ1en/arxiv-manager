// PDF测试程序
use poppler::Document;

fn main() {
    let file_path = "/home/koishi/Documents/rust/arxiv_manager/downloads/0311136v1.pdf";
    
    // 测试直接路径
    println!("Testing direct path: {}", file_path);
    match Document::from_file(file_path, None) {
        Ok(doc) => {
            println!("✓ Direct path works! Pages: {}", doc.n_pages());
        }
        Err(e) => {
            println!("✗ Direct path failed: {}", e);
        }
    }
    
    // 测试file:// URI
    let file_uri = format!("file://{}", file_path);
    println!("Testing file URI: {}", file_uri);
    match Document::from_file(&file_uri, None) {
        Ok(doc) => {
            println!("✓ File URI works! Pages: {}", doc.n_pages());
        }
        Err(e) => {
            println!("✗ File URI failed: {}", e);
        }
    }
}
