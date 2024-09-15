use anyhow::{Context, Result};
use std::{fs, path::Path};
use tui_tree_widget::TreeItem;

pub fn build_tree<'a, P>(path: P) -> Result<Vec<TreeItem<'a, String>>, anyhow::Error>
where
    P: AsRef<Path>,
{
    let mut items = Vec::new();

    let current_path = path.as_ref();

    let current_item =
        TreeItem::new_leaf(current_path.to_str().unwrap().to_owned(), ".".to_owned());
    items.push(current_item);

    if let Some(parent_path) = current_path.parent() {
        let parent_item =
            TreeItem::new_leaf(parent_path.to_str().unwrap().to_owned(), "..".to_owned());
        items.push(parent_item);
    }

    if current_path.is_dir() {
        for entry in fs::read_dir(current_path)? {
            let entry = entry?;
            let path = entry.path();
            let child = build_tree(&path)?;
            let filename = entry.file_name();

            if path.is_dir() {
                let item = TreeItem::new(
                    path.to_str().unwrap().to_owned(),
                    filename.to_str().unwrap().to_owned(),
                    child,
                )
                .context("Failed to create tree item")?;
                items.push(item);
            } else if path.is_file() || path.is_symlink() {
                let item = TreeItem::new_leaf(
                    path.to_str().unwrap().to_owned(),
                    filename.to_str().unwrap().to_owned(),
                );
                items.push(item);
            }
        }
    }

    Ok(items)
}
