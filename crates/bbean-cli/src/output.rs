pub fn format_status(label: &str, value: &str) -> String {
    format!("[{}] {}", label.to_uppercase(), value)
}

pub fn format_table(title: &str, rows: &[(&str, &str)]) -> String {
    let mut out = String::new();
    out.push_str(&format!("--- {} ---\n", title));
    let max_key = rows.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    for (key, val) in rows {
        out.push_str(&format!("  {:width$}  {}\n", key, val, width = max_key));
    }
    out
}

pub fn format_node_list(nodes: &[(String, String, String)]) -> String {
    let mut out = String::new();
    out.push_str("--- Nodes ---\n");
    out.push_str(&format!(
        "  {:<36}  {:<15}  {}\n",
        "ID", "STATUS", "COMPUTE"
    ));
    out.push_str(&format!("  {}\n", "-".repeat(65)));
    for (id, status, compute) in nodes {
        out.push_str(&format!("  {:<36}  {:<15}  {}\n", id, status, compute));
    }
    out
}

pub fn format_task_receipt(id: &str, status: &str, queue_pos: Option<u64>) -> String {
    let mut out = format!("[TASK] {}\n", id);
    out.push_str(&format!("  Status: {}\n", status));
    if let Some(pos) = queue_pos {
        out.push_str(&format!("  Queue position: {}\n", pos));
    }
    out
}

pub fn format_error(msg: &str) -> String {
    format!("[ERROR] {}", msg)
}

pub fn format_bytes(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}
